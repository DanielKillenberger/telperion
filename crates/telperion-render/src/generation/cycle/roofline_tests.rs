//! fn-256 task 7, measured (`#[ignore = "a measurement"]`): the card's
//! practical peaks (a copy kernel's bandwidth, an FMA kernel's rate), and
//! for Telperion 82, the oak 40 and the beech 40 at seed 42, grown as a
//! build grows them: every dispatch's stamped time by cycle, phase and
//! kernel, each cycle's census, and what each cycle visits
//! (`roofline_life`). Files go to `ROOFLINE_OUT`; `ROOFLINE_PRESETS`
//! names a comma list. `cargo test --profile ci -p telperion-render
//! the_roofline_is_measured -- --ignored --nocapture`, under the GPU lock.
use super::build::Tables;
use super::build_arena::hint;
use super::build_grow::{grow_now as grow, Sizes, COMPACTION};
use super::grown_tests::{device, grown_from};
use super::roofline_life::{lives, Lives};
use crate::Gpu;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use telperion_core::presets::Preset;

const CASES: [(&str, u32, u64); 3] = [("telperion", 82, 42), ("oregon-white-oak", 40, 42), ("european-beech", 40, 42)];

#[test]
#[ignore = "a measurement"]
fn the_roofline_is_measured() {
    let _held = super::super::whole_tree();
    let out = PathBuf::from(std::env::var("ROOFLINE_OUT").expect("ROOFLINE_OUT names a directory"));
    std::fs::create_dir_all(&out).expect("the output directory");
    let Some((gpu, kit)) = device() else {
        return;
    };
    peaks(&gpu, &out);
    let only = std::env::var("ROOFLINE_PRESETS").ok();
    for (id, age, seed) in CASES.into_iter().filter(|c| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == c.0))) {
        let growth = telperion_core::pipeline::executor::growth(&Preset::from_id(id).expect("a preset").parameters()).expect("a growth");
        let g = grown_from(growth.species, growth.request.light, (age, seed));
        let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
        let hinted = Sizes::hinted(hint(&g.species, age).expect("a hint")).expect("sized");
        // Warm: the build's buffers and the driver's caches made once.
        drop(grow(&gpu, &kit, &t, &g.tables, (hinted, age)).expect("a build"));
        gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("freed");
        assert!(super::super::profile::start(&gpu), "timestamps");
        let (b, _, grew) = grow(&gpu, &kit, &t, &g.tables, (hinted, age)).expect("a build");
        let stamps = super::super::profile::finish(&gpu);
        assert_eq!(stamps.len(), grew.stamped.len(), "a phase a stamp");
        assert_eq!(grew.moves, 0, "grown in its arena");
        let keeps = b.retire.rec_n > 0;
        let (lit, dirs) = (b.light.lit(), b.light.n_dirs);
        drop(b);
        gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("freed");
        // Each dispatch's cycle: the cycle whose recording it fell in, a
        // compaction counted with the cycle before it.
        let cycle_of = |k: usize| grew.ends.iter().position(|&e| k < e).map_or(age, |c| c as u32 + 1);
        let mut csv = String::from("cycle,phase,kernel,ns\n");
        let mut compacted = Vec::new();
        for (k, (phase, (kernel, a, z))) in grew.stamped.iter().zip(&stamps).enumerate() {
            let cycle = if k < grew.ends[0] && *phase == "start" { 0 } else { cycle_of(k) };
            let cycle = if *phase == COMPACTION { cycle - 1 } else { cycle };
            if *phase == COMPACTION && compacted.last() != Some(&cycle) {
                compacted.push(cycle);
            }
            writeln!(csv, "{cycle},{phase},{kernel},{:.0}", z - a).unwrap();
        }
        write(&out, &format!("{id}-stamps.csv"), &csv);
        let mut csv = String::from("cycle");
        (0..256).for_each(|w| write!(csv, ",w{w}").unwrap());
        for (c, read) in grew.censuses.iter().enumerate() {
            write!(csv, "\n{}", c + 1).unwrap();
            read[..256].iter().for_each(|w| write!(csv, ",{w}").unwrap());
        }
        write(&out, &format!("{id}-census.csv"), &csv);
        println!("{id} {age}: {} dispatches, compacted after {compacted:?}, records kept {keeps}, lit {lit}, directions {dirs}", stamps.len());
        // The CPU reference's visits, unless `ROOFLINE_VISITS=0` asks for
        // the stamps and censuses alone.
        if std::env::var("ROOFLINE_VISITS").is_ok_and(|v| v == "0") {
            continue;
        }
        let held = |let_go| lives(&g.species, g.request, &compacted, let_go);
        let l = held(!keeps);
        write(&out, &format!("{id}-visits.csv"), &visits(&l));
    }
}

/// Each cycle's visits as columns, `_3`, `_5` and `_10` on the short-lived.
fn visits(l: &Lives) -> String {
    let mut csv = format!("# phytomers {:?} laterals {:?}\n", l.phytomers, l.laterals);
    csv.push_str("cycle,held,held_3,held_5,held_10,changed,stopped,grew,grew_3,grew_5,grew_10,live,live_3,live_5,live_10,laid,laid_3,laid_5,laid_10\n");
    for (c, v) in l.cycles.iter().enumerate().skip(1) {
        let q = |x: &[u64; 4]| x.map(|n| n.to_string()).join(",");
        writeln!(csv, "{c},{},{},{},{},{},{}", q(&v.held), v.changed, v.stopped, q(&v.grew), q(&v.live), q(&v.laid)).unwrap();
    }
    csv
}

fn write(out: &Path, name: &str, text: &str) {
    std::fs::write(out.join(name), text).expect("written");
}

/// The card's practical peaks: a copy of 256 MiB through a kernel (its
/// read and write bytes over its stamped time), FMA chains (two flops an
/// FMA) and the same as u32 multiply-adds, each the best of 20.
fn peaks(gpu: &Gpu, out: &Path) {
    const COPY: &str = "@group(0) @binding(0) var<storage, read> a: array<vec4<u32>>;
@group(0) @binding(1) var<storage, read_write> b: array<vec4<u32>>;
@compute @workgroup_size(256) fn main(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let stride = n.x * 256u;
    for (var i = g.x; i < arrayLength(&a); i += stride) { b[i] = a[i]; }
}";
    const FMA: &str = "@group(0) @binding(0) var<storage, read> a: array<vec4<u32>>;
@group(0) @binding(1) var<storage, read_write> b: array<vec4<u32>>;
@compute @workgroup_size(256) fn main(@builtin(global_invocation_id) g: vec3<u32>) {
    var x = vec4<f32>(f32(g.x), 1.0, 2.0, f32(a[0].x)) * 1e-9;
    var y = x + 0.5; var z = x + 0.25; var w = x + 0.125;
    let m = vec4<f32>(0.9999, 0.9998, 0.9997, 0.9996);
    for (var k = 0u; k < 1024u; k++) { x = fma(x, m, y); y = fma(y, m, z); z = fma(z, m, w); w = fma(w, m, x); }
    if (dot(x + y + z + w, vec4<f32>(1.0)) == 12345.0) { b[g.x] = vec4<u32>(1u); }
}";
    const IMAD: &str = "@group(0) @binding(0) var<storage, read> a: array<vec4<u32>>;
@group(0) @binding(1) var<storage, read_write> b: array<vec4<u32>>;
@compute @workgroup_size(256) fn main(@builtin(global_invocation_id) g: vec3<u32>) {
    var x = vec4<u32>(g.x, 1u, 2u, a[0].x);
    var y = x + 5u; var z = x + 7u; var w = x + 11u;
    let m = vec4<u32>(1664525u, 22695477u, 1103515245u, 134775813u);
    for (var k = 0u; k < 1024u; k++) { x = x * m + y; y = y * m + z; z = z * m + w; w = w * m + x; }
    if (dot(x ^ y ^ z ^ w, vec4<u32>(1u)) == 12345u) { b[g.x] = vec4<u32>(1u); }
}";
    let bytes = 256u64 << 20;
    let usage = wgpu::BufferUsages::STORAGE;
    let a = super::super::io::buffer(gpu, "roofline a", bytes, usage, &[]).expect("a");
    let b = super::super::io::buffer(gpu, "roofline b", bytes, usage, &[]).expect("b");
    let copy = best(gpu, (COPY, 32768), (&a, &b));
    // 65,535 workgroups of 256 lanes, 4 chains of vec4 FMAs, 1,024 rounds.
    let fma = best(gpu, (FMA, 65535), (&a, &b));
    // The same chains as u32 multiply-adds (two integer ops each).
    let imad = best(gpu, (IMAD, 65535), (&a, &b));
    let flops = 65535.0 * 256.0 * 1024.0 * 4.0 * 4.0 * 2.0;
    let text = format!(
        "copy 256 MiB: best {:.3} ms, {:.1} GB/s read and written\nfma: best {:.3} ms, {:.2} TFLOP/s fp32\nu32 multiply-add: best {:.3} ms, {:.2} Tops int32\n",
        copy * 1e-6,
        2.0 * bytes as f64 / copy,
        fma * 1e-6,
        flops / fma * 1e-3,
        imad * 1e-6,
        flops / imad * 1e-3
    );
    print!("{text}");
    write(out, "peaks.txt", &text);
}

/// The best stamped time in ns of 20 dispatches of `source`'s `main`.
fn best(gpu: &Gpu, (source, groups): (&str, u32), (a, b): (&wgpu::Buffer, &wgpu::Buffer)) -> f64 {
    let module = gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor { label: Some("roofline"), source: wgpu::ShaderSource::Wgsl(source.into()) });
    let pipeline = gpu.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("roofline"),
        layout: None,
        module: &module,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });
    let entries = [wgpu::BindGroupEntry { binding: 0, resource: a.as_entire_binding() }, wgpu::BindGroupEntry { binding: 1, resource: b.as_entire_binding() }];
    let group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout: &pipeline.get_bind_group_layout(0), entries: &entries });
    let set = gpu.device.create_query_set(&wgpu::QuerySetDescriptor { label: None, ty: wgpu::QueryType::Timestamp, count: 40 });
    let resolved = super::super::io::buffer(gpu, "roofline stamps", 320, wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC, &[]).expect("stamps");
    let mut e = gpu.encoder(&Default::default());
    for k in 0..20u32 {
        let writes = wgpu::ComputePassTimestampWrites { query_set: &set, beginning_of_pass_write_index: Some(2 * k), end_of_pass_write_index: Some(2 * k + 1) };
        let mut pass = e.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: Some(writes) });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(groups, 1, 1);
    }
    e.resolve_query_set(&set, 0..40, &resolved, 0);
    gpu.submit([e.finish()]);
    let raw = super::super::io::read(gpu, &resolved, 320).expect("read");
    let ticks: &[u64] = bytemuck::cast_slice(&raw);
    let period = f64::from(gpu.queue.get_timestamp_period());
    ticks.chunks(2).map(|t| (t[1] - t[0]) as f64 * period).fold(f64::INFINITY, f64::min)
}
