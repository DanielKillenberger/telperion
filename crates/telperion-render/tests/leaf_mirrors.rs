//! fn-217 E2: the supernatural group's two leaf mirrors - a metal on the
//! underside and on the margin band - act only where their reflectance is
//! above zero, and only on the face each names.
mod common;
use telperion_core::presets::Preset;
use telperion_core::supernatural::Supernatural;
use telperion_core::{math::Vec3, mesh, Family};
use telperion_render::{render, Camera, Level, Renderer, SceneRow, Still, View, STILL_FORMAT};

/// A young beech, its leaf drawn alone.
fn beech() -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    (f.age, f.skeleton.seed) = (10.0, 1);
    f
}

/// The group's mirrors at `back` and `margin`, silver beneath.
fn mirrors(enabled: bool, back: f64, margin: f64) -> Supernatural {
    Supernatural {
        enabled,
        back_reflectance: back,
        back_red: 0.92,
        back_green: 0.94,
        back_blue: 0.97,
        margin_reflectance: margin,
        roughness: 0.25,
        ..Supernatural::NONE
    }
}

/// The leaf from in front or behind, the sun low behind the eye's side.
fn leaf(renderer: &mut Renderer, back: bool) -> Still {
    let side = if back { -1.0 } else { 1.0 };
    let camera = Camera {
        position: Vec3::new(0.0, 0.055, 0.3 * side),
        target: Vec3::new(0.0, 0.055, 0.0),
        field_of_view: 38.0,
        near: 0.001,
        far: 2.0,
    };
    renderer.set_scene(SceneRow {
        sun_azimuth: if back { 180.0 } else { 0.0 },
        sun_elevation: 20.0,
        ..Default::default()
    });
    render(renderer, &camera, 192, 192).unwrap()
}

fn changed(a: &Still, b: &Still) -> usize {
    a.rgba.iter().zip(&b.rgba).filter(|(x, y)| x != y).count()
}

#[test]
fn the_mirrors_are_neutral_off_and_at_zero_and_each_acts_on_its_own_face() {
    let Some(gpu) = common::gpu() else { return };
    let family = beech();
    let tree = mesh::build(&family).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.set_view(View::Leaf);
    renderer.submit_at(&tree, Level::Forced(0)).unwrap();
    let mut with = |g: Option<Supernatural>, back: bool| {
        match g {
            None => renderer.set_material(family.material),
            Some(g) => renderer.set_family(&Family {
                supernatural: g,
                ..family.clone()
            }),
        }
        leaf(&mut renderer, back)
    };
    for back in [false, true] {
        let natural = with(None, back);
        // Off, every mirror set draws the natural leaf; on, at zero, too.
        let off = with(Some(mirrors(false, 0.8, 0.8)), back);
        assert_eq!(changed(&natural, &off), 0, "off, back={back}");
        let zero = with(Some(mirrors(true, 0.0, 0.0)), back);
        assert_eq!(changed(&natural, &zero), 0, "zero, back={back}");
        // The underside's mirror on the underside alone.
        let silver = changed(&natural, &with(Some(mirrors(true, 0.8, 0.0)), back));
        if back {
            assert!(silver > 100, "silver from behind: {silver}");
        } else {
            assert_eq!(silver, 0, "silver from in front");
        }
        // The rim's on either face.
        let rim = changed(&natural, &with(Some(mirrors(true, 0.0, 0.8)), back));
        assert!(rim > 20, "rim, back={back}: {rim}");
    }
}
