//! Stable blade coordinates carry colour and a front-face cuticle at every level.
mod common;
use telperion_core::{
    foliage::{build_element, ElementParams},
    material::MaterialParams,
    math::Vec3,
    mesh,
    presets::Preset,
};
use telperion_render::{render, Camera, Level, Renderer, SceneRow, View, STILL_FORMAT};

#[test]
fn blade_colour_rows_survive_levels_and_the_back_has_no_cuticle() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(20);
    let mut tree = mesh::build(&family).unwrap();
    tree.foliage.element = build_element(ElementParams {
        cup: 0.0,
        curl: 0.0,
        cross_segments: 8,
        axial_segments: 20,
        ..Default::default()
    })
    .unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.set_view(View::Leaf);
    renderer.set_scene(SceneRow {
        sun_azimuth: 0.0,
        sun_elevation: 5.0,
        ..Default::default()
    });
    let off = MaterialParams::default();
    let rows = [
        (
            "mottle",
            MaterialParams {
                blade_mottle_scale: 6.0,
                blade_mottle_strength: 0.8,
                ..off
            },
        ),
        (
            "margin",
            MaterialParams {
                margin_width: 0.15,
                margin_red: 0.12,
                margin_green: 0.12,
                margin_blue: 0.03,
                ..off
            },
        ),
        (
            "gloss",
            MaterialParams {
                cuticle_gloss: 0.6,
                ..off
            },
        ),
    ];
    for back in [false, true] {
        let camera = Camera {
            position: Vec3::new(0.0, 0.055, if back { -0.3 } else { 0.3 }),
            target: Vec3::new(0.0, 0.055, 0.0),
            field_of_view: 38.0,
            near: 0.001,
            far: 2.0,
        };
        for level in 0..tree.foliage.element.levels.len() {
            renderer
                .submit_at(&tree, Level::Forced(level as u32))
                .unwrap();
            renderer.set_material(off);
            let plain = render(&mut renderer, &camera, 192, 192).unwrap();
            for (name, row) in rows {
                renderer.set_material(row);
                let on = render(&mut renderer, &camera, 192, 192).unwrap();
                let changed = plain
                    .rgba
                    .iter()
                    .zip(&on.rgba)
                    .filter(|(a, b)| a != b)
                    .count();
                if back && name == "gloss" {
                    assert_eq!(changed, 0, "back level {level}");
                } else {
                    assert!(
                        changed > 100,
                        "{name}, back={back}, level={level}: {changed}"
                    );
                }
                assert_eq!(
                    on.rgba,
                    render(&mut renderer, &camera, 192, 192).unwrap().rgba
                );
            }
        }
    }
}

#[test]
fn a_matte_needle_row_is_inert_even_with_scales_and_tints_set() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::Ordinary.parameters();
    family.skeleton.growth.max_nodes = Some(20);
    family.element.section_roundness = 1.0;
    let tree = mesh::build(&family).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.set_view(View::Leaf);
    for level in 0..tree.foliage.element.levels.len() {
        renderer
            .submit_at(&tree, Level::Forced(level as u32))
            .unwrap();
        let camera = telperion_render::hero_pose(renderer.bounds().unwrap(), 1.0, 0.0);
        renderer.set_material(MaterialParams::default());
        let off = render(&mut renderer, &camera, 128, 128).unwrap();
        renderer.set_material(MaterialParams {
            blade_mottle_scale: 8.0,
            blade_mottle_strength: 0.0,
            margin_red: 0.1,
            margin_green: 0.2,
            margin_blue: 0.03,
            margin_width: 0.0,
            cuticle_gloss: 0.0,
            ..Default::default()
        });
        let on = render(&mut renderer, &camera, 128, 128).unwrap();
        assert_eq!(off.rgba, on.rgba, "needle level {level}");
    }
}
