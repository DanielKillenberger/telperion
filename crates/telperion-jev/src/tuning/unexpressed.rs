//! Traits the generator cannot draw yet, each named with the open spec whose
//! landing will let it. The reviewer still judges them and readiness still
//! wants them passing; only the closing review's rollback ignores them, since
//! no dial can move a trait that has no organ to draw it.
use super::reference_first::{Inventory, RuntimeConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Unexpressed {
    /// A trait id from the reference-first inventory.
    #[serde(rename = "trait")]
    pub trait_id: String,
    /// The open spec whose landing lets the generator draw it.
    pub spec: String,
}

/// Refuses an entry with no spec, or one whose trait the inventory lacks.
pub fn verify(entries: &[Unexpressed], prepared: Option<&RuntimeConfig>) -> Result<(), String> {
    if entries.is_empty() {
        return Ok(());
    }
    if let Some(entry) = entries.iter().find(|e| e.spec.trim().is_empty()) {
        return Err(format!(
            "unexpressed trait {} names no spec",
            entry.trait_id
        ));
    }
    let prepared = prepared.ok_or("unexpressed traits need a reference-first inventory")?;
    let inventory: Inventory =
        serde_json::from_slice(&prepared.inventory.bytes()?).map_err(|e| e.to_string())?;
    match entries
        .iter()
        .find(|e| !inventory.traits.iter().any(|t| t.id == e.trait_id))
    {
        Some(entry) => Err(format!(
            "unexpressed trait {} is not in the inventory",
            entry.trait_id
        )),
        None => Ok(()),
    }
}
