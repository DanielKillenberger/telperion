use super::*;
use telperion_core::{
    foliage::{Reference, TwigPlacement},
    math::Vec3,
    pipeline::executor::{self, Expansion, LeafInput},
    tree::{Node, NodeKind, Tree},
    Family,
};

/// A hand-built tree prepared through the interface, the family's twig rows
/// set to `twig`.
fn expansion(tree: &Tree, f: &Family, twig: TwigPlacement) -> Expansion {
    let mut f = f.clone();
    f.skeleton.twigs.twig.internode_length = twig.internode_length;
    f.skeleton.twigs.twig.stations_per_internode = twig.stations_per_internode;
    executor::expand(tree.clone(), &f).unwrap()
}

/// The rows the GPU packs, read off a test family.
fn leaves(f: &Family) -> LeafInput {
    LeafInput {
        envelope: f.skeleton.envelope,
        canopy: f.canopy,
        seed: f.skeleton.seed,
        shell_depth: f.shell_depth,
    }
}
fn fixture(angle: f64) -> Tree {
    let mut root = Node::root();
    root.radius = 0.03;
    root.start_radius = 0.03;
    let mut nodes = vec![root];
    for (i, point) in [
        Vec3::new(0.0, 0.5, 0.0),
        Vec3::new(0.12, 1.0, 0.1),
        Vec3::new(-0.05, 1.5, 0.2),
    ]
    .into_iter()
    .enumerate()
    {
        let mut n = Node::root();
        n.parent = Some(i as u32);
        n.position = point.rotate(Vec3::new(0.0, 1.0, 0.0), angle);
        n.radius = 0.02;
        n.start_radius = 0.03;
        n.base_radius = 0.03;
        n.kind = NodeKind::Twig;
        n.branch = 1;
        nodes.push(n);
    }
    Tree {
        nodes,
        crossover: 1,
        ..Default::default()
    }
}
#[test]
fn compact_gpu_foliage_is_repeatable_seated_and_bounded() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let generator = Generator::new(&renderer).unwrap();
    let mut f = Family::default();
    f.skeleton.envelope.height = 2.0;
    f.skeleton.envelope.spread = 0.5;
    f.shell_depth = 1.0;
    f.canopy.surface_contact = 1.0;
    f.canopy.short_shoot_spacing = 0.0;
    f.canopy.limb_clumping = 0.0;
    let twig = TwigPlacement {
        internode_length: 0.07,
        stations_per_internode: 3,
    };
    let reference = Reference::spanning(Vec3::new(-2.0, -1.0, -2.0), Vec3::new(2.0, 3.0, 2.0));
    let element = executor::element(f.element).unwrap();
    for (angle, shell) in [(0.0, 1.0), (0.73, 0.01)] {
        f.shell_depth = shell;
        let tree = fixture(angle);
        let x = expansion(&tree, &f, twig);
        let prepare = || expansion(&tree, &f, twig).stations().unwrap().unwrap();
        let mut metrics = Metrics::default();
        let first = generator
            .compute(
                prepare(),
                &leaves(&f),
                twig,
                &element,
                reference,
                &mut metrics,
            )
            .unwrap();
        let second = generator
            .compute(
                prepare(),
                &leaves(&f),
                twig,
                &element,
                reference,
                &mut metrics,
            )
            .unwrap();
        let a = io::read(
            &generator.gpu,
            first.leaves.buffer(),
            u64::from(first.count) * 12,
        )
        .unwrap();
        let b = io::read(
            &generator.gpu,
            second.leaves.buffer(),
            u64::from(second.count) * 12,
        )
        .unwrap();
        assert_eq!(a, b);
        let shared = x.prepared_with_contacts().unwrap().unwrap();
        let p = x.shared_stations(&shared).unwrap().unwrap();
        let (segments, count, ring_size) = (p.segments, p.count, p.ring_size);
        let uploaded = generator
            .upload_wood(shared.into_surface(), &mut metrics)
            .unwrap()
            .unwrap();
        let positions = uploaded.positions.clone();
        let p = foliage::prepared::PreparedStations {
            segments,
            count,
            ring_size,
            rings: std::borrow::Cow::Borrowed(&uploaded.positions),
        };
        let shared_leaves = pollster::block_on(generator.compute_buffer_async(
            p,
            &leaves(&f),
            twig,
            &element,
            reference,
            &mut metrics,
        ))
        .unwrap();
        assert_eq!(
            a,
            io::read(
                &generator.gpu,
                shared_leaves.leaves.buffer(),
                u64::from(shared_leaves.count) * 12
            )
            .unwrap()
        );
        let wood = pollster::block_on(generator.expand_uploaded_wood(uploaded, &mut metrics))
            .unwrap()
            .unwrap();
        assert_eq!(wood.positions.buffer(), &positions);

        // The pipeline's CPU leaves on the same tree, and before the cull
        // (a shell of one keeps every leaf).
        let cpu = x.mesh().unwrap().foliage.instances;
        let mut whole = f.clone();
        whole.shell_depth = 1.0;
        let placed = expansion(&tree, &whole, twig)
            .mesh()
            .unwrap()
            .foliage
            .instances;
        if shell < 1.0 {
            assert!(!cpu.is_empty() && cpu.len() < placed.len());
        }
        let mut actual = Instances::new(reference);
        actual.leaves = a
            .chunks_exact(12)
            .map(|b| {
                [
                    u32::from_ne_bytes(b[0..4].try_into().unwrap()),
                    u32::from_ne_bytes(b[4..8].try_into().unwrap()),
                    u32::from_ne_bytes(b[8..12].try_into().unwrap()),
                ]
            })
            .collect();
        actual.validate().unwrap();
        assert_eq!(actual.len(), cpu.len());
        // Each position quantizes to its own 16-bit box: the GPU's to the
        // fixture's, the CPU reference's to the family's. Two full code steps
        // of each plus 32 f32 epsilons at fixture scale bound separate arithmetic.
        let steps = reference.step().length() + cpu.reference.step().length();
        let tolerance = steps * 2.0 + 32.0 * f32::EPSILON as f64 * 4.0;
        for i in 0..cpu.len() {
            let a = actual.matrix(i);
            let b = cpu.matrix(i);
            for column in 0..3 {
                let direction = |m: &[f32; 16]| {
                    Vec3::new(
                        m[column * 4] as f64,
                        m[column * 4 + 1] as f64,
                        m[column * 4 + 2] as f64,
                    )
                };
                let x = direction(&a);
                let y = direction(&b);
                assert!(
                    x.normalized().distance(y.normalized()) < 0.005,
                    "random orientation at surviving ordinal {i}"
                );
                assert!(
                    (x.length() - y.length()).abs() <= y.length() / 1024.0,
                    "random scale at surviving ordinal {i}"
                );
            }
            assert!(
                actual.position(i).distance(cpu.position(i)) <= tolerance,
                "station {i}: {:?} vs {:?}",
                actual.position(i),
                cpu.position(i)
            );
        }
        let exact = actual.bounds(&element).unwrap().unwrap();
        let bounds = first.full.unwrap();
        let tol = 64.0 * f32::EPSILON as f64 * 4.0;
        assert!(exact.min.distance(bounds.min) < tol && exact.max.distance(bounds.max) < tol);
        let mass = io::read(
            &generator.gpu,
            first.masses.buffer(),
            first.masses.region().used(),
        )
        .unwrap();
        let expected = crate::mass::grid(&actual, first.crown);
        let observed: Vec<f32> = mass
            .chunks_exact(4)
            .map(|b| f32::from_ne_bytes(b.try_into().unwrap()))
            .collect();
        assert_eq!(expected.len(), observed.len());
        for (a, b) in expected.iter().zip(&observed) {
            assert!((a - b).abs() < 1e-5, "mass {a} vs {b}");
        }
    }
}
#[test]
fn preparation_capabilities_and_failures_are_explicit() {
    let tree = fixture(0.0);
    let mut f = Family::default();
    let twig = TwigPlacement::default();
    f.canopy.short_shoot_spacing = 0.1;
    assert!(expansion(&tree, &f, twig).stations().unwrap().is_none());
    f.canopy.size = f64::NAN;
    assert!(expansion(&tree, &f, twig).stations().is_err());
    f.canopy.size = 1.0;
    f.canopy.short_shoot_spacing = 0.0;
    f.canopy.max_instances = 1;
    assert!(expansion(&tree, &f, twig).stations().is_err());
    f.canopy.size = 0.0;
    assert_eq!(
        expansion(&tree, &f, twig)
            .stations()
            .unwrap()
            .unwrap()
            .count,
        0
    );
}

#[test]
fn foreign_prepared_result_preserves_the_live_tree() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let source = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let mut destination = Renderer::new(gpu, crate::STILL_FORMAT);
    let f = Family::default();
    let tree = fixture(0.0);
    let mesh = executor::expand(tree.clone(), &f).unwrap().mesh().unwrap();
    destination.submit(&mesh).unwrap();
    let previous = destination.bounds();
    let previous_region = destination.foliage_region();
    let generator = Generator::new(&source).unwrap();
    let x = executor::expand(tree.clone(), &f).unwrap();
    let compact = x.prepared_wood().unwrap().unwrap();
    let wood = pollster::block_on(generator.expand_wood(compact, &mut Metrics::default())).unwrap();
    assert!(wood.is_some());
    let previous_wood = destination.wood.regions();
    let prepared = Prepared {
        wood,
        identity: source.identity.clone(),
        mesh,
        resident: None,
        backend: Backend::CpuFallback,
        metrics: Metrics::default(),
    };
    assert!(matches!(
        destination.submit_prepared(prepared),
        Err(crate::RenderError::Generation(
            telperion_core::Error::InvalidInput("generation renderer mismatch")
        ))
    ));
    assert_eq!(destination.wood.regions(), previous_wood);
    assert_eq!(destination.bounds(), previous);
    assert_eq!(destination.foliage_region(), previous_region);
}

#[test]
fn huge_phases_fall_back_but_supported_large_ordinals_keep_orientation() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let generator = Generator::new(&renderer).unwrap();
    let tree = fixture(0.0);
    let mut f = Family::default();
    f.canopy.surface_contact = 0.0;
    f.canopy.short_shoot_spacing = 0.0;
    f.canopy.limb_clumping = 0.0;
    f.canopy.divergence = 1e9;
    let tiny = TwigPlacement {
        internode_length: 1e-6,
        stations_per_internode: 1,
    };
    assert!(expansion(&tree, &f, tiny).stations().unwrap().is_none());
    f.canopy.divergence = 137.5;
    f.canopy.scatter = 0.0;
    f.canopy.outward = 0.0;
    f.canopy.upward = 0.0;
    f.canopy.forward_lean = 0.0;
    f.canopy.lean_rise = 0.0;
    f.shell_depth = 1.0;
    let twig = TwigPlacement {
        internode_length: 0.07,
        stations_per_internode: 3,
    };
    let mut p = expansion(&tree, &f, twig).stations().unwrap().unwrap();
    p.segments.truncate(1);
    p.count = 256;
    let segment = &mut p.segments[0];
    segment.count = 256;
    segment.first = 0;
    segment.run_station = 3_000_000;
    segment.span = 0.0;
    let base =
        (segment.run_station / 3) as f64 * f.canopy.divergence * std::f64::consts::PI / 180.0;
    segment.phase = [base.sin(), base.cos()];
    let frame = segment.frame;
    let reference = Reference::spanning(Vec3::new(-2.0, -1.0, -2.0), Vec3::new(2.0, 3.0, 2.0));
    let element = executor::element(f.element).unwrap();
    let output = generator
        .compute(
            p,
            &leaves(&f),
            twig,
            &element,
            reference,
            &mut Metrics::default(),
        )
        .unwrap();
    let bytes = io::read(
        &generator.gpu,
        output.leaves.buffer(),
        u64::from(output.count) * 12,
    )
    .unwrap();
    assert_eq!(output.count, 256);
    for (i, b) in bytes.chunks_exact(12).enumerate() {
        let leaf = [
            u32::from_ne_bytes(b[0..4].try_into().unwrap()),
            u32::from_ne_bytes(b[4..8].try_into().unwrap()),
            u32::from_ne_bytes(b[8..12].try_into().unwrap()),
        ];
        let matrix = reference.unpack(leaf);
        let axis = Vec3::new(matrix[4] as f64, matrix[5] as f64, matrix[6] as f64).normalized();
        let k = 3_000_000 + i;
        let turn = (k / 3) as f64 * f.canopy.divergence * std::f64::consts::PI / 180.0
            + (k % 3) as f64 * std::f64::consts::TAU / 3.0;
        let expected = frame[1] * turn.cos() + frame[2] * turn.sin();
        assert!(axis.distance(expected) < 0.004, "ordinal {k}");
    }
}

#[test]
fn gpu_extrema_order_preserves_signed_zero_and_finite_signs() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let source = format!(
        "{}\n{}",
        include_str!("order.wgsl"),
        r#"
        @group(0) @binding(1) var<storage,read_write> out:array<u32>;
        @compute @workgroup_size(1) fn probe() {
            out[0]=ordered(-3.402823466e38);out[1]=ordered(-1.0);
            out[2]=ordered(bitcast<f32>(0x80000000u));out[3]=ordered(0.0);
            out[4]=ordered(1.0);out[5]=ordered(3.402823466e38);
        }
    "#
    );
    let pass = io::Pass::new(&gpu, source, &[false], &["probe"]);
    let config = io::buffer(
        &gpu,
        "order probe config",
        16,
        wgpu::BufferUsages::UNIFORM,
        &[],
    )
    .unwrap();
    let out = io::buffer(
        &gpu,
        "order probe",
        24,
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        &[],
    )
    .unwrap();
    let bind = pass.bind(&gpu, &[&config, &out]);
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    pass.run(&mut encoder, &bind, 0, 1);
    gpu.queue.submit([encoder.finish()]);
    let bytes = io::read(&gpu, &out, 24).unwrap();
    let words: Vec<u32> = bytes
        .chunks_exact(4)
        .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
        .collect();
    assert!(words.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(words[2], 0x7fffffff);
    assert_eq!(words[3], 0x80000000);
    let values: Vec<f32> = words
        .iter()
        .map(|&word| {
            f32::from_bits(if word & 0x80000000 != 0 {
                word ^ 0x80000000
            } else {
                !word
            })
        })
        .collect();
    assert!(values.iter().all(|v| v.is_finite()));
    assert_eq!(values[2].to_bits(), (-0.0f32).to_bits());
    assert_eq!(values[3].to_bits(), 0.0f32.to_bits());
}

#[test]
fn compact_positions_seat_contacts_on_the_rendered_surface() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let g = Generator::new(&renderer).unwrap();
    let scopes = io::scope(&g.gpu);
    let tree = fixture(0.73);
    let mut f = Family::default();
    f.surface.lobes = 0;
    f.skeleton.envelope.height = 2.0;
    f.skeleton.envelope.spread = 0.5;
    f.shell_depth = 1.0;
    f.canopy.surface_contact = 1.0;
    f.canopy.short_shoot_spacing = 0.0;
    f.canopy.limb_clumping = 0.0;
    let twig = TwigPlacement {
        internode_length: 0.07,
        stations_per_internode: 3,
    };
    let reference = Reference::spanning(Vec3::new(-2.0, -1.0, -2.0), Vec3::new(2.0, 3.0, 2.0));
    let element = executor::element(f.element).unwrap();
    let x = expansion(&tree, &f, twig);
    let shared = x.compact_with_contacts().unwrap();
    let p = x.compact_stations(&shared).unwrap().unwrap();
    let mut metrics = Metrics::default();
    let wood = pollster::block_on(g.emit_positions(shared.into_surface(), &mut metrics))
        .unwrap()
        .unwrap();
    let positions = wood.positions.clone();
    let p = foliage::prepared::PreparedStations {
        segments: p.segments,
        count: p.count,
        ring_size: p.ring_size,
        rings: std::borrow::Cow::Borrowed(&positions),
    };
    let leaves = pollster::block_on(g.compute_buffer_async(
        p,
        &leaves(&f),
        twig,
        &element,
        reference,
        &mut metrics,
    ))
    .unwrap();
    let mut actual = Instances::new(reference);
    actual.leaves = pollster::block_on(io::read_leaves_async(
        &g.gpu,
        leaves.leaves.buffer(),
        leaves.count,
    ))
    .unwrap();
    actual.validate().unwrap();
    assert!(!actual.is_empty());
    let wood = pollster::block_on(g.expand_uploaded_wood(wood, &mut metrics))
        .unwrap()
        .unwrap();
    assert_eq!(wood.positions.buffer(), &positions);
    let xyz: Vec<f32> = io::read(&g.gpu, &positions, wood.vertices as u64 * 12)
        .unwrap()
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    let indices: Vec<u32> = io::read(&g.gpu, wood.indices.buffer(), wood.index_count as u64 * 4)
        .unwrap()
        .chunks_exact(4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    let point = |i: u32| {
        Vec3::new(
            xyz[i as usize * 3] as f64,
            xyz[i as usize * 3 + 1] as f64,
            xyz[i as usize * 3 + 2] as f64,
        )
    };
    let tolerance = reference.step().length() * 2.0 + 32.0 * f32::EPSILON as f64 * 4.0;
    for i in 0..actual.len() {
        let p = actual.position(i);
        let nearest = indices
            .chunks_exact(3)
            .map(|t| {
                let [a, b, c] = [point(t[0]), point(t[1]), point(t[2])];
                let normal = (b - a).cross(c - a).normalized();
                let projected = p - normal * (p - a).dot(normal);
                let inside = [(a, b), (b, c), (c, a)]
                    .into_iter()
                    .all(|(x, y)| (y - x).cross(projected - x).dot(normal) >= -1e-12);
                if inside {
                    return p.distance(projected);
                }
                [(a, b), (b, c), (c, a)]
                    .into_iter()
                    .map(|(x, y)| {
                        let edge = y - x;
                        p.distance(
                            x + edge * ((p - x).dot(edge) / edge.length_squared()).clamp(0.0, 1.0),
                        )
                    })
                    .fold(f64::INFINITY, f64::min)
            })
            .fold(f64::INFINITY, f64::min);
        assert!(nearest <= tolerance, "contact {i}: {nearest} > {tolerance}");
    }
    pollster::block_on(io::errors(&g.gpu, scopes)).unwrap();
}

#[test]
fn compact_position_candidate_does_not_override_station_capability() {
    let tree = fixture(0.0);
    let mut f = Family::default();
    f.surface.lobes = 0;
    f.canopy.surface_contact = 0.0;
    f.canopy.short_shoot_spacing = 0.0;
    f.canopy.limb_clumping = 0.0;
    f.canopy.divergence = 1e9;
    assert!(executor::expand(tree.clone(), &f)
        .unwrap()
        .compact()
        .unwrap()
        .qualified());
    assert!(expansion(
        &tree,
        &f,
        TwigPlacement {
            internode_length: 1e-6,
            stations_per_internode: 1
        }
    )
    .stations()
    .unwrap()
    .is_none());
}

thread_local! {
    static LATE_STATION: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
    static LATE_SUBMITTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub(super) fn late_station_result<T>(
    result: telperion_core::Result<Option<T>>,
    submitted: bool,
) -> telperion_core::Result<Option<T>> {
    match LATE_STATION.with(|s| s.take()) {
        None => result,
        Some(error) => {
            LATE_SUBMITTED.with(|s| s.set(submitted));
            if error {
                Err(telperion_core::Error::InvalidInput(
                    "injected station error",
                ))
            } else {
                Ok(None)
            }
        }
    }
}

#[test]
fn late_station_capability_and_error_join_positions_before_reusing_generator() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let g = Generator::new(&renderer).unwrap();
    let mut f = Family::default();
    f.skeleton.attractors = 16;
    f.surface.lobes = 0;
    f.canopy.surface_contact = 0.0;
    f.canopy.short_shoot_spacing = 0.0;
    f.canopy.limb_clumping = 0.0;
    for error in [false, true] {
        LATE_STATION.with(|s| s.set(Some(error)));
        LATE_SUBMITTED.with(|s| s.set(false));
        let result = g.prepare(&f, Delivery::Resident);
        assert!(
            LATE_SUBMITTED.with(|s| s.get()),
            "station result must follow submitted positions"
        );
        if error {
            assert!(result
                .err()
                .unwrap()
                .to_string()
                .contains("injected station error"));
        } else {
            let p = result.unwrap();
            assert_eq!(p.backend, Backend::CpuFallback);
            assert!(!p.metrics.gpu_positions);
            assert_eq!(p.metrics.position_retained_metadata_bytes, 0);
            assert!(p.metrics.position_gpu_peak_bytes > 0);
            assert!(p.resident.is_none() && p.wood.is_none());
        }
        let p = g.prepare(&f, Delivery::Resident).unwrap();
        assert!(p.metrics.gpu_positions);
        assert!(p.count() > 0);
    }
}
