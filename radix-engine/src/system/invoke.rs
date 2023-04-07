use crate::blueprints::access_controller::AccessControllerNativePackage;
use crate::blueprints::account::AccountNativePackage;
use crate::blueprints::clock::ClockNativePackage;
use crate::blueprints::epoch_manager::EpochManagerNativePackage;
use crate::blueprints::identity::IdentityNativePackage;
use crate::blueprints::package::{PackageCodeTypeSubstate, PackageNativePackage};
use crate::blueprints::resource::ResourceManagerNativePackage;
use crate::blueprints::transaction_processor::TransactionProcessorNativePackage;
use crate::errors::{InterpreterError, RuntimeError};
use crate::kernel::actor::Actor;
use crate::kernel::call_frame::{CallFrameUpdate, RefType};
use crate::kernel::kernel_api::{KernelInternalApi, KernelNodeApi, KernelSubstateApi, KernelInvokeUpstreamApi, KernelWasmApi};
use crate::system::node_modules::access_rules::AccessRulesNativePackage;
use crate::system::node_modules::metadata::MetadataNativePackage;
use crate::system::node_modules::royalty::RoyaltyNativePackage;
use crate::system::node_modules::type_info::{TypeInfoBlueprint, TypeInfoSubstate};
use crate::types::*;
use crate::wasm::{WasmEngine, WasmInstance, WasmInstrumenter, WasmMeteringConfig, WasmRuntime};
use radix_engine_interface::api::kernel_modules::virtualization::VirtualLazyLoadInput;
use radix_engine_interface::api::substate_api::LockFlags;
use radix_engine_interface::api::ClientApi;
use radix_engine_interface::blueprints::package::*;
use radix_engine_interface::schema::BlueprintSchema;
use resources_tracker_macro::trace_resources;
use crate::kernel::interpreters::ScryptoRuntime;

fn validate_input(
    blueprint_schema: &BlueprintSchema,
    fn_ident: &str,
    with_receiver: bool,
    input: &IndexedScryptoValue,
) -> Result<String, RuntimeError> {
    let function_schema =
        blueprint_schema
            .functions
            .get(fn_ident)
            .ok_or(RuntimeError::InterpreterError(
                InterpreterError::ScryptoFunctionNotFound(fn_ident.to_string()),
            ))?;

    if function_schema.receiver.is_some() != with_receiver {
        return Err(RuntimeError::InterpreterError(
            InterpreterError::ScryptoReceiverNotMatch(fn_ident.to_string()),
        ));
    }

    validate_payload_against_schema(
        input.as_slice(),
        &blueprint_schema.schema,
        function_schema.input,
    )
        .map_err(|err| {
            RuntimeError::InterpreterError(InterpreterError::ScryptoInputSchemaNotMatch(
                fn_ident.to_string(),
                err.error_message(&blueprint_schema.schema),
            ))
        })?;

    Ok(function_schema.export_name.clone())
}

fn validate_output(
    blueprint_schema: &BlueprintSchema,
    fn_ident: &str,
    output: Vec<u8>,
) -> Result<IndexedScryptoValue, RuntimeError> {
    let value = IndexedScryptoValue::from_vec(output).map_err(|e| {
        RuntimeError::InterpreterError(InterpreterError::ScryptoOutputDecodeError(e))
    })?;

    let function_schema = blueprint_schema
        .functions
        .get(fn_ident)
        .expect("Checked by `validate_input`");

    validate_payload_against_schema(
        value.as_slice(),
        &blueprint_schema.schema,
        function_schema.output,
    )
        .map_err(|err| {
            RuntimeError::InterpreterError(InterpreterError::ScryptoOutputSchemaNotMatch(
                fn_ident.to_string(),
                err.error_message(&blueprint_schema.schema),
            ))
        })?;

    Ok(value)
}

#[derive(Debug)]
pub struct SystemInvocation {
    pub blueprint: Blueprint,
    pub ident: FnIdent,
    pub receiver: Option<MethodIdentifier>,
}

pub struct SystemInvoke;

impl KernelInvokeUpstreamApi for SystemInvoke {
    #[trace_resources(log={self.ident.to_debug_string()}, log={self.blueprint.package_address.to_hex()})]
    fn invoke_upstream<Y, W>(
        invocation: SystemInvocation,
        args: &IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + KernelWasmApi<W> + KernelInternalApi + ClientApi<RuntimeError>,
            W: WasmEngine,
    {
        let output = if invocation.blueprint.package_address.eq(&PACKAGE_PACKAGE) {
            // TODO: Clean this up
            api.kernel_load_package_package_dependencies();

            // TODO: Clean this up
            // Do we need to check against the abi? Probably not since we should be able to verify this
            // in the native package itself.
            let export_name = match invocation.ident {
                FnIdent::Application(ident) => ident,
                FnIdent::System(..) => {
                    return Err(RuntimeError::InterpreterError(
                        InterpreterError::InvalidSystemCall,
                    ))
                }
            };
            // Make dependent resources/components visible
            let handle = api.kernel_lock_substate(
                invocation.blueprint.package_address.as_node_id(),
                SysModuleId::ObjectState,
                &PackageOffset::Info.into(),
                LockFlags::read_only(),
            );

            if let Ok(handle) = handle {
                api.kernel_drop_lock(handle)?;
            }

            NativeVm::invoke_native_package(
                PACKAGE_CODE_ID,
                &invocation.receiver,
                &export_name,
                args,
                api,
            )?
        } else if invocation
            .blueprint
            .package_address
            .eq(&TRANSACTION_PROCESSOR_PACKAGE)
        {
            // TODO: the above special rule can be removed if we move schema validation
            // into a kernel model, and turn it off for genesis.
            let export_name = match invocation.ident {
                FnIdent::Application(ident) => ident,
                FnIdent::System(..) => {
                    return Err(RuntimeError::InterpreterError(
                        InterpreterError::InvalidSystemCall,
                    ))
                }
            };

            NativeVm::invoke_native_package(
                TRANSACTION_PROCESSOR_CODE_ID,
                &invocation.receiver,
                &export_name,
                args,
                api,
            )?
        } else {
            // Make dependent resources/components visible
            let handle = api.kernel_lock_substate(
                invocation.blueprint.package_address.as_node_id(),
                SysModuleId::ObjectState,
                &PackageOffset::Info.into(),
                LockFlags::read_only(),
            )?;
            api.kernel_drop_lock(handle)?;

            // TODO: Remove this weirdness or move to a kernel module if we still want to support this
            // Make common resources/components visible
            api.kernel_load_common();

            // Load schema
            let schema = {
                let handle = api.kernel_lock_substate(
                    invocation.blueprint.package_address.as_node_id(),
                    SysModuleId::ObjectState,
                    &PackageOffset::Info.into(),
                    LockFlags::read_only(),
                )?;
                let package_info: PackageInfoSubstate = api.sys_read_substate_typed(handle)?;
                let schema = package_info
                    .schema
                    .blueprints
                    .get(&invocation.blueprint.blueprint_name)
                    .ok_or(RuntimeError::InterpreterError(
                        InterpreterError::ScryptoBlueprintNotFound(invocation.blueprint.clone()),
                    ))?
                    .clone();
                api.kernel_drop_lock(handle)?;
                Box::new(schema)
            };

            //  Validate input
            let export_name = match &invocation.ident {
                FnIdent::Application(ident) => {
                    let export_name =
                        validate_input(&schema, &ident, invocation.receiver.is_some(), &args)?;
                    export_name
                }
                FnIdent::System(system_func_id) => {
                    if let Some(sys_func) = schema.virtual_lazy_load_functions.get(&system_func_id)
                    {
                        sys_func.export_name.to_string()
                    } else {
                        return Err(RuntimeError::InterpreterError(
                            InterpreterError::InvalidSystemCall,
                        ));
                    }
                }
            };

            // Interpret
            let code_type = {
                let handle = api.kernel_lock_substate(
                    invocation.blueprint.package_address.as_node_id(),
                    SysModuleId::ObjectState,
                    &PackageOffset::CodeType.into(),
                    LockFlags::read_only(),
                )?;
                let code_type: PackageCodeTypeSubstate = api.sys_read_substate_typed(handle)?;
                let code_type = code_type.clone();
                api.kernel_drop_lock(handle)?;
                code_type
            };
            let output = match code_type {
                PackageCodeTypeSubstate::Native => {
                    let handle = api.kernel_lock_substate(
                        invocation.blueprint.package_address.as_node_id(),
                        SysModuleId::ObjectState,
                        &PackageOffset::Code.into(),
                        LockFlags::read_only(),
                    )?;
                    let code: PackageCodeSubstate = api.sys_read_substate_typed(handle)?;
                    let native_package_code_id = code.code[0];
                    api.kernel_drop_lock(handle)?;

                    NativeVm::invoke_native_package(
                        native_package_code_id,
                        &invocation.receiver,
                        &export_name,
                        args,
                        api,
                    )?
                        .into()
                }
                PackageCodeTypeSubstate::Wasm => {
                    let mut wasm_instance = {
                        let handle = api.kernel_lock_substate(
                            invocation.blueprint.package_address.as_node_id(),
                            SysModuleId::ObjectState,
                            &PackageOffset::Code.into(),
                            LockFlags::read_only(),
                        )?;
                        let wasm_instance = api
                            .kernel_create_wasm_instance(invocation.blueprint.package_address, handle)?;
                        api.kernel_drop_lock(handle)?;

                        wasm_instance
                    };

                    let output = {
                        let mut runtime: Box<dyn WasmRuntime> = Box::new(ScryptoRuntime::new(api));

                        let mut input = Vec::new();
                        if let Some(MethodIdentifier(node_id, ..)) = invocation.receiver {
                            input.push(
                                runtime
                                    .allocate_buffer(
                                        scrypto_encode(&node_id)
                                            .expect("Failed to encode component id"),
                                    )
                                    .expect("Failed to allocate buffer"),
                            );
                        }
                        input.push(
                            runtime
                                .allocate_buffer(args.as_slice().to_vec())
                                .expect("Failed to allocate buffer"),
                        );

                        wasm_instance.invoke_export(&export_name, input, &mut runtime)?
                    };

                    api.update_wasm_memory_usage(wasm_instance.consumed_memory()?)?;

                    output
                }
            };

            // Validate output
            let output = match invocation.ident {
                FnIdent::Application(ident) => validate_output(&schema, &ident, output)?,
                FnIdent::System(..) => {
                    // TODO: Validate against virtual schema
                    let value = IndexedScryptoValue::from_vec(output).map_err(|e| {
                        RuntimeError::InterpreterError(InterpreterError::ScryptoOutputDecodeError(
                            e,
                        ))
                    })?;
                    value
                }
            };

            output
        };

        Ok(output)
    }
}

struct NativeVm;

impl NativeVm {
    pub fn invoke_native_package<Y>(
        native_package_code_id: u8,
        receiver: &Option<MethodIdentifier>,
        export_name: &str,
        input: &IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
        where
            Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let receiver = receiver.as_ref().map(|x| &x.0);

        match native_package_code_id {
            PACKAGE_CODE_ID => {
                PackageNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            RESOURCE_MANAGER_CODE_ID => {
                ResourceManagerNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            EPOCH_MANAGER_CODE_ID => {
                EpochManagerNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            IDENTITY_CODE_ID => {
                IdentityNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            CLOCK_CODE_ID => ClockNativePackage::invoke_export(&export_name, receiver, input, api),
            ACCOUNT_CODE_ID => {
                AccountNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            ACCESS_CONTROLLER_CODE_ID => {
                AccessControllerNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            TRANSACTION_PROCESSOR_CODE_ID => {
                TransactionProcessorNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            METADATA_CODE_ID => {
                MetadataNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            ROYALTY_CODE_ID => {
                RoyaltyNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            ACCESS_RULES_CODE_ID => {
                AccessRulesNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            _ => Err(RuntimeError::InterpreterError(
                InterpreterError::NativeInvalidCodeId(native_package_code_id),
            )),
        }
    }
}