use crate::blueprints::resource::ProofError;
use crate::errors::*;
use crate::kernel::actor::{ResolvedActor, ResolvedReceiver};
use crate::kernel::call_frame::CallFrameUpdate;
use crate::kernel::kernel_api::{
    ExecutableInvocation, Executor, KernelNodeApi, KernelSubstateApi, LockFlags,
};
use crate::system::kernel_modules::auth::convert_contextless;
use crate::system::kernel_modules::auth::*;
use crate::system::node::RENodeInit;
use crate::types::*;
use crate::wasm::WasmEngine;
use radix_engine_interface::api::node_modules::auth::*;
use radix_engine_interface::api::types::{
    AuthZoneStackFn, AuthZoneStackOffset, GlobalAddress, NativeFn, ProofOffset, RENodeId,
    ResourceManagerOffset, SubstateOffset,
};
use radix_engine_interface::api::{ClientApi, ClientDerefApi};
use radix_engine_interface::blueprints::resource::*;
use radix_engine_interface::data::ScryptoValue;
use sbor::rust::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq, ScryptoCategorize, ScryptoEncode, ScryptoDecode)]
pub enum AuthZoneError {
    EmptyAuthZone,
    ProofError(ProofError),
    CouldNotCreateProof,
    InvalidRequestData(DecodeError),
    CouldNotGetProof,
    CouldNotGetResource,
    NoMethodSpecified,
    AssertAccessRuleError(MethodAuthorization, MethodAuthorizationError),
}

pub struct AuthZoneNativePackage;

impl AuthZoneNativePackage {
    pub fn invoke_export<Y>(
        export_name: &str,
        receiver: Option<ComponentId>,
        input: ScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        match export_name {
            AUTH_ZONE_POP_IDENT => {
                let receiver = receiver.ok_or(RuntimeError::InterpreterError(
                    InterpreterError::NativeExpectedReceiver(export_name.to_string()),
                ))?;
                AuthZoneBlueprint::pop(receiver, input, api)
            }
            AUTH_ZONE_PUSH_IDENT => {
                let receiver = receiver.ok_or(RuntimeError::InterpreterError(
                    InterpreterError::NativeExpectedReceiver(export_name.to_string()),
                ))?;
                AuthZoneBlueprint::push(receiver, input, api)
            }
            AUTH_ZONE_CREATE_PROOF_IDENT => {
                let receiver = receiver.ok_or(RuntimeError::InterpreterError(
                    InterpreterError::NativeExpectedReceiver(export_name.to_string()),
                ))?;
                AuthZoneBlueprint::create_proof(receiver, input, api)
            }
            AUTH_ZONE_CREATE_PROOF_BY_AMOUNT_IDENT => {
                let receiver = receiver.ok_or(RuntimeError::InterpreterError(
                    InterpreterError::NativeExpectedReceiver(export_name.to_string()),
                ))?;
                AuthZoneBlueprint::create_proof_by_amount(receiver, input, api)
            }
            AUTH_ZONE_CREATE_PROOF_BY_IDS_IDENT => {
                let receiver = receiver.ok_or(RuntimeError::InterpreterError(
                    InterpreterError::NativeExpectedReceiver(export_name.to_string()),
                ))?;
                AuthZoneBlueprint::create_proof_by_ids(receiver, input, api)
            }
            _ => Err(RuntimeError::InterpreterError(
                InterpreterError::NativeExportDoesNotExist(export_name.to_string()),
            )),
        }
    }
}

pub struct AuthZoneBlueprint;

impl AuthZoneBlueprint {
    pub(crate) fn pop<Y>(
        _ignored: ComponentId,
        input: ScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let _input: AuthZonePopInput = scrypto_decode(&scrypto_encode(&input).unwrap())
            .map_err(|_| RuntimeError::InterpreterError(InterpreterError::InvalidInvocation))?;

        let auth_zone_handle = api.kernel_lock_substate(
            RENodeId::AuthZoneStack,
            NodeModuleId::SELF,
            SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
            LockFlags::MUTABLE,
        )?;

        let proof = {
            let mut substate_mut = api.kernel_get_substate_ref_mut(auth_zone_handle)?;
            let auth_zone_stack = substate_mut.auth_zone_stack();
            let proof = auth_zone_stack.cur_auth_zone_mut().pop()?;
            proof
        };

        let node_id = api.kernel_allocate_node_id(RENodeType::Proof)?;
        api.kernel_create_node(node_id, RENodeInit::Proof(proof), BTreeMap::new())?;
        let proof_id = node_id.into();

        Ok(IndexedScryptoValue::from_typed(&Proof(proof_id)))
    }

    pub(crate) fn push<Y>(
        _ignored: ComponentId,
        input: ScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: AuthZonePushInput = scrypto_decode(&scrypto_encode(&input).unwrap())
            .map_err(|_| RuntimeError::InterpreterError(InterpreterError::InvalidInvocation))?;

        let auth_zone_handle = api.kernel_lock_substate(
            RENodeId::AuthZoneStack,
            NodeModuleId::SELF,
            SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
            LockFlags::MUTABLE,
        )?;

        let handle = api.kernel_lock_substate(
            RENodeId::Proof(input.proof.0),
            NodeModuleId::SELF,
            SubstateOffset::Proof(ProofOffset::Proof),
            LockFlags::read_only(),
        )?;
        let substate_ref = api.kernel_get_substate_ref(handle)?;
        let proof = substate_ref.proof();
        // Take control of the proof lock as the proof in the call frame will lose it's lock once dropped
        let mut cloned_proof = proof.clone();
        cloned_proof.change_to_unrestricted();

        let mut substate_mut = api.kernel_get_substate_ref_mut(auth_zone_handle)?;
        let auth_zone_stack = substate_mut.auth_zone_stack();
        auth_zone_stack.cur_auth_zone_mut().push(cloned_proof);
        api.kernel_drop_lock(auth_zone_handle)?;

        Ok(IndexedScryptoValue::from_typed(&()))
    }

    pub(crate) fn create_proof<Y>(
        _ignored: ComponentId,
        input: ScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: AuthZoneCreateProofInput = scrypto_decode(&scrypto_encode(&input).unwrap())
            .map_err(|_| RuntimeError::InterpreterError(InterpreterError::InvalidInvocation))?;

        let auth_zone_handle = api.kernel_lock_substate(
            RENodeId::AuthZoneStack,
            NodeModuleId::SELF,
            SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
            LockFlags::MUTABLE,
        )?;

        let resource_type = {
            let resource_id = RENodeId::Global(GlobalAddress::Resource(input.resource_address));
            let offset = SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager);
            let resource_handle = api.kernel_lock_substate(
                resource_id,
                NodeModuleId::SELF,
                offset,
                LockFlags::read_only(),
            )?;
            let substate_ref = api.kernel_get_substate_ref(resource_handle)?;
            substate_ref.resource_manager().resource_type
        };

        let proof = {
            let mut substate_mut = api.kernel_get_substate_ref_mut(auth_zone_handle)?;
            let auth_zone_stack = substate_mut.auth_zone_stack();
            let proof = auth_zone_stack
                .cur_auth_zone()
                .create_proof(input.resource_address, resource_type)?;
            proof
        };

        let node_id = api.kernel_allocate_node_id(RENodeType::Proof)?;
        api.kernel_create_node(node_id, RENodeInit::Proof(proof), BTreeMap::new())?;
        let proof_id = node_id.into();

        Ok(IndexedScryptoValue::from_typed(&Proof(proof_id)))
    }

    pub(crate) fn create_proof_by_amount<Y>(
        _ignored: ComponentId,
        input: ScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: AuthZoneCreateProofByAmountInput = scrypto_decode(&scrypto_encode(&input).unwrap())
            .map_err(|_| RuntimeError::InterpreterError(InterpreterError::InvalidInvocation))?;

        let auth_zone_handle = api.kernel_lock_substate(
            RENodeId::AuthZoneStack,
            NodeModuleId::SELF,
            SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
            LockFlags::MUTABLE,
        )?;

        let resource_type = {
            let resource_id = RENodeId::Global(GlobalAddress::Resource(input.resource_address));
            let offset = SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager);
            let resource_handle = api.kernel_lock_substate(
                resource_id,
                NodeModuleId::SELF,
                offset,
                LockFlags::read_only(),
            )?;
            let substate_ref = api.kernel_get_substate_ref(resource_handle)?;
            substate_ref.resource_manager().resource_type
        };

        let proof = {
            let mut substate_mut = api.kernel_get_substate_ref_mut(auth_zone_handle)?;
            let auth_zone_stack = substate_mut.auth_zone_stack();
            let proof = auth_zone_stack.cur_auth_zone().create_proof_by_amount(
                input.amount,
                input.resource_address,
                resource_type,
            )?;

            proof
        };

        let node_id = api.kernel_allocate_node_id(RENodeType::Proof)?;
        api.kernel_create_node(node_id, RENodeInit::Proof(proof), BTreeMap::new())?;
        let proof_id = node_id.into();

        Ok(IndexedScryptoValue::from_typed(&Proof(proof_id)))
    }

    pub(crate) fn create_proof_by_ids<Y>(
        _ignored: ComponentId,
        input: ScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: AuthZoneCreateProofByIdsInput = scrypto_decode(&scrypto_encode(&input).unwrap())
            .map_err(|_| RuntimeError::InterpreterError(InterpreterError::InvalidInvocation))?;

        let auth_zone_handle = api.kernel_lock_substate(
            RENodeId::AuthZoneStack,
            NodeModuleId::SELF,
            SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
            LockFlags::MUTABLE,
        )?;

        let resource_type = {
            let resource_id = RENodeId::Global(GlobalAddress::Resource(input.resource_address));
            let offset = SubstateOffset::ResourceManager(ResourceManagerOffset::ResourceManager);
            let resource_handle = api.kernel_lock_substate(
                resource_id,
                NodeModuleId::SELF,
                offset,
                LockFlags::read_only(),
            )?;
            let substate_ref = api.kernel_get_substate_ref(resource_handle)?;
            substate_ref.resource_manager().resource_type
        };

        let proof = {
            let substate_ref = api.kernel_get_substate_ref(auth_zone_handle)?;
            let auth_zone_stack = substate_ref.auth_zone_stack();
            let proof = auth_zone_stack.cur_auth_zone().create_proof_by_ids(
                &input.ids,
                input.resource_address,
                resource_type,
            )?;

            proof
        };

        let node_id = api.kernel_allocate_node_id(RENodeType::Proof)?;
        api.kernel_create_node(node_id, RENodeInit::Proof(proof), BTreeMap::new())?;
        let proof_id = node_id.into();

        Ok(IndexedScryptoValue::from_typed(&Proof(proof_id)))
    }
}

impl ExecutableInvocation for AuthZoneClearInvocation {
    type Exec = Self;

    fn resolve<D: ClientDerefApi<RuntimeError>>(
        self,
        _deref: &mut D,
    ) -> Result<(ResolvedActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let receiver = RENodeId::AuthZoneStack;
        let resolved_receiver = ResolvedReceiver::new(receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);

        let actor = ResolvedActor::method(
            NativeFn::AuthZoneStack(AuthZoneStackFn::Clear),
            resolved_receiver,
        );

        Ok((actor, call_frame_update, self))
    }
}

impl Executor for AuthZoneClearInvocation {
    type Output = ();

    fn execute<Y, W: WasmEngine>(self, api: &mut Y) -> Result<((), CallFrameUpdate), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi,
    {
        let auth_zone_handle = api.kernel_lock_substate(
            RENodeId::AuthZoneStack,
            NodeModuleId::SELF,
            SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
            LockFlags::MUTABLE,
        )?;
        let mut substate_mut = api.kernel_get_substate_ref_mut(auth_zone_handle)?;
        let auth_zone_stack = substate_mut.auth_zone_stack();
        auth_zone_stack.cur_auth_zone_mut().clear();

        Ok(((), CallFrameUpdate::empty()))
    }
}

impl ExecutableInvocation for AuthZoneDrainInvocation {
    type Exec = Self;

    fn resolve<D: ClientDerefApi<RuntimeError>>(
        self,
        _deref: &mut D,
    ) -> Result<(ResolvedActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let receiver = RENodeId::AuthZoneStack;
        let resolved_receiver = ResolvedReceiver::new(receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);

        let actor = ResolvedActor::method(
            NativeFn::AuthZoneStack(AuthZoneStackFn::Drain),
            resolved_receiver,
        );

        Ok((actor, call_frame_update, self))
    }
}

impl Executor for AuthZoneDrainInvocation {
    type Output = Vec<Proof>;

    fn execute<Y, W: WasmEngine>(
        self,
        api: &mut Y,
    ) -> Result<(Vec<Proof>, CallFrameUpdate), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi,
    {
        let auth_zone_handle = api.kernel_lock_substate(
            RENodeId::AuthZoneStack,
            NodeModuleId::SELF,
            SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
            LockFlags::MUTABLE,
        )?;

        let proofs = {
            let mut substate_mut = api.kernel_get_substate_ref_mut(auth_zone_handle)?;
            let auth_zone_stack = substate_mut.auth_zone_stack();
            let proofs = auth_zone_stack.cur_auth_zone_mut().drain();
            proofs
        };

        let mut proof_ids: Vec<Proof> = Vec::new();
        let mut nodes_to_move = Vec::new();
        for proof in proofs {
            let node_id = api.kernel_allocate_node_id(RENodeType::Proof)?;
            api.kernel_create_node(node_id, RENodeInit::Proof(proof), BTreeMap::new())?;
            let proof_id = node_id.into();
            proof_ids.push(Proof(proof_id));
            nodes_to_move.push(RENodeId::Proof(proof_id));
        }

        Ok((
            proof_ids,
            CallFrameUpdate {
                nodes_to_move,
                node_refs_to_copy: HashSet::new(),
            },
        ))
    }
}

impl ExecutableInvocation for AuthZoneAssertAccessRuleInvocation {
    type Exec = Self;

    fn resolve<D: ClientDerefApi<RuntimeError>>(
        self,
        _deref: &mut D,
    ) -> Result<(ResolvedActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let receiver = RENodeId::AuthZoneStack;
        let resolved_receiver = ResolvedReceiver::new(receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);

        let actor = ResolvedActor::method(
            NativeFn::AuthZoneStack(AuthZoneStackFn::AssertAccessRule),
            resolved_receiver,
        );

        Ok((actor, call_frame_update, self))
    }
}

impl Executor for AuthZoneAssertAccessRuleInvocation {
    type Output = ();

    fn execute<Y, W: WasmEngine>(self, api: &mut Y) -> Result<((), CallFrameUpdate), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi,
    {
        let handle = api.kernel_lock_substate(
            RENodeId::AuthZoneStack,
            NodeModuleId::SELF,
            SubstateOffset::AuthZoneStack(AuthZoneStackOffset::AuthZoneStack),
            LockFlags::read_only(),
        )?;
        let substate_ref = api.kernel_get_substate_ref(handle)?;
        let auth_zone_stack = substate_ref.auth_zone_stack();
        let authorization = convert_contextless(&self.access_rule);

        // Authorization check
        auth_zone_stack
            .check_auth(false, vec![authorization])
            .map_err(|(authorization, error)| {
                RuntimeError::ApplicationError(ApplicationError::AuthZoneError(
                    AuthZoneError::AssertAccessRuleError(authorization, error),
                ))
            })?;

        api.kernel_drop_lock(handle)?;

        Ok(((), CallFrameUpdate::empty()))
    }
}
