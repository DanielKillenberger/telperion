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
    use crate::RenderError;
    let gpu = match pollster::block_on(Gpu::request(None)) {
        Ok(gpu) => gpu,
        Err(error @ (RenderError::WebGpuUnavailable(_) | RenderError::FallbackOnly { .. })) => {
            eprintln!("skipped: {error}");
            return;
        }
        Err(error) => panic!("device refused: {error}"),
    };
    let mut selection = Select::new(&gpu);
    for (instances, forced) in [
        (65_539, Level::Chosen),
        (65_539, Level::Forced(0)),
        (65_539, Level::Forced(15)),
        (257, Level::Chosen),
        (1, Level::Chosen),
    ] {
        let foliage = interleaved(instances);
        selection.submit(&gpu, &foliage, forced);
        for _ in 0..2 {
            check_compaction(&gpu, &selection, instances, forced);
        }
    }
}

fn interleaved(instances: usize) -> mesh::Foliage {
    use telperion_core::foliage::{Element, Instances, Level as Section};
    mesh::Foliage {
        // A point sphere makes classification exact: every instance is one
        // metre ahead of the eye, or behind it, and scales select the levels.
        element: Element {
            levels: (0..MAX_LEVELS)
                .map(|i| Section {
                    indices: 0..3,
                    deviation: if i == MAX_LEVELS - 1 {
                        0.0
                    } else {
                        0.25 / (1u32 << i) as f64
                    },
                })
                .collect(),
            ..Element::default()
        },
        instances: Instances {
            matrices: (0..instances)
                .map(|id| {
                    let class = id % (MAX_LEVELS + 1);
                    let scale = (1u32 << class) as f32;
                    let mut matrix = [0.0; 16];
                    for i in [0, 5, 10] {
                        matrix[i] = scale;
                    }
                    matrix[14] = if class == MAX_LEVELS { 1.0 } else { -1.0 };
                    matrix[15] = 1.0;
                    matrix
                })
                .collect(),
        },
    }
}

fn check_compaction(gpu: &Gpu, selection: &Select, instances: usize, forced: Level) {
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
                let class = *id as usize % (MAX_LEVELS + 1);
                let chosen = match forced {
                    Level::Forced(index) if class < MAX_LEVELS => index as usize,
                    _ => class,
                };
                chosen == level
            })
            .collect();
        assert_eq!(count as usize, expected.len(), "level {level}, {forced:?}");
        if level < MAX_LEVELS {
            let start = level * selection.stride as usize / size_of::<u32>();
            assert_eq!(
                &words[start..start + expected.len()],
                expected,
                "level {level}, {forced:?}"
            );
        }
    }
}
