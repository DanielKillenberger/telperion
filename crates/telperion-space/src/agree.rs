//! Two trees compared by lineage (fn-254 R4, check 4): every node is
//! matched by its draw's key, the same node under any path, and the
//! trees agree where few nodes go unmatched and the matched ones stand,
//! size and thicken alike. Two paths that sum in different orders hold
//! the f32 tree to this, never to the bit.
use crate::structure::Structure;
use std::collections::HashMap;

/// How far two trees part: the nodes of the first, those either holds
/// that the other does not, and the largest difference of a matched
/// node's place (a share of the first tree's height), scale, and radius
/// (a share of the first tree's thickest).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Agreement {
    pub nodes: usize,
    pub unmatched: usize,
    pub tip: f64,
    pub scale: f64,
    pub radius: f64,
}

/// Two growth paths that group the tree-wide sums differently (the whole
/// tree, a coupled pass, a partition, the stepped executor): measured
/// over the oak, beech and spruce at 24, seeds 1 to 6 (fn-254), no node
/// unmatched, tips 1.03e-5 of the height apart, scales 3.6e-4, radii
/// 5.1e-5 of the thickest, the beech and spruce, which read no light, to
/// the bit. Bound at about twice the measured.
pub const PATHS: Agreement = Agreement {
    nodes: 0,
    unmatched: 0,
    tip: 2e-5,
    scale: 6e-4,
    radius: 1e-4,
};

/// `a` and `b` compared node by node (module doc).
pub fn agreement(a: &Structure, b: &Structure) -> Agreement {
    let nodes = |t: &Structure| {
        let mut by: HashMap<u64, Vec<_>> = HashMap::new();
        for p in t.axes.iter().flat_map(|x| &x.phytomers) {
            by.entry(p.key).or_default().push(*p);
        }
        by
    };
    let (of_a, mut of_b) = (nodes(a), nodes(b));
    let all = a.axes.iter().flat_map(|x| &x.phytomers);
    let height = all.clone().fold(1e-9f64, |h, p| h.max(p.tip.z));
    let thickest = all.fold(1e-12f64, |r, p| r.max(p.radius));
    let mut out = Agreement::default();
    for (key, xs) in &of_a {
        out.nodes += xs.len();
        let ys = of_b.remove(key).unwrap_or_default();
        out.unmatched += xs.len().abs_diff(ys.len());
        for (x, y) in xs.iter().zip(&ys) {
            out.tip = out.tip.max((x.tip - y.tip).length() / height);
            out.scale = out.scale.max((x.scale - y.scale).abs());
            out.radius = out.radius.max((x.radius - y.radius).abs() / thickest);
        }
    }
    out.unmatched += of_b.values().map(Vec::len).sum::<usize>();
    out
}
