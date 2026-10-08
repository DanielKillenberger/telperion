//! fn-234 R3: the flowering cherry beech with the petals' own colour on
//! the CPU path (`mesh::build`) and the resident device path (`prepare`
//! with `Delivery::Resident`, through `prepare_fine`), from the hero camera:
//! the same florets, the same organ coverage and the same drawn petals.
use super::super::{Delivery, Generator};
use super::blossom_measure::{coverage, linear, srgb};
use super::blossom_trees::cherry;
use crate::{hero_pose, render, Level, Renderer, SceneRow, Still, View, GROUND_REACH};

const SIZE: (u32, u32) = (480, 360);

/// The organ mask and the whole still of what `renderer` holds.
fn shoot(renderer: &mut Renderer, camera: &crate::Camera) -> (Still, Still) {
    renderer.set_scene(SceneRow {
        sun_azimuth: 115.0,
        sun_elevation: 40.0,
        ..SceneRow::default()
    });
    renderer.set_view(View::Clay);
    renderer.set_organ_mask(true);
    let mask = render(renderer, camera, SIZE.0, SIZE.1).unwrap();
    renderer.set_organ_mask(false);
    renderer.set_view(View::Whole);
    (mask, render(renderer, camera, SIZE.0, SIZE.1).unwrap())
}

/// The whole still's mean sRGB where the mask covers at least half a pixel.
fn petals(mask: &Still, whole: &Still) -> [f64; 3] {
    let at: Vec<usize> = (0..mask.rgba.len() / 4)
        .filter(|k| linear(mask.rgba[k * 4 + 1]) >= 0.5)
        .collect();
    assert!(at.len() > 100, "the florets cover {} pixels", at.len());
    [0, 1, 2].map(|ch| {
        let sum: f64 = at.iter().map(|k| linear(whole.rgba[k * 4 + ch])).sum();
        srgb(sum / at.len() as f64)
    })
}

#[test]
fn the_flowering_tree_draws_alike_on_both_executors() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let family = cherry();
    let mesh = telperion_core::mesh::build(&family).unwrap();
    let florets = mesh.organ.as_ref().unwrap().instances.len();
    let camera = hero_pose(
        mesh.bounds,
        f64::from(SIZE.0) / f64::from(SIZE.1),
        GROUND_REACH,
    );
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    renderer.submit_at(&mesh, Level::Chosen).unwrap();
    renderer.set_family(&family);
    let (cpu_mask, cpu_whole) = shoot(&mut renderer, &camera);
    drop(renderer);
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let prepared = Generator::new(&renderer)
        .unwrap()
        .prepare(&family, Delivery::Resident)
        .unwrap();
    let held = prepared
        .mesh
        .organ
        .as_ref()
        .map_or(0, |o| o.instances.len());
    let placed = prepared.organ.as_ref().map_or(0, |o| o.count as usize);
    assert_eq!(
        (held, placed),
        (0, florets),
        "the device places every floret"
    );
    renderer.submit_prepared(prepared).unwrap();
    renderer.set_family(&family);
    let (device_mask, device_whole) = shoot(&mut renderer, &camera);
    let (a, b) = (coverage(&cpu_mask), coverage(&device_mask));
    let (p, q) = (
        petals(&cpu_mask, &cpu_whole),
        petals(&device_mask, &device_whole),
    );
    println!("coverage cpu {a:.5} device {b:.5}; petals cpu {p:.1?} device {q:.1?}");
    assert!((a - b).abs() <= 0.01 * a, "coverage cpu {a} device {b}");
    for ch in 0..3 {
        assert!(
            (p[ch] - q[ch]).abs() <= 2.0,
            "petals cpu {p:?} device {q:?}"
        );
    }
}
