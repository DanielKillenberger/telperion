//! fn-257 task 10's measurement (`#[ignore = "a measurement"]`): where a
//! harness build's drawing sits in memory. For each shipped preset at its
//! default age (`DRAWN_SEEDS`, seed 1 by default): the build's peak and
//! its largest buffers, the drawn records and swept nodes at that peak,
//! the buffers the drawn tree keeps once submitted, and the buffers a
//! still at the hero pose adds.
use super::super::Generator;
use crate::Renderer;

fn megabytes(at: &[(&'static str, u64)], top: usize) -> String {
    (at.iter().take(top))
        .map(|(l, b)| format!("{l} {:.1}", *b as f64 / 1e6))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Every foliage buffer the drawn tree keeps, in bytes (fn-292).
fn foliage_bytes(at: &[(&'static str, u64)]) -> String {
    (at.iter().filter(|(l, _)| l.starts_with("foliage ")))
        .map(|(l, b)| format!("{l} {b}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Each layer's leaves and levels as the still's selection counted them,
/// with the level lists the base (0a6664741) allocated for that crown,
/// computed: a whole leaf count per level, with the buffer's headroom.
fn base_lists(renderer: &Renderer) -> String {
    let alignment = renderer
        .gpu
        .device
        .limits()
        .min_storage_buffer_offset_alignment;
    [("leaves", &renderer.foliage), ("organ", &renderer.organ)]
        .iter()
        .filter_map(|(layer, foliage)| {
            let counted = foliage.counted(&renderer.gpu)?;
            let leaves: u64 = counted.iter().map(|&c| u64::from(c)).sum();
            let levels = (counted.len() as u64).checked_sub(1)?;
            let stride = (leaves * 4).div_ceil(u64::from(alignment)) * u64::from(alignment);
            let base = crate::buffer::Region::capacity_for(stride * levels);
            Some(format!(
                "{layer} {leaves} leaves at {levels} levels, base lists {base}"
            ))
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn named(at: &[(&'static str, u64)], label: &str) -> f64 {
    at.iter()
        .filter(|(l, _)| *l == label)
        .map(|(_, b)| *b as f64 / 1e6)
        .sum()
}

#[test]
#[ignore = "a measurement"]
fn the_drawing_s_memory() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let seeds = std::env::var("DRAWN_SEEDS").unwrap_or_else(|_| "1".into());
    for id in telperion_core::presets::CATALOGUE.iter().map(|c| c.1) {
        for seed in seeds.split(',').map(|s| s.parse::<u32>().expect("a seed")) {
            let mut family = telperion_core::presets::by_identity(id).expect("a preset");
            family.skeleton.seed = seed;
            let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
            let gen = Generator::new(&renderer).expect("a generator");
            pollster::block_on(crate::generation::io::complete(&gen.gpu)).expect("idle");
            gen.gpu.ledger.take_peak_census();
            let prepared = gen.prepare(&family).expect("prepared");
            let (peak, at) = gen.gpu.ledger.take_peak_census();
            renderer.submit_prepared(prepared).expect("submitted");
            pollster::block_on(crate::generation::io::complete(&gen.gpu)).expect("idle");
            let kept = gen.gpu.ledger.census_now();
            let tree = Generator::tree_buffer_bytes(&renderer);
            let bounds = renderer.bounds().expect("a tree drawn");
            let camera = crate::hero_pose(bounds, 4.0 / 3.0, crate::GROUND_REACH);
            crate::render(&mut renderer, &camera, 1280, 960).expect("a still");
            pollster::block_on(crate::generation::io::complete(&gen.gpu)).expect("idle");
            println!(
                "FOLIAGE {id} {seed}: kept {}; {}",
                foliage_bytes(&kept),
                base_lists(&renderer)
            );
            let drawing = gen.gpu.ledger.census_now();
            let grew: Vec<(&'static str, u64)> = drawing
                .iter()
                .map(|(l, b)| {
                    (
                        *l,
                        b.saturating_sub(kept.iter().find(|k| k.0 == *l).map_or(0, |k| k.1)),
                    )
                })
                .filter(|e| e.1 > 0)
                .collect();
            println!(
                "DRAWN {id} {seed}: build peak {:.1} MB (drawn {:.1}, sweep nodes {:.1}; {}); kept {:.1} MB, the tree's {:.1} ({}); a still adds {}, the curve then {:.1} MB",
                peak as f64 / 1e6,
                named(&at, "drawn"),
                named(&at, "sweep nodes"),
                megabytes(&at, 6),
                kept.iter().map(|e| e.1).sum::<u64>() as f64 / 1e6,
                tree as f64 / 1e6,
                megabytes(&kept, 8),
                megabytes(&grew, 6),
                renderer.curve_bytes().unwrap_or(0) as f64 / 1e6,
            );
            drop(renderer);
            pollster::block_on(crate::generation::io::complete(&gen.gpu)).expect("idle");
        }
    }
}

/// The stills' close view (`examples/resident_stills.rs`): the crown's edge
/// on the hero camera's side, `share` of the tree's height out.
fn nearer(
    bounds: &telperion_core::surface::Bounds,
    hero: &crate::Camera,
    share: f64,
) -> crate::Camera {
    use telperion_core::math::Vec3;
    let t = Vec3::new(hero.position.x, 0.0, hero.position.z).normalized();
    let height = bounds.max.y - bounds.min.y;
    let reach = 0.5 * (bounds.max.x - bounds.min.x).max(bounds.max.z - bounds.min.z);
    let side = Vec3::new(t.z, 0.0, -t.x);
    let target = Vec3::Y * (0.55 * height) + t * (0.5 * reach) + side * (0.2 * reach);
    let distance = share * height;
    crate::Camera {
        position: target + t * distance + Vec3::Y * (0.1 * distance),
        target,
        field_of_view: 38.0,
        near: 0.005,
        far: 4.0 * height,
    }
}

/// FNV-1a over the ids in the order a level draws them.
fn ordered_hash(ids: &[u32]) -> u64 {
    (ids.iter().flat_map(|id| id.to_le_bytes())).fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// fn-292's identity proof (`#[ignore = "a measurement"]`): every shipped
/// preset at its default age, seed 1, at the stills' whole and close views;
/// per layer and level, the leaves the level draws and a hash of their ids
/// in drawn order. The level lists' layout may change; these lines may not.
#[test]
#[ignore = "a measurement"]
fn the_drawn_levels() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    const SIZE: (u32, u32) = (960, 720);
    for id in telperion_core::presets::CATALOGUE.iter().map(|c| c.1) {
        let family = telperion_core::presets::by_identity(id).expect("a preset");
        let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
        let gen = Generator::new(&renderer).expect("a generator");
        let prepared = gen.prepare(&family).expect("prepared");
        renderer.submit_prepared(prepared).expect("submitted");
        renderer.set_family(&family);
        let bounds = renderer.bounds().expect("a tree drawn");
        let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
        let hero = crate::hero_pose(bounds, aspect, crate::GROUND_REACH);
        for (view, camera) in [("whole", hero), ("close", nearer(&bounds, &hero, 0.12))] {
            crate::render(&mut renderer, &camera, SIZE.0, SIZE.1).expect("a still");
            for (layer, foliage) in [("leaves", &renderer.foliage), ("organ", &renderer.organ)] {
                for (level, ids) in foliage.level_lists(&renderer.gpu).iter().enumerate() {
                    println!(
                        "LEVELS {id} {view} {layer} {level}: {} {:016x}",
                        ids.len(),
                        ordered_hash(ids)
                    );
                }
            }
        }
        drop(renderer);
        pollster::block_on(crate::generation::io::complete(&gen.gpu)).expect("idle");
    }
}
