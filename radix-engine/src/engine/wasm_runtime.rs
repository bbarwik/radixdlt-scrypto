use sbor::rust::marker::PhantomData;
use sbor::rust::vec::Vec;
use sbor::*;
use scrypto::buffer::scrypto_decode;
use scrypto::core::{Receiver, ScryptoActor, TypeName};
use scrypto::engine::api::RadixEngineInput;
use scrypto::engine::types::*;
use scrypto::resource::AccessRule;
use scrypto::values::ScryptoValue;

use crate::engine::SystemApi;
use crate::engine::{PreCommittedKeyValueStore, RuntimeError};
use crate::fee::*;
use crate::model::{Component, ComponentState};
use crate::wasm::*;

/// A glue between system api (call frame and track abstraction) and WASM.
///
/// Execution is free from a costing perspective, as we assume
/// the system api will bill properly.
pub struct RadixEngineWasmRuntime<'y, 'p, 's, Y, W, I, C>
where
    Y: SystemApi<'p, 's, W, I, C>,
    W: WasmEngine<I>,
    I: WasmInstance,
    C: CostUnitCounter,
{
    actor: ScryptoActor,
    system_api: &'y mut Y,
    phantom1: PhantomData<W>,
    phantom2: PhantomData<I>,
    phantom3: PhantomData<C>,
    phantom4: PhantomData<&'p ()>,
    phantom5: PhantomData<&'s ()>,
}

impl<'y, 'p, 's, Y, W, I, C> RadixEngineWasmRuntime<'y, 'p, 's, Y, W, I, C>
where
    Y: SystemApi<'p, 's, W, I, C>,
    W: WasmEngine<I>,
    I: WasmInstance,
    C: CostUnitCounter,
{
    pub fn new(actor: ScryptoActor, system_api: &'y mut Y) -> Self {
        RadixEngineWasmRuntime {
            actor,
            system_api,
            phantom1: PhantomData,
            phantom2: PhantomData,
            phantom3: PhantomData,
            phantom4: PhantomData,
            phantom5: PhantomData,
        }
    }

    fn cost_unit_counter(&mut self) -> &mut C {
        self.system_api.cost_unit_counter()
    }

    // FIXME: limit access to the API

    fn handle_invoke_function(
        &mut self,
        type_name: TypeName,
        fn_ident: String,
        input: Vec<u8>,
    ) -> Result<ScryptoValue, RuntimeError> {
        let call_data = ScryptoValue::from_slice(&input).map_err(RuntimeError::DecodeError)?;
        self.system_api
            .invoke_function(type_name, fn_ident, call_data)
    }

    fn handle_invoke_method(
        &mut self,
        receiver: Receiver,
        fn_ident: String,
        input: Vec<u8>,
    ) -> Result<ScryptoValue, RuntimeError> {
        let call_data = ScryptoValue::from_slice(&input).map_err(RuntimeError::DecodeError)?;
        self.system_api.invoke_method(receiver, fn_ident, call_data)
    }

    fn handle_create_local_component(
        &mut self,
        package_address: PackageAddress, // FIXME only allow creation of local component from the owning package?
        blueprint_name: String,
        state: Vec<u8>,
    ) -> Result<ComponentAddress, RuntimeError> {
        // Create component
        let component = Component::new(package_address, blueprint_name, Vec::new());
        let component_state = ComponentState::new(state);

        let id = self.system_api.create_node((component, component_state))?;
        Ok(id.into())
    }

    fn handle_create_kv_store(&mut self) -> Result<KeyValueStoreId, RuntimeError> {
        let node_id = self
            .system_api
            .create_node(PreCommittedKeyValueStore::new())?;
        match node_id {
            RENodeId::KeyValueStore(kv_store_id) => Ok(kv_store_id),
            _ => panic!("Expected to be a kv store"),
        }
    }

    // TODO: This logic should move into KeyValueEntry decoding
    fn verify_stored_key(value: &ScryptoValue) -> Result<(), RuntimeError> {
        if !value.bucket_ids.is_empty() {
            return Err(RuntimeError::BucketNotAllowed);
        }
        if !value.proof_ids.is_empty() {
            return Err(RuntimeError::ProofNotAllowed);
        }
        if !value.vault_ids.is_empty() {
            return Err(RuntimeError::VaultNotAllowed);
        }
        if !value.kv_store_ids.is_empty() {
            return Err(RuntimeError::KeyValueStoreNotAllowed);
        }
        Ok(())
    }

    fn handle_node_globalize(&mut self, node_id: RENodeId) -> Result<ScryptoValue, RuntimeError> {
        self.system_api.node_globalize(&node_id)?;
        Ok(ScryptoValue::unit())
    }

    fn handle_substate_read(
        &mut self,
        substate_id: SubstateId,
    ) -> Result<ScryptoValue, RuntimeError> {
        match &substate_id {
            SubstateId::KeyValueStoreEntry(_kv_store_id, key_bytes) => {
                let key_data =
                    ScryptoValue::from_slice(&key_bytes).map_err(RuntimeError::DecodeError)?;
                Self::verify_stored_key(&key_data)?;
            }
            _ => {}
        }

        self.system_api.substate_read(substate_id)
    }

    fn handle_substate_write(
        &mut self,
        substate_id: SubstateId,
        value: Vec<u8>,
    ) -> Result<ScryptoValue, RuntimeError> {
        match &substate_id {
            SubstateId::KeyValueStoreEntry(_kv_store_id, key_bytes) => {
                let key_data =
                    ScryptoValue::from_slice(&key_bytes).map_err(RuntimeError::DecodeError)?;
                Self::verify_stored_key(&key_data)?;
            }
            _ => {}
        }
        let scrypto_value = ScryptoValue::from_slice(&value).map_err(RuntimeError::DecodeError)?;
        self.system_api.substate_write(substate_id, scrypto_value)?;
        Ok(ScryptoValue::unit())
    }

    fn handle_get_actor(&mut self) -> Result<ScryptoActor, RuntimeError> {
        return Ok(self.actor.clone());
    }

    fn handle_generate_uuid(&mut self) -> Result<u128, RuntimeError> {
        let uuid = self
            .system_api
            .generate_uuid()
            .map_err(RuntimeError::CostingError)?;
        Ok(uuid)
    }

    fn handle_emit_log(&mut self, level: Level, message: String) -> Result<(), RuntimeError> {
        self.system_api
            .emit_log(level, message)
            .map_err(RuntimeError::CostingError)?;
        Ok(())
    }

    fn handle_check_access_rule(
        &mut self,
        access_rule: AccessRule,
        proof_ids: Vec<ProofId>,
    ) -> Result<bool, RuntimeError> {
        self.system_api.check_access_rule(access_rule, proof_ids)
    }
}

fn encode<T: Encode>(output: T) -> ScryptoValue {
    ScryptoValue::from_typed(&output)
}

impl<
        'y,
        'p,
        's,
        Y: SystemApi<'p, 's, W, I, C>,
        W: WasmEngine<I>,
        I: WasmInstance,
        C: CostUnitCounter,
    > WasmRuntime for RadixEngineWasmRuntime<'y, 'p, 's, Y, W, I, C>
{
    fn main(&mut self, input: ScryptoValue) -> Result<ScryptoValue, InvokeError> {
        let input: RadixEngineInput =
            scrypto_decode(&input.raw).map_err(|_| InvokeError::InvalidRadixEngineInput)?;
        match input {
            RadixEngineInput::InvokeFunction(type_name, fn_ident, input_bytes) => {
                self.handle_invoke_function(type_name, fn_ident, input_bytes)
            }
            RadixEngineInput::InvokeMethod(receiver, fn_ident, input_bytes) => {
                self.handle_invoke_method(receiver, fn_ident, input_bytes)
            }
            RadixEngineInput::RENodeGlobalize(node_id) => self.handle_node_globalize(node_id),
            RadixEngineInput::CreateComponent(package_address, blueprint_name, state) => self
                .handle_create_local_component(package_address, blueprint_name, state)
                .map(encode),
            RadixEngineInput::CreateKeyValueStore() => self.handle_create_kv_store().map(encode),
            RadixEngineInput::GetActor() => self.handle_get_actor().map(encode),
            RadixEngineInput::SubstateRead(substate_id) => self.handle_substate_read(substate_id),
            RadixEngineInput::SubstateWrite(substate_id, value) => {
                self.handle_substate_write(substate_id, value)
            }
            RadixEngineInput::GenerateUuid() => self.handle_generate_uuid().map(encode),
            RadixEngineInput::EmitLog(level, message) => {
                self.handle_emit_log(level, message).map(encode)
            }
            RadixEngineInput::CheckAccessRule(rule, proof_ids) => {
                self.handle_check_access_rule(rule, proof_ids).map(encode)
            }
        }
        .map_err(InvokeError::RuntimeError)
    }

    fn consume_cost_units(&mut self, n: u32) -> Result<(), InvokeError> {
        self.cost_unit_counter()
            .consume(n, "run_wasm")
            .map_err(InvokeError::CostingError)
    }
}

/// A `Nop` runtime accepts any external function calls by doing nothing and returning void.
pub struct NopWasmRuntime {
    cost_unit_counter: SystemLoanCostUnitCounter,
}

impl NopWasmRuntime {
    pub fn new(cost_unit_counter: SystemLoanCostUnitCounter) -> Self {
        Self { cost_unit_counter }
    }
}

impl WasmRuntime for NopWasmRuntime {
    fn main(&mut self, _input: ScryptoValue) -> Result<ScryptoValue, InvokeError> {
        Ok(ScryptoValue::unit())
    }

    fn consume_cost_units(&mut self, n: u32) -> Result<(), InvokeError> {
        self.cost_unit_counter
            .consume(n, "run_wasm")
            .map_err(InvokeError::CostingError)
    }
}
