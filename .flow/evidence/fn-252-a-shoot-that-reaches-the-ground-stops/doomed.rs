//! fn-252 R2a, before building: which kept axes outlive their age's
//! stated life today because what they bear still lives, per species and
//! age, and what they carry. An axis's life runs from the birth of its
//! chain in its age (a relay carries its axis's time on); an axis carried
//! on into another age, or relayed, has not ended. Measure only; copied
//! into `crates/telperion-core/examples/` to run, never committed there.
//!
//!   cargo run --profile ci -p telperion-core --example doomed -- <preset> <ages,..> <seeds,..>
use std::collections::BTreeMap;
use telperion_core::presets::Preset;
use telperion_space::{grow, Origin, Request};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, ages, seeds] = &args[..] else {
        return Err("usage: doomed <preset> <ages> <seeds>".into());
    };
    let list = |t: &str| -> Vec<u32> { t.split(',').map(|v| v.parse().expect("number")).collect() };
    let family0 = Preset::from_id(id).ok_or("no preset")?.parameters();
    let species = family0.space.species.clone().ok_or("no engine species")?;
    for (pa, s) in species.states.iter().enumerate() {
        if s.lifespan > 0.0 {
            println!(
                "  pa{pa}: lifespan {}, continuation {}, relay_ended {}, shedding {}",
                s.lifespan, s.continuation, s.relay_ended, s.shedding
            );
        }
    }
    for age in list(ages) {
        for seed in list(seeds) {
            let request = Request { age, seed: u64::from(seed), budget: 20_000_000, light: family0.space.light };
            let s = grow(&species, request).map_err(|e| e.to_string())?;
            let n = s.axes.len();
            let mut carried = vec![false; n];
            let mut start = vec![0u32; n];
            for (i, a) in s.axes.iter().enumerate() {
                start[i] = a.birth;
                match a.origin {
                    Origin::Continuation { parent } => carried[parent] = true,
                    Origin::Relay { parent, .. } => {
                        carried[parent] = true;
                        start[i] = start[parent];
                    }
                    _ => {}
                }
            }
            // Over its life, and how far past it (cycles).
            let past: Vec<Option<f64>> = (0..n)
                .map(|i| {
                    let a = &s.axes[i];
                    let st = &species.states[a.pa];
                    let lived = f64::from(age.saturating_sub(start[i]));
                    (!carried[i] && st.lifespan.is_finite() && lived > st.lifespan).then(|| lived - st.lifespan)
                })
                .collect();
            let mut sub = vec![0usize; n];
            let mut pas: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new(); n];
            for i in (0..n).rev() {
                sub[i] += s.axes[i].phytomers.len();
                *pas[i].entry(s.axes[i].pa).or_default() += 1;
                if let Some(p) = s.axes[i].origin.parent() {
                    sub[p] += sub[i];
                    let child = std::mem::take(&mut pas[i]);
                    for (k, v) in &child {
                        *pas[p].entry(*k).or_default() += v;
                    }
                    pas[i] = child;
                }
            }
            let total: usize = s.axes.iter().map(|a| a.phytomers.len()).sum();
            // Outermost axes past their life (none of their bearers is).
            let mut over = vec![false; n];
            let mut rows: BTreeMap<usize, (u32, usize, f64, f64, f64, BTreeMap<usize, usize>, usize)> = BTreeMap::new();
            for i in 0..n {
                let up = s.axes[i].origin.parent().is_some_and(|p| over[p]);
                over[i] = up || past[i].is_some();
                if up || past[i].is_none() {
                    continue;
                }
                let st = &species.states[s.axes[i].pa];
                let gone = past[i].unwrap() > st.shedding;
                let r = rows.entry(s.axes[i].pa).or_insert((0, 0, f64::INFINITY, 0.0, 0.0, BTreeMap::new(), 0));
                if gone {
                    r.6 += sub[i];
                }
                r.0 += 1;
                r.1 += sub[i];
                r.2 = r.2.min(s.axes[i].base.z);
                r.3 = r.3.max(s.axes[i].base.z);
                r.4 += f64::from(u8::from(gone));
                for (k, v) in &pas[i] {
                    *r.5.entry(*k).or_default() += v;
                }
            }
            println!("== {id} age {age} seed {seed}: {n} axes, {total} phytomers");
            for (pa, (count, phy, lo, hi, gone, carry, lost)) in rows {
                let c: Vec<String> = carry.iter().map(|(k, v)| format!("pa{k}:{v}")).collect();
                // The narrowed R2a: a terminal age dies at its life; an age
                // that carries on does so by its clock, by its chance.
                let (go, next) = match species.successor(pa) {
                    Some((q, p)) => (p, format!("carries on to pa{q} at {p:.2} (its shedding {})", species.states[q].shedding)),
                    None => (0.0, "terminal".into()),
                };
                println!(
                    "  pa{pa} [{next}]: dies {:.1}% of the tree's phytomers, carries on {:.1}% (outermost past life, both by chance)",
                    100.0 * (1.0 - go) * lost as f64 / total as f64,
                    100.0 * go * phy as f64 / total as f64
                );
                println!(
                    "  pa{pa}: {count} axes past their life ({gone} past life + shedding: {:.1}% of the tree's phytomers), {phy} phytomers ({:.1}%), based {lo:.2}-{hi:.2} m; they carry {}",
                    100.0 * lost as f64 / total as f64,
                    100.0 * phy as f64 / total as f64,
                    c.join(" ")
                );
            }
        }
    }
    Ok(())
}
