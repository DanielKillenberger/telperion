//! The scene row: where the sun stands, what colour it is, and what the sky
//! and the ground are, and how its casters and comparison are coarsened.
//! It is not a property of a tree - two trees under one
//! sun share it - so it lives on the renderer with a default, beside the view,
//! rather than in a family. It never blends: a walk between two trees walks
//! their material rows under one unchanged sky.
//!
//! Every value is a number and the row is a closed schema, so the panel reaches
//! it through the same generic numeric controls a family's traits go through,
//! and a name nobody has is refused rather than ignored. Colours are linear:
//! the sun and the sky are radiances and may carry intensity above one, the
//! ground is a surface colour and may not. Caster stride and filter radius
//! are used as whole counts; the threshold and normal offset are in texels.
//!
//! The default is the owner's clear neutral day (fn-310 D1, variant C): a sky
//! with a glow round the sun in the sun's colour, the glow's strength a
//! multiple of the sun; a ground turned in broad patches toward its dry
//! colour by the variation's share; and far ground fading into the horizon
//! over the haze distance in metres. Each at zero is off.
use crate::device::{RenderError, Result};

/// The row's fields in one place, each with the name it takes on the wire, the
/// range it holds and the words a refusal names it by. Every operation over the
/// row - reading it, writing it, judging it - walks this one table.
macro_rules! fields {
    ($op:ident) => {
        $op! {
            ("sunAzimuth", sun_azimuth, 0.0, 360.0, 135.0, "sun azimuth"),
            ("sunElevation", sun_elevation, 0.0, 90.0, 30.0, "sun elevation"),
            ("sunRed", sun_red, 0.0, 10.0, 3.064, "sun red"),
            ("sunGreen", sun_green, 0.0, 10.0, 2.854, "sun green"),
            ("sunBlue", sun_blue, 0.0, 10.0, 2.374, "sun blue"),
            ("skyZenithRed", sky_zenith_red, 0.0, 10.0, 0.11, "sky zenith red"),
            ("skyZenithGreen", sky_zenith_green, 0.0, 10.0, 0.22, "sky zenith green"),
            ("skyZenithBlue", sky_zenith_blue, 0.0, 10.0, 0.56, "sky zenith blue"),
            ("skyHorizonRed", sky_horizon_red, 0.0, 10.0, 0.60, "sky horizon red"),
            ("skyHorizonGreen", sky_horizon_green, 0.0, 10.0, 0.70, "sky horizon green"),
            ("skyHorizonBlue", sky_horizon_blue, 0.0, 10.0, 0.84, "sky horizon blue"),
            ("skyGlow", sky_glow, 0.0, 4.0, 0.6, "sky glow"),
            ("groundRed", ground_red, 0.0, 1.0, 0.10, "ground red"),
            ("groundGreen", ground_green, 0.0, 1.0, 0.12, "ground green"),
            ("groundBlue", ground_blue, 0.0, 1.0, 0.065, "ground blue"),
            ("groundDryRed", ground_dry_red, 0.0, 1.0, 0.145, "ground dry red"),
            ("groundDryGreen", ground_dry_green, 0.0, 1.0, 0.15, "ground dry green"),
            ("groundDryBlue", ground_dry_blue, 0.0, 1.0, 0.075, "ground dry blue"),
            ("groundVariation", ground_variation, 0.0, 1.0, 0.5, "ground variation"),
            ("hazeDistance", haze_distance, 0.0, 5000.0, 500.0, "haze distance"),
            ("casterStride", caster_stride, 1.0, 64.0, 4.0, "casterStride"),
            ("shadowFilterTexels", shadow_filter_texels, 0.0, 3.0, 1.0, "shadowFilterTexels"),
            ("shadowNormalOffset", shadow_normal_offset, 0.0, 4.0, 1.0, "shadowNormalOffset"),
        }
    };
}

macro_rules! define {
    ($(($wire:literal, $field:ident, $low:literal, $high:literal, $default:literal, $name:literal)),* $(,)?) => {
        /// The sun, sky, ground and shadow coarsening under which a tree stands.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct SceneRow { $(pub $field: f64,)* }
        impl Default for SceneRow {
            fn default() -> Self { Self { $($field: $default,)* } }
        }
    };
}
fields!(define);

impl SceneRow {
    /// Every field by name, so a refusal says which one was wrong and what it
    /// would have taken instead.
    pub fn validate(&self) -> Result<()> {
        macro_rules! judge {
            ($(($wire:literal, $field:ident, $low:literal, $high:literal, $default:literal, $name:literal)),* $(,)?) => { $(
                let value = self.$field;
                if !value.is_finite() || value < $low || value > $high {
                    return Err(RenderError::Scene(format!(
                        "{} wants {} to {}, not {value}",
                        $name, $low, $high
                    )));
                }
            )* };
        }
        fields!(judge);
        Ok(())
    }

    /// The row as the JSON both the command line and the page hand it over as,
    /// and as the panel reads the default back out of.
    pub fn to_json(&self) -> String {
        let mut object = serde_json::Map::new();
        macro_rules! emit {
            ($(($wire:literal, $field:ident, $low:literal, $high:literal, $default:literal, $name:literal)),* $(,)?) => { $(
                object.insert($wire.into(), serde_json::json!(self.$field));
            )* };
        }
        fields!(emit);
        serde_json::Value::Object(object).to_string()
    }

    /// The row this JSON states, read against the default: a set that names
    /// three fields states three and takes the default for the rest, so one
    /// text is one whole sky and never a patch on whatever was set before. A
    /// field the row has no name for is refused rather than ignored, so a typo
    /// in a flag or a page call is a message and not a picture nobody asked
    /// for.
    pub fn parse(text: &str) -> Result<Self> {
        let refused = |detail: String| RenderError::Scene(detail);
        let value: serde_json::Value = serde_json::from_str(text)
            .map_err(|error| refused(format!("this is not JSON: {error}")))?;
        let object = value
            .as_object()
            .ok_or_else(|| refused("this is not an object of numbers".into()))?;
        let mut row = Self::default();
        let mut known = Vec::new();
        macro_rules! read {
            ($(($wire:literal, $field:ident, $low:literal, $high:literal, $default:literal, $name:literal)),* $(,)?) => { $(
                known.push($wire);
                if let Some(stated) = object.get($wire) {
                    row.$field = stated.as_f64().ok_or_else(|| {
                        refused(format!("{} wants a number, not {stated}", $name))
                    })?;
                }
            )* };
        }
        fields!(read);
        for name in object.keys() {
            if !known.contains(&name.as_str()) {
                return Err(refused(format!(
                    "there is no scene field \"{name}\"; one of {}",
                    known.join(", ")
                )));
            }
        }
        row.validate()?;
        Ok(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_is_a_row_the_renderer_will_have() {
        assert!(SceneRow::default().validate().is_ok());
        // Midday: the sun is up, and the sky pales towards the horizon.
        let row = SceneRow::default();
        assert!(row.sun_elevation > 0.0);
        assert!(row.sky_horizon_blue > row.sky_zenith_blue);
    }

    /// The default sun is tinted so the full daylight a sunlit level floor
    /// takes, the sun at its elevation plus the sky over it, is white, as a
    /// camera balanced to daylight sees it (host, fn-228). Shade, the sky
    /// alone, stays cool. The sun keeps the luminance of the (3, 2.85, 2.6)
    /// it replaced.
    #[test]
    fn the_default_daylight_is_white_at_the_luminance_it_had() {
        let row = SceneRow::default();
        let luminance = |c: [f64; 3]| 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
        let sun = [row.sun_red, row.sun_green, row.sun_blue];
        let sky = [
            0.5 * (row.sky_zenith_red + row.sky_horizon_red),
            0.5 * (row.sky_zenith_green + row.sky_horizon_green),
            0.5 * (row.sky_zenith_blue + row.sky_horizon_blue),
        ];
        let rise = row.sun_elevation.to_radians().sin();
        let daylight: Vec<f64> = (0..3).map(|i| sky[i] + rise * sun[i]).collect();
        let mean = daylight.iter().sum::<f64>() / 3.0;
        for channel in &daylight {
            assert!((channel / mean - 1.0).abs() < 1e-3, "daylight {daylight:?}");
        }
        assert!((luminance(sun) - luminance([3.0, 2.85, 2.6])).abs() < 1e-3);
        assert!(sky[2] > sky[0], "the sky light was made grey");
    }

    #[test]
    fn the_row_survives_its_own_json_in_both_directions() {
        let row = SceneRow {
            sun_azimuth: 12.5,
            ground_green: 0.4,
            ground_dry_blue: 0.3,
            ground_variation: 0.25,
            sky_glow: 1.5,
            haze_distance: 1200.0,
            ..Default::default()
        };
        assert_eq!(SceneRow::parse(&row.to_json()).unwrap(), row);
        let unknown = SceneRow::parse(r#"{"unknown":0}"#).unwrap_err().to_string();
        for name in [
            "casterStride",
            "shadowFilterTexels",
            "shadowNormalOffset",
            "groundDryRed",
            "groundDryGreen",
            "groundDryBlue",
            "groundVariation",
            "skyGlow",
            "hazeDistance",
        ] {
            assert!(row.to_json().contains(name));
            assert!(unknown.contains(name));
        }
        // A set that names two fields moves two and leaves the rest alone.
        let partial = SceneRow::parse(r#"{"sunElevation":20.0,"skyZenithRed":0.4}"#).unwrap();
        assert_eq!(partial.sun_elevation, 20.0);
        assert_eq!(partial.sky_zenith_red, 0.4);
        assert_eq!(partial.sun_azimuth, SceneRow::default().sun_azimuth);
        // A row written before the glow, the dry ground and the haze takes
        // the default's, which is the chosen look and not none of it.
        let d = SceneRow::default();
        assert_eq!(
            (
                partial.sky_glow,
                partial.ground_variation,
                partial.haze_distance
            ),
            (d.sky_glow, d.ground_variation, d.haze_distance)
        );
        assert_eq!(
            [
                partial.ground_dry_red,
                partial.ground_dry_green,
                partial.ground_dry_blue
            ],
            [d.ground_dry_red, d.ground_dry_green, d.ground_dry_blue]
        );
        // An explicit zero is taken as stated: it is how an effect is off.
        let off = SceneRow::parse(r#"{"skyGlow":0,"groundVariation":0,"hazeDistance":0}"#);
        let off = off.unwrap();
        assert_eq!(
            (off.sky_glow, off.ground_variation, off.haze_distance),
            (0.0, 0.0, 0.0)
        );
    }

    /// The owner's pick (fn-310 D1): variant C's sky, ground, dry ground,
    /// variation, glow and haze, under the sun position the default always
    /// had.
    #[test]
    fn the_default_is_the_clear_neutral_day() {
        let d = SceneRow::default();
        assert_eq!((d.sun_azimuth, d.sun_elevation), (135.0, 30.0));
        assert_eq!(
            [d.sky_zenith_red, d.sky_zenith_green, d.sky_zenith_blue],
            [0.11, 0.22, 0.56]
        );
        assert_eq!(
            [d.sky_horizon_red, d.sky_horizon_green, d.sky_horizon_blue],
            [0.6, 0.7, 0.84]
        );
        assert_eq!(
            [d.ground_red, d.ground_green, d.ground_blue],
            [0.1, 0.12, 0.065]
        );
        assert_eq!(
            [d.ground_dry_red, d.ground_dry_green, d.ground_dry_blue],
            [0.145, 0.15, 0.075]
        );
        assert_eq!(
            (d.ground_variation, d.sky_glow, d.haze_distance),
            (0.5, 0.6, 500.0)
        );
    }

    #[test]
    fn a_row_the_renderer_will_not_have_says_what_was_wrong_with_it() {
        for (text, expected) in [
            (r#"{"sunElevation":120.0}"#, "sun elevation"),
            (r#"{"groundRed":4.0}"#, "ground red"),
            (r#"{"casterStride":0}"#, "casterStride"),
            (r#"{"shadowFilterTexels":3.1}"#, "shadowFilterTexels"),
            (r#"{"shadowNormalOffset":-0.1}"#, "shadowNormalOffset"),
            (r#"{"sunRed":-1.0}"#, "sun red"),
            (r#"{"skyZenithBlue":"blue"}"#, "sky zenith blue"),
            (r#"{"sunHeight":10.0}"#, "sunHeight"),
            (r#"{"groundDryRed":1.5}"#, "ground dry red"),
            (r#"{"groundDryGreen":-0.1}"#, "ground dry green"),
            (r#"{"groundDryBlue":2.0}"#, "ground dry blue"),
            (r#"{"groundVariation":1.1}"#, "ground variation"),
            (r#"{"skyGlow":-0.5}"#, "sky glow"),
            (r#"{"skyGlow":4.5}"#, "sky glow"),
            (r#"{"hazeDistance":-1}"#, "haze distance"),
            (r#"{"hazeDistance":6000}"#, "haze distance"),
            ("not json at all", "not JSON"),
            ("[1,2,3]", "not an object"),
        ] {
            let error = SceneRow::parse(text).expect_err("the row was taken anyway");
            let said = error.to_string();
            assert!(said.contains(expected), "{said} does not name {expected}");
        }
    }
}
