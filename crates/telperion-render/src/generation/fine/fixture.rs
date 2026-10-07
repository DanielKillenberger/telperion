//! The probe's fixture (fn-212 tasks 4 and 5): a shipped preset grown by
//! the pipeline as the CPU reference, the same tree settled with its fine
//! roots on the device, and those roots drawn there from each one's
//! attachment as the pipeline drew its bearer.
use super::device::Device;
use super::dress::{dress, Dressed, Dresser, Stub, StubNode};
use super::roots::{Kernels, Tables};
use crate::Gpu;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor::{self, Expansion};
use telperion_core::presets::Family;
use telperion_core::tree::Tree;
use telperion_space::{settle, ChainTables, GeometryTables, Plan, Request};

pub(super) struct Fixture {
    pub family: Family,
    pub reference: Expansion,
    /// The reference's nodes the device draws: its fine roots' seams'
    /// subtrees, those laid on the CPU aside.
    pub fine: Vec<bool>,
    pub landed: usize,
    pub dress_ms: f64,
}

fn f3(v: Vec3) -> [f32; 3] {
    [v.x as f32, v.y as f32, v.z as f32]
}

/// Every node in a seam's subtree of a fine root not in `landed` (by its
/// first axis's lineage).
fn fine_nodes(tree: &Tree, landed: &HashSet<u64>) -> Vec<bool> {
    let mut fine = vec![false; tree.nodes.len()];
    let kept = tree
        .attachments
        .iter()
        .filter(|a| !landed.contains(&a.lineage));
    for &s in kept.flat_map(|a| &a.seams) {
        fine[s as usize] = true;
    }
    for i in 1..tree.nodes.len() {
        let p = tree.nodes[i].parent.unwrap() as usize;
        fine[i] |= fine[p];
    }
    fine
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

/// The preset `id` at `age` and `seed` built by the pipeline, settled and
/// drawn on the device; `then` reads both.
pub(super) fn with_dressed<R>(
    gpu: &Gpu,
    k: &Kernels,
    d: &Dresser,
    id: &str,
    (age, seed): (u32, u32),
    then: impl FnOnce(&Fixture, &Dressed) -> R,
) -> R {
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    family.age = f64::from(age);
    family.skeleton.seed = seed;
    let reference = executor::grow(&family).unwrap().expansion().unwrap();
    let tree = reference.tree();
    let distance = distances(tree);
    let species = family.space.species.clone().unwrap();
    let request = Request {
        age,
        seed: u64::from(seed),
        budget: 20_000_000,
        light: family.space.light,
    };
    let chain = ChainTables::new(&species, age).unwrap();
    let geo = GeometryTables::new(&species, age).unwrap();
    let tables = Tables::new(gpu, Arc::new(chain), &geo);
    let mut dev = Device::new(gpu, k, &tables, 1 << 16).sown_from(64);
    let (settled, _) = settle(&species, request, Plan::default(), &mut dev).unwrap();
    dev.wait();
    let attached: HashMap<u64, u32> = (tree.attachments.iter())
        .map(|a| (a.lineage, a.node))
        .collect();
    let lineage = |r: usize| settled.at[r].map(|i| settled.structure.axes[i].lineage);
    let landed: HashSet<usize> = settled.grounded.iter().copied().collect();
    let stubs: Vec<Stub> = (0..settled.at.len())
        .map(|r| {
            let at = lineage(r)
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
        .collect();
    let landed_lineages: HashSet<u64> = landed.iter().filter_map(|&r| lineage(r)).collect();
    let fine = fine_nodes(tree, &landed_lineages);
    let d0 = std::time::Instant::now();
    let dressed = dress(gpu, d, &dev, &stubs);
    let dress_ms = d0.elapsed().as_secs_f64() * 1e3;
    let fx = Fixture {
        family,
        reference,
        fine,
        landed: landed.len(),
        dress_ms,
    };
    then(&fx, &dressed)
}
