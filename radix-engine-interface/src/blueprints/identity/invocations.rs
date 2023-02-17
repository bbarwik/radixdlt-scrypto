use crate::blueprints::resource::*;
use crate::*;
use sbor::rust::collections::BTreeMap;
use sbor::rust::fmt::Debug;
use sbor::rust::string::String;
use scrypto_abi::BlueprintAbi;

pub struct IdentityAbi;

impl IdentityAbi {
    pub fn blueprint_abis() -> BTreeMap<String, BlueprintAbi> {
        BTreeMap::new()
    }
}

pub const IDENTITY_BLUEPRINT: &str = "Identity";

pub const IDENTITY_CREATE_IDENT: &str = "create";

#[derive(Debug, Clone, Eq, PartialEq, ScryptoCategorize, ScryptoEncode, ScryptoDecode)]
pub struct IdentityCreateInput {
    pub access_rule: AccessRule,
}