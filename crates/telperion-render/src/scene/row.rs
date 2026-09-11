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
        $op!("sunAzimuth", sun_azimuth, 0.0, 360.0, "sun azimuth");
        $op!("sunElevation", sun_elevation, 0.0, 90.0, "sun elevation");
        $op!("sunRed", sun_red, 0.0, 10.0, "sun red");
        $op!("sunGreen", sun_green, 0.0, 10.0, "sun green");
        $op!("sunBlue", sun_blue, 0.0, 10.0, "sun blue");
        $op!("skyZenithRed", sky_zenith_red, 0.0, 10.0, "sky zenith red");
        $op!(
            "skyZenithGreen",
            sky_zenith_green,
            0.0,
            10.0,
            "sky zenith green"
        );
        $op!(
            "skyZenithBlue",
            sky_zenith_blue,
            0.0,
            10.0,
            "sky zenith blue"
        );
        $op!(
            "skyHorizonRed",
            sky_horizon_red,
            0.0,
            10.0,
            "sky horizon red"
        );
        $op!(
            "skyHorizonGreen",
            sky_horizon_green,
            0.0,
            10.0,
            "sky horizon green"
        );
        $op!(
            "skyHorizonBlue",
            sky_horizon_blue,
            0.0,
            10.0,
            "sky horizon blue"
        );
        $op!("groundRed", ground_red, 0.0, 1.0, "ground red");
        $op!("groundGreen", ground_green, 0.0, 1.0, "ground green");
        $op!("groundBlue", ground_blue, 0.0, 1.0, "ground blue");
    };
}

/// Degrees clockwise from north and degrees above the horizon for the sun;
/// linear colours for everything the frame is lit by and stands on.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SceneRow {
    pub sun_azimuth: f64,
    pub sun_elevation: f64,
    pub sun_red: f64,
    pub sun_green: f64,
    pub sun_blue: f64,
    pub sky_zenith_red: f64,
    pub sky_zenith_green: f64,
    pub sky_zenith_blue: f64,
    pub sky_horizon_red: f64,
    pub sky_horizon_green: f64,
    pub sky_horizon_blue: f64,
    pub ground_red: f64,
    pub ground_green: f64,
    pub ground_blue: f64,
}

impl Default for SceneRow {
    /// Outdoors at midday: a high sun a little off the camera's shoulder, warm
    /// against a blue zenith that pales towards the horizon, over dry ground.
    fn default() -> Self {
        Self {
            sun_azimuth: 135.0,
            sun_elevation: 55.0,
            sun_red: 3.0,
            sun_green: 2.85,
            sun_blue: 2.6,
            sky_zenith_red: 0.18,
            sky_zenith_green: 0.30,
            sky_zenith_blue: 0.62,
            sky_horizon_red: 0.55,
            sky_horizon_green: 0.66,
            sky_horizon_blue: 0.80,
            ground_red: 0.11,
            ground_green: 0.12,
            ground_blue: 0.07,
        }
    }
}

impl SceneRow {
    /// Every field by name, so a refusal says which one was wrong and what it
    /// would have taken instead.
    pub fn validate(&self) -> Result<()> {
        macro_rules! judge {
            ($wire:literal, $field:ident, $low:literal, $high:literal, $name:literal) => {
                let value = self.$field;
                if !value.is_finite() || value < $low || value > $high {
                    return Err(RenderError::Scene(format!(
                        "{} wants {} to {}, not {value}",
                        $name, $low, $high
                    )));
                }
            };
        }
        fields!(judge);
        Ok(())
    }

    /// The row as the JSON both the command line and the page hand it over as,
    /// and as the panel reads the default back out of.
    pub fn to_json(&self) -> String {
        let mut object = serde_json::Map::new();
        macro_rules! emit {
            ($wire:literal, $field:ident, $low:literal, $high:literal, $name:literal) => {
                object.insert($wire.into(), serde_json::json!(self.$field));
            };
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
            ($wire:literal, $field:ident, $low:literal, $high:literal, $name:literal) => {
                known.push($wire);
                if let Some(stated) = object.get($wire) {
                    row.$field = stated.as_f64().ok_or_else(|| {
                        refused(format!("{} wants a number, not {stated}", $name))
                    })?;
                }
            };
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
