use super::*;
use sbor::rust::collections::IndexSet;
use sbor::*;

pub type ScryptoTypeKind<L> = TypeKind<ScryptoCustomTypeId, ScryptoCustomTypeKind<L>, L>;
pub type ScryptoSchema = Schema<ScryptoCustomTypeExtension>;

/// A schema for the values that a codec can decode / views as valid
#[derive(Debug, Clone, PartialEq, Eq, TypeId, Encode, Decode)]
pub enum ScryptoCustomTypeKind<L: SchemaTypeLink> {
    // Global address types
    PackageAddress,
    ComponentAddress,
    ResourceAddress,
    SystemAddress,

    // Other Engine types
    Own,
    NonFungibleAddress,
    Component,
    KeyValueStore { key_type: L, value_type: L },

    // Manifest types
    Blob,
    Bucket,
    Proof,
    Expression,

    // Uninterpreted
    Hash,
    EcdsaSecp256k1PublicKey,
    EcdsaSecp256k1Signature,
    EddsaEd25519PublicKey,
    EddsaEd25519Signature,
    Decimal,
    PreciseDecimal,
    NonFungibleId,
}

impl<L: SchemaTypeLink> CustomTypeKind<L> for ScryptoCustomTypeKind<L> {
    type CustomTypeId = ScryptoCustomTypeId;
    type CustomTypeExtension = ScryptoCustomTypeExtension;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScryptoCustomTypeValidation {}

impl CustomTypeValidation for ScryptoCustomTypeValidation {}

pub enum ScryptoCustomTypeExtension {}

impl CustomTypeExtension for ScryptoCustomTypeExtension {
    type CustomTypeId = ScryptoCustomTypeId;
    type CustomTypeKind<L: SchemaTypeLink> = ScryptoCustomTypeKind<L>;
    type CustomTypeValidation = ScryptoCustomTypeValidation;

    fn linearize_type_kind(
        type_kind: Self::CustomTypeKind<GlobalTypeId>,
        schemas: &IndexSet<TypeHash>,
    ) -> Self::CustomTypeKind<LocalTypeIndex> {
        match type_kind {
            ScryptoCustomTypeKind::PackageAddress => ScryptoCustomTypeKind::PackageAddress,
            ScryptoCustomTypeKind::ComponentAddress => ScryptoCustomTypeKind::ComponentAddress,
            ScryptoCustomTypeKind::ResourceAddress => ScryptoCustomTypeKind::ResourceAddress,
            ScryptoCustomTypeKind::SystemAddress => ScryptoCustomTypeKind::SystemAddress,
            ScryptoCustomTypeKind::Component => ScryptoCustomTypeKind::Component,
            ScryptoCustomTypeKind::KeyValueStore {
                key_type,
                value_type,
            } => ScryptoCustomTypeKind::KeyValueStore {
                key_type: resolve_local_type_ref(schemas, &key_type),
                value_type: resolve_local_type_ref(schemas, &value_type),
            },
            ScryptoCustomTypeKind::Bucket => ScryptoCustomTypeKind::Bucket,
            ScryptoCustomTypeKind::Proof => ScryptoCustomTypeKind::Proof,
            ScryptoCustomTypeKind::Own => ScryptoCustomTypeKind::Own,
            ScryptoCustomTypeKind::Expression => ScryptoCustomTypeKind::Expression,
            ScryptoCustomTypeKind::Blob => ScryptoCustomTypeKind::Blob,
            ScryptoCustomTypeKind::NonFungibleAddress => ScryptoCustomTypeKind::NonFungibleAddress,
            ScryptoCustomTypeKind::Hash => ScryptoCustomTypeKind::Hash,
            ScryptoCustomTypeKind::EcdsaSecp256k1PublicKey => {
                ScryptoCustomTypeKind::EcdsaSecp256k1PublicKey
            }
            ScryptoCustomTypeKind::EcdsaSecp256k1Signature => {
                ScryptoCustomTypeKind::EcdsaSecp256k1Signature
            }
            ScryptoCustomTypeKind::EddsaEd25519PublicKey => {
                ScryptoCustomTypeKind::EddsaEd25519PublicKey
            }
            ScryptoCustomTypeKind::EddsaEd25519Signature => {
                ScryptoCustomTypeKind::EddsaEd25519Signature
            }
            ScryptoCustomTypeKind::Decimal => ScryptoCustomTypeKind::Decimal,
            ScryptoCustomTypeKind::PreciseDecimal => ScryptoCustomTypeKind::PreciseDecimal,
            ScryptoCustomTypeKind::NonFungibleId => ScryptoCustomTypeKind::NonFungibleId,
        }
    }

    fn resolve_custom_well_known_type(
        well_known_index: u8,
    ) -> Option<TypeData<Self::CustomTypeKind<LocalTypeIndex>, LocalTypeIndex>> {
        let (name, custom_type_schema) = match well_known_index {
            PACKAGE_ADDRESS_ID => ("PackageAddress", ScryptoCustomTypeKind::PackageAddress),
            COMPONENT_ADDRESS_ID => ("ComponentAddress", ScryptoCustomTypeKind::ComponentAddress),
            RESOURCE_ADDRESS_ID => ("ResourceAddress", ScryptoCustomTypeKind::ResourceAddress),
            SYSTEM_ADDRESS_ID => ("SystemAddress", ScryptoCustomTypeKind::SystemAddress),

            OWN_ID => ("Own", ScryptoCustomTypeKind::Own),
            NON_FUNGIBLE_ADDRESS_ID => (
                "NonFungibleAddress",
                ScryptoCustomTypeKind::NonFungibleAddress,
            ),
            COMPONENT_ID => ("Component", ScryptoCustomTypeKind::Component),

            BLOB_ID => ("Blob", ScryptoCustomTypeKind::Blob),
            BUCKET_ID => ("Bucket", ScryptoCustomTypeKind::Bucket),
            PROOF_ID => ("Proof", ScryptoCustomTypeKind::Proof),
            EXPRESSION_ID => ("Expression", ScryptoCustomTypeKind::Expression),

            HASH_ID => ("Hash", ScryptoCustomTypeKind::Hash),
            ECDSA_SECP256K1_PUBLIC_KEY_ID => (
                "EcdsaSecp256k1PublicKey",
                ScryptoCustomTypeKind::EcdsaSecp256k1PublicKey,
            ),
            ECDSA_SECP256K1_SIGNATURE_ID => (
                "EcdsaSecp256k1Signature",
                ScryptoCustomTypeKind::EcdsaSecp256k1Signature,
            ),
            EDDSA_ED25519_PUBLIC_KEY_ID => (
                "EddsaEd25519PublicKey",
                ScryptoCustomTypeKind::EddsaEd25519PublicKey,
            ),
            EDDSA_ED25519_SIGNATURE_ID => (
                "EddsaEd25519Signature",
                ScryptoCustomTypeKind::EddsaEd25519Signature,
            ),
            DECIMAL_ID => ("Decimal", ScryptoCustomTypeKind::Decimal),
            PRECISE_DECIMAL_ID => ("PreciseDecimal", ScryptoCustomTypeKind::PreciseDecimal),
            NON_FUNGIBLE_ID_ID => ("NonFungibleId", ScryptoCustomTypeKind::NonFungibleId),
            _ => return None,
        };

        Some(TypeData::named_no_child_names(
            name,
            TypeKind::Custom(custom_type_schema),
        ))
    }
}

use well_known_scrypto_types::*;

mod well_known_scrypto_types {
    use super::*;

    pub const PACKAGE_ADDRESS_ID: u8 = TYPE_PACKAGE_ADDRESS;
    pub const COMPONENT_ADDRESS_ID: u8 = TYPE_COMPONENT_ADDRESS;
    pub const RESOURCE_ADDRESS_ID: u8 = TYPE_RESOURCE_ADDRESS;
    pub const SYSTEM_ADDRESS_ID: u8 = TYPE_SYSTEM_ADDRESS;

    pub const OWN_ID: u8 = TYPE_OWN;
    pub const NON_FUNGIBLE_ADDRESS_ID: u8 = TYPE_NON_FUNGIBLE_ADDRESS;
    pub const COMPONENT_ID: u8 = TYPE_COMPONENT;
    // We skip KeyValueStore because it has generic parameters

    pub const BLOB_ID: u8 = TYPE_BLOB;
    pub const BUCKET_ID: u8 = TYPE_BUCKET;
    pub const PROOF_ID: u8 = TYPE_PROOF;
    pub const EXPRESSION_ID: u8 = TYPE_EXPRESSION;

    pub const HASH_ID: u8 = TYPE_HASH;
    pub const ECDSA_SECP256K1_PUBLIC_KEY_ID: u8 = TYPE_ECDSA_SECP256K1_PUBLIC_KEY;
    pub const ECDSA_SECP256K1_SIGNATURE_ID: u8 = TYPE_ECDSA_SECP256K1_SIGNATURE;
    pub const EDDSA_ED25519_PUBLIC_KEY_ID: u8 = TYPE_EDDSA_ED25519_PUBLIC_KEY;
    pub const EDDSA_ED25519_SIGNATURE_ID: u8 = TYPE_EDDSA_ED25519_SIGNATURE;
    pub const DECIMAL_ID: u8 = TYPE_DECIMAL;
    pub const PRECISE_DECIMAL_ID: u8 = TYPE_PRECISE_DECIMAL;
    pub const NON_FUNGIBLE_ID_ID: u8 = TYPE_NON_FUNGIBLE_ID;
}
