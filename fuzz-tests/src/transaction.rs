#![cfg_attr(feature = "libfuzzer-sys", no_main)]

#[cfg(feature = "libfuzzer-sys")]
use libfuzzer_sys::fuzz_target;
#[cfg(feature = "libfuzzer-sys")]
use once_cell::sync::Lazy;

#[cfg(feature = "afl")]
use afl::fuzz;
#[cfg(feature = "afl")]
use std::panic::AssertUnwindSafe;

#[cfg(feature = "simple-fuzzer")]
mod simple_fuzzer;

use radix_engine::types::{ComponentAddress, EcdsaSecp256k1PublicKey, ResourceAddress};
use radix_engine_interface::blueprints::resource::{FromPublicKey, NonFungibleGlobalId};
use scrypto_unit::TestRunner;
use transaction::ecdsa_secp256k1::EcdsaSecp256k1PrivateKey;
use transaction::model::Instruction;
use transaction::model::TransactionManifest;

struct Account {
    public_key: EcdsaSecp256k1PublicKey,
    _private_key: EcdsaSecp256k1PrivateKey,
    address: ComponentAddress,
}

struct Fuzzer {
    runner: TestRunner,
    accounts: Vec<Account>,
    resources: Vec<ResourceAddress>,
}

impl Fuzzer {
    fn new() -> Self {
        let mut runner = TestRunner::builder().without_trace().build();
        let accounts: Vec<Account> = (0..2)
            .map(|_| {
                let acc = runner.new_account(false);
                println!("addr = {:?}", acc.2);
                Account {
                    public_key: acc.0,
                    _private_key: acc.1,
                    address: acc.2,
                }
            })
            .collect();
        let resources: Vec<ResourceAddress> = vec![
            runner.create_fungible_resource(1000.into(), 18, accounts[0].address),
            runner.create_non_fungible_resource(accounts[0].address),
        ];

        println!("resources = {:?}", resources);

        Self {
            runner,
            accounts,
            resources,
        }
    }

    // pick account from the preallocated pool basing on the input data
    fn get_account(&mut self, data: &[u8]) -> Option<ComponentAddress> {
        let len = data.len();
        if len >= 2 && data[len - 2] % 2 == 0 {
            let idx = *data.last().unwrap() as usize % self.accounts.len();
            return Some(self.accounts[idx].address);
        }
        None
    }

    // pick resource from the preallocated pool basing on the input data
    fn get_resource(&mut self, data: &[u8]) -> Option<ResourceAddress> {
        let len = data.len();
        if len >= 2 && data[len - 2] % 2 == 0 {
            let idx = *data.last().unwrap() as usize % self.accounts.len();
            return Some(self.resources[idx]);
        }
        None
    }

    // Smartly replace some data in the manifest using some preallocated resources.
    // This is to let fuzzing go "deeper" into the manifest instructions and not to reject the
    // transaction on the very early stage
    fn smart_mutate_manifest(&mut self, manifest: &mut TransactionManifest) {
        for i in &mut manifest.instructions {
            match i {
                Instruction::CallMethod {
                    component_address, ..
                }
                | Instruction::SetComponentRoyaltyConfig {
                    component_address, ..
                }
                | Instruction::ClaimComponentRoyalty { component_address } => {
                    if let Some(address) =
                        self.get_account(&component_address.to_array_without_entity_id())
                    {
                        *component_address = address;
                    }
                }
                Instruction::TakeFromWorktop { resource_address }
                | Instruction::TakeFromWorktopByAmount {
                    resource_address, ..
                }
                | Instruction::TakeFromWorktopByIds {
                    resource_address, ..
                }
                | Instruction::AssertWorktopContains { resource_address }
                | Instruction::AssertWorktopContainsByAmount {
                    resource_address, ..
                }
                | Instruction::AssertWorktopContainsByIds {
                    resource_address, ..
                }
                | Instruction::CreateProofFromAuthZone { resource_address }
                | Instruction::CreateProofFromAuthZoneByAmount {
                    resource_address, ..
                }
                | Instruction::CreateProofFromAuthZoneByIds {
                    resource_address, ..
                }
                | Instruction::MintFungible {
                    resource_address, ..
                }
                | Instruction::MintNonFungible {
                    resource_address, ..
                }
                | Instruction::MintUuidNonFungible {
                    resource_address, ..
                } => {
                    if let Some(address) =
                        self.get_resource(&resource_address.to_array_without_entity_id())
                    {
                        *resource_address = address;
                    }
                }
                _ => {}
            }
        }
    }

    fn fuzz_tx_manifest(&mut self, data: &[u8]) -> TxStatus {
        let result = TransactionManifest::from_slice(data);
        match result {
            Ok(mut manifest) => {
                self.smart_mutate_manifest(&mut manifest);
                let receipt = self.runner.execute_manifest(
                    manifest,
                    vec![NonFungibleGlobalId::from_public_key(
                        &self.accounts[0].public_key,
                    )],
                );
                if receipt.is_commit_success() {
                    TxStatus::CommitSuccess
                }
                else {
                    TxStatus::CommitFailure
                }
            }
            Err(_err) => {
                //println!("manifest decoding error {:?}", err);
                TxStatus::DecodeError
            }
        }
    }
}

#[derive(Debug)]
pub enum TxStatus {
    // Transaction commit success
    CommitSuccess,
    // Transaction commit failure
    CommitFailure,
    // TransactionIntent parse error
    DecodeError,
}

#[test]
fn test_fuzz_tx() {
    let mut fuzzer = Fuzzer::new();
    let data = std::fs::read(
        "fuzz_input/transaction/manifest_e057a3853ccb0e33c8b61f2cde91f655473b202c6c095e2202c2ad93caee4e34.raw",
    )
    .unwrap();
    fuzzer.fuzz_tx_manifest(&data);
}

// Fuzzer entry points
#[cfg(feature = "libfuzzer-sys")]
fuzz_target!(|data: &[u8]|{
    unsafe {
        static mut FUZZER: Lazy<Fuzzer> = Lazy::new(|| Fuzzer::new());

        FUZZER.fuzz_tx_manifest(data);
    }
});

#[cfg(feature = "afl")]
fn main() {
    // fuzz! uses `catch_unwind` and it requires RefUnwindSafe trait, which is not auto-implemented by
    // Fuzzer members (TestRunner mainly). `AssertUnwindSafe` annotates the variable is indeed
    // unwind safe
    let mut fuzzer = AssertUnwindSafe(Fuzzer::new());

    fuzz!(|data: &[u8]| {
        fuzzer.fuzz_tx_manifest(data);
    });
}

#[cfg(feature = "simple-fuzzer")]
fn main() {
    let mut fuzzer = Fuzzer::new();

    simple_fuzzer::fuzz(|data: &[u8]| -> TxStatus {
        fuzzer.fuzz_tx_manifest(data)
    });
}