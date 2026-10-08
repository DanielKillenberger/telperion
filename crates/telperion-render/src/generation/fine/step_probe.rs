//! fn-211.5, probe (b) (DESIGN-INTERLEAVE.md, "After Astra's review",
//! item 5): what stepping fine roots one cycle at a time costs in
//! synchronisation alone, on the oak's real exchange schedule
//! (PHASES.md) with persistent device buffers. Every trip writes the
//! records going down, runs a kernel a thread a record that only touches
//! them, and reads the records coming up; the useful work (growth, lays,
//! the sweep) is left out, so what is timed is the floor a stepped
//! executor pays before it does anything. `cargo test --profile ci -p
//! telperion-render stepping_costs -- --ignored --nocapture`.
use super::super::io::{self, Pass};
use crate::Gpu;
use std::time::Instant;
use telperion_space::{grow, oak, Light, Request};

const SHADER: &str = "
struct Run { n: u32, salt: u32, pad0: u32, pad1: u32 }
@group(0) @binding(0) var<uniform> run: Run;
@group(0) @binding(1) var<storage, read> down: array<u32>;
@group(0) @binding(2) var<storage, read_write> up: array<u32>;
@compute @workgroup_size(64)
fn touch(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x + id.y * 65535u * 64u;
    if (i >= run.n) { return; }
    up[i] = down[i % arrayLength(&down)] ^ run.salt;
}
";

/// One exchange: words down (written before the kernel), words up (read
/// after it).
#[derive(Clone, Copy)]
struct Trip {
    down: u64,
    up: u64,
}

/// The oak's schedule per cycle (PHASES.md): (i) balance, 3 words up and
/// 3 down a root; (ii) demand, 2 up (an f64) and 2 down; (iii) the
/// normalisers, each PA's two Q64.64 sums (8 words) up and down; (iv) the
/// lattice's Q64.64 deposits up (4 words a node) and its swept light
/// down (2). A re-lay adds two trips a root: its section, retained pipe,
/// mass and moment up (12 words), its bearer's frame down (24), then its
/// load up and its bent frame down.
fn schedule(roots: u64, pas: u64, nodes: u64, relay: bool) -> Vec<Trip> {
    let mut trips = vec![
        Trip {
            down: 3 * roots,
            up: 3 * roots,
        },
        Trip {
            down: 2 * roots,
            up: 2 * roots,
        },
        Trip {
            down: 8 * pas,
            up: 8 * pas,
        },
        Trip {
            down: 2 * nodes,
            up: 4 * nodes,
        },
    ];
    if relay {
        let frame = Trip {
            down: 24 * roots,
            up: 12 * roots,
        };
        trips.extend([frame, frame]);
    }
    trips
}

struct Buffers {
    run: crate::buffer::Owned,
    down: crate::buffer::Owned,
    up: crate::buffer::Owned,
    staging: crate::buffer::Owned,
    bind: wgpu::BindGroup,
}

/// Milliseconds: writing and submitting, waiting for the device, and
/// reading the mapped words.
#[derive(Default, Clone, Copy)]
struct Spent {
    submit: f64,
    wait: f64,
    read: f64,
}

fn trip(gpu: &Gpu, pass: &Pass, b: &Buffers, t: Trip, salt: u32) -> Spent {
    let start = Instant::now();
    let down = vec![salt; t.down.max(1) as usize];
    gpu.queue
        .write_buffer(&b.down, 0, bytemuck::cast_slice(&down));
    let run = [t.up as u32, salt, 0, 0];
    gpu.queue
        .write_buffer(&b.run, 0, bytemuck::cast_slice(&run));
    let mut encoder = gpu.encoder(&Default::default());
    pass.run(&mut encoder, &b.bind, 0, (t.up as u32).div_ceil(64));
    let bytes = t.up * 4;
    encoder.copy_buffer_to_buffer(&b.up, 0, &b.staging, 0, bytes);
    gpu.submit([encoder.finish()]);
    let (send, receive) = std::sync::mpsc::channel();
    b.staging
        .slice(..bytes)
        .map_async(wgpu::MapMode::Read, move |r| {
            let _ = send.send(r);
        });
    let submitted = Instant::now();
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("the device");
    receive.recv().expect("mapped").expect("mapped");
    let waited = Instant::now();
    let view = b.staging.slice(..bytes).get_mapped_range().expect("a view");
    let words: Vec<u32> = bytemuck::cast_slice(&view).to_vec();
    drop(view);
    b.staging.unmap();
    assert_eq!(words[0], salt ^ salt, "the kernel ran");
    let done = Instant::now();
    Spent {
        submit: (submitted - start).as_secs_f64() * 1e3,
        wait: (waited - submitted).as_secs_f64() * 1e3,
        read: (done - waited).as_secs_f64() * 1e3,
    }
}

/// The lattice's nodes over the lit oak's extent at the light's 0.5 m.
fn nodes(age: u32, seed: u64) -> u64 {
    let tree = grow(
        &oak(),
        Request {
            age,
            seed,
            budget: 20_000_000,
            light: Light {
                extinction: 0.5,
                sky: 0.5,
            },
        },
    )
    .expect("grows");
    let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
    for p in tree.axes.iter().flat_map(|a| &a.phytomers) {
        for (k, c) in [p.tip.x, p.tip.y, p.tip.z].into_iter().enumerate() {
            lo[k] = lo[k].min(c);
            hi[k] = hi[k].max(c);
        }
    }
    (0..3)
        .map(|k| ((hi[k] / 0.5).floor() - (lo[k] / 0.5).floor() + 2.0) as u64)
        .product()
}

#[test]
#[ignore = "a measurement"]
fn stepping_costs() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let pass = Pass::new(&gpu, SHADER.into(), &[true, false], &["touch"]);
    let pas = oak().states.len() as u64;
    let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    println!("load at the start: {}", load.trim());
    println!("| Oak | Roots | Lattice nodes | Trips | Total ms | Per trip median / max ms | Submit / wait / read ms |");
    println!("|---|--:|--:|--:|--:|--:|--:|");
    // Fine roots at cut 1,024 (TASK4 PHASES.md, stepped executor's count).
    for (age, seed, roots) in [
        (40u32, 1u64, 1_821u64),
        (40, 7, 2_937),
        (80, 1, 7_179),
        (80, 7, 17_370),
    ] {
        let nodes = nodes(age, seed);
        let most = (0..=1)
            .flat_map(|r| schedule(roots, pas, nodes, r == 1))
            .map(|t| t.up.max(t.down))
            .max()
            .expect("a trip");
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let b = |label, usage| io::buffer(&gpu, label, most * 4, usage, &[]).expect("a buffer");
        let (run, down, up) = (
            io::buffer(&gpu, "run", 16, wgpu::BufferUsages::UNIFORM, &[]).expect("a buffer"),
            b("down", storage),
            b("up", storage),
        );
        let staging = b("staging", wgpu::BufferUsages::MAP_READ);
        let bind = pass.bind(&gpu, &[&run, &down, &up]);
        let bufs = Buffers {
            run,
            down,
            up,
            staging,
            bind,
        };
        for round in 0..2 {
            let started = Instant::now();
            let mut each = Vec::new();
            for cycle in 1..age {
                for t in schedule(roots, pas, nodes, cycle % 10 == 0) {
                    each.push(trip(&gpu, &pass, &bufs, t, cycle));
                }
            }
            let total = started.elapsed().as_secs_f64() * 1e3;
            let mut per: Vec<f64> = each.iter().map(|s| s.submit + s.wait + s.read).collect();
            per.sort_by(f64::total_cmp);
            let sum = |f: fn(&Spent) -> f64| each.iter().map(f).sum::<f64>();
            println!(
                "| {age} s{seed} ({}) | {roots} | {nodes} | {} | {total:.0} | {:.2} / {:.2} | {:.0} / {:.0} / {:.0} |",
                ["cold", "warm"][round],
                each.len(),
                per[per.len() / 2],
                per[per.len() - 1],
                sum(|s| s.submit),
                sum(|s| s.wait),
                sum(|s| s.read),
            );
        }
    }
    let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    println!("load at the end: {}", load.trim());
}
