//! The shipped presets as the build compiled them from their value files in
//! `crates/telperion-core/presets/`: each file a table of rows, every row
//! checked against the catalogue as the crate compiles, so a key the
//! catalogue does not hold or a value off its row's bounds fails the build
//! with the file and line named.
use super::{grammar::Form, Preset};
use crate::catalogue::{self, Entry, Kind};
use crate::Family;

/// One row of a preset: where it sits in the family and what it holds.
#[derive(Clone, Copy)]
pub struct Value {
    entry: Entry,
    value: f64,
}

/// Why a row is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// The catalogue holds no row at the path.
    Unknown,
    /// The value is written in a form the row's type does not hold.
    Kind,
    /// The value is off the row's bounds.
    Bounds,
}

impl Value {
    /// The row at `path` holding `value`, written in `form`, if the
    /// catalogue admits it; callable in a const context.
    pub const fn row(path: &str, form: Form, value: f64) -> Result<Self, Refused> {
        let Some((entry, bounds, kind)) = catalogue::locate(path) else {
            return Err(Refused::Unknown);
        };
        let fits = match kind {
            Kind::Real | Kind::OptionalReal => matches!(form, Form::Whole | Form::Real),
            Kind::Count | Kind::Size | Kind::OptionalSize => matches!(form, Form::Whole),
            Kind::Switch => matches!(form, Form::Switch),
        };
        if !fits {
            return Err(Refused::Kind);
        }
        if !bounds.admits(value) {
            return Err(Refused::Bounds);
        }
        Ok(Self { entry, value })
    }

    pub fn apply(self, f: &mut Family) {
        self.entry.put(f, self.value);
    }
}

/// One compiled value file: `name` holds its rows, each checked where the
/// constant is evaluated, and a refusal names `file` and the row's line.
#[doc(hidden)]
#[macro_export]
macro_rules! preset_values {
    ($name:ident, $file:literal, [$(($line:literal, $path:literal, $form:ident, $value:expr)),* $(,)?]) => {
        pub const $name: &[$crate::presets::table::Value] = &[$(
            match $crate::presets::table::Value::row(
                $path,
                $crate::presets::grammar::Form::$form,
                $value,
            ) {
                Ok(value) => value,
                Err($crate::presets::table::Refused::Unknown) => panic!(concat!(
                    "presets/", $file, ":", $line, ": ", $path, " is not a catalogue row"
                )),
                Err($crate::presets::table::Refused::Kind) => panic!(concat!(
                    "presets/", $file, ":", $line, ": ", $path, " holds another kind of value"
                )),
                Err($crate::presets::table::Refused::Bounds) => panic!(concat!(
                    "presets/", $file, ":", $line, ": ", $path, " is off its bounds"
                )),
            },
        )*];
    };
}

mod compiled {
    use super::Preset;
    include!(concat!(env!("OUT_DIR"), "/presets.rs"));
}

/// The rows `preset`'s value file sets over the default family.
pub fn rows(preset: Preset) -> &'static [Value] {
    compiled::rows(preset)
}

/// The default family with `preset`'s rows set.
pub fn family(preset: Preset) -> Family {
    let mut f = Family::default();
    for value in rows(preset) {
        value.apply(&mut f);
    }
    f
}
