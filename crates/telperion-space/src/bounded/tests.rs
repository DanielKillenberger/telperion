use super::{grow_bounded, Bounded, Expected};
use crate::grow::{grow, Request};
use crate::light::Light;
use crate::structure::{Origin, Structure};

fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 2_000_000,
        light: Light::NEUTRAL,
    }
}

fn bits(s: &Structure) -> Vec<u64> {
    s.axes
        .iter()
        .flat_map(|a| &a.phytomers)
        .flat_map(|p| [p.radius, p.tip.x, p.tip.y, p.tip.z, p.heading.z].map(f64::to_bits))
        .collect()
}

/// Every order of resolution gives the one-pass tree, to the bit, on one
/// core and on several; and nothing drawn changes as later chains resolve.
#[test]
fn any_order_is_the_one_pass_tree() {
    for (species, age) in [(crate::beech(), 15), (crate::spruce(), 12)] {
        let expected = Expected::new(&species, age, 2);
        let one = grow_bounded(&species, request(age, 3), Some(1)).expect("grows");
        let four = grow_bounded(&species, request(age, 3), Some(4)).expect("grows");
        assert_eq!(bits(&one), bits(&four));
        let mut tree = Bounded::with(&species, request(age, 3), &expected).expect("settles");
        // Bearers first, the youngest chains first among the ready.
        let mut ready = vec![0];
        let mut drawn = Vec::new();
        while let Some(c) = ready.pop() {
            tree.resolve(c).expect("resolves");
            let s = tree.structure();
            drawn.push((
                c,
                tree.axes(c)
                    .iter()
                    .flat_map(|&i| s.axes[i].phytomers.iter().map(|p| p.radius.to_bits()))
                    .collect::<Vec<_>>(),
            ));
            ready.extend((c + 1..tree.chains()).filter(|&k| tree.parent(k) == Some(c)));
        }
        let s = tree.structure();
        for (c, radii) in drawn {
            let now: Vec<u64> = tree
                .axes(c)
                .iter()
                .flat_map(|&i| s.axes[i].phytomers.iter().map(|p| p.radius.to_bits()))
                .collect();
            assert_eq!(radii, now, "chain {c} changed after it was drawn");
        }
        assert_eq!(bits(&one), bits(tree.structure()));
    }
}

/// A chain's base meets its committed radius exactly, and its topology
/// is the reference engine's.
#[test]
fn a_chain_meets_its_committed_base() {
    let species = crate::beech();
    let reference = grow(&species, request(15, 2)).expect("grows");
    let mut tree = Bounded::new(&species, request(15, 2)).expect("settles");
    tree.resolve_all(1).expect("resolves");
    assert_eq!(tree.structure().axes.len(), reference.axes.len());
    for c in 0..tree.chains() {
        let first = tree
            .axes(c)
            .iter()
            .find_map(|&i| tree.structure().axes[i].phytomers.first());
        let Some(first) = first else { continue };
        let committed = tree.committed(c);
        assert!(
            (first.radius - committed).abs() <= 1e-12 * committed.max(1e-9),
            "chain {c}"
        );
        let i = tree.axes(c)[0];
        assert!(c == 0 || matches!(tree.structure().axes[i].origin, Origin::Lateral { .. }));
    }
}
