//! The scene row: where the sun stands, what colour it is, and what the sky
//! and the ground are. It is not a property of a tree - two trees under one
//! sun share it - so it lives on the renderer with a default, beside the view,
//! rather than in a family. It never blends: a walk between two trees walks
//! their material rows under one unchanged sky.
//!
//! Every value is a number and the row is a closed schema, so the panel reaches
//! it through the same generic numeric controls a family's traits go through,
//! and a name nobody has is refused rather than ignored. Colours are linear:
//! the sun and the sky are radiances and may carry intensity above one, the
//! ground is a surface colour and may not.
use crate::device::{RenderError, Result};

/// The row's fields in one place, each with the name it takes on the wire, the
/// range it holds and the words a refusal names it by. Every operation over the
/// row - reading it, writing it, judging it - walks this one table.
macro_rules! fields {
    ($op:ident) => {
        $op! {
            ("sunAzimuth", sun_azimuth, 0.0, 360.0, 135.0, "sun azimuth"),
            ("sunElevation", sun_elevation, 0.0, 90.0, 55.0, "sun elevation"),
            ("sunRed", sun_red, 0.0, 10.0, 3.0, "sun red"),
            ("sunGreen", sun_green, 0.0, 10.0, 2.85, "sun green"),
            ("sunBlue", sun_blue, 0.0, 10.0, 2.6, "sun blue"),
            ("skyZenithRed", sky_zenith_red, 0.0, 10.0, 0.18, "sky zenith red"),
            ("skyZenithGreen", sky_zenith_green, 0.0, 10.0, 0.30, "sky zenith green"),
            ("skyZenithBlue", sky_zenith_blue, 0.0, 10.0, 0.62, "sky zenith blue"),
            ("skyHorizonRed", sky_horizon_red, 0.0, 10.0, 0.55, "sky horizon red"),
            ("skyHorizonGreen", sky_horizon_green, 0.0, 10.0, 0.66, "sky horizon green"),
            ("skyHorizonBlue", sky_horizon_blue, 0.0, 10.0, 0.80, "sky horizon blue"),
            ("groundRed", ground_red, 0.0, 1.0, 0.11, "ground red"),
            ("groundGreen", ground_green, 0.0, 1.0, 0.12, "ground green"),
            ("groundBlue", ground_blue, 0.0, 1.0, 0.07, "ground blue"),
            ("casterTexels", caster_texels, 0.0, 8.0, 1.0, "casterTexels"),
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

    #[test]
    fn the_row_survives_its_own_json_in_both_directions() {
        let row = SceneRow {
            sun_azimuth: 12.5,
            ground_green: 0.4,
            ..Default::default()
        };
        assert_eq!(SceneRow::parse(&row.to_json()).unwrap(), row);
        let unknown = SceneRow::parse(r#"{"unknown":0}"#).unwrap_err().to_string();
        for name in [
            "casterTexels",
            "casterStride",
            "shadowFilterTexels",
            "shadowNormalOffset",
        ] {
            assert!(row.to_json().contains(name));
            assert!(unknown.contains(name));
        }
        // A set that names two fields moves two and leaves the rest alone.
        let partial = SceneRow::parse(r#"{"sunElevation":20.0,"skyZenithRed":0.4}"#).unwrap();
        assert_eq!(partial.sun_elevation, 20.0);
        assert_eq!(partial.sky_zenith_red, 0.4);
        assert_eq!(partial.sun_azimuth, SceneRow::default().sun_azimuth);
    }

    #[test]
    fn a_row_the_renderer_will_not_have_says_what_was_wrong_with_it() {
        for (text, expected) in [
            (r#"{"sunElevation":120.0}"#, "sun elevation"),
            (r#"{"groundRed":4.0}"#, "ground red"),
            (r#"{"casterTexels":8.1}"#, "casterTexels"),
            (r#"{"casterStride":0}"#, "casterStride"),
            (r#"{"shadowFilterTexels":3.1}"#, "shadowFilterTexels"),
            (r#"{"shadowNormalOffset":-0.1}"#, "shadowNormalOffset"),
            (r#"{"sunRed":-1.0}"#, "sun red"),
            (r#"{"skyZenithBlue":"blue"}"#, "sky zenith blue"),
            (r#"{"sunHeight":10.0}"#, "sunHeight"),
            ("not json at all", "not JSON"),
            ("[1,2,3]", "not an object"),
        ] {
            let error = SceneRow::parse(text).expect_err("the row was taken anyway");
            let said = error.to_string();
            assert!(said.contains(expected), "{said} does not name {expected}");
        }
    }
}
