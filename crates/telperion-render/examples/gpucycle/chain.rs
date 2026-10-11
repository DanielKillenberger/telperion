//! The empty dependency chain: 80 cycles × 8 phases of indirect
//! dispatches in one submission, each reading its workgroup count from
//! the dispatch before; in one pass, a pass each, and as direct no-ops.
use crate::gpu::{Gpu, Params, Recording};
use std::time::Instant;

const SET: u32 = 4096;

pub fn chain(gpu: &Gpu, reps: usize) {
    let count = 80 * 8;
    let source = format!(
        "{}\n{}\n{}\n{}",
        include_str!("../../src/generation/fine/wide.wgsl"),
        include_str!("../../src/generation/fine/lineage.wgsl"),
        include_str!("common.wgsl"),
        include_str!("chain.wgsl")
    );
    let kit = gpu.kit(&source, &["step", "noop"], 2);
    println!("chain kit compiled in {:.1} ms", 1e3 * kit.compile);
    let uniform = gpu.uniform(count + 8);
    let args = [
        gpu.zeroed(12 * (count as u64 + 2)),
        gpu.zeroed(12 * (count as u64 + 2)),
    ];
    let state = gpu.zeroed(16);
    let binds = [
        gpu.bind(&kit, &uniform, &[&args[1], &state]),
        gpu.bind(&kit, &uniform, &[&args[0], &state]),
    ];
    for form in ["one pass", "a pass each", "direct no-ops, one pass"] {
        let mut runs = Vec::new();
        for rep in 0..=reps {
            for a in &args {
                gpu.queue
                    .write_buffer(a, 0, bytemuck::cast_slice(&vec![0u32; 3 * (count + 2)]));
            }
            gpu.queue
                .write_buffer(&args[0], 0, bytemuck::cast_slice(&[1u32, 1, 1]));
            gpu.queue
                .write_buffer(&state, 0, bytemuck::cast_slice(&[0u32; 4]));
            let begun = Instant::now();
            let mut r = Recording::new(gpu, &uniform);
            match form {
                "a pass each" => {
                    for k in 0..count {
                        r.indirect(
                            gpu,
                            (&kit, &binds[k % 2]),
                            "step",
                            "chain",
                            (&args[k % 2], 12 * k as u64),
                            offsets_param(k),
                        );
                    }
                }
                _ => {
                    let offsets: Vec<u32> =
                        (0..count).map(|k| r.offset(offsets_param(k))).collect();
                    let (set, at) = r.slot(gpu, "chain");
                    debug_assert!(at + 1 < SET);
                    let pipeline_entry = if form == "one pass" { "step" } else { "noop" };
                    let mut pass = r.encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                        label: None,
                        timestamp_writes: Some(wgpu::ComputePassTimestampWrites {
                            query_set: &r.sets[set],
                            beginning_of_pass_write_index: Some(at),
                            end_of_pass_write_index: Some(at + 1),
                        }),
                    });
                    pass.set_pipeline(kit.pipeline(pipeline_entry));
                    for (k, &o) in offsets.iter().enumerate() {
                        pass.set_bind_group(0, &binds[k % 2], &[o]);
                        if form == "one pass" {
                            pass.dispatch_workgroups_indirect(&args[k % 2], 12 * k as u64);
                        } else {
                            pass.dispatch_workgroups(1, 1, 1);
                        }
                    }
                    drop(pass);
                    r.dispatches += count;
                }
            }
            let (encoding, wall, timed) = r.submit(gpu, begun);
            let done: Vec<u32> = gpu.read(&state, 1);
            if rep > 0 {
                runs.push((encoding, wall, timed.span, done[0]));
            }
        }
        runs.sort_by(|a, b| a.1.total_cmp(&b.1));
        let (e, w, span, done) = runs[runs.len() / 2];
        println!(
            "  {form}: {count} dispatches; encoding {:.2} ms, fenced wall {:.2} ms, GPU first-to-last stamp {:.2} ms, {:.2} µs a dispatch (wall); steps done {done}",
            1e3 * e,
            1e3 * w,
            1e3 * span,
            1e6 * w / count as f64
        );
    }
}

fn offsets_param(k: usize) -> Params {
    Params {
        a: [k as u32, 0, 0, 0, 0, 0, 0, 0],
        f: [0.0; 8],
    }
}
