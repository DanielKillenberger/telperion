//! The junction example's arithmetic: quantiles, sibling spread, hashes
//! and the process's resident memory.
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use telperion_space::{Structure, Vec3};

/// p5, p50 and p95; NaN for none.
pub fn quantiles(values: &[f64]) -> (f64, f64, f64) {
    if values.is_empty() {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let mut v = values.to_vec();
    v.sort_by(f64::total_cmp);
    let q = |f: f64| v[((v.len() - 1) as f64 * f).round() as usize];
    (q(0.05), q(0.5), q(0.95))
}

/// p50, p99 and the most.
pub fn tail(values: &mut [f64]) -> (f64, f64, f64) {
    values.sort_by(f64::total_cmp);
    let q = |f: f64| values[((values.len() - 1) as f64 * f).round() as usize];
    (q(0.5), q(0.99), q(1.0))
}

/// The median, over sibling groups of three or more with a reference
/// spread, of the coefficient of variation of each side's radii; and the
/// groups counted.
pub fn cv_by_group<K: Hash + Eq + Copy>(rows: &[(K, f64, f64)]) -> (f64, f64, usize) {
    let mut groups: HashMap<K, (Vec<f64>, Vec<f64>)> = HashMap::new();
    for &(k, ours, reference) in rows {
        let g = groups.entry(k).or_default();
        g.0.push(ours);
        g.1.push(reference);
    }
    let cv = |v: &[f64]| {
        let n = v.len() as f64;
        let mean = v.iter().sum::<f64>() / n;
        let var = v.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / n;
        if mean > 0.0 {
            var.sqrt() / mean
        } else {
            0.0
        }
    };
    let (mut ours, mut reference) = (Vec::new(), Vec::new());
    for (a, b) in groups.values().filter(|g| g.0.len() >= 3) {
        ours.push(cv(a));
        reference.push(cv(b));
    }
    let n = ours.len();
    (quantiles(&ours).1, quantiles(&reference).1, n)
}

/// A tree's hash over every public field, to the bit.
pub fn hash(tree: &Structure) -> u64 {
    let mut h = DefaultHasher::new();
    let v = |h: &mut DefaultHasher, v: Vec3| [v.x, v.y, v.z].map(|c| c.to_bits()).hash(h);
    (tree.age, tree.pas).hash(&mut h);
    for axis in &tree.axes {
        (axis.lineage, axis.pa, axis.birth, axis.apex_end).hash(&mut h);
        format!("{:?}", axis.origin).hash(&mut h);
        axis.vigour.to_bits().hash(&mut h);
        v(&mut h, axis.base);
        v(&mut h, axis.heading);
        v(&mut h, axis.side);
        for p in &axis.phytomers {
            (p.cycle, p.radius.to_bits(), p.scale.to_bits()).hash(&mut h);
            v(&mut h, p.tip);
            v(&mut h, p.heading);
            v(&mut h, p.side);
        }
    }
    h.finish()
}

/// The process's resident memory now, in bytes (Linux).
pub fn rss() -> f64 {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    status
        .lines()
        .find_map(|l| l.strip_prefix("VmRSS:"))
        .and_then(|v| v.split_whitespace().next()?.parse::<f64>().ok())
        .map_or(0.0, |kb| kb * 1024.0)
}
