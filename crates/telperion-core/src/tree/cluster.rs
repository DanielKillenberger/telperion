//! Where a short shoot bears its leaves (fn-223): an engine axis whose age
//! states a cluster bears its leaves at its own tip, its last drawn node,
//! and none along its length. The conversion keeps one record a tip beside
//! the tree, as `blooms` are kept; the expansion seats the leaves. Empty on
//! every tree whose ages state none.

/// One short shoot's tip and its cluster.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cluster {
    /// The axis's last drawn node: its key keys every draw of the cluster.
    pub node: u32,
    /// Whole leaves in the cluster (`PaState::cluster_leaves`).
    pub leaves: u32,
    /// Radians either side of the tip's level bearing the leaves fan
    /// across (`PaState::cluster_spread`).
    pub spread: f64,
}

/// A cluster's stream key: the tip's key under the registry's `CLUSTER`
/// (fn-223 R4), so its leaves draw apart from any run's.
pub fn cluster_key(tip: u32) -> u32 {
    super::mix32(tip ^ telperion_space::lineage_keys::CLUSTER as u32)
}
