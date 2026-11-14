//! A relay's join drawn as a node of its parent (fn-244 host decision 4):
//! where the engine's girth says it joins (`telperion_space::Join`), `t`
//! of the way along the span of its parent's node, at the parent's radius
//! there with the relay whole. The parent's run goes on through it into
//! the relay; the stopped module's head beyond starts from its own wood.
use super::up;
use crate::math::Vec3;
use telperion_space::{Axis, Join, Origin, Structure};

/// A join's share at the far end of its span: the parent's node.
pub(super) const ONE: f32 = 1.0;

/// Each axis's relays and their joins, by the parent's node and then
/// along the span, so the drawing meets them in order.
pub(super) fn by_parent(structure: &Structure) -> Vec<Vec<(usize, Join)>> {
    let mut joins = vec![Vec::new(); structure.axes.len()];
    for (i, axis) in structure.axes.iter().enumerate() {
        // A relay that grew no phytomer carries no wood: nothing joins.
        if axis.phytomers.is_empty() {
            continue;
        }
        if let (Origin::Relay { parent, .. }, Some(join)) = (axis.origin, axis.joined()) {
            joins[parent].push((i, join));
        }
    }
    for list in &mut joins {
        list.sort_by(|(_, a), (_, b)| (a.node, a.t).partial_cmp(&(b.node, b.t)).expect("a share"));
    }
    joins
}

/// What the drawing draws a node of: its place, radius, identity, key and
/// the wood grown into it; the relay it is the join of, if it is one.
pub(super) struct Draw {
    pub tip: Vec3,
    pub radius: f64,
    pub identity: u64,
    pub key: u32,
    pub piece: f64,
    pub relay: Option<usize>,
}

impl Draw {
    /// Phytomer `place` of `axis`: its own node.
    pub fn of(axis: &Axis, place: usize, piece: f64) -> Self {
        let p = &axis.phytomers[place];
        Self {
            tip: up(p.tip),
            radius: p.radius,
            identity: p.key(),
            key: crate::tree::node_key(axis.lineage, place),
            piece,
            relay: None,
        }
    }
}

/// The join of relay `i` on `parent`, inside the span of its node: `t` of
/// the way along it, at the radius with the relay whole, and `t` of the
/// span's grown wood.
pub(super) fn draw(parent: &Axis, (i, relay): (usize, &Axis), join: Join, piece: f64) -> Draw {
    let t = f64::from(join.t);
    let end = up(parent.phytomers[join.node].tip);
    let start = up(match join.node {
        0 => parent.base,
        k => parent.phytomers[k - 1].tip,
    });
    let key = telperion_space::join_key(relay.phytomers[0].key());
    Draw {
        tip: start + (end - start) * t,
        radius: join.below,
        identity: key,
        key: crate::tree::node_key(key, 0),
        piece: piece * t,
        relay: Some(i),
    }
}
