//! Codominant forks, one rule from the ground to the crown (fn-170). Neutral
//! first - at rate zero no table forks, whatever the other fork rows say, to
//! the byte - then the rails and the retired stems rows, the seed deciding
//! each fork, forks repeating up the crown, and the walk from no fork to a
//! clump.
//! No device is needed; this is the core's own arithmetic.
use super::specimens;
use telperion_core::{
    blend, branching, params, presets::values, presets::Family, presets::Preset, tree::NodeKind,
    tree::Tree, Error,
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
/// Every synthetic tree is grown under one cap, the way the sweep does: what
/// is judged here is where the forks fall, not the crown's full size.
const NODES: usize = 8_000;
/// The fixed seed set R3 and R4 read.
const SEEDS: std::ops::Range<u32> = 1..25;

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

/// The oak's table forking at `rate`, its forks drawn about `height` of the
/// tree `spread` wide, grown under the cap.
fn forking(seed: u32, rate: f64, height: f64, spread: f64) -> Family {
    family(Preset::OregonWhiteOak, |f| {
        f.skeleton.seed = seed;
        f.skeleton.habit.codominance = rate;
        f.skeleton.habit.fork_height = height;
        f.skeleton.habit.fork_height_spread = spread;
        f.skeleton.habit.fork_lean = 20.0;
        f.skeleton.habit.fork_divergence = 90.0;
        f.skeleton.growth.max_nodes = Some(NODES);
    })
}

fn grow(family: &Family) -> Tree {
    specimens::tree(family)
}

/// Every codominant sibling's first node.
fn siblings(tree: &Tree) -> Vec<usize> {
    (1..tree.nodes.len())
        .filter(|&i| {
            tree.nodes[i].kind == NodeKind::Structural && tree.nodes[i].codominant.is_some()
        })
        .collect()
}

#[test]
fn at_rate_zero_no_table_forks_whatever_the_other_rows_say() {
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
        let still = family(preset, |f| f.skeleton.habit.codominance = 0.0);
        let dialled = family(preset, |f| {
            f.skeleton.habit.codominance = 0.0;
            f.skeleton.habit.fork_height = 0.3;
            f.skeleton.habit.fork_height_spread = 0.2;
            f.skeleton.habit.fork_ways = 3.5;
            f.skeleton.habit.fork_divergence = 90.0;
            f.skeleton.habit.fork_lean = 30.0;
            f.skeleton.habit.fork_lean_spread = 0.5;
            f.radii.fork_balance = 0.5;
        });
        assert_eq!(
            bytes(&dialled),
            bytes(&still),
            "{preset:?} moved at rate zero"
        );
        assert!(siblings(&grow(&dialled)).is_empty(), "{preset:?} forked");
    }
}

#[test]
fn each_rail_is_refused_by_the_name_of_its_row() {
    type Set = fn(&mut Family, f64);
    let rows: [(&str, Set, [f64; 2]); 7] = [
        (
            "codominance",
            |f, v| f.skeleton.habit.codominance = v,
            [-0.01, 1.01],
        ),
        (
            "fork height",
            |f, v| f.skeleton.habit.fork_height = v,
            [-0.01, 1.01],
        ),
        (
            "fork height spread",
            |f, v| f.skeleton.habit.fork_height_spread = v,
            [-0.01, 1.01],
        ),
        (
            "fork ways",
            |f, v| f.skeleton.habit.fork_ways = v,
            [1.99, 4.01],
        ),
        (
            "fork divergence",
            |f, v| f.skeleton.habit.fork_divergence = v,
            [-1.0, 120.5],
        ),
        (
            "fork lean",
            |f, v| f.skeleton.habit.fork_lean = v,
            [-1.0, 45.5],
        ),
        (
            "fork lean spread",
            |f, v| f.skeleton.habit.fork_lean_spread = v,
            [-0.01, 1.01],
        ),
    ];
    for (row, set, off) in rows {
        for value in off.into_iter().chain([f64::NAN]) {
            let f = family(Preset::SilverBirch, |f| set(f, value));
            assert_eq!(
                branching::generate(&f.skeleton, f.radii).err(),
                Some(Error::InvalidInput(row)),
                "{row} of {value} was accepted"
            );
        }
    }
    // Rows beside a rate of zero are inert, never an error.
    let f = family(Preset::OregonWhiteOak, |f| {
        f.skeleton.habit.codominance = 0.0;
        f.skeleton.habit.fork_lean = 0.0;
        f.skeleton.habit.fork_divergence = 0.0;
    });
    assert!(branching::generate(&f.skeleton, f.radii).is_ok());
}

#[test]
fn a_retired_stems_row_is_refused_naming_its_replacement() {
    for (row, key, replacement) in [
        ("stems", 2.0, "/skeleton/habit/codominance"),
        ("stemDivergence", 30.0, "/skeleton/habit/forkDivergence"),
        ("stemLean", 20.0, "/skeleton/habit/forkLean"),
        ("stemLeanSpread", 0.5, "/skeleton/habit/forkLeanSpread"),
        ("stemForkHeight", 0.2, "/skeleton/habit/forkHeight"),
    ] {
        let wire = serde_json::json!({"skeleton": {"habit": {row: key}}});
        match params::parse(&wire) {
            Err(Error::InvalidInput(why)) => {
                assert!(why.contains(row) && why.contains(replacement), "{why}")
            }
            other => panic!("{row} on the wire gave {other:?}"),
        }
        let file = format!("/skeleton/habit/{row} = {key}\n");
        let why = values::read(&file).expect_err("a retired row was read");
        assert!(why.starts_with("1: ") && why.contains(replacement), "{why}");
    }
}

#[test]
fn the_seed_decides_each_fork_and_the_rate_adds_them() {
    // Forks drawn low on the trunk: at a middle rate some of the fixed seeds
    // fork there and some do not, the share rises with the rate, none fork at
    // zero, and the same seed and rows grow the same tree.
    let low = |tree: &Tree, f: &Family| {
        siblings(tree).into_iter().any(|i| {
            tree.nodes[i].stem && tree.nodes[i].position.y < 0.3 * f.skeleton.envelope.height
        })
    };
    let mut shares = Vec::new();
    for rate in [0.0, 0.25, 0.5, 0.75] {
        let forked = SEEDS
            .filter(|&seed| {
                let f = forking(seed, rate, 0.12, 0.05);
                low(&grow(&f), &f)
            })
            .count();
        shares.push(forked);
    }
    assert_eq!(shares[0], 0, "a seed forked at rate zero");
    assert!(
        shares[2] > 0 && shares[2] < SEEDS.len(),
        "at a middle rate {} of 24 forked low",
        shares[2]
    );
    assert!(shares.windows(2).all(|w| w[1] >= w[0]), "{shares:?}");
    assert!(shares[3] > shares[1], "{shares:?}");
    let f = forking(3, 0.5, 0.12, 0.05);
    assert_eq!(grow(&f), grow(&f), "one seed grew two trees");
}

#[test]
fn forks_repeat_up_the_crown() {
    // A bell over the whole crown forks limbs as well as the trunk, and a
    // part a fork left forks again.
    let (mut limbs, mut again) = (0, 0);
    for seed in SEEDS {
        let tree = grow(&forking(seed, 0.6, 0.55, 0.25));
        let forks = siblings(&tree);
        limbs += forks.iter().filter(|&&i| !tree.nodes[i].stem).count();
        again += forks
            .iter()
            .filter(|&&i| {
                let mut at = tree.nodes[i].parent;
                while let Some(p) = at {
                    if tree.nodes[p as usize].codominant.is_some() {
                        return true;
                    }
                    at = tree.nodes[p as usize].parent;
                }
                false
            })
            .count();
    }
    assert!(limbs > 0, "no limb forked codominantly on 24 seeds");
    assert!(again > 0, "no fork's part forked again on 24 seeds");
}

#[test]
fn the_walk_from_no_fork_to_a_clump_opens_it_rather_than_switching_it() {
    // Every point of the walk grows a tree; the stems at the root never go
    // back, and once the second is there it only ever stands further out.
    let from = family(Preset::OregonWhiteOak, |f| {
        f.skeleton.growth.max_nodes = Some(NODES)
    });
    let to = family(Preset::OregonWhiteOak, |f| {
        f.skeleton.habit.codominance = 1.0;
        f.skeleton.habit.fork_divergence = 100.0;
        f.skeleton.habit.fork_lean = 16.0;
        f.skeleton.growth.max_nodes = Some(NODES);
    });
    const STEPS: usize = 11;
    let (mut counts, mut apart, mut skeletons) = (Vec::new(), Vec::new(), Vec::new());
    for step in 0..STEPS {
        let t = step as f64 / (STEPS - 1) as f64;
        let f = blend::families(&from, &to, t).expect("the walk is a family");
        let tree = grow(&f);
        let roots: Vec<usize> = (1..tree.nodes.len())
            .filter(|&i| tree.nodes[i].parent == Some(0))
            .collect();
        counts.push(roots.len());
        apart.push(match roots.as_slice() {
            [a, b] => tree.nodes[*a].position.distance(tree.nodes[*b].position),
            _ => 0.0,
        });
        skeletons.push(fnv(tree.nodes.iter().skip(1).flat_map(|n| {
            [n.position.x, n.position.y, n.position.z]
                .into_iter()
                .flat_map(f64::to_le_bytes)
        })));
    }
    assert_eq!((counts[0], counts[STEPS - 1]), (1, 2), "{counts:?}");
    assert!(counts.windows(2).all(|w| w[1] >= w[0]), "{counts:?}");
    let born = counts.iter().position(|&c| c > 1).unwrap();
    for step in 0..born {
        assert_eq!(skeletons[step], skeletons[0], "step {step} moved the oak");
    }
    for step in born + 1..STEPS {
        assert!(
            apart[step] > apart[step - 1],
            "step {step} closed the clump"
        );
    }
}

#[test]
fn walking_the_ways_from_two_to_three_grows_the_third_part_from_nothing() {
    // Two full parts at the root; the third grows from the fork, its wood
    // longer at every step and none at all at two.
    let wood = |ways: f64| {
        let f = family(Preset::OregonWhiteOak, |f| {
            f.skeleton.habit.codominance = 1.0;
            f.skeleton.habit.fork_ways = ways;
            f.skeleton.habit.fork_divergence = 100.0;
            f.skeleton.habit.fork_lean = 16.0;
            f.skeleton.growth.max_nodes = Some(NODES);
        });
        let tree = grow(&f);
        let roots: Vec<usize> = (1..tree.nodes.len())
            .filter(|&i| tree.nodes[i].parent == Some(0))
            .collect();
        let third = roots.get(2).map_or(0.0, |&first| {
            // The run a stem is: its first node and the stem nodes above it.
            let mut length = tree.nodes[first].position.length();
            let mut at = first;
            while let Some(next) = (at + 1..tree.nodes.len())
                .find(|&k| tree.nodes[k].parent == Some(at as u32) && tree.nodes[k].stem)
            {
                length += tree.nodes[next].position.distance(tree.nodes[at].position);
                at = next;
            }
            length
        });
        (roots.len(), third)
    };
    assert_eq!(wood(2.0), (2, 0.0));
    let mut was = 0.0;
    for ways in [2.25, 2.5, 2.75, 3.0] {
        let (parts, third) = wood(ways);
        assert_eq!(parts, 3, "{ways}");
        assert!(
            third > was,
            "at {ways} the third part is {third} after {was}"
        );
        was = third;
    }
}

#[test]
fn across_a_seeds_point_on_the_rate_the_fork_grows_in_from_the_root() {
    // A clump drawn at the root: below the seed's point no fork, past it the
    // sibling's weight and its wood grow with the rate, never all at once.
    let sibling = |rate: f64| -> (f64, f64) {
        let f = family(Preset::OregonWhiteOak, |f| {
            f.skeleton.habit.codominance = rate;
            f.skeleton.habit.fork_divergence = 100.0;
            f.skeleton.habit.fork_lean = 16.0;
            f.skeleton.growth.max_nodes = Some(NODES);
        });
        let tree = grow(&f);
        (1..tree.nodes.len())
            .find(|&i| tree.nodes[i].parent == Some(0) && tree.nodes[i].codominant.is_some())
            .map_or((0.0, 0.0), |i| {
                (
                    tree.nodes[i].codominant.unwrap(),
                    tree.nodes[i].start_radius,
                )
            })
    };
    let onset = (1..=20)
        .map(|k| f64::from(k) * 0.05)
        .find(|&rate| sibling(rate).0 > 0.0)
        .expect("the seed forks by a rate of one");
    let mut was = (0.0, 0.0);
    for k in 0..=24 {
        let now = sibling((onset - 0.06 + f64::from(k) * 0.005).clamp(0.0, 1.0));
        assert!(
            now.0 >= was.0 && now.0 - was.0 < 0.25,
            "weight {was:?} to {now:?}"
        );
        assert!(
            now.1 >= was.1 - 1e-12,
            "the sibling's wood shrank: {was:?} to {now:?}"
        );
        was = now;
    }
    assert_eq!(was.0, 1.0, "the fork never grew in whole");
}
