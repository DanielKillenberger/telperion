//! S1 (fn-217 section 7.2): the scale draws the natural tree enlarged,
//! once, on every output the pipeline builds from it.
use crate::pipeline::{self, executor, grown_envelope, Request};
use crate::presets::{Family, Preset};
use crate::tree::Tree;

/// The young beech at `scale`, the group on.
fn beech(scale: f64) -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    (f.age, f.skeleton.seed) = (12.0, 1);
    f.supernatural.enabled = true;
    f.supernatural.scale = scale;
    f
}

fn built(f: &Family) -> pipeline::Built {
    let request = Request {
        leaves: true,
        field: Some(None),
        ..Request::default()
    };
    pipeline::build(f, request).unwrap()
}

/// Every node at its natural place times `s`, every radius and every
/// length of grown wood times `s`, and its topology, frames and blooms the
/// natural tree's.
fn enlarged(tree: &Tree, natural: &Tree, s: f64) {
    assert_eq!(tree.nodes.len(), natural.nodes.len());
    for (n, m) in tree.nodes.iter().zip(&natural.nodes) {
        assert_eq!(
            (n.parent, n.branch, n.kind, n.stem),
            (m.parent, m.branch, m.kind, m.stem)
        );
        assert_eq!((n.codominant, n.identity), (m.codominant, m.identity));
        assert_eq!(n.position, m.position * s);
        let radii = [n.radius, n.start_radius, n.base_radius];
        assert_eq!(radii, [m.radius * s, m.start_radius * s, m.base_radius * s]);
    }
    let grown: Vec<f64> = natural.grown.iter().map(|g| g * s).collect();
    assert_eq!(tree.grown, grown);
    assert_eq!((&tree.frames, &tree.keys), (&natural.frames, &natural.keys));
}

#[test]
fn the_scale_draws_the_natural_tree_enlarged() {
    let natural = built(&beech(1.0));
    let mut plain = beech(1.0);
    plain.supernatural.enabled = false;
    // One is the natural tree to the node.
    enlarged(&natural.skeleton.tree, &built(&plain).skeleton.tree, 1.0);
    let leaves = |b: &pipeline::Built| b.outputs.leaves.as_ref().unwrap().retained;
    let close = |a: f64, b: f64| (a - b).abs() <= 1e-9 * b.abs().max(1.0);
    for s in [0.1, 100.0] {
        let b = built(&beech(s));
        enlarged(&b.skeleton.tree, &natural.skeleton.tree, s);
        // The whole-tree folds are taken after the scale: the envelope is
        // the natural one enlarged, its shares the natural ones.
        let (e, n) = (
            grown_envelope(&b.skeleton.tree),
            grown_envelope(&natural.skeleton.tree),
        );
        assert!(
            close(e.height, n.height * s),
            "{s}: {} {}",
            e.height,
            n.height
        );
        for (a, b) in [
            (e.crown_base, n.crown_base),
            (e.spread, n.spread),
            (e.fullness, n.fullness),
        ] {
            assert!(close(a, b), "{s}: {a} against {b}");
        }
        // The field reads the enlarged wood.
        let wood = |b: &pipeline::Built| b.outputs.field.as_ref().unwrap().snapshot().unwrap().wood;
        let (mine, theirs) = (wood(&b), wood(&natural));
        assert_eq!(mine.len(), theirs.len());
        assert!(
            mine.iter().zip(&theirs).all(|(a, t)| close(*a, t * s)),
            "{s}: the field's wood"
        );
        // Leaves keep their spacing along the enlarged wood and their size;
        // a short shoot's cluster keeps its count (fn-248).
        let tips = |b: &pipeline::Built| -> f64 {
            b.skeleton
                .tree
                .clusters
                .iter()
                .map(|c| f64::from(c.leaves))
                .sum()
        };
        let count = leaves(&b) as f64 - tips(&b);
        let base = leaves(&natural) as f64 - tips(&natural);
        if s > 1.0 {
            assert!(count > 0.5 * s * base, "{s}: {count} leaves against {base}");
        } else {
            assert!(count < base, "{s}: {count} leaves against {base}");
        }
        let size = |b: &pipeline::Built| {
            let i = &b.outputs.leaves.as_ref().unwrap().instances;
            let all: Vec<f64> = (0..i.leaves.len())
                .map(|k| i.reference.scale(i.leaves[k]))
                .collect();
            all.iter().sum::<f64>() / all.len() as f64
        };
        let (mine, theirs) = (size(&b), size(&natural));
        assert!(
            (mine / theirs - 1.0).abs() < 0.05,
            "{s}: leaf size {mine} against {theirs}"
        );
    }
}

/// The executor's step reads the scale through the one stage input, so it
/// grows the tree the pipeline grows.
#[test]
fn the_executor_step_draws_the_same_enlarged_tree() {
    for s in [0.1, 100.0] {
        let f = beech(s);
        let step = executor::grow(&f).unwrap().expansion().unwrap();
        enlarged(step.tree(), &built(&f).skeleton.tree, 1.0);
    }
}

/// A scale off its range is refused by name before anything grows.
#[test]
fn a_scale_off_its_range_is_refused_by_the_build() {
    for s in [0.05, 101.0] {
        let refused = pipeline::build(&beech(s), Request::mesh()).err();
        assert!(
            matches!(
                refused,
                Some(crate::Error::InvalidValue { field: "scale", .. })
            ),
            "{s}: {refused:?}"
        );
    }
}

/// Contact precision (Astra's review): a leaf's position is packed against
/// the box of the enlarged wood, so its code step grows with the scale.
/// Reported for the record at each scale against the leaf's own length, and
/// held to half a step: the packing loses no more at any scale than its
/// box's step.
#[test]
fn contact_precision_at_every_scale() {
    for s in [0.1, 1.0, 5.0, 100.0] {
        contact(&beech(s), &format!("scale {s}"));
    }
}

/// Laurelin at her own age and scale (fn-218 R3, fn-217 8.4): the contact
/// error of the shipped preset, held to the same half step, which is inside
/// fn-125's screen of two steps. A build of eleven million leaves, so run by
/// name (`--ignored`) when her values change.
#[test]
#[ignore = "measurement: the laurelin preset at 80, eleven million leaves"]
fn laurelin_contact_at_her_scale() {
    let f = Preset::Laurelin.parameters();
    contact(
        &f,
        &format!("laurelin {} at scale {}", f.age, f.supernatural.scale),
    );
}

/// Telperion at his own age and scale (fn-219 R3), as Laurelin's above.
#[test]
#[ignore = "measurement: the telperion preset at his own age"]
fn telperion_contact_at_his_scale() {
    let f = Preset::Telperion.parameters();
    contact(
        &f,
        &format!("telperion {} at scale {}", f.age, f.supernatural.scale),
    );
}

/// Builds `f` with its leaves' transforms recorded, holds every packed
/// position to half its box's step, and prints the worst error against the
/// smallest leaf.
fn contact(f: &Family, label: &str) {
    use crate::foliage::RECORD_UNQUANTISED;
    RECORD_UNQUANTISED.with(|r| r.set(true));
    let request = Request {
        leaves: true,
        schedule: pipeline::Schedule::Serial,
        ..Request::default()
    };
    let b = pipeline::build(f, request).unwrap();
    RECORD_UNQUANTISED.with(|r| r.set(false));
    let placed = &b.outputs.leaves.as_ref().unwrap().instances;
    assert_eq!(placed.unquantised.len(), placed.leaves.len());
    let step = placed.reference.step();
    // The element's own length along its longest side, in metres at a
    // scale of one.
    let e = &b.outputs.element.as_ref().unwrap().positions;
    let side = |f: fn(&crate::math::Vec3) -> f64| {
        let (lo, hi) = e
            .iter()
            .map(f)
            .fold((f64::MAX, f64::MIN), |(l, h), v| (l.min(v), h.max(v)));
        hi - lo
    };
    let element = side(|p| p.x).max(side(|p| p.y)).max(side(|p| p.z));
    let (mut worst, mut length) = (0.0f64, f64::MAX);
    for (k, before) in placed.unquantised.iter().enumerate() {
        let after = placed.matrix(k);
        for (axis, step) in [step.x, step.y, step.z].into_iter().enumerate() {
            let d = (f64::from(before[12 + axis]) - f64::from(after[12 + axis])).abs();
            let ulp = f64::from(before[12 + axis].abs()) * f64::from(f32::EPSILON) + 1e-9;
            assert!(
                d <= step / 2.0 + ulp,
                "{label}: leaf {k} moved {d} m on axis {axis}"
            );
            worst = worst.max(d);
        }
        length = length.min(element * placed.reference.scale(placed.leaves[k]));
    }
    println!(
        "{label}: {} leaves, step {:.2e} m, worst contact error {worst:.2e} m, \
         {:.2}% of the smallest leaf ({length:.3} m)",
        placed.leaves.len(),
        step.length(),
        100.0 * worst / length
    );
}
