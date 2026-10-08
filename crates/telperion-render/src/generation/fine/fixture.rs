//! The probe's fixture (fn-212 tasks 4 and 5): a shipped preset grown by
//! the pipeline as the CPU reference, the same tree settled with its fine
//! roots on the device, and those roots drawn there from each one's
//! attachment as the pipeline drew its bearer.
use super::device::Device;
use super::dress::{dress, Dressed, Dresser};
use super::roots::{Kernels, Tables};
use super::stubs::stubs;
use crate::Gpu;
use std::collections::HashSet;
use std::sync::Arc;
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
    let species = family.space.species.clone().unwrap();
    let request = Request {
        age,
        seed: u64::from(seed),
        budget: 20_000_000,
        light: family.space.light,
    };
    let chain = ChainTables::new(&species, age).unwrap();
    let geo = GeometryTables::new(&species, age).unwrap();
    let tables = Tables::new(gpu, Arc::new(chain), &geo).unwrap();
    let mut dev = Device::new(gpu, k, &tables, 1 << 16).sown_from(64);
    let (settled, _) =
        pollster::block_on(settle(&species, request, Plan::default(), &mut dev)).unwrap();
    dev.wait();
    let lineages: Vec<Option<u64>> = (settled.at.iter())
        .map(|a| a.map(|i| settled.structure.axes[i].lineage))
        .collect();
    let landed: HashSet<usize> = settled.grounded.iter().copied().collect();
    let stubs = stubs(tree, &dev, &lineages, &landed);
    let landed_lineages: HashSet<u64> = landed.iter().filter_map(|&r| lineages[r]).collect();
    let fine = fine_nodes(tree, &landed_lineages);
    let d0 = std::time::Instant::now();
    let dressed = dress(gpu, d, &dev, &stubs).unwrap();
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
