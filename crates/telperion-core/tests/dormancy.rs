//! A row the catalogue calls dormant does nothing while its condition holds:
//! moving it changes no artifact of the build, the field included. Each case
//! is one representative row per mechanism, and each carries its control: the
//! same move with the condition lifted changes the build, so a digest that
//! missed an artifact cannot pass a case.
use std::fmt::Write;
use telperion_core::field::IndexSnapshot;
use telperion_core::{catalogue, pipeline, presets::Preset, Family};

struct Case {
    preset: Preset,
    row: &'static str,
    to: f64,
    /// The rows that put `row` to sleep, per the catalogue's `applies`.
    asleep: &'static [(&'static str, f64)],
    /// The same rows with the condition lifted.
    awake: &'static [(&'static str, f64)],
}

const CASES: &[Case] = &[
    Case {
        preset: Preset::Ordinary,
        row: "/skeleton/habit/stemDivergence",
        to: 40.0,
        asleep: &[("/skeleton/habit/stems", 1.0)],
        awake: &[
            ("/skeleton/habit/stems", 3.0),
            ("/skeleton/habit/stemLean", 10.0),
            ("/skeleton/habit/stemDivergence", 100.0),
        ],
    },
    Case {
        preset: Preset::Ordinary,
        row: "/skeleton/habit/lateralPitch",
        to: 30.0,
        asleep: &[("/skeleton/habit/lateralOrders", 0.0)],
        awake: &[("/skeleton/habit/lateralOrders", 3.0)],
    },
    Case {
        preset: Preset::Ordinary,
        row: "/skeleton/attractors",
        to: 900.0,
        asleep: &[("/skeleton/habit/attractorWeight", 0.0)],
        awake: &[("/skeleton/habit/attractorWeight", 1.0)],
    },
    Case {
        preset: Preset::Ordinary,
        row: "/skeleton/bias/supernatural/writheAmplitude",
        to: 0.1,
        asleep: &[("/skeleton/bias/supernatural/enabled", 0.0)],
        awake: &[("/skeleton/bias/supernatural/enabled", 1.0)],
    },
    Case {
        preset: Preset::Ordinary,
        row: "/skeleton/twigs/pendulousRadius",
        to: 0.0,
        asleep: &[("/skeleton/twigs/hang", 0.0)],
        awake: &[("/skeleton/twigs/hang", 1.0)],
    },
    Case {
        preset: Preset::Ordinary,
        row: "/radii/maxTaperExponent",
        to: 0.0,
        asleep: &[("/radii/lengthTaper", 0.0)],
        awake: &[("/radii/lengthTaper", 2.0)],
    },
    Case {
        preset: Preset::Ordinary,
        row: "/surface/flareFalloff",
        to: 0.5,
        asleep: &[("/surface/flareRadius", 1.0)],
        awake: &[("/surface/flareRadius", 2.0)],
    },
    Case {
        preset: Preset::Ordinary,
        row: "/canopy/shortShootLeaves",
        to: 6.0,
        asleep: &[("/canopy/shortShootSpacing", 0.0)],
        awake: &[("/canopy/shortShootSpacing", 0.05)],
    },
    Case {
        preset: Preset::Ordinary,
        row: "/element/cup",
        to: 0.4,
        asleep: &[("/element/card", 1.0)],
        awake: &[("/element/card", 0.0)],
    },
    Case {
        preset: Preset::DatePalm,
        row: "/canopy/skirtLength",
        to: 0.5,
        asleep: &[("/canopy/skirtFronds", 0.0)],
        awake: &[],
    },
    Case {
        preset: Preset::DatePalm,
        row: "/canopy/leafBaseRadius",
        to: 0.2,
        asleep: &[("/canopy/leafBases", 0.0)],
        awake: &[],
    },
];

/// FNV-1a over the `Debug` text of a value.
fn digest(value: &dyn std::fmt::Debug) -> u64 {
    struct Fnv(u64);
    impl Write for Fnv {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            for b in s.bytes() {
                self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
            }
            Ok(())
        }
    }
    let mut fnv = Fnv(0xcbf2_9ce4_8422_2325);
    let _ = write!(fnv, "{value:?}");
    fnv.0
}

fn set(family: &mut Family, rows: &[(&str, f64)]) {
    for &(path, value) in rows {
        let entry = catalogue::entry(path).unwrap_or_else(|| panic!("no row {path}"));
        entry.set(family).put(value);
    }
}

/// Every artifact a build returns, each by name with its digest: the
/// skeleton, the wood, the leaf element and instances, the structure and
/// the field.
fn build(family: &Family) -> Vec<(&'static str, u64)> {
    let request = pipeline::Request {
        wood: true,
        leaves: true,
        field: Some(None),
        structure: true,
        ..pipeline::Request::default()
    };
    let built = pipeline::build(family, request).expect("the family builds");
    let out = &built.outputs;
    let leaves = out
        .leaves
        .as_ref()
        .map(|l| (&l.instances, l.placed, l.retained));
    let structure = out.structure.as_ref().map(|s| (&s.nodes, &s.topology));
    let f = out.field.as_ref().expect("a field").snapshot().unwrap();
    let index = |i: &IndexSnapshot| digest(&(&i.bounds, &i.topology, i.node_count));
    vec![
        (
            "skeleton",
            digest(&(&built.skeleton.tree, built.skeleton.shed)),
        ),
        ("wood", digest(&out.wood)),
        ("element", digest(&out.element)),
        ("leaves", digest(&leaves)),
        ("structure", digest(&structure)),
        ("field wood", digest(&f.wood) ^ index(&f.wood_index)),
        ("field leaves", index(&f.leaves)),
        (
            "field plan",
            digest(&(&f.plan, &f.plan_stations, &f.plan_sides)) ^ index(&f.plan_index),
        ),
    ]
}

/// The artifacts that change when `row` moves, under `rows`.
fn moved(case: &Case, rows: &[(&str, f64)]) -> Vec<&'static str> {
    let mut family = case.preset.parameters();
    set(&mut family, rows);
    let before = build(&family);
    set(&mut family, &[(case.row, case.to)]);
    let after = build(&family);
    before
        .iter()
        .zip(&after)
        .filter(|(a, b)| a.1 != b.1)
        .map(|(a, _)| a.0)
        .collect()
}

#[test]
fn a_dormant_row_moves_no_artifact() {
    for case in CASES {
        let applies = catalogue::entry(case.row).unwrap().info().applies;
        assert!(!applies.is_empty(), "{} claims no dormancy", case.row);
        let asleep = moved(case, case.asleep);
        assert!(
            asleep.is_empty(),
            "{} moved {asleep:?} while dormant ({applies})",
            case.row
        );
        assert!(
            !moved(case, case.awake).is_empty(),
            "{} moved nothing awake: the case proves nothing",
            case.row
        );
    }
}
