//! The settled node table is the same in every word whatever runs it
//! (R2, R2a): over fresh device contexts and over the hierarchy's block
//! caps the growth before it ran under, built plain. And the CPU's
//! `grow_cycle`, retiring subtrees as it grows, returns the reference
//! tree the settle is held to (SETTLE-PLAN.md risk 5).
use super::build::Tables;
use super::build_grow::{grow_now as grow, Sizes};
use super::grown_tests::{device, grown, Grown, YOUNG};
use super::hier_scan::read_words;
use super::settle_device::Kernels;
use super::settle_run::settle_now as settle;
use super::settle_tests::constants;
use telperion_core::presets::Preset;
use telperion_space::SettleTo;

/// `g`'s node table, laid, on a fresh context from `sizes`.
fn fresh(g: &Grown, sizes: Sizes) -> Option<Vec<u32>> {
    let (gpu, kit) = device()?;
    let k = Kernels::new(&gpu);
    let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
    let (b, ..) = grow(&gpu, &kit, &t, &g.tables, (sizes, g.request.age)).expect("a build");
    let s = settle(&gpu, (&kit, &k), (&b, &t), (&constants(g), SettleTo::Laid)).expect("a settle");
    let mut words = s.error.to_vec();
    words.extend(read_words(&gpu, &s.nt, 0, s.table.words).expect("a readback"));
    Some(words)
}

#[test]
fn the_node_table_is_the_same_on_every_context_and_block_cap() {
    let _held = super::super::whole_tree();
    let mut differ = Vec::new();
    for (preset, name, age) in [(Preset::OregonWhiteOak, "oregon-white-oak", 9), (Preset::NorwaySpruce, "norway-spruce", 12)] {
        let g = grown(preset, age, 42);
        let Some(first) = fresh(&g, Sizes::least()) else {
            return;
        };
        let mut runs: Vec<(String, Sizes)> = (2..=10).map(|k| (format!("context {k}"), Sizes::least())).collect();
        for cap in [64, 128, 1024] {
            runs.push((format!("block cap {cap}"), Sizes { block_cap: cap, ..Sizes::least() }));
        }
        for (label, sizes) in runs {
            let again = fresh(&g, sizes).expect("a device");
            let parted = again.iter().zip(&first).filter(|(a, b)| a != b).count() + again.len().abs_diff(first.len());
            println!("{name} {age}, {label}: {} words, {parted} differ", first.len());
            if parted > 0 {
                differ.push(format!("{name} {label}"));
            }
        }
    }
    assert!(differ.is_empty(), "the node table differs: {differ:?}");
}

#[test]
fn young_trees_grown_with_retirement_are_the_reference_the_settle_is_held_to() {
    let mut differ = Vec::new();
    for (preset, name, age) in YOUNG {
        let g = grown(preset, age, 42);
        let (same, retired) = telperion_space::grow_cycle_is_the_reference(&g.species, g.request).expect("a tree");
        println!("{name} {age}: {retired} retired, the reference's tree {same}");
        if !same {
            differ.push(name);
        }
    }
    assert!(differ.is_empty(), "grow_cycle is not the reference: {differ:?}");
}
