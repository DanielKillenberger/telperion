//! The curtain hangs below the crown. A hanging shoot may fall past the
//! shell's lower surface, the drop row's share of the way down to a clearance
//! above the ground, and only under the crown's footprint; nothing else leaves
//! the shell. Neutral is the shell holding the curtain, byte for byte.
//! No device is needed; this is the core's own arithmetic.
use std::collections::BTreeMap;

use telperion_core::{
    blend,
    branching::{self, in_curtain_band, Specimen},
    params,
    presets::{Family, Preset},
    tree::{NodeKind, Tree},
    twigs::TwigParams,
    Error,
};

const SEED: u32 = 7;
/// Every shipped table, pinned or not.
const IDS: [&str; 7] = [
    "ordinary",
    "oregon-white-oak",
    "norway-spruce",
    "european-beech",
    "silver-birch",
    "telperion",
    "laurelin",
];
/// The tree each shipped table grew with the row forced to its neutral: the
/// trees `tests/identity.rs` and the strands' neutral pins recorded before the
/// row existed, the birch's being its round-8b table's, which is what says the
/// row at zero reaches nothing.
/// The beech's is its identity skeleton, taken at the merges into fn-34's
/// integration branch (round 11's, then fn-50's): its own table moved, not
/// the row. The birch's
/// is re-recorded at the same merges, when fn-48's unequal clump joined its
/// table.
const NEUTRAL: [u64; 7] = [
    17046456021212146411,
    14986275773972546726,
    12735573889651776723,
    8478216101743105815,
    15114279530833391460,
    12471405148157309180,
    14199367530911903060,
];
/// The cooked curtain's clearance, below its crown base of 2.16 m.
const CLEARANCE: f64 = 1.0;
/// Slack on the containment the tests read, as the shell's own tests take it.
const TOLERANCE: f64 = 1e-8;

fn preset(id: &str) -> Family {
    let mut f = Preset::from_id(id).expect("preset identity").parameters();
    f.skeleton.seed = SEED;
    f
}

fn tree(f: &Family) -> Tree {
    branching::generate(&f.skeleton, f.radii)
        .expect("the row grows a tree")
        .tree
}

/// FNV-1a over node positions and parent links, the pattern the pins use.
fn skeleton(tree: &Tree) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for n in tree.nodes.iter().skip(1) {
        for byte in [n.position.x, n.position.y, n.position.z]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .chain(n.parent.expect("non-root parent").to_le_bytes())
        {
            hash = (hash ^ u64::from(byte)).wrapping_mul(1099511628211);
        }
    }
    hash
}

/// The birch's curtain over a crown that leaves it room to fall, cooked as the
/// sag and strand tests cook it: one stem, straight axes, fine leaf-bearing
/// wood, a full hang and a full sag, strands of a metre and a half or less.
fn curtain(drop: f64) -> Family {
    let mut f = preset("silver-birch");
    f.skeleton.habit.stems = 1;
    f.skeleton.habit.crookedness = 0.0;
    f.skeleton.envelope.crown_base = 0.12;
    f.skeleton.envelope.irregularity = 0.15;
    f.skeleton.envelope.lobe_scale = 0.45;
    f.skeleton.growth.max_nodes = Some(150_000);
    f.skeleton.twigs.twig.internode_length = 0.036;
    f.skeleton.twigs.twig.bearing_diameter = 0.015;
    f.skeleton.twigs.hang = 1.0;
    f.skeleton.twigs.pendulous_length = 1.5;
    f.skeleton.twigs.sag = 1.0;
    f.skeleton.twigs.pendulous_variation = 0.5;
    f.skeleton.twigs.curtain_drop = drop;
    f.skeleton.twigs.curtain_clearance = CLEARANCE;
    f
}

/// Whether a node is where the local law may put one for `f`: inside the
/// shell, or in the band a hanging shoot may fall into below it.
fn held(f: &Family, twigs: &TwigParams, p: telperion_core::math::Vec3) -> bool {
    let (envelope, seed) = (f.skeleton.envelope, f.skeleton.seed);
    envelope.contains(p, TOLERANCE, seed) || in_curtain_band(&envelope, twigs, seed, p, TOLERANCE)
}

/// The nodes past the crossover that lie outside the shell, by position.
fn fallen<'a>(f: &Family, tree: &'a Tree) -> Vec<&'a telperion_core::tree::Node> {
    let (envelope, seed) = (f.skeleton.envelope, f.skeleton.seed);
    tree.nodes[tree.crossover..]
        .iter()
        .filter(|n| !envelope.contains(n.position, TOLERANCE, seed))
        .collect()
}

#[test]
fn every_shipped_preset_at_neutral_drop_is_the_tree_the_row_never_reached() {
    for (id, pin) in IDS.iter().zip(NEUTRAL) {
        let mut f = preset(id);
        f.skeleton.twigs.curtain_drop = 0.0;
        let grown = tree(&f);
        assert_eq!(skeleton(&grown), pin, "{id}: a neutral drop moved the tree");
        // And at zero the old invariant holds exactly: nothing leaves the shell.
        assert!(
            fallen(&f, &grown).is_empty(),
            "{id} seed {SEED}: a node left the shell at a neutral drop"
        );
    }
}

/// Prints the neutral pin of every shipped table, to re-record after a value
/// change: `cargo test --release --test drop -- --ignored --nocapture`.
#[test]
#[ignore]
fn print_neutral() {
    for id in IDS {
        let mut f = preset(id);
        f.skeleton.twigs.curtain_drop = 0.0;
        println!("NEUTRAL {id} {}", skeleton(&tree(&f)));
    }
}

#[test]
fn a_drop_never_reaches_a_table_that_hangs_nothing() {
    for id in IDS {
        let plain = preset(id);
        if plain.skeleton.twigs.hang > 0.0 {
            continue;
        }
        let mut loud = preset(id);
        loud.skeleton.twigs.curtain_drop = 1.0;
        loud.skeleton.twigs.curtain_clearance = 0.0;
        assert_eq!(
            skeleton(&tree(&loud)),
            skeleton(&tree(&plain)),
            "{id}: a drop moved a tree that hangs nothing"
        );
    }
}

#[test]
fn the_rows_ride_the_wire_and_are_refused_by_name_off_their_rails() {
    let mut wire = params::metadata(&preset("silver-birch"));
    wire["skeleton"]["twigs"]["curtainDrop"] = serde_json::json!(0.35);
    wire["skeleton"]["twigs"]["curtainClearance"] = serde_json::json!(1.25);
    let parsed = params::parse(&wire).expect("the wire carries the rows");
    assert_eq!(parsed.skeleton.twigs.curtain_drop, 0.35);
    assert_eq!(parsed.skeleton.twigs.curtain_clearance, 1.25);
    for (row, bad, name) in [
        ("curtainDrop", -0.001, "curtain drop"),
        ("curtainDrop", 1.001, "curtain drop"),
        ("curtainClearance", -0.001, "curtain clearance"),
        ("curtainClearance", 5.001, "curtain clearance"),
    ] {
        let mut wire = wire.clone();
        wire["skeleton"]["twigs"][row] = serde_json::json!(bad);
        let f = params::parse(&wire).expect("the wire parses the row");
        assert_eq!(
            branching::generate(&f.skeleton, f.radii).err(),
            Some(Error::InvalidInput(name)),
            "{row} of {bad} grew a tree"
        );
    }
}

#[test]
fn a_hanging_curtain_falls_into_its_band_at_half_and_whole_drop() {
    let mut lowest = Vec::new();
    for drop in [0.5, 1.0] {
        let f = curtain(drop);
        let grown = tree(&f);
        let twigs = f.skeleton.twigs;
        for node in &grown.nodes[grown.crossover..] {
            assert!(
                held(&f, &twigs, node.position),
                "seed {SEED} drop {drop}: {:?} is outside the shell and the band",
                node.position
            );
        }
        let below = fallen(&f, &grown);
        assert!(
            below.len() * 20 >= grown.nodes.len() - grown.crossover,
            "seed {SEED} drop {drop}: only {} nodes fell past the shell",
            below.len()
        );
        lowest.push(below.iter().map(|n| n.position.y).fold(f64::MAX, f64::min));
        if drop == 1.0 {
            // The whole drop falls further than half of it would let it.
            let half = TwigParams {
                curtain_drop: 0.5,
                ..twigs
            };
            let deeper = below.iter().filter(|n| !held(&f, &half, n.position));
            assert!(
                deeper.count() * 25 >= below.len(),
                "seed {SEED}: the whole drop fell no further than half of it"
            );
        }
    }
    assert!(
        lowest[1] < lowest[0] - 0.25,
        "seed {SEED}: the curtain's lowest node stood at {:.2} m at half drop and {:.2} m at whole",
        lowest[0],
        lowest[1]
    );
}

#[test]
fn only_hanging_wood_falls_and_it_falls_from_the_shell() {
    // What leaves the shell is the local law's own wood, and every node of it
    // hangs from a node inside the shell or from one that fell before it: the
    // band is entered from the crown above it and never from outside the band.
    let f = curtain(1.0);
    let grown = tree(&f);
    let twigs = f.skeleton.twigs;
    let (envelope, seed) = (f.skeleton.envelope, f.skeleton.seed);
    for node in fallen(&f, &grown) {
        assert!(
            node.kind != NodeKind::Structural,
            "seed {SEED}: a structural node fell past the shell"
        );
        let parent =
            grown.nodes[node.parent.expect("a fallen node has a parent") as usize].position;
        assert!(
            envelope.contains(parent, TOLERANCE, seed)
                || in_curtain_band(&envelope, &twigs, seed, parent, TOLERANCE),
            "seed {SEED}: {:?} fell from {parent:?}, outside the crown",
            node.position
        );
    }
    // And a curtain that does not hang drops nothing, whatever the row says.
    let mut dry = curtain(1.0);
    dry.skeleton.twigs.hang = 0.0;
    let still = tree(&dry);
    assert!(
        fallen(&dry, &still).is_empty(),
        "seed {SEED}: a shoot that does not hang fell past the shell"
    );
}

#[test]
fn no_shoot_falls_below_the_clearance_nor_the_clearance_above_the_crown_base() {
    let f = curtain(1.0);
    let base = f.skeleton.envelope.height * f.skeleton.envelope.crown_base;
    let lowest = |f: &Family| {
        let grown = tree(f);
        grown.nodes[grown.crossover..]
            .iter()
            .map(|n| n.position.y)
            .fold(f64::MAX, f64::min)
    };
    let low = lowest(&f);
    assert!(
        low >= CLEARANCE - 1e-9 && low < base - 0.5,
        "seed {SEED}: the curtain's lowest node stands at {low:.3} m over a clearance of \
         {CLEARANCE} m and a crown base of {base:.2} m"
    );
    // A clearance above the crown's base is the crown's base.
    let mut high = curtain(1.0);
    high.skeleton.twigs.curtain_clearance = 5.0;
    let low = lowest(&high);
    assert!(
        low >= base - 1e-9,
        "seed {SEED}: a clearance above the crown base let a shoot to {low:.3} m under {base:.2} m"
    );
}

#[test]
fn one_seed_is_one_band_however_it_grows() {
    let f = curtain(1.0);
    assert_eq!(
        tree(&f),
        tree(&f),
        "seed {SEED}: one table grew two curtains"
    );
    // Built at an age in one call, month by month or in uneven pieces, the
    // same nodes fall past the same shell to the same places.
    const AGE: f64 = 25.0;
    let seed = f.skeleton.seed;
    let band = |s: &Specimen| -> BTreeMap<u64, [u64; 3]> {
        let (envelope, tree) = (f.skeleton.envelope, s.tree());
        tree.nodes[tree.crossover..]
            .iter()
            .filter(|n| !envelope.contains(n.position, TOLERANCE, seed))
            .map(|n| {
                let p = n.position;
                let bits = [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
                (n.identity.birth_order(), bits)
            })
            .collect()
    };
    let mut aged = f.clone();
    aged.age = AGE;
    let built = band(&Specimen::build(&aged).expect("the curtain builds"));
    assert!(
        built.len() >= 100,
        "seed {SEED}: at {AGE} years only {} nodes fell past the shell",
        built.len()
    );
    let mut young = f.clone();
    young.age = 0.0;
    let mut monthly = Specimen::build(&young).expect("the seedling builds");
    for _ in 0..(AGE as usize * 12) {
        // Each month's births lie in the crown that month has, or its band.
        let born = |n: &telperion_core::tree::Node| n.identity.birth_order();
        let last = monthly.tree().nodes.iter().map(born).max();
        monthly.advance(1.0 / 12.0).expect("a month grows");
        let live = monthly.envelope();
        let tree = monthly.tree();
        let births = tree.nodes[tree.crossover..]
            .iter()
            .filter(|n| Some(born(n)) > last);
        for n in births {
            let p = n.position;
            assert!(
                live.contains(p, TOLERANCE, seed)
                    || in_curtain_band(&live, &f.skeleton.twigs, seed, p, TOLERANCE),
                "seed {SEED} at {:.2} years: {p:?} is outside the live shell and its band",
                monthly.age()
            );
        }
    }
    let mut uneven = Specimen::build(&young).expect("the seedling builds");
    for piece in [0.3, 4.7, 0.01, 7.99, 12.0] {
        uneven.advance(piece).expect("a piece grows");
    }
    for (name, other) in [("monthly", band(&monthly)), ("uneven", band(&uneven))] {
        assert!(
            other == built,
            "seed {SEED}: the {name} build dropped {} nodes where the one-call build dropped {}",
            other.len(),
            built.len()
        );
    }
}

#[test]
fn a_walk_of_the_drop_row_lowers_the_curtain_continuously() {
    let (held_in, dropped) = (curtain(0.0), curtain(1.0));
    let mut depths = Vec::new();
    let mut hashes = Vec::new();
    let mut nodes = Vec::new();
    for step in 0..=10 {
        let t = f64::from(step) / 10.0;
        let family = blend::families(&held_in, &dropped, t).expect("the walk validates");
        let row = family.skeleton.twigs.curtain_drop;
        assert!(
            (row - t).abs() < 1e-12,
            "step {step} walked the row to {row}, not {t}"
        );
        let grown = tree(&family);
        let twigs = family.skeleton.twigs;
        for node in &grown.nodes[grown.crossover..] {
            assert!(
                held(&family, &twigs, node.position),
                "seed {SEED} step {step}: {:?} is outside the shell and the band",
                node.position
            );
        }
        // How far below the crown base the curtain's lowest tenth reaches.
        let mut heights: Vec<f64> = grown.nodes[grown.crossover..]
            .iter()
            .map(|n| n.position.y)
            .collect();
        heights.sort_by(f64::total_cmp);
        depths
            .push(heights[..heights.len() / 10].iter().sum::<f64>() / (heights.len() / 10) as f64);
        hashes.push(skeleton(&grown));
        nodes.push(grown.nodes.len());
    }
    for step in 1..=10 {
        assert_ne!(
            hashes[step - 1],
            hashes[step],
            "step {step} grew the tree the step before it did"
        );
        assert!(
            depths[step] < depths[step - 1],
            "seed {SEED} step {step}: the curtain's lowest tenth rose, {:.4} m to {:.4} m",
            depths[step - 1],
            depths[step]
        );
        let (a, b) = (nodes[step - 1], nodes[step]);
        assert!(
            a.abs_diff(b) * 4 <= a.max(b),
            "step {step} changed the tree's kind: {a} nodes to {b}"
        );
    }
}
