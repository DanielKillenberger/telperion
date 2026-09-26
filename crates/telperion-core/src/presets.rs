//! Named families. Change the seed separately to draw another specimen.
//! Each preset is a value file in `crates/telperion-core/presets/`, compiled
//! into a table of rows over the default family (`table.rs`).
#[doc(hidden)]
pub mod grammar;
#[doc(hidden)]
pub mod table;
pub mod values;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Ordinary,
    OregonWhiteOak,
    NorwaySpruce,
    EuropeanBeech,
    SilverBirch,
    DatePalm,
    Telperion,
    Laurelin,
}
pub use crate::Family;
use crate::{Error, Result};

// Numeric ABI IDs remain stable for existing callers; catalogue order is irrelevant.
pub const CATALOGUE: &[(u32, &str, &str, &str)] = &[
    (0, "ordinary", "Ordinary", "Natural baseline"),
    (
        3,
        "oregon-white-oak",
        "Oregon white oak",
        "Quercus garryana",
    ),
    (4, "norway-spruce", "Norway spruce", "Picea abies"),
    (6, "silver-birch", "Silver birch", "Betula pendula"),
    (7, "date-palm", "Date palm", "Phoenix dactylifera"),
    (1, "telperion", "Telperion", "The silver tree"),
    (2, "laurelin", "Laurelin", "The golden tree"),
];
/// Tables still being judged. Their ABI ids are reserved, and they are not
/// listed, served by id or built by name: the core's tests and the species
/// runner reach them through `Preset`. The European beech ships when fn-62
/// accepts it. The date palm shipped when the owner accepted it (fn-82).
pub const IN_WORK: &[(u32, &str, &str, &str)] =
    &[(5, "european-beech", "European beech", "Fagus sylvatica")];
/// The family a shipped identity serves: the catalogue's tables, with the
/// growth default an unset turn limit falls back to. Every binding resolves
/// a species id through here.
pub fn by_identity(id: &str) -> Result<Family> {
    if !CATALOGUE.iter().any(|entry| entry.1 == id) {
        return Err(Error::InvalidInput("preset identity"));
    }
    let mut f = Preset::from_id(id)
        .ok_or(Error::InvalidInput("preset identity"))?
        .parameters();
    f.skeleton
        .growth
        .max_turn_per_step
        .get_or_insert(crate::colonization::GrowthConfig::default().max_turn_per_step);
    Ok(f)
}

impl Preset {
    /// Stable research identity; synthetic families have no botanical profile.
    pub fn profile_id(self) -> Option<&'static str> {
        match self {
            Self::OregonWhiteOak => Some("oregon-white-oak"),
            Self::NorwaySpruce => Some("norway-spruce"),
            Self::EuropeanBeech => Some("european-beech"),
            Self::SilverBirch => Some("silver-birch"),
            Self::DatePalm => Some("date-palm"),
            _ => None,
        }
    }

    /// Native selection is explicit and independent of catalogue order or seed.
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "ordinary" => Some(Self::Ordinary),
            "oregon-white-oak" => Some(Self::OregonWhiteOak),
            "norway-spruce" => Some(Self::NorwaySpruce),
            "european-beech" => Some(Self::EuropeanBeech),
            "silver-birch" => Some(Self::SilverBirch),
            "date-palm" => Some(Self::DatePalm),
            "telperion" => Some(Self::Telperion),
            "laurelin" => Some(Self::Laurelin),
            _ => None,
        }
    }

    /// The default family with the preset's value file set over it.
    pub fn parameters(self) -> Family {
        table::family(self)
    }
}
