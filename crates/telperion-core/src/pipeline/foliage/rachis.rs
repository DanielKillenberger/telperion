//! A frond's axis as wood (fn-283): its leaf base, petiole and rachis, one
//! run from where the frond leaves the stem through every station its
//! leaflets stand at.
//!
//! The axis reads the fronds the leaflets are placed from
//! (`rosette::placed`) and the stations they stand at (`leaflet::station`,
//! `leaflet::rachis`), so every leaflet and spine base is a node of it and a
//! young frond's axis ends at its last segment's base. It is hung beside the
//! retained leaf bases, after the radius solve, so it never thickens the
//! stem; a node of wood that is no stem, so no foliage clothes it.
use super::{
    leaflet::{rachis, station},
    rosette::{self, frame},
    CanopyParams,
};
use crate::{
    math::Vec3,
    pipeline::{branching, space::FINEST},
    tree::{Node, NodeKind, Tree},
};

/// One frond's axis: the stem node it leaves from, and its points and
/// radii from the frond's insertion on the stem's axis to its last station.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Rachis {
    pub(crate) at: usize,
    pub(crate) points: Vec<Vec3>,
    pub(crate) radii: Vec<f64>,
}

/// Whether this table draws any axis: a width, a rosette, and leaflets
/// strung along a rachis. Each frond is judged again by its own draws.
pub(crate) fn draws(p: &CanopyParams) -> bool {
    p.rachis_width > 0. && rosette::bearing(p) && p.leaflet_count > 1. && p.rachis_length > 0.
}

/// Every frond's axis on this tree, in the order the leaflets are placed,
/// `blade` the leaf element's length a young frond's stations read. None
/// for a frond of one leaflet (its blade stands on the stem) or of no
/// rachis length (its leaflets all stand where it leaves the stem).
pub(crate) fn rachises(tree: &Tree, p: &CanopyParams, blade: f64) -> Vec<Rachis> {
    if !draws(p) {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut stem = None;
    for placed in rosette::placed(tree, p, blade) {
        let (frond, c, count) = (placed.frond, placed.table, placed.count);
        if count < 2 || c.rachis_length <= 0. {
            continue;
        }
        let apex = placed.rosette.apex;
        if stem.as_ref().is_none_or(|(of, _, _)| *of != apex) {
            let (nodes, depth) = branching::polyline(tree, apex);
            stem = Some((apex, nodes, depth));
        }
        let (_, nodes, depth) = stem.as_ref().unwrap();
        let below = frond.at.distance(placed.rosette.at);
        // A stem of one node has no depth along it to search.
        let at = match nodes.len() {
            1 => nodes[0],
            _ => branching::along(tree, nodes, depth, below).0,
        };
        // Its width scaled by the frond's size, tapering by its station alone,
        // so a station has one girth at every age.
        let r0 = p.rachis_width / 2. * c.rachis_length / placed.apex.rachis_length;
        let (lift, _) = frame(frond.heading);
        let shares =
            std::iter::once(0.).chain((0..count).map(|k| station(k, count, &c, frond.split).0));
        let (points, radii) = shares
            .map(|t| {
                let point = rachis(frond.at, frond.heading, lift, &c, t);
                (point, (r0 * (1. - t)).max(FINEST))
            })
            .unzip();
        out.push(Rachis { at, points, radii });
    }
    out
}

/// Each axis as a run of its own appended to `nodes`, whose first stands
/// at node `start` of the tree: wood that is no stem, each node's radius
/// its distal edge and its start radius the one before.
pub(crate) fn hang(axes: &[Rachis], start: usize, nodes: &mut Vec<Node>) {
    for axis in axes {
        let first = (start + nodes.len()) as u32;
        let base = axis.radii[0];
        let mut parent = axis.at as u32;
        let mut before = base;
        for (&position, &radius) in axis.points.iter().zip(&axis.radii) {
            nodes.push(Node {
                position,
                parent: Some(parent),
                radius,
                start_radius: before,
                base_radius: base,
                branch: first,
                kind: NodeKind::Branch,
                stem: false,
                ..Node::root()
            });
            parent = (start + nodes.len() - 1) as u32;
            before = radius;
        }
    }
}
