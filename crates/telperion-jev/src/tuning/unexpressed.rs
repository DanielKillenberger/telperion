//! Traits the generator cannot draw yet, each named with the open spec whose
//! landing will let it. The reviewer still judges them, but no dial can move
//! a trait that has no organ to draw it: the closing review's rollback
//! ignores them (fn-116), and the core-coverage gate that readiness rests on
//! leaves them out as known gaps (fn-136).
use super::reference_first::{Inventory, Priority, RuntimeConfig};
use super::state::{CellStatus, TraitStatus};
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

/// The core traits that keep reference-first coverage from passing, in
/// inventory order, and the status they give it: fail when any fails,
/// unknown when any other is not an unqualified pass. A trait listed
/// unexpressed is left out: the generator cannot draw it until its spec
/// lands, so it is a known gap, never a failure of the tree (fn-136).
pub fn core_coverage(
    inventory: &Inventory,
    statuses: &[TraitStatus],
    unexpressed: &[Unexpressed],
) -> (CellStatus, Vec<String>) {
    let mut status = CellStatus::Pass;
    let mut blocking = Vec::new();
    let core = inventory.traits.iter().filter(|t| {
        t.priority == Priority::Core && !unexpressed.iter().any(|u| u.trait_id == t.id)
    });
    for t in core {
        match statuses
            .iter()
            .find(|s| s.trait_id == t.id)
            .map(|s| s.status)
        {
            Some(CellStatus::Pass) if !t.uncertain => continue,
            Some(CellStatus::Fail) => status = CellStatus::Fail,
            _ if status == CellStatus::Pass => status = CellStatus::Unknown,
            _ => {}
        }
        blocking.push(t.id.clone());
    }
    (status, blocking)
}
