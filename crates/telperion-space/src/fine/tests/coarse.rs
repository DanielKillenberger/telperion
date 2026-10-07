//! The coarse tree (fn-214): the structure above the cut grown as
//! `grow_fine` grows it, each fine root standing as its expectations.
use super::request;
use crate::fine::{grow_coarse, grow_fine, Plan};
use crate::structure::Structure;

fn kept(s: &Structure) -> usize {
    s.axes.iter().map(|a| a.phytomers.len()).sum()
}

/// At cut 0 only the buds expected to keep nothing are fine: the coarse
/// tree holds every phytomer of the tree where it stands, at its radius,
/// and no stand.
#[test]
fn at_cut_zero_the_coarse_tree_is_the_tree() {
    let species = crate::spruce();
    let coarse = grow_coarse(&species, request(12, 3), 0.0).expect("grows");
    let plan = Plan {
        cut: 0.0,
        ..Plan::default()
    };
    let fine = grow_fine(&species, request(12, 3), plan).expect("grows");
    assert!(coarse.stands.is_empty());
    let wood = |s: &Structure| {
        let mut w: Vec<[u64; 4]> = s
            .axes
            .iter()
            .flat_map(|a| &a.phytomers)
            .map(|p| {
                [
                    p.tip.x.to_bits(),
                    p.tip.y.to_bits(),
                    p.tip.z.to_bits(),
                    p.radius.to_bits(),
                ]
            })
            .collect();
        w.sort_unstable();
        w
    };
    assert_eq!(wood(&coarse.structure), wood(&fine.structure));
}

/// Above the cut the coarse tree grows what `grow_fine` grows; below it
/// each root stands for its predictor's expected kept phytomers, and the
/// trunk carries their expected sections near what their realized wood
/// gives it.
#[test]
fn below_the_cut_each_root_stands_for_its_expectation() {
    let species = crate::spruce();
    let coarse = grow_coarse(&species, request(20, 3), 1024.0).expect("grows");
    let fine = grow_fine(&species, request(20, 3), Plan::default()).expect("grows");
    let realized: usize = fine.roots.iter().map(|r| r.kept).sum();
    assert_eq!(kept(&coarse.structure), kept(&fine.structure) - realized);
    let expected: f64 = coarse.stands.iter().map(|s| s.phytomers).sum();
    let predicted: f64 = fine.roots.iter().map(|r| r.predicted).sum();
    assert!(!coarse.stands.is_empty());
    assert!(
        (expected / predicted - 1.0).abs() < 1e-9,
        "{expected} {predicted}"
    );
    for s in &coarse.stands {
        assert!(
            s.radius >= 0.0 && s.centre >= 0.0 && s.phytomers > 0.0,
            "{s:?}"
        );
        assert!(
            s.heading.length() > 0.99 && s.heading.length() < 1.01,
            "{s:?}"
        );
    }
    let base = |s: &Structure| s.axes[0].phytomers[0].radius;
    let ratio = base(&coarse.structure) / base(&fine.structure);
    assert!(
        (ratio - 1.0).abs() < 0.1,
        "base radius coarse / grown {ratio}"
    );
}

/// A species whose structure above the cut reads its fine roots as grown
/// is refused a coarse tree by name.
#[test]
fn a_species_that_reads_its_tree_as_grown_is_refused() {
    let refused = grow_coarse(&crate::oak(), request(5, 1), 1024.0);
    assert!(
        matches!(refused, Err(crate::Error::Refused { .. })),
        "{:?}",
        refused.err()
    );
}
