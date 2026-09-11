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
        }
    }
}

impl MaterialParams {
    /// Every field by name, so a refusal says which one was wrong rather than
    /// that the material was. A range whose low end is above its high end is
    /// refused by the pair's name: no leaf could be drawn from it.
    pub fn validate(&self) -> Result<()> {
        for (value, low, high, name) in [
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
        let refusals: [(fn(&mut MaterialParams), &str); 15] = [
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
