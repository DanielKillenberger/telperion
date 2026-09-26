//! The parameter catalogue: every wire row declared once, beside the domain
//! that reads it. One `rows!` declaration emits the family field and the row's
//! entry: its path, type, unit, bounds, meaning (the field's doc comment), the
//! stages that read it, where the growth path and validation reach it, where
//! it lies dormant, how a walk between two families moves it and how the
//! tuning loop may step it. The wire, scalar validation, ordinary blending,
//! the dial table, `docs/parameters.md` and the browser's parameter metadata
//! are generated from these entries.
//!
//! Dependencies between rows are written as prose on the entry (`applies`,
//! `note`), never executed: derived values stay in the functions that compute
//! them at the lifetime their inputs exist.
mod browser;
mod check;
mod mount;
mod reference;
mod scalar;
mod walk;

pub use browser::browser;
pub use check::check;
pub use mount::{entries, entry, Entry};
pub use reference::reference;
pub use scalar::{Kind, Scalar};
pub use walk::{degrees, density, linear, walk, weighted};

/// A stage of the one pipeline that reads a row on the direct build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Grow,
    Plan,
    Expand,
    Cull,
    Draw,
}

/// How the hidden growth path reads a row, beside the direct build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Growth {
    /// Read as the direct build reads it.
    Same,
    /// Only the growth path reads it; the direct build validates it at most.
    Only,
    /// The growth path never reads it.
    Ignored,
    /// Read differently on the growth path, as stated.
    Differs(&'static str),
}

/// The values a row admits. A count's bounds are whole numbers held as reals.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub low: f64,
    pub high: f64,
    /// The low end itself is refused.
    pub low_open: bool,
    /// Zero is admitted besides the range, as "none of it".
    pub zero: bool,
}
impl Bounds {
    pub const fn closed(low: f64, high: f64) -> Self {
        Self {
            low,
            high,
            low_open: false,
            zero: false,
        }
    }
    /// Every finite value.
    pub const FINITE: Self = Self::closed(f64::NEG_INFINITY, f64::INFINITY);
    pub const fn at_least(low: f64) -> Self {
        Self::closed(low, f64::INFINITY)
    }
    /// Above `low`, which is itself refused.
    pub const fn above(low: f64) -> Self {
        Self::at_least(low).open()
    }
    pub const fn open(self) -> Self {
        Self {
            low_open: true,
            ..self
        }
    }
    pub const fn or_zero(self) -> Self {
        Self { zero: true, ..self }
    }
    pub fn admits(self, v: f64) -> bool {
        let low = if self.low_open {
            v > self.low
        } else {
            v >= self.low
        };
        v.is_finite() && ((self.zero && v == 0.0) || (low && v <= self.high))
    }
}

/// The error a row off its bounds is refused with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// `Error::InvalidValue`, naming the row and the value.
    Value(&'static str),
    /// `Error::InvalidInput`, naming the row or the group it shares.
    Input(&'static str),
}

/// The validation sites the generated scalar check serves, one per place a
/// family's rows are judged. `rank` orders a site's rows: the first refusal a
/// site answers with is the one it always answered with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Site {
    Growth,
    Habit,
    Sampling,
    Step,
    Envelope,
    Outline,
    Writhe,
    Bias,
    Radius,
    Twig,
    Surface,
    Material,
    Canopy,
    ShortShoots,
    Rosette,
    Leaf,
    LeafCounts,
    Shell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Check {
    pub site: Site,
    pub rank: u8,
    pub refusal: Refusal,
}
pub const fn value(site: Site, rank: u8, name: &'static str) -> Option<Check> {
    Some(Check {
        site,
        rank,
        refusal: Refusal::Value(name),
    })
}
pub const fn input(site: Site, rank: u8, name: &'static str) -> Option<Check> {
    Some(Check {
        site,
        rank,
        refusal: Refusal::Input(name),
    })
}

/// How a walk between two families moves a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blend {
    Linear,
    /// Linear, clamped to the two ends.
    Weighted,
    /// Degrees along the shorter arc.
    Degrees,
    /// Rounded to the nearest whole number.
    Count,
    /// Rounded down.
    Down,
    /// Rounded up.
    Up,
    /// A large count, rounded to the nearest.
    Many,
    /// A spacing, walked as the density it stands for.
    Density,
    /// The first family's value, unwalked.
    Kept,
    /// Walked with the rows it is coupled to, by `blend::families`.
    Coupled,
}

/// How the tuning loop may step a row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dial {
    /// Excluded for what the entry already says: a switch, an unset option,
    /// a growth-path row or a deprecated one.
    Derived,
    /// Excluded, for the reason given.
    Excluded(&'static str),
    Tuned(Tuning),
}

/// A dial's own profile: the question's id and wording, the window it steps
/// within and its two step sizes. The window is the row's bounds unless the
/// tuning loop was given a narrower one, and `basis` says where it came from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tuning {
    pub id: &'static str,
    pub ask: &'static str,
    pub window: Option<[f64; 2]>,
    pub small: f64,
    pub substantial: f64,
    pub basis: &'static str,
    /// Where the shipped presets sit: a stride hint and a prior, never a wall.
    pub span: Option<[f64; 2]>,
    /// Why a "capped" window keeps a side at the preset span.
    pub cap: &'static str,
}
/// A dial that steps across the row's own bounds.
pub const fn bounded(id: &'static str, ask: &'static str, steps: [f64; 2]) -> Dial {
    Dial::Tuned(Tuning {
        id,
        ask,
        window: None,
        small: steps[0],
        substantial: steps[1],
        basis: "validated bound",
        span: None,
        cap: "",
    })
}
/// A dial that steps within a window of its own.
pub const fn tuned(
    id: &'static str,
    ask: &'static str,
    window: [f64; 2],
    steps: [f64; 2],
    basis: &'static str,
) -> Dial {
    Dial::Tuned(Tuning {
        id,
        ask,
        window: Some(window),
        small: steps[0],
        substantial: steps[1],
        basis,
        span: None,
        cap: "",
    })
}
impl Dial {
    /// The span the shipped presets occupy on this dial.
    pub const fn span(self, span: [f64; 2]) -> Self {
        match self {
            Self::Tuned(t) => Self::Tuned(Tuning {
                span: Some(span),
                ..t
            }),
            other => other,
        }
    }
    /// Why the window keeps a side at the preset span.
    pub const fn cap(self, cap: &'static str) -> Self {
        match self {
            Self::Tuned(t) => Self::Tuned(Tuning { cap, ..t }),
            other => other,
        }
    }
}

/// Everything a row's declaration states.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Info {
    /// The field's doc comment.
    pub meaning: &'static str,
    pub unit: &'static str,
    pub bounds: Bounds,
    /// Stages that read the row on the direct build; none only for a row the
    /// growth path alone reads or a deprecated one.
    pub reads: &'static [Stage],
    /// The row's rank on the wire before the catalogue (`fields!` order).
    pub wire: u16,
    pub growth: Growth,
    /// Where the generated scalar check refuses it; `None` where the row is
    /// judged by a named relational check or not at all (see `note`).
    pub check: Option<Check>,
    /// Where the row lies dormant, empty where it always acts.
    pub applies: &'static str,
    /// Couplings, relational checks and conflicting bounds.
    pub note: &'static str,
    pub blend: Blend,
    pub dial: Dial,
    /// Read by no production stage; parsed, overlaid and written as before.
    pub deprecated: bool,
}
impl Info {
    /// The optional fields' defaults; `rows!` always states the rest.
    pub const OPTIONAL: Self = Self {
        meaning: "",
        unit: "",
        bounds: Bounds::FINITE,
        reads: &[],
        wire: u16::MAX,
        growth: Growth::Same,
        check: None,
        applies: "",
        note: "",
        blend: Blend::Linear,
        dial: Dial::Derived,
        deprecated: false,
    };
}

/// One row of a group `S` as the wire and a walk read it: its path and its
/// value inside `S`. What the scalar check reads is `S::CHECKS`, and what the
/// row documents `S::INFO`, both at the same index and kept apart, so a build
/// that only validates carries no path and one that never renders a row
/// carries none of its prose.
pub struct Row<S: 'static> {
    pub path: &'static str,
    /// The row's rank on the wire before the catalogue: decoding refuses the
    /// first malformed value in this order.
    pub wire: u16,
    pub get: fn(&S) -> &dyn Scalar,
    pub set: fn(&mut S) -> &mut dyn Scalar,
    pub blend: Blend,
}

/// One row of a group `S` as the scalar check reads it.
pub struct Checked<S: 'static> {
    pub get: fn(&S) -> &dyn Scalar,
    pub bounds: Bounds,
    pub check: Option<Check>,
}

/// Where `rows!` keeps each field's doc comment as a string, one inherent
/// const per field, apart from the group's own methods.
#[doc(hidden)]
pub struct Meanings<S>(std::marker::PhantomData<S>);

/// Where `rows!` keeps each row's getter, shared by `ROWS` and `CHECKS`.
#[doc(hidden)]
pub struct Getters<S>(std::marker::PhantomData<S>);

/// Declares a parameter group: the struct, with each field's doc comment and
/// serde attribute as written, and one catalogue row per field that names its
/// wire key. A row states its key, unit, bounds and the stages that read it;
/// the rest of `Info` follows in braces where it differs from the default. A
/// row with no doc comment, and a row that names no stage without being
/// growth-path-only or deprecated, does not compile.
/// `ROWS` carries what the wire reads, `CHECKS` what the scalar check reads
/// and `INFO` everything declared.
macro_rules! rows {
    (
        $(#[$meta:meta])*
        pub struct $name:ident in $prefix:literal {
            $(
                $(#[doc = $doc:literal])*
                $(#[cfg_attr(feature = "json", serde($($serde:tt)*))])?
                pub $field:ident: $ty:ty
                $(= $key:literal $unit:literal $bounds:expr => [$($stage:ident),*]
                    $({ $($k:ident: $v:expr),* $(,)? })?)?
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        pub struct $name {
            $(
                $(#[doc = $doc])*
                $(#[cfg_attr(feature = "json", serde($($serde)*))])?
                pub $field: $ty,
            )*
        }
        #[allow(non_upper_case_globals)]
        impl $crate::catalogue::Meanings<$name> {
            $(pub const $field: &'static str = concat!($($doc, "\n",)* "");)*
        }
        #[allow(non_upper_case_globals)]
        impl $crate::catalogue::Getters<$name> {
            $($(
                #[doc = $key]
                pub const $field: fn(&$name) -> &dyn $crate::catalogue::Scalar = {
                    fn get(s: &$name) -> &dyn $crate::catalogue::Scalar {
                        &s.$field
                    }
                    get
                };
            )?)*
        }
        impl $name {
            /// This group's catalogue rows, in declaration order.
            pub const ROWS: &'static [$crate::catalogue::Row<Self>] = &[$($(
                {
                    const DECLARED: $crate::catalogue::Info = $crate::catalogue::Info {
                        bounds: $bounds,
                        $($($k: $v,)*)?
                        ..$crate::catalogue::Info::OPTIONAL
                    };
                    $crate::catalogue::Row {
                        path: concat!($prefix, "/", $key),
                        wire: DECLARED.wire,
                        get: $crate::catalogue::Getters::<$name>::$field,
                        set: {
                            fn set(s: &mut $name) -> &mut dyn $crate::catalogue::Scalar {
                                &mut s.$field
                            }
                            set
                        },
                        blend: DECLARED.blend,
                    }
                },
            )?)*];
            /// What the scalar check reads of each row, at its index in `ROWS`.
            pub const CHECKS: &'static [$crate::catalogue::Checked<Self>] = &[$($(
                {
                    const DECLARED: $crate::catalogue::Info = $crate::catalogue::Info {
                        bounds: $bounds,
                        $($($k: $v,)*)?
                        ..$crate::catalogue::Info::OPTIONAL
                    };
                    $crate::catalogue::Checked {
                        get: $crate::catalogue::Getters::<$name>::$field,
                        bounds: DECLARED.bounds,
                        check: DECLARED.check,
                    }
                },
            )?)*];
            /// Everything each row declares, at its index in `ROWS`.
            pub const INFO: &'static [$crate::catalogue::Info] = &[$($(
                {
                    const DECLARED: $crate::catalogue::Info = $crate::catalogue::Info {
                        meaning: $crate::catalogue::Meanings::<$name>::$field,
                        unit: $unit,
                        bounds: $bounds,
                        reads: &[$($crate::catalogue::Stage::$stage),*],
                        $($($k: $v,)*)?
                        ..$crate::catalogue::Info::OPTIONAL
                    };
                    assert!(!DECLARED.meaning.is_empty(), concat!($key, " has no meaning"));
                    assert!(
                        !DECLARED.reads.is_empty()
                            || DECLARED.deprecated
                            || matches!(DECLARED.growth, $crate::catalogue::Growth::Only),
                        concat!($key, " names no stage that reads it")
                    );
                    DECLARED
                },
            )?)*];
        }
    };
}
pub(crate) use rows;

#[cfg(test)]
mod tests;
