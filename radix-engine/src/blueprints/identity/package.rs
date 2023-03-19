use crate::blueprints::util::{PresecurifiedAccessRules, SecurifiedAccessRules};
use crate::errors::InterpreterError;
use crate::errors::RuntimeError;
use crate::system::kernel_modules::costing::FIXED_LOW_FEE;
use crate::types::*;
use native_sdk::modules::access_rules::AccessRules;
use native_sdk::modules::metadata::Metadata;
use native_sdk::modules::royalty::ComponentRoyalty;
use radix_engine_interface::api::types::ClientCostingReason;
use radix_engine_interface::api::ClientApi;
use radix_engine_interface::blueprints::identity::*;
use radix_engine_interface::blueprints::resource::*;
use radix_engine_interface::schema::FunctionSchema;
use radix_engine_interface::schema::PackageSchema;
use radix_engine_interface::schema::{BlueprintSchema, Receiver};

pub const OWNER_GROUP_NAME: &str = "owner";

pub struct IdentityNativePackage;

impl IdentityNativePackage {
    pub fn schema() -> PackageSchema {
        let mut aggregator = TypeAggregator::<ScryptoCustomTypeKind>::new();

        let substates = Vec::new();

        let mut functions = BTreeMap::new();
        functions.insert(
            IDENTITY_CREATE_ADVANCED_IDENT.to_string(),
            FunctionSchema {
                receiver: None,
                input: aggregator.add_child_type_and_descendents::<IdentityCreateAdvancedInput>(),
                output: aggregator.add_child_type_and_descendents::<IdentityCreateAdvancedOutput>(),
                export_name: IDENTITY_CREATE_ADVANCED_IDENT.to_string(),
            },
        );
        functions.insert(
            IDENTITY_CREATE_IDENT.to_string(),
            FunctionSchema {
                receiver: None,
                input: aggregator.add_child_type_and_descendents::<IdentityCreateInput>(),
                output: aggregator.add_child_type_and_descendents::<IdentityCreateOutput>(),
                export_name: IDENTITY_CREATE_IDENT.to_string(),
            },
        );
        functions.insert(
            IDENTITY_SECURIFY_IDENT.to_string(),
            FunctionSchema {
                receiver: Some(Receiver::SelfRefMut),
                input: aggregator
                    .add_child_type_and_descendents::<IdentitySecurifyToSingleBadgeInput>(),
                output: aggregator
                    .add_child_type_and_descendents::<IdentitySecurifyToSingleBadgeOutput>(),
                export_name: IDENTITY_SECURIFY_IDENT.to_string(),
            },
        );

        // TODO: Make these not visible to client (should only be called by virtualization)
        functions.insert(
            IDENTITY_CREATE_VIRTUAL_ECDSA_256K1_IDENT.to_string(),
            FunctionSchema {
                receiver: None,
                input: aggregator.add_child_type_and_descendents::<VirtualLazyLoadInput>(),
                output: aggregator.add_child_type_and_descendents::<VirtualLazyLoadOutput>(),
                export_name: IDENTITY_CREATE_VIRTUAL_ECDSA_256K1_IDENT.to_string(),
            },
        );
        functions.insert(
            IDENTITY_CREATE_VIRTUAL_EDDSA_25519_IDENT.to_string(),
            FunctionSchema {
                receiver: None,
                input: aggregator.add_child_type_and_descendents::<VirtualLazyLoadInput>(),
                output: aggregator.add_child_type_and_descendents::<VirtualLazyLoadOutput>(),
                export_name: IDENTITY_CREATE_VIRTUAL_EDDSA_25519_IDENT.to_string(),
            },
        );

        let schema = generate_full_schema(aggregator);
        PackageSchema {
            blueprints: btreemap!(
                IDENTITY_BLUEPRINT.to_string() => BlueprintSchema {
                    schema,
                    substates,
                    functions
                }
            ),
        }
    }

    pub fn invoke_export<Y>(
        export_name: &str,
        receiver: Option<RENodeId>,
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        match export_name {
            IDENTITY_CREATE_ADVANCED_IDENT => {
                api.consume_cost_units(FIXED_LOW_FEE, ClientCostingReason::RunNative)?;

                if receiver.is_some() {
                    return Err(RuntimeError::InterpreterError(
                        InterpreterError::NativeUnexpectedReceiver(export_name.to_string()),
                    ));
                }
                let input: IdentityCreateAdvancedInput = input.as_typed().map_err(|e| {
                    RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
                })?;

                let rtn =
                    IdentityBlueprint::create_advanced(input.access_rule, input.mutability, api)?;

                Ok(IndexedScryptoValue::from_typed(&rtn))
            }
            IDENTITY_CREATE_IDENT => {
                api.consume_cost_units(FIXED_LOW_FEE, ClientCostingReason::RunNative)?;

                if receiver.is_some() {
                    return Err(RuntimeError::InterpreterError(
                        InterpreterError::NativeUnexpectedReceiver(export_name.to_string()),
                    ));
                }
                let _input: IdentityCreateInput = input.as_typed().map_err(|e| {
                    RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
                })?;

                let rtn = IdentityBlueprint::create(api)?;

                Ok(IndexedScryptoValue::from_typed(&rtn))
            }
            IDENTITY_SECURIFY_IDENT => {
                api.consume_cost_units(FIXED_LOW_FEE, ClientCostingReason::RunNative)?;

                let receiver = receiver.ok_or(RuntimeError::InterpreterError(
                    InterpreterError::NativeExpectedReceiver(export_name.to_string()),
                ))?;
                let _input: IdentitySecurifyToSingleBadgeInput = input.as_typed().map_err(|e| {
                    RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
                })?;

                let rtn = IdentityBlueprint::securify(receiver, api)?;

                Ok(IndexedScryptoValue::from_typed(&rtn))
            }
            IDENTITY_CREATE_VIRTUAL_ECDSA_256K1_IDENT => {
                api.consume_cost_units(FIXED_LOW_FEE, ClientCostingReason::RunNative)?;

                if receiver.is_some() {
                    return Err(RuntimeError::InterpreterError(
                        InterpreterError::NativeUnexpectedReceiver(export_name.to_string()),
                    ));
                }
                let input: VirtualLazyLoadInput = input.as_typed().map_err(|e| {
                    RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
                })?;

                let rtn = IdentityBlueprint::create_ecdsa_virtual(input.id, api)?;

                Ok(IndexedScryptoValue::from_typed(&rtn))
            }
            IDENTITY_CREATE_VIRTUAL_EDDSA_25519_IDENT => {
                api.consume_cost_units(FIXED_LOW_FEE, ClientCostingReason::RunNative)?;

                if receiver.is_some() {
                    return Err(RuntimeError::InterpreterError(
                        InterpreterError::NativeUnexpectedReceiver(export_name.to_string()),
                    ));
                }
                let input: VirtualLazyLoadInput = input.as_typed().map_err(|e| {
                    RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
                })?;

                let rtn = IdentityBlueprint::create_eddsa_virtual(input.id, api)?;

                Ok(IndexedScryptoValue::from_typed(&rtn))
            }
            _ => Err(RuntimeError::InterpreterError(
                InterpreterError::NativeExportDoesNotExist(export_name.to_string()),
            )),
        }
    }
}

struct SecurifiedIdentity;

impl SecurifiedAccessRules for SecurifiedIdentity {
    const SECURIFY_IDENT: Option<&'static str> = Some(IDENTITY_SECURIFY_IDENT);
    const OWNER_GROUP_NAME: &'static str = "owner";
    const OWNER_TOKEN: ResourceAddress = IDENTITY_OWNER_TOKEN;
}

impl PresecurifiedAccessRules for SecurifiedIdentity {
    const PACKAGE: PackageAddress = IDENTITY_PACKAGE;
}

pub struct IdentityBlueprint;

impl IdentityBlueprint {
    pub fn create_advanced<Y>(
        access_rule: AccessRule,
        mutability: AccessRule,
        api: &mut Y,
    ) -> Result<Address, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let access_rules = SecurifiedIdentity::create_advanced(access_rule, mutability, api)?;

        let (object, modules) = Self::create_object(access_rules, api)?;
        let modules = modules
            .into_iter()
            .map(|(id, own)| (id, own.id()))
            .collect();
        let address = api.globalize(RENodeId::Object(object.id()), modules)?;
        Ok(address)
    }

    pub fn create<Y>(api: &mut Y) -> Result<(Address, Bucket), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let (access_rules, bucket) = SecurifiedIdentity::create_securified(api)?;

        let (object, modules) = Self::create_object(access_rules, api)?;
        let modules = modules
            .into_iter()
            .map(|(id, own)| (id, own.id()))
            .collect();
        let address = api.globalize(RENodeId::Object(object.id()), modules)?;
        Ok((address, bucket))
    }

    pub fn create_ecdsa_virtual<Y>(
        id: [u8; 26],
        api: &mut Y,
    ) -> Result<(Own, BTreeMap<NodeModuleId, Own>), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let non_fungible_global_id = NonFungibleGlobalId::new(
            ECDSA_SECP256K1_TOKEN,
            NonFungibleLocalId::bytes(id.to_vec()).unwrap(),
        );
        let access_rules = SecurifiedIdentity::create_presecurified(non_fungible_global_id, api)?;

        Self::create_object(access_rules, api)
    }

    pub fn create_eddsa_virtual<Y>(
        id: [u8; 26],
        api: &mut Y,
    ) -> Result<(Own, BTreeMap<NodeModuleId, Own>), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let non_fungible_global_id = NonFungibleGlobalId::new(
            EDDSA_ED25519_TOKEN,
            NonFungibleLocalId::bytes(id.to_vec()).unwrap(),
        );
        let access_rules = SecurifiedIdentity::create_presecurified(non_fungible_global_id, api)?;

        Self::create_object(access_rules, api)
    }

    fn securify<Y>(receiver: RENodeId, api: &mut Y) -> Result<Bucket, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        SecurifiedIdentity::securify(receiver, api)
    }

    fn create_object<Y>(
        access_rules: AccessRules,
        api: &mut Y,
    ) -> Result<(Own, BTreeMap<NodeModuleId, Own>), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let metadata = Metadata::sys_create(api)?;
        let royalty = ComponentRoyalty::sys_create(api, RoyaltyConfig::default())?;

        let object_id = api.new_object(IDENTITY_BLUEPRINT, vec![])?;

        let modules = btreemap!(
            NodeModuleId::AccessRules => access_rules.0,
            NodeModuleId::Metadata => metadata,
            NodeModuleId::ComponentRoyalty => royalty,
        );

        Ok((Own::Object(object_id), modules))
    }
}
