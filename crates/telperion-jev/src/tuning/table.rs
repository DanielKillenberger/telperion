//! The dial table, generated from the parameter catalogue: every row that
//! states a dial is one, stepping inside its own bounds unless the row names
//! a narrower window, and every other numeric row is excluded for the reason
//! its entry gives. A tuning config names dials by id against a revision of
//! this table; a config written before the catalogue carries its own copy.
use super::actions::Dial;
use crate::sha256_hex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use telperion_core::{
    catalogue::{self, Dial as Offer, Growth, Kind},
    Family,
};

/// Every dial the catalogue offers, in catalogue order.
pub fn authored() -> Vec<Dial> {
    catalogue::entries()
        .filter_map(|entry| {
            let Offer::Tuned(t) = entry.info().dial else {
                return None;
            };
            let bounds = entry.info().bounds;
            let [min, max] = t.window.unwrap_or([bounds.low, bounds.high]);
            let group = entry.path().split('/').nth(1).unwrap_or_default();
            Some(Dial {
                id: t.id.into(),
                path: entry.path().into(),
                meaning: t.ask.into(),
                min,
                max,
                integer: entry.get(&Family::default()).kind().whole(),
                small: t.small,
                substantial: t.substantial,
                group: Some(group.into()),
                score_visible: Some(group != "material"),
                meaning_basis: None,
                range_basis: Some(t.basis.into()),
                source: None,
                preset_span: t.span,
                cap: (!t.cap.is_empty()).then(|| t.cap.into()),
            })
        })
        .collect()
}

/// Every numeric row that is not a dial, by path, with the reason.
pub fn excluded() -> BTreeMap<String, String> {
    let defaults = Family::default();
    catalogue::entries()
        .filter_map(|entry| {
            let info = entry.info();
            let kind = entry.get(&defaults).kind();
            let why = match info.dial {
                Offer::Tuned(_) => return None,
                Offer::Excluded(why) => why,
                Offer::Derived if info.deprecated => "Deprecated: no production stage reads it.",
                Offer::Derived if info.growth == Growth::Only => "Growth-path only.",
                Offer::Derived if kind == Kind::Switch => "Boolean, not a numeric scalar.",
                Offer::Derived => "An unset option has no current value to step.",
            };
            Some((entry.path().into(), why.into()))
        })
        .collect()
}

/// The table's revision: the SHA-256 of its dials as a config would embed them.
pub fn revision() -> String {
    sha256_hex(&serde_json::to_vec(&authored()).unwrap())
}

/// A tuning config's dials: its own copy, as every config before the
/// catalogue carried, or ids into the generated table at a stated revision.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dials {
    Embedded(Vec<Dial>),
    Named(Named),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Named {
    /// The table revision the ids were chosen against.
    pub catalogue: String,
    pub ids: Vec<String>,
    /// A narrower window or other steps for a named dial, by id.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub overrides: BTreeMap<String, Override>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Override {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub small: Option<f64>,
    pub substantial: Option<f64>,
}

impl Dials {
    /// The dials a run steps. Named dials are refused when the table has
    /// moved since they were chosen, when an id names no dial, and when an
    /// override names a dial the config did not.
    pub fn resolve(&self) -> Result<Vec<Dial>, String> {
        let named = match self {
            Self::Embedded(dials) => return Ok(dials.clone()),
            Self::Named(named) => named,
        };
        let now = revision();
        if named.catalogue != now {
            return Err(format!(
                "tuning config names dials at catalogue revision {}; the table is at {now}",
                named.catalogue
            ));
        }
        if let Some(id) = named.overrides.keys().find(|id| !named.ids.contains(id)) {
            return Err(format!(
                "override for dial {id}, which the config does not name"
            ));
        }
        let table = authored();
        named
            .ids
            .iter()
            .map(|id| {
                let mut dial = table
                    .iter()
                    .find(|d| &d.id == id)
                    .cloned()
                    .ok_or_else(|| format!("unknown dial id {id}"))?;
                if let Some(o) = named.overrides.get(id) {
                    dial.min = o.min.unwrap_or(dial.min);
                    dial.max = o.max.unwrap_or(dial.max);
                    dial.small = o.small.unwrap_or(dial.small);
                    dial.substantial = o.substantial.unwrap_or(dial.substantial);
                }
                dial.validate()?;
                Ok(dial)
            })
            .collect()
    }
}
