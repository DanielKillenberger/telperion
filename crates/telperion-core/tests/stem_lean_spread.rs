//! The clump's lean spread: how unequally its stems lean. Neutral first — at
//! one stem the row reaches nothing on any table, to the byte — then its rail,
//! the stems of a grown clump leaning in their order, and the walk from an
//! even lean to a spread one.
//! No device is needed; this is the core's own arithmetic.
mod specimens;
use telperion_core::{
    blend, branching, presets::Family, presets::Preset, tree::NodeKind, tree::Tree, Error,
};

/// Every shipped table, so neutrality is asserted on all of them at once.
const PRESETS: [Preset; 7] = [
    Preset::Ordinary,
    Preset::OregonWhiteOak,
    Preset::NorwaySpruce,
    Preset::EuropeanBeech,
    Preset::SilverBirch,
    Preset::Telperion,
    Preset::Laurelin,
];
const SEED: u32 = 7;
/// Every step of the walk is grown under one cap, the way the sweep does.
const WALK_NODES: usize = 8_000;
/// The clump's own lean, in degrees.
const LEAN: f64 = 24.0;

/// FNV-1a over the bytes, the pattern the identity pins already hash with.
fn fnv(bytes: impl IntoIterator<Item = u8>) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in bytes {
        hash = (hash ^ u64::from(byte)).wrapping_mul(1099511628211);
    }
    hash
}

fn family(preset: Preset, row: impl Fn(&mut Family)) -> Family {
    let mut family = preset.parameters();
    family.skeleton.seed = SEED;
    row(&mut family);
    family
}

/// A clump the oak's table can stand on, leaning `spread` unequally.
fn clump(stems: u32, spread: f64) -> Family {
    family(Preset::OregonWhiteOak, |f| {
        f.skeleton.habit.stems = stems;
        f.skeleton.habit.stem_divergence = 100.0;
        f.skeleton.habit.stem_lean = LEAN;
        f.skeleton.habit.stem_lean_spread = spread;
        f.skeleton.growth.max_nodes = Some(WALK_NODES);
    })
}

fn grow(family: &Family) -> Tree {
    specimens::tree(family)
}

/// Degrees from vertical each stem's first edge leaves the root at, in the
/// order the stems were born.
fn leans(tree: &Tree) -> Vec<f64> {
    let root = tree.nodes[0].position;
    tree.nodes
        .iter()
        .skip(1)
        .filter(|n| n.parent == Some(0) && n.kind == NodeKind::Structural)
        .map(|n| {
            let d = n.position - root;
            (d.y / d.length()).clamp(-1.0, 1.0).acos().to_degrees()
        })
        .collect()
}

#[test]
fn at_one_stem_the_spread_reaches_no_table() {
    for preset in PRESETS {
        let bytes = |f: &Family| {
            let m = specimens::mesh(f);
            (
                fnv(m.wood.positions.iter().flat_map(|v| v.to_le_bytes())),
                fnv(m
                    .foliage
                    .instances
                    .leaves
                    .iter()
                    .flatten()
                    .flat_map(|w| w.to_le_bytes())),
            )
        };
        let was = bytes(&family(preset, |f| f.skeleton.habit.stems = 1));
        let dialled = family(preset, |f| {
            f.skeleton.habit.stems = 1;
            f.skeleton.habit.stem_lean_spread = 1.0;
        });
        assert_eq!(bytes(&dialled), was, "{preset:?} moved at one stem");
    }
}

#[test]
fn the_rail_is_refused_by_the_name_of_the_row() {
    for value in [-0.01, 1.01, f64::NAN, f64::INFINITY] {
        let f = family(Preset::SilverBirch, |f| {
            f.skeleton.habit.stem_lean_spread = value;
        });
        assert_eq!(
            branching::generate(&f.skeleton, f.radii).err(),
            Some(Error::InvalidInput("stem lean spread")),
            "a spread of {value} was accepted"
        );
    }
    for value in [0.0, 1.0] {
        let f = family(Preset::SilverBirch, |f| {
            f.skeleton.habit.stem_lean_spread = value;
        });
        assert!(
            f.skeleton.habit.validate().is_ok(),
            "{value} is on the rail"
        );
    }
}

#[test]
fn a_spread_clump_grows_its_stems_leaning_in_order() {
    // At the whole spread the first stem stands upright and each one after it
    // leans further out, to the last by the whole angle. The first edge is the
    // stem's heading bent by the axis's own small rise, so it is read to a
    // degree rather than to the bit.
    for stems in 2..=4_u32 {
        let tree = grow(&clump(stems, 1.0));
        let leans = leans(&tree);
        assert_eq!(leans.len(), stems as usize, "{stems} stems were not born");
        let last = leans.len() - 1;
        assert!(leans[0] < 1.0, "the first of {stems} leans {}", leans[0]);
        assert!(
            (leans[last] - LEAN).abs() < 1.0,
            "the last leans {}",
            leans[last]
        );
        for k in 1..leans.len() {
            assert!(
                leans[k] > leans[k - 1],
                "{stems} stems: stem {k} leans {} after {}",
                leans[k],
                leans[k - 1]
            );
        }
    }
    // Part of the way, the first keeps what the spread leaves it.
    let leans = leans(&grow(&clump(2, 0.6)));
    assert!(
        (leans[0] - LEAN * 0.4).abs() < 1.0,
        "the first leans {}",
        leans[0]
    );
    assert!(
        (leans[1] - LEAN).abs() < 1.0,
        "the second leans {}",
        leans[1]
    );
}

#[test]
fn the_walk_from_an_even_lean_to_a_spread_one_stands_the_first_stem_up() {
    // The spread walks like any fraction. Every step is a family that grows a
    // tree on the same two stems; the first stem rises out of its lean step by
    // step while the last keeps the whole of it, and no step grows the tree
    // the step before it did.
    let (from, to) = (clump(2, 0.0), clump(2, 1.0));
    const STEPS: usize = 11;
    let mut walked: Vec<(Vec<f64>, u64)> = Vec::new();
    for step in 0..STEPS {
        let t = step as f64 / (STEPS - 1) as f64;
        let f = blend::families(&from, &to, t).expect("the walk is a family");
        assert!(
            (f.skeleton.habit.stem_lean_spread - t).abs() < 1e-12,
            "step {step} is not {t} of the way"
        );
        let tree = grow(&f);
        let leans = leans(&tree);
        assert_eq!(leans.len(), 2, "step {step} grew {} stems", leans.len());
        let skeleton = fnv(tree.nodes.iter().skip(1).flat_map(|n| {
            [n.position.x, n.position.y, n.position.z]
                .into_iter()
                .flat_map(f64::to_le_bytes)
        }));
        walked.push((leans, skeleton));
    }
    for step in 1..STEPS {
        let (was, now) = (&walked[step - 1], &walked[step]);
        assert!(
            now.0[0] < was.0[0],
            "step {step}: the first stem leaned out again"
        );
        assert!(
            (now.0[1] - was.0[1]).abs() < 1e-9,
            "step {step}: the last stem moved"
        );
        assert_ne!(
            now.1, was.1,
            "step {step} grew the tree the step before did"
        );
    }
}
