//! fn-252 R2's open point: of the axes the final lay rests on the ground,
//! how many the rough layout growth reads (`telperion_space::sketch`, laid
//! by `grow/draw.rs` with no sag load) already brings to the ground, and
//! how high it holds the rest; and how far the hanging strands drop in
//! each. Measure only; copied into `crates/telperion-core/examples/` to
//! run, never committed there.
//!
//!   cargo run --profile ci -p telperion-core --example rough -- <preset> <ages,..> <seeds,..> [strand pa]
use telperion_core::presets::Preset;
use telperion_space::{grow, sketch, Origin, Request, Structure};

/// Wood this low rests on the ground (m).
const ON: f64 = 1e-3;

fn quart(mut v: Vec<f64>) -> String {
    if v.is_empty() {
        return "-".into();
    }
    v.sort_by(f64::total_cmp);
    let q = |p: f64| v[((v.len() - 1) as f64 * p).round() as usize];
    format!("{:.2}/{:.2}/{:.2}/{:.2}", q(0.1), q(0.5), q(0.9), v[v.len() - 1])
}

/// Which axes are stem: the seed and what carries it on in a stem age.
fn stems(s: &Structure, trunk: &[usize]) -> Vec<bool> {
    let mut stem = vec![false; s.axes.len()];
    for (i, a) in s.axes.iter().enumerate() {
        stem[i] = match a.origin {
            Origin::Seed => true,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                stem[parent] && trunk.contains(&a.pa)
            }
            Origin::Lateral { .. } => false,
        };
    }
    stem
}

/// An axis's drop: its base's height less its lowest node's.
fn drop(a: &telperion_space::Axis) -> f64 {
    a.base.z - a.phytomers.iter().map(|p| p.tip.z).fold(a.base.z, f64::min)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, ages, seeds, rest @ ..] = &args[..] else {
        return Err("usage: rough <preset> <ages> <seeds> [strand pa]".into());
    };
    let list = |t: &str| -> Vec<u32> { t.split(',').map(|v| v.parse().expect("number")).collect() };
    let strand: Option<usize> = rest.first().map(|v| v.parse().expect("pa"));
    let family = Preset::from_id(id).ok_or("no preset")?.parameters();
    let species = family.space.species.clone().ok_or("no engine species")?;
    println!("{id}: light extinction {}", family.space.light.extinction);
    for age in list(ages) {
        for seed in list(seeds) {
            let request = Request { age, seed: u64::from(seed), budget: 20_000_000, light: family.space.light };
            let laid = grow(&species, request).map_err(|e| e.to_string())?;
            let rough = match sketch(&species, request) {
                Ok(r) => r,
                Err(e) => {
                    println!("== {id} age {age} seed {seed}: no rough layout ({e})");
                    continue;
                }
            };
            let same = laid.axes.len() == rough.axes.len()
                && laid.axes.iter().zip(&rough.axes).all(|(a, b)| a.lineage == b.lineage && a.phytomers.len() == b.phytomers.len());
            // Axes matched by lineage, age and birth: the two paths may keep
            // different axes (one drops what it is certain to shed).
            let by: std::collections::HashMap<(u64, usize, u32), usize> = rough
                .axes
                .iter()
                .enumerate()
                .map(|(k, a)| ((a.lineage, a.pa, a.birth), k))
                .collect();
            let twin = |a: &telperion_space::Axis| {
                by.get(&(a.lineage, a.pa, a.birth))
                    .map(|&k| &rough.axes[k])
                    .filter(|r| r.phytomers.len() == a.phytomers.len())
            };
            let mut unmatched = 0;
            let stem = stems(&laid, &family.space.trunk);
            let (mut resting, mut touch, mut near) = (0, 0, 0);
            let mut above: Vec<f64> = Vec::new();
            let mut lowest_rough: Vec<f64> = Vec::new();
            let (mut on_final, mut on_rough) = (0usize, 0usize);
            for (i, a) in laid.axes.iter().enumerate() {
                if stem[i] {
                    continue;
                }
                on_final += a.phytomers.iter().filter(|p| p.tip.z < ON).count();
                let Some(r) = twin(a) else {
                    unmatched += usize::from(a.phytomers.iter().any(|p| p.tip.z < ON));
                    continue;
                };
                on_rough += r.phytomers.iter().filter(|p| p.tip.z < ON).count();
                if !a.phytomers.iter().any(|p| p.tip.z < ON) {
                    continue;
                }
                resting += 1;
                let low = r.phytomers.iter().map(|p| p.tip.z).fold(r.base.z, f64::min);
                let internode = species.states[a.pa].internode;
                lowest_rough.push(low);
                if low < ON {
                    touch += 1;
                } else if low <= internode {
                    near += 1;
                } else {
                    above.push(low);
                }
            }
            println!(
                "== {id} age {age} seed {seed}: rough and final the same axes: {same}; resting phytomers final {on_final}, rough {on_rough}; resting axes with no rough twin {unmatched}"
            );
            if resting > 0 {
                println!(
                    "  {resting} axes rest in the final lay: in the rough lay {touch} touch ({:.0}%), {near} within one internode ({:.0}%), {} held above ({:.0}%); their lowest rough node p10/p50/p90/max {} m; of those held above {} m",
                    100.0 * touch as f64 / resting as f64,
                    100.0 * near as f64 / resting as f64,
                    above.len(),
                    100.0 * above.len() as f64 / resting as f64,
                    quart(lowest_rough),
                    quart(above.clone())
                );
            }
            if let Some(pa) = strand {
                let drops = |s: &Structure| -> Vec<f64> {
                    s.axes.iter().filter(|a| a.pa == pa && !a.phytomers.is_empty()).map(drop).collect()
                };
                let heads = |s: &Structure| -> Vec<f64> {
                    s.axes.iter().filter(|a| a.pa == pa).flat_map(|a| a.phytomers.iter().map(|p| p.heading.z)).collect()
                };
                println!(
                    "  strands pa{pa}: drop p10/p50/p90/max rough {} m, final {} m; heading z rough {}, final {}",
                    quart(drops(&rough)),
                    quart(drops(&laid)),
                    quart(heads(&rough)),
                    quart(heads(&laid))
                );
            }
        }
    }
    Ok(())
}
