//! The device's light held to the CPU's `read_light`: young trees in
//! seconds, every word of the box, quanta, densities and lattice exact,
//! the f32 reads compared apart; and the busy cycles, measured.
use super::columns::Cap;
use super::light_device::{Kernels, Lit, Out, Run};
use crate::Gpu;
use telperion_core::presets::Preset;
use telperion_space::{Capture, Request};

/// `preset` grown to `age` at `seed` through the one cycle and its `at`th
/// cycle's light captured; light as the preset sets it.
fn captured(preset: Preset, age: u32, seed: u64, at: u32) -> Capture {
    let family = preset.parameters();
    let species = family.space.species.expect("an engine species");
    let request = Request {
        age,
        seed,
        budget: 20_000_000,
        light: family.space.light,
    };
    telperion_space::grow_capture_light_at(&species, request, at).expect("a capture")
}

/// The words of each output that differ from the CPU's, by name: how
/// many of how many, the first, and for reals the largest difference.
struct Agreement(Vec<(&'static str, usize, usize, Option<usize>, f32)>);

impl Agreement {
    fn of(cap: &Cap, out: &Out) -> Self {
        let words = |n: &str| cap.u32s(n);
        let pairs: [(&'static str, Vec<u32>, Vec<u32>); 6] = [
            ("errors", vec![0], vec![out.errors]),
            (
                "box",
                [words("cpu_grid_low"), words("cpu_grid_size")].concat(),
                [out.low, out.size].concat(),
            ),
            ("quanta", words("cpu_quanta"), out.quanta.clone()),
            ("densities", words("cpu_density"), out.density.clone()),
            ("lattice", words("cpu_lattice"), out.lattice.clone()),
            ("reads", words("cpu_reads"), out.reads.clone()),
        ];
        Self(
            pairs
                .into_iter()
                .map(|(name, cpu, dev)| {
                    let first = (0..cpu.len().max(dev.len())).find(|&i| cpu.get(i) != dev.get(i));
                    let n = (0..cpu.len().max(dev.len()))
                        .filter(|&i| cpu.get(i) != dev.get(i))
                        .count();
                    let real = |w: Option<&u32>| w.map_or(f32::NAN, |w| f32::from_bits(*w));
                    let most = (0..cpu.len().max(dev.len()))
                        .map(|i| (real(cpu.get(i)) - real(dev.get(i))).abs())
                        .fold(0.0, f32::max);
                    (name, n, cpu.len(), first, most)
                })
                .collect(),
        )
    }

    /// Whether every word of the named outputs agrees.
    fn exact(&self, names: &[&str]) -> bool {
        self.0
            .iter()
            .filter(|r| names.contains(&r.0))
            .all(|r| r.1 == 0)
    }

    /// Whether the named real output, a light of at most one, is within
    /// the ruled tolerance of the CPU's (`ruled.rs`).
    fn within(&self, name: &str) -> bool {
        self.0.iter().filter(|r| r.0 == name).all(|r| f64::from(r.4) <= super::ruled::FLOAT_TOLERANCE)
    }

    fn report(&self) -> String {
        let rows: Vec<String> = (self.0.iter())
            .map(|(name, n, of, first, most)| match first {
                None => format!("  {name}: exact ({of} words)"),
                Some(i) => format!("  {name}: {n} of {of} words differ, the first at {i}; as f32s, the largest by {most:e}"),
            })
            .collect();
        rows.join("\n")
    }
}

const LATTICE: [&str; 5] = ["errors", "box", "quanta", "densities", "lattice"];

fn capacity(cap: &Cap) -> u32 {
    cap.u32s("cpu_grid_size").iter().product()
}

/// The captured light on the device, compared with the CPU's, the
/// comparison printed; swept under `extinction` where one is given.
fn replay(
    gpu: &Gpu,
    k: &Kernels,
    name: &str,
    c: &Capture,
    extinction: Option<f64>,
) -> (Run, Agreement) {
    let cap = Cap(c);
    let lit = Lit::new(&cap, capacity(&cap), extinction);
    let run = Run::new(gpu, k, &lit).expect("buffers");
    let mut e = gpu.encoder(&Default::default());
    run.record(k, &mut e);
    gpu.submit([e.finish()]);
    let out = run.read(gpu).expect("a readback");
    let agreement = Agreement::of(&cap, &out);
    let [leaves, tips, dirs] = lit.counts();
    let size = cap.u32s("cpu_grid_size");
    println!(
        "{name}: cycle {}, {leaves} leaves, {tips} tips, {dirs} directions, lit {}, lattice {size:?} = {} nodes, {} holding leaf area; refused: {:?}",
        cap.u32s("light_cycle")[0], lit.lit, capacity(&cap), out.quanta.iter().filter(|&&q| q > 0).count(), out.refusals()
    );
    println!("{}", agreement.report());
    (run, agreement)
}

fn device() -> Option<(Gpu, Kernels)> {
    let gpu = super::super::test_gpu()?;
    let started = std::time::Instant::now();
    let k = Kernels::new(&gpu);
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("compiled");
    println!(
        "light kernels compiled in {:.2} s",
        started.elapsed().as_secs_f64()
    );
    Some((gpu, k))
}

/// The young trees, every one under the site's light (fn-264 R1a): the
/// oak's, the ordinary's and Telperion's leaves, the beech's on him.
const YOUNG: [(Preset, &str, u32, u32); 4] = [
    (Preset::OregonWhiteOak, "oregon-white-oak", 16, 12),
    (Preset::OregonWhiteOak, "oregon-white-oak", 8, 5),
    (Preset::Ordinary, "ordinary", 16, 12),
    (Preset::Telperion, "telperion", 14, 10),
];

/// Every young capture's agreement by `holds`; the captures that differ.
fn young_differ(holds: &dyn Fn(&Agreement) -> bool) -> Vec<String> {
    let Some((gpu, k)) = device() else {
        return Vec::new();
    };
    let mut differ = Vec::new();
    for (preset, name, age, at) in YOUNG {
        let name = format!("{name} {age} at {at}");
        let (_, agreement) = replay(&gpu, &k, &name, &captured(preset, age, 42, at), None);
        if !holds(&agreement) {
            differ.push(name);
        }
    }
    differ
}

#[test]
fn young_lattices_agree_with_the_cpu() {
    let _held = super::super::whole_tree();
    let differ = young_differ(&|a| a.exact(&LATTICE));
    assert!(
        differ.is_empty(),
        "the device's lattice is not the CPU's: {differ:?}"
    );
}

/// The reads within the ruled tolerance: the driver fuses the read's
/// product into its sum (`light_sweep.wgsl` `read_main`, LIGHT.md).
#[test]
fn young_reads_agree_with_the_cpu() {
    let _held = super::super::whole_tree();
    let differ = young_differ(&|a| a.within("reads"));
    assert!(
        differ.is_empty(),
        "the device's reads are not the CPU's: {differ:?}"
    );
}

/// The refusals only the deposit raises: a leaf's area, a node's quanta
/// and a leaf outside its box (`light.wgsl`'s `E_AREA`, `E_DEPOSIT`,
/// `E_OUTSIDE`).
const DEPOSIT: u32 = 8 | 16 | 32;

/// `lit` on the device, swept or dormant: its reads and refusals.
fn reads(gpu: &Gpu, k: &Kernels, lit: &Lit) -> (Vec<u32>, u32) {
    let run = Run::new(gpu, k, lit).expect("buffers");
    let mut e = gpu.encoder(&Default::default());
    run.record(k, &mut e);
    gpu.submit([e.finish()]);
    let out = run.read(gpu).expect("a readback");
    (out.reads, out.errors)
}

/// fn-256 task 9: where nothing shades, the lattice's dispatches are not
/// recorded and each tip sums its eight rounded weights against a light
/// of one, word for word the swept lattice's read: on Telperion and the
/// oak with their leaf area taken away, under the site's light, whose open
/// sky is one (every preset grows lit since fn-264 R1a).
/// Every refusal but the deposit's is the swept lattice's; the deposit,
/// which guards only the lattice's input, is not recorded (task 12).
#[test]
fn a_dormant_lattice_reads_as_the_swept_one() {
    let Some((gpu, k)) = device() else {
        return;
    };
    let mut differ = Vec::new();
    for (preset, name, age, at) in [(Preset::Telperion, "telperion", 14, 10), (Preset::OregonWhiteOak, "oregon-white-oak", 8, 5)] {
        let c = captured(preset, age, 42, at);
        let cap = Cap(&c);
        let mut lit = Lit::new(&cap, capacity(&cap), None);
        assert!(lit.lit && super::build_light::open(&cap.u64s("light_directions")), "{name}: a lit site under an open sky of one");
        lit.leaves.iter_mut().skip(3).step_by(4).for_each(|area| *area = 0.0);
        let swept = reads(&gpu, &k, &lit);
        lit.dormant = true;
        let dormant = reads(&gpu, &k, &lit);
        let n = swept.0.iter().zip(&dormant.0).filter(|(a, b)| a != b).count();
        println!("{name} {age} at {at}: {} tips, {n} reads differ, refusals {:x} and {:x}", swept.0.len(), swept.1, dormant.1);
        if n > 0 || swept.0.len() != dormant.0.len() || swept.1 & !DEPOSIT != dormant.1 {
            differ.push(name);
        }
    }
    assert!(differ.is_empty(), "a dormant lattice reads apart from the swept one: {differ:?}");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// A phase's median over the runs: its ms in all, its dispatches, and
/// its longest dispatch's ms.
struct Phase {
    name: &'static str,
    total: f64,
    dispatches: usize,
    longest: f64,
}

/// One run's passes by phase, in order: each phase's sum and longest
/// pass, then the first stamp to the last.
fn by_phase(
    phases: &[&'static str],
    stamps: &[(String, f64, f64)],
) -> Vec<(&'static str, f64, usize, f64)> {
    let mut row: Vec<(&'static str, f64, usize, f64)> = Vec::new();
    for (p, s) in phases.iter().zip(stamps) {
        let ms = (s.2 - s.1) * 1e-6;
        match row.iter_mut().find(|r| r.0 == *p) {
            Some(r) => (r.1, r.2, r.3) = (r.1 + ms, r.2 + 1, r.3.max(ms)),
            None => row.push((p, ms, 1, ms)),
        }
    }
    let all = (stamps[stamps.len() - 1].2 - stamps[0].1) * 1e-6;
    row.push(("first to last", all, phases.len(), all));
    row
}

/// Nine runs of the light, each pass stamped, after a second of untimed
/// runs so the device runs at its working clocks: each phase's medians.
fn timed(gpu: &Gpu, k: &Kernels, run: &Run) -> Vec<Phase> {
    let warm = std::time::Instant::now();
    while warm.elapsed().as_secs_f64() < 1.0 {
        let mut e = gpu.encoder(&Default::default());
        run.record(k, &mut e);
        gpu.submit([e.finish()]);
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("a run");
    }
    let mut rows = Vec::new();
    for _ in 0..9 {
        assert!(super::super::profile::start(gpu), "timestamps");
        let mut e = gpu.encoder(&Default::default());
        let phases = run.record(k, &mut e);
        gpu.submit([e.finish()]);
        let stamps = super::super::profile::finish(gpu);
        assert_eq!(stamps.len(), phases.len(), "a stamp a pass");
        rows.push(by_phase(&phases, &stamps));
    }
    (0..rows[0].len())
        .map(|i| Phase {
            name: rows[0][i].0,
            total: median(rows.iter().map(|r| r[i].1).collect()),
            dispatches: rows[0][i].2,
            longest: median(rows.iter().map(|r| r[i].3).collect()),
        })
        .collect()
}

#[test]
#[ignore = "a measurement"]
fn busy_lights_agree_and_are_timed() {
    let _held = super::super::whole_tree();
    let Some((gpu, k)) = device() else {
        return;
    };
    let mut differ = Vec::new();
    for (preset, name, age, seed) in [
        (Preset::OregonWhiteOak, "oregon-white-oak", 80, 1),
        (Preset::Telperion, "telperion", 82, 42),
    ] {
        let started = std::time::Instant::now();
        let capture = captured(preset, age, seed, age - 1);
        println!(
            "{name} {age}: captured in {:.1} s",
            started.elapsed().as_secs_f64()
        );
        let unlit = Cap(&capture).u64s("light_extinction")[0] == 0;
        let mut runs = vec![(format!("{name} {age} at {}", age - 1), None)];
        if unlit {
            // Swept as if lit, for what a lit site would pay; not compared.
            runs.push((
                format!("{name} {age} at {}, swept at extinction 0.5", age - 1),
                Some(0.5),
            ));
        }
        for (label, extinction) in runs {
            let (run, agreement) = replay(&gpu, &k, &label, &capture, extinction);
            if extinction.is_none() && !(agreement.exact(&LATTICE) && agreement.within("reads")) {
                differ.push(label.clone());
            }
            let times = timed(&gpu, &k, &run);
            let passes = &times[..times.len() - 1];
            let longest = passes
                .iter()
                .max_by(|a, b| a.longest.total_cmp(&b.longest))
                .expect("a pass");
            println!(
                "{label}: {} dispatches, longest {} {:.3} ms; {:.2} MB resident, {:.2} MB scratch",
                times[times.len() - 1].dispatches,
                longest.name,
                longest.longest,
                run.resident as f64 / 1e6,
                run.scratch as f64 / 1e6
            );
            for p in times {
                println!(
                    "{label}: {:>14} {:8.3} ms in {} dispatches, the longest {:.3} ms",
                    p.name, p.total, p.dispatches, p.longest
                );
            }
        }
    }
    assert!(
        differ.is_empty(),
        "the device's light is not the CPU's: {differ:?}"
    );
}
