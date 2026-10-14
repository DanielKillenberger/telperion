//! A walk's size against its ends' (fn-251 R1, R2, R3): for each point `t`
//! of the walk from one preset to another at one age (or an age walk's two,
//! `5:80`), the species the walk gives (`blend::families`), grown by the
//! build's one cycle at the frame's seed and light, with its realized kept
//! and grown phytomers, beside the oracle's band: the smallest and largest
//! of both ends' counts over seeds 1, 7 and 42 at the frame's light and
//! age, widened by 10 % or two phytomers, whichever is wider (host,
//! 2026-10-14). An end is grown once for each age and light a walk asks.
//! Also the frame's light-neutral predicted counts and the size factor the
//! walk solved. One line a point:
//!
//!   from to age seed t | light | factor | frame: predicted kept grown,
//!   realized kept grown, seconds | band: kept lo hi, grown lo hi |
//!   oracle: k or K! for kept, g or G! for grown
//!
//!   cargo run --profile ci -p telperion-core --example walk_size -- \
//!     <from> <to> <age> <seed> <t,t,...>
use std::time::Instant;
use telperion_core::{blend, presets::Preset, Family};
use telperion_space::{
    expected_grown, expected_kept, grow_cycle, Light, Request, Species,
};

/// Twice the engine's budget: a point past it is reported, not grown on.
const CAP: u32 = 40_000_000;
/// The oracle's widening of the ends' range (fn-251 R1b).
const WIDEN: f64 = 0.1;
/// Its slack in whole phytomers, for seedlings (host, 2026-10-14).
const SLACK: f64 = 2.0;
/// The seeds whose ends make the oracle's reference (host, 2026-10-14).
const SEEDS: [u64; 3] = [1, 7, 42];

/// Kept and grown phytomers.
type Counts = [f64; 2];

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [from, to, age, seed, ts] = &args[..] else {
        return Err("from to age seed t,t,...".into());
    };
    let preset = |id: &str| Preset::from_id(id).ok_or(format!("unknown tree {id}"));
    // One age, or an age walk's two: `5:80`.
    let years = |s: &str| s.parse::<f64>().map_err(|_| "an age");
    let (from_age, to_age) = match age.split_once(':') {
        Some((x, y)) => (years(x)?, years(y)?),
        None => (years(age)?, years(age)?),
    };
    let seed: u32 = seed.parse().map_err(|_| "a seed")?;
    let end = |p: Preset, age: f64| {
        let mut f = p.parameters();
        (f.age, f.skeleton.seed) = (age, seed);
        f
    };
    let (a, b) = (end(preset(from)?, from_age), end(preset(to)?, to_age));
    let mut bands: Vec<((u32, Light), [Counts; 2])> = Vec::new();
    for t in ts.split(',') {
        let t: f64 = t.parse().map_err(|_| "a share")?;
        let walked = blend::families(&a, &b, t).map_err(|e| e.to_string())?;
        let bare = telperion_space::blend(species(&a), species(&b), t)
            .map_err(|e| e.to_string())?;
        let (age, light) = (walked.age as u32, walked.space.light);
        let band = match bands.iter().find(|(key, _)| *key == (age, light)) {
            Some((_, band)) => *band,
            None => {
                let band = band(&[species(&a), species(&b)], age, light);
                bands.push(((age, light), band));
                band
            }
        };
        let request = Request {
            age,
            seed: u64::from(seed),
            budget: CAP,
            light,
        };
        let frame = species(&walked);
        let start = Instant::now();
        let realized = grown(frame, request);
        let seconds = start.elapsed().as_secs_f64();
        let mark = |k: usize, ok: &str, bad: &str| match realized {
            Some(r) if within(r[k], band[0][k], band[1][k]) => ok.to_owned(),
            _ => bad.to_owned(),
        };
        println!(
            "{from} {to} {age} {seed} {t} | light {:.4} | factor {:.6} | frame {} {} {seconds:.1}s | band {:.0} {:.0} {:.0} {:.0} | oracle {}{}",
            light.extinction,
            factor(frame, &bare),
            predicted(frame, age),
            realized.map_or("over-cap over-cap".into(), |[k, g]| format!("{k:.0} {g:.0}")),
            band[0][0],
            band[1][0],
            band[0][1],
            band[1][1],
            mark(0, "k", "K!"),
            mark(1, "g", "G!"),
        );
    }
    Ok(())
}

fn species(family: &Family) -> &Species {
    family.space.species.as_ref().expect("an engine preset")
}

/// The smallest and largest of `ends`' realized counts over `SEEDS` at
/// `age` and `light`; an end past the cap counts at the cap.
fn band(ends: &[&Species], age: u32, light: Light) -> [Counts; 2] {
    let (mut lo, mut hi) = ([f64::INFINITY; 2], [0.0f64; 2]);
    for (end, seed) in ends.iter().flat_map(|e| SEEDS.map(|s| (*e, s))) {
        let request = Request {
            age,
            seed,
            budget: CAP,
            light,
        };
        let n = grown(end, request).unwrap_or([f64::from(CAP); 2]);
        for k in 0..2 {
            (lo[k], hi[k]) = (lo[k].min(n[k]), hi[k].max(n[k]));
        }
    }
    [lo, hi]
}

/// Whether `n` lies within `lo` to `hi` widened by `WIDEN`, or by `SLACK`
/// phytomers where that is wider.
fn within(n: f64, lo: f64, hi: f64) -> bool {
    let low = (lo * (1.0 - WIDEN)).min(lo - SLACK);
    let high = (hi * (1.0 + WIDEN)).max(hi + SLACK);
    (low..=high).contains(&n)
}

/// The light-neutral predicted kept and grown counts.
fn predicted(species: &Species, age: u32) -> String {
    let mut n = species.clone();
    for s in &mut n.states {
        (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
    }
    let kept = expected_kept(&n, age).unwrap().unwrap_or(f64::NAN);
    format!("{kept:.0} {:.0}", expected_grown(&n, age).unwrap())
}

/// The realized kept and grown phytomers by the build's one cycle, none
/// past the cap.
fn grown(species: &Species, request: Request) -> Option<Counts> {
    let cycled = grow_cycle(species, request).ok()?;
    let kept: usize = cycled.structure.axes.iter().map(|a| a.phytomers.len()).sum();
    Some([kept as f64, cycled.grown as f64])
}

/// The lateral factor the walk solved: its laterals over the bare walk's.
fn factor(sized: &Species, bare: &Species) -> f64 {
    let pairs = sized.states.iter().zip(&bare.states).flat_map(|(s, m)| {
        s.zones
            .iter()
            .zip(&m.zones)
            .flat_map(|(p, q)| p.lateral.iter().zip(&q.lateral))
    });
    pairs
        .filter(|(_, q)| **q > 0.0)
        .map(|(p, q)| p / q)
        .next()
        .unwrap_or(1.0)
}
