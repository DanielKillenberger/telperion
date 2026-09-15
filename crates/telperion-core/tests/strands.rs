//! The curtain's strands, each its own length. A hanging shoot runs the
//! pendulous length times one minus the variation row times a draw in 0 to 1
//! keyed by the shoot's own key and the seed, so a curtain ends in a ragged
//! hem rather than a level one and one seed is one curtain however it grows.
//! Neutral is fn-44's curtain of one length, byte for byte.
//! No device is needed; this is the core's own arithmetic.
use std::collections::BTreeMap;
use std::f64::consts::FRAC_PI_2;

use telperion_core::{
    blend,
    branching::{self, Specimen},
    params,
    presets::{Family, Preset},
    tree::{NodeKind, Tree},
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
/// The tree each shipped table grew with the row forced to its neutral,
/// photographed when fn-47 landed and before any table stated the row. Each is
/// the tree fn-44's own neutral pins and `tests/identity.rs` recorded before
/// the row existed, which is what says the row at zero reaches nothing.
const NEUTRAL: [u64; 7] = [
    17046456021212146411,
    14986275773972546726,
    12735573889651776723,
    12852486373694172527,
    13073206196273823952,
    12471405148157309180,
    14199367530911903060,
];
/// The cooked curtain's pendulous length: short enough that a crown of whole
/// runs grows in a tenth of a second.
const PENDULOUS: f64 = 1.0;
/// How near straight down, in radians, the last step of a strand at full sag
/// is when the strand ran its own whole length.
const VERTICAL: f64 = 1e-6;

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

/// The birch's curtain over a crown that leaves it room to fall, as fn-44's
/// sag test cooks it: one stem, straight axes, fine leaf-bearing wood so a run
/// is an arc of several chords, a full hang and a full sag.
fn curtain(variation: f64) -> Family {
    let mut f = preset("silver-birch");
    f.skeleton.habit.stems = 1;
    f.skeleton.habit.crookedness = 0.0;
    f.skeleton.envelope.crown_base = 0.02;
    f.skeleton.growth.max_nodes = Some(150_000);
    f.skeleton.twigs.twig.internode_length = 0.036;
    f.skeleton.twigs.twig.bearing_diameter = 0.015;
    f.skeleton.twigs.hang = 1.0;
    f.skeleton.twigs.pendulous_length = PENDULOUS;
    f.skeleton.twigs.sag = 1.0;
    f.skeleton.twigs.pendulous_variation = variation;
    f
}

/// One descending run, base to tip: the birth identity of its first node, its
/// length, and each step's angle to straight down with the arc length run by
/// the end of it.
struct Strand {
    identity: u64,
    length: f64,
    steps: Vec<(f64, f64)>,
}

impl Strand {
    /// At full sag a shoot reaches straight down exactly where its own run
    /// ends, so a strand whose last step is vertical ran its whole length.
    fn whole(&self) -> bool {
        self.steps.last().is_some_and(|s| s.0 < VERTICAL)
    }
}

/// Every run of a tree that departs downward. A run is one branch identity's
/// own chain of nodes; a chain broken by the frontier's order is dropped.
fn strands(tree: &Tree) -> Vec<Strand> {
    let mut chains: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        if n.kind == NodeKind::Branch {
            chains.entry(n.branch).or_default().push(i);
        }
    }
    let linked = |nodes: &Vec<usize>| {
        nodes
            .windows(2)
            .all(|w| tree.nodes[w[1]].parent == Some(w[0] as u32))
    };
    chains
        .into_iter()
        .filter(|(_, nodes)| linked(nodes))
        .filter_map(|(branch, nodes)| {
            let mut along = 0.0;
            let steps: Vec<(f64, f64)> = nodes
                .iter()
                .map(|&i| {
                    let node = &tree.nodes[i];
                    let parent =
                        &tree.nodes[node.parent.expect("a run node has a parent") as usize];
                    let step = node.position - parent.position;
                    along += step.length();
                    ((-step.normalized().y).clamp(-1.0, 1.0).acos(), along)
                })
                .collect();
            (steps[0].0 < FRAC_PI_2).then(|| Strand {
                identity: tree.nodes[branch as usize].identity.birth_order(),
                length: along,
                steps,
            })
        })
        .collect()
}

/// The lengths of the whole strands of one grown curtain, shortest first.
fn whole_lengths(f: &Family) -> Vec<f64> {
    let mut lengths: Vec<f64> = strands(&tree(f))
        .iter()
        .filter(|s| s.whole())
        .map(|s| s.length)
        .collect();
    lengths.sort_by(f64::total_cmp);
    lengths
}

#[test]
fn every_shipped_preset_at_neutral_variation_is_the_tree_the_row_never_reached() {
    for (id, pin) in IDS.iter().zip(NEUTRAL) {
        let mut f = preset(id);
        f.skeleton.twigs.pendulous_variation = 0.0;
        assert_eq!(
            skeleton(&tree(&f)),
            pin,
            "{id}: a neutral variation moved the tree"
        );
    }
}

/// Prints the neutral pin of every shipped table, to re-record after a value
/// change: `cargo test --release --test strands -- --ignored --nocapture`.
#[test]
#[ignore]
fn print_neutral() {
    for id in IDS {
        let mut f = preset(id);
        f.skeleton.twigs.pendulous_variation = 0.0;
        println!("NEUTRAL {id} {}", skeleton(&tree(&f)));
    }
}

#[test]
fn a_variation_never_reaches_a_table_that_hangs_nothing() {
    for id in IDS {
        let plain = preset(id);
        if plain.skeleton.twigs.hang > 0.0 {
            continue;
        }
        let pin = skeleton(&tree(&plain));
        let mut loud = preset(id);
        loud.skeleton.twigs.pendulous_variation = 1.0;
        assert_eq!(
            skeleton(&tree(&loud)),
            pin,
            "{id}: a variation moved a dry tree"
        );
    }
}

#[test]
fn the_row_rides_the_wire_and_is_refused_by_name_off_its_rail() {
    let mut wire = params::metadata(&preset("silver-birch"));
    wire["skeleton"]["twigs"]["pendulousVariation"] = serde_json::json!(0.35);
    let parsed = params::parse(&wire).expect("the wire carries the row");
    assert_eq!(parsed.skeleton.twigs.pendulous_variation, 0.35);
    for bad in [-0.001, 1.001] {
        wire["skeleton"]["twigs"]["pendulousVariation"] = serde_json::json!(bad);
        let f = params::parse(&wire).expect("the wire parses the row");
        assert_eq!(
            branching::generate(&f.skeleton, f.radii).err(),
            Some(Error::InvalidInput("pendulous variation")),
            "a variation of {bad} grew a tree"
        );
    }
}

#[test]
fn the_runs_spread_between_the_pendulous_length_and_its_shortened_share() {
    // Neutral, every whole strand is the one pendulous length. A planned run
    // the shell cuts short can keep a last chord planned whole, so a sliver of
    // strands reads whole and short; the promise is about the rest.
    let one = whole_lengths(&curtain(0.0));
    let at = one.iter().filter(|l| (*l - PENDULOUS).abs() < 1e-6).count();
    assert!(
        at * 100 >= one.len() * 99,
        "seed {SEED}: only {at} of {} whole strands ran the one pendulous length",
        one.len()
    );
    const VARIATION: f64 = 0.8;
    let short = PENDULOUS * (1.0 - VARIATION);
    let spread = whole_lengths(&curtain(VARIATION));
    let longest = spread.last().copied().unwrap_or(0.0);
    assert!(
        longest <= PENDULOUS + 1e-9,
        "seed {SEED}: a strand ran {longest:.6} m past the {PENDULOUS} m pendulous length"
    );
    let band = spread.iter().filter(|l| **l >= short - 1e-9).count();
    assert!(
        band * 100 >= spread.len() * 99,
        "seed {SEED}: only {band} of {} whole strands lie between {short} m and {PENDULOUS} m",
        spread.len()
    );
    // Across the band, not bunched: each fifth of it holds a tenth of them.
    for fifth in 0..5 {
        let low = short + (PENDULOUS - short) * f64::from(fifth) / 5.0;
        let high = low + (PENDULOUS - short) / 5.0;
        let held = spread
            .iter()
            .filter(|l| (low..high + 1e-9).contains(*l))
            .count();
        assert!(
            held * 10 >= spread.len(),
            "seed {SEED}: {held} of {} whole strands between {low:.2} m and {high:.2} m",
            spread.len()
        );
    }
}

#[test]
fn a_short_strand_under_a_full_sag_ends_hanging_straight_down() {
    // The sag's arc is spent over the shoot's own run, not the table's
    // pendulous length: a strand of a third of it has turned all the way down
    // by its end, where over the table's length it would still point out.
    let tree = tree(&curtain(0.8));
    let short: Vec<Strand> = strands(&tree)
        .into_iter()
        .filter(|s| s.length < PENDULOUS / 2.0 && s.steps.len() >= 4)
        .collect();
    // A shoot the shell or the floor cut short never reached its own end; a
    // sliver of the short ones are those, and nine in ten are the rest.
    let vertical = short.iter().filter(|s| s.whole()).count();
    assert!(
        short.len() >= 50 && vertical * 10 >= short.len() * 9,
        "seed {SEED}: only {vertical} of {} strands under half the pendulous length \
         ended straight down",
        short.len()
    );
    for s in short.iter().filter(|s| s.whole()) {
        let half = s.length / 2.0;
        let worst = s
            .steps
            .iter()
            .filter(|(_, along)| *along >= half)
            .map(|(a, _)| *a)
            .fold(0.0, f64::max);
        assert!(
            worst.to_degrees() <= 15.0,
            "seed {SEED}: a {:.3} m strand hung its lower half {:.1} degrees off vertical",
            s.length,
            worst.to_degrees()
        );
    }
}

#[test]
fn one_seed_is_one_curtain_whatever_order_it_grows_in() {
    // A strand's length is drawn from its own key and the seed, never from
    // where it falls in the order the tree grows: built at an age in one call,
    // month by month, or in uneven pieces, every strand is the same length.
    const AGE: f64 = 25.0;
    let runs = |s: &Specimen| -> BTreeMap<u64, u64> {
        strands(s.tree())
            .iter()
            .map(|s| (s.identity, s.length.to_bits()))
            .collect()
    };
    let mut f = curtain(0.8);
    f.age = AGE;
    let built = runs(&Specimen::build(&f).expect("the curtain builds"));
    let whole = strands(Specimen::build(&f).expect("the curtain builds").tree())
        .iter()
        .filter(|s| s.whole())
        .count();
    assert!(
        whole >= 100,
        "seed {SEED}: the curtain at {AGE} years hung {whole} whole strands"
    );
    f.age = 0.0;
    let mut monthly = Specimen::build(&f).expect("the seedling builds");
    for _ in 0..(AGE as usize * 12) {
        monthly.advance(1.0 / 12.0).expect("a month grows");
    }
    let mut uneven = Specimen::build(&f).expect("the seedling builds");
    for piece in [0.3, 4.7, 0.01, 7.99, 12.0] {
        uneven.advance(piece).expect("a piece grows");
    }
    for (name, other) in [("monthly", runs(&monthly)), ("uneven", runs(&uneven))] {
        let moved = built
            .iter()
            .find(|(identity, bits)| other.get(identity) != Some(bits));
        assert!(
            moved.is_none() && other.len() == built.len(),
            "seed {SEED}: the {name} build grew strand {:?} to another length",
            moved.map(|(identity, _)| identity)
        );
    }
    let mut other = curtain(0.8);
    other.skeleton.seed = SEED + 1;
    assert_ne!(
        whole_lengths(&curtain(0.8)),
        whole_lengths(&other),
        "seeds {SEED} and {}: two seeds hung one curtain",
        SEED + 1
    );
}

#[test]
fn a_walk_of_the_variation_row_shortens_the_curtain_continuously() {
    let (even, ragged) = (curtain(0.0), curtain(1.0));
    let mut means = Vec::new();
    let mut hashes = Vec::new();
    let mut nodes = Vec::new();
    for step in 0..=10 {
        let t = f64::from(step) / 10.0;
        let family = blend::families(&even, &ragged, t).expect("the walk validates");
        let row = family.skeleton.twigs.pendulous_variation;
        assert!(
            (row - t).abs() < 1e-12,
            "step {step} walked the row to {row}, not {t}"
        );
        let grown = tree(&family);
        let lengths: Vec<f64> = strands(&grown)
            .iter()
            .filter(|s| s.whole())
            .map(|s| s.length)
            .collect();
        let longest = lengths.iter().copied().fold(0.0, f64::max);
        assert!(
            longest <= PENDULOUS + 1e-9,
            "seed {SEED} step {step}: a strand ran {longest:.6} m"
        );
        means.push(lengths.iter().sum::<f64>() / lengths.len() as f64);
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
            means[step] < means[step - 1],
            "seed {SEED} step {step}: the curtain's strands lengthened, {:.4} m to {:.4} m",
            means[step - 1],
            means[step]
        );
        let (a, b) = (nodes[step - 1], nodes[step]);
        assert!(
            a.abs_diff(b) * 4 <= a.max(b),
            "step {step} changed the tree's kind: {a} nodes to {b}"
        );
    }
}
