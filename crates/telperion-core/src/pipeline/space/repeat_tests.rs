//! fn-254 R4 checks 1 and 3 on every shipped preset: a tree is the same
//! tree, to the bit, built again and on one core or four, and it keeps
//! its invariants (`telperion_space::invariants`). Each build is folded
//! and dropped before the next, so one tree is held at a time (fn-274 R1).
use super::*;
use crate::presets::{by_identity, CATALOGUE};
use std::fmt::Write as _;
use std::hash::{DefaultHasher, Hasher};

fn grown(family: &crate::Family) -> telperion_space::Structure {
    // The species as production grows it: its supernatural overrides
    // applied (fn-254 review).
    let species = family.applied().space.species.clone().unwrap();
    let request = Request {
        age: family.age.round() as u32,
        seed: u64::from(family.skeleton.seed),
        budget: BUDGET,
        light: family.space.light,
    };
    telperion_space::grow(&species, request).unwrap()
}

fn on(threads: usize, family: &crate::Family) -> telperion_space::Structure {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();
    pool.install(|| grown(family))
}

/// Every field of the tree in build order, folded as it is printed. The
/// derived `Debug` reaches every field, crate-private ones too, and prints
/// each float in the shortest form that reads back to its bits, so two
/// trees fold alike where they are equal field by field, axis by axis.
/// Every NaN prints alike, so a NaN fails, as it failed `==`.
fn print(tree: &telperion_space::Structure) -> u64 {
    struct Fold(DefaultHasher);
    impl std::fmt::Write for Fold {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            assert!(!s.contains("NaN"), "a NaN in the tree");
            self.0.write(s.as_bytes());
            Ok(())
        }
    }
    let mut fold = Fold(DefaultHasher::new());
    write!(fold, "{tree:?}").unwrap();
    fold.0.finish()
}

/// One preset grown three times: twice on four cores, once on one.
fn grows_the_same_tree_again_and_on_any_cores(id: &str) {
    let family = by_identity(id).unwrap();
    let first = {
        let tree = on(4, &family);
        let broken = telperion_space::invariants::invariants(&tree);
        assert!(broken.is_empty(), "{id}: {broken:#?}");
        println!("{id}: {} axes", tree.axes.len());
        print(&tree)
    };
    assert_eq!(first, print(&on(4, &family)), "{id}: built again");
    assert_eq!(first, print(&on(1, &family)), "{id}: on one core");
}

/// The pipeline's determinism on the two presets that exercise the most
/// stages: Telperion, the largest tree, and the date palm, the one stem
/// with fronds (host, fn-304 ruling 7). Every preset is the ignored test
/// below.
#[test]
fn the_largest_tree_and_the_palm_grow_the_same_tree_again_and_on_any_cores() {
    let _turn = crate::one_tree::turn();
    the_print_tells_a_bit_and_a_build_order_apart();
    for id in ["telperion", "date-palm"] {
        grows_the_same_tree_again_and_on_any_cores(id);
    }
}

/// `cargo test --profile ci -p telperion-core --lib repeat_tests -- --ignored`
#[test]
#[ignore = "fn-304: the full catalogue sweep, outside the gate; about 3 minutes"]
fn every_preset_grows_the_same_tree_again_and_on_any_cores() {
    let _turn = crate::one_tree::turn();
    for &(_, id, _, _) in CATALOGUE {
        grows_the_same_tree_again_and_on_any_cores(id);
    }
}

/// The print tells one bit apart, and one tree built in another order:
/// two axes that bear nothing swapped are the same tree by lineage
/// (`telperion_space::digest`) and another build. A NaN is refused.
fn the_print_tells_a_bit_and_a_build_order_apart() {
    let tree = on(4, &by_identity("ordinary").unwrap());
    let base = print(&tree);
    let n = tree.axes.len();
    let bears = |i: usize| tree.axes.iter().any(|a| a.origin.parent() == Some(i));
    assert!(
        !bears(n - 1) && !bears(n - 2),
        "the last two axes bear nothing"
    );

    let mut bit = tree.clone();
    let p = &mut bit.axes[0].phytomers[0];
    p.radius = f64::from_bits(p.radius.to_bits() ^ 1);
    assert_ne!(print(&bit), base, "one bit");

    let mut swapped = tree.clone();
    swapped.axes.swap(n - 1, n - 2);
    assert_eq!(
        telperion_space::digest(&swapped),
        telperion_space::digest(&tree)
    );
    assert_ne!(print(&swapped), base, "build order");

    let mut nan = tree.clone();
    nan.axes[0].base.x = f64::NAN;
    let refused = std::panic::catch_unwind(|| print(&nan));
    assert!(refused.is_err(), "a NaN");
}
