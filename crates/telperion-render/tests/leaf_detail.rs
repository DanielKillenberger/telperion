//! The blade's detail survives the level ladder and disappears on a round section.
mod common;
use telperion_core::{
    foliage::{build_element, ElementParams},
    material::MaterialParams,
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{hero_pose, render, Level, Renderer, View, GROUND_REACH, STILL_FORMAT};

#[test]
fn veins_draw_at_every_blade_level_and_never_on_a_round_section() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(20);
    let mut tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.set_view(View::Leaf);
    for roundness in [0.0, 1.0] {
        tree.foliage.element = build_element(ElementParams {
            section_roundness: roundness,
            cross_segments: 8,
            axial_segments: 20,
            ..Default::default()
        })
        .unwrap();
        for level in 0..tree.foliage.element.levels.len() {
            renderer
                .submit_at(&tree, Level::Forced(level as u32))
                .unwrap();
            let camera = hero_pose(renderer.bounds().unwrap(), 1.0, GROUND_REACH);
            renderer.set_material(MaterialParams::default());
            let off = render(&mut renderer, &camera, 192, 192).unwrap();
            renderer.set_material(MaterialParams {
                vein_contrast: 0.8,
                ..Default::default()
            });
            let on = render(&mut renderer, &camera, 192, 192).unwrap();
            let changed = off
                .rgba
                .iter()
                .zip(&on.rgba)
                .filter(|(a, b)| a != b)
                .count();
            if roundness == 1.0 {
                assert_eq!(changed, 0, "round section at level {level}");
            } else {
                assert!(
                    changed > 100,
                    "blade at level {level}: only {changed} channels changed"
                );
            }
        }
    }
}

#[test]
fn transmission_reaches_the_drawn_leaf_through_the_material_uniform() {
    use telperion_core::math::Vec3;
    use telperion_render::{Camera, SceneRow};
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(20);
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_view(View::Leaf);
    renderer.set_scene(SceneRow {
        sun_azimuth: 180.0,
        sun_elevation: 5.0,
        ..Default::default()
    });
    let camera = Camera {
        position: Vec3::new(0.0, 0.055, 0.3),
        target: Vec3::new(0.0, 0.055, 0.0),
        field_of_view: 38.0,
        near: 0.001,
        far: 2.0,
    };
    for level in 0..tree.foliage.element.levels.len() {
        renderer
            .submit_at(&tree, Level::Forced(level as u32))
            .unwrap();
        renderer.set_material(MaterialParams::default());
        let off = render(&mut renderer, &camera, 128, 128).unwrap();
        renderer.set_material(MaterialParams {
            transmission_strength: 0.8,
            thickness: 0.5,
            ..Default::default()
        });
        let on = render(&mut renderer, &camera, 128, 128).unwrap();
        let gained: i64 = on
            .rgba
            .iter()
            .zip(&off.rgba)
            .map(|(a, b)| i64::from(*a) - i64::from(*b))
            .sum();
        assert!(gained > 1000, "backlit blade gained only {gained}");
    }
}
