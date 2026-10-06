//! fn-211.4: how far one light value per fine root can serve its buds at
//! a cut, on today's whole oak laid in full. Each living bud's light in
//! the laid tree against its fine root's mean bud light (decision 7: a
//! fine subtree's buds read one recorded value); no record can do better
//! than this spread.
//!
//!   cargo run --profile ci -p telperion-space --example cutlight -- [ages] [seeds] [cuts]
#[path = "junctions/stats.rs"]
mod stats;

use stats::quantiles;
use telperion_space::{bud_light, oak, survey_laid, Expected, Light, Request};

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize, d: &str| -> Vec<f64> {
        let text = args.get(i).map_or(d, String::as_str);
        text.split(',')
            .map(|v| v.parse().expect("a number"))
            .collect()
    };
    let (ages, seeds, cuts) = (list(0, "40,80"), list(1, "1,7"), list(2, "1024"));
    let species = oak();
    for &age in &ages {
        let expected = Expected::new(&species, age as u32, 0);
        for &seed in &seeds {
            let request = Request {
                age: age as u32,
                seed: seed as u64,
                budget: 20_000_000,
                light: LIT,
            };
            for &cut in &cuts {
                let (tree, s) = survey_laid(&species, request, &expected, cut).expect("grows");
                let lights = bud_light(&tree, &species, &LIT);
                let mean_all = lights.iter().map(|l| l.1).sum::<f64>() / lights.len() as f64;
                let mut by_root = vec![(0.0f64, 0usize); s.roots.len()];
                for &(i, l) in &lights {
                    if let Some(r) = s.root_of[i] {
                        by_root[r].0 += l;
                        by_root[r].1 += 1;
                    }
                }
                let mut within = Vec::new();
                for &(i, l) in &lights {
                    if let Some(r) = s.root_of[i] {
                        within.push((l - by_root[r].0 / by_root[r].1 as f64).abs());
                    }
                }
                let lit_roots = by_root.iter().filter(|r| r.1 > 0).count();
                // One value and a gradient per root: each root's bud light
                // fitted by least squares on its buds' height alone, and on
                // their position (the residual is what such a record misses).
                let tip = |i: usize| {
                    let a = &tree.axes[i];
                    a.phytomers.last().map_or(a.base, |p| p.tip)
                };
                let mut buds: Vec<Vec<(f64, [f64; 3])>> = vec![Vec::new(); s.roots.len()];
                for &(i, l) in &lights {
                    if let Some(r) = s.root_of[i] {
                        let t = tip(i);
                        buds[r].push((l, [t.x, t.y, t.z]));
                    }
                }
                let (mut res_z, mut res_xyz) = (Vec::new(), Vec::new());
                for b in buds.iter().filter(|b| !b.is_empty()) {
                    res_z.extend(fit(b, &[2]));
                    res_xyz.extend(fit(b, &[0, 1, 2]));
                }
                let (_, z50, z95) = quantiles(&res_z);
                let (_, x50, x95) = quantiles(&res_xyz);
                println!(
                    "  a gradient per root: on height p50 {:.1} % p95 {:.1} %; on position p50 {:.1} % p95 {:.1} %",
                    100.0 * z50 / mean_all, 100.0 * z95 / mean_all, 100.0 * x50 / mean_all, 100.0 * x95 / mean_all
                );
                let (_, p50, p95) = quantiles(&within);
                println!(
                    "oak {age} s{seed} cut {cut}: {} roots ({lit_roots} with living buds), {} of {} living buds fine, mean bud light {mean_all:.3}; |bud - its root's mean| p50 {p50:.4} ({:.1} %) p95 {p95:.4} ({:.1} %); above the cut {} phytomers",
                    s.roots.len(),
                    within.len(),
                    lights.len(),
                    100.0 * p50 / mean_all,
                    100.0 * p95 / mean_all,
                    s.above
                );
            }
        }
    }
}

/// Each bud's |residual| after a least-squares fit of light on a constant
/// and the coordinates `on` of its position (ridge-damped, so a root of
/// few buds falls back to its mean).
#[allow(clippy::needless_range_loop)]
fn fit(buds: &[(f64, [f64; 3])], on: &[usize]) -> Vec<f64> {
    let n = buds.len() as f64;
    let k = on.len();
    let mean_l = buds.iter().map(|b| b.0).sum::<f64>() / n;
    let mean_x: Vec<f64> = on
        .iter()
        .map(|&c| buds.iter().map(|b| b.1[c]).sum::<f64>() / n)
        .collect();
    // Normal equations on centred coordinates, k <= 3.
    let mut a = [[0.0f64; 3]; 3];
    let mut y = [0.0f64; 3];
    for b in buds {
        let x: Vec<f64> = on.iter().zip(&mean_x).map(|(&c, m)| b.1[c] - m).collect();
        for i in 0..k {
            y[i] += x[i] * (b.0 - mean_l);
            for j in 0..k {
                a[i][j] += x[i] * x[j];
            }
        }
    }
    for (i, row) in a.iter_mut().enumerate().take(k) {
        row[i] += 1e-6;
    }
    let w = solve(a, y, k);
    buds.iter()
        .map(|b| {
            let pred = mean_l + (0..k).map(|i| w[i] * (b.1[on[i]] - mean_x[i])).sum::<f64>();
            (b.0 - pred).abs()
        })
        .collect()
}

#[allow(clippy::needless_range_loop)]
fn solve(mut a: [[f64; 3]; 3], mut y: [f64; 3], k: usize) -> [f64; 3] {
    for c in 0..k {
        let p = (c..k)
            .max_by(|&i, &j| a[i][c].abs().total_cmp(&a[j][c].abs()))
            .unwrap();
        a.swap(c, p);
        y.swap(c, p);
        for r in c + 1..k {
            let f = a[r][c] / a[c][c];
            for j in c..k {
                a[r][j] -= f * a[c][j];
            }
            y[r] -= f * y[c];
        }
    }
    let mut w = [0.0; 3];
    for c in (0..k).rev() {
        w[c] = (y[c] - (c + 1..k).map(|j| a[c][j] * w[j]).sum::<f64>()) / a[c][c];
    }
    w
}
