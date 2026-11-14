//! fn-351 P10: "a native entry with no browser, harness or engine dependency
//! takes parameters and a seed, never a species name, and returns the tree's
//! data at any age; a headless test grows the oak through it" (fn-365 R2).
//!
//! The entry is `pipeline::growth` (`docs/native-entry.md`). The oak reaches
//! it as an engine would hand it over: parameter text, read by
//! `params::parse`. The setup renders that text from the oak's value file
//! through `params::metadata`, which is not the entry; no fixture is
//! committed, so the text is the shipped table and never a copy that drifts
//! from it.
use std::fmt::Write;
use std::sync::Arc;
use telperion_core::pipeline::growth::{self, GrowthState};
use telperion_core::{params, presets::Preset};

const SEED: u32 = 7;

/// FNV-1a over the `Debug` text of a value: every float to its last bit,
/// as the crate's tests digest a build (`tests/controls.rs`).
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

/// The tree at `age` as an engine reads it: a non-empty structure, wood and
/// leaf instances, and bounds holding every node. Returns its height and
/// the digest of the settled tree.
fn read(state: &GrowthState, age: f64, family: &telperion_core::Family) -> (f64, u64) {
    let tree = growth::settle(state, age).expect("the tree settles");
    let mesh = growth::mesh(state, age, family).expect("the tree is dressed");
    assert!(!tree.nodes.is_empty(), "a structure at {age}");
    assert!(mesh.wood_triangles() > 0, "wood at {age}");
    assert!(!mesh.curve.points.is_empty(), "the wood's curve at {age}");
    assert!(mesh.foliage_instances() > 0, "leaf instances at {age}");
    let b = mesh.bounds;
    for (i, n) in tree.nodes.iter().enumerate() {
        let p = n.position;
        let inside = (b.min.x..=b.max.x).contains(&p.x)
            && (b.min.y..=b.max.y).contains(&p.y)
            && (b.min.z..=b.max.z).contains(&p.z);
        assert!(inside, "node {i} at {age} lies outside {b:?}: {p:?}");
    }
    let height = tree
        .nodes
        .iter()
        .map(|n| n.position.y)
        .fold(f64::MIN, f64::max);
    (height, digest(&tree))
}

#[test]
fn a_native_entry_takes_parameters_and_a_seed_and_returns_the_trees_data_at_any_age() {
    let text = params::metadata(&Preset::OregonWhiteOak.parameters()).to_string();
    let wire: serde_json::Value = serde_json::from_str(&text).expect("parameter text");
    let family = params::parse(&wire).expect("the oak's parameters");
    let params = Arc::new(growth::Params::of(&family).expect("growth parameters"));

    let mut on = growth::begin(&params, SEED, 40.0).expect("begun for 40");
    growth::grow_to(&mut on, 20.0).expect("grown to 20");
    let (at_20, way) = read(&on, 20.0, &family);
    growth::grow_to(&mut on, 40.0).expect("grown on to 40");
    let (at_40, _) = read(&on, 40.0, &family);
    assert!(at_40 > at_20, "taller at 40 ({at_40}) than at 20 ({at_20})");

    let mut alone = growth::begin(&params, SEED, 20.0).expect("begun for 20");
    growth::grow_to(&mut alone, 20.0).expect("grown to 20 alone");
    let tree = growth::settle(&alone, 20.0).expect("the tree at 20 alone");
    assert_eq!(
        way,
        digest(&tree),
        "the tree at 20 on the way to 40 is the tree at 20 alone"
    );
}
