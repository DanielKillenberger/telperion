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
        viewport: (u32, u32),
        light: &Light,
        view: View,
    ) {
        let aspect = crate::pass::aspect_of(viewport);
        // The pixels a metre spans at a metre, which thinning sizes a leaf
        // on screen by, as selection does (fn-246).
        let pixels = crate::select::frame::pixels_per_metre(
            f64::from(viewport.1.max(1)),
            camera.field_of_view,
        );
        let (m, r) = (&self.material, &self.reflection);
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
        // The leaf view binds one placement at identity, packed against a box
        // of no extent, so it has to be decoded against that same box: the
        // crown's own would carry the leaf off to wherever its words land in
        // a box thirty metres wide.
        let leaf_box = if view == View::Leaf {
            telperion_core::foliage::Reference::default()
        } else {
            self.leaf_reference
        };
        let radii = inside.map_or([0.0; 4], |b| {
            let half = (b.max - b.min) * 0.5;
            colour(half.x, half.y, half.z, 0.0)
        });
        let mut uniforms = Uniforms {
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
            eye: colour(
                camera.position.x,
                camera.position.y,
                camera.position.z,
                pixels,
            ),
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
            leaf_box_min: colour(
                leaf_box.min.x,
                leaf_box.min.y,
                leaf_box.min.z,
                f64::from(self.leaf_radius),
            ),
            leaf_box_extent: colour(leaf_box.extent.x, leaf_box.extent.y, leaf_box.extent.z, 0.0),
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
                f64::from(self.band_shares.at(m.margin_width)),
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
            shoot: colour(m.shoot_red, m.shoot_green, m.shoot_blue, m.shoot_radius),
            canopy: colour(
                m.canopy_normal,
                m.light_wrap,
                m.diffuse_transmission,
                m.leaf_sheen,
            ),
            crown_shade: colour(m.crown_shade, m.lobe_shade, 0.0, 0.0),
            lichen: colour(
                m.lichen_red,
                m.lichen_green,
                m.lichen_blue,
                m.lichen_strength,
            ),
            lichen_detail: colour(m.lichen_scale, m.lichen_coverage, 0.0, 0.0),
            lenticel: colour(
                m.lenticel_density,
                m.lenticel_length,
                m.lenticel_strength,
                m.lenticel_tint,
            ),
            peel: colour(m.peel_red, m.peel_green, m.peel_blue, m.peel_curl),
            reflectance: colour(
                m.bark_reflectance,
                m.leaf_reflectance,
                r.margin,
                r.roughness,
            ),
            grain: colour(
                m.bark_grain_scale,
                m.bark_grain_strength,
                m.blade_grain_scale,
                m.blade_grain_strength,
            ),
            plate_profile: colour(m.plate_edge_shape, 0.0, 0.0, 0.0),
            leaf_dead: colour(
                m.leaf_dead_red,
                m.leaf_dead_green,
                m.leaf_dead_blue,
                self.colouring,
            ),
            mature: colour(
                m.mature_red,
                m.mature_green,
                m.mature_blue,
                m.mature_furrow_strength,
            ),
            mature_radius: colour(m.mature_radius, m.mature_span, 0.0, 0.0),
            leaf_reflection: colour(r.back_tint[0], r.back_tint[1], r.back_tint[2], r.back),
            bloom_eye: [0.0; 4],
            stalk: [0.0; 4],
            atmosphere: colour(
                self.row.sky_glow,
                self.row.haze_distance,
                super::GROUND_REACH,
                0.0,
            ),
            ground_dry: colour(
                self.row.ground_dry_red,
                self.row.ground_dry_green,
                self.row.ground_dry_blue,
                self.row.ground_variation,
            ),
        };
        if self.mask {
            // The clay room unlit and black: nothing but the flowers shows.
            uniforms.clay = [0.0, 0.0, 0.0, uniforms.clay[3]];
            (uniforms.sky, uniforms.ground) = ([0.0; 4], [0.0; 4]);
        }
        gpu.queue
            .write_buffer(&self.uniforms, 0, bytemuck::bytes_of(&uniforms));
        if let Some((reference, radius)) = self.organ_reference.filter(|_| view != View::Leaf) {
            let mut organ = organ(uniforms, m, reference, radius);
            organ.stalk = self.stalk;
            if self.mask {
                organ.clay = [1.0, 1.0, 1.0, organ.clay[3]];
                (organ.sky, organ.ground) = ([1.0; 4], [1.0; 4]);
            }
            gpu.queue
                .write_buffer(&self.organ_uniforms, 0, bytemuck::bytes_of(&organ));
        }
    }
}

/// The flowers' block (fn-216): the frame's, drawn by the same leaf shader,
/// with the petals' two colours, the front one also what they pass, no
/// veins, no margin, no mottle, none of the leaf's mirrors and no canopy
/// normal, their eye, and their florets' own box.
fn organ(
    mut u: Uniforms,
    m: &telperion_core::material::MaterialParams,
    reference: telperion_core::foliage::Reference,
    radius: f32,
) -> Uniforms {
    let colour = |r: f64, g: f64, b: f64, w: f32| [r as f32, g as f32, b as f32, w];
    u.leaf_front = colour(
        m.bloom_front_red,
        m.bloom_front_green,
        m.bloom_front_blue,
        u.leaf_front[3],
    );
    u.leaf_back = colour(m.bloom_back_red, m.bloom_back_green, m.bloom_back_blue, 1.0);
    // A petal passes its own colour, not the leaf's (fn-234 R2).
    u.transmission = colour(
        m.bloom_front_red,
        m.bloom_front_green,
        m.bloom_front_blue,
        u.transmission[3],
    );
    u.leaf_detail[1] = 0.0;
    u.leaf_colour_detail[1] = 0.0;
    u.margin[3] = 0.0;
    // The leaf's mirrors are the leaf's: no petal takes a silver underside
    // or a metallic rim (fn-217 E2).
    u.leaf_reflection[3] = 0.0;
    u.reflectance[2] = 0.0;
    // A petal is lit as its own surface: the normal a leaf bends toward the
    // crown's mass would shade a flower as part of it (fn-217 F3).
    u.canopy[0] = 0.0;
    u.bloom_eye = colour(
        m.bloom_eye_red,
        m.bloom_eye_green,
        m.bloom_eye_blue,
        m.bloom_eye_share as f32,
    );
    u.leaf_box_min = colour(reference.min.x, reference.min.y, reference.min.z, radius);
    u.leaf_box_extent = colour(
        reference.extent.x,
        reference.extent.y,
        reference.extent.z,
        0.0,
    );
    u
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

#[cfg(test)]
mod tests {
    use super::*;

    /// No petal takes the leaf's mirrors: the organ's block zeroes the
    /// underside's and the margin's reflectance whatever the leaves carry.
    #[test]
    fn the_flowers_take_none_of_the_leaf_s_mirrors() {
        let mut leaves = Uniforms::zeroed();
        leaves.leaf_reflection = [0.9, 0.9, 0.9, 0.8];
        leaves.reflectance = [0.1, 0.2, 0.7, 0.3];
        let m = telperion_core::material::MaterialParams::default();
        let o = organ(
            leaves,
            &m,
            telperion_core::foliage::Reference::default(),
            0.0,
        );
        assert_eq!((o.leaf_reflection[3], o.reflectance[2]), (0.0, 0.0));
        assert_eq!((o.reflectance[0], o.reflectance[1]), (0.1, 0.2));
    }

    /// A petal is lit as its own surface, not bent toward the crown's mass
    /// as a leaf is (fn-217 F3): the organ's block takes no canopy normal
    /// and keeps the leaves' wrap, transmission and sheen.
    #[test]
    fn a_petal_is_lit_as_a_surface_not_as_the_crown_s_mass() {
        let mut leaves = Uniforms::zeroed();
        leaves.canopy = [0.6, 0.3, 0.4, 0.2];
        let m = telperion_core::material::MaterialParams::default();
        let o = organ(
            leaves,
            &m,
            telperion_core::foliage::Reference::default(),
            0.0,
        );
        assert_eq!(o.canopy, [0.0, 0.3, 0.4, 0.2]);
    }

    /// The eye is the flowers' alone (fn-217 F1): the organ's block carries
    /// the material's eye colour and share, the leaves' none.
    #[test]
    fn the_eye_is_the_flowers_alone() {
        let m = telperion_core::material::MaterialParams {
            bloom_eye_red: 0.2,
            bloom_eye_green: 0.15,
            bloom_eye_blue: 0.05,
            bloom_eye_share: 0.3,
            ..Default::default()
        };
        let leaves = Uniforms::zeroed();
        let o = organ(
            leaves,
            &m,
            telperion_core::foliage::Reference::default(),
            0.0,
        );
        assert_eq!(o.bloom_eye, [0.2, 0.15, 0.05, 0.3]);
        assert_eq!(leaves.bloom_eye, [0.0; 4]);
    }
}
