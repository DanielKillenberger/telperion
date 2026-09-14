//! A joined fork needs contact shade even when its bark relief is disabled.
mod common;
use telperion_core::{
    material::MaterialParams,
    math::Vec3,
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{render, Camera, Level, Renderer, SceneRow, View, STILL_FORMAT};

#[test]
fn a_fork_socket_occludes_sun_and_sky_without_relief() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit_at(&tree, Level::Chosen).unwrap();
    renderer.set_view(View::Bare);
    let camera = Camera {
        position: Vec3::new(-1.335209229402595, 4.351800788132381, -1.9281673692824055),
        target: Vec3::new(0.0, 4.0, 0.0),
        field_of_view: 38.0,
        near: 0.01,
        far: 1000.0,
    };
    let off = MaterialParams::default();
    for sun in [false, true] {
        renderer.set_scene(if sun {
            SceneRow::default()
        } else {
            SceneRow {
                sun_red: 0.0,
                sun_green: 0.0,
                sun_blue: 0.0,
                ..Default::default()
            }
        });
        renderer.set_material(off);
        let plain = render(&mut renderer, &camera, 512, 512).unwrap();
        renderer.set_material(MaterialParams {
            cavity_strength: 0.8,
            ..off
        });
        let dark = render(&mut renderer, &camera, 512, 512).unwrap();
        // The central socket crop lies well above the ground. A zero-relief
        // row separates geometric contact from the existing fissure cavity.
        let mut before = 0.0;
        let mut after = 0.0;
        for y in 128..384 {
            for x in 128..384 {
                let i = (y * 512 + x) * 4;
                for (channel, weight) in [(0, 0.2126), (1, 0.7152), (2, 0.0722)] {
                    before += f64::from(plain.rgba[i + channel]) * weight;
                    after += f64::from(dark.rgba[i + channel]) * weight;
                }
            }
        }
        eprintln!("socket sun={sun}: plain={before}, dark={after}");
        assert!(
            after < before * 0.999,
            "socket sun={sun}: plain={before}, dark={after}"
        );
        assert_eq!(
            dark.rgba,
            render(&mut renderer, &camera, 512, 512).unwrap().rgba
        );
    }
}
