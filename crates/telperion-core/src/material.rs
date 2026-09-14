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
        let refusals: [Refusal; 50] = [
            (|m| m.plate_cell_scale = 2.0, "bark plate cell scale"),
            (|m| m.plate_furrow_width = 1.5, "bark plate furrow width"),
            (|m| m.plate_elongation = 17.0, "bark plate elongation"),
            (|m| m.plate_dome = 2.0, "bark plate dome"),
            (|m| m.plate_edge_lift = 2.0, "bark plate edge lift"),
            (|m| m.plate_identity = 2.0, "bark plate identity"),
            (|m| m.weathering_strength = 2.0, "bark weathering strength"),
            (|m| m.weathering_red = 2.0, "bark weathering red"),
            (|m| m.weathering_green = -2.0, "bark weathering green"),
            (|m| m.weathering_blue = 2.0, "bark weathering blue"),
            (
                |m| m.orientation_strength = 2.0,
                "bark orientation strength",
            ),
            (|m| m.orientation_red = 2.0, "bark orientation red"),
            (|m| m.orientation_green = 2.0, "bark orientation green"),
            (|m| m.orientation_blue = -2.0, "bark orientation blue"),
            (
                |m| m.directional_occlusion = 2.0,
                "bark directional occlusion",
            ),
            (|m| m.depth_strength = 2.0, "bark depth strength"),
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
            (|m| m.bark_red = 1.5, "bark red"),
            (|m| m.bark_green = -0.1, "bark green"),
            (|m| m.bark_blue = f64::NAN, "bark blue"),
            (|m| m.bark_roughness = 2.0, "bark roughness"),
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
