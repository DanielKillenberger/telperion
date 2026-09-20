use super::*;
use telperion_core::{
    math::Vec3,
    surface::SurfaceParams,
    tree::{Node, Tree},
};
fn tree(rings: u32) -> Tree {
    Tree {
        nodes: (0..rings)
            .map(|i| Node {
                position: Vec3::new(
                    0.02 * (i as f64).sin(),
                    i as f64 * 0.3,
                    0.01 * (i as f64).cos(),
                ),
                parent: (i > 0).then(|| i - 1),
                radius: 0.1,
                start_radius: 0.1,
                ..Node::root()
            })
            .collect(),
        ..Tree::default()
    }
}
fn floats(g: &Generator, b: &Held) -> Vec<f32> {
    io::read(&g.gpu, b.buffer(), b.region().used())
        .unwrap()
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
        .collect()
}
#[test]
fn resident_wood_matches_cpu_and_remains_writable_after_adoption() {
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let g = Generator::new(&renderer).unwrap();
    for rings in [2, 4, 1025] {
        let params = SurfaceParams::default();
        let tree = tree(rings);
        let cpu = surface::build(&tree, 400.0, &params).unwrap();
        let p = surface::prepared::prepare(&tree, 400.0, &params)
            .unwrap()
            .unwrap();
        let scopes = io::scope(&g.gpu);
        let w = pollster::block_on(g.expand_wood(p, &mut Metrics::default()))
            .unwrap()
            .unwrap();
        pollster::block_on(io::errors(&g.gpu, scopes)).unwrap();
        assert_eq!(floats(&g, &w.positions), cpu.positions);
        assert_eq!(floats(&g, &w.coords), cpu.coords);
        let indices: Vec<u32> = io::read(&g.gpu, w.indices.buffer(), w.indices.region().used())
            .unwrap()
            .chunks_exact(4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
            .collect();
        assert_eq!(indices, cpu.indices);
        assert_eq!(floats(&g, &w.radii), crate::wood::radius::radii(&cpu));
        assert_eq!(w.bounds, cpu.bounds);
        assert_eq!(w.runs, cpu.run_table);
        for (gpu, cpu) in floats(&g, &w.normals)
            .chunks_exact(3)
            .zip(cpu.normals.chunks_exact(3))
        {
            let a = Vec3::new(gpu[0] as f64, gpu[1] as f64, gpu[2] as f64);
            let b = Vec3::new(cpu[0] as f64, cpu[1] as f64, cpu[2] as f64);
            assert!(a.is_finite() && (a.length() - 1.0).abs() < 1e-6);
            assert!(a.normalized().dot(b.normalized()).clamp(-1.0, 1.0).acos() < 0.001);
        }
        let scopes = io::scope(&g.gpu);
        renderer.wood.submit_resident(
            &g.gpu,
            w.positions,
            w.normals,
            w.coords,
            w.indices,
            w.radii,
            w.index_count,
            w.runs,
        );
        renderer.wood.submit(&g.gpu, &cpu);
        let larger = surface::build(&self::tree(rings + 1), 400.0, &params).unwrap();
        renderer.wood.submit(&g.gpu, &larger);
        pollster::block_on(io::errors(&g.gpu, scopes)).unwrap();
    }
}

#[test]
fn multirow_empty_and_unusable_normals_have_explicit_outcomes() {
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let g = Generator::new(&renderer).unwrap();
    let params = SurfaceParams {
        radial_segments: 3,
        lobes: 0,
        flare_depth: 0.0,
        ..SurfaceParams::default()
    };
    let source = surface::prepared::prepare(&tree(2), 1.0, &params)
        .unwrap()
        .unwrap();
    let mut p = surface::prepared::PreparedSurface::default();
    p.segments = source.segments;
    p.angles = source.angles.clone();
    for _ in 0..65536 {
        let mut run = source.runs[0];
        run.base = p.positions.len() as u32 / 3;
        run.first_index = p.index_count;
        run.ring_start = p.rings.len() as u32;
        p.index_count += run.index_count;
        p.runs.push(run);
        p.positions.extend(&source.positions);
        p.rings.extend(&source.rings);
    }
    let scopes = io::scope(&g.gpu);
    let w = pollster::block_on(g.expand_wood(p, &mut Metrics::default()))
        .unwrap()
        .unwrap();
    let indices = io::read(&g.gpu, w.indices.buffer(), w.indices.region().used()).unwrap();
    let tail: Vec<u32> = indices[indices.len() - source.index_count as usize * 4..]
        .chunks_exact(4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    let mut last = source.runs[0];
    last.base = 65535 * (source.positions.len() as u32 / 3);
    assert_eq!(
        tail,
        (0..last.index_count / 3)
            .flat_map(|f| last.triangle(f, source.segments))
            .collect::<Vec<_>>()
    );
    let empty = surface::prepared::prepare(&Tree::default(), 1.0, &params)
        .unwrap()
        .unwrap();
    let empty = pollster::block_on(g.expand_wood(empty, &mut Metrics::default()))
        .unwrap()
        .unwrap();
    assert_eq!((empty.vertices, empty.index_count), (0, 0));
    let mut bad = source;
    bad.positions.fill(0.0);
    let mut metrics = Metrics::default();
    assert!(pollster::block_on(g.expand_wood(bad, &mut metrics))
        .unwrap()
        .is_none());
    assert_eq!(metrics.wood_fallback, Some("GPU normal unusable"));
    pollster::block_on(io::errors(&g.gpu, scopes)).unwrap();
}

fn digest(mesh: &surface::SurfaceMesh) -> String {
    let mut hash = 14695981039346656037_u64;
    let mut put = |bytes: &[u8]| {
        for &byte in bytes {
            hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
        }
    };
    // Length-prefix every array; fixed little-endian IEEE float bits, u32 indices,
    // u64 counts; bounds presence byte then six f64s; ordered run records.
    for values in [&mesh.positions, &mesh.normals, &mesh.coords] {
        put(&(values.len() as u64).to_le_bytes());
        for v in values {
            put(&v.to_le_bytes());
        }
    }
    put(&(mesh.indices.len() as u64).to_le_bytes());
    for v in &mesh.indices {
        put(&v.to_le_bytes());
    }
    put(&[u8::from(mesh.bounds.is_some())]);
    if let Some(b) = mesh.bounds {
        for v in [b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z] {
            put(&v.to_le_bytes());
        }
    }
    for v in [mesh.runs, mesh.dropped, mesh.run_table.len()] {
        put(&(v as u64).to_le_bytes());
    }
    for r in &mesh.run_table {
        put(&r.first_index.to_le_bytes());
        put(&r.index_count.to_le_bytes());
        put(&r.largest_radius.to_le_bytes());
    }
    format!("{hash:016x}")
}

#[test]
fn measured_specimens_keep_all_cpu_fields_and_gpu_surface_contracts() {
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let g = Generator::new(&renderer).unwrap();
    for species in ["oregon-white-oak", "norway-spruce"] {
        for seed in [1, 7] {
            let mut family = telperion_core::presets::Preset::from_id(species)
                .unwrap()
                .parameters();
            family.skeleton.seed = seed;
            let tree = branching::generate(&family.skeleton, family.radii)
                .unwrap()
                .tree;
            let cpu =
                surface::build(&tree, family.skeleton.envelope.height, &family.surface).unwrap();
            let p =
                surface::prepared::prepare(&tree, family.skeleton.envelope.height, &family.surface)
                    .unwrap()
                    .unwrap();
            let scopes = io::scope(&g.gpu);
            let w = pollster::block_on(g.expand_wood(p, &mut Metrics::default()))
                .unwrap()
                .unwrap();
            assert_eq!(floats(&g, &w.positions), cpu.positions);
            assert_eq!(floats(&g, &w.coords), cpu.coords);
            assert_eq!(floats(&g, &w.radii), crate::wood::radius::radii(&cpu));
            let indices = io::read(&g.gpu, w.indices.buffer(), w.indices.region().used()).unwrap();
            assert_eq!(indices, bytemuck::cast_slice::<u32, u8>(&cpu.indices));
            assert_eq!(w.bounds, cpu.bounds);
            assert_eq!(w.runs, cpu.run_table);
            let mut max_angle = 0.0_f64;
            for (a, b) in floats(&g, &w.normals)
                .chunks_exact(3)
                .zip(cpu.normals.chunks_exact(3))
            {
                let a = Vec3::new(a[0] as f64, a[1] as f64, a[2] as f64);
                let b = Vec3::new(b[0] as f64, b[1] as f64, b[2] as f64);
                assert!(a.is_finite() && (a.length() - 1.0).abs() < 1e-6);
                max_angle =
                    max_angle.max(a.normalized().dot(b.normalized()).clamp(-1.0, 1.0).acos());
            }
            assert!(max_angle < 0.001, "{species} {seed}: {max_angle}");
            println!(
                "WOOD_PARITY species={species} seed={seed} cpu_hash={} max_angle={max_angle}",
                digest(&cpu)
            );
            pollster::block_on(io::errors(&g.gpu, scopes)).unwrap();
        }
    }
}

#[test]
fn compute_limits_and_device_failure_never_return_partial_wood() {
    let limits = wgpu::Limits {
        max_buffer_size: 1024,
        max_storage_buffer_binding_size: 128,
        max_compute_workgroups_per_dimension: 2,
        ..Default::default()
    };
    assert!(wood::compute_fits(&limits, &[128], 4));
    assert!(!wood::compute_fits(&limits, &[129], 4));
    assert!(!wood::compute_fits(&limits, &[128], 5));
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let g = Generator::new(&renderer).unwrap();
    let p = surface::prepared::prepare(&tree(2), 1.0, &SurfaceParams::default())
        .unwrap()
        .unwrap();
    g.gpu.device.destroy();
    let scopes = io::scope(&g.gpu);
    let result = pollster::block_on(g.expand_wood(p, &mut Metrics::default()));
    let error = pollster::block_on(io::errors(&g.gpu, scopes));
    assert!(result.is_err() || error.is_err());
}

#[path = "position_probe.rs"]
mod position_probe;
