//! Measurements, not tests (fn-209 STEP1-C.md), run by hand with
//! `--ignored`: the closed form's kept quantities and extent bound against
//! the beech and the spruce.
use super::super::{Kept, LEAVES, MASS, SECTION, WOOD};
use super::bound::farthest;
use super::{about, raw, tree};
use crate::presence::Windows;
use crate::schedule::Carried;
use crate::species::Species;
use crate::structure::Origin;
use std::collections::HashMap;

/// A measurement, not a test (fn-209 STEP1-C.md): the beech's and the
/// spruce's kept wood, base sections and mass against the formula, by PA
/// and age band, every lateral made counted; and how near the extent
/// bound their wood reaches.
#[test]
#[ignore]
fn measure_production() {
    for (name, make) in [
        ("beech", crate::beech as fn() -> Species),
        ("spruce", crate::spruce),
    ] {
        let species = make();
        let mut twin = species.clone();
        twin.states
            .iter_mut()
            .for_each(|s| s.shedding = f64::INFINITY);
        let age = 25u32;
        let windows = Windows::new(&species, age);
        let started = std::time::Instant::now();
        let mut kept = Kept::new(&species, &windows);
        // by (pa, age band of 5 cycles): realized and formula wood, and made count
        let mut acc: std::collections::BTreeMap<(usize, u32), [f64; 7]> = Default::default();
        let mut tight: Vec<f64> = Vec::new();
        let mut prep = 0.0;
        for seed in 0..4 {
            let (Ok(grown), Ok(whole)) = (tree(&species, age, seed), tree(&twin, age, seed)) else {
                continue;
            };
            let sums = raw(&grown, &species);
            let reached = farthest(&grown);
            for (i, axis) in grown.axes.iter().enumerate() {
                let Origin::Lateral { parent, node, .. } = axis.origin else {
                    continue;
                };
                let base = grown.axes[parent].phytomers[node].scale * axis.vigour;
                let m = (age - axis.birth) as usize;
                let bound = kept.reach_bound(axis.pa, m, Carried::default(), None) * base;
                if bound > 0.0 {
                    tight.push(reached[i] / bound);
                }
            }
            let mut by: HashMap<u64, usize> = HashMap::new();
            for (i, a) in grown.axes.iter().enumerate() {
                by.entry(a.lineage).or_insert(i);
            }
            for axis in &whole.axes {
                let Origin::Lateral {
                    parent,
                    node,
                    woken: false,
                    ..
                } = axis.origin
                else {
                    continue;
                };
                let base = whole.axes[parent].phytomers[node].scale * axis.vigour;
                if base <= 0.0 {
                    continue;
                }
                let m = (age - axis.birth) as usize;
                let t0 = std::time::Instant::now();
                let delay = species.states[axis.pa].shedding;
                let t = kept.shed((axis.pa, m, Carried::default(), None), delay);
                prep += t0.elapsed().as_secs_f64();
                let e = species.states[axis.pa].form.exponent;
                let a = acc.entry((axis.pa, m as u32 / 5)).or_default();
                a[0] += 1.0;
                a[2] += t.b[m][WOOD] * base;
                a[4] += t.b[m][SECTION] * base.powf(e);
                a[6] += (t.b[m][MASS] + t.b[m][LEAVES]) * base.powi(3);
                if let Some(&k) = by.get(&axis.lineage) {
                    let v = about(&grown, &sums, k, e);
                    a[1] += v[0];
                    a[5] += v[3];
                    a[3] += grown.axes[k]
                        .phytomers
                        .first()
                        .map_or(0.0, |p| p.radius.powf(e));
                }
            }
        }
        println!(
            "{name}: tables {:.2} s (first use, all included {:.2} s)",
            prep,
            started.elapsed().as_secs_f64()
        );
        for ((pa, band), a) in acc.iter().filter(|(_, a)| a[0] >= 100.0) {
            println!(
                "  {name} PA {pa} age {}-{} ({:.0} made): realized/formula wood {:.3}, section {:.3}, mass {:.3}",
                band * 5,
                band * 5 + 4,
                a[0],
                a[1] / a[2],
                a[3] / a[4],
                a[5] / a[6]
            );
        }
        tight.sort_by(f64::total_cmp);
        let q = |f: f64| tight[((tight.len() - 1) as f64 * f) as usize];
        println!(
            "  {name}: reach / extent bound over {} laterals: median {:.3}, p99 {:.3}, max {:.3}",
            tight.len(),
            q(0.5),
            q(0.99),
            q(1.0)
        );
    }
}
