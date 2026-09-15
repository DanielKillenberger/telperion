//! The curtain under its own weight. A hanging shoot leaves its limb with
//! fn-37's droop and then turns toward straight down along its run, by the
//! share of the way there its table states, spread as an arc steepest at the
//! wood that bears it. Neutral is fn-37's straight rod, byte for byte.
//! No device is needed; this is the core's own arithmetic.
use std::collections::BTreeMap;
use std::f64::consts::FRAC_PI_2;

use telperion_core::{
    blend, branching,
    presets::{Family, Preset},
    tree::NodeKind,
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
/// The tree each shipped table grew with the row forced to its neutral,
/// photographed when fn-44 landed. The four species `tests/identity.rs` pins
/// were byte-identical across the change, which is what says these are the
/// trees the row never reached and not merely the trees it grows today.
/// The birch's was re-recorded once when round 6b shortened its pendulous
/// length from 3.5 m to 2.5 m: its own table moved, not the neutral law.
/// And once more at the merge with fn-38, whose clump puts the same table on
/// two stems; every other table's neutral stood. Round 7 lifts that birch's
/// crown base, which moves its tree with the row at any value. fn-47's birch
/// states a pendulous variation over a 3 m pendulous length, which caps a
/// shoot that does not sag as surely as it sets the run of one that does, and
/// a deeper, finer outline for its shell.
const NEUTRAL: [u64; 7] = [
    17046456021212146411,
    14986275773972546726,
    12735573889651776723,
    12852486373694172527,
    4895668800915172604,
    12471405148157309180,
    14199367530911903060,
];

/// The share of its angle to straight down a shoot still carries after
/// running `travelled` of its pendulous length, stated here independently of
/// the generator: a cubic ease-out that has spent all of `sag` by the end of
/// the run, and holds what it reached beyond it.
fn remaining(sag: f64, travelled: f64, pendulous: f64) -> f64 {
    let left = 1.0 - (travelled / pendulous).clamp(0.0, 1.0);
    1.0 - sag * (1.0 - left * left * left)
}

fn preset(id: &str) -> Family {
    let mut f = Preset::from_id(id).expect("preset identity").parameters();
    f.skeleton.seed = SEED;
    f
}

/// FNV-1a over node positions and parent links, the pattern the pins use.
fn skeleton(f: &Family) -> u64 {
    grown(f).0
}

/// The tree one family grows: the hash of its skeleton and how many nodes it
/// carries, which is what says a walk bends a curtain rather than replacing it.
fn grown(f: &Family) -> (u64, usize) {
    let tree = branching::generate(&f.skeleton, f.radii)
        .expect("the row grows a tree")
        .tree;
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
    (hash, tree.nodes.len())
}

/// One step of one run: the angle between the step and straight down, and the
/// arc length from the shoot's departure to the end of the step.
struct Step {
    angle: f64,
    along: f64,
    lowest: f64,
}

/// A family cooked to hang one readable shoot: the birch's curtain over a
/// crown that leaves it somewhere to fall, its axes straight so that what
/// bends a shoot is its own weight and not the habit's crookedness, and the
/// wood that bears leaves thin enough that a hanging run is subdivided into an
/// arc rather than the two or three chords the shipped table gives it. It
/// stands on one stem: fn-38's clump splits the birch's wood between two, and
/// wood that thin bears its hanging shoots in one or two chords again. Its
/// internodes stay the 36 mm the arc was read on: round 7's 60 mm lays a run's
/// stations unevenly, and a longer stride turns further in one step while the
/// turn per metre still eases off the whole way down.
fn hanging(sag: f64, pendulous: f64) -> Family {
    let mut f = preset("silver-birch");
    f.skeleton.habit.stems = 1;
    f.skeleton.twigs.twig.internode_length = 0.036;
    f.skeleton.habit.crookedness = 0.0;
    f.skeleton.envelope.crown_base = 0.02;
    // Room for the curtain to finish its runs: a shoot that hangs its whole
    // pendulous length is six or seven nodes, and a crown cut off at its cap
    // is a crown of half-grown shoots.
    f.skeleton.growth.max_nodes = Some(150_000);
    f.skeleton.twigs.hang = 1.0;
    f.skeleton.twigs.pendulous_length = pendulous;
    f.skeleton.twigs.twig.bearing_diameter = 0.015;
    f.skeleton.twigs.sag = sag;
    // One length for every strand: the arc is read against the pendulous
    // length, and fn-47's variation gives each shoot its own.
    f.skeleton.twigs.pendulous_variation = 0.0;
    f
}

/// The runs that both hang and ran their whole pendulous length, which are the
/// shoots the row makes a promise about.
fn whole_runs(f: &Family, pendulous: f64) -> Vec<Vec<Step>> {
    hanging_runs(f)
        .into_iter()
        .filter(|r| r.len() >= 6 && r.last().is_some_and(|s| (s.along - pendulous).abs() < 1e-6))
        .collect()
}

/// Every run of a grown family that departs downward, base to tip. A run is
/// one branch identity's own chain of nodes; a chain broken by the frontier's
/// ordering is dropped rather than read as a shoot.
fn hanging_runs(f: &Family) -> Vec<Vec<Step>> {
    let tree = branching::generate(&f.skeleton, f.radii)
        .expect("the row grows a tree")
        .tree;
    let mut chains: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        if n.kind == NodeKind::Branch {
            chains.entry(n.branch).or_default().push(i);
        }
    }
    chains
        .into_values()
        .filter(|nodes| {
            nodes
                .windows(2)
                .all(|w| tree.nodes[w[1]].parent == Some(w[0] as u32))
        })
        .filter_map(|nodes| {
            let mut along = 0.0;
            let steps: Vec<Step> = nodes
                .iter()
                .map(|&i| {
                    let node = &tree.nodes[i];
                    let parent =
                        &tree.nodes[node.parent.expect("a run node has a parent") as usize];
                    let step = node.position - parent.position;
                    along += step.length();
                    Step {
                        angle: (-step.normalized().y).clamp(-1.0, 1.0).acos(),
                        along,
                        lowest: node.position.y,
                    }
                })
                .collect();
            steps
                .first()
                .is_some_and(|s| s.angle < FRAC_PI_2)
                .then_some(steps)
        })
        .collect()
}

#[test]
fn every_shipped_preset_at_neutral_sag_is_the_tree_the_row_never_reached() {
    for (id, pin) in IDS.iter().zip(NEUTRAL) {
        let mut f = preset(id);
        f.skeleton.twigs.sag = 0.0;
        assert_eq!(skeleton(&f), pin, "{id}: a neutral sag moved the tree");
    }
}

/// Prints the neutral pin of every shipped table, to re-record after a value
/// change: `cargo test --release --test sag -- --ignored --nocapture`.
#[test]
#[ignore]
fn print_neutral() {
    for id in IDS {
        let mut f = preset(id);
        f.skeleton.twigs.sag = 0.0;
        println!("NEUTRAL {id} {}", skeleton(&f));
    }
}

#[test]
fn a_sag_never_reaches_a_table_that_hangs_nothing() {
    // The row is the weight on a hanging shoot, so a table whose shoots do not
    // hang grows the tree it grew before the row existed, whatever it states.
    for id in IDS {
        let plain = preset(id);
        if plain.skeleton.twigs.hang > 0.0 {
            continue;
        }
        let pin = skeleton(&plain);
        for sag in [0.25, 0.5, 1.0] {
            let mut loud = preset(id);
            loud.skeleton.twigs.sag = sag;
            assert_eq!(skeleton(&loud), pin, "{id}: sag {sag} moved a dry tree");
        }
    }
}

#[test]
fn the_same_seed_and_row_grow_the_same_curtain() {
    for sag in [0.0, 0.4, 1.0] {
        assert_eq!(
            skeleton(&hanging(sag, 3.5)),
            skeleton(&hanging(sag, 3.5)),
            "sag {sag}: one seed grew two curtains"
        );
        let mut other = hanging(sag, 3.5);
        other.skeleton.seed = SEED + 1;
        if sag > 0.0 {
            assert_ne!(
                skeleton(&hanging(sag, 3.5)),
                skeleton(&other),
                "sag {sag}: two seeds grew one curtain"
            );
        }
    }
}

#[test]
fn a_walk_of_the_sag_row_bends_the_curtain_continuously() {
    // The row blends linearly and every step of the walk bends the curtain
    // further down without changing the tree's kind; the two ends of the walk
    // are a rod and a shoot hanging nearly vertical.
    const PENDULOUS: f64 = 0.3;
    let (dry, wet) = (hanging(0.0, PENDULOUS), hanging(1.0, PENDULOUS));
    let mut descent = Vec::new();
    let mut skeletons = Vec::new();
    let mut nodes = Vec::new();
    for step in 0..=10 {
        let t = f64::from(step) / 10.0;
        let family = blend::families(&dry, &wet, t).expect("the walk validates");
        assert!(
            (family.skeleton.twigs.sag - t).abs() < 1e-12,
            "step {step} walked the row to {} and not {t}",
            family.skeleton.twigs.sag
        );
        // Read per shoot, as the share of its departure angle it still
        // carries: which shoots the floor leaves whole changes along the walk,
        // and a share does not care which ones they are.
        let kept: Vec<f64> = whole_runs(&family, PENDULOUS)
            .iter()
            .map(|r| {
                let first = &r[0];
                let departure = first.angle / remaining(t, first.along, PENDULOUS);
                r.last().expect("a run has a step").angle / departure
            })
            .collect();
        assert!(!kept.is_empty(), "step {step} hung no whole shoot");
        for kept in &kept {
            assert!(
                (kept - (1.0 - t)).abs() < 1e-6,
                "step {step} left a shoot carrying {kept:.6} of its departure \
                 angle, not the {:.6} the blended row states",
                1.0 - t
            );
        }
        let (hash, count) = grown(&family);
        descent.push(kept.iter().sum::<f64>() / kept.len() as f64);
        nodes.push(count);
        skeletons.push(hash);
    }
    for step in 1..skeletons.len() {
        assert_ne!(
            skeletons[step - 1],
            skeletons[step],
            "step {step} grew the tree the step before it did"
        );
        assert!(
            descent[step] <= descent[step - 1] + 1e-9,
            "step {step} hung the curtain higher than the step before it"
        );
        let (a, b) = (nodes[step - 1], nodes[step]);
        assert!(
            a.abs_diff(b) * 4 <= a.max(b),
            "step {step} changed the tree's kind: {a} nodes to {b}"
        );
    }
    assert!(
        descent[0] > 0.999 && descent[10] < 0.001,
        "the ends of the walk are not a rod and a shoot hanging vertical: \
         {:.4} to {:.4} of the departure angle",
        descent[0],
        descent[10]
    );
}

/// What the arc itself does to a shoot: its own file, beside the row's.
#[path = "sag/arc.rs"]
mod arc;
