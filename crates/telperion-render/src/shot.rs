//! An authored shot: where the camera stands and how the subject fills the
//! frame, stated as numbers so a still can imitate a photograph. The hero
//! pose authors only a direction and solves the rest; a shot authors the
//! direction, the fill, the aim and the lens and still solves the distance,
//! so a stated fill means the same thing on every seed and every aspect.
//!
//! The row is closed the way the scene row is: every field is a number, a
//! name nobody has is refused, and a text that names two fields states two
//! and takes the default for the rest. The default shot is the hero pose.
use crate::device::{RenderError, Result};

/// The row's fields in one place: wire name, field, range, default, and the
/// words a refusal names it by. Azimuth is degrees clockwise from +Z seen
/// from above and elevation is the eye's angle above the point it looks at,
/// the same angles the walk reads off a pose. Fill is the crown's height as a
/// fraction of the frame's; above one the crown overflows the frame, which is
/// what a close-up of the base is. The aim is a fraction of the subject's
/// height, from its base at zero to its top at one. A distance in metres
/// above zero stands the eye there instead of solving it from the fill, which
/// is how a photograph taken from an arm's length of bark is imitated.
macro_rules! fields {
    ($op:ident) => {
        $op! {
            ("azimuth", azimuth, 0.0, 360.0, 31.799, "azimuth"),
            ("elevation", elevation, -20.0, 80.0, 13.415, "elevation"),
            ("fill", fill, 0.2, 20.0, 0.8696, "fill"),
            ("targetHeight", target_height, 0.0, 1.0, 0.5, "targetHeight"),
            ("fov", fov, 10.0, 90.0, 38.0, "fov"),
            ("distance", distance, 0.0, 1000.0, 0.0, "distance"),
        }
    };
}

macro_rules! define {
    ($(($wire:literal, $field:ident, $low:literal, $high:literal, $default:literal, $name:literal)),* $(,)?) => {
        /// Where the camera stands and what it frames, in degrees, fractions and degrees.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct Shot { $(pub $field: f64,)* }
        impl Default for Shot {
            fn default() -> Self { Self { $($field: $default,)* } }
        }
    };
}
fields!(define);

impl Shot {
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

    /// The shot this JSON states, read against the default, with any name
    /// the row lacks refused rather than ignored.
    pub fn parse(text: &str) -> Result<Self> {
        let refused = |detail: String| RenderError::Scene(detail);
        let value: serde_json::Value = serde_json::from_str(text)
            .map_err(|error| refused(format!("this is not JSON: {error}")))?;
        let object = value
            .as_object()
            .ok_or_else(|| refused("this is not an object of numbers".into()))?;
        let mut shot = Self::default();
        let mut known = Vec::new();
        macro_rules! read {
            ($(($wire:literal, $field:ident, $low:literal, $high:literal, $default:literal, $name:literal)),* $(,)?) => { $(
                known.push($wire);
                if let Some(stated) = object.get($wire) {
                    shot.$field = stated.as_f64().ok_or_else(|| {
                        refused(format!("{} wants a number, not {stated}", $name))
                    })?;
                }
            )* };
        }
        fields!(read);
        for name in object.keys() {
            if !known.contains(&name.as_str()) {
                return Err(refused(format!(
                    "there is no camera field \"{name}\"; one of {}",
                    known.join(", ")
                )));
            }
        }
        shot.validate()?;
        Ok(shot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_shot_is_the_hero_pose() {
        let shot = Shot::default();
        assert!(shot.validate().is_ok());
        assert!((shot.fov - crate::FIELD_OF_VIEW).abs() < 1e-9);
        assert!((shot.fill - 1.0 / crate::FRAME_MARGIN).abs() < 1e-3);
    }

    #[test]
    fn the_shot_survives_its_own_json_and_refuses_what_it_cannot_frame() {
        let shot = Shot {
            azimuth: 210.0,
            elevation: -8.0,
            fill: 0.92,
            ..Default::default()
        };
        assert_eq!(Shot::parse(&shot.to_json()).unwrap(), shot);
        let partial = Shot::parse(r#"{"fill":1.0}"#).unwrap();
        assert_eq!(partial.fill, 1.0);
        assert_eq!(
            partial.distance, 0.0,
            "the distance is solved unless stated"
        );
        assert_eq!(partial.azimuth, Shot::default().azimuth);
        for (text, expected) in [
            (r#"{"elevation":95.0}"#, "elevation"),
            (r#"{"fill":0.0}"#, "fill"),
            (r#"{"fov":5.0}"#, "fov"),
            (r#"{"targetHeight":1.5}"#, "targetHeight"),
            (r#"{"focal":50.0}"#, "focal"),
            (r#"{"azimuth":"north"}"#, "azimuth"),
            ("[]", "not an object"),
        ] {
            let said = Shot::parse(text)
                .expect_err("the shot was taken anyway")
                .to_string();
            assert!(said.contains(expected), "{said} does not name {expected}");
        }
    }
}
