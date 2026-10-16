//! A grown tree's hash for the bit-equality proof of fn-198 task 2: every
//! node's every field by its bits, then the rest of the tree.
use std::hash::{DefaultHasher, Hash, Hasher};
use telperion_core::tree::Tree;

pub fn tree(tree: &Tree) -> u64 {
    let mut h = DefaultHasher::new();
    tree.nodes.len().hash(&mut h);
    for n in &tree.nodes {
        format!("{:?}{:?}{:?}", n.identity, n.shoot, n.kind).hash(&mut h);
        let p = n.position;
        for v in [p.x, p.y, p.z, n.radius, n.start_radius, n.base_radius] {
            v.to_bits().hash(&mut h);
        }
        (n.parent, n.branch, n.stem, n.codominant.map(f64::to_bits)).hash(&mut h);
    }
    tree.crossover.hash(&mut h);
    format!("{:?}{:?}", tree.diagnostics, tree.sections).hash(&mut h);
    for p in &tree.pipe {
        (p[0].to_bits(), p[1].to_bits()).hash(&mut h);
    }
    h.finish()
}
