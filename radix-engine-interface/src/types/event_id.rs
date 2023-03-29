use super::TypedModuleId;
use crate::ScryptoSbor;
use radix_engine_common::types::*;
use sbor::rust::string::String;
use sbor::LocalTypeIndex;

/// Identifies a specific event schema emitter by some emitter RENode.
///
/// This type is an identifier uses to identify the schema of events emitted by an RENode of some
/// [`NodeId`]. With this identifier, the schema for an event can be queried, obtained, and with
/// it, the SBOR encoded event data can be decoded and understood.
///
/// It is important to note that application events are always emitted by an RENode, meaning that
/// there is always an emitter of some [`NodeId`].
#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct EventTypeIdentifier(pub Emitter, pub LocalTypeIndex);

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum Emitter {
    // (Node id, module id, blueprint name)
    Function(NodeId, TypedModuleId, String),
    // (Node id, module id)
    Method(NodeId, TypedModuleId),
}