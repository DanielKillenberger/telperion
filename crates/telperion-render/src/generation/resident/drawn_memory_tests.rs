//! fn-257 task 10's measurement (`#[ignore = "a measurement"]`): where a
//! harness build's drawing sits in memory. For each shipped preset at its
//! default age (`DRAWN_SEEDS`, seed 1 by default): the build's peak and
//! its largest buffers, the drawn records and swept nodes at that peak,
//! the buffers the drawn tree keeps once submitted, and the buffers a
//! still at the hero pose adds.
use super::super::{Delivery, Generator};
use crate::Renderer;

fn megabytes(at: &[(&'static str, u64)], top: usize) -> String {
    (at.iter().take(top))
        .map(|(l, b)| format!("{l} {:.1}", *b as f64 / 1e6))
        .collect::<Vec<_>>()
        .join(", ")
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
            let prepared = gen.prepare(&family, Delivery::Resident).expect("prepared");
            let (peak, at) = gen.gpu.ledger.take_peak_census();
            renderer.submit_prepared(prepared).expect("submitted");
            pollster::block_on(crate::generation::io::complete(&gen.gpu)).expect("idle");
            let kept = gen.gpu.ledger.census_now();
            let tree = Generator::tree_buffer_bytes(&renderer);
            let bounds = renderer.bounds().expect("a tree drawn");
            let camera = crate::hero_pose(bounds, 4.0 / 3.0, crate::GROUND_REACH);
            crate::render(&mut renderer, &camera, 1280, 960).expect("a still");
            pollster::block_on(crate::generation::io::complete(&gen.gpu)).expect("idle");
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
