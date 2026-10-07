//! fn-214's probe: the coarse tree (its fine roots standing as their
//! expectations) against the tree grown whole, one core.
//!
//!   RAYON_NUM_THREADS=1 cargo run --release -p telperion-space --example coarse -- [cut] [age] [seeds]
use std::time::Instant;
use telperion_space::{grow_coarse, grow_fine, palm, spruce, Light, Plan, Request, Structure};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cut: f64 = args.first().map_or(1024.0, |a| a.parse().expect("a cut"));
    let age: u32 = args.get(1).map_or(40, |a| a.parse().expect("an age"));
    let seeds: Vec<u64> = args
        .get(2)
        .map_or("1234,5678", String::as_str)
        .split(',')
        .map(|s| s.parse().expect("a seed"))
        .collect();
    let kept = |s: &Structure| s.axes.iter().map(|a| a.phytomers.len()).sum::<usize>();
    for (name, species) in [("spruce", spruce()), ("palm", palm())] {
        for &seed in &seeds {
            let request = Request {
                age,
                seed,
                budget: 20_000_000,
                light: Light::default(),
            };
            let t = Instant::now();
            let fine = grow_fine(
                &species,
                request,
                Plan {
                    cut,
                    ..Plan::default()
                },
            )
            .expect("grows");
            let grown_ms = t.elapsed().as_secs_f64() * 1e3;
            let realized: usize = fine.roots.iter().map(|r| r.kept).sum();
            let t = Instant::now();
            let coarse = grow_coarse(&species, request, cut).expect("grows coarse");
            let coarse_ms = t.elapsed().as_secs_f64() * 1e3;
            let expected: f64 = coarse.stands.iter().map(|s| s.phytomers).sum();
            let mut radii: Vec<f64> = coarse.stands.iter().map(|s| s.radius).collect();
            let mut centres: Vec<f64> = coarse.stands.iter().map(|s| s.centre).collect();
            radii.sort_by(f64::total_cmp);
            centres.sort_by(f64::total_cmp);
            let at = |v: &[f64], f: f64| {
                v.get(((v.len().max(1) - 1) as f64 * f) as usize)
                    .copied()
                    .unwrap_or(0.0)
            };
            let base = |s: &Structure| s.axes[0].phytomers[0].radius;
            println!(
                "{name} {age} seed {seed}: grown {grown_ms:.0} ms, {} roots ({} shed), {} kept below the cut; coarse {coarse_ms:.0} ms, {} stands for {expected:.0} expected; above the cut {} / {} phytomers; stand radius p10/50/90 {:.4}/{:.4}/{:.4} m, centre {:.3}/{:.3}/{:.3} m; base radius grown {:.3} m, coarse {:.3} m",
                fine.roots.len(), fine.roots.iter().filter(|r| r.at.is_none()).count(), realized,
                coarse.stands.len(), kept(&coarse.structure), kept(&fine.structure) - realized,
                at(&radii, 0.1), at(&radii, 0.5), at(&radii, 0.9), at(&centres, 0.1), at(&centres, 0.5), at(&centres, 0.9),
                base(&fine.structure), base(&coarse.structure),
            );
        }
    }
}
