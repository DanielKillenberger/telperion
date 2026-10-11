//! The device's rough layout held to the CPU's: young trees in seconds,
//! and the busy cycles DESIGN.md section 7 names, measured.
use super::columns::Cap;
use super::compare::Agreement;
use super::lay_columns::Laid;
use super::lay_compare::{compare, laying, Laying};
use super::lay_device::{Kernels, Run};
use crate::Gpu;
use telperion_core::presets::Preset;
use telperion_space::{Capture, Request};

/// `preset` grown to `age` at `seed` through the one cycle and its `at`th
/// cycle's layout captured; light as the preset sets it.
fn captured(preset: Preset, age: u32, seed: u64, at: u32) -> Capture {
    let family = preset.parameters();
    let species = family.space.species.expect("an engine species");
    let request = Request {
        age,
        seed,
        budget: 20_000_000,
        light: family.space.light,
    };
    telperion_space::grow_capture_layout_at(&species, request, at).expect("a capture")
}

/// The new axes by origin: laterals (woken), continuations, relays
/// (carried off their line, blend above none).
fn kinds(laid: &Laid) -> String {
    let n = laid.axes as usize;
    let col = |k: usize| &laid.in_axes[k * n..(k + 1) * n];
    let (origin, blend) = (col(2), col(8));
    let mut count = [0usize; 6];
    for i in laid.first_new as usize..n {
        let kind = (origin[i] & 0xff) as usize;
        count[kind] += 1;
        count[4] += usize::from(kind == 1 && origin[i] & 0x10000 != 0);
        count[5] += usize::from(kind == 3 && f32::from_bits(blend[i]) > 0.0);
    }
    format!(
        "{} laterals ({} woken), {} continuations, {} relays ({} off their line)",
        count[1], count[4], count[2], count[3], count[5]
    )
}

/// The captured layout on the device, compared with the CPU's, the
/// comparison printed.
fn replay(gpu: &Gpu, k: &Kernels, name: &str, c: &Capture) -> (Run, Laying, Agreement) {
    let cap = Cap(c);
    let laid = Laid::new(&cap);
    let run = Run::new(gpu, k, &laid).expect("buffers");
    let mut e = gpu.encoder(&Default::default());
    run.record(gpu, k, &mut e);
    gpu.submit([e.finish()]);
    let out = run.read(gpu).expect("a readback");
    let agreement = compare(&cap, &laid, &out);
    let l = laying(&cap, &laid);
    println!(
        "{name}: cycle {}, {} axes; {} grown and {} new axes drawn ({}), {} phytomers laid, {} generations",
        laid.cycle, laid.axes, laid.grown(), l.framed, kinds(&laid), l.laid, l.generations
    );
    println!("{}", agreement.report());
    (run, l, agreement)
}

fn device() -> Option<(Gpu, Kernels)> {
    let gpu = super::super::test_gpu()?;
    let started = std::time::Instant::now();
    let k = Kernels::new(&gpu);
    println!("layout kernels compiled in {:.1} s", started.elapsed().as_secs_f64());
    Some((gpu, k))
}

#[test]
#[ignore = "red on the driver's division, contraction and reordering until the host rules (LAYOUT.md)"]
fn young_layouts_agree_with_the_cpu() {
    let _held = super::super::whole_tree();
    let Some((gpu, k)) = device() else {
        return;
    };
    let mut differ = Vec::new();
    for (preset, name, age, at) in [
        (Preset::EuropeanBeech, "european-beech", 10, 8),
        (Preset::Telperion, "telperion", 14, 10),
        (Preset::Ordinary, "ordinary", 16, 12),
        (Preset::NorwaySpruce, "norway-spruce", 24, 16),
        (Preset::OregonWhiteOak, "oregon-white-oak", 16, 12),
        (Preset::SilverBirch, "silver-birch", 16, 12),
        (Preset::DatePalm, "date-palm", 16, 12),
    ] {
        let name = format!("{name} {age} at {at}");
        if !replay(&gpu, &k, &name, &captured(preset, age, 42, at)).2.exact() {
            differ.push(name);
        }
    }
    assert!(differ.is_empty(), "the device's layout is not the CPU's: {differ:?}");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// Nine runs of the layout, each pass stamped: each phase's median ms,
/// in the order the phases first run, and the passes a run takes.
fn timed(gpu: &Gpu, k: &Kernels, run: &Run) -> (Vec<(&'static str, f64)>, usize) {
    let mut each: Vec<(&'static str, Vec<f64>)> = Vec::new();
    let mut passes = 0;
    for _ in 0..9 {
        assert!(super::super::profile::start(gpu), "timestamps");
        let mut e = gpu.encoder(&Default::default());
        let phases = run.record(gpu, k, &mut e);
        gpu.submit([e.finish()]);
        let stamps = super::super::profile::finish(gpu);
        assert_eq!(stamps.len(), phases.len(), "a stamp a pass");
        passes = phases.len();
        let mut sums: Vec<(&'static str, f64)> = Vec::new();
        for (phase, s) in phases.iter().zip(&stamps) {
            match sums.iter_mut().find(|p| p.0 == *phase) {
                Some(p) => p.1 += (s.2 - s.1) * 1e-6,
                None => sums.push((phase, (s.2 - s.1) * 1e-6)),
            }
        }
        sums.push(("first to last", (stamps[passes - 1].2 - stamps[0].1) * 1e-6));
        for (phase, ms) in sums {
            match each.iter_mut().find(|p| p.0 == phase) {
                Some(p) => p.1.push(ms),
                None => each.push((phase, vec![ms])),
            }
        }
    }
    (each.into_iter().map(|(p, v)| (p, median(v))).collect(), passes)
}

#[test]
#[ignore = "a measurement"]
fn busy_layouts_agree_and_are_timed() {
    let _held = super::super::whole_tree();
    let Some((gpu, k)) = device() else {
        return;
    };
    let mut differ = Vec::new();
    for (preset, name, age) in [(Preset::Telperion, "telperion", 82), (Preset::EuropeanBeech, "european-beech", 40)] {
        let started = std::time::Instant::now();
        let capture = captured(preset, age, 42, age - 1);
        println!("{name} {age}: captured in {:.1} s", started.elapsed().as_secs_f64());
        let (run, l, agreement) = replay(&gpu, &k, &format!("{name} {age} at {}", age - 1), &capture);
        if !agreement.exact() {
            differ.push(name);
        }
        drop(capture);
        let (times, passes) = timed(&gpu, &k, &run);
        println!(
            "{name} {age}: {passes} dispatches, {} generations, {} axes drawn, {} framed, {} phytomers laid; {:.1} MB resident, {:.2} MB scratch",
            l.generations, l.drawn, l.framed, l.laid, run.resident as f64 / 1e6, run.scratch as f64 / 1e6
        );
        for (phase, ms) in times {
            println!("{name} {age}: {phase:>14} {ms:8.3} ms");
        }
    }
    assert!(differ.is_empty(), "the device's layout is not the CPU's: {differ:?}");
}
