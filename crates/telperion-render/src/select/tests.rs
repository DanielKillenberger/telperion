use super::*;

#[test]
fn every_level_is_given_a_list_the_whole_crown_could_fill() {
    let four = sizes(1_000, 4, 256);
    assert_eq!(four.stride % 256, 0, "a list cannot be bound at an offset");
    assert!(
        four.stride >= 4_000,
        "a list too small for the crown: {}",
        four.stride
    );
    assert_eq!(four.lists, four.stride * 4);
    // One counter per level and one for the leaves no level drew.
    assert_eq!(four.counts, 5 * 4);
    assert_eq!(four.arguments, 4 * 5 * 4);
    assert_eq!(four.scratch, (1_000 + 5 * 4) * 4);

    // A crown of nothing asks for nothing, at any number of levels.
    assert_eq!(sizes(0, 4, 256).lists, 0);
    assert_eq!(sizes(1_000, 0, 256).lists, 0);
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
        selection.submit(&gpu, &foliage, forced);
        for _ in 0..2 {
            check_compaction(&gpu, &selection, instances, MAX_LEVELS, forced);
        }
    }
}

/// A crown past 65,535 workgroups in one dimension, the size fn-194 ran into:
/// the per-leaf passes fold their groups into a second dimension and every leaf is still compacted in placement order.
#[test]
fn a_crown_past_one_dispatch_dimension_is_selected_in_order() {
    let Some(gpu) = device() else { return };
    let maximum = gpu.device.limits().max_compute_workgroups_per_dimension;
    let instances = (maximum as usize + 1) * WORKGROUP as usize + 3;
    let mut selection = Select::new(&gpu);
    selection.submit(&gpu, &interleaved(instances, 2), Level::Chosen);
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
    use telperion_core::foliage::{Element, Instances, Level as Section};
    mesh::Foliage {
        // A point sphere makes classification exact: every instance is one
        // metre ahead of the eye, or behind it, and scales select the levels.
        element: Element {
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
                let class = id % (levels + 1);
                let scale = (1u32 << class) as f32;
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

fn check_compaction(gpu: &Gpu, selection: &Select, instances: usize, levels: usize, forced: Level) {
    use telperion_core::math::Vec3;
    let camera = Camera {
        position: Vec3::ZERO,
        target: Vec3::new(0.0, 0.0, -1.0),
        field_of_view: 90.0,
        near: 0.1,
        far: 10.0,
    };
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    selection.dispatch(gpu, &mut encoder, &camera, (2, 2), true, None);
    gpu.queue.submit([encoder.finish()]);
    let counts = selection.counted(gpu).expect("counts read back");
    let bytes = buffer::read(gpu, selection.lists.as_ref().unwrap()).expect("lists read back");
    let words: Vec<u32> = bytes
        .as_chunks::<4>()
        .0
        .iter()
        .map(|word| u32::from_ne_bytes(*word))
        .collect();
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
            let start = level * selection.stride as usize / size_of::<u32>();
            assert_eq!(
                &words[start..start + expected.len()],
                expected,
                "level {level}, {forced:?}"
            );
        }
    }
}
