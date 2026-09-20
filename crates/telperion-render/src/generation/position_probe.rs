//! Native-only feasibility diagnostic; never selected by a generation request.
use super::*;
use std::time::Instant;
use telperion_core::surface::compact::{self, CompactSurface};

struct Candidate {
    positions: wgpu::Buffer,
    status: [u32; 8],
    upload_ms: f64,
    completion_ms: f64,
    gpu_bytes: u64,
    cpu_bytes: usize,
}
fn millis(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}
fn execute(g: &Generator, pass: &io::Pass, p: &CompactSurface) -> Candidate {
    let start = Instant::now();
    let metadata: Vec<[u32; 4]> = p
        .runs
        .iter()
        .map(|r| [r.base, r.first_index, r.ring_start, r.rings])
        .collect();
    let cfg = [
        p.rings.len() as u32,
        p.runs.len() as u32,
        p.segments,
        g.gpu.device.limits().max_compute_workgroups_per_dimension,
        p.lobes,
        p.depth.to_bits(),
        0,
        0,
    ];
    let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let uniform = io::buffer(
        &g.gpu,
        "probe config",
        32,
        wgpu::BufferUsages::UNIFORM,
        bytemuck::cast_slice(&cfg),
    )
    .unwrap();
    let rings = io::buffer(
        &g.gpu,
        "probe rings",
        p.rings.len() as u64 * 64,
        storage,
        bytemuck::cast_slice(&p.rings),
    )
    .unwrap();
    let angular = io::buffer(
        &g.gpu,
        "probe angular",
        p.angular.len() as u64 * 16,
        storage,
        bytemuck::cast_slice(&p.angular),
    )
    .unwrap();
    let runs = io::buffer(
        &g.gpu,
        "probe runs",
        metadata.len() as u64 * 16,
        storage,
        bytemuck::cast_slice(&metadata),
    )
    .unwrap();
    let positions = io::buffer(
        &g.gpu,
        "probe positions",
        p.vertices as u64 * 12,
        storage,
        &[],
    )
    .unwrap();
    let partial = io::buffer(
        &g.gpu,
        "probe bounds partial",
        p.runs.len() as u64 * 32,
        storage,
        &[],
    )
    .unwrap();
    let status = io::buffer(&g.gpu, "probe status", 32, storage, &[]).unwrap();
    let bind = pass.bind(
        &g.gpu,
        &[
            &uniform, &rings, &angular, &runs, &positions, &partial, &status,
        ],
    );
    let mut encoder = g.gpu.device.create_command_encoder(&Default::default());
    if !p.rings.is_empty() {
        pass.run(&mut encoder, &bind, 0, p.rings.len() as u32);
        pass.run(&mut encoder, &bind, 1, p.runs.len() as u32);
    }
    pass.run(&mut encoder, &bind, 2, 1);
    g.gpu.queue.submit([encoder.finish()]);
    let upload_ms = millis(start);
    let wait = Instant::now();
    let bytes = io::read(&g.gpu, &status, 32).unwrap();
    let completion_ms = millis(wait);
    let status_words = bytes
        .chunks_exact(4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let gpu_bytes = [
        &uniform, &rings, &angular, &runs, &positions, &partial, &status,
    ]
    .iter()
    .map(|b| b.size())
    .sum::<u64>()
        + 32;
    let cpu_bytes = p.rings.capacity() * 64
        + p.angular.capacity() * 16
        + p.runs.capacity() * size_of::<surface::prepared::Run>()
        + p.run_table.capacity() * size_of::<surface::SurfaceRun>()
        + metadata.capacity() * 16;
    Candidate {
        positions,
        status: status_words,
        upload_ms,
        completion_ms,
        gpu_bytes,
        cpu_bytes,
    }
}
fn read(g: &Generator, c: &Candidate, vertices: u32) -> Vec<f32> {
    io::read(&g.gpu, &c.positions, vertices as u64 * 12)
        .unwrap()
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
        .collect()
}
fn point(p: &[f32], i: u32) -> Vec3 {
    let i = i as usize * 3;
    Vec3::new(p[i] as f64, p[i + 1] as f64, p[i + 2] as f64)
}
fn admitted(n: Vec3) -> bool {
    let scale = n.x.abs().max(n.y.abs()).max(n.z.abs());
    n.is_finite()
        && n.length_squared() > 0.0
        && scale >= f32::MIN_POSITIVE as f64
        && scale < f32::MAX as f64 / 256.0
}
fn compare(
    label: &str,
    p: &CompactSurface,
    c: &Candidate,
    xyz: &[f32],
    cpu: &surface::SurfaceMesh,
) -> serde_json::Value {
    assert_eq!(xyz.len(), cpu.positions.len());
    assert_eq!(p.vertices as usize * 3, xyz.len());
    assert_eq!(p.run_table.len(), cpu.run_table.len());
    let mut max = 0.0f64;
    let mut squared = 0.0;
    let mut finite = 0usize;
    let mut enclosed = true;
    for i in 0..p.vertices {
        let a = point(xyz, i);
        let error = a.distance(point(&cpu.positions, i));
        max = max.max(error);
        squared += error * error;
        finite += usize::from(a.is_finite());
        for k in 0..3 {
            let v = xyz[i as usize * 3 + k];
            enclosed &= v >= f32::from_bits(c.status[k]) && v <= f32::from_bits(c.status[k + 4]);
        }
    }
    let mut new_collapse = 0;
    let mut old_collapse = 0;
    let mut candidate_collapse = 0;
    let mut candidate_zero_area = 0;
    let mut candidate_range_rejection = 0;
    let mut candidate_nonfinite_triangle = 0;
    let mut max_face_angle = 0.0f64;
    let mut cpu_normals = vec![Vec3::ZERO; p.vertices as usize];
    let mut gpu_normals = cpu_normals.clone();
    let mut topology = true;
    let mut canonical_index = 0usize;
    let mut caps = true;
    let mut max_radius = 0.0f64;
    let mut min_radius = f64::INFINITY;
    let mut small_max = 0.0f64;
    for run in &p.runs {
        for f in 0..run.index_count / 3 {
            let t = run.triangle(f, p.segments);
            let normal = |positions: &[f32]| {
                let [a, b, c] = t.map(|i| point(positions, i));
                (c - b).cross(a - b)
            };
            let a = normal(&cpu.positions);
            let b = normal(xyz);
            let old = admitted(a);
            let new = admitted(b);
            candidate_nonfinite_triangle += usize::from(!b.is_finite());
            candidate_zero_area += usize::from(b.is_finite() && b.length_squared() == 0.0);
            candidate_range_rejection +=
                usize::from(b.is_finite() && b.length_squared() > 0.0 && !new);
            old_collapse += usize::from(!old);
            candidate_collapse += usize::from(!new);
            new_collapse += usize::from(old && !new);
            if old {
                topology &= cpu.indices[canonical_index..canonical_index + 3] == t;
                canonical_index += 3;
            }
            if old && new {
                max_face_angle =
                    max_face_angle.max(a.normalized().dot(b.normalized()).clamp(-1.0, 1.0).acos());
            }
            for i in t {
                if old {
                    cpu_normals[i as usize] = cpu_normals[i as usize] + a;
                }
                if new {
                    gpu_normals[i as usize] = gpu_normals[i as usize] + b;
                }
            }
        }
        let bottom = run.base + run.rings * p.segments;
        caps &= xyz[bottom as usize * 3..(bottom as usize + 2) * 3]
            == cpu.positions[bottom as usize * 3..(bottom as usize + 2) * 3];
        for r in 0..run.rings {
            let start = (run.base + r * p.segments) as usize * 3;
            let end = start + p.segments as usize * 3;
            let a = surface::prepared::ring_radius(&cpu.positions[start..end]);
            let b = surface::prepared::ring_radius(&xyz[start..end]);
            max_radius = max_radius.max((a - b).abs() as f64);
            min_radius = min_radius.min(a as f64);
            if a < 0.001 {
                for i in start / 3..end / 3 {
                    small_max = small_max
                        .max(point(xyz, i as u32).distance(point(&cpu.positions, i as u32)));
                }
            }
        }
    }
    let mut max_normal_angle = 0.0f64;
    for (a, b) in cpu_normals.iter().zip(&gpu_normals) {
        if a.length_squared() > 0.0 && b.length_squared() > 0.0 {
            max_normal_angle =
                max_normal_angle.max(a.normalized().dot(b.normalized()).clamp(-1.0, 1.0).acos());
        }
    }
    let rms = (squared / (p.vertices.max(1) as f64)).sqrt();
    assert!(
        topology && caps && enclosed,
        "{label}: topology/caps/bounds"
    );
    assert_eq!(canonical_index, cpu.indices.len());
    serde_json::json!({"label":label,"vertices":p.vertices,"runs":p.runs.len(),"rings":p.rings.len(),"proceduralIndices":p.indices,"canonicalIndices":cpu.indices.len(),"topology":topology,"caps":caps,"enclosed":enclosed,"finite":finite,"gpuInvalid":c.status[3],"gpuCollapsed":c.status[7],"canonicalCollapsed":old_collapse,"candidateCollapsed":candidate_collapse,"candidateZeroArea":candidate_zero_area,"candidateRangeRejection":candidate_range_rejection,"candidateNonfiniteTriangle":candidate_nonfinite_triangle,"admissionParity":c.status[7] as usize==candidate_collapse,"newCollapsed":new_collapse,"maxMetres":max,"rmsMetres":rms,"smallBranchMaxMetres":small_max,"minRadiusMetres":min_radius,"maxRadiusMetres":max_radius,"maxFaceAngleRadians":max_face_angle,"maxVertexNormalAngleRadians":max_normal_angle,"numericPass":finite==p.vertices as usize&&c.status[3]==0&&c.status[7]==0&&candidate_collapse==0&&new_collapse==0&&max<=0.00005&&rms<=0.00001})
}
fn pass(g: &Generator) -> io::Pass {
    io::Pass::new(
        &g.gpu,
        include_str!("position_probe.wgsl").into(),
        &[true, true, true, false, false, false],
        &["emit", "validate", "finish"],
    )
}

#[test]
fn compact_position_edge_cases() {
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let g = Generator::new(&renderer).unwrap();
    let scopes = io::scope(&g.gpu);
    let shader = pass(&g);
    println!("POSITION_ADAPTER {:?}", g.gpu.adapter);
    for (label, count, offset, radius, lobes, twist) in [
        ("minimal", 2, 0.0, 0.1, 0, 0.0),
        ("curved_twisted", 13, 0.0, 0.1, 5, 1.5),
        ("small_branch", 8, 25.0, 0.0001, 5, 1.5),
        ("large_coordinate", 3, 100000.0, 1.0, 0, 0.0),
        ("degenerate_tip", 3, 100000.0, 1.0, 0, 0.0),
    ] {
        let mut tree = super::tree(count);
        for n in &mut tree.nodes {
            n.position.x += offset;
            n.position.z += offset;
            n.radius = radius;
            n.start_radius = radius;
        }
        if label == "degenerate_tip" {
            tree.nodes.last_mut().unwrap().radius = 0.001;
        }
        let params = SurfaceParams {
            radial_segments: 12,
            lobes,
            twist_rate: twist,
            flare_depth: 0.0,
            flare_radius: 1.0,
            ..Default::default()
        };
        let p = compact::prepare(&tree, 24.0, &params).unwrap();
        let candidate = execute(&g, &shader, &p);
        let xyz = read(&g, &candidate, p.vertices);
        let cpu = surface::build(&tree, 24.0, &params).unwrap();
        let report = compare(label, &p, &candidate, &xyz, &cpu);
        println!("POSITION_EDGE {report}");
        assert_eq!(
            candidate.status[7] as u64,
            report["candidateCollapsed"].as_u64().unwrap()
        );
        let again = execute(&g, &shader, &p);
        assert_eq!(xyz, read(&g, &again, p.vertices));
        assert_eq!(candidate.status, again.status);
    }
    for tree in [
        Tree::default(),
        Tree {
            nodes: vec![Node::root()],
            ..Tree::default()
        },
    ] {
        let p = compact::prepare(&tree, 1.0, &SurfaceParams::default()).unwrap();
        assert_eq!(p.vertices, 0);
        let c = execute(&g, &shader, &p);
        assert_eq!((c.status[3], c.status[7]), (0, 0));
    }
    let mut invalid = super::tree(2);
    invalid.nodes[0].position.x = 1e40;
    assert!(compact::prepare(&invalid, 1.0, &SurfaceParams::default()).is_err());
    assert!(compact::prepare(&super::tree(2), 0.0, &SurfaceParams::default()).is_err());
    let mut p = compact::prepare(&super::tree(2), 1.0, &SurfaceParams::default()).unwrap();
    p.rings[0][0] = f32::INFINITY.to_bits();
    assert!(execute(&g, &shader, &p).status[3] > 0);
    let mut p = compact::prepare(&super::tree(2), 1.0, &SurfaceParams::default()).unwrap();
    for ring in &mut p.rings {
        ring[0] = 1e30_f32.to_bits();
        ring[1] = (f32::from_bits(ring[1]) * 1e30).to_bits();
        ring[3] = 1e30_f32.to_bits();
    }
    let arithmetic = execute(&g, &shader, &p);
    assert!(read(&g, &arithmetic, p.vertices)
        .iter()
        .all(|v| v.is_finite()));
    assert!(
        arithmetic.status[3] > 0,
        "nonfinite triangle arithmetic must be rejected"
    );
    pollster::block_on(io::errors(&g.gpu, scopes)).unwrap();
}

#[test]
#[ignore = "isolated four-fixture release feasibility measurement"]
fn compact_position_mature_measurement() {
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let g = Generator::new(&renderer).unwrap();
    let scopes = io::scope(&g.gpu);
    let shader = pass(&g);
    println!("POSITION_ADAPTER {:?}", g.gpu.adapter);
    for species in ["oregon-white-oak", "norway-spruce"] {
        for seed in [1, 7] {
            let mut f = telperion_core::presets::Preset::from_id(species)
                .unwrap()
                .parameters();
            f.skeleton.seed = seed;
            let tree = branching::generate(&f.skeleton, f.radii).unwrap().tree;
            let height = f.skeleton.envelope.height;
            let mut times = Vec::new();
            let mut reference_times = Vec::new();
            for sample in 0..if std::env::var_os("FN91_VALIDATION_ONLY").is_some() {
                0
            } else {
                4
            } {
                let start = Instant::now();
                let canonical = surface::prepared::prepare(&tree, height, &f.surface)
                    .unwrap()
                    .unwrap();
                let canonical_ms = millis(start);
                drop(canonical);
                let start = Instant::now();
                let p = compact::prepare(&tree, height, &f.surface).unwrap();
                let prep_ms = millis(start);
                let c = execute(&g, &shader, &p);
                let total = prep_ms + c.upload_ms + c.completion_ms;
                println!(
                    "POSITION_TIME {}",
                    serde_json::json!({"species":species,"seed":seed,"sample":sample,"canonicalMs":canonical_ms,"compactMs":prep_ms,"uploadDispatchMs":c.upload_ms,"completionMs":c.completion_ms,"totalMs":total,"gpuBytes":c.gpu_bytes,"cpuUploadBytes":c.cpu_bytes,"retainedPositionBytes":c.positions.size()})
                );
                if sample > 0 {
                    times.push(total);
                    reference_times.push(canonical_ms);
                }
            }
            let p = compact::prepare(&tree, height, &f.surface).unwrap();
            let c = execute(&g, &shader, &p);
            let xyz = read(&g, &c, p.vertices);
            let again = execute(&g, &shader, &p);
            assert_eq!(xyz, read(&g, &again, p.vertices));
            assert_eq!(c.status, again.status);
            let cpu = surface::build(&tree, height, &f.surface).unwrap();
            let mut report = compare(&format!("{species}-{seed}"), &p, &c, &xyz, &cpu);
            times.sort_by(f64::total_cmp);
            reference_times.sort_by(f64::total_cmp);
            if !times.is_empty() {
                report["medianMs"] = times[1].into();
                report["canonicalMedianMs"] = reference_times[1].into();
                report["speedup"] = (reference_times[1] / times[1]).into();
            }
            report["repeatable"] = true.into();
            println!("POSITION_RESULT {report}");
        }
    }
    pollster::block_on(io::errors(&g.gpu, scopes)).unwrap();
}

#[path = "position_tests.rs"]
mod production;
