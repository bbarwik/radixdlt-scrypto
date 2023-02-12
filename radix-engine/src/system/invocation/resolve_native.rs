use crate::errors::{InterpreterError, RuntimeError};
use crate::types::*;
use radix_engine_interface::api::component::*;
use radix_engine_interface::api::node_modules::{auth::*, metadata::*};
use radix_engine_interface::api::package::*;
use radix_engine_interface::blueprints::resource::WorktopAssertContainsInvocation;
use radix_engine_interface::blueprints::{
    epoch_manager::*, logger::*, resource::*, transaction_runtime::*,
};

pub fn resolve_native(
    native_fn: NativeFn,
    invocation: Vec<u8>,
) -> Result<CallTableInvocation, RuntimeError> {
    match native_fn {
        NativeFn::Component(component_fn) => match component_fn {
            ComponentFn::Globalize => {
                let invocation = scrypto_decode::<ComponentGlobalizeInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ComponentFn::GlobalizeWithOwner => {
                let invocation =
                    scrypto_decode::<ComponentGlobalizeWithOwnerInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ComponentFn::SetRoyaltyConfig => {
                let invocation = scrypto_decode::<ComponentSetRoyaltyConfigInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ComponentFn::ClaimRoyalty => {
                let invocation = scrypto_decode::<ComponentClaimRoyaltyInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::Package(package_fn) => match package_fn {
            PackageFn::Publish => {
                let invocation = scrypto_decode::<PackagePublishInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            PackageFn::PublishNative => {
                let invocation = scrypto_decode::<PackagePublishNativeInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            PackageFn::SetRoyaltyConfig => {
                let invocation = scrypto_decode::<PackageSetRoyaltyConfigInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            PackageFn::ClaimRoyalty => {
                let invocation = scrypto_decode::<PackageClaimRoyaltyInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::Bucket(bucket_fn) => match bucket_fn {
            BucketFn::Take => {
                let invocation = scrypto_decode::<BucketTakeInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            BucketFn::CreateProof => {
                let invocation = scrypto_decode::<BucketCreateProofInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            BucketFn::TakeNonFungibles => {
                let invocation = scrypto_decode::<BucketTakeNonFungiblesInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            BucketFn::GetNonFungibleLocalIds => {
                let invocation =
                    scrypto_decode::<BucketGetNonFungibleLocalIdsInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            BucketFn::GetAmount => {
                let invocation = scrypto_decode::<BucketGetAmountInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            BucketFn::Put => {
                let invocation = scrypto_decode::<BucketPutInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            BucketFn::GetResourceAddress => {
                let invocation = scrypto_decode::<BucketGetResourceAddressInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::AuthZoneStack(auth_zone_fn) => match auth_zone_fn {
            AuthZoneStackFn::Pop => {
                let invocation = scrypto_decode::<AuthZonePopInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AuthZoneStackFn::Push => {
                let invocation = scrypto_decode::<AuthZonePushInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AuthZoneStackFn::CreateProof => {
                let invocation = scrypto_decode::<AuthZoneCreateProofInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AuthZoneStackFn::CreateProofByAmount => {
                let invocation =
                    scrypto_decode::<AuthZoneCreateProofByAmountInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AuthZoneStackFn::CreateProofByIds => {
                let invocation = scrypto_decode::<AuthZoneCreateProofByIdsInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AuthZoneStackFn::Clear => {
                let invocation = scrypto_decode::<AuthZoneClearInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AuthZoneStackFn::Drain => {
                let invocation = scrypto_decode::<AuthZoneDrainInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AuthZoneStackFn::AssertAccessRule => {
                let invocation = scrypto_decode::<AuthZoneAssertAccessRuleInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::Proof(proof_fn) => match proof_fn {
            ProofFn::GetAmount => {
                let invocation = scrypto_decode::<ProofGetAmountInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ProofFn::GetNonFungibleLocalIds => {
                let invocation =
                    scrypto_decode::<ProofGetNonFungibleLocalIdsInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ProofFn::GetResourceAddress => {
                let invocation = scrypto_decode::<ProofGetResourceAddressInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ProofFn::Clone => {
                let invocation = scrypto_decode::<ProofCloneInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::Vault(vault_fn) => match vault_fn {
            VaultFn::Take => {
                let invocation = scrypto_decode::<VaultTakeInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            VaultFn::Put => {
                let invocation = scrypto_decode::<VaultPutInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            VaultFn::LockFee => {
                let invocation = scrypto_decode::<VaultLockFeeInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            VaultFn::TakeNonFungibles => {
                let invocation = scrypto_decode::<VaultTakeNonFungiblesInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            VaultFn::GetAmount => {
                let invocation = scrypto_decode::<VaultGetAmountInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            VaultFn::GetResourceAddress => {
                let invocation = scrypto_decode::<VaultGetResourceAddressInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            VaultFn::GetNonFungibleLocalIds => {
                let invocation =
                    scrypto_decode::<VaultGetNonFungibleLocalIdsInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            VaultFn::CreateProof => {
                let invocation = scrypto_decode::<VaultCreateProofInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            VaultFn::CreateProofByAmount => {
                let invocation = scrypto_decode::<VaultCreateProofByAmountInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            VaultFn::CreateProofByIds => {
                let invocation = scrypto_decode::<VaultCreateProofByIdsInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            VaultFn::Recall => {
                let invocation = scrypto_decode::<VaultRecallInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            VaultFn::RecallNonFungibles => {
                let invocation = scrypto_decode::<VaultRecallNonFungiblesInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::AccessRulesChain(access_rules_fn) => match access_rules_fn {
            AccessRulesChainFn::AddAccessCheck => {
                let invocation = scrypto_decode::<AccessRulesAddAccessCheckInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AccessRulesChainFn::SetMethodAccessRule => {
                let invocation =
                    scrypto_decode::<AccessRulesSetMethodAccessRuleInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AccessRulesChainFn::SetMethodMutability => {
                let invocation =
                    scrypto_decode::<AccessRulesSetMethodMutabilityInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AccessRulesChainFn::SetGroupAccessRule => {
                let invocation =
                    scrypto_decode::<AccessRulesSetGroupAccessRuleInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AccessRulesChainFn::SetGroupMutability => {
                let invocation =
                    scrypto_decode::<AccessRulesSetGroupMutabilityInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            AccessRulesChainFn::GetLength => {
                let invocation = scrypto_decode::<AccessRulesGetLengthInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::Metadata(metadata_fn) => match metadata_fn {
            MetadataFn::Set => {
                let invocation = scrypto_decode::<MetadataSetInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            MetadataFn::Get => {
                let invocation = scrypto_decode::<MetadataGetInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::ResourceManager(resource_manager_fn) => match resource_manager_fn {
            ResourceManagerFn::BurnBucket => {
                let invocation = scrypto_decode::<ResourceManagerBurnBucketInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::Burn => {
                let invocation = scrypto_decode::<ResourceManagerBurnInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::UpdateVaultAuth => {
                let invocation =
                    scrypto_decode::<ResourceManagerUpdateVaultAuthInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::SetVaultAuthMutability => {
                let invocation =
                    scrypto_decode::<ResourceManagerSetVaultAuthMutabilityInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::CreateVault => {
                let invocation =
                    scrypto_decode::<ResourceManagerCreateVaultInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::CreateBucket => {
                let invocation =
                    scrypto_decode::<ResourceManagerCreateBucketInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::MintNonFungible => {
                let invocation =
                    scrypto_decode::<ResourceManagerMintNonFungibleInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::MintUuidNonFungible => {
                let invocation =
                    scrypto_decode::<ResourceManagerMintUuidNonFungibleInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::MintFungible => {
                let invocation =
                    scrypto_decode::<ResourceManagerMintFungibleInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::GetResourceType => {
                let invocation =
                    scrypto_decode::<ResourceManagerGetResourceTypeInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::GetTotalSupply => {
                let invocation =
                    scrypto_decode::<ResourceManagerGetTotalSupplyInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::UpdateNonFungibleData => {
                let invocation =
                    scrypto_decode::<ResourceManagerUpdateNonFungibleDataInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::NonFungibleExists => {
                let invocation =
                    scrypto_decode::<ResourceManagerNonFungibleExistsInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ResourceManagerFn::GetNonFungible => {
                let invocation =
                    scrypto_decode::<ResourceManagerGetNonFungibleInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::EpochManager(epoch_manager_fn) => match epoch_manager_fn {
            EpochManagerFn::GetCurrentEpoch => {
                let invocation =
                    scrypto_decode::<EpochManagerGetCurrentEpochInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            EpochManagerFn::NextRound => {
                let invocation = scrypto_decode::<EpochManagerNextRoundInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            EpochManagerFn::SetEpoch => {
                let invocation = scrypto_decode::<EpochManagerSetEpochInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            EpochManagerFn::CreateValidator => {
                let invocation =
                    scrypto_decode::<EpochManagerCreateValidatorInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            EpochManagerFn::UpdateValidator => {
                let invocation =
                    scrypto_decode::<EpochManagerUpdateValidatorInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::Validator(validator_fn) => match validator_fn {
            ValidatorFn::Register => {
                let invocation = scrypto_decode::<ValidatorRegisterInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ValidatorFn::Unregister => {
                let invocation = scrypto_decode::<ValidatorUnregisterInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ValidatorFn::Stake => {
                let invocation = scrypto_decode::<ValidatorStakeInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ValidatorFn::Unstake => {
                let invocation = scrypto_decode::<ValidatorUnstakeInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ValidatorFn::ClaimXrd => {
                let invocation = scrypto_decode::<ValidatorClaimXrdInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ValidatorFn::UpdateKey => {
                let invocation = scrypto_decode::<ValidatorUpdateKeyInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            ValidatorFn::UpdateAcceptDelegatedStake => {
                let invocation =
                    scrypto_decode::<ValidatorUpdateAcceptDelegatedStakeInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::Logger(logger_fn) => match logger_fn {
            LoggerFn::Log => {
                let invocation = scrypto_decode::<LoggerLogInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::Worktop(worktop_fn) => match worktop_fn {
            WorktopFn::TakeNonFungibles => {
                let invocation = scrypto_decode::<WorktopTakeNonFungiblesInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            WorktopFn::Put => {
                let invocation = scrypto_decode::<WorktopPutInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            WorktopFn::Drain => {
                let invocation = scrypto_decode::<WorktopDrainInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            WorktopFn::AssertContainsNonFungibles => {
                let invocation =
                    scrypto_decode::<WorktopAssertContainsNonFungiblesInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            WorktopFn::AssertContains => {
                let invocation = scrypto_decode::<WorktopAssertContainsInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            WorktopFn::AssertContainsAmount => {
                let invocation =
                    scrypto_decode::<WorktopAssertContainsAmountInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            WorktopFn::TakeAll => {
                let invocation = scrypto_decode::<WorktopTakeAllInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            WorktopFn::TakeAmount => {
                let invocation = scrypto_decode::<WorktopTakeAmountInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::TransactionRuntime(tx_runtime_fn) => match tx_runtime_fn {
            TransactionRuntimeFn::GetHash => {
                let invocation = scrypto_decode::<TransactionRuntimeGetHashInvocation>(&invocation)
                    .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
            TransactionRuntimeFn::GenerateUuid => {
                let invocation =
                    scrypto_decode::<TransactionRuntimeGenerateUuidInvocation>(&invocation)
                        .map_err(|_| InterpreterError::InvalidInvocation)?;
                Ok(invocation.into())
            }
        },
        NativeFn::TransactionProcessor(_) => Err(RuntimeError::InterpreterError(
            InterpreterError::DisallowedInvocation(native_fn),
        )),
    }
}
