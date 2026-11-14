//! The scene's air and ground (fn-310 R3, R4): the glow round the sun, the
//! ground's patches and the far floor's haze are each a row value, each off
//! at an explicit zero, the haze never reaches the subject, and the same frame
//! is the same bytes.
//!
//! The room is drawn on its own, with no tree and no figure, at one sample a
//! pixel, so every floor pixel is lit alike and a pixel is wholly one surface.
use telperion_core::{math::Vec3, mesh};
use telperion_render::{
    hero_pose, render, Camera, Renderer, SceneRow, Still, GROUND_REACH, STILL_FORMAT,
};

mod common;

const SIZE: (u32, u32) = (192, 128);

/// The eye two metres up, looking ten degrees above the horizon toward the
/// default sun, which stands twenty degrees above the middle of the frame:
/// the sky with the glow in it, and the floor out to the skyline under it.
fn toward_the_sun() -> Camera {
    let row = SceneRow::default();
    let azimuth = row.sun_azimuth.to_radians();
    let (rise, run) = 10.0_f64.to_radians().sin_cos();
    let position = Vec3::new(0.0, 2.0, 0.0);
    let look = Vec3::new(run * azimuth.sin(), rise, run * azimuth.cos());
    Camera {
        position,
        target: position + look,
        field_of_view: 60.0,
        near: 0.1,
        far: 2.0 * GROUND_REACH,
    }
}

/// The direction the sky shader gives a pixel's centre.
fn ray(camera: &Camera, x: u32, y: u32) -> Vec3 {
    let (w, h) = (f64::from(SIZE.0), f64::from(SIZE.1));
    let forward = (camera.target - camera.position).normalized();
    let right = forward.cross(Vec3::Y).normalized();
    let up = right.cross(forward);
    let tangent = (camera.field_of_view.to_radians() / 2.0).tan();
    let across = (f64::from(x) + 0.5) / w * 2.0 - 1.0;
    let down = 1.0 - (f64::from(y) + 0.5) / h * 2.0;
    (forward + right * (tangent * w / h * across) + up * (tangent * down)).normalized()
}

/// The plain zenith-to-horizon gradient along a direction, through the frame's
/// tone curve and the target's sRGB encoding: the sky with no glow.
fn gradient(row: &SceneRow, direction: Vec3) -> [u8; 3] {
    let height = direction.y.clamp(0.0, 1.0).sqrt();
    let zenith = [
        row.sky_zenith_red,
        row.sky_zenith_green,
        row.sky_zenith_blue,
    ];
    let horizon = [
        row.sky_horizon_red,
        row.sky_horizon_green,
        row.sky_horizon_blue,
    ];
    std::array::from_fn(|i| {
        let c = (horizon[i] + (zenith[i] - horizon[i]) * height).max(0.0) * 0.6;
        let toned = ((c * (2.51 * c + 0.03)) / (c * (2.43 * c + 0.59) + 0.14)).clamp(0.0, 1.0);
        let encoded = if toned <= 0.003_130_8 {
            12.92 * toned
        } else {
            1.055 * toned.powf(1.0 / 2.4) - 0.055
        };
        (encoded * 255.0).round() as u8
    })
}

fn pixel(still: &Still, x: u32, y: u32) -> [u8; 3] {
    let i = ((y * still.width + x) * 4) as usize;
    [still.rgba[i], still.rgba[i + 1], still.rgba[i + 2]]
}

/// The worst channel by which the sky stands off the plain gradient, over
/// every pixel looking more than half a degree above the horizon.
fn off_gradient(still: &Still, row: &SceneRow, camera: &Camera) -> u8 {
    let mut worst = 0;
    for y in 0..SIZE.1 {
        for x in 0..SIZE.0 {
            let direction = ray(camera, x, y);
            if direction.y > 0.01 {
                let (got, want) = (pixel(still, x, y), gradient(row, direction));
                worst = (0..3).fold(worst, |w, i| w.max(got[i].abs_diff(want[i])));
            }
        }
    }
    worst
}

/// The distinct colours of the floor looking more than three degrees down,
/// where every ray meets the disc well inside its rim.
fn floor_colours(still: &Still, camera: &Camera) -> Vec<[u8; 3]> {
    let mut seen = Vec::new();
    for y in 0..SIZE.1 {
        for x in 0..SIZE.0 {
            let colour = pixel(still, x, y);
            if ray(camera, x, y).y < -0.05 && !seen.contains(&colour) {
                seen.push(colour);
            }
        }
    }
    seen
}

fn bare_room() -> Option<Renderer> {
    let gpu = common::gpu()?;
    let mut renderer = Renderer::with_samples(gpu, STILL_FORMAT, 1);
    renderer.set_figure(false);
    Some(renderer)
}

#[test]
fn glow_variation_and_haze_are_each_off_at_an_explicit_zero() {
    let Some(mut renderer) = bare_room() else {
        return;
    };
    let camera = toward_the_sun();
    let off = SceneRow {
        sky_glow: 0.0,
        ground_variation: 0.0,
        haze_distance: 0.0,
        ..SceneRow::default()
    };
    let mut draw = |row: SceneRow| {
        renderer.set_scene(row);
        render(&mut renderer, &camera, SIZE.0, SIZE.1).expect("the frame was drawn")
    };

    let plain = draw(off);
    // The dither is the only thing between the sky and the gradient.
    let worst = off_gradient(&plain, &off, &camera);
    assert!(
        worst <= 1,
        "with no glow the sky stands {worst} steps off the gradient"
    );
    // One flat colour, lit alike everywhere, and not the black a NaN is drawn as.
    let floor = floor_colours(&plain, &camera);
    assert_eq!(
        floor.len(),
        1,
        "with no variation and no haze the floor varies: {floor:?}"
    );
    assert_ne!(floor[0], [0, 0, 0], "the floor came out black");

    // Each effect alone is visible where the zero frame showed none of it, so
    // the checks above can fail.
    let glow = draw(SceneRow {
        sky_glow: SceneRow::default().sky_glow,
        ..off
    });
    assert!(
        off_gradient(&glow, &off, &camera) > 4,
        "the default glow drew nothing"
    );
    for (name, row) in [
        (
            "variation",
            SceneRow {
                ground_variation: SceneRow::default().ground_variation,
                ..off
            },
        ),
        (
            "haze",
            SceneRow {
                haze_distance: SceneRow::default().haze_distance,
                ..off
            },
        ),
    ] {
        let colours = floor_colours(&draw(row), &camera).len();
        assert!(
            colours > 4,
            "the default {name} left the floor {colours} colours"
        );
    }
}

/// Straight down from above, where one floor pixel lies under the eye and its
/// horizon has no direction: the default row draws no black there.
#[test]
fn the_floor_under_the_eye_is_drawn() {
    let Some(mut renderer) = bare_room() else {
        return;
    };
    let camera = Camera {
        position: Vec3::new(0.0, 30.0, 0.0),
        target: Vec3::new(0.0, 0.0, 0.001),
        field_of_view: 40.0,
        near: 0.1,
        far: 100.0,
    };
    let still = render(&mut renderer, &camera, SIZE.0, SIZE.1).expect("the frame was drawn");
    let black = still.rgba.chunks(4).filter(|p| p[..3] == [0, 0, 0]).count();
    assert_eq!(black, 0, "{black} pixels of the floor are black");
}

/// The first frames of a tree size its buffers to what they asked for and
/// draw the sun again under them (fn-258); what is compared comes after.
fn settle(renderer: &mut Renderer, camera: &Camera) {
    for _ in 0..3 {
        render(renderer, camera, SIZE.0, SIZE.1).expect("the frame was drawn");
    }
}

/// The tree and the figure are never hazed (fn-310 D4): the subject's pixels
/// are those where a frame with the tree differs from the bare room under the
/// same row, and between the default haze and none only other pixels move.
/// The sun is put out so the tree throws no shadow onto the floor.
#[test]
fn the_haze_changes_only_floor_pixels() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&common::ordinary()).expect("the tree grew");
    let mut subject = Renderer::with_samples(gpu.clone(), STILL_FORMAT, 1);
    subject.submit(&tree).expect("the tree fits the device");
    let mut room = Renderer::with_samples(gpu, STILL_FORMAT, 1);
    room.set_figure(false);
    let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
    let camera = hero_pose(subject.bounds().unwrap(), aspect, GROUND_REACH);
    let dark = SceneRow {
        sun_red: 0.0,
        sun_green: 0.0,
        sun_blue: 0.0,
        ..SceneRow::default()
    };
    let clear = SceneRow {
        haze_distance: 0.0,
        ..dark
    };
    let draw = |renderer: &mut Renderer, row: SceneRow| {
        renderer.set_scene(row);
        render(renderer, &camera, SIZE.0, SIZE.1).expect("the frame was drawn")
    };
    settle(&mut subject, &camera);
    let (with, without) = (draw(&mut subject, clear), draw(&mut room, clear));
    let hazed = draw(&mut subject, dark);
    let (mut subject_pixels, mut moved, mut moved_on_subject) = (0, 0, 0);
    for ((tree, bare), haze) in with
        .rgba
        .chunks(4)
        .zip(without.rgba.chunks(4))
        .zip(hazed.rgba.chunks(4))
    {
        let on_subject = tree != bare;
        subject_pixels += usize::from(on_subject);
        moved += usize::from(haze != tree);
        moved_on_subject += usize::from(on_subject && haze != tree);
    }
    assert!(
        subject_pixels > 500,
        "the subject covers {subject_pixels} pixels"
    );
    assert!(moved > 500, "the haze moved {moved} pixels");
    assert_eq!(
        moved_on_subject, 0,
        "the haze reached {moved_on_subject} subject pixels"
    );
}

/// The noise and the dither are hashes of place and pixel: the same frame on
/// one device is the same bytes (fn-310 R4).
#[test]
fn the_same_frame_is_the_same_bytes() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&common::ordinary()).expect("the tree grew");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).expect("the tree fits the device");
    let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
    let camera = hero_pose(renderer.bounds().unwrap(), aspect, GROUND_REACH);
    settle(&mut renderer, &camera);
    let first = render(&mut renderer, &camera, SIZE.0, SIZE.1).expect("the frame was drawn");
    let second = render(&mut renderer, &camera, SIZE.0, SIZE.1).expect("the frame was drawn");
    assert!(first.rgba == second.rgba, "one frame drew two pictures");
}
