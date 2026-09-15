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
            canopy_normal: 0.0,
            light_wrap: 0.0,
            diffuse_transmission: 0.0,
            leaf_sheen: 0.0,
            crown_shade: 0.0,
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
mod tests {
    use super::*;

    #[test]
    fn the_row_every_family_starts_from_is_one_a_leaf_can_be_drawn_from() {
        assert_eq!(MaterialParams::default().validate(), Ok(()));
    }

    #[test]
    fn a_value_off_its_range_is_refused_by_its_own_name() {
        // One field at a time, so the name in the refusal is the only name it
        // could have come from. Every field is covered: a row that grew a
        // field without a bound would fail the count below.
        // One field put off its range, and the name the refusal must carry.
        type Refusal = (fn(&mut MaterialParams), &'static str);
        let refusals: [Refusal; 43] = [
            (|m| m.fissure_red = 2.0, "bark fissure red"),
            (|m| m.fissure_green = 2.0, "bark fissure green"),
            (|m| m.fissure_blue = 2.0, "bark fissure blue"),
            (|m| m.fissure_strength = 2.0, "bark fissure strength"),
            (|m| m.crest_red = 2.0, "bark crest red"),
            (|m| m.crest_green = 2.0, "bark crest green"),
            (|m| m.crest_blue = 2.0, "bark crest blue"),
            (|m| m.crest_strength = 2.0, "bark crest strength"),
            (|m| m.bark_mottle_scale = 9.0, "bark mottle scale"),
            (|m| m.bark_mottle_strength = 2.0, "bark mottle strength"),
            (|m| m.cavity_strength = 2.0, "bark cavity strength"),
            (|m| m.blade_mottle_scale = 33.0, "leaf blade mottle scale"),
            (
                |m| m.blade_mottle_strength = 2.0,
                "leaf blade mottle strength",
            ),
            (|m| m.margin_width = 1.5, "leaf margin width"),
            (|m| m.margin_red = 2.0, "leaf margin red"),
            (|m| m.margin_green = 2.0, "leaf margin green"),
            (|m| m.margin_blue = 2.0, "leaf margin blue"),
            (|m| m.cuticle_gloss = 2.0, "leaf cuticle gloss"),
            (|m| m.sky_occlusion_strength = 2.0, "sky occlusion strength"),
            (|m| m.canopy_normal = 1.1, "leaf canopy normal"),
            (|m| m.light_wrap = -0.1, "leaf light wrap"),
            (
                |m| m.diffuse_transmission = f64::NAN,
                "leaf diffuse transmission",
            ),
            (|m| m.leaf_sheen = 0.51, "leaf sheen"),
            (|m| m.crown_shade = 1.5, "leaf crown shade"),
            (|m| m.bark_red = 1.5, "bark red"),
            (|m| m.bark_green = -0.1, "bark green"),
            (|m| m.bark_blue = f64::NAN, "bark blue"),
            (|m| m.bark_roughness = 2.0, "bark roughness"),
            (|m| m.shoot_red = -0.1, "young shoot red"),
            (|m| m.shoot_green = 1.1, "young shoot green"),
            (|m| m.shoot_blue = f64::NAN, "young shoot blue"),
            (|m| m.shoot_radius = 0.101, "young shoot radius"),
            (|m| m.leaf_front_red = -1.0, "leaf front red"),
            (|m| m.leaf_front_green = 1.2, "leaf front green"),
            (|m| m.leaf_front_blue = f64::INFINITY, "leaf front blue"),
            (|m| m.leaf_back_red = 3.0, "leaf back red"),
            (|m| m.leaf_back_green = -0.2, "leaf back green"),
            (|m| m.leaf_back_blue = 1.000_1, "leaf back blue"),
            (|m| m.hue_range_low = -0.7, "leaf hue range low"),
            (|m| m.hue_range_high = 0.7, "leaf hue range high"),
            (
                |m| m.brightness_range_low = -1.5,
                "leaf brightness range low",
            ),
            (
                |m| m.brightness_range_high = 1.5,
                "leaf brightness range high",
            ),
            (|m| m.interior_darkening = 1.1, "leaf interior darkening"),
        ];
        for (break_it, name) in refusals {
            let mut row = MaterialParams::default();
            break_it(&mut row);
            assert_eq!(row.validate(), Err(Error::InvalidInput(name)));
        }
    }

    #[test]
    fn a_range_that_runs_backwards_is_refused_by_the_pairs_name() {
        let mut hue = MaterialParams::default();
        (hue.hue_range_low, hue.hue_range_high) = (0.2, -0.2);
        assert_eq!(hue.validate(), Err(Error::InvalidInput("leaf hue range")));
        let mut brightness = MaterialParams::default();
        (
            brightness.brightness_range_low,
            brightness.brightness_range_high,
        ) = (0.3, 0.1);
        assert_eq!(
            brightness.validate(),
            Err(Error::InvalidInput("leaf brightness range"))
        );
        // A range of no width is a leaf that varies not at all, which is a
        // family with one leaf colour and not an error.
        let mut flat = MaterialParams::default();
        (flat.hue_range_low, flat.hue_range_high) = (0.0, 0.0);
        assert_eq!(flat.validate(), Ok(()));
    }
}
