use arbitrary::{Arbitrary, Unstructured};
use radix_engine::types::*;
use radix_engine_interface::api::node_modules::auth::*;
use radix_engine_interface::api::node_modules::royalty::{
    COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT, COMPONENT_ROYALTY_LOCK_ROYALTY_IDENT,
    COMPONENT_ROYALTY_SET_ROYALTY_IDENT,
};
use radix_engine_interface::blueprints::access_controller::*;
use radix_engine_interface::blueprints::account::*;
use radix_engine_interface::blueprints::consensus_manager::*;
use radix_engine_interface::blueprints::identity::*;
use radix_engine_interface::blueprints::package::*;
use radix_engine_interface::blueprints::resource::{FromPublicKey, NonFungibleGlobalId};
#[cfg(feature = "dummy_fuzzing")]
use radix_engine_interface::data::manifest::manifest_decode;
use radix_engine_store_interface::db_key_mapper::{MappedSubstateDatabase, SpreadPrefixKeyMapper};
use scrypto_unit::{DefaultTestRunner, TestRunnerBuilder, TestRunnerSnapshot};
#[cfg(test)]
use std::panic::{catch_unwind, AssertUnwindSafe};
use strum::EnumCount;
use transaction::manifest::ast::{Instruction, InstructionDiscriminants};
use transaction::model::InstructionV1;
use transaction::prelude::*;
use transaction::signing::secp256k1::Secp256k1PrivateKey;

use crate::transaction::common::*;

#[allow(unused)]
const INSTRUCTION_MAX_CNT: u8 = 10;

// Verbose version
#[cfg(feature = "verbose")]
macro_rules! dbg {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-verbose version
#[cfg(not(feature = "verbose"))]
macro_rules! dbg {
    ($( $args:expr ),*) => {};
}

#[derive(Debug, Clone)]
struct Account {
    public_key: Secp256k1PublicKey,
    //_private_key: Secp256k1PrivateKey,
    #[allow(unused)]
    address: ComponentAddress,
    #[allow(unused)]
    resources: Vec<ResourceAddress>,
}

pub struct TxFuzzer {
    runner: DefaultTestRunner,
    snapshot: TestRunnerSnapshot,
    accounts: Vec<Account>,
    #[allow(unused)]
    component_addresses: Vec<ComponentAddress>,
    #[allow(unused)]
    all_resource_addresses: Vec<ResourceAddress>,
    #[allow(unused)]
    fungible_resource_addresses: Vec<ResourceAddress>,
    #[allow(unused)]
    non_fungible_resource_addresses: Vec<ResourceAddress>,
    package_addresses: Vec<PackageAddress>,
    public_keys: Vec<Secp256k1PublicKey>,
}

impl TxFuzzer {
    pub fn new() -> Self {
        let mut runner = TestRunnerBuilder::new().without_trace().build();
        let mut public_keys = vec![];
        let accounts: Vec<Account> = (0..2)
            .map(|_| {
                let acc = runner.new_account(false);
                let resources: Vec<ResourceAddress> = vec![
                    runner.create_fungible_resource(10000.into(), 18, acc.2),
                    runner.create_fungible_resource(10000.into(), 18, acc.2),
                    runner.create_non_fungible_resource(acc.2),
                    runner.create_non_fungible_resource(acc.2),
                ];
                public_keys.push(acc.0);

                Account {
                    public_key: acc.0,
                    //_private_key: acc.1,
                    address: acc.2,
                    resources,
                }
            })
            .collect();

        let (
            package_addresses,
            component_addresses,
            fungible_resource_addresses,
            non_fungible_resource_addresses,
        ) = get_ledger_entries(runner.substate_db());

        let mut all_resource_addresses = fungible_resource_addresses.clone();
        all_resource_addresses.extend(non_fungible_resource_addresses.clone());

        let snapshot = runner.create_snapshot();

        Self {
            runner,
            snapshot,
            accounts,
            component_addresses,
            all_resource_addresses,
            fungible_resource_addresses,
            non_fungible_resource_addresses,
            package_addresses,
            public_keys,
        }
    }

    pub fn reset_runner(&mut self) {
        self.runner.restore_snapshot(self.snapshot.clone());
    }

    #[allow(unused)]
    fn get_non_fungible_local_id(
        &mut self,
        component_address: ComponentAddress,
        resource_address: ResourceAddress,
    ) -> Vec<NonFungibleLocalId> {
        let vaults = self
            .runner
            .get_component_vaults(component_address, resource_address);
        let mut btree_ids = vec![];
        for vault in vaults {
            let mut substate_iter = self
                .runner
                .substate_db()
                .list_mapped::<SpreadPrefixKeyMapper, (), MapKey>(
                    &vault,
                    MAIN_BASE_PARTITION.at_offset(PartitionOffset(1u8)).unwrap(),
                );

            substate_iter.next().map(|(key, _value)| {
                let id: NonFungibleLocalId = scrypto_decode(key.for_map().unwrap()).unwrap();
                btree_ids.push(id);
            });
        }
        btree_ids
    }

    #[allow(unused)]
    fn build_manifest(&mut self, data: &[u8]) -> Result<TransactionManifestV1, TxStatus> {
        // Arbitrary does not return error if not enough data to construct a full instance of
        // Self. It uses dummy values (zeros) instead.
        // TODO: to consider if this is ok to allow it.
        let mut unstructured = Unstructured::new(&data);

        let mut builder = ManifestBuilder::new();
        let mut buckets: Vec<ManifestBucket> =
            vec![ManifestBucket::arbitrary(&mut unstructured).unwrap()];
        let mut proof_ids: Vec<ManifestProof> =
            vec![ManifestProof::arbitrary(&mut unstructured).unwrap()];

        let mut public_keys = self.public_keys.clone();
        public_keys.push(Secp256k1PublicKey::arbitrary(&mut unstructured).unwrap());

        let public_key = unstructured.choose(&public_keys[..]).unwrap().clone();

        let mut package_addresses = self.package_addresses.clone();

        let resource_address = unstructured
            .choose(&self.all_resource_addresses[..])
            .unwrap()
            .clone();
        let component_address = unstructured
            .choose(&self.component_addresses[..])
            .unwrap()
            .clone();
        let non_fungible_resource_address = unstructured
            .choose(&self.non_fungible_resource_addresses[..])
            .unwrap()
            .clone();

        let mut global_addresses = {
            let package_address = unstructured.choose(&package_addresses[..]).unwrap().clone();
            vec![
                GlobalAddress::from(component_address),
                GlobalAddress::from(resource_address),
                GlobalAddress::from(package_address),
            ]
        };
        // TODO: if resource_address of not NonFungible resource is given then we got panic in get_mapped_substate
        // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: UnexpectedSize { expected: 2, actual: 1 }', /Users/lukaszrubaszewski/work/radixdlt/radixdlt-scrypto/radix-engine-stores/src/interface.rs:200:41
        let non_fungible_ids =
            self.get_non_fungible_local_id(component_address, non_fungible_resource_address);

        // To increase the chance of the successful transaction:
        // - fuzz fee amount for 5% of attempts
        // - use random component_address for 5% of attempts
        let fee = if unstructured.int_in_range(0..=100).unwrap() < 5 {
            Decimal::arbitrary(&mut unstructured).unwrap()
        } else {
            Decimal::from(100)
        };
        let fee_address = if unstructured.int_in_range(0..=100).unwrap() < 5 {
            component_address
        } else {
            FAUCET
        };

        builder = builder.lock_fee(fee_address, fee);

        let mut i = 0;
        let instruction_cnt = unstructured.int_in_range(1..=INSTRUCTION_MAX_CNT).unwrap();

        while i < instruction_cnt && !unstructured.is_empty() {
            let next: u8 = unstructured
                .int_in_range(0..=Instruction::COUNT as u8 - 1)
                .unwrap();
            let discriminant = InstructionDiscriminants::from_repr(next as usize).unwrap();

            // TODO: add fuzzing support for not covered instructions
            let instruction = match discriminant {
                // AllocateGlobalAddress
                InstructionDiscriminants::AllocateGlobalAddress => None,
                // AssertWorktopContains
                InstructionDiscriminants::AssertWorktopContains => {
                    let amount = Decimal::arbitrary(&mut unstructured).unwrap();

                    Some(InstructionV1::AssertWorktopContains {
                        amount,
                        resource_address,
                    })
                }
                // AssertWorktopContainsAny
                InstructionDiscriminants::AssertWorktopContainsAny => {
                    Some(InstructionV1::AssertWorktopContainsAny { resource_address })
                }
                // AssertWorktopContainsNonFungibles
                InstructionDiscriminants::AssertWorktopContainsNonFungibles => {
                    Some(InstructionV1::AssertWorktopContainsNonFungibles {
                        resource_address,
                        ids: non_fungible_ids.clone(),
                    })
                }
                // BurnResource
                InstructionDiscriminants::BurnResource => {
                    let bucket_id = *unstructured.choose(&buckets[..]).unwrap();

                    Some(InstructionV1::BurnResource { bucket_id })
                }
                // CallRoleAssignmentMethod
                InstructionDiscriminants::CallRoleAssignmentMethod => {
                    global_addresses.push(GlobalAddress::arbitrary(&mut unstructured).unwrap());
                    let address = *unstructured.choose(&global_addresses[..]).unwrap();
                    let input = RoleAssignmentCreateInput::arbitrary(&mut unstructured).unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallRoleAssignmentMethod {
                            address: address.into(),
                            method_name: ROLE_ASSIGNMENT_CREATE_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // CallFunction
                InstructionDiscriminants::CallFunction => None,
                // CallMetadataMethod
                InstructionDiscriminants::CallMetadataMethod => None,
                // CallMethod
                InstructionDiscriminants::CallMethod => None,
                // CallRoyaltyMethod
                InstructionDiscriminants::CallRoyaltyMethod => {
                    Some(InstructionV1::CallRoyaltyMethod {
                        address: component_address.into(),
                        method_name: COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT.to_string(),
                        args: manifest_args!().into(),
                    })
                }
                // ClaimComponentRoyalty
                InstructionDiscriminants::ClaimComponentRoyalties => {
                    Some(InstructionV1::CallRoyaltyMethod {
                        address: component_address.into(),
                        method_name: COMPONENT_ROYALTY_CLAIM_ROYALTIES_IDENT.to_string(),
                        args: manifest_args!().into(),
                    })
                }
                // ClaimPackageRoyalty
                InstructionDiscriminants::ClaimPackageRoyalties => {
                    package_addresses.push(PackageAddress::arbitrary(&mut unstructured).unwrap());
                    let package_address = *unstructured.choose(&package_addresses[..]).unwrap();
                    Some(InstructionV1::CallMethod {
                        address: package_address.into(),
                        method_name: PACKAGE_CLAIM_ROYALTIES_IDENT.to_string(),
                        args: manifest_args!().into(),
                    })
                }
                // DropAuthZoneProofs
                InstructionDiscriminants::DropAuthZoneProofs => {
                    Some(InstructionV1::DropAuthZoneProofs)
                }
                // DropAuthZoneSignatureProofs
                InstructionDiscriminants::DropAuthZoneSignatureProofs => {
                    Some(InstructionV1::DropAuthZoneSignatureProofs)
                }
                // CloneProof
                InstructionDiscriminants::CloneProof => {
                    let proof_id = *unstructured.choose(&proof_ids[..]).unwrap();

                    Some(InstructionV1::CloneProof { proof_id })
                }
                // CreateAccessController
                InstructionDiscriminants::CreateAccessController => {
                    package_addresses.push(PackageAddress::arbitrary(&mut unstructured).unwrap());
                    let package_address = *unstructured.choose(&package_addresses[..]).unwrap();
                    let bucket_id = *unstructured.choose(&buckets[..]).unwrap();
                    let rule_set = RuleSet::arbitrary(&mut unstructured).unwrap();
                    let timed_recovery_delay_in_minutes =
                        <Option<u32>>::arbitrary(&mut unstructured).unwrap();

                    Some(InstructionV1::CallFunction {
                        package_address: package_address.into(),
                        blueprint_name: ACCESS_CONTROLLER_BLUEPRINT.to_string(),
                        function_name: ACCESS_CONTROLLER_CREATE_IDENT.to_string(),
                        args: manifest_args!(bucket_id, rule_set, timed_recovery_delay_in_minutes)
                            .into(),
                    })
                }
                // CreateAccount
                InstructionDiscriminants::CreateAccount => {
                    package_addresses.push(PackageAddress::arbitrary(&mut unstructured).unwrap());
                    let package_address = *unstructured.choose(&package_addresses[..]).unwrap();
                    let input = AccountCreateInput::arbitrary(&mut unstructured).unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallFunction {
                            package_address: package_address.into(),
                            blueprint_name: ACCOUNT_BLUEPRINT.to_string(),
                            function_name: ACCOUNT_CREATE_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // CreateAccountAdvanced
                InstructionDiscriminants::CreateAccountAdvanced => {
                    package_addresses.push(PackageAddress::arbitrary(&mut unstructured).unwrap());
                    let package_address = *unstructured.choose(&package_addresses[..]).unwrap();
                    let input = AccountCreateAdvancedManifestInput::arbitrary(&mut unstructured).unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallFunction {
                            package_address: package_address.into(),
                            blueprint_name: ACCOUNT_BLUEPRINT.to_string(),
                            function_name: ACCOUNT_CREATE_ADVANCED_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // CreateFungibleResource
                InstructionDiscriminants::CreateFungibleResource => {
                    package_addresses.push(PackageAddress::arbitrary(&mut unstructured).unwrap());
                    let package_address = *unstructured.choose(&package_addresses[..]).unwrap();
                    let input =
                        FungibleResourceManagerCreateManifestInput::arbitrary(&mut unstructured)
                            .unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallFunction {
                            package_address: package_address.into(),
                            blueprint_name: FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
                            function_name: FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // CreateFungibleResourceWithInitialSupply
                InstructionDiscriminants::CreateFungibleResourceWithInitialSupply => {
                    package_addresses.push(PackageAddress::arbitrary(&mut unstructured).unwrap());
                    let package_address = *unstructured.choose(&package_addresses[..]).unwrap();
                    let input =
                        FungibleResourceManagerCreateWithInitialSupplyManifestInput::arbitrary(
                            &mut unstructured,
                        )
                        .unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallFunction {
                            package_address: package_address.into(),
                            blueprint_name: FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
                            function_name:
                                FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT
                                    .to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // CreateIdentity
                InstructionDiscriminants::CreateIdentity => {
                    package_addresses.push(PackageAddress::arbitrary(&mut unstructured).unwrap());
                    let package_address = *unstructured.choose(&package_addresses[..]).unwrap();
                    let input = IdentityCreateInput::arbitrary(&mut unstructured).unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallFunction {
                            package_address: package_address.into(),
                            blueprint_name: IDENTITY_BLUEPRINT.to_string(),
                            function_name: IDENTITY_CREATE_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // CreateIdentityAdvanced
                InstructionDiscriminants::CreateIdentityAdvanced => {
                    package_addresses.push(PackageAddress::arbitrary(&mut unstructured).unwrap());
                    let package_address = *unstructured.choose(&package_addresses[..]).unwrap();
                    let input = IdentityCreateAdvancedInput::arbitrary(&mut unstructured).unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallFunction {
                            package_address: package_address.into(),
                            blueprint_name: IDENTITY_BLUEPRINT.to_string(),
                            function_name: IDENTITY_CREATE_ADVANCED_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // CreateNonFungibleResource
                InstructionDiscriminants::CreateNonFungibleResource => {
                    package_addresses.push(PackageAddress::arbitrary(&mut unstructured).unwrap());
                    let package_address = *unstructured.choose(&package_addresses[..]).unwrap();
                    let input =
                        NonFungibleResourceManagerCreateManifestInput::arbitrary(&mut unstructured)
                            .unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallFunction {
                            package_address: package_address.into(),
                            blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
                            function_name: NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }

                // CreateNonFungibleResourceWithInitialSupply
                InstructionDiscriminants::CreateNonFungibleResourceWithInitialSupply => {
                    package_addresses.push(PackageAddress::arbitrary(&mut unstructured).unwrap());
                    let package_address = *unstructured.choose(&package_addresses[..]).unwrap();
                    let input =
                        &NonFungibleResourceManagerCreateWithInitialSupplyManifestInput::arbitrary(
                            &mut unstructured,
                        )
                        .unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallFunction {
                            package_address: package_address.into(),
                            blueprint_name: NON_FUNGIBLE_RESOURCE_MANAGER_BLUEPRINT.to_string(),
                            function_name:
                                NON_FUNGIBLE_RESOURCE_MANAGER_CREATE_WITH_INITIAL_SUPPLY_IDENT
                                    .to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // CreateProofFromAuthZoneofAll
                InstructionDiscriminants::CreateProofFromAuthZoneOfAll => {
                    Some(InstructionV1::CreateProofFromAuthZoneOfAll { resource_address })
                }
                // CreateProofFromAuthZoneOfAmount
                InstructionDiscriminants::CreateProofFromAuthZoneOfAmount => {
                    let amount = Decimal::arbitrary(&mut unstructured).unwrap();

                    Some(InstructionV1::CreateProofFromAuthZoneOfAmount {
                        amount,
                        resource_address,
                    })
                }
                // CreateProofFromAuthZoneOfNonFungibles
                InstructionDiscriminants::CreateProofFromAuthZoneOfNonFungibles => {
                    Some(InstructionV1::CreateProofFromAuthZoneOfNonFungibles {
                        ids: non_fungible_ids.clone(),
                        resource_address,
                    })
                }
                // CreateProofFromBucketOfAll
                InstructionDiscriminants::CreateProofFromBucketOfAll => {
                    let bucket_id = *unstructured.choose(&buckets[..]).unwrap();

                    Some(InstructionV1::CreateProofFromBucketOfAll { bucket_id })
                }
                // CreateProofFromBucketOfAmount
                InstructionDiscriminants::CreateProofFromBucketOfAmount => {
                    let bucket_id = *unstructured.choose(&buckets[..]).unwrap();
                    let amount = Decimal::arbitrary(&mut unstructured).unwrap();

                    Some(InstructionV1::CreateProofFromBucketOfAmount { bucket_id, amount })
                }
                // CreateProofFromBucketOfNonFungibles
                InstructionDiscriminants::CreateProofFromBucketOfNonFungibles => {
                    let ids = non_fungible_ids.clone();
                    let bucket_id = *unstructured.choose(&buckets[..]).unwrap();

                    Some(InstructionV1::CreateProofFromBucketOfNonFungibles { bucket_id, ids })
                }
                // CreateValidator
                InstructionDiscriminants::CreateValidator => {
                    let bucket_id = *unstructured.choose(&buckets[..]).unwrap();

                    let input = ConsensusManagerCreateValidatorManifestInput {
                        key: public_key,
                        fee_factor: Decimal::ONE,
                        xrd_payment: bucket_id,
                    };

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallMethod {
                            address: component_address.into(),
                            method_name: CONSENSUS_MANAGER_CREATE_VALIDATOR_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // DropAllProofs
                InstructionDiscriminants::DropAllProofs => Some(InstructionV1::DropAllProofs),
                // DropProof
                InstructionDiscriminants::DropProof => {
                    let proof_id = *unstructured.choose(&proof_ids[..]).unwrap();

                    Some(InstructionV1::DropProof { proof_id })
                }
                // FreezeVault
                InstructionDiscriminants::FreezeVault => {
                    let vault_id = {
                        let vaults = self
                            .runner
                            .get_component_vaults(component_address, resource_address);
                        if !vaults.is_empty() {
                            *unstructured.choose(&vaults[..]).unwrap()
                        } else {
                            InternalAddress::arbitrary(&mut unstructured)
                                .unwrap()
                                .into()
                        }
                    };
                    let input = VaultFreezeInput::arbitrary(&mut unstructured).unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallDirectVaultMethod {
                            address: InternalAddress::new_or_panic(vault_id.into()),
                            method_name: VAULT_FREEZE_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // LockComponentRoyalty
                InstructionDiscriminants::LockComponentRoyalty => {
                    let method = String::arbitrary(&mut unstructured).unwrap();

                    Some(InstructionV1::CallRoyaltyMethod {
                        address: component_address.into(),
                        method_name: COMPONENT_ROYALTY_LOCK_ROYALTY_IDENT.to_string(),
                        args: manifest_args!(method).into(),
                    })
                }
                // LockMetadata
                InstructionDiscriminants::LockMetadata => {
                    global_addresses.push(GlobalAddress::arbitrary(&mut unstructured).unwrap());
                    let address = *unstructured.choose(&global_addresses[..]).unwrap();
                    let key = String::arbitrary(&mut unstructured).unwrap();

                    Some(InstructionV1::CallMetadataMethod {
                        address: address.into(),
                        method_name: METADATA_LOCK_IDENT.to_string(),
                        args: manifest_args!(key).into(),
                    })
                }
                // LockOwnerRole
                InstructionDiscriminants::LockOwnerRole => {
                    global_addresses.push(GlobalAddress::arbitrary(&mut unstructured).unwrap());
                    let address = *unstructured.choose(&global_addresses[..]).unwrap();
                    let input = RoleAssignmentLockOwnerInput::arbitrary(&mut unstructured).unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallRoleAssignmentMethod {
                            address: address.into(),
                            method_name: ROLE_ASSIGNMENT_LOCK_OWNER_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // MintFungible
                InstructionDiscriminants::MintFungible => {
                    let amount = Decimal::arbitrary(&mut unstructured).unwrap();

                    Some(InstructionV1::CallMethod {
                        address: resource_address.into(),
                        method_name: FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT.to_string(),
                        args: manifest_args!(amount).into(),
                    })
                }
                // MintNonFungible
                InstructionDiscriminants::MintNonFungible => {
                    let input =
                        NonFungibleResourceManagerMintManifestInput::arbitrary(&mut unstructured)
                            .unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallMethod {
                            address: resource_address.into(),
                            method_name: NON_FUNGIBLE_RESOURCE_MANAGER_MINT_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // MintRuidNonFungible
                InstructionDiscriminants::MintRuidNonFungible => {
                    let input = NonFungibleResourceManagerMintRuidManifestInput::arbitrary(
                        &mut unstructured,
                    )
                    .unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallMethod {
                            address: resource_address.into(),
                            method_name: NON_FUNGIBLE_RESOURCE_MANAGER_MINT_RUID_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // PopFromAuthZone
                InstructionDiscriminants::PopFromAuthZone => {
                    Some(InstructionV1::PopFromAuthZone {})
                }
                // PublishPackage | PublishPackageAdvanced
                InstructionDiscriminants::PublishPackage
                | InstructionDiscriminants::PublishPackageAdvanced => {
                    // Publishing package involves a compilation by scrypto compiler.
                    // In case of AFL invoking external tool breaks fuzzing.
                    // For now we skip this step
                    // TODO: compile some packages before starting AFL and read compiled
                    //  binaries in AFL
                    None
                }
                // PushToAuthZone
                InstructionDiscriminants::PushToAuthZone => {
                    let proof_id = *unstructured.choose(&proof_ids[..]).unwrap();

                    Some(InstructionV1::PushToAuthZone { proof_id })
                }
                // RecallFromVault
                InstructionDiscriminants::RecallFromVault => {
                    let amount = Decimal::arbitrary(&mut unstructured).unwrap();
                    let vault_id = {
                        let vaults = self
                            .runner
                            .get_component_vaults(component_address, resource_address);
                        if !vaults.is_empty() {
                            *unstructured.choose(&vaults[..]).unwrap()
                        } else {
                            InternalAddress::arbitrary(&mut unstructured)
                                .unwrap()
                                .into()
                        }
                    };

                    Some(InstructionV1::CallDirectVaultMethod {
                        address: InternalAddress::new_or_panic(vault_id.into()),
                        method_name: VAULT_RECALL_IDENT.to_string(),
                        args: manifest_args!(amount).into(),
                    })
                }
                // RecallNonFungiblesFromVault
                InstructionDiscriminants::RecallNonFungiblesFromVault => {
                    let input = NonFungibleVaultRecallNonFungiblesInput {
                        non_fungible_local_ids: IndexSet::from_iter(
                            non_fungible_ids.clone().into_iter(),
                        ),
                    };

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallMethod {
                            address: resource_address.into(),
                            method_name: NON_FUNGIBLE_VAULT_RECALL_NON_FUNGIBLES_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // RemoveMetadata
                InstructionDiscriminants::RemoveMetadata => {
                    global_addresses.push(GlobalAddress::arbitrary(&mut unstructured).unwrap());
                    let address = *unstructured.choose(&global_addresses[..]).unwrap();
                    let key = String::arbitrary(&mut unstructured).unwrap();

                    Some(InstructionV1::CallMetadataMethod {
                        address: address.into(),
                        method_name: METADATA_REMOVE_IDENT.to_string(),
                        args: manifest_args!(key).into(),
                    })
                }
                // ReturnToWorktop
                InstructionDiscriminants::ReturnToWorktop => {
                    let bucket_id = *unstructured.choose(&buckets[..]).unwrap();

                    Some(InstructionV1::ReturnToWorktop { bucket_id })
                }
                // SetComponentRoyalty
                InstructionDiscriminants::SetComponentRoyalty => {
                    let method = String::arbitrary(&mut unstructured).unwrap();
                    let amount = RoyaltyAmount::arbitrary(&mut unstructured).unwrap();

                    Some(InstructionV1::CallRoyaltyMethod {
                        address: component_address.into(),
                        method_name: COMPONENT_ROYALTY_SET_ROYALTY_IDENT.to_string(),
                        args: manifest_args!(method, amount).into(),
                    })
                }
                // SetMetadata
                InstructionDiscriminants::SetMetadata => {
                    global_addresses.push(GlobalAddress::arbitrary(&mut unstructured).unwrap());
                    let address = *unstructured.choose(&global_addresses[..]).unwrap();
                    let key = String::arbitrary(&mut unstructured).unwrap();
                    let value = MetadataValue::arbitrary(&mut unstructured).unwrap();

                    Some(InstructionV1::CallMetadataMethod {
                        address: address.into(),
                        method_name: METADATA_SET_IDENT.to_string(),
                        args: manifest_args!(key, value).into(),
                    })
                }
                // SetOwnerRole
                InstructionDiscriminants::SetOwnerRole => {
                    global_addresses.push(GlobalAddress::arbitrary(&mut unstructured).unwrap());
                    let address = *unstructured.choose(&global_addresses[..]).unwrap();
                    let input = RoleAssignmentSetOwnerInput::arbitrary(&mut unstructured).unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallRoleAssignmentMethod {
                            address: address.into(),
                            method_name: ROLE_ASSIGNMENT_SET_OWNER_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // SetRole
                InstructionDiscriminants::SetRole => {
                    global_addresses.push(GlobalAddress::arbitrary(&mut unstructured).unwrap());
                    let address = *unstructured.choose(&global_addresses[..]).unwrap();
                    let input = RoleAssignmentSetInput::arbitrary(&mut unstructured).unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallRoleAssignmentMethod {
                            address: address.into(),
                            method_name: ROLE_ASSIGNMENT_SET_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                // TakeAllFromWorktop
                InstructionDiscriminants::TakeAllFromWorktop => {
                    Some(InstructionV1::TakeAllFromWorktop { resource_address })
                }
                // TakeFromWorktop
                InstructionDiscriminants::TakeFromWorktop => {
                    let amount = Decimal::arbitrary(&mut unstructured).unwrap();

                    Some(InstructionV1::TakeFromWorktop {
                        amount,
                        resource_address,
                    })
                }
                // TakeNonFungiblesFromWorktop
                InstructionDiscriminants::TakeNonFungiblesFromWorktop => {
                    Some(InstructionV1::TakeNonFungiblesFromWorktop {
                        ids: non_fungible_ids.clone(),
                        resource_address,
                    })
                }
                // UnfreezeVault
                InstructionDiscriminants::UnfreezeVault => {
                    let vault_id = {
                        let vaults = self
                            .runner
                            .get_component_vaults(component_address, resource_address);
                        if !vaults.is_empty() {
                            *unstructured.choose(&vaults[..]).unwrap()
                        } else {
                            InternalAddress::arbitrary(&mut unstructured)
                                .unwrap()
                                .into()
                        }
                    };
                    let input = VaultUnfreezeInput::arbitrary(&mut unstructured).unwrap();

                    match to_manifest_value(&input) {
                        Ok(args) => Some(InstructionV1::CallDirectVaultMethod {
                            address: InternalAddress::new_or_panic(vault_id.into()),
                            method_name: VAULT_UNFREEZE_IDENT.to_string(),
                            args,
                        }),
                        Err(_) => None,
                    }
                }
                InstructionDiscriminants::DropAuthZoneRegularProofs => {
                    Some(InstructionV1::DropAuthZoneRegularProofs)
                }
                InstructionDiscriminants::DropNamedProofs => Some(InstructionV1::DropNamedProofs),
            };
            if let Some(instruction) = instruction {
                let (
                    updated_builder,
                    NewSymbols {
                        new_bucket,
                        new_proof,
                        ..
                    },
                ) = builder.add_instruction_advanced(instruction);
                builder = updated_builder;
                if let Some(bucket_id) = new_bucket {
                    buckets.push(bucket_id)
                }
                if let Some(proof_id) = new_proof {
                    proof_ids.push(proof_id)
                }
                i += 1;
            }
        }

        let manifest = builder.build();
        dbg!("manifest = {:?}", manifest);
        Ok(manifest)
    }

    pub fn fuzz_tx_manifest(&mut self, data: &[u8]) -> TxStatus {
        #[cfg(feature = "dummy_fuzzing")]
        let result = manifest_decode::<TransactionManifestV1>(data);
        #[cfg(not(feature = "dummy_fuzzing"))]
        let result = self.build_manifest(data);

        match result {
            #[allow(unused_mut)]
            Ok(mut manifest) => {
                let receipt = self.runner.execute_manifest(
                    manifest,
                    vec![NonFungibleGlobalId::from_public_key(
                        &self.accounts[0].public_key,
                    )],
                );
                if receipt.is_commit_success() {
                    TxStatus::CommitSuccess
                } else {
                    TxStatus::CommitFailure
                }
            }
            Err(_err) => TxStatus::DecodeError,
        }
    }
}

#[derive(Debug)]
pub enum TxStatus {
    // Transaction manifest build error
    #[allow(unused)]
    ManifestBuildError,
    // Transaction commit success
    CommitSuccess,
    // Transaction commit failure
    CommitFailure,
    // Transaction manifest parse error
    #[allow(unused)]
    DecodeError,
}

#[test]
fn test_check_fuzzed_instruction_coverage() {
    use rand::{Rng, RngCore};
    use rand_chacha::rand_core::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    let mut rng = ChaCha8Rng::seed_from_u64(1234);
    let mut fuzzer = TxFuzzer::new();
    for _ in 0..5000 {
        let len = rng.gen_range(0..1024);
        let mut bytes: Vec<u8> = (0..len).map(|_| rng.gen_range(0..u8::MAX)).collect();
        rng.fill_bytes(&mut bytes[..]);

        let result = catch_unwind(AssertUnwindSafe(|| {
            fuzzer.reset_runner();
            fuzzer.build_manifest(&bytes[..])
        }));
        if let Err(err) = result {
            let err_msg = err.downcast::<String>().unwrap();
            if err_msg.contains("Not covered instruction") {
                panic!("Found not covered instruction");
            }
        }
    }
}

// This test tries is supposed to generate fuzz input data.
// It generates and executes manifest. If transaction successful then save the manifest data.
#[test]
#[cfg(not(feature = "dummy_fuzzing"))]
fn test_generate_fuzz_input_data() {
    use rand::{Rng, RngCore};
    use rand_chacha::rand_core::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    let mut rng = ChaCha8Rng::seed_from_u64(1234);
    let mut fuzzer = TxFuzzer::new();
    for _ in 0..5000 {
        let len = rng.gen_range(0..1024);
        let mut bytes: Vec<u8> = (0..len).map(|_| rng.gen_range(0..u8::MAX)).collect();
        rng.fill_bytes(&mut bytes[..]);

        let _result = catch_unwind(AssertUnwindSafe(|| {
            fuzzer.reset_runner();
            if let TxStatus::CommitSuccess = fuzzer.fuzz_tx_manifest(&bytes[..]) {
                let m_hash = hash(&bytes);
                let path = format!("manifest_{:?}.raw", m_hash);
                std::fs::write(&path, bytes).unwrap();
                println!("manifest dumped to file {}", &path);
            }
        }));
    }
}

// This test is supposed to generate fuzz input data.
// It runs radix-engine-tests tests with "dump_manifest_to_file" flag,
// which writes each used transaction manifest to file.
#[test]
#[cfg(feature = "dummy_fuzzing")]
fn test_generate_fuzz_input_data() {
    /*
    cargo nextest run -p radix-engine-tests --release --features dump_manifest_to_file
    mv ../radix-engine-tests/manifest_*.raw ${curr_path}/${raw_dir}
    */
    use std::fs;

    use std::io::{BufRead, BufReader};
    use std::process::Command;
    use std::process::Stdio;
    const WORK_DIR: &str = "/Users/lukaszrubaszewski/work/radixdlt/radixdlt-scrypto";
    const PACKAGE: &str = "radix-engine-tests";

    let mut child = Command::new("cargo")
        .current_dir(WORK_DIR)
        .stdin(Stdio::null())
        .arg("nextest")
        .arg("run")
        .arg("-p")
        .arg(PACKAGE)
        .arg("--release")
        .arg("--features")
        .arg("dump_manifest_to_file")
        .spawn()
        .expect("failed to execute process");

    if let Some(stdout) = &mut child.stdout {
        let lines = BufReader::new(stdout).lines().enumerate().take(10);
        for (_, line) in lines {
            println!("{:?}", line);
        }
    }

    child.wait().expect("failed to wait");

    let entries = fs::read_dir(format!("{}/{}", WORK_DIR, PACKAGE)).unwrap();

    entries
        .filter_map(|entry| Some(entry.unwrap()))
        .for_each(|entry| {
            let path = entry.path();
            let fname = path.file_name().unwrap().to_str().unwrap();
            if fname.ends_with(".raw") && fname.starts_with("manifest_") {
                fs::rename(entry.path(), fname).unwrap();
            }
        });
}

// Initialize static objects outside the fuzzing loop to assure deterministic instrumentation
// output across runs.
pub fn fuzz_tx_init_statics() {
    // Following code initializes secp256k1::SECP256K1 global static context
    let private_key = Secp256k1PrivateKey::from_u64(100).unwrap();
    let _public_key = private_key.public_key();
}