//! Each fine root as the device draws it (fn-212 tasks 4 and 7): from its
//! attachment in the pipeline's tree, as the pipeline drew its bearer.
use super::device::Device;
use super::dress::{Stub, StubNode};
use std::collections::{HashMap, HashSet};
use telperion_core::math::Vec3;
use telperion_core::tree::Tree;

fn f3(v: Vec3) -> [f32; 3] {
    [v.x as f32, v.y as f32, v.z as f32]
}

/// Each node's distance along the wood from the root, as the sweep reads it.
fn distances(tree: &Tree) -> Vec<f64> {
    let mut d = vec![0.0; tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let p = n.parent.unwrap() as usize;
        d[i] = d[p] + tree.nodes[p].position.distance(n.position);
    }
    d
}

/// Each fine root as the device draws it (`dress`): from its attachment
/// in `tree` (by its first axis's lineage, none where shed), as the
/// pipeline drew its bearer, from its origin as laid on the device, and
/// whether it was laid on the CPU (`landed`, by root).
pub(super) fn stubs(
    tree: &Tree,
    dev: &Device,
    lineages: &[Option<u64>],
    landed: &HashSet<usize>,
) -> Vec<Stub> {
    let distance = distances(tree);
    let attached: HashMap<u64, u32> = (tree.attachments.iter())
        .map(|a| (a.lineage, a.node))
        .collect();
    (0..lineages.len())
        .map(|r| {
            let at = lineages[r]
                .and_then(|l| attached.get(&l))
                .map(|&n| n as usize);
            let node = at.map(|i| {
                let n = &tree.nodes[i];
                let parent = n
                    .parent
                    .map_or(n.position, |p| tree.nodes[p as usize].position);
                StubNode {
                    position: f3(n.position),
                    parent: f3(parent),
                    radius: n.radius as f32,
                    start: n.start_radius as f32,
                    base: n.base_radius as f32,
                    frame: tree.frames.get(i).copied().unwrap_or_default(),
                    branch: n.branch,
                    key: tree.key(i),
                }
            });
            let o = dev.origin(r);
            Stub {
                node,
                origin: [o.x as f32, o.y as f32, o.z as f32],
                landed: landed.contains(&r),
                distance: at.map_or(0.0, |i| distance[i] as f32),
            }
        })
        .collect()
}
