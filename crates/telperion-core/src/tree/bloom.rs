//! Where a tree flowers (fn-216, option B): the engine's ages say which of
//! their nodes may bear an inflorescence, and the conversion keeps one
//! record a node beside the tree, as `sections` is kept. The expansion draws
//! what flowers there. Empty on every tree whose ages bear none.
use crate::math::Vec3;

/// One node that may bear an inflorescence.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct Bloom {
    /// The node it stands on; its identity keys every draw of it.
    pub node: u32,
    /// The share of its age's nodes that flower (`PaState::flowering`).
    pub share: f64,
    /// The unit side the node's bud faces, the way an axillary
    /// inflorescence leaves it.
    pub side: Vec3,
}
