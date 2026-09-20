use super::*;

fn production_contract(
    g: &Generator,
    tree: &Tree,
    params: &SurfaceParams,
    height: f64,
    label: &str,
) {
    let p = compact::prepare(tree, height, params).unwrap();
    assert!(p.qualified(), "{label} must qualify");
    let mut metrics = Metrics::default();
    let uploaded = pollster::block_on(g.emit_positions(
        compact::prepare(tree, height, params).unwrap(),
        &mut metrics,
    ))
    .unwrap()
    .unwrap();
    let bounds = uploaded.bounds;
    let position_buffer = uploaded.positions.clone();
    let wood = pollster::block_on(g.expand_uploaded_wood(uploaded, &mut metrics))
        .unwrap()
        .unwrap();
    assert_eq!(wood.positions.buffer(), &position_buffer);
    let xyz = super::super::floats(g, &wood.positions);
    let again = pollster::block_on(g.emit_positions(
        compact::prepare(tree, height, params).unwrap(),
        &mut metrics,
    ))
    .unwrap()
    .unwrap();
    assert_eq!(
        io::read(&g.gpu, &again.positions, p.vertices as u64 * 12).unwrap(),
        bytemuck::cast_slice::<f32, u8>(&xyz)
    );
    let cpu = surface::build(tree, height, params).unwrap();
    let mut status = [0u32; 8];
    if let Some(b) = bounds {
        for (i, v) in [b.min.x, b.min.y, b.min.z].into_iter().enumerate() {
            status[i] = (v as f32).to_bits();
        }
        for (i, v) in [b.max.x, b.max.y, b.max.z].into_iter().enumerate() {
            status[i + 4] = (v as f32).to_bits();
        }
    }
    let candidate = Candidate {
        positions: position_buffer,
        status,
        upload_ms: 0.0,
        completion_ms: 0.0,
        gpu_bytes: 0,
        cpu_bytes: 0,
    };
    let report = compare(label, &p, &candidate, &xyz, &cpu);
    assert_eq!(report["numericPass"], true, "{report}");
    let radii = super::super::floats(g, &wood.radii);
    let normals = super::super::floats(g, &wood.normals);
    let mut max_radius_absolute = 0.0_f32;
    let mut max_radius_relative = 0.0_f32;
    for run in &p.runs {
        for ring in 0..run.rings {
            let base = (run.base + ring * p.segments) as usize;
            let actual =
                surface::prepared::ring_radius(&xyz[base * 3..(base + p.segments as usize) * 3]);
            for &r in &radii[base..base + p.segments as usize] {
                max_radius_absolute = max_radius_absolute.max((r - actual).abs());
                max_radius_relative = max_radius_relative.max((r - actual).abs() / actual);
            }
        }
    }
    assert!(
        max_radius_absolute <= 0.000001 && max_radius_relative <= 0.000002,
        "{label}: radius {max_radius_absolute} relative {max_radius_relative}"
    );
    let mut accumulated = vec![Vec3::ZERO; p.vertices as usize];
    for run in &p.runs {
        for face in 0..run.index_count / 3 {
            let t = run.triangle(face, p.segments);
            let [a, b, c] = t.map(|i| point(&xyz, i));
            for i in t {
                accumulated[i as usize] = accumulated[i as usize] + (c - b).cross(a - b);
            }
        }
    }
    let mut max_angle = 0.0_f64;
    for (n, expected) in normals.chunks_exact(3).zip(accumulated) {
        let n = Vec3::new(n[0] as f64, n[1] as f64, n[2] as f64);
        assert!(n.is_finite());
        max_angle = max_angle.max(
            n.normalized()
                .dot(expected.normalized())
                .clamp(-1.0, 1.0)
                .acos(),
        );
    }
    assert!(
        max_angle < 0.001,
        "{label}: actual-position normal {max_angle}"
    );
    println!(
        "PRODUCTION_POSITION {}",
        serde_json::json!({"geometry":report,"maxRadiusAbsolute":max_radius_absolute,"maxRadiusRelative":max_radius_relative,"maxActualNormalAngle":max_angle})
    );
}

#[test]
fn production_positions_admit_geometry_and_reject_unsupported_inputs() {
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let g = Generator::new(&renderer).unwrap();
    let scopes = io::scope(&g.gpu);
    let params = SurfaceParams {
        lobes: 0,
        flare_depth: 0.0,
        ..Default::default()
    };
    for (label, count, offset, radius, accepted) in [
        ("minimal", 2, 0.0, 0.1, true),
        ("curved", 13, 0.0, 0.1, true),
        ("boundary", 2, 63.0, 64.0 / 131072.0, true),
        ("tiny_far", 2, 25.0, 0.0001, false),
        ("far", 2, 100000.0, 1.0, false),
    ] {
        let mut tree = super::super::tree(count);
        for node in &mut tree.nodes {
            node.position.x += offset;
            node.radius = radius;
            node.start_radius = radius;
        }
        if accepted {
            production_contract(&g, &tree, &params, 24.0, label);
        } else {
            let mut metrics = Metrics::default();
            assert!(pollster::block_on(g.emit_positions(
                compact::prepare(&tree, 24.0, &params).unwrap(),
                &mut metrics
            ))
            .unwrap()
            .is_none());
            assert_eq!(metrics.position_fallback, Some("precision domain"));
            assert!(surface::prepared::prepare(&tree, 24.0, &params)
                .unwrap()
                .is_some());
        }
    }
    for tree in [
        Tree::default(),
        Tree {
            nodes: vec![Node::root()],
            ..Default::default()
        },
    ] {
        let w = pollster::block_on(g.emit_positions(
            compact::prepare(&tree, 1.0, &params).unwrap(),
            &mut Metrics::default(),
        ))
        .unwrap()
        .unwrap();
        assert_eq!((w.vertices, w.index_count, w.bounds), (0, 0, None));
    }
    let mut collapsed = compact::prepare(&super::super::tree(2), 1.0, &params).unwrap();
    let centre = collapsed.rings[0][0..12].to_vec();
    for r in &mut collapsed.rings {
        r[0..12].copy_from_slice(&centre);
    }
    let mut metrics = Metrics::default();
    assert!(
        pollster::block_on(g.emit_positions(collapsed, &mut metrics))
            .unwrap()
            .is_none()
    );
    assert_eq!(
        metrics.position_fallback,
        Some("geometry or normal admission")
    );
    pollster::block_on(io::errors(&g.gpu, scopes)).unwrap();
    g.gpu.device.destroy();
    let scopes = io::scope(&g.gpu);
    let result = pollster::block_on(g.emit_positions(
        compact::prepare(&super::super::tree(2), 1.0, &params).unwrap(),
        &mut metrics,
    ));
    let error = pollster::block_on(io::errors(&g.gpu, scopes));
    assert!(result.is_err() || error.is_err());
}

#[test]
#[ignore = "isolated production four-fixture geometry screening"]
fn production_position_mature_contracts() {
    let gpu = pollster::block_on(Gpu::request(None)).unwrap();
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let g = Generator::new(&renderer).unwrap();
    let scopes = io::scope(&g.gpu);
    for species in ["oregon-white-oak", "norway-spruce"] {
        for seed in [1, 7] {
            let mut f = telperion_core::presets::Preset::from_id(species)
                .unwrap()
                .parameters();
            f.skeleton.seed = seed;
            let tree = branching::generate(&f.skeleton, f.radii).unwrap().tree;
            production_contract(
                &g,
                &tree,
                &f.surface,
                f.skeleton.envelope.height,
                &format!("{species}-{seed}"),
            );
        }
    }
    pollster::block_on(io::errors(&g.gpu, scopes)).unwrap();
}
