use radix_engine::{
    errors::{ModuleError, RuntimeError, KernelError},
    system::kernel_modules::transaction_limits::TransactionLimitsError,
    types::*,
    wasm::WASM_MEMORY_PAGE_SIZE,
};
use radix_engine_constants::{
    DEFAULT_MAX_CALL_DEPTH, DEFAULT_MAX_WASM_MEM_PER_CALL_FRAME,
    DEFAULT_MAX_WASM_MEM_PER_TRANSACTION,
};
use radix_engine_interface::blueprints::resource::*;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;
use transaction::data::{manifest_args, ManifestExpression};



#[test]
fn test_max_call_frame_memory_exceeded() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();

    // Grow memory (wasm pages) to exceed default max wasm memory per instance.
    let grow_value: usize = DEFAULT_MAX_WASM_MEM_PER_CALL_FRAME / WASM_MEMORY_PAGE_SIZE as usize;

    // Act
    let code = wat2wasm(&include_str!("wasm/memory.wat").replace("${n}", &grow_value.to_string()));
    let package_address = test_runner.publish_package(
        code,
        generate_single_function_abi(
            "Test",
            "f",
            Type::Tuple {
                element_types: vec![],
            },
        ),
        BTreeMap::new(),
        BTreeMap::new(),
        AccessRules::new(),
    );
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(package_address, "Test", "f", manifest_args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert, exceeded memory should be larger by 1 memory page than the limit
    let expected_mem = DEFAULT_MAX_WASM_MEM_PER_CALL_FRAME + WASM_MEMORY_PAGE_SIZE as usize;
    receipt.expect_specific_failure(|e| match e {
        RuntimeError::ModuleError(ModuleError::TransactionLimitsError(
            TransactionLimitsError::MaxWasmInstanceMemoryExceeded(x),
        )) => *x == expected_mem,
        _ => false,
    })
}

#[test]
fn test_max_transaction_memory_exceeded() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("tests/blueprints/recursion");

    // Calculate value of additional bytes to allocate per call to exceed
    // max wasm memory per transaction limit in nested calls.
    let grow_value: usize = DEFAULT_MAX_WASM_MEM_PER_CALL_FRAME / 2;

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "Caller",
            "recursive_with_memory",
            manifest_args!(DEFAULT_MAX_CALL_DEPTH as u32, grow_value),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert

    // One call frame mem:
    //  => 18 pages from system
    //  => grow value pages from execution of blueprint
    //  => one aditional page from blueprint execution
    let call_frame_mem =
        (18 + grow_value / WASM_MEMORY_PAGE_SIZE as usize + 1) * WASM_MEMORY_PAGE_SIZE as usize;

    // Expected memory equals how many call_frame_mem can fit in per transaction
    // memory plus one, as the limit needs to be exceeded to break transaction.
    let expected_mem = (DEFAULT_MAX_WASM_MEM_PER_TRANSACTION / call_frame_mem + 1) * call_frame_mem;

    // If this assert fails, then adjust grow_value variable.
    assert!((DEFAULT_MAX_WASM_MEM_PER_TRANSACTION / call_frame_mem + 1) < DEFAULT_MAX_CALL_DEPTH);

    receipt.expect_specific_failure(|e| match e {
        RuntimeError::ModuleError(ModuleError::TransactionLimitsError(
            TransactionLimitsError::MaxWasmMemoryExceeded(x),
        )) => *x == expected_mem,
        _ => false,
    })
}

#[test]
fn too_large_manifest_fails_on_substate_read_limit() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();

    // Act
    let (public_key, _, account) = test_runner.new_allocated_account();

    // Act
    let mut builder = ManifestBuilder::new();
    builder.lock_fee(account, 100u32.into());
    builder.withdraw_from_account(account, RADIX_TOKEN, 100u32.into());
    for _ in 0..350 {
        builder.take_from_worktop_by_amount(1.into(), RADIX_TOKEN, |builder, bid| {
            builder.return_to_worktop(bid)
        });
    }
    builder.call_method(
        account,
        "deposit_batch",
        manifest_args!(ManifestExpression::EntireWorktop),
    );
    let manifest = builder.build();

    let receipt = test_runner.execute_manifest(manifest, vec![NonFungibleGlobalId::from_public_key(&public_key)]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ModuleError(ModuleError::TransactionLimitsError(
                TransactionLimitsError::MaxSubstateReadsCountExceeded
            ))
        )
    });
}

#[test]
fn transaction_limit_exceeded_substate_writes_should_fail() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let package_address = test_runner.compile_and_publish("./tests/blueprints/kv_store");
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_function(
            package_address,
            "KeyValueStoreTest",
            "new_kv_store_into_vector",
            manifest_args!(),
        )
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    let component_address = receipt
        .expect_commit()
        .entity_changes
        .new_component_addresses[0];

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(FAUCET_COMPONENT, 10.into())
        .call_method(component_address, "clear_vector", manifest_args!())
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::KernelError(KernelError::StoredNodeRemoved(_))
        )
    });
}