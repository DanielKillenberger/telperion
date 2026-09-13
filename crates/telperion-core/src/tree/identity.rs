//! Birth order is independent of slot reuse and compact node-vector storage.
use slotmap::{new_key_type, Key};

new_key_type! { pub(crate) struct NodeKey; }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeIdentity {
    pub(crate) birth: u64,
    pub(crate) key: NodeKey,
}
impl Default for NodeIdentity {
    fn default() -> Self {
        Self {
            birth: u64::MAX,
            key: NodeKey::null(),
        }
    }
}
impl NodeIdentity {
    /// Monotone within the specimen, including after slots have been reused.
    pub fn birth_order(self) -> u64 {
        self.birth
    }
    #[cfg(test)]
    pub(crate) fn to_le_bytes(self) -> [u8; 16] {
        let mut bytes = [0; 16];
        bytes[..8].copy_from_slice(&self.birth.to_le_bytes());
        bytes[8..].copy_from_slice(&self.key.data().as_ffi().to_le_bytes());
        bytes
    }
}
