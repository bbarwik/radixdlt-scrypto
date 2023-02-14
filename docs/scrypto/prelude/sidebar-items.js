window.SIDEBAR_ITEMS = {"attr":[["blueprint","Declares a blueprint."]],"constant":[["ACCOUNT_BLUEPRINT",""],["ACCOUNT_PACKAGE","The address of the account package."],["CLOCK",""],["ECDSA_SECP256K1_TOKEN","The ECDSA virtual resource address."],["EDDSA_ED25519_TOKEN","The ED25519 virtual resource address."],["EPOCH_MANAGER",""],["FAUCET_BLUEPRINT",""],["FAUCET_COMPONENT","The address of the faucet component, test network only."],["FAUCET_PACKAGE","The address of the faucet package."],["MAX_SCRYPTO_SBOR_DEPTH",""],["NON_FUNGIBLE_LOCAL_ID_MAX_LENGTH",""],["OLYMPIA_VALIDATOR_TOKEN",""],["PACKAGE_TOKEN",""],["RADIX_TOKEN","The XRD resource address."],["SCRYPTO_SBOR_V1_PAYLOAD_PREFIX",""],["SYSTEM_TOKEN","The system token which allows access to system resources (e.g. setting epoch)"],["VALUE_KIND_BLOB",""],["VALUE_KIND_BUCKET",""],["VALUE_KIND_COMPONENT_ADDRESS",""],["VALUE_KIND_DECIMAL",""],["VALUE_KIND_ECDSA_SECP256K1_PUBLIC_KEY",""],["VALUE_KIND_ECDSA_SECP256K1_SIGNATURE",""],["VALUE_KIND_EDDSA_ED25519_PUBLIC_KEY",""],["VALUE_KIND_EDDSA_ED25519_SIGNATURE",""],["VALUE_KIND_EXPRESSION",""],["VALUE_KIND_HASH",""],["VALUE_KIND_NON_FUNGIBLE_LOCAL_ID",""],["VALUE_KIND_OWN",""],["VALUE_KIND_PACKAGE_ADDRESS",""],["VALUE_KIND_PRECISE_DECIMAL",""],["VALUE_KIND_PROOF",""],["VALUE_KIND_RESOURCE_ADDRESS",""]],"derive":[["Categorize","Derive code that returns the value kind."],["Decode","Derive code that decodes this data structure from a byte array."],["Encode","Derive code that encodes this data structure"],["LegacyDescribe","Derive code that describes this data structure."],["NonFungibleData","Derive code that describe a non-fungible data structure."],["ScryptoCategorize","Derives code for categorizing a struct or enum with Scrypto value model."],["ScryptoDecode","Derives code for decoding a struct or enum with Scrypto value model."],["ScryptoDescribe","Derives code for describing a struct or enum with Scrypto schema."],["ScryptoEncode","Derives code for encoding a struct or enum with Scrypto value model."],["ScryptoSbor","Derive code that implements `ScryptoCategorize`, `ScryptoEncode`, `ScryptoDecode`, and `ScryptoDescribe` traits for this struct or enum."]],"enum":[["AccessControllerInvocation",""],["AccessRule",""],["AccessRuleEntry",""],["AccessRuleKey",""],["AccessRuleNode",""],["AccessRulesChainInvocation",""],["AuthZoneStackInvocation",""],["BucketInvocation",""],["CallTableInvocation",""],["ClockInvocation",""],["ComponentAddress","An instance of a blueprint, which lives in the ledger state."],["ComponentInvocation",""],["ContentValidationError",""],["DateTimeError",""],["DecodeError","Represents an error ocurred during decoding."],["EpochManagerInvocation",""],["IdentityInvocation",""],["Level","Represents the level of a log message."],["LoggerInvocation",""],["ManifestExpression",""],["MetadataInvocation",""],["NativeInvocation",""],["NonFungibleIdType","Represents type of non-fungible id"],["NonFungibleLocalId","Represents the local id of a non-fungible."],["Own",""],["PackageAddress","A collection of blueprints, compiled and published as a single unit."],["PackageInvocation",""],["ParseDecimalError","Represents an error when parsing Decimal from another type."],["ParseEcdsaSecp256k1PublicKeyError","Represents an error when parsing ECDSA public key from hex."],["ParseEcdsaSecp256k1SignatureError",""],["ParseEddsaEd25519PublicKeyError","Represents an error when parsing ED25519 public key from hex."],["ParseEddsaEd25519SignatureError",""],["ParseHashError","Represents an error when parsing hash."],["ParseI128Error",""],["ParseI16Error",""],["ParseI256Error",""],["ParseI32Error",""],["ParseI384Error",""],["ParseI512Error",""],["ParseI64Error",""],["ParseI768Error",""],["ParseI8Error",""],["ParseManifestBlobRefError","Represents an error when parsing ManifestBlobRef."],["ParseManifestBucketError","Represents an error when parsing ManifestBucket."],["ParseManifestExpressionError","Represents an error when parsing ManifestExpression."],["ParseManifestProofError","Represents an error when parsing ManifestProof."],["ParseNonFungibleGlobalIdError","Represents an error when parsing non-fungible address."],["ParseNonFungibleLocalIdError","Represents an error when decoding non-fungible id."],["ParseOwnError",""],["ParsePreciseDecimalError","Represents an error when parsing PreciseDecimal from another type."],["ParseU128Error",""],["ParseU16Error",""],["ParseU256Error",""],["ParseU32Error",""],["ParseU384Error",""],["ParseU512Error",""],["ParseU64Error",""],["ParseU768Error",""],["ParseU8Error",""],["ParseUtcDateTimeError",""],["Parsei128Error",""],["Parsei16Error",""],["Parsei32Error",""],["Parsei64Error",""],["Parsei8Error",""],["ParseisizeError",""],["Parseu128Error",""],["Parseu16Error",""],["Parseu32Error",""],["Parseu64Error",""],["Parseu8Error",""],["ParseusizeError",""],["ProofInvocation",""],["ProofRule","Resource Proof Rules"],["ProofValidationError","Represents an error when validating proof."],["ProofValidationMode","Specifies the validation mode that should be used for validating a `Proof`."],["Proposer","The set of roles allowed to propose recoveries. Only Primary and Recovery roles can initiate, or propose recoveries, Confirmation can’t initiate nor propose."],["PublicKey","Represents any natively supported public key."],["ReadOwnedNodesError","Represents an error when reading the owned node ids from a value."],["Reference",""],["ReplaceManifestValuesError","Represents an error when replacing manifest values."],["ResourceAddress","Represents a resource address."],["ResourceInvocation",""],["ResourceMethodAuthKey",""],["ResourceType","Represents the type of a resource."],["Role","An enum of the roles in the Access Controller component"],["RoundingMode","Defines how rounding should be done."],["SchemaPathParseError",""],["SchemaSubPath",""],["ScryptoCustomTypeExtension",""],["ScryptoCustomTypeKind","A schema for the values that a codec can decode / views as valid"],["ScryptoCustomTypeValidation",""],["ScryptoCustomValue",""],["ScryptoCustomValueKind",""],["ScryptoReceiver",""],["ScryptoValueSerializationType",""],["Signature","Represents any natively supported signature."],["SignatureWithPublicKey","Represents any natively supported signature, including public key."],["SoftCount",""],["SoftDecimal",""],["SoftResource",""],["SoftResourceOrNonFungible",""],["SoftResourceOrNonFungibleList",""],["TimeComparisonOperator",""],["TimePrecision",""],["TransactionRuntimeInvocation",""],["UpdateValidator",""],["ValidatorInvocation",""],["ValueIndexingError",""],["VaultInvocation",""],["VaultMethodAuthKey",""],["WorktopInvocation",""]],"fn":[["display_value_kind",""],["format_custom_value",""],["format_elements",""],["format_kv_entries",""],["format_scrypto_value",""],["format_tuple",""],["format_value_kind",""],["get_value_kind",""],["hash","Computes the hash digest of a message."],["match_schema_with_value",""],["package_access_rules_from_owner_badge",""],["require",""],["require_all_of",""],["require_amount",""],["require_any_of",""],["require_n_of",""],["resource_access_rules_from_owner_badge",""],["scrypto_decode","Decodes a data structure from a byte array."],["scrypto_encode","Encodes a data structure into byte array."],["serialize_custom_value",""],["serialize_schemaless_scrypto_value",""],["serialize_schemaless_scrypto_value_slice",""],["sha256","Computes the SHA-256 digest of a message."],["sha256_twice","Computes the double SHA-256 digest of a message."],["sha3","Computes the SHA3 digest of a message."]],"macro":[["access_and_or",""],["access_rule_node",""],["args","Constructs argument list for Scrypto function/method invocation."],["borrow_component","This macro converts a `ComponentAddress` into a `&Component` via the Radix Engine component subsystem."],["borrow_package","This macro creates a `&Package` from a `PackageAddress` via the Radix Engine component subsystem."],["borrow_resource_manager","This macro creates a `&ResourceManager` from a `ResourceAddress` via the Radix Engine resource subsystem."],["count",""],["debug","Logs a `DEBUG` message."],["dec","Creates a `Decimal` from literals."],["error","Logs an `ERROR` message."],["external_blueprint","Generates a bridge/stub to make package calls to a blueprint."],["external_component","Generates a bridge/stub to make cross-component calls."],["i","Creates a safe integer from literals. You must specify the type of the integer you want to create."],["import","Imports a blueprint from its ABI."],["include_abi","Includes the ABI file of a Scrypto package."],["include_code","Includes the WASM file of a Scrypto package."],["info","Logs an `INFO` message."],["pdec","Creates a `PreciseDecimal` from literals."],["resource_list",""],["rule",""],["this_package",""],["trace","Logs a `TRACE` message."],["vec","Creates a `Vec` containing the arguments."],["warn","Logs a `WARN` message."]],"mod":[["bnum_integer","Definitions of safe integers and uints."],["decimal",""],["instant",""],["integer","Definitions of safe integers and uints."],["integer_test_macros",""],["precise_decimal",""],["rounding_mode",""],["types","Scrypto custom types"],["utc_date_time",""],["vec","A contiguous growable array type with heap-allocated contents, written `Vec<T>`."]],"struct":[["AccessControllerCancelPrimaryRoleRecoveryProposalInvocation",""],["AccessControllerCancelPrimaryRoleRecoveryProposalMethodArgs",""],["AccessControllerCancelRecoveryRoleRecoveryProposalInvocation",""],["AccessControllerCancelRecoveryRoleRecoveryProposalMethodArgs",""],["AccessControllerCreateGlobalInvocation",""],["AccessControllerCreateProofInvocation",""],["AccessControllerCreateProofMethodArgs",""],["AccessControllerInitiateRecoveryAsPrimaryInvocation",""],["AccessControllerInitiateRecoveryAsPrimaryMethodArgs",""],["AccessControllerInitiateRecoveryAsRecoveryInvocation",""],["AccessControllerInitiateRecoveryAsRecoveryMethodArgs",""],["AccessControllerLockPrimaryRoleInvocation",""],["AccessControllerLockPrimaryRoleMethodArgs",""],["AccessControllerQuickConfirmPrimaryRoleRecoveryProposalInvocation",""],["AccessControllerQuickConfirmPrimaryRoleRecoveryProposalMethodArgs",""],["AccessControllerQuickConfirmRecoveryRoleRecoveryProposalInvocation",""],["AccessControllerQuickConfirmRecoveryRoleRecoveryProposalMethodArgs",""],["AccessControllerStopTimedRecoveryInvocation",""],["AccessControllerStopTimedRecoveryMethodArgs",""],["AccessControllerTimedConfirmRecoveryInvocation",""],["AccessControllerTimedConfirmRecoveryMethodArgs",""],["AccessControllerUnlockPrimaryRoleInvocation",""],["AccessControllerUnlockPrimaryRoleMethodArgs",""],["AccessRules","Method authorization rules for a component"],["AccessRulesAddAccessCheckInvocation",""],["AccessRulesGetLengthInvocation",""],["AccessRulesSetGroupAccessRuleInvocation",""],["AccessRulesSetGroupMutabilityInvocation",""],["AccessRulesSetMethodAccessRuleInvocation",""],["AccessRulesSetMethodMutabilityInvocation",""],["ArrayValue",""],["AuthZoneAssertAccessRuleInvocation",""],["AuthZoneClearInvocation",""],["AuthZoneCreateProofByAmountInvocation",""],["AuthZoneCreateProofByIdsInvocation",""],["AuthZoneCreateProofInvocation",""],["AuthZoneDrainInvocation",""],["AuthZonePopInvocation",""],["AuthZonePushInvocation",""],["BTreeMap","An ordered map based on a B-Tree."],["BTreeSet","An ordered set based on a B-Tree."],["BnumI256","Provides safe integer arithmetic."],["BnumI384","Provides safe integer arithmetic."],["BnumI512","Provides safe integer arithmetic."],["BnumI768","Provides safe integer arithmetic."],["BnumU256","Provides safe integer arithmetic."],["BnumU384","Provides safe integer arithmetic."],["BnumU512","Provides safe integer arithmetic."],["BnumU768","Provides safe integer arithmetic."],["Bucket",""],["BucketCreateProofInvocation",""],["BucketGetAmountInvocation",""],["BucketGetNonFungibleLocalIdsInvocation",""],["BucketGetResourceAddressInvocation",""],["BucketPutInvocation",""],["BucketTakeInvocation",""],["BucketTakeNonFungiblesInvocation",""],["BytesNonFungibleLocalId","Bytes, of length between 1 and 64."],["BytesValue",""],["ClockCompareCurrentTimeInvocation",""],["ClockCompareCurrentTimeMethodArgs",""],["ClockCreateInvocation",""],["ClockGetCurrentTimeInvocation",""],["ClockGetCurrentTimeMethodArgs",""],["ClockSetCurrentTimeInvocation",""],["ClockSetCurrentTimeMethodArgs",""],["ComponentClaimRoyaltyInvocation",""],["ComponentGlobalizeInvocation",""],["ComponentGlobalizeWithOwnerInvocation",""],["ComponentSetRoyaltyConfigInvocation",""],["Decimal","`Decimal` represents a 256 bit representation of a fixed-scale decimal number."],["DisplayableScryptoValueKind",""],["EcdsaSecp256k1PublicKey","Represents an ECDSA public key."],["EcdsaSecp256k1Signature","Represents an ECDSA signature."],["EcdsaSecp256k1Verifier","EcdsaSecp256k1 signature verifier."],["EddsaEd25519PublicKey","Represents an ED25519 public key."],["EddsaEd25519Signature","Represents an ED25519 signature."],["EddsaEd25519Verifier","EddsaEd25519 signature verifier."],["EnumVariant",""],["EpochManagerCreateInvocation",""],["EpochManagerCreateValidatorInvocation",""],["EpochManagerCreateValidatorMethodArgs",""],["EpochManagerGetCurrentEpochInvocation",""],["EpochManagerGetCurrentEpochMethodArgs",""],["EpochManagerNextRoundInvocation",""],["EpochManagerNextRoundMethodArgs",""],["EpochManagerSetEpochInvocation",""],["EpochManagerSetEpochMethodArgs",""],["EpochManagerUpdateValidatorInvocation",""],["EpochManagerUpdateValidatorMethodArgs",""],["Hash","Represents a 32-byte hash digest."],["HashMap","A hash map implemented with quadratic probing and SIMD lookup."],["HashSet","A hash set implemented as a `HashMap` where the value is `()`."],["I128","Provides safe integer arithmetic."],["I16","Provides safe integer arithmetic."],["I256","Provides safe integer arithmetic."],["I32","Provides safe integer arithmetic."],["I384","Provides safe integer arithmetic."],["I512","Provides safe integer arithmetic."],["I64","Provides safe integer arithmetic."],["I768","Provides safe integer arithmetic."],["I8","Provides safe integer arithmetic."],["IdentityCreateInvocation",""],["IndexedScryptoValue",""],["Instant","Represents a Unix timestamp, capturing the seconds since the unix epoch."],["IntegerNonFungibleLocalId","Unsigned integers, up to u64."],["LoggerLogInvocation",""],["ManifestBlobRef",""],["ManifestBucket",""],["ManifestProof",""],["MapValue",""],["MetadataGetInvocation",""],["MetadataSetInvocation",""],["NonFungibleGlobalId","Represents the global id of a non-fungible."],["PackageClaimRoyaltyExecutable",""],["PackageClaimRoyaltyInvocation",""],["PackagePublishInvocation",""],["PackageSetRoyaltyConfigExecutable",""],["PackageSetRoyaltyConfigInvocation",""],["PreciseDecimal","`PreciseDecimal` represents a 512 bit representation of a fixed-scale decimal number."],["Proof",""],["ProofCloneInvocation",""],["ProofGetAmountInvocation",""],["ProofGetNonFungibleLocalIdsInvocation",""],["ProofGetResourceAddressInvocation",""],["RecoveryProposal",""],["ResourceManagerBucketBurnInvocation",""],["ResourceManagerBurnInvocation",""],["ResourceManagerCreateBucketInvocation",""],["ResourceManagerCreateFungibleInvocation",""],["ResourceManagerCreateFungibleWithInitialSupplyInvocation",""],["ResourceManagerCreateNonFungibleInvocation",""],["ResourceManagerCreateNonFungibleWithInitialSupplyInvocation",""],["ResourceManagerCreateUuidNonFungibleWithInitialSupplyInvocation",""],["ResourceManagerCreateVaultInvocation",""],["ResourceManagerGetNonFungibleInvocation",""],["ResourceManagerGetResourceTypeInvocation",""],["ResourceManagerGetTotalSupplyInvocation",""],["ResourceManagerMintFungibleInvocation",""],["ResourceManagerMintNonFungibleInvocation",""],["ResourceManagerMintUuidNonFungibleInvocation",""],["ResourceManagerNonFungibleExistsInvocation",""],["ResourceManagerSetVaultAuthMutabilityInvocation",""],["ResourceManagerUpdateNonFungibleDataInvocation",""],["ResourceManagerUpdateVaultAuthInvocation",""],["RoyaltyConfig","Royalty rules"],["RoyaltyConfigBuilder",""],["RuleSet","A struct with the set of rule associated with each role - used when creating a new access controller for the initial rules and also used during recovery for proposing a rule set."],["SchemaPath","Describes a value located in some sbor given a schema for that sbor"],["ScryptoInvocation","Scrypto function/method invocation."],["ScryptoValueFormattingContext",""],["ScryptoValueVisitor","A visitor the indexes scrypto custom values."],["String","A UTF-8–encoded, growable string."],["StringNonFungibleLocalId","A string matching `[_0-9a-zA-Z]{1,64}`."],["TransactionRuntimeGenerateUuidInvocation",""],["TransactionRuntimeGetHashInvocation",""],["U128","Provides safe integer arithmetic."],["U16","Provides safe integer arithmetic."],["U256","Provides safe integer arithmetic."],["U32","Provides safe integer arithmetic."],["U384","Provides safe integer arithmetic."],["U512","Provides safe integer arithmetic."],["U64","Provides safe integer arithmetic."],["U768","Provides safe integer arithmetic."],["U8","Provides safe integer arithmetic."],["UUIDNonFungibleLocalId","UUID, v4, variant 1, big endian. See https://www.rfc-editor.org/rfc/rfc4122"],["UtcDateTime","A `UtcDateTime` represents a Unix timestamp on the UTC Calendar."],["ValidatorClaimXrdInvocation",""],["ValidatorClaimXrdMethodArgs",""],["ValidatorInit",""],["ValidatorRegisterInvocation",""],["ValidatorRegisterMethodArgs",""],["ValidatorStakeInvocation",""],["ValidatorStakeMethodArgs",""],["ValidatorUnregisterInvocation",""],["ValidatorUnregisterValidatorMethodArgs",""],["ValidatorUnstakeInvocation",""],["ValidatorUnstakeMethodArgs",""],["ValidatorUpdateAcceptDelegatedStakeInvocation",""],["ValidatorUpdateAcceptDelegatedStakeMethodArgs",""],["ValidatorUpdateKeyInvocation",""],["ValidatorUpdateKeyMethodArgs",""],["ValueFormattingContext",""],["VaultCreateProofByAmountInvocation",""],["VaultCreateProofByIdsInvocation",""],["VaultCreateProofInvocation",""],["VaultGetAmountInvocation",""],["VaultGetNonFungibleLocalIdsInvocation",""],["VaultGetResourceAddressInvocation",""],["VaultLockFeeInvocation",""],["VaultPutInvocation",""],["VaultRecallInvocation",""],["VaultRecallNonFungiblesInvocation",""],["VaultTakeInvocation",""],["VaultTakeNonFungiblesInvocation",""],["Vec","A contiguous growable array type, written as `Vec<T>`, short for ‘vector’."],["WorktopAssertContainsAmountInvocation",""],["WorktopAssertContainsInvocation",""],["WorktopAssertContainsNonFungiblesInvocation",""],["WorktopDrainInvocation",""],["WorktopPutInvocation",""],["WorktopTakeAllInvocation",""],["WorktopTakeAmountInvocation",""],["WorktopTakeNonFungiblesInvocation",""]],"trait":[["By","Trait for short hand notation for try_from().unwrap() As opposed to `try_from(x).unwrap()` this will panic if the conversion fails."],["Categorize","The `Categorize` trait marks a rust type as having a fixed value kind for SBOR encoding/decoding."],["CheckedAdd",""],["CheckedDiv",""],["CheckedMul",""],["CheckedNeg",""],["CheckedPow",""],["CheckedRem",""],["CheckedSub",""],["Decode","A data structure that can be decoded from a byte array using SBOR."],["Encode","A data structure that can be serialized into a byte array using SBOR."],["FromPrimitive","A generic trait for converting a number to a value."],["FromPublicKey",""],["FromStr","Parse a value from a string"],["IsNonAutoGeneratedNonFungibleLocalId","Marks the rust type that represents a non-fungible id, of non-auto-generated kind (i.e. String, Integer and Bytes)."],["IsNonFungibleLocalId","Marks the rust type that represents a non-fungible id, of any kind (i.e. String, Integer, Bytes and UUID)."],["Min",""],["NonFungibleData","Represents the data structure of a non-fungible."],["One","Defines a multiplicative identity element for `Self`."],["Pow","Binary operator for raising a value to a power."],["PrimIntExt",""],["ScryptoCategorize",""],["ScryptoDecode",""],["ScryptoEncode",""],["SerializableScryptoValue",""],["Signed","Useful functions for signed numbers (i.e. numbers that can be negative)."],["ToOwned","A generalization of `Clone` to borrowed data."],["ToPrimitive","A generic trait for converting a value to a number."],["ToString","A trait for converting a value to a `String`."],["Truncate",""],["Zero","Defines an additive identity element for `Self`."]],"type":[["ScryptoDecoder",""],["ScryptoEncoder",""],["ScryptoSchema",""],["ScryptoTypeKind",""],["ScryptoValue",""],["ScryptoValueKind",""]]};