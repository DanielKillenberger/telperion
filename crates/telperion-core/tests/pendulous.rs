//! The curtain as rows. Neutral at zero, continuous across every value,
//! refused by its own name outside its rail, and floored at every hang.
//! No device is needed; this is the core's own arithmetic.
use telperion_core::{
    branching, presets::Family, presets::Preset, tree::NodeKind, twigs::TwigParams, Error,
};

const SEED: u32 = 7;

/// One tree from one row: the FNV-1a hash of its skeleton — the pattern the
/// identity pins already use — the height of every node, and the height of
/// every leaf-bearing end.
struct Grown {
    skeleton: u64,
    nodes: Vec<f64>,
    twigs: Vec<f64>,
}

fn grow(preset: Preset, row: impl Fn(&mut Family)) -> Grown {
    let mut family = preset.parameters();
    family.skeleton.seed = SEED;
    row(&mut family);
    let tree = branching::generate(&family.skeleton, family.radii)
        .expect("the row grows a tree")
        .tree;
    let mut skeleton = 14695981039346656037_u64;
    let (mut nodes, mut twigs) = (Vec::with_capacity(tree.nodes.len()), Vec::new());
    for n in tree.nodes.iter().skip(1) {
        nodes.push(n.position.y);
        if n.kind == NodeKind::Twig {
            twigs.push(n.position.y);
        }
        for byte in [n.position.x, n.position.y, n.position.z]
            .into_iter()
            .flat_map(f64::to_le_bytes)
            .chain(n.parent.expect("non-root parent").to_le_bytes())
        {
            skeleton = (skeleton ^ u64::from(byte)).wrapping_mul(1099511628211);
        }
    }
    Grown {
        skeleton,
        nodes,
        twigs,
    }
}

/// The curtain with room to hang: no bare trunk under the crown, so what stops
/// a pendulous shoot is its own floor and not the envelope's lower edge.
fn hanging_room(f: &mut Family, hang: f64) {
    f.skeleton.envelope.crown_base = 0.0;
    f.skeleton.twigs.hang = hang;
    f.skeleton.twigs.pendulous_length = 5.0;
}

#[test]
fn a_family_at_hang_zero_is_the_tree_the_curtain_never_reached() {
    // The three magnitudes say nothing at all until something hangs, so a row
    // that leaves hang at zero grows the tree it grew before the rows existed,
    // whatever the other three say. The oak is the witness: it never hung.
    let plain = grow(Preset::OregonWhiteOak, |_| {}).skeleton;
    for (length, radius, separation) in [(0.05, 0.0, 1.0), (5.0, 1.0, 45.0), (2.0, 0.4, 17.5)] {
        let loud = grow(Preset::OregonWhiteOak, |f| {
            f.skeleton.twigs.pendulous_length = length;
            f.skeleton.twigs.pendulous_radius = radius;
            f.skeleton.twigs.curtain_separation = separation;
        })
        .skeleton;
        assert_eq!(plain, loud, "a curtain nobody hangs moved the oak");
    }
}

#[test]
fn the_curtain_grows_in_from_nothing_rather_than_switching_on() {
    // Hang is a row and not a mode: every step of a walk across it moves the
    // tree, and no step of it is the frame where the tree changes kind. The
    // birch is the witness, since its table is the one that hangs.
    let walk: Vec<Grown> = (0..=10)
        .map(|step| {
            grow(Preset::SilverBirch, |f| {
                f.skeleton.twigs.hang = f64::from(step) / 10.0;
            })
        })
        .collect();
    assert_eq!(
        walk[0].skeleton,
        grow(Preset::SilverBirch, |f| {
            f.skeleton.twigs.hang = 0.0;
            f.skeleton.twigs.pendulous_length = 5.0;
            f.skeleton.twigs.curtain_separation = 45.0;
        })
        .skeleton,
        "hang zero is not neutral"
    );
    for step in 1..walk.len() {
        assert_ne!(
            walk[step - 1].skeleton,
            walk[step].skeleton,
            "step {step} grew the tree the step before it did"
        );
        // A tenth of the row is a tenth of the curtain, not a different tree.
        let (a, b) = (walk[step - 1].nodes.len(), walk[step].nodes.len());
        assert!(
            a.abs_diff(b) * 4 <= a.max(b),
            "step {step} changed the tree's kind: {a} nodes to {b}"
        );
    }
    // And the walk is a walk: a full hang does not leave the crown where the
    // neutral row leaves it. What the curtain moves is where the leaf-bearing
    // ends sit, so that is what is measured.
    let mean = |g: &Grown| g.twigs.iter().sum::<f64>() / g.twigs.len() as f64;
    let (none, full) = (mean(&walk[0]), mean(&walk[10]));
    assert!(
        (full - none).abs() > none / 50.0,
        "a full hang left the crown where a neutral row left it: {none} m to {full} m"
    );
}

#[test]
fn each_curtain_row_is_refused_by_its_own_name() {
    for (row, bad, name) in [
        ("hang", -0.001, "hang"),
        ("hang", 3.001, "hang"),
        ("hang", f64::NAN, "hang"),
        ("length", 0.049, "pendulous length"),
        ("length", 5.001, "pendulous length"),
        ("radius", -0.001, "pendulous radius"),
        ("radius", 1.001, "pendulous radius"),
        ("separation", 0.999, "curtain separation"),
        ("separation", 45.001, "curtain separation"),
    ] {
        let mut t = TwigParams::default();
        match row {
            "hang" => t.hang = bad,
            "length" => t.pendulous_length = bad,
            "radius" => t.pendulous_radius = bad,
            _ => t.curtain_separation = bad,
        }
        assert_eq!(
            t.resolved().err(),
            Some(Error::InvalidInput(name)),
            "{row} {bad} was accepted"
        );
    }
    // Both ends of every rail are themselves rows.
    for t in [
        TwigParams {
            hang: 0.0,
            pendulous_length: 0.05,
            pendulous_radius: 0.0,
            curtain_separation: 1.0,
            ..TwigParams::default()
        },
        TwigParams {
            hang: 3.0,
            pendulous_length: 5.0,
            pendulous_radius: 1.0,
            curtain_separation: 45.0,
            ..TwigParams::default()
        },
    ] {
        assert!(t.resolved().is_ok(), "a rail's own end was refused");
    }
}

#[test]
fn the_floor_holds_under_every_hang_and_the_curtain_does_not_stack_on_it() {
    // A pendulous shoot stops above the floor its ancestor's tip set. Given a
    // crown that reaches the ground and a pendulous length longer than the
    // tree is tall, the curtain still stands clear of the ground and does not
    // pile its ends at one height.
    for hang in [0.25, 0.5, 0.75, 1.0] {
        let grown = grow(Preset::SilverBirch, |f| hanging_room(f, hang));
        let floor = grown.twigs.iter().copied().fold(f64::INFINITY, f64::min);
        let stacked = grown.twigs.iter().filter(|y| **y < floor + 0.02).count();
        assert!(floor > 0.0, "hang {hang}: the curtain reached the ground");
        assert!(
            stacked * 20 < grown.twigs.len(),
            "hang {hang}: {stacked} of {} twig ends stacked on the floor",
            grown.twigs.len()
        );
    }
}

#[test]
fn the_spruce_states_the_values_that_reproduce_the_constants() {
    // The curtain's magnitudes left the source as literals and arrive as rows;
    // the spruce's table is where the values they had are now written down.
    let t = Preset::NorwaySpruce.parameters().skeleton.twigs;
    assert_eq!(t.hang, 1.0);
    assert_eq!(t.pendulous_length, t.twig.length);
    assert_eq!(t.pendulous_radius, 1.0);
    assert_eq!(t.curtain_separation, 4.0);
}
