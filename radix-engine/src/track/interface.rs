use crate::errors::RuntimeError;
use crate::types::*;
use radix_engine_interface::types::*;
use radix_engine_store_interface::db_key_mapper::SubstateKeyContent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallbackError<E, C> {
    Error(E),
    CallbackError(C),
}

impl<E> CallbackError<E, RuntimeError> {
    pub fn to_runtime_error<F: FnOnce(E) -> RuntimeError>(self, f: F) -> RuntimeError {
        match self {
            CallbackError::Error(e) => f(e),
            CallbackError::CallbackError(c) => c,
        }
    }
}

impl<E, C> CallbackError<E, C> {
    pub fn map<N, F: FnOnce(E) -> N>(self, f: F) -> CallbackError<N, C> {
        match self {
            CallbackError::Error(e) => CallbackError::Error(f(e)),
            CallbackError::CallbackError(c) => CallbackError::CallbackError(c),
        }
    }
}

pub type NodeSubstates = BTreeMap<PartitionNumber, BTreeMap<SubstateKey, IndexedScryptoValue>>;

pub enum TrackedSubstateInfo {
    New,
    Updated,
    Unmodified,
}

/// Represents the interface between Radix Engine and Track.
///
/// In practice, we will likely end up with only one implementation.
///
/// The trait here is for formalizing the interface and intended user flow.
pub trait SubstateStore {
    /// Inserts a node into the substate store.
    ///
    /// Clients must ensure the `node_id` is new and unique; otherwise, the behavior is undefined.
    ///
    /// # Panics
    /// - If the partition is invalid
    fn create_node<E, F: FnMut(StoreAccess) -> Result<(), E>>(
        &mut self,
        node_id: NodeId,
        node_substates: NodeSubstates,
        on_store_access: &mut F,
    ) -> Result<(), E>;

    fn get_tracked_substate_info(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        substate_key: &SubstateKey,
    ) -> TrackedSubstateInfo;

    fn read_substate(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        substate_key: &SubstateKey,
    ) -> Option<&IndexedScryptoValue> {
        self.get_substate(node_id, partition_num, substate_key, &mut |_| -> Result<
            (),
            (),
        > {
            Ok(())
        })
        .unwrap()
    }

    fn get_substate<E, F: FnMut(StoreAccess) -> Result<(), E>>(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        substate_key: &SubstateKey,
        on_store_access: &mut F,
    ) -> Result<Option<&IndexedScryptoValue>, E>;

    /// Inserts a substate into the substate store.
    ///
    /// Clients must ensure the `node_id`/`partition_num` is a node which has been created; otherwise, the behavior
    /// is undefined.
    fn set_substate<E, F: FnMut(StoreAccess) -> Result<(), E>>(
        &mut self,
        node_id: NodeId,
        partition_num: PartitionNumber,
        substate_key: SubstateKey,
        substate_value: IndexedScryptoValue,
        on_store_access: &mut F,
    ) -> Result<(), E>;

    fn force_write(
        &mut self,
        node_id: &NodeId,
        partition_num: &PartitionNumber,
        substate_key: &SubstateKey,
    );

    /// Deletes a substate from the substate store.
    ///
    /// Clients must ensure the `node_id`/`partition_num` is a node which has been created;
    /// Clients must ensure this isn't called on a virtualized partition;
    /// Otherwise, the behavior is undefined.
    ///
    /// Returns tuple of substate and boolean which is true for the first database access.
    fn remove_substate<E, F: FnMut(StoreAccess) -> Result<(), E>>(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        substate_key: &SubstateKey,
        on_store_access: &mut F,
    ) -> Result<Option<IndexedScryptoValue>, E>;

    /// Returns Substate Keys of maximum count for a given partition.
    ///
    /// Clients must ensure that the SubstateKeyContent which the partition is
    /// associated with is passed in. The returned SubstateKeys are guaranteed to be of
    /// this type.
    /// Otherwise, behavior is undefined.
    ///
    /// Returns list of substate keys and database access info
    fn scan_keys<K: SubstateKeyContent, E, F: FnMut(StoreAccess) -> Result<(), E>>(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        count: u32,
        on_store_access: &mut F,
    ) -> Result<Vec<SubstateKey>, E>;

    /// Removes substates of maximum count for a given partition.
    ///
    /// Clients must ensure that the SubstateKeyContent which the partition is
    /// associated with is passed in. The returned SubstateKeys are guaranteed to be of
    /// this type.
    /// Otherwise, behavior is undefined.
    ///
    /// Returns list of removed substates with their associated keys and values, as well as database access info
    fn drain_substates<K: SubstateKeyContent, E, F: FnMut(StoreAccess) -> Result<(), E>>(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        count: u32,
        on_store_access: &mut F,
    ) -> Result<Vec<(SubstateKey, IndexedScryptoValue)>, E>;

    /// Returns tuple of substate vector and boolean which is true for the first database access.
    fn scan_sorted_substates<E, F: FnMut(StoreAccess) -> Result<(), E>>(
        &mut self,
        node_id: &NodeId,
        partition_num: PartitionNumber,
        count: u32,
        on_store_access: &mut F,
    ) -> Result<Vec<(SortedU16Key, IndexedScryptoValue)>, E>;

    /// Note: unstable interface, for intent transaction tracker only
    fn delete_partition(&mut self, node_id: &NodeId, partition_num: PartitionNumber);

    /// Return the commit info
    fn get_commit_info(&mut self) -> StoreCommitInfo;
}

#[derive(Debug, Clone, Copy)]
pub struct CanonicalPartition {
    pub node_id: NodeId,
    pub partition_number: PartitionNumber,
}

#[derive(Debug, Clone)]
pub struct CanonicalSubstateKey {
    pub node_id: NodeId,
    pub partition_number: PartitionNumber,
    pub substate_key: SubstateKey,
}

impl CanonicalSubstateKey {
    pub fn of(partition: CanonicalPartition, substate_key: SubstateKey) -> Self {
        Self {
            node_id: partition.node_id,
            partition_number: partition.partition_number,
            substate_key,
        }
    }
}

impl CanonicalSubstateKey {
    pub fn logical_size(&self) -> usize {
        self.node_id.as_bytes().len()
            + 1
            + match &self.substate_key {
                SubstateKey::Field(_) => 1,
                SubstateKey::Map(k) => k.len(),
                SubstateKey::Sorted(k) => 2 + k.1.len(),
            }
    }
}

#[derive(Debug, Clone)]
pub enum StoreAccess {
    /// Some substate was read from database.
    ReadFromDb(CanonicalSubstateKey, usize),
    /// Non-existent substate was read from database.
    ReadFromDbNotFound(CanonicalSubstateKey),
    /// A new entry has been added to track
    /// System limits how many items that can be tracked.
    NewEntryInTrack(CanonicalSubstateKey, usize),
}

impl StoreAccess {
    pub fn node_id(&self) -> NodeId {
        match self {
            StoreAccess::ReadFromDb(key, _)
            | StoreAccess::ReadFromDbNotFound(key)
            | StoreAccess::NewEntryInTrack(key, _) => key.node_id,
        }
    }
}

pub type StoreCommitInfo = Vec<StoreCommit>;

#[derive(Debug, Clone)]
pub enum StoreCommit {
    Insert {
        canonical_substate_key: CanonicalSubstateKey,
        size: usize,
    },
    Update {
        canonical_substate_key: CanonicalSubstateKey,
        size: usize,
        old_size: usize,
    },
    Delete {
        canonical_substate_key: CanonicalSubstateKey,
        old_size: usize,
    },
}

impl StoreCommit {
    pub fn node_id(&self) -> NodeId {
        match self {
            StoreCommit::Insert {
                canonical_substate_key,
                ..
            }
            | StoreCommit::Update {
                canonical_substate_key,
                ..
            }
            | StoreCommit::Delete {
                canonical_substate_key,
                ..
            } => canonical_substate_key.node_id,
        }
    }

    pub fn logical_size_increase(&self) -> usize {
        match self {
            StoreCommit::Insert {
                canonical_substate_key,
                size,
                ..
            } => canonical_substate_key.logical_size() + *size,
            StoreCommit::Update { size, old_size, .. } => {
                if *size > *old_size {
                    *size - *old_size
                } else {
                    0
                }
            }
            StoreCommit::Delete { .. } => 0, // TODO: refund?
        }
    }
}
