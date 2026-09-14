//! Fill the shared frame uniform from the scene row, material and fitted light.
use super::*;

impl Scene {
    /// Writes the frame's one uniform block: where the eye stands and what it
    /// looks along, where the sun stands with the map it threw, and the two
    /// rows the frame is drawn from - the scene's and the subject's.
    pub fn set_frame(
        &self,
        gpu: &Gpu,
        camera: &crate::Camera,
        aspect: f64,
        light: &Light,
        view: View,
    ) {
        let m = &self.material;
        let colour = |r: f64, g: f64, b: f64, w: f64| [r as f32, g as f32, b as f32, w as f32];
        let clay = linear(CLAY);
        let (right, up, forward) = axes(camera, aspect);
        // A leaf on its own has no crown to stand deep inside, and the clay
        // room takes no material term at all.
        let inside = self
            .crown
            .filter(|_| view == View::Whole || view == View::Bare);
        let centre = inside.map_or([0.0; 4], |b| {
            let c = (b.min + b.max) * 0.5;
            colour(c.x, c.y, c.z, 1.0)
        });
        let radii = inside.map_or([0.0; 4], |b| {
            let half = (b.max - b.min) * 0.5;
            colour(half.x, half.y, half.z, 0.0)
        });
        gpu.queue.write_buffer(
            &self.uniforms,
            0,
            bytemuck::bytes_of(&Uniforms {
                view_projection: camera.view_projection(aspect),
                sky: linear(SKY_LIGHT),
                ground: linear(GROUND_LIGHT),
                clay: [
                    clay[0],
                    clay[1],
                    clay[2],
                    f32::from(u8::from(view == View::Clay)),
                ],
                light_view_projection: light.view_projection,
                shadow_filter: [
                    self.row.shadow_filter_texels as f32,
                    1.0 / crate::shadow::RESOLUTION as f32,
                    0.0,
                    0.0,
                ],
                shadow_offset: [
                    (self.row.shadow_normal_offset * light.texel_size) as f32,
                    0.0,
                    0.0,
                    0.0,
                ],
                sun: colour(self.row.sun_red, self.row.sun_green, self.row.sun_blue, 1.0),
                sun_direction: light.direction,
                eye: colour(camera.position.x, camera.position.y, camera.position.z, 1.0),
                ray_right: colour(right.x, right.y, right.z, 0.0),
                ray_up: colour(up.x, up.y, up.z, 0.0),
                ray_forward: colour(forward.x, forward.y, forward.z, 0.0),
                sky_zenith: colour(
                    self.row.sky_zenith_red,
                    self.row.sky_zenith_green,
                    self.row.sky_zenith_blue,
                    1.0,
                ),
                sky_horizon: colour(
                    self.row.sky_horizon_red,
                    self.row.sky_horizon_green,
                    self.row.sky_horizon_blue,
                    1.0,
                ),
                ground_colour: colour(
                    self.row.ground_red,
                    self.row.ground_green,
                    self.row.ground_blue,
                    1.0,
                ),
                bark: colour(m.bark_red, m.bark_green, m.bark_blue, m.bark_roughness),
                bark_detail: colour(
                    m.ridge_scale,
                    m.plate_scale,
                    m.roughness_detail,
                    m.furrow_strength,
                ),
                leaf_detail: colour(
                    m.vein_scale,
                    m.vein_contrast,
                    self.section_roundness,
                    m.thickness,
                ),
                transmission: colour(
                    m.transmission_red,
                    m.transmission_green,
                    m.transmission_blue,
                    m.transmission_strength,
                ),
                leaf_front: colour(
                    m.leaf_front_red,
                    m.leaf_front_green,
                    m.leaf_front_blue,
                    m.interior_darkening,
                ),
                leaf_back: colour(m.leaf_back_red, m.leaf_back_green, m.leaf_back_blue, 1.0),
                leaf_variation: [
                    m.hue_range_low as f32,
                    m.hue_range_high as f32,
                    m.brightness_range_low as f32,
                    m.brightness_range_high as f32,
                ],
                crown_centre: centre,
                crown_radii: radii,
                fissure: colour(
                    m.fissure_red,
                    m.fissure_green,
                    m.fissure_blue,
                    m.fissure_strength,
                ),
                crest: colour(m.crest_red, m.crest_green, m.crest_blue, m.crest_strength),
                bark_colour_detail: colour(
                    m.bark_mottle_scale,
                    m.bark_mottle_strength,
                    m.cavity_strength,
                    m.sky_occlusion_strength,
                ),
                leaf_colour_detail: colour(
                    m.blade_mottle_scale,
                    m.blade_mottle_strength,
                    m.cuticle_gloss,
                    0.0,
                ),
                margin: colour(m.margin_red, m.margin_green, m.margin_blue, m.margin_width),
                plate: colour(
                    m.plate_cell_scale,
                    m.plate_elongation,
                    m.plate_dome,
                    m.plate_edge_lift,
                ),
                bark_structure: colour(
                    m.plate_identity,
                    m.directional_occlusion,
                    m.depth_strength,
                    m.plate_furrow_width,
                ),
                weathering: colour(
                    m.weathering_red,
                    m.weathering_green,
                    m.weathering_blue,
                    m.weathering_strength,
                ),
                orientation: colour(
                    m.orientation_red,
                    m.orientation_green,
                    m.orientation_blue,
                    m.orientation_strength,
                ),
            }),
        );
    }
}

/// The picture's axes from the pose it was taken at: where the eye looks, and
/// how far a pixel at the edge of the frame leans off that in each direction.
/// A ray through the frame is the forward axis plus the two, so the sky can be
/// read per pixel without the projection being inverted.
fn axes(camera: &crate::Camera, aspect: f64) -> (Vec3, Vec3, Vec3) {
    let forward = (camera.target - camera.position).normalized();
    let right = forward.cross(Vec3::Y).normalized();
    let up = right.cross(forward);
    let tangent = (camera.field_of_view.to_radians() / 2.0).tan();
    (right * (tangent * aspect), up * tangent, forward)
}
