//! Whether the scene's light is white (fn-228). A floor of stated neutral
//! albedo is drawn under the default sun and sky from above, with a tree on
//! it to throw a shadow, and its colour is read back in sun and in shade.
//! The photographs the rows are measured off were balanced to daylight, so a
//! neutral surface in full daylight, sun plus sky, has to come back neutral:
//! a light that tints it tints every species the same way. Shade, the blue
//! sky alone, is cool in those photographs too, so it is reported and not
//! bounded (host, fn-228).
//!
//! The floor is the neutral object because its shader is the light and
//! nothing else, the albedo times the ambient plus the sun, and its colour is
//! a scene-row value, so the test states it without touching a material row.
use telperion_core::{math::Vec3, mesh};
use telperion_render::{render, Camera, Renderer, SceneRow, Still, STILL_FORMAT};

mod common;

/// The white and the mid-grey, linear albedo.
const ALBEDOS: [f64; 2] = [0.8, 0.18];
/// How far one channel may stand off the mean of the three, as a share of it.
const TOLERANCE: f64 = 0.03;
const SIZE: u32 = 256;

/// One display byte back to linear.
fn linear(byte: u8) -> f64 {
    let c = f64::from(byte) / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// The floor's mean linear colour in sun and in shade. The floor is every
/// pixel that is exactly black when the ground is black: the sky and the tree
/// never are. Of those, the sunlit are the brightest and the shaded the
/// darkest, each within five per cent of its extreme, so the penumbra between
/// them is left out.
fn floor(lit: &Still, black: &Still) -> ([f64; 3], [f64; 3], usize, usize) {
    let pixels: Vec<[f64; 3]> = lit
        .rgba
        .chunks(4)
        .zip(black.rgba.chunks(4))
        .filter(|(_, dark)| dark[..3] == [0, 0, 0])
        .map(|(pixel, _)| [linear(pixel[0]), linear(pixel[1]), linear(pixel[2])])
        .collect();
    let luminance = |p: &[f64; 3]| 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2];
    let mut sorted: Vec<f64> = pixels.iter().map(luminance).collect();
    sorted.sort_by(f64::total_cmp);
    let low = sorted[sorted.len() / 100];
    let high = sorted[sorted.len() * 99 / 100];
    let mean = |keep: &dyn Fn(f64) -> bool| {
        let chosen: Vec<&[f64; 3]> = pixels.iter().filter(|p| keep(luminance(p))).collect();
        let n = chosen.len() as f64;
        let sum = chosen
            .iter()
            .fold([0.0; 3], |s, p| [s[0] + p[0], s[1] + p[1], s[2] + p[2]]);
        ([sum[0] / n, sum[1] / n, sum[2] / n], chosen.len())
    };
    let (sun, suns) = mean(&|l| l >= high * 0.95);
    let (shade, shades) = mean(&|l| l <= low * 1.05);
    (sun, shade, suns, shades)
}

/// How far the colour's furthest channel stands off the mean of the three.
fn cast(colour: [f64; 3]) -> f64 {
    let mean = colour.iter().sum::<f64>() / 3.0;
    colour
        .iter()
        .map(|c| (c - mean).abs() / mean)
        .fold(0.0, f64::max)
}

fn ground(row: SceneRow, albedo: f64) -> SceneRow {
    SceneRow {
        ground_red: albedo,
        ground_green: albedo,
        ground_blue: albedo,
        ..row
    }
}

/// A grey of the same luminance as this colour.
fn grey(r: f64, g: f64, b: f64) -> [f64; 3] {
    let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    [y; 3]
}

/// The lights one at a time: the default, the sky's light made grey, the sun
/// made grey, and both. The tone map is the same curve on every channel, so
/// what is left once both lights are grey is what it and the exposure add.
fn variants() -> Vec<(&'static str, SceneRow)> {
    let d = SceneRow::default();
    let zenith = grey(d.sky_zenith_red, d.sky_zenith_green, d.sky_zenith_blue);
    let horizon = grey(d.sky_horizon_red, d.sky_horizon_green, d.sky_horizon_blue);
    let sun = grey(d.sun_red, d.sun_green, d.sun_blue);
    let grey_sky = SceneRow {
        sky_zenith_red: zenith[0],
        sky_zenith_green: zenith[1],
        sky_zenith_blue: zenith[2],
        sky_horizon_red: horizon[0],
        sky_horizon_green: horizon[1],
        sky_horizon_blue: horizon[2],
        ..d
    };
    let grey_sun = |row: SceneRow| SceneRow {
        sun_red: sun[0],
        sun_green: sun[1],
        sun_blue: sun[2],
        ..row
    };
    vec![
        ("default", d),
        ("grey sky", grey_sky),
        ("grey sun", grey_sun(d)),
        ("grey sky and sun", grey_sun(grey_sky)),
    ]
}

/// Straight down on the tree from high enough to hold its shadow at the
/// default sun's thirty degrees, tipped a little so the view has an up.
fn above(renderer: &Renderer) -> Camera {
    let bounds = renderer.bounds().expect("a tree is up");
    let height = bounds.max.y - bounds.min.y;
    let reach = 2.2 * height;
    let distance = reach / (19.0_f64).to_radians().tan();
    let target = Vec3::new(
        0.5 * (bounds.min.x + bounds.max.x),
        0.0,
        0.5 * (bounds.min.z + bounds.max.z),
    );
    Camera {
        position: target + Vec3::new(0.0, distance, 0.05 * distance),
        target,
        field_of_view: 38.0,
        near: 0.5,
        far: 4.0 * distance,
    }
}

#[test]
fn a_neutral_floor_renders_neutral_in_full_daylight() {
    let Some(gpu) = common::gpu() else { return };
    let tree = mesh::build(&common::ordinary()).expect("the tree grew");
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit(&tree).expect("the tree fits the device");
    let camera = above(&renderer);
    let mut draw = |row: SceneRow| {
        renderer.set_scene(row);
        render(&mut renderer, &camera, SIZE, SIZE).expect("the frame was drawn")
    };
    let mut failures = Vec::new();
    for (name, row) in variants() {
        let black = draw(ground(row, 0.0));
        for albedo in ALBEDOS {
            let still = draw(ground(row, albedo));
            if let Ok(path) = std::env::var("NEUTRAL_LIGHT_STILLS") {
                let file = format!("{path}/{}-{albedo}.png", name.replace(' ', "-"));
                telperion_render::write_png(std::path::Path::new(&file), &still).unwrap();
            }
            let (sun, shade, suns, shades) = floor(&still, &black);
            assert!(
                suns > 50 && shades > 50,
                "{name}: {suns} sunlit and {shades} shaded pixels"
            );
            println!(
                "{name:18} albedo {albedo:4}: sun {:.3}/{:.3}/{:.3} b:g {:.2} ({suns} px), \
                 shade {:.3}/{:.3}/{:.3} b:g {:.2} ({shades} px)",
                sun[0],
                sun[1],
                sun[2],
                sun[2] / sun[1],
                shade[0],
                shade[1],
                shade[2],
                shade[2] / shade[1],
            );
            if name == "default" && cast(sun) > TOLERANCE {
                failures.push(format!("albedo {albedo} in sun: {sun:.3?}"));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "the default daylight tints a neutral floor past {TOLERANCE}: {failures:?}"
    );
}
