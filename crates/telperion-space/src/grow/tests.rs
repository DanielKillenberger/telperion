//! The speed levers leave every tree as it was (fn-210).
use super::{run, Certain, Grown, Options};
use crate::{beech, oak, palm, spruce, Light, Request, Species};
use std::collections::HashSet;

fn grown(species: &Species, age: u32, seed: u64, certain: Certain) -> Grown {
    let options = Options {
        certain,
        ..Options::default()
    };
    grown_in(species, (age, seed, Light::NEUTRAL), options)
}

fn grown_in(species: &Species, (age, seed, light): (u32, u64, Light), options: Options) -> Grown {
    let request = Request {
        age,
        seed,
        budget: 20_000_000,
        light,
    };
    run(species, request, &mut |_| {}, true, options).unwrap()
}

/// Leaves at every angle under the standard overcast sky.
const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

/// The beech with its twigs' bearers kept long after they die, so a twig
/// that sheds early is the last growth some kept bearer had: its fade then
/// reads the twig, and the twig may not be left ungrown.
fn long_kept_bearers() -> Species {
    let mut species = beech();
    for state in &mut species.states[9..13] {
        state.shedding = 6.0;
    }
    species
}

fn species() -> [(&'static str, Species); 4] {
    [
        ("beech", beech()),
        ("spruce", spruce()),
        ("palm", palm()),
        ("beech with long-kept bearers", long_kept_bearers()),
    ]
}

/// Every lateral the rule finds certain to be shed is gone from the tree:
/// its presence at the tree's age is exactly 0.
#[test]
fn a_lateral_certain_to_be_shed_is_shed() {
    for (name, species) in species() {
        for seed in [1, 7] {
            let tree = grown(&species, 30, seed, Certain::Mark);
            let kept: HashSet<u64> = tree.structure.axes.iter().map(|a| a.lineage).collect();
            let standing = tree.marked.iter().filter(|m| kept.contains(m)).count();
            assert_eq!(
                standing, 0,
                "{name} seed {seed}: {standing} marked laterals stand"
            );
            if name != "palm" {
                assert!(!tree.marked.is_empty(), "{name} seed {seed}: none marked");
            }
        }
    }
}

/// Leaving those laterals ungrown leaves the tree the same to the bit,
/// and grows less of it.
#[test]
fn a_tree_grows_the_same_without_what_it_is_certain_to_shed() {
    for (name, species) in species() {
        for seed in [1, 7] {
            let all = grown(&species, 30, seed, Certain::Grow);
            let skipped = grown(&species, 30, seed, Certain::Skip);
            assert!(
                skipped.structure == all.structure,
                "{name} seed {seed}: the tree changed"
            );
            // The shipped trees' twigs are a third of what they grow or
            // more; the palm sheds nothing.
            let share = match name {
                "palm" => continue,
                "beech with long-kept bearers" => 0.9,
                _ => 2.0 / 3.0,
            };
            assert!(
                (skipped.grown as f64) < share * all.grown as f64,
                "{name} seed {seed}: {} of {} grown",
                skipped.grown,
                all.grown
            );
        }
    }
}

/// Where something reads a lateral while it lives, the oak's girth here,
/// nothing is decided before growth.
#[test]
fn nothing_is_skipped_where_shed_wood_is_read() {
    let tree = grown(&oak(), 20, 1, Certain::Mark);
    assert!(tree.marked.is_empty());
}

/// The light work no bud reads changes nothing: the rough layout of a
/// species that reads no light, and the oak's last cycle's layout, full
/// lay and sweep.
#[test]
fn light_no_bud_reads_is_not_worked() {
    let all = Options {
        unread_light: true,
        ..Options::default()
    };
    for (name, species, age) in [
        ("beech", beech(), 30),
        ("spruce", spruce(), 30),
        ("oak", oak(), 20),
    ] {
        for seed in [1, 7] {
            let at = (age, seed, LIT);
            let lean = grown_in(&species, at, Options::default());
            let full = grown_in(&species, at, all);
            assert!(
                lean.structure == full.structure,
                "{name} seed {seed}: the tree changed"
            );
        }
    }
}

/// A cycle grown in shares on many cores is the tree grown in turn on
/// one: the beech, the spruce, whose sleeping buds wake, the oak in light
/// and the palm.
#[test]
fn many_cores_grow_the_tree_one_does() {
    let cores = |threads| Options {
        threads: Some(threads),
        share: 7,
        ..Options::default()
    };
    // The spruce with its lifespans between whole cycles: an age that ends
    // within a cycle carries on in it, and its sleeping buds are put to
    // sleep on an axis that cycle made.
    let mut halves = spruce();
    for state in &mut halves.states {
        if state.lifespan > 0.0 {
            state.lifespan += 0.5;
        }
    }
    let trees = [
        ("spruce with half cycles", halves, (25, Light::NEUTRAL)),
        ("beech", beech(), (30, LIT)),
        ("spruce", spruce(), (30, Light::NEUTRAL)),
        ("oak", oak(), (20, LIT)),
        ("palm", palm(), (30, Light::NEUTRAL)),
    ];
    for (name, species, (age, light)) in trees {
        for seed in [1, 7] {
            let one = grown_in(&species, (age, seed, light), cores(1));
            let many = grown_in(&species, (age, seed, light), cores(8));
            assert!(
                one.structure == many.structure,
                "{name} seed {seed}: the tree changed"
            );
            assert_eq!(one.grown, many.grown, "{name} seed {seed}");
        }
    }
}
