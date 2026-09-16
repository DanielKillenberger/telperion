//! What a family's bark and leaves are made of, as numbers. Every value is a
//! row like any other trait: nothing here is a texture, a swatch name or a
//! species switch, so a walk between two families walks the material too.
//!
//! Colours are linear reflectances in 0..1, not sRGB hex: the shaders light in
//! linear and the target encodes on the way out, so a colour that arrived as a
//! swatch would be converted twice. The two ranges are OFFSETS a leaf may take
//! about its own colour - a hue offset as a fraction of the colour circle and a
//! brightness offset about one - so a range is symmetric about no change and a
//! row that states neither varies nothing.
use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialParams {
    pub bark_red: f64,
    pub bark_green: f64,
    pub bark_blue: f64,
    /// How diffuse the bark is: 0 is a mirror, 1 is chalk.
    pub bark_roughness: f64,
    /// The young wood's own colour, before its bark has formed. Wood thinner
    /// than `shoot_radius` takes it, and gives it up to the bark colour on a
    /// smoothstep of its radius by twice that; zero means no wood is young.
    pub shoot_red: f64,
    pub shoot_green: f64,
    pub shoot_blue: f64,
    /// Metres. The radius below which wood is young.
    pub shoot_radius: f64,
    pub leaf_front_red: f64,
    pub leaf_front_green: f64,
    pub leaf_front_blue: f64,
    pub leaf_back_red: f64,
    pub leaf_back_green: f64,
    pub leaf_back_blue: f64,
    /// The hue offsets one leaf may take, as a fraction of the colour circle.
    pub hue_range_low: f64,
    pub hue_range_high: f64,
    /// The brightness offsets one leaf may take, about no change at all.
    pub brightness_range_low: f64,
    pub brightness_range_high: f64,
    /// How far a leaf deep inside the crown is darkened towards a shaded mass.
    pub interior_darkening: f64,
    /// Circumferential ridge spacing in metres; zero disables relief.
    pub ridge_scale: f64,
    /// Axial scale control in metres; spacing is bounded to 1.5–2 ridge widths.
    /// Larger ratios lengthen and deepen furrows; zero omits breaks.
    pub plate_scale: f64,
    /// Furrow width and depth together; zero leaves tightly packed scales.
    pub furrow_strength: f64,
    /// Roughness variation about the base value, clamped to 0..1.
    pub roughness_detail: f64,
    /// Secondary vein pairs per blade, continuously interpolated.
    pub vein_scale: f64,
    pub vein_contrast: f64,
    pub transmission_strength: f64,
    /// Linear transmission tint, in 0..1 per channel.
    pub transmission_red: f64,
    pub transmission_green: f64,
    pub transmission_blue: f64,
    /// Optical thickness: attenuation is exp(-thickness).
    pub thickness: f64,
    pub fissure_red: f64,
    pub fissure_green: f64,
    pub fissure_blue: f64,
    pub fissure_strength: f64,
    pub crest_red: f64,
    pub crest_green: f64,
    pub crest_blue: f64,
    pub crest_strength: f64,
    pub bark_mottle_scale: f64,
    pub bark_mottle_strength: f64,
    pub cavity_strength: f64,
    pub blade_mottle_scale: f64,
    pub blade_mottle_strength: f64,
    pub margin_width: f64,
    pub margin_red: f64,
    pub margin_green: f64,
    pub margin_blue: f64,
    pub cuticle_gloss: f64,
    pub sky_occlusion_strength: f64,
    /// Circumferential size of one bark plate in metres, before girth scales
    /// it. Zero leaves the field the ridges it has always been.
    pub plate_cell_scale: f64,
    /// How much longer a plate runs than it is wide: nought is as long as it
    /// is wide, one is twice as long.
    pub plate_elongation: f64,
    /// How far a plate's face rises from its own edge towards its middle.
    pub plate_dome: f64,
    /// How far a plate's rim stands off the furrow it borders: the scale that
    /// lifts rather than the plate that sits flat.
    pub plate_edge_lift: f64,
    /// How wide the flat floor of a furrow is cut, as a fraction of a plate's
    /// own width, so a bigger plate carries a wider furrow off one row. Zero
    /// leaves the hairline the network has always cut between two faces.
    pub plate_furrow_width: f64,
    /// How much of its own a plate keeps: how proud it stands, how it leans,
    /// and the value and cast it holds against its neighbours.
    pub plate_identity: f64,
    /// How far a weathered face is greyed and tinted against a fresh furrow.
    pub weathering_strength: f64,
    pub weathering_red: f64,
    pub weathering_green: f64,
    pub weathering_blue: f64,
    /// How far the side away from the sun and the foot of the trunk take a
    /// colour of their own - what damp growth would look like, not what it is.
    pub orientation_strength: f64,
    pub orientation_red: f64,
    pub orientation_green: f64,
    pub orientation_blue: f64,
    /// How far a furrow floor is darkened by its own crest standing between it
    /// and the sun. Zero leaves the sun on both sides of every furrow alike.
    pub directional_occlusion: f64,
    /// How far the relief is given depth beyond the shaded normal.
    pub depth_strength: f64,
    /// How far a leaf is lit as part of its crown rather than as a lone card:
    /// its lighting normal bends from the blade's toward the crown's outward
    /// direction at its placement, so the sunward shell of the mass is lit
    /// whichever way its blades turn. Zero lights the blade alone.
    pub canopy_normal: f64,
    /// How far the leaf's sunlight wraps past the terminator, as a fraction
    /// of a right angle's cosine; a face square to the sun takes what it
    /// always took. Zero is the plain cosine.
    pub light_wrap: f64,
    /// The share of the blade's transmission that leaves it diffusely, as a
    /// thin leaf's does, rather than on the forward lobe toward the sun; the
    /// diffuse share also carries the sky through the blade. Zero is the lobe.
    pub diffuse_transmission: f64,
    /// The cuticle's reflectance of the sky at normal incidence, rising to
    /// the whole sky at grazing by Schlick's Fresnel; zero reflects no sky.
    pub leaf_sheen: f64,
    /// How much of the sky one crown radius of leaves takes from a leaf that
    /// reads it through the mass - the sky over it, behind it and in its
    /// sheen - so the underside of a crown falls into its own shade. Zero
    /// sees the sky through the mass.
    pub crown_shade: f64,
    /// Smooth bark's lichen: the size in metres of the cells its patches are
    /// scattered over. Zero leaves no patch anywhere.
    pub lichen_scale: f64,
    /// The share of those cells that hold a patch.
    pub lichen_coverage: f64,
    /// A patch's own colour, a linear reflectance like the bark's.
    pub lichen_red: f64,
    pub lichen_green: f64,
    pub lichen_blue: f64,
    /// How far a patch covers the bark with that colour.
    pub lichen_strength: f64,
    /// Rows of lenticel dashes per metre along the wood.
    pub lenticel_density: f64,
    /// The longest dash across the wood, in metres; the shortest is under half.
    pub lenticel_length: f64,
    /// How far a dash shows: its tint, and the shallow groove it cuts.
    pub lenticel_strength: f64,
    /// A dash's value against the bark it marks: -1 is black, 0 no change.
    pub lenticel_tint: f64,
    /// How far the plate network's strips curl away: they stretch across the
    /// wood into bands, lift at their lower edge, and this share of them has
    /// peeled off to show the inner bark.
    pub peel_curl: f64,
    /// The inner bark a peeled strip leaves showing.
    pub peel_red: f64,
    pub peel_green: f64,
    pub peel_blue: f64,
    /// How much of the sky and of what passes through the blade a leaf loses
    /// to the leaves of its own lobe standing over it, read from the crown's
    /// own placements rather than from one smooth ellipsoid: a lobe's face is
    /// lit and what hangs under it falls into its shade. Zero sees none of it.
    pub lobe_shade: f64,
}

impl Default for MaterialParams {
    /// A mid-brown bark under a mid-green blade, paler underneath, varying a
    /// little. Neither a species nor a look: the row every family starts from.
    fn default() -> Self {
        Self {
            bark_red: 0.147,
            bark_green: 0.105,
            bark_blue: 0.068,
            bark_roughness: 0.8,
            // Young wood the colour of the bark, and none of it young.
            shoot_red: 0.147,
            shoot_green: 0.105,
            shoot_blue: 0.068,
            shoot_radius: 0.0,
            leaf_front_red: 0.068,
            leaf_front_green: 0.195,
            leaf_front_blue: 0.036,
            leaf_back_red: 0.105,
            leaf_back_green: 0.240,
            leaf_back_blue: 0.070,
            hue_range_low: -0.03,
            hue_range_high: 0.03,
            brightness_range_low: -0.12,
            brightness_range_high: 0.12,
            interior_darkening: 0.5,
            ridge_scale: 0.0,
            plate_scale: 0.0,
            furrow_strength: 1.0,
            roughness_detail: 0.0,
            vein_scale: 8.0,
            vein_contrast: 0.0,
            transmission_strength: 0.0,
            transmission_red: 0.3,
            transmission_green: 0.6,
            transmission_blue: 0.1,
            thickness: 1.0,
            fissure_red: 0.0,
            fissure_green: 0.0,
            fissure_blue: 0.0,
            fissure_strength: 0.0,
            crest_red: 0.0,
            crest_green: 0.0,
            crest_blue: 0.0,
            crest_strength: 0.0,
            bark_mottle_scale: 0.0,
            bark_mottle_strength: 0.0,
            cavity_strength: 0.0,
            blade_mottle_scale: 0.0,
            blade_mottle_strength: 0.0,
            margin_width: 0.0,
            margin_red: 0.0,
            margin_green: 0.0,
            margin_blue: 0.0,
            cuticle_gloss: 0.0,
            sky_occlusion_strength: 0.0,
            plate_cell_scale: 0.0,
            plate_elongation: 0.0,
            plate_dome: 0.0,
            plate_edge_lift: 0.0,
            plate_furrow_width: 0.0,
            plate_identity: 0.0,
            weathering_strength: 0.0,
            weathering_red: 0.0,
            weathering_green: 0.0,
            weathering_blue: 0.0,
            orientation_strength: 0.0,
            orientation_red: 0.0,
            orientation_green: 0.0,
            orientation_blue: 0.0,
            directional_occlusion: 0.0,
            depth_strength: 0.0,
            canopy_normal: 0.0,
            light_wrap: 0.0,
            diffuse_transmission: 0.0,
            leaf_sheen: 0.0,
            crown_shade: 0.0,
            lichen_scale: 0.0,
            lichen_coverage: 0.0,
            lichen_red: 0.0,
            lichen_green: 0.0,
            lichen_blue: 0.0,
            lichen_strength: 0.0,
            lenticel_density: 0.0,
            lenticel_length: 0.0,
            lenticel_strength: 0.0,
            lenticel_tint: 0.0,
            peel_curl: 0.0,
            peel_red: 0.0,
            peel_green: 0.0,
            peel_blue: 0.0,
            lobe_shade: 0.0,
        }
    }
}

impl MaterialParams {
    /// Every field by name, so a refusal says which one was wrong rather than
    /// that the material was. A range whose low end is above its high end is
    /// refused by the pair's name: no leaf could be drawn from it.
    pub fn validate(&self) -> Result<()> {
        for (value, low, high, name) in [
            (self.fissure_red, -1.0, 1.0, "bark fissure red"),
            (self.fissure_green, -1.0, 1.0, "bark fissure green"),
            (self.fissure_blue, -1.0, 1.0, "bark fissure blue"),
            (self.fissure_strength, 0.0, 1.0, "bark fissure strength"),
            (self.crest_red, -1.0, 1.0, "bark crest red"),
            (self.crest_green, -1.0, 1.0, "bark crest green"),
            (self.crest_blue, -1.0, 1.0, "bark crest blue"),
            (self.crest_strength, 0.0, 1.0, "bark crest strength"),
            (self.bark_mottle_scale, 0.0, 8.0, "bark mottle scale"),
            (self.bark_mottle_strength, 0.0, 1.0, "bark mottle strength"),
            (self.cavity_strength, 0.0, 1.0, "bark cavity strength"),
            (
                self.blade_mottle_scale,
                0.0,
                32.0,
                "leaf blade mottle scale",
            ),
            (
                self.blade_mottle_strength,
                0.0,
                1.0,
                "leaf blade mottle strength",
            ),
            (self.margin_width, 0.0, 0.5, "leaf margin width"),
            (self.margin_red, -1.0, 1.0, "leaf margin red"),
            (self.margin_green, -1.0, 1.0, "leaf margin green"),
            (self.margin_blue, -1.0, 1.0, "leaf margin blue"),
            (self.cuticle_gloss, 0.0, 1.0, "leaf cuticle gloss"),
            (
                self.sky_occlusion_strength,
                0.0,
                1.0,
                "sky occlusion strength",
            ),
            (self.plate_cell_scale, 0.0, 1.0, "bark plate cell scale"),
            (self.plate_elongation, 0.0, 16.0, "bark plate elongation"),
            (self.plate_dome, 0.0, 1.0, "bark plate dome"),
            (self.plate_edge_lift, 0.0, 1.0, "bark plate edge lift"),
            (self.plate_furrow_width, 0.0, 1.0, "bark plate furrow width"),
            (self.plate_identity, 0.0, 1.0, "bark plate identity"),
            (
                self.weathering_strength,
                0.0,
                1.0,
                "bark weathering strength",
            ),
            (self.weathering_red, -1.0, 1.0, "bark weathering red"),
            (self.weathering_green, -1.0, 1.0, "bark weathering green"),
            (self.weathering_blue, -1.0, 1.0, "bark weathering blue"),
            (
                self.orientation_strength,
                0.0,
                1.0,
                "bark orientation strength",
            ),
            (self.orientation_red, -1.0, 1.0, "bark orientation red"),
            (self.orientation_green, -1.0, 1.0, "bark orientation green"),
            (self.orientation_blue, -1.0, 1.0, "bark orientation blue"),
            (
                self.directional_occlusion,
                0.0,
                1.0,
                "bark directional occlusion",
            ),
            (self.depth_strength, 0.0, 1.0, "bark depth strength"),
            (self.canopy_normal, 0.0, 1.0, "leaf canopy normal"),
            (self.light_wrap, 0.0, 1.0, "leaf light wrap"),
            (
                self.diffuse_transmission,
                0.0,
                1.0,
                "leaf diffuse transmission",
            ),
            (self.leaf_sheen, 0.0, 0.5, "leaf sheen"),
            (self.crown_shade, 0.0, 1.0, "leaf crown shade"),
            (self.lichen_scale, 0.0, 1.0, "bark lichen scale"),
            (self.lichen_coverage, 0.0, 1.0, "bark lichen coverage"),
            (self.lichen_red, 0.0, 1.0, "bark lichen red"),
            (self.lichen_green, 0.0, 1.0, "bark lichen green"),
            (self.lichen_blue, 0.0, 1.0, "bark lichen blue"),
            (self.lichen_strength, 0.0, 1.0, "bark lichen strength"),
            (self.lenticel_density, 0.0, 400.0, "bark lenticel density"),
            (self.lenticel_length, 0.0, 0.5, "bark lenticel length"),
            (self.lenticel_strength, 0.0, 1.0, "bark lenticel strength"),
            (self.lenticel_tint, -1.0, 1.0, "bark lenticel tint"),
            (self.peel_curl, 0.0, 1.0, "bark peel curl"),
            (self.peel_red, 0.0, 1.0, "bark peel red"),
            (self.peel_green, 0.0, 1.0, "bark peel green"),
            (self.peel_blue, 0.0, 1.0, "bark peel blue"),
            (self.lobe_shade, 0.0, 1.0, "leaf lobe shade"),
            (self.ridge_scale, 0.0, 1.0, "bark ridge scale"),
            (self.plate_scale, 0.0, 1.0, "bark plate scale"),
            (self.furrow_strength, 0.0, 1.0, "bark furrow strength"),
            (self.roughness_detail, 0.0, 1.0, "bark roughness detail"),
            (self.vein_scale, 0.0, 32.0, "leaf vein scale"),
            (self.vein_contrast, 0.0, 1.0, "leaf vein contrast"),
            (
                self.transmission_strength,
                0.0,
                1.0,
                "leaf transmission strength",
            ),
            (self.transmission_red, 0.0, 1.0, "leaf transmission red"),
            (self.transmission_green, 0.0, 1.0, "leaf transmission green"),
            (self.transmission_blue, 0.0, 1.0, "leaf transmission blue"),
            (self.thickness, 0.0, 8.0, "leaf thickness"),
            (self.bark_red, 0.0, 1.0, "bark red"),
            (self.bark_green, 0.0, 1.0, "bark green"),
            (self.bark_blue, 0.0, 1.0, "bark blue"),
            (self.bark_roughness, 0.0, 1.0, "bark roughness"),
            (self.shoot_red, 0.0, 1.0, "young shoot red"),
            (self.shoot_green, 0.0, 1.0, "young shoot green"),
            (self.shoot_blue, 0.0, 1.0, "young shoot blue"),
            (self.shoot_radius, 0.0, 0.1, "young shoot radius"),
            (self.leaf_front_red, 0.0, 1.0, "leaf front red"),
            (self.leaf_front_green, 0.0, 1.0, "leaf front green"),
            (self.leaf_front_blue, 0.0, 1.0, "leaf front blue"),
            (self.leaf_back_red, 0.0, 1.0, "leaf back red"),
            (self.leaf_back_green, 0.0, 1.0, "leaf back green"),
            (self.leaf_back_blue, 0.0, 1.0, "leaf back blue"),
            (self.hue_range_low, -0.5, 0.5, "leaf hue range low"),
            (self.hue_range_high, -0.5, 0.5, "leaf hue range high"),
            (
                self.brightness_range_low,
                -1.0,
                1.0,
                "leaf brightness range low",
            ),
            (
                self.brightness_range_high,
                -1.0,
                1.0,
                "leaf brightness range high",
            ),
            (self.interior_darkening, 0.0, 1.0, "leaf interior darkening"),
        ] {
            if !value.is_finite() || value < low || value > high {
                return Err(Error::InvalidInput(name));
            }
        }
        for (low, high, name) in [
            (self.hue_range_low, self.hue_range_high, "leaf hue range"),
            (
                self.brightness_range_low,
                self.brightness_range_high,
                "leaf brightness range",
            ),
        ] {
            if low > high {
                return Err(Error::InvalidInput(name));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
