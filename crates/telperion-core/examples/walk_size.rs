//! A walk's size against its ends' (fn-251 R1, R2): for each point `t` of
//! the walk from one preset to another at one age (or an age walk's two,
//! `5:80`) and seed, the species the walk gives (`blend::families`), grown
//! by the build's one cycle at the frame's light, with its realized kept
//! and grown phytomers, beside the ends grown at the same seed, light and
//! age. Also the light-neutral predicted counts and the size factor the
//! walk solved. One line a point:
//!
//!   from to age seed t | light | factor | sized: predicted kept grown,
//!   realized kept grown, seconds | ends: the same at a, at b | oracle:
//!   whether each realized count lies within the ends' widened 10 %
//!
//!   cargo run --profile ci -p telperion-core --example walk_size -- \
//!     <from> <to> <age> <seed> <t,t,...>
use std::time::Instant;
use telperion_core::{blend, presets::Preset, Family};
use telperion_space::{expected_grown, expected_kept, grow_cycle, Request, Species};

/// Twice the engine's budget: a point past it is reported, not grown on.
const CAP: u32 = 40_000_000;
/// The oracle's widening of the ends' range (fn-251 R1b).
const WIDEN: f64 = 0.1;

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
    for t in ts.split(',') {
        let t: f64 = t.parse().map_err(|_| "a share")?;
        let walked = blend::families(&a, &b, t).map_err(|e| e.to_string())?;
        let bare = telperion_space::blend(species(&a), species(&b), t).map_err(|e| e.to_string())?;
        let light = walked.space.light;
        // The ends are grown at the frame's own age and light.
        let request = Request {
            age: walked.age as u32,
            seed: u64::from(seed),
            budget: CAP,
            light,
        };
        let ends = [measured(species(&a), request), measured(species(&b), request)];
        let sized = measured(species(&walked), request);
        let within = |n: Option<f64>, k: usize| match (n, ends[0].0, ends[1].0) {
            (Some(n), Some(x), Some(y)) => {
                let (x, y) = (x[k], y[k]);
                n >= x.min(y) * (1.0 - WIDEN) && n <= x.max(y) * (1.0 + WIDEN)
            }
            _ => false,
        };
        let verdict = |r: Option<[f64; 2]>| {
            format!(
                "{}{}",
                if within(r.map(|r| r[0]), 0) { "k" } else { "K!" },
                if within(r.map(|r| r[1]), 1) { "g" } else { "G!" }
            )
        };
        println!(
            "{from} {to} {} {seed} {t} | light {:.4} | factor {:.6} | sized {} | ends {} / {} | oracle {}",
            walked.age,
            light.extinction,
            factor(species(&walked), &bare),
            sized.1.join(" "),
            ends[0].1.join(" "),
            ends[1].1.join(" "),
            verdict(sized.0),
        );
    }
    Ok(())
}

fn species(family: &Family) -> &Species {
    family.space.species.as_ref().expect("an engine preset")
}

/// The light-neutral predicted counts, and the realized ones as grown.
fn measured(species: &Species, request: Request) -> (Option<[f64; 2]>, [String; 2]) {
    let n = neutral(species);
    let predicted = format!(
        "{:.0} {:.0}",
        expected_kept(&n, request.age).unwrap().unwrap_or(f64::NAN),
        expected_grown(&n, request.age).unwrap()
    );
    let (realized, seconds) = grown(species, request);
    (realized, [predicted, format!("{} {seconds:.1}s", shown(realized))])
}

fn shown(r: Option<[f64; 2]>) -> String {
    r.map_or("over-cap over-cap".into(), |[k, g]| format!("{k:.0} {g:.0}"))
}

/// The realized kept and grown phytomers by the build's one cycle, none
/// past the cap, and the seconds it took.
fn grown(species: &Species, request: Request) -> (Option<[f64; 2]>, f64) {
    let start = Instant::now();
    let out = grow_cycle(species, request).ok();
    let counts = out.map(|c| {
        let kept: usize = c.structure.axes.iter().map(|a| a.phytomers.len()).sum();
        [kept as f64, c.grown as f64]
    });
    (counts, start.elapsed().as_secs_f64())
}

fn neutral(species: &Species) -> Species {
    let mut neutral = species.clone();
    for s in &mut neutral.states {
        (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
    }
    neutral
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
