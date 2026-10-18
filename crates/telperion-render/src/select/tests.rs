use super::*;

#[test]
fn the_levels_share_one_list_the_crown_fills_once() {
    let four = sizes(1_000, 4, 256);
    // Each leaf is in at most one level, so every level end to end holds
    // the crown once.
    assert_eq!(
        four.lists, 4_096,
        "the lists are not the crown once, aligned"
    );
    // One start word per level, each at a bindable offset.
    assert_eq!((four.slot, four.starts), (256, 4 * 256));
    // One counter per level and one for the leaves no level drew.
    assert_eq!(four.counts, 5 * 4);
    assert_eq!(four.arguments, 4 * 5 * 4);
    assert_eq!(four.scratch, (1_000 + 5 * 4) * 4);

    // A crown of nothing asks for no list, and no levels for no starts.
    assert_eq!(sizes(0, 4, 256).lists, 0);
    assert_eq!(sizes(1_000, 0, 256).starts, 0);

    // A start word is written as storage and read as a uniform, so its slot
    // takes the wider of the two alignments, whichever that is.
    for (uniform, storage) in [(256, 32), (64, 256)] {
        let limits = wgpu::Limits {
            min_uniform_buffer_offset_alignment: uniform,
            min_storage_buffer_offset_alignment: storage,
            ..Default::default()
        };
        assert_eq!(slot_alignment(&limits), 256);
    }
}

/// R1: a level no leaf falls in this frame starts where the next level does
/// and leaves every other level's list as it was; a forced level starts at
/// zero.
#[test]
fn an_empty_level_starts_where_the_next_does_and_a_forced_one_at_zero() {
    let Some(gpu) = device() else { return };
    let levels = 4;
    let full = interleaved(1_000, levels);
    // Level 1's leaves moved behind the eye, to the unseen bucket.
    let emptied = classed(1_000, levels, |id| match id % (levels + 1) {
        1 => levels,
        class => class,
    });
    let run = |foliage: &mesh::Foliage, forced| {
        let mut selection = Select::new(&gpu);
        selection.submit(&gpu, foliage, &foliage.element, forced);
        run_frame(&gpu, &selection);
        let starts = selection.starts(&gpu).expect("starts read back");
        (selection.level_lists(&gpu), starts)
    };
    let (lists, _) = run(&full, Level::Chosen);
    let (without, starts) = run(&emptied, Level::Chosen);
    assert!(without[1].is_empty(), "level 1 drew {:?}", without[1]);
    assert_eq!(starts[1], starts[2], "{starts:?}");
    for level in [0, 2, 3] {
        assert_eq!(without[level], lists[level], "level {level}");
    }
    let (forced, starts) = run(&full, Level::Forced(2));
    assert_eq!(starts[2], 0, "{starts:?}");
    assert_eq!(forced[2].len(), 800, "every seen leaf at the forced level");
}

/// Interleave every level and the culled bucket across words, workgroups and
/// prefix chunks. More than 256 workgroups exercises the multi-count chunks;
/// subsequent smaller submissions exercise reused buffers and partial tails.
#[test]
fn compaction_preserves_placement_order_at_every_level() {
    let Some(gpu) = device() else { return };
    let mut selection = Select::new(&gpu);
    for (instances, forced) in [
        (65_539, Level::Chosen),
        (65_539, Level::Forced(0)),
        (65_539, Level::Forced(15)),
        (257, Level::Chosen),
        (1, Level::Chosen),
    ] {
        let foliage = interleaved(instances, MAX_LEVELS);
        selection.submit(&gpu, &foliage, &foliage.element, forced);
        for _ in 0..2 {
            check_compaction(&gpu, &selection, instances, MAX_LEVELS, forced);
        }
    }
}

/// A crown past 65,535 workgroups in one dimension, the size fn-194 ran into:
/// the per-leaf passes fold their groups into a second dimension and every
/// leaf is still compacted in placement order. A device that cannot bind a
/// crown this size skips, by name, as one without an adapter does.
#[test]
fn a_crown_past_one_dispatch_dimension_is_selected_in_order() {
    let Some(gpu) = device() else { return };
    let limits = gpu.device.limits();
    let instances =
        (limits.max_compute_workgroups_per_dimension as usize + 1) * WORKGROUP as usize + 3;
    let foliage = interleaved(instances, 2);
    let placements = Region::capacity_for(size_of_val(&foliage.instances.leaves[..]) as u64);
    if placements > limits.max_storage_buffer_binding_size {
        eprintln!(
            "skipped: {placements} bytes of placements, {} bindable",
            limits.max_storage_buffer_binding_size
        );
        return;
    }
    let mut selection = Select::new(&gpu);
    selection.submit(&gpu, &foliage, &foliage.element, Level::Chosen);
    check_compaction(&gpu, &selection, instances, 2, Level::Chosen);
}

/// Below the limit the grid is the flat dispatch it replaced; above it, the
/// fold covers every group with less than one row to spare.
#[test]
fn the_grid_is_flat_until_one_dimension_runs_out() {
    assert_eq!(grid(0, 65_535), (0, 1));
    assert_eq!(grid(65_535, 65_535), (65_535, 1));
    assert_eq!(grid(65_536, 65_535), (65_535, 2));
    let (x, y) = grid(74_572, 65_535);
    assert!(x * y >= 74_572 && x * (y - 1) < 74_572);
}

fn device() -> Option<Gpu> {
    use crate::RenderError;
    match pollster::block_on(Gpu::request(None)) {
        Ok(gpu) => Some(gpu),
        Err(error @ (RenderError::WebGpuUnavailable(_) | RenderError::FallbackOnly { .. })) => {
            eprintln!("skipped: {error}");
            None
        }
        Err(error) => panic!("device refused: {error}"),
    }
}

/// Every class of leaf in turn: one per level and the culled bucket last.
fn interleaved(instances: usize, levels: usize) -> mesh::Foliage {
    classed(instances, levels, |id| id % (levels + 1))
}

/// Each leaf in the class `class` gives it: a level, or `levels` for the
/// culled bucket.
fn classed(instances: usize, levels: usize, class: impl Fn(usize) -> usize) -> mesh::Foliage {
    use telperion_core::foliage::{Element, Instances, Level as Section};
    mesh::Foliage {
        // A unit sphere at the origin makes classification exact: every
        // instance is one metre ahead of the eye, or behind it, and scales
        // select the levels. Its radius puts the smallest class at two
        // pixels across, where nothing is thinned (fn-246).
        element: Element {
            positions: vec![
                telperion_core::math::Vec3::new(-1.0, 0.0, 0.0),
                telperion_core::math::Vec3::new(1.0, 0.0, 0.0),
            ],
            levels: (0..levels)
                .map(|i| Section {
                    indices: 0..3,
                    deviation: if i == levels - 1 {
                        0.0
                    } else {
                        0.25 / (1u32 << i) as f64
                    },
                })
                .collect(),
            ..Element::default()
        },
        instances: {
            // A box a code wide on z and none at all across, so the two
            // depths the fixture uses land on codes of their own and the
            // leaves stand exactly where they were put.
            let mut out = Instances::new(telperion_core::foliage::Reference::spanning(
                telperion_core::math::Vec3::new(0.0, 0.0, -1.0),
                telperion_core::math::Vec3::new(0.0, 0.0, 1.0),
            ));
            for id in 0..instances {
                let class = class(id);
                // The culled class behind the eye keeps a unit scale, so its
                // sphere stays clear of the near plane.
                let scale = if class == levels {
                    1.0
                } else {
                    (1u32 << class) as f32
                };
                let mut matrix = [0.0; 16];
                for i in [0, 5, 10] {
                    matrix[i] = scale;
                }
                matrix[14] = if class == levels { 1.0 } else { -1.0 };
                matrix[15] = 1.0;
                out.push(&matrix);
            }
            out
        },
    }
}

/// One frame from the eye at the origin looking down -z, which the fixtures
/// class their leaves against.
fn run_frame(gpu: &Gpu, selection: &Select) {
    use telperion_core::math::Vec3;
    let camera = Camera {
        position: Vec3::ZERO,
        target: Vec3::new(0.0, 0.0, -1.0),
        field_of_view: 90.0,
        near: 0.1,
        far: 10.0,
    };
    let mut encoder = gpu.encoder(&Default::default());
    selection.dispatch(gpu, &mut encoder, &camera, (2, 2), true, None);
    gpu.submit([encoder.finish()]);
}

fn check_compaction(gpu: &Gpu, selection: &Select, instances: usize, levels: usize, forced: Level) {
    run_frame(gpu, selection);
    let counts = selection.counted(gpu).expect("counts read back");
    let lists = selection.level_lists(gpu);
    for (level, count) in counts.into_iter().enumerate() {
        let expected: Vec<u32> = (0..instances as u32)
            .filter(|id| {
                let class = *id as usize % (levels + 1);
                let chosen = match forced {
                    Level::Forced(index) if class < levels => index as usize,
                    _ => class,
                };
                chosen == level
            })
            .collect();
        assert_eq!(count as usize, expected.len(), "level {level}, {forced:?}");
        if level < levels {
            assert_eq!(lists[level], expected, "level {level}, {forced:?}");
        }
    }
}

/// fn-253: an organ's inflorescence is thinned as one. Its florets, then
/// its rachis, stand in a run; at a size thinning keeps a share of, every
/// unit is kept or dropped whole, by its rachis's rank, and some of each.
/// On the leaves (no stalk), the rachis's mark is a withered leaf's and
/// each thins on its own.
#[test]
fn an_inflorescence_is_kept_or_dropped_whole() {
    use telperion_core::foliage::WITHERED;
    let Some(gpu) = device() else { return };
    let mut foliage = interleaved(4_000, 4);
    for (id, leaf) in foliage.instances.leaves.iter_mut().enumerate() {
        // Every instance a metre ahead at unit scale, each unit turned its
        // own way so its rachis ranks apart from the others.
        leaf[0] = (id / 4) as u32 * 2_654_435_761 % (1 << 30);
        leaf[1] = 0;
        leaf[2] = u32::from(half(1.0)) << 16;
        if id % 4 == 3 {
            leaf[2] |= WITHERED;
        }
    }
    let mut selection = Select::new(&gpu);
    selection.submit(&gpu, &foliage, &foliage.element, Level::Chosen);
    selection.set_stalk(1e-4, 0.0, 0.0);
    let camera = Camera {
        position: telperion_core::math::Vec3::ZERO,
        target: telperion_core::math::Vec3::new(0.0, 0.0, -1.0),
        // A metre spans 0.58 px a metre ahead: a unit of reach 1 keeps a third.
        field_of_view: 120.0,
        near: 0.1,
        far: 10.0,
    };
    let mut encoder = gpu.encoder(&Default::default());
    selection.dispatch(&gpu, &mut encoder, &camera, (2, 2), true, None);
    gpu.submit([encoder.finish()]);
    let chosen: Vec<u32> = (selection.chosen(&gpu).into_iter())
        .map(|level| level.unwrap_or(4))
        .collect();
    let mut kept = 0;
    for unit in chosen.chunks(4) {
        let drawn: Vec<bool> = unit.iter().map(|&c| c < 4).collect();
        assert!(
            drawn.iter().all(|&d| d == drawn[0]),
            "a unit split: {unit:?}"
        );
        kept += usize::from(drawn[0]);
    }
    assert!((200..800).contains(&kept), "{kept} of 1000 units kept");
}

/// `v` as the half float a placement's scale is stored in.
fn half(v: f32) -> u16 {
    let bits = v.to_bits();
    let exponent = ((bits >> 23) & 0xff) as i32 - 127 + 15;
    ((exponent as u32) << 10 | ((bits >> 13) & 0x3ff)) as u16
}

/// fn-253 review: a forced level is held within each kind's own ladder, so
/// forcing the coarsest or the finest still draws every rachis and every
/// floret: the rachis's levels lead, the floret's follow.
#[test]
fn a_forced_level_stays_within_each_kind_s_ladder() {
    use telperion_core::foliage::{Element, Level as Section, WITHERED};
    use telperion_core::math::Vec3;
    let Some(gpu) = device() else { return };
    let mut foliage = interleaved(400, 2);
    let p = |x, y| Vec3::new(x, y, 0.0);
    foliage.element = Element {
        positions: vec![
            p(-1.0, 0.0),
            p(1.0, 0.0),
            p(0.0, 1.0),
            p(-1e-6, 0.0),
            p(1e-6, 0.0),
            p(0.0, 1e-6),
        ],
        coords: vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -2.0, 0.0, -2.0, 1.0, -2.0, 2.0,
        ],
        level_indices: vec![3, 4, 5, 3, 4, 5, 0, 1, 2, 0, 1, 2],
        levels: [(0, 1e30), (3, 1e29), (6, 0.25), (9, 0.0)]
            .map(|(at, deviation)| Section {
                indices: at..at + 3,
                deviation,
            })
            .to_vec(),
        ..Element::default()
    };
    for (id, leaf) in foliage.instances.leaves.iter_mut().enumerate() {
        (leaf[0], leaf[1], leaf[2]) = (id as u32 / 2, 0, u32::from(half(1.0)) << 16);
        if id % 2 == 1 {
            leaf[2] |= WITHERED;
        }
    }
    let camera = Camera {
        position: Vec3::ZERO,
        target: Vec3::new(0.0, 0.0, -1.0),
        field_of_view: 90.0,
        near: 0.1,
        far: 10.0,
    };
    for (forced, rachis, floret) in [(0, 0, 2), (1_000, 1, 3)] {
        let mut selection = Select::new(&gpu);
        selection.submit(&gpu, &foliage, &foliage.element, Level::Forced(forced));
        selection.set_stalk(1.0, 0.0, 0.0);
        let mut encoder = gpu.encoder(&Default::default());
        selection.dispatch(&gpu, &mut encoder, &camera, (2, 2), true, None);
        gpu.submit([encoder.finish()]);
        let counts = selection.counted(&gpu).expect("counts read back");
        assert_eq!(
            (counts[rachis], counts[floret]),
            (200, 200),
            "forced {forced}: {counts:?}"
        );
    }
}

/// fn-253 review: a thin rachis takes the coarsest ribbon whose chords stand
/// under half a pixel off its axis, where its pedicels are seated. A
/// curved rachis seen large takes the ribbon through every ring; seen
/// small, or straight, the coarsest.
#[test]
fn a_thin_rachis_takes_the_coarsest_ribbon_true_to_its_axis() {
    use telperion_core::foliage::{Element, Instances, Level as Section, Reference, WITHERED};
    use telperion_core::math::Vec3;
    let Some(gpu) = device() else { return };
    let p = |x, y| Vec3::new(x, y, 0.0);
    let stalk = |k: f64| [p(-1e-6 * k, 0.0), p(1e-6 * k, 0.0), p(0.0, 1e-6 * k)];
    let mut positions = vec![p(-1.0, 0.0), p(1.0, 0.0), p(0.0, 1.0)];
    for k in 1..=4 {
        positions.extend(stalk(f64::from(k)));
    }
    let mut coords = vec![0.0; 6];
    coords.extend((0..12).flat_map(|i| [-2.0, i as f32]));
    // Four rachis levels (ribbons over 2, 4 and 8 segments, the tube),
    // then two floret levels.
    let level_indices: Vec<u32> = (0..4u32)
        .flat_map(|k| [3 + 3 * k, 4 + 3 * k, 5 + 3 * k])
        .chain([0, 1, 2, 0, 1, 2])
        .collect();
    let deviations = [1e30, 1e29, 1e28, 1e27, 0.25, 0.0];
    let element = Element {
        positions,
        coords,
        levels: (0..6u32)
            .map(|k| Section {
                indices: 3 * k..3 * k + 3,
                deviation: deviations[k as usize],
            })
            .collect(),
        level_indices,
        ..Element::default()
    };
    let foliage = |heading: Vec3| {
        let face = Vec3::Y.cross(heading);
        let face = if face.length_squared() > 0.0 {
            face.normalized()
        } else {
            Vec3::Z
        };
        let side = heading.cross(face);
        let r = Reference::spanning(Vec3::new(-1.0, -1.0, -2.0), Vec3::new(1.0, 1.0, 0.0));
        let mut out = Instances::new(r);
        let s = 1e-4;
        let mut m = [0.0f32; 16];
        for (c, v) in [side, heading, face].iter().enumerate() {
            (m[4 * c], m[4 * c + 1], m[4 * c + 2]) =
                ((v.x * s) as f32, (v.y * s) as f32, (v.z * s) as f32);
        }
        (m[14], m[15]) = (-1.0, 1.0);
        out.push(&m);
        out.leaves[0][2] |= WITHERED;
        mesh::Foliage {
            element: element.clone(),
            instances: out,
        }
    };
    for (droop, viewport, chosen) in [(1.0, 2, 0), (1.0, 2_000, 2), (0.0, 2_000, 0)] {
        let tree = foliage(Vec3::X);
        let mut selection = Select::new(&gpu);
        selection.submit(&gpu, &tree, &tree.element, Level::Chosen);
        selection.set_stalk(1.0, 0.0, droop);
        let camera = Camera {
            position: Vec3::ZERO,
            target: Vec3::new(0.0, 0.0, -1.0),
            field_of_view: 90.0,
            near: 0.1,
            far: 10.0,
        };
        let mut encoder = gpu.encoder(&Default::default());
        selection.dispatch(
            &gpu,
            &mut encoder,
            &camera,
            (viewport, viewport),
            true,
            None,
        );
        gpu.submit([encoder.finish()]);
        let counts = selection.counted(&gpu).expect("counts read back");
        assert_eq!(
            counts[chosen], 1,
            "droop {droop} at {viewport} px: {counts:?}"
        );
    }
}
