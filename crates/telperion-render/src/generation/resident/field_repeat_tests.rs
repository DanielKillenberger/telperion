//! fn-320: a tree grown twice on the device grows the same field. The
//! driver fuses a product into the add that takes it on some runs of a
//! program and not others, so the marked arithmetic is kept apart
//! (`cycle/kept.wgsl`, the evidence's FUSED.md).
use super::field_tests::{family, generator};
use std::collections::BTreeMap;
use telperion_core::field::Grid;

/// The grid's frame and answers, hashed to the bit.
fn hash(grid: &Grid) -> u64 {
    let f = grid.frame;
    let mut words = vec![f.cell.to_bits(), f.origin.x.to_bits(), f.origin.y.to_bits()];
    words.push(f.origin.z.to_bits());
    for a in &grid.cells {
        let flags = u64::from(a.wood) | u64::from(a.foliage) << 1;
        let limb = u64::from(a.limb.unwrap_or(u32::MAX));
        let [radius, leaves] = [a.wood_radius, a.leaves].map(f64::to_bits);
        words.extend([flags, radius, leaves, limb, a.flowers.to_bits()]);
    }
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in words.iter().flat_map(|w| w.to_le_bytes()) {
        hash = (hash ^ u64::from(byte)).wrapping_mul(0x100_0000_01b3);
    }
    hash
}

/// The trees that parted most often before the marks were kept apart: the
/// birch on every repeat on F4, the oak on about one in ten on this head.
#[test]
fn the_worst_cases_grow_the_same_field_each_time() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    for (id, seed) in [("silver-birch", 5678), ("oregon-white-oak", 5678)] {
        let f = family(id, seed);
        let grow = || pollster::block_on(gen.grow_field(&f, 24, None)).expect("read out");
        let first = grow();
        for k in 1..8 {
            assert!(grow() == first, "{id} {seed}: grow {k} parted from the first");
        }
    }
}

/// fn-320 R2, a measurement: every shipped preset at seeds 1 and 5678, 30
/// grows in one process, each a byte the first's. Where `FIELD_REPEAT_PIN`
/// names a file, the hashes are written there the first time and held to
/// it after, so a second process is held to the first.
#[test]
#[ignore = "a measurement: every preset's field on 30 repeats"]
fn every_preset_grows_the_same_field_on_thirty_repeats() {
    let _held = super::super::whole_tree();
    let Some(gen) = generator() else {
        return;
    };
    let mut got = BTreeMap::new();
    let mut parted = Vec::new();
    for &(_, id, ..) in telperion_core::presets::CATALOGUE {
        for seed in [1, 5678] {
            let f = family(id, seed);
            let grow = || pollster::block_on(gen.grow_field(&f, 24, None)).expect("read out");
            let first = grow();
            let off = (1..30).filter(|_| grow() != first).count();
            println!("FIELD_REPEAT {id} {seed} {:016x}, {off} of 29 parted", hash(&first));
            if off > 0 {
                parted.push(format!("{id} {seed}: {off}"));
            }
            got.insert(format!("{id} {seed}"), format!("{:016x}", hash(&first)));
        }
    }
    assert!(parted.is_empty(), "the field parted: {parted:?}");
    let Ok(pin) = std::env::var("FIELD_REPEAT_PIN") else {
        return;
    };
    let lines: Vec<String> = got.iter().map(|(k, v)| format!("{k} {v}")).collect();
    match std::fs::read_to_string(&pin) {
        Ok(was) => assert_eq!(was, lines.join("\n"), "another process grew another field"),
        Err(_) => std::fs::write(&pin, lines.join("\n")).expect("the pin written"),
    }
}
