//! The contract's mirror against the CPU's (fn-254, after fn-209 F1 R1):
//! every function of `telperion_space::contract`, on the GPU and the CPU,
//! over a quarter of a million cases each, endpoints and sentinels among
//! them. The discrete contracts (keys, draws, integer conversions,
//! thresholds of the same f32) give the same bits (R4, check 2); the f32
//! arithmetic agrees within the units in the last place `Tolerance`
//! states, measured on the RTX 3080 (fn-254 R4, check 4).
use super::super::io;
use super::selectors::{Selector, Selector as S, ALL};
use crate::Gpu;
use std::time::Instant;
use telperion_space::contract::{self as c, decide, presence as pr};

const CASES: usize = 1 << 18;
const PROBE: &str = include_str!("probe.wgsl");

/// SplitMix64, for cases.
struct Cases(u64);

impl Cases {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    /// Uniform in [lo, hi], with its ends and their neighbours often.
    fn within(&mut self, lo: u64, hi: u64) -> u64 {
        match self.next() % 16 {
            0 => lo,
            1 => hi,
            2 => (lo + 1).min(hi),
            3 => (hi - 1).max(lo),
            _ => lo + self.next() % (hi - lo + 1),
        }
    }
    fn draw(&mut self) -> u64 {
        self.within(0, c::ONE - 1)
    }
    /// A real in [0, 1], its ends and tiny and near-whole ones often.
    fn unit(&mut self) -> f32 {
        let u = (self.next() >> 40) as f32 / (1u64 << 24) as f32;
        match self.next() % 8 {
            0 => 0.0,
            1 => 1.0,
            2 => u * 1e-6,
            3 => 1.0 - u * 1e-6,
            _ => u,
        }
    }
    /// A `-log2` chance: none, unbounded, or up to `most`.
    fn log(&mut self, most: f32) -> f32 {
        match self.next() % 8 {
            0 => c::INF,
            1 => 0.0,
            _ => self.unit() * most,
        }
    }
    fn real(&mut self, lo: f32, hi: f32) -> f32 {
        lo + self.unit() * (hi - lo)
    }
}

/// An f32 as a probe word.
fn w(x: f32) -> u64 {
    u64::from(x.to_bits())
}

/// Selector `s`'s cases, and the presences the relay cases read.
fn cases(s: Selector, rng: &mut Cases, presences: &mut Vec<f32>) -> Vec<[u64; 8]> {
    (0..CASES)
        .map(|_| {
            let mut a = [0u64; 8];
            match s {
                S::ChildDraw | S::Internode | S::Angles => {
                    a[..2].copy_from_slice(&[rng.next(), rng.next()])
                }
                S::Convert => {
                    a[0] = rng.next() >> (rng.next() % 64);
                    a[1] = w(rng.real(0.0, 1.0) * 2f32.powi((rng.next() % 62) as i32));
                }
                S::Threshold | S::Logs | S::Logit => {
                    a[0] = w(rng.unit());
                    a[1] = match s {
                        S::Logit => rng.draw(),
                        _ => w(rng.log(70.0)),
                    };
                }
                S::Survival | S::Abortion | S::Lived => {
                    a[0] = w(rng.log(8.0));
                    a[1] = rng.within(0, 1);
                    a[2] = w(rng.real(-4.0, 4.0));
                    a[3] = rng.within(0, 120 << 20);
                    a[4] = rng.within(0, 1 << 32);
                    a[5] = w(rng.log(8.0));
                    a[6] = rng.within(0, 1 << 20);
                    a[7] = rng.draw();
                    if s == S::Survival {
                        a[1] = a[4];
                    }
                }
                S::Moved => {
                    // The move's log-odds, certain one time in eight.
                    a[0] = w(if rng.next() % 8 == 0 {
                        c::INF
                    } else {
                        rng.real(-20.0, 20.0)
                    });
                    a[1] = rng.draw();
                    a[2] = w(if rng.next() % 8 == 0 {
                        -c::INF
                    } else {
                        rng.real(-30.0, 0.0)
                    });
                    a[3] = w(if rng.next() % 8 == 0 {
                        -c::INF
                    } else {
                        rng.real(-30.0, 0.0)
                    });
                    a[4] = rng.within(0, 1 << 20);
                    a[5] = w(rng.real(0.0, 6.0));
                    a[6] = w(rng.real(0.0, 0.05));
                    a[7] = w(rng.real(0.05, 2.0));
                }
                S::Past => {
                    // The bound's own side: a log or log-odds, none one time
                    // in eight.
                    a[0] = rng.draw();
                    a[1] = w(if rng.next() % 8 == 0 {
                        -c::INF
                    } else {
                        rng.real(-30.0, 10.0)
                    });
                }
                S::Wake => {
                    a[0] = rng.within(0, 40 << 20);
                    a[1] = w(rng.real(0.0, 100.0));
                    a[2] = rng.draw();
                }
                S::Lead => {
                    let (x, y) = (rng.draw(), rng.draw());
                    a[0] = if rng.next() % 4 == 0 { 0 } else { x.min(y) };
                    a[1] = if rng.next() % 4 == 0 {
                        c::ONE
                    } else {
                        x.max(y)
                    };
                    a[2] = rng.draw();
                }
                S::Window => {
                    a[0] = w(if rng.next() % 8 == 0 {
                        c::INF
                    } else {
                        rng.real(-4.0, 4.0)
                    });
                    a[1] = w(rng.real(0.0, 2.0));
                    a[2] = w(if rng.next() % 8 == 0 {
                        -c::INF
                    } else {
                        rng.real(-30.0, 0.0)
                    });
                    a[3] = w(if rng.next() % 8 == 0 {
                        -c::INF
                    } else {
                        rng.real(-30.0, 0.0)
                    });
                    a[4] = rng.within(0, 1 << 20);
                    a[5] = w(rng.real(0.0, 6.0));
                    a[6] = w(rng.real(0.0, 0.05));
                    a[7] = w(rng.real(0.05, 2.0));
                }
                S::Carry => {
                    let (birth, age) = (rng.within(0, 80), rng.within(0, 80));
                    a[0] = w(rng.real(0.0, 80.0));
                    a[1] = w(rng.unit());
                    a[2] = rng.within(0, 80) | age << 32;
                    a[3] = w(rng.real(0.0, 80.0));
                    a[4] = w(rng.real(0.0, 40.0));
                    a[5] = birth;
                    a[6] = w(rng.real(0.0, 80.0));
                    a[7] = w(rng.unit());
                }
                S::RelayNode => {
                    let len = rng.within(1, 40);
                    a[0] = presences.len() as u64;
                    a[1] = len;
                    a[2] = w(rng.unit());
                    for _ in 0..len {
                        presences.push(match rng.next() % 4 {
                            0 => 0.0,
                            1 => 1.0,
                            _ => rng.unit(),
                        });
                    }
                }
            }
            a
        })
        .collect()
}

/// The CPU's results for one case.
fn expected(s: Selector, a: &[u64; 8], presences: &[f32]) -> [u64; 2] {
    let f = |k: usize| f32::from_bits(a[k] as u32);
    match s {
        S::ChildDraw => [c::child(a[0], a[1]), c::draw(a[0])],
        S::Internode => [c::internode_draw(a[0]), 0],
        S::Angles => [c::insertion_draw(a[0]), c::tropism_draw(a[0])],
        S::Convert => [w(a[0] as f32), f(1) as u64],
        S::Threshold => [c::threshold(f(0)), (f64::from(f(0)) * c::ONE as f64) as u64],
        S::Logs => [w(c::neg_log2(f(0))), w(c::prob(f(1)))],
        S::Logit => [w(c::logit(f(0))), w(c::logit53(a[1]))],
        S::Survival => [decide::survival_fine(f(0), a[1]), w(c::scaled(f(0), a[1]))],
        S::Abortion => [
            decide::abortion_fine(f(0), (a[1] != 0).then(|| f(2)), a[3], a[4]),
            decide::lived_ticks(f(5), a[6], a[7]),
        ],
        S::Lived => [
            decide::lived_fine(f(5), a[6], a[7]),
            w(decide::draw_log2(a[7])),
        ],
        S::Moved => [
            w(decide::moved_f(
                f(0),
                a[1],
                (f(2), f(3)),
                a[4],
                (f(5), f(6), f(7)),
            )),
            0,
        ],
        S::Past => [w(decide::past(a[0], f(1))), w(decide::above(a[0], f(1)))],
        S::Wake => [decide::wake_ticks(a[0], f(1), a[2]).unwrap_or(u64::MAX), 0],
        S::Lead => [w(pr::over(a[0], a[2]).min(pr::under(a[1], a[2]))), 0],
        S::Window => {
            let wood = pr::wood_at((f(2), f(3)), a[4]);
            [
                w(pr::window(wood, (f(5), f(6), f(7)))),
                w(c::presence(f(0), f(1))),
            ]
        }
        S::Carry => {
            let (gap, age) = ((a[2] & 0xffff_ffff) as u32, (a[2] >> 32) as u32);
            [
                w(pr::carry(f(0), f(1), gap, f(3))),
                w(pr::fade(f(4), a[5] as u32, (f(6), f(7)), age)),
            ]
        }
        S::RelayNode => {
            let unit = &presences[a[0] as usize..(a[0] + a[1]) as usize];
            [decide::relay_node_f(unit, 0, unit.len(), f(2)) as u64, 0]
        }
    }
}

/// How two results of one function may differ.
#[derive(Debug, Clone, Copy)]
enum Tolerance {
    /// To the bit.
    Exact,
    /// Two f32 within `2^-e + REL |x|` of each other.
    Near(i32),
    /// Two draw thresholds whose chances are `Near`.
    Chance,
    /// Two counts of ticks or of 2^-32 of a cycle within this many.
    Count(u64),
}

/// What each selector's two results are held to.
fn tolerance(s: Selector) -> [Tolerance; 2] {
    use Tolerance::*;
    match s {
        S::ChildDraw | S::Internode | S::Angles | S::Convert | S::Threshold => [Exact; 2],
        S::Logs | S::Logit | S::Past | S::Window | S::Carry => [Near(ABS); 2],
        S::Survival => [Chance, Near(ABS)],
        S::Abortion => [Chance, Count(1)],
        S::Lived => [Count(1 << 12), Near(ABS)],
        S::Lead => [Near(ABS), Exact],
        // A presence is a lead over a window as narrow as 1/20: a lead's
        // two log-odds of up to 37 each, apart by their last places,
        // twenty times over.
        S::Moved => [Near(14), Exact],
        S::Wake => [Count(WAKE), Exact],
        S::RelayNode => [Exact; 2],
    }
}

/// How far two executors' f32 results may differ by the same operations
/// (the device's division within 2.5 units in the last place, its sums
/// and products fused or not): twice the most measured over the probe's
/// cases on the RTX 3080, to a power of two (fn-254 R4; `PROBE.md`), its
/// absolute part `2^-ABS`.
const ABS: i32 = 20;
const REL: f64 = 1.0 / (1u64 << 17) as f64;
/// The ticks a waking time may differ by: twice the most measured.
const WAKE: u64 = 1 << 8;

/// How far apart two results are, as a share of what `t` allows, and
/// whether that is within it.
fn apart(t: Tolerance, cpu: u64, gpu: u64) -> (f64, bool) {
    let near =
        |e: i32, a: f64, b: f64| (a - b).abs() / (2f64.powi(-e) + REL * a.abs().max(b.abs()));
    let real = |x: u64| f64::from(f32::from_bits(x as u32));
    let chance = |t: u64| t as f64 / c::ONE as f64;
    let d = match t {
        Tolerance::Exact => f64::from(u8::from(cpu != gpu)),
        Tolerance::Near(e) => near(e, real(cpu), real(gpu)),
        Tolerance::Chance => near(ABS, chance(cpu), chance(gpu)),
        Tolerance::Count(n) => cpu.abs_diff(gpu) as f64 / n as f64,
    };
    (d, d <= 1.0)
}

/// The contract probe's WGSL.
pub(super) fn source() -> String {
    format!("{}\n{PROBE}", super::contract())
}

struct Kernel {
    pass: io::Pass,
}

impl Kernel {
    fn new(gpu: &Gpu) -> Self {
        Self {
            pass: io::Pass::new(gpu, source(), &[true, true, false], &["main"]),
        }
    }

    /// Selector `s` over `cases` on the GPU: the results, and the time
    /// from upload to readback.
    fn run(
        &self,
        gpu: &Gpu,
        s: Selector,
        cases: &[[u64; 8]],
        presences: &[f32],
    ) -> (Vec<[u64; 2]>, f64) {
        let start = Instant::now();
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let n = cases.len() as u32;
        let groups = n.div_ceil(64);
        let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension) * 64;
        let cfg = [n, s as u32, row, 0];
        let words: &[u8] = bytemuck::cast_slice(cases);
        let uniform = io::buffer(
            gpu,
            "contract cfg",
            16,
            wgpu::BufferUsages::UNIFORM,
            bytemuck::cast_slice(&cfg),
        )
        .unwrap();
        let input = io::buffer(gpu, "contract cases", words.len() as u64, storage, words).unwrap();
        let held = io::buffer(
            gpu,
            "contract presences",
            (presences.len() as u64 * 4).max(16),
            storage,
            bytemuck::cast_slice(presences),
        )
        .unwrap();
        let bytes = cases.len() as u64 * 16;
        let output = io::buffer(gpu, "contract results", bytes, storage, &[]).unwrap();
        let bind = self.pass.bind(gpu, &[&uniform, &input, &held, &output]);
        let mut encoder = gpu.encoder(&Default::default());
        self.pass.run(&mut encoder, &bind, 0, groups);
        gpu.submit([encoder.finish()]);
        let out = io::read(gpu, &output, bytes).unwrap();
        let ms = start.elapsed().as_secs_f64() * 1e3;
        let results = bytemuck::cast_slice::<u8, u64>(&out)
            .as_chunks::<2>()
            .0
            .to_vec();
        (results, ms)
    }
}

fn selector(s: Selector) -> (Vec<[u64; 8]>, Vec<f32>) {
    let mut presences = Vec::new();
    let cases = cases(s, &mut Cases(0x5eed + s as u64), &mut presences);
    (cases, presences)
}

#[test]
fn the_gpu_takes_every_decision_the_cpu_does() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let probe = Kernel::new(&gpu);
    let mut failed = Vec::new();
    for (s, _, _) in ALL {
        let (cases, presences) = selector(s);
        let (got, _) = probe.run(&gpu, s, &cases, &presences);
        let held = tolerance(s);
        let (mut most, mut wrong) = ([0f64; 2], [0usize; 2]);
        for (a, g) in cases.iter().zip(&got) {
            let want = expected(s, a, &presences);
            for k in 0..2 {
                let (d, ok) = apart(held[k], want[k], g[k]);
                most[k] = most[k].max(d);
                if !ok {
                    if wrong[k] < 3 {
                        eprintln!(
                            "{}[{k}]: {a:x?}: cpu {:x} gpu {:x}",
                            s.name(),
                            want[k],
                            g[k]
                        );
                    }
                    wrong[k] += 1;
                }
            }
        }
        println!("{}: most apart, of its bound {most:.3?}", s.name());
        for k in 0..2 {
            if wrong[k] > 0 {
                failed.push(format!(
                    "{}[{k}]: {} of {} cases",
                    s.name(),
                    wrong[k],
                    cases.len()
                ));
            }
        }
    }
    assert!(failed.is_empty(), "past their tolerance: {failed:?}");
}

/// The contract's cost per case on the GPU (upload to readback, cold and
/// warm) against the CPU on one core and on all: `cargo test --profile ci
/// -p telperion-render contract_cost -- --ignored --nocapture`.
#[test]
#[ignore = "a measurement, not a check"]
fn contract_cost() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let cores = std::thread::available_parallelism().map_or(1, |n| n.get());
    let started = Instant::now();
    let probe = Kernel::new(&gpu);
    println!(
        "pipeline built in {:.1} ms",
        started.elapsed().as_secs_f64() * 1e3
    );
    println!("| Functions | GPU cold ms | GPU warm ms | CPU 1 core ms | CPU {cores} cores ms | ns a case: GPU warm / CPU 1 / CPU all |");
    println!("|---|--:|--:|--:|--:|--:|");
    for (s, _, _) in ALL {
        let (cases, presences) = selector(s);
        let (_, cold) = probe.run(&gpu, s, &cases, &presences);
        let (_, warm) = probe.run(&gpu, s, &cases, &presences);
        let t = Instant::now();
        let one: Vec<[u64; 2]> = cases.iter().map(|a| expected(s, a, &presences)).collect();
        let single = t.elapsed().as_secs_f64() * 1e3;
        let t = Instant::now();
        let chunk = cases.len().div_ceil(cores);
        let all: Vec<[u64; 2]> = std::thread::scope(|scope| {
            let parts: Vec<_> = cases
                .chunks(chunk)
                .map(|part| {
                    scope.spawn(|| {
                        part.iter()
                            .map(|a| expected(s, a, &presences))
                            .collect::<Vec<_>>()
                    })
                })
                .collect();
            parts.into_iter().flat_map(|p| p.join().unwrap()).collect()
        });
        let many = t.elapsed().as_secs_f64() * 1e3;
        assert_eq!(one, all);
        let per = |ms: f64| ms * 1e6 / cases.len() as f64;
        println!(
            "| {} | {cold:.2} | {warm:.2} | {single:.2} | {many:.2} | {:.1} / {:.1} / {:.1} |",
            s.name(),
            per(warm),
            per(single),
            per(many)
        );
    }
}
