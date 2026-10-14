//! fn-251's oracle on every walk frame of the path (fn-272 R3): the frame
//! the film draws, grown by the build's one cycle at the path's seed and
//! the site's light, keeps and grows a phytomer count within the band of
//! both ends' counts over seeds 1, 7 and 42 at that age and light, widened
//! by 10 % or two phytomers, whichever is wider (`tests/walk_sizes.rs`).
//! A frame whose species is an end's grows as that end does at the seed,
//! which the band already holds, so it is not grown again.
use super::path::{ease, Frame, Kind, Path};
use telperion_core::{blend, presets::by_identity, Family};
use telperion_space::{grow_cycle, Request, Species};

/// The engine's phytomer budget, the build's own (`pipeline/space.rs`).
const BUDGET: u32 = 20_000_000;
const WIDEN: f64 = 0.1;
const SLACK: f64 = 2.0;
const SEEDS: [u64; 3] = [1, 7, 42];

fn species(f: &Family) -> Result<&Species, String> {
    f.space.species.as_ref().ok_or("a walk frame carries an engine species".into())
}

/// Kept and grown phytomers at `age` and `seed` under the family's light.
fn counts(f: &Family, seed: u64) -> Result<[f64; 2], String> {
    let request = Request { age: f.age as u32, seed, budget: BUDGET, light: f.space.light };
    let cycled = grow_cycle(species(f)?, request).map_err(|e| e.to_string())?;
    let kept: usize = cycled.structure.axes.iter().map(|a| a.phytomers.len()).sum();
    Ok([kept as f64, cycled.grown as f64])
}

fn within(n: f64, lo: f64, hi: f64) -> bool {
    let low = (lo * (1.0 - WIDEN)).min(lo - SLACK);
    let high = (hi * (1.0 + WIDEN)).max(hi + SLACK);
    (low..=high).contains(&n)
}

/// Every walk frame of `path` judged, one line each; how many miss.
pub fn check(path: &Path, frames: &[Frame]) -> Result<usize, String> {
    let mut missed = 0;
    for (k, segment) in path.segments.iter().enumerate() {
        let Kind::Walk { from, to, age } = &segment.kind else { continue };
        let end = |id: &str| -> Result<Family, String> {
            let mut f = by_identity(id).map_err(|e| format!("{id}: {e}"))?;
            (f.age, f.skeleton.seed) = (*age, path.seed);
            Ok(f)
        };
        let (a, b) = (end(from)?, end(to)?);
        let started = std::time::Instant::now();
        let (mut lo, mut hi) = ([f64::INFINITY; 2], [0.0f64; 2]);
        let mut at_seed = Vec::new();
        for f in [&a, &b] {
            for seed in SEEDS {
                let n = counts(f, seed)?;
                if seed == u64::from(path.seed) {
                    at_seed.push((species(f)?.clone(), n));
                }
                for j in 0..2 {
                    (lo[j], hi[j]) = (lo[j].min(n[j]), hi[j].max(n[j]));
                }
            }
        }
        println!("walk {k} {from} -> {to} at {age}: band kept {lo0:.0}-{hi0:.0}, grown {lo1:.0}-{hi1:.0} ({:.0} s)", started.elapsed().as_secs_f64(), lo0 = lo[0], hi0 = hi[0], lo1 = lo[1], hi1 = hi[1]);
        let mut last = None;
        for frame in frames.iter().filter(|f| f.segment == k && !f.held) {
            let t = ease(frame.along);
            if last == Some(t) {
                continue;
            }
            last = Some(t);
            let walked = blend::families(&a, &b, t).map_err(|e| format!("t {t}: {e}"))?;
            let started = std::time::Instant::now();
            let same = at_seed.iter().find(|(s, _)| Some(s) == walked.space.species.as_ref());
            let (n, how) = match same {
                Some((_, n)) => (*n, "an end's species"),
                None => (counts(&walked, u64::from(path.seed))?, "grown"),
            };
            let ok = [0, 1].map(|j| within(n[j], lo[j], hi[j]));
            missed += usize::from(!(ok[0] && ok[1]));
            let mark = |j: usize, yes: &'static str, no: &'static str| if ok[j] { yes } else { no };
            println!(
                "walk {k} t {t:.4}: kept {:.0} grown {:.0} {} {} ({how}, {:.1} s)",
                n[0],
                n[1],
                mark(0, "k", "K!"),
                mark(1, "g", "G!"),
                started.elapsed().as_secs_f64()
            );
        }
    }
    Ok(missed)
}
