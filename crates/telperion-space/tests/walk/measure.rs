//! The shape measures a walk compares: total length, height, spread, and
//! each branch's base, tip and own length by lineage.
use super::Setting;
use std::collections::HashMap;
use telperion_space::{Structure, Vec3};

pub struct Shape {
    pub length: f64,
    pub height: f64,
    pub spread: f64,
    pub branches: HashMap<u64, (Vec3, Vec3, f64)>,
}

impl Shape {
    pub fn of(tree: &Structure) -> Self {
        let mut shape = Shape {
            length: 0.0,
            height: 0.0,
            spread: 0.0,
            branches: HashMap::with_capacity(tree.axes.len()),
        };
        for axis in &tree.axes {
            let mut from = axis.base;
            let mut own = 0.0;
            for p in &axis.phytomers {
                own += (p.tip - from).length();
                shape.height = shape.height.max(p.tip.z);
                shape.spread = shape.spread.max(p.tip.x.hypot(p.tip.y));
                from = p.tip;
            }
            shape.length += own;
            shape.branches.insert(axis.lineage, (axis.base, from, own));
        }
        shape
    }
}

/// The largest change between two trees, as a share of the larger tree's
/// length (for lengths) or height (for places), and what changed.
pub fn change(a: &Shape, b: &Shape) -> (f64, String) {
    let mut worst = (0.0, String::new());
    let mut note = |value: f64, what: &dyn Fn() -> String| {
        if value > worst.0 {
            worst = (value, what());
        }
    };
    let (length, height) = (a.length.max(b.length), a.height.max(b.height));
    note((a.length - b.length).abs() / length, &|| {
        "total length".into()
    });
    note((a.height - b.height).abs() / height, &|| "height".into());
    note((a.spread - b.spread).abs() / height, &|| "spread".into());
    for (key, &(base, tip, _)) in &a.branches {
        if let Some(&(base2, tip2, _)) = b.branches.get(key) {
            let moved = (base - base2).length().max((tip - tip2).length());
            note(moved / height, &|| format!("branch {key:016x}"));
        }
    }
    let (made, _) = made(a, b);
    note(made, &|| "wood made or unmade".into());
    worst
}

/// The wood on branches in one tree and not the other, as a share of the
/// larger tree's length, and the branch with the most of it.
pub fn made(a: &Shape, b: &Shape) -> (f64, Option<u64>) {
    let mut total = 0.0;
    let mut largest = (0.0, None);
    for (x, y) in [(a, b), (b, a)] {
        for (key, &(_, _, own)) in &x.branches {
            if !y.branches.contains_key(key) {
                total += own;
                if own >= largest.0 {
                    largest = (own, Some(*key));
                }
            }
        }
    }
    (total / a.length.max(b.length), largest.1)
}

/// One step of a walk: its ends in walk coordinates, its largest change
/// per unit of the coordinate, what changed, and the wood it made.
pub struct Step {
    pub from: f64,
    pub to: f64,
    pub slope: f64,
    pub what: String,
    pub made: f64,
}

/// Every step of the walk of `setting` at `seed`.
pub fn walk(setting: &Setting, seed: u64) -> Vec<Step> {
    let mut previous = setting.shape(setting.step(0), seed);
    (1..=super::STEPS)
        .map(|i| {
            let (from, to) = (setting.step(i - 1), setting.step(i));
            let next = setting.shape(to, seed);
            let (moved, what) = change(&previous, &next);
            let (made, _) = made(&previous, &next);
            previous = next;
            Step {
                from,
                to,
                slope: moved / (to - from).abs(),
                what,
                made,
            }
        })
        .collect()
}

/// The largest change over `from..to` split into `split` sub-steps, then
/// the largest over the worst sub-step split again, `levels` times: a
/// continuous change shrinks with its step, a jump does not.
pub fn refine(
    setting: &Setting,
    seed: u64,
    from: f64,
    to: f64,
    levels: u32,
    split: u32,
) -> Vec<f64> {
    let (mut from, mut to) = (from, to);
    let mut changes = vec![change(&setting.shape(from, seed), &setting.shape(to, seed)).0];
    for _ in 0..levels {
        let at = |k: u32| from + (to - from) * f64::from(k) / f64::from(split);
        let shapes: Vec<Shape> = (0..=split).map(|k| setting.shape(at(k), seed)).collect();
        let (k, worst) = (0..split)
            .map(|k| (k, change(&shapes[k as usize], &shapes[k as usize + 1]).0))
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .unwrap();
        changes.push(worst);
        (from, to) = (at(k), at(k + 1));
    }
    changes
}

/// Bisects `from..to`, over which branch `lineage` is made or unmade,
/// `rounds` times, and returns its wood as a share of the tree where it
/// last stood: what it was the moment the setting crossed its draw.
pub fn crossing(
    setting: &Setting,
    seed: u64,
    from: f64,
    to: f64,
    lineage: u64,
    rounds: u32,
) -> f64 {
    let stands = |x: f64| setting.shape(x, seed);
    let (mut a, mut b) = (from, to);
    let first = stands(a).branches.contains_key(&lineage);
    for _ in 0..rounds {
        let mid = (a + b) / 2.0;
        if stands(mid).branches.contains_key(&lineage) == first {
            a = mid;
        } else {
            b = mid;
        }
    }
    let side = if first { a } else { b };
    let shape = stands(side);
    shape.branches[&lineage].2 / shape.length
}
