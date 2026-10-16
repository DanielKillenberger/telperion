//! Where a fine root stands on its bearer (fn-209 F1 task 6): its first
//! axis's node, which the coarse tree seats its stand on.
use crate::structure::{Axis, Origin};

/// A fine root's first axis as its bearer made it: the node it stands
/// on.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Made {
    pub node: usize,
}

impl Made {
    pub fn of(axis: &Axis) -> Self {
        let Origin::Lateral { node, .. } = axis.origin else {
            unreachable!("a fine root is a lateral bud")
        };
        Self { node }
    }
}
