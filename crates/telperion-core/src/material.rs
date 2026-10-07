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
mod params;
use crate::{catalogue::Site, Error, Result};
pub use params::MaterialParams;

impl MaterialParams {
    /// Every field by name, so a refusal says which one was wrong rather than
    /// that the material was. A range whose low end is above its high end is
    /// refused by the pair's name: no leaf could be drawn from it.
    pub fn validate(&self) -> Result<()> {
        crate::catalogue::check(Self::CHECKS, self, Site::Material)?;
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
