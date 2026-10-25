//! fn-256 task 3: each shipped preset's resident build, from its family
//! to draw-ready, timed warm (its second build in the process) at its
//! default age and seed: the same call before and after the one cycle
//! took the harness. `BUILD_PRESETS` names a comma list to time alone,
//! `BUILD_AGE` an age and `BUILD_SEED` a seed for them all; each build's peak owned bytes are
//! printed by label (fn-256 task 5's census).
//!
//! fn-306 R3: `BUILD_FRAME=<from>,<to>,<t>` times one walk frame, built as
//! the film-walk tests build theirs (each end at the run's age and seed,
//! blended at `t`), beside the presets `BUILD_PRESETS` names, or alone.
//! A frame needs `BUILD_AGE` and `BUILD_SEED`, and a bad value fails the
//! run. The arms run interleaved, every arm's cold build and then every
//! arm's warm one, so their timings share the desk's load.
//! `cargo test --profile ci -p telperion-render build_time -- --ignored
//! --nocapture`.
use super::cpu_builds_tests::at;
use super::Generator;
use crate::Renderer;
use std::str::FromStr;
use telperion_core::presets::{by_identity, Family};

/// One timed family: its label, family and what its builds measured.
struct Arm {
    label: String,
    family: Family,
    ms: Vec<f64>,
    count: usize,
    peak: u64,
    refused: Option<String>,
}

#[test]
#[ignore = "a measurement"]
fn build_time() {
    let mut arms: Vec<Arm> = arms().into_iter().map(Arm::new).collect();
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    for _ in 0..2 {
        for arm in arms.iter_mut().filter(|a| a.refused.is_none()) {
            arm.build(&gen, &gpu);
        }
    }
    println!("| Preset | Age | Cold ms | Warm ms | Instances | Device peak MB |");
    println!("|---|--:|--:|--:|--:|--:|");
    for arm in &arms {
        arm.row();
    }
}

impl Arm {
    fn new((label, family): (String, Family)) -> Self {
        let ms = Vec::new();
        Arm {
            label,
            family,
            ms,
            count: 0,
            peak: 0,
            refused: None,
        }
    }

    /// One build, timed, its census printed; a refusal ends the arm.
    fn build(&mut self, gen: &Generator, gpu: &crate::Gpu) {
        gpu.ledger.take_peak_census();
        let t = super::Clock::now();
        let prepared = match gen.prepare(&self.family) {
            Ok(p) => p,
            Err(e) => {
                self.refused = Some(e.to_string());
                census(&self.label, gpu.ledger.take_peak_census());
                return;
            }
        };
        self.ms.push(t.elapsed_ms());
        self.count = prepared.count();
        let labelled = gpu.ledger.take_peak_census();
        self.peak = labelled.0;
        census(&self.label, labelled);
        drop(prepared);
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
    }

    fn row(&self) {
        let (id, age) = (&self.label, self.family.age);
        match (&self.refused, self.ms.as_slice()) {
            (Some(e), _) => println!("| {id} | {age} | refused: {e} | | |"),
            (None, [cold, warm]) => println!(
                "| {id} | {age} | {cold:.0} | {warm:.0} | {} | {:.0} |",
                self.count,
                self.peak as f64 / 1e6
            ),
            _ => {}
        }
    }
}

/// The run's families, labelled: the frame `BUILD_FRAME` names, then the
/// presets `BUILD_PRESETS` names (every preset when neither is set).
fn arms() -> Vec<(String, Family)> {
    let frame = std::env::var("BUILD_FRAME").ok();
    let (age, seed): (Option<f64>, Option<u32>) = match &frame {
        Some(_) => (Some(required("BUILD_AGE")), Some(required("BUILD_SEED"))),
        None => (optional("BUILD_AGE"), optional("BUILD_SEED")),
    };
    let only = std::env::var("BUILD_PRESETS").ok();
    let mut out: Vec<(String, Family)> = frame
        .iter()
        .map(|f| walk_frame(f, age.unwrap(), seed.unwrap()))
        .collect();
    if frame.is_some() && only.is_none() {
        return out;
    }
    let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
    let ids = ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id)));
    for id in ids {
        let mut family = by_identity(id).unwrap();
        family.age = age.unwrap_or(family.age);
        family.skeleton.seed = seed.unwrap_or(family.skeleton.seed);
        out.push((id.to_string(), family));
    }
    out
}

/// The walk frame `spec` (`<from>,<to>,<t>`) at `age` and `seed`, named.
fn walk_frame(spec: &str, age: f64, seed: u32) -> (String, Family) {
    let fail = |why: String| -> ! { panic!("BUILD_FRAME {spec}: {why}") };
    let [from, to, t] = spec.split(',').collect::<Vec<_>>()[..] else {
        fail("not <from>,<to>,<t>".into());
    };
    for id in [from, to] {
        by_identity(id).unwrap_or_else(|e| fail(format!("{id}: {e}")));
    }
    let t: f64 = t.parse().unwrap_or_else(|_| fail(format!("no t {t}")));
    let (x, y) = (at(from, age, seed), at(to, age, seed));
    let family = telperion_core::blend::families(&x, &y, t).unwrap_or_else(|e| fail(e.to_string()));
    (format!("{from} to {to} t {t}"), family)
}

/// The variable `name` as a number, or none when it is unset or unparsed
/// (the preset rows' options, as fn-256 read them).
fn optional<T: FromStr>(name: &str) -> Option<T> {
    std::env::var(name).ok().and_then(|v| v.parse().ok())
}

/// The variable `name` as a number, which a frame needs.
fn required<T: FromStr>(name: &str) -> T {
    let v = std::env::var(name).unwrap_or_else(|_| panic!("BUILD_FRAME needs {name}"));
    v.parse()
        .unwrap_or_else(|_| panic!("{name} {v}: not a number"))
}

/// A build's peak owned bytes by label, the largest first.
fn census(id: &str, (peak, by): (u64, Vec<(&'static str, u64)>)) {
    let mb = |b: u64| b as f64 / 1e6;
    let rows: Vec<String> = by
        .iter()
        .take(16)
        .map(|(l, b)| format!("{l} {:.0}", mb(*b)))
        .collect();
    println!("census {id}: peak {:.0} MB: {}", mb(peak), rows.join(", "));
}
