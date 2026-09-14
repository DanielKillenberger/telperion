//! A furrow floor is darkened by its own crest standing between it and the
//! sun. What separates that from fn-29's cavity is which side it darkens: move
//! the sun across the trunk and the shaded side must move with it.
mod common;
use telperion_core::{
    material::MaterialParams,
    math::Vec3,
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{render, Camera, Renderer, SceneRow, Still, View, STILL_FORMAT};

const SIZE: (u32, u32) = (480, 300);

fn trunk() -> Camera {
    Camera {
        position: Vec3::new(1.7307636095778745, 2.2967023330704928, -1.7307636095778745),
        target: Vec3::new(0.0, 2.0, 0.0),
        field_of_view: 38.0,
        near: 0.01,
        far: 1000.0,
    }
}

/// Mean luminance over a vertical strip of the frame, in the delivered sRGB
/// the comparison below uses on both sides alike.
fn strip(frame: &Still, from: u32, to: u32) -> f64 {
    let (width, height) = SIZE;
    let mut sum = 0.0;
    let mut count = 0.0;
    for y in height / 4..3 * height / 4 {
        for x in from..to {
            let p = 4 * (y * width + x) as usize;
            sum += 0.2126 * f64::from(frame.rgba[p])
                + 0.7152 * f64::from(frame.rgba[p + 1])
                + 0.0722 * f64::from(frame.rgba[p + 2]);
            count += 1.0;
        }
    }
    sum / count
}

#[test]
fn the_darkened_side_of_a_furrow_follows_the_sun() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_view(View::Bare);
    let lit = MaterialParams {
        directional_occlusion: 0.0,
        ..family.material
    };
    assert!(
        family.material.directional_occlusion > 0.0,
        "the oak's own row is what is being measured"
    );
    // The trunk fills the middle of the frame at this pose; a quarter of the
    // width on each side of centre is wood on both, lit from opposite hands.
    let (left, right) = (
        SIZE.0 / 2 - 90..SIZE.0 / 2 - 20,
        SIZE.0 / 2 + 20..SIZE.0 / 2 + 90,
    );
    let mut swing = Vec::new();
    for azimuth in [45.0, 225.0] {
        renderer.set_scene(SceneRow {
            sun_azimuth: azimuth,
            sun_elevation: 20.0,
            ..SceneRow::default()
        });
        renderer.set_material(lit);
        let plain = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
        renderer.set_material(family.material);
        let shaded = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
        let again = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
        assert_eq!(shaded.rgba, again.rgba, "the identical draw changed");
        // What the term took out of each side of the trunk, in luminance.
        let took = |range: std::ops::Range<u32>| {
            strip(&plain, range.start, range.end) - strip(&shaded, range.start, range.end)
        };
        let (took_left, took_right) = (took(left.clone()), took(right.clone()));
        eprintln!(
            "sun {azimuth}: took {took_left:.4} from the left, {took_right:.4} from the right"
        );
        assert!(
            took_left + took_right > 0.05,
            "the term did nothing at sun {azimuth}: {took_left}, {took_right}"
        );
        assert!(
            took_left >= -1e-9 && took_right >= -1e-9,
            "the term returned light at sun {azimuth}: {took_left}, {took_right}"
        );
        swing.push(took_left - took_right);
    }
    // fn-29's cavity darkens both sides of every furrow alike, so it would
    // leave this difference unchanged when the sun crosses. A term derived
    // from the height towards the sun cannot: the side it takes from swaps.
    eprintln!("left-minus-right darkening: {:?}", swing);
    assert!(
        swing[0] * swing[1] < 0.0,
        "the darkening did not change sides with the sun: {swing:?}"
    );
    assert!(
        swing.iter().all(|s| s.abs() > 0.05),
        "the asymmetry is too small to be the term rather than noise: {swing:?}"
    );
}

#[test]
fn the_row_at_zero_leaves_the_frame_exactly_as_it_was() {
    let Some(gpu) = common::gpu() else { return };
    let mut family = Preset::OregonWhiteOak.parameters();
    family.skeleton.seed = 7;
    let tree = mesh::build(&family, Detail::Full).unwrap();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).unwrap();
    renderer.set_view(View::Bare);
    let off = MaterialParams {
        directional_occlusion: 0.0,
        ..family.material
    };
    renderer.set_material(off);
    let first = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
    renderer.set_material(family.material);
    let _ = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
    renderer.set_material(off);
    let again = render(&mut renderer, &trunk(), SIZE.0, SIZE.1).unwrap();
    assert_eq!(
        first.rgba, again.rgba,
        "the row at zero did not return the frame it started from"
    );
}
