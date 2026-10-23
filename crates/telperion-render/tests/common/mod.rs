//! What every device-backed test file needs: a device, or a printed reason and
//! a test that does not run. A machine without a GPU is not a failing build.
use telperion_render::{Gpu, RenderError};

/// A device, or `None` with the reason printed. Every device-backed assertion
/// hangs off this; a refusal that is not about the hardware still fails.
pub fn gpu() -> Option<Gpu> {
    match pollster::block_on(Gpu::request(None)) {
        Ok(gpu) => Some(gpu),
        Err(error @ (RenderError::WebGpuUnavailable(_) | RenderError::FallbackOnly { .. })) => {
            println!("skipped: {error}");
            None
        }
        Err(error) => panic!("the device was there and still refused: {error}"),
    }
}

/// The ordinary tree's dressing on a young engine beech: the tree a test
/// that drew the ordinary preset draws since the engine grows stage 2 and
/// a preset with no engine species is refused (fn-198).
#[allow(dead_code)]
pub fn ordinary() -> telperion_core::Family {
    use telperion_core::presets::Preset;
    let mut family = Preset::Ordinary.parameters();
    family.space = Preset::EuropeanBeech.parameters().space;
    family.age = 40.0;
    family
}

/// The sun's map fitted to `casters` under a sun at (azimuth, elevation).
fn sun_map(
    (sun_azimuth, sun_elevation): (f64, f64),
    casters: &[telperion_core::surface::Bounds],
) -> telperion_render::SunLight {
    let row = telperion_render::SceneRow {
        sun_azimuth,
        sun_elevation,
        ..telperion_render::SceneRow::default()
    };
    telperion_render::sun_light(&row, casters)
}

/// The world size of the larger texel axis of the sun's map fitted to
/// `casters`, the size its shaders read (`Light::texel_size`), in metres.
#[allow(dead_code)]
pub fn texel_size(sun: (f64, f64), casters: &[telperion_core::surface::Bounds]) -> f64 {
    sun_map(sun, casters).texel_size
}

/// Metres of ground one texel of the sun's map covers along each of its two
/// axes, fitted to `casters` under a sun at (azimuth, elevation): the axis
/// across the sun as it is, the one up the sun's view stretched over the
/// ground by the sun's elevation.
#[allow(dead_code)]
pub fn ground_texels(sun: (f64, f64), casters: &[telperion_core::surface::Bounds]) -> [f64; 2] {
    let m = sun_map(sun, casters).view_projection.map(f64::from);
    // Each clip axis's row of the matrix is its world axis over half the
    // map's extent along it.
    let texel = |r: usize| {
        2.0 / m[r].hypot(m[4 + r]).hypot(m[8 + r]) / f64::from(telperion_render::SUN_RESOLUTION)
    };
    [texel(0), texel(1) / sun.1.to_radians().sin()]
}

/// How much coarser the sun's map is fitted to `casters` and the person's
/// box than to `casters` alone: the larger of its two axes' ratios of ground
/// per texel.
#[allow(dead_code)]
pub fn coarsened(
    sun: (f64, f64),
    casters: &[telperion_core::surface::Bounds],
    person: telperion_core::surface::Bounds,
) -> f64 {
    let alone = ground_texels(sun, casters);
    let mut all = casters.to_vec();
    all.push(person);
    let with = ground_texels(sun, &all);
    (with[0] / alone[0]).max(with[1] / alone[1])
}

/// The box a person's body points stand in.
#[allow(dead_code)]
pub fn box_of(points: &[telperion_core::math::Vec3]) -> telperion_core::surface::Bounds {
    let (mut min, mut max) = (points[0], points[0]);
    for p in points {
        min = telperion_core::math::Vec3::new(min.x.min(p.x), min.y.min(p.y), min.z.min(p.z));
        max = telperion_core::math::Vec3::new(max.x.max(p.x), max.y.max(p.y), max.z.max(p.z));
    }
    telperion_core::surface::Bounds { min, max }
}
