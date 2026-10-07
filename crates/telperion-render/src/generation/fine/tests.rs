//! The contract's mirror against the CPU's (fn-209 F1, R1): every function
//! of `telperion_space::fixed` and the integer forms of `fixed::decide`,
//! on the GPU and the CPU, over a quarter of a million cases each, the
//! threshold-adjacent and saturating ones among them, give the same bits.
use super::super::io;
use crate::Gpu;
use std::time::Instant;
use telperion_space::fixed::{self as fx, decide, presence as pr};

const CASES: usize = 1 << 18;
const PROBE: &str = include_str!("probe.wgsl");

/// The functions the probe selects, by its `cfg.y`.
const NAMES: [&str; 16] = [
    "child, draw",
    "log2, exp2_neg",
    "exp2, neg_log2",
    "ln_ratio, exp",
    "mul, presence",
    "survival_q, scaled",
    "abortion_q",
    "moved_q",
    "past_q, above_q",
    "wake_q, kept",
    "relay_node_q",
    "and",
    "under_q, over_q",
    "times_q, window_q, wood_at_q",
    "carry_q, fade_q",
    "lead_q",
];

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
    fn within(&mut self, lo: i64, hi: i64) -> i64 {
        let span = (hi as i128 - lo as i128 + 1) as u128;
        match self.next() % 16 {
            0 => lo,
            1 => hi,
            2 => (lo + 1).min(hi),
            3 => (hi - 1).max(lo),
            _ => (lo as i128 + (u128::from(self.next()) % span) as i128) as i64,
        }
    }
    fn draw(&mut self) -> u64 {
        self.within(0, (1 << 53) - 1) as u64
    }
    fn chance(&mut self) -> u64 {
        self.within(0, fx::P_ONE as i64) as u64
    }
    fn log(&mut self, most: i64) -> i64 {
        if self.next().is_multiple_of(8) {
            fx::INF
        } else {
            self.within(0, most)
        }
    }
}

const Q: i64 = fx::ONE32;

/// Selector `s`'s cases, and the presences the relay cases read.
fn cases(s: u32, rng: &mut Cases, presences: &mut Vec<u64>) -> Vec<[u64; 8]> {
    (0..CASES)
        .map(|_| {
            let mut a = [0u64; 8];
            match s {
                0 => a[..2].copy_from_slice(&[rng.next(), rng.next()]),
                1 => {
                    a[0] = rng.next() >> (rng.next() % 64);
                    a[1] = rng.within(-(1 << 40), 70 * Q) as u64;
                }
                2 => {
                    a[0] = rng.within(-70 * Q, 70 * Q) as u64;
                    a[1] = rng.next() >> (rng.next() % 64);
                }
                3 => {
                    a[0] = rng.next() >> (rng.next() % 64);
                    a[1] = rng.next() >> (rng.next() % 64);
                    a[2] = rng.within(-fx::INF, 50 * Q) as u64;
                }
                4 => {
                    a[0] = rng.within(-fx::INF, fx::INF) as u64;
                    a[1] = rng.within(-(1 << 40), 1 << 40) as u64;
                    a[2] = if rng.next().is_multiple_of(8) {
                        fx::INF
                    } else {
                        rng.within(-(1 << 40), 1 << 45)
                    } as u64;
                    a[3] = rng.within(-(1 << 10), 1 << 45) as u64;
                }
                5 => {
                    a[0] = rng.log(70 * Q) as u64;
                    a[1] = rng.within(0, 1 << 21) as u64;
                }
                6 => {
                    a[0] = rng.log(60 * Q) as u64;
                    a[1] = rng.next() % 2;
                    a[2] = rng.within(-4 * Q, 4 * Q) as u64;
                    a[3] = rng.within(0, 4096 << 20) as u64;
                    a[4] = rng.within(0, 1 << 20) as u64;
                }
                7 => {
                    let stake = rng.within(0, 1 << 20) as f64 / f64::from(1 << 20);
                    let wood = |rng: &mut Cases| match rng.next() % 6 {
                        0 => -fx::INF,
                        _ => rng.within(-30 * Q, Q),
                    };
                    a[0] = rng.chance();
                    a[1] = rng.draw();
                    a[2] = wood(rng) as u64;
                    a[3] = wood(rng) as u64;
                    a[4] = rng.within(0, 1 << 20) as u64;
                    a[5] = fx::q32(6.0 * stake) as u64;
                    a[6] = fx::q32(0.05 * stake) as u64;
                    a[7] = fx::q32(2.0) as u64;
                }
                8 => a[..2].copy_from_slice(&[rng.draw(), rng.chance()]),
                9 => {
                    a[0] = rng.within(0, 1 << 24) as u64;
                    a[1] = fx::q32(1.0 / (0.01 + rng.within(0, 1000) as f64 / 100.0)) as u64;
                    a[2] = rng.draw();
                    a[3] = rng.log(70 * Q) as u64;
                    a[4] = rng.next() % 2;
                    a[5] = rng.log(70 * Q) as u64;
                    a[6] = rng.draw();
                }
                11 => a[..2].copy_from_slice(&[rng.log(fx::INF) as u64, rng.log(fx::INF) as u64]),
                12 => {
                    let t = match rng.next() % 8 {
                        0 => 0,
                        1 => 1 << 53,
                        _ => rng.draw(),
                    };
                    a[0] = t;
                    a[1] = match rng.next() % 4 {
                        0 => 0,
                        1 => t.saturating_sub(rng.next() % 4),
                        2 => t + rng.next() % 4,
                        _ => rng.draw(),
                    };
                }
                13 => {
                    let wood = |rng: &mut Cases| match rng.next() % 6 {
                        0 => -fx::INF,
                        _ => rng.within(-30 * Q, Q),
                    };
                    let stake = rng.within(0, 1 << 20) as f64 / f64::from(1 << 20);
                    a[0] = rng.within(0, Q + 1) as u64;
                    a[1] = rng.within(0, 1 << 44) as u64;
                    a[2] = wood(rng) as u64;
                    a[3] = wood(rng) as u64;
                    a[4] = rng.within(0, 1 << 20) as u64;
                    a[5] = fx::q32(6.0 * stake) as u64;
                    a[6] = fx::q32(0.05 * stake) as u64;
                    a[7] = fx::q32(2.0) as u64;
                }
                15 => {
                    let d = match rng.next() % 8 {
                        0 => 0,
                        _ => rng.draw(),
                    };
                    let near = |rng: &mut Cases| match rng.next() % 4 {
                        0 => d.saturating_sub(rng.next() % 4),
                        1 => d + rng.next() % 4,
                        _ => rng.draw(),
                    };
                    a[0] = match rng.next() % 4 {
                        0 => 0,
                        _ => near(rng),
                    };
                    a[1] = match rng.next() % 4 {
                        0 => 1 << 53,
                        _ => near(rng),
                    };
                    a[2] = d;
                    // Each bound's log-odds given or not; a value not
                    // given is noise the lead must not read.
                    a[3] = rng.next() % 4;
                    a[4] = match a[3] & 1 {
                        0 => rng.next(),
                        _ => fx::logit(a[0] << 9) as u64,
                    };
                    a[5] = match a[3] & 2 {
                        0 => rng.next(),
                        _ => fx::logit(a[1] << 9) as u64,
                    };
                }
                14 => {
                    let age = rng.within(1, 4096) as u64;
                    a[0] = rng.within(0, 1 << 44) as u64;
                    a[1] = rng.within(0, Q + 1) as u64;
                    a[2] = (age << 32) | rng.within(0, 100) as u64;
                    a[3] = rng.within(0, 1 << 44) as u64;
                    a[4] = rng.within(0, 1 << 40) as u64;
                    a[5] = rng.within(0, age as i64 + 1) as u64;
                    a[6] = rng.within(0, (age as i64) << 32) as u64;
                    a[7] = match rng.next() % 3 {
                        0 => 0,
                        _ => rng.within(0, 2 * Q) as u64,
                    };
                }
                _ => {
                    let len = 1 + rng.next() % 12;
                    a[0] = presences.len() as u64;
                    a[1] = len;
                    a[2] = rng.within(0, Q) as u64;
                    for _ in 0..len {
                        presences.push(match rng.next() % 4 {
                            0 => 0,
                            1 => Q as u64,
                            _ => rng.within(0, Q) as u64,
                        });
                    }
                }
            }
            a
        })
        .collect()
}

/// The CPU's results for one case.
fn expected(s: u32, a: &[u64; 8], presences: &[u64]) -> [u64; 2] {
    let i = |k: usize| a[k] as i64;
    match s {
        0 => [fx::child(a[0], a[1]), fx::draw(a[0])],
        1 => [fx::log2(a[0]) as u64, fx::exp2_neg(i(1))],
        2 => [fx::exp2(i(0)) as u64, fx::neg_log2(a[1]) as u64],
        3 => [fx::ln_ratio(a[0], a[1]) as u64, fx::exp(i(2)) as u64],
        4 => [fx::mul(i(0), i(1)) as u64, fx::presence(i(2), i(3))],
        5 => [
            decide::survival_q(i(0), a[1]),
            fx::scaled(i(0), a[1]) as u64,
        ],
        6 => [
            decide::abortion_q(i(0), (a[1] != 0).then(|| i(2)), a[3], a[4]),
            0,
        ],
        7 => {
            let window = (i(5), i(6), i(7));
            [decide::moved_q(a[0], a[1], (i(2), i(3)), a[4], window), 0]
        }
        8 => [
            decide::past_q(a[0], a[1]) as u64,
            decide::above_q(a[0], a[1]) as u64,
        ],
        9 => [
            decide::wake_q(a[0], i(1), a[2]).unwrap_or(u64::MAX),
            u64::from(decide::kept(i(3), (a[4] != 0).then(|| i(5)), a[6])),
        ],
        11 => [decide::and(i(0), i(1)) as u64, 0],
        12 => [
            pr::under(a[0], a[1]) as u64,
            pr::over(a[0], a[1]) as u64,
        ],
        13 => {
            let wood = pr::wood_at((i(2), i(3)), a[4]);
            [pr::times(a[0], a[1]), pr::window(wood, (i(5), i(6), i(7))) as u64]
        }
        14 => {
            let (gap, age) = (a[2] & 0xffff_ffff, (a[2] >> 32) as u32);
            [
                pr::carry(a[0], a[1], gap, a[3]),
                pr::fade(a[4], a[5] as u32, (a[6], a[7]), age),
            ]
        }
        15 => [pr::over(a[0], a[2]).min(pr::under(a[1], a[2])) as u64, 0],
        _ => {
            let unit = &presences[a[0] as usize..(a[0] + a[1]) as usize];
            [decide::relay_node_q(unit, 0, unit.len(), a[2]) as u64, 0]
        }
    }
}

/// The contract probe's WGSL.
pub(super) fn source() -> String {
    format!("{}\n{PROBE}", super::contract())
}

struct Probe {
    pass: io::Pass,
}

impl Probe {
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
        s: u32,
        cases: &[[u64; 8]],
        presences: &[u64],
    ) -> (Vec<[u64; 2]>, f64) {
        let start = Instant::now();
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let n = cases.len() as u32;
        let groups = n.div_ceil(64);
        let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension) * 64;
        let cfg = [n, s, row, 0];
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
            presences.len() as u64 * 8,
            storage,
            bytemuck::cast_slice(presences),
        )
        .unwrap();
        let bytes = cases.len() as u64 * 16;
        let output = io::buffer(gpu, "contract results", bytes, storage, &[]).unwrap();
        let bind = self.pass.bind(gpu, &[&uniform, &input, &held, &output]);
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        self.pass.run(&mut encoder, &bind, 0, groups);
        gpu.queue.submit([encoder.finish()]);
        let out = io::read(gpu, &output, bytes).unwrap();
        let ms = start.elapsed().as_secs_f64() * 1e3;
        let results = bytemuck::cast_slice::<u8, u64>(&out)
            .as_chunks::<2>()
            .0
            .to_vec();
        (results, ms)
    }
}

fn selector(s: u32) -> (Vec<[u64; 8]>, Vec<u64>) {
    let mut presences = Vec::new();
    let cases = cases(s, &mut Cases(0x5eed + u64::from(s)), &mut presences);
    (cases, presences)
}

#[test]
fn the_gpu_takes_every_decision_the_cpu_does() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let probe = Probe::new(&gpu);
    for s in 0..NAMES.len() as u32 {
        let (cases, presences) = selector(s);
        let (got, _) = probe.run(&gpu, s, &cases, &presences);
        let mut wrong = 0;
        for (a, g) in cases.iter().zip(&got) {
            let want = expected(s, a, &presences);
            if want != *g {
                if wrong < 5 {
                    eprintln!("{}: {a:x?}: cpu {want:x?} gpu {g:x?}", NAMES[s as usize]);
                }
                wrong += 1;
            }
        }
        assert_eq!(
            wrong,
            0,
            "{}: {wrong} of {} cases differ",
            NAMES[s as usize],
            cases.len()
        );
    }
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
    let probe = Probe::new(&gpu);
    println!(
        "pipeline built in {:.1} ms",
        started.elapsed().as_secs_f64() * 1e3
    );
    println!("| Functions | GPU cold ms | GPU warm ms | CPU 1 core ms | CPU {cores} cores ms | ns a case: GPU warm / CPU 1 / CPU all |");
    println!("|---|--:|--:|--:|--:|--:|");
    for s in 0..NAMES.len() as u32 {
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
            NAMES[s as usize],
            per(warm),
            per(single),
            per(many)
        );
    }
}
