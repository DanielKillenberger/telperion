//! Every dial Jev supported this round, each in its own direction, stepped
//! together. One round's move is the whole bundle at a shared strength.
//!
//! Single steps on single dials cannot reach a look that needs several rows
//! at once: on the beech the six pendulous-twig rows are each inert alone
//! (`hanging-inert-on-beech-2026-09-21.md`). Jev still names directions only,
//! and code computes every value here.
mod isolate;
mod round;
mod track;
mod words;
mod worse;
pub(in crate::tuning) use round::{note as note_for, round};
pub use track::{verify as verify_tracks, Track};
pub(in crate::tuning) use words::words;
pub use worse::{excluded_families, part_family, EXCLUDED};

use super::{
    actions::{Action, Dial},
    engine::Proposal,
};
use crate::sha256_hex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// What one dial does inside a bundle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Move {
    pub dial: String,
    /// `up` or `down`: the direction Jev supported, never a magnitude.
    pub direction: String,
    pub from: f64,
    pub to: f64,
}

/// Why a dial Jev supported is not in the bundle after all.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dropped {
    pub dial: String,
    pub reason: String,
}

/// One strength variant of the round's bundle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bundle {
    pub strength: f64,
    pub moves: Vec<Move>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dropped: Vec<Dropped>,
    /// The bundle's identity: its dials and directions, its strength and the
    /// candidate it steps from. Two rounds proposing the same thing share it.
    pub id: String,
}

/// The direction a supported action carries, or none for a hold.
pub fn direction(action: Action) -> Option<i8> {
    match action {
        Action::SmallIncrease | Action::SubstantialIncrease => Some(1),
        Action::SmallDecrease | Action::SubstantialDecrease => Some(-1),
        Action::Hold | Action::InsufficientEvidence => None,
    }
}

/// Every dial Jev supported, in the order it proposed them, each once. A dial
/// proposed twice keeps its first direction; the second is dropped.
pub fn directions(proposals: &[Proposal]) -> Vec<(String, i8)> {
    let mut out: Vec<(String, i8)> = vec![];
    for proposal in proposals {
        let Some(sign) = direction(proposal.action) else {
            continue;
        };
        if !out.iter().any(|(dial, _)| dial == &proposal.dial) {
            out.push((proposal.dial.clone(), sign));
        }
    }
    out
}

/// The value one dial takes at this strength, or why it takes none.
fn step(dial: &Dial, current: f64, sign: i8, strength: f64) -> Result<f64, String> {
    let raw = strength * dial.small * f64::from(sign);
    let delta = if dial.integer {
        // Round half away from zero, and never round a whole step to nothing.
        let rounded = raw.abs().round().copysign(raw);
        if rounded == 0.0 {
            if strength >= 1.0 {
                f64::from(sign)
            } else {
                return Err("step rounds to no move at this strength".into());
            }
        } else {
            rounded
        }
    } else {
        raw
    };
    let to = (current + delta).clamp(dial.min, dial.max);
    if to == current {
        return Err("already at its bound in this direction".into());
    }
    Ok(to)
}

fn patch(path: &str, value: Value) -> Value {
    let mut partial = value;
    for key in path
        .split('/')
        .skip(1)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        partial = json!({key.replace("~1", "/").replace("~0", "~"): partial});
    }
    partial
}

/// The bundle at one strength, and the wire patch that draws it. A dial the
/// generator refuses, or that cannot move, drops out and is recorded; the
/// bundle itself is never clamped into feasibility.
pub fn build(
    preset: &str,
    effective: &Value,
    dials: &[Dial],
    wanted: &[(String, i8)],
    strength: f64,
    base_key: &str,
    track: &str,
) -> Result<(Bundle, Value), String> {
    if !strength.is_finite() || strength <= 0.0 {
        return Err("invalid bundle strength".into());
    }
    let base = telperion_core::presets::Preset::from_id(preset).ok_or("unknown preset")?;
    let family = base.parameters();
    let (mut moves, mut dropped, mut overlay) = (vec![], vec![], json!({}));
    for (id, sign) in wanted {
        let Some(dial) = dials.iter().find(|d| &d.id == id) else {
            dropped.push(Dropped {
                dial: id.clone(),
                reason: "no such dial in the table".into(),
            });
            continue;
        };
        let Some(current) = effective.pointer(&dial.path).and_then(Value::as_f64) else {
            dropped.push(Dropped {
                dial: id.clone(),
                reason: "no current value on the wire".into(),
            });
            continue;
        };
        let to = match step(dial, current, *sign, strength) {
            Ok(to) => to,
            Err(reason) => {
                dropped.push(Dropped {
                    dial: id.clone(),
                    reason,
                });
                continue;
            }
        };
        let value = if dial.integer {
            json!(to as i64)
        } else {
            json!(to)
        };
        let mut wire = effective.clone();
        *wire
            .pointer_mut(&dial.path)
            .ok_or("row disappeared from the wire")? = value.clone();
        if let Err(e) = telperion_core::params::overlay(&family, &wire) {
            dropped.push(Dropped {
                dial: id.clone(),
                reason: format!("generator refused the value: {e:?}"),
            });
            continue;
        }
        merge(&mut overlay, &patch(&dial.path, value));
        moves.push(Move {
            dial: id.clone(),
            direction: if *sign > 0 { "up" } else { "down" }.into(),
            from: current,
            to,
        });
    }
    if moves.is_empty() {
        return Err("every dial in the bundle dropped out".into());
    }
    let id = id(&moves, strength, base_key, track);
    Ok((
        Bundle {
            strength,
            moves,
            dropped,
            id,
        },
        overlay,
    ))
}

/// What makes two bundles the same attempt: the dials and their directions,
/// the strength, the tree they step from and the track they belong to.
pub fn id(moves: &[Move], strength: f64, base_key: &str, track: &str) -> String {
    let mut rows = moves
        .iter()
        .map(|m| format!("{}:{}", m.dial, m.direction))
        .collect::<Vec<_>>();
    rows.sort();
    sha256_hex(
        &serde_json::to_vec(
            &json!({"moves":rows,"strength":strength,"base":base_key,"track":track}),
        )
        .unwrap(),
    )
}

/// Halve a bundle to find what breaks: by the dial table's groups when there
/// is more than one, largest group against the rest, and otherwise by the
/// order Jev proposed them, alternating.
pub fn split(moves: &[Move], dials: &[Dial]) -> (Vec<Move>, Vec<Move>) {
    let group = |m: &Move| {
        dials
            .iter()
            .find(|d| d.id == m.dial)
            .and_then(|d| d.group.clone())
            .unwrap_or_default()
    };
    let mut counts: Vec<(String, usize)> = vec![];
    for m in moves {
        let g = group(m);
        match counts.iter_mut().find(|(name, _)| name == &g) {
            Some((_, n)) => *n += 1,
            None => counts.push((g, 1)),
        }
    }
    if counts.len() > 1 {
        let largest = counts
            .iter()
            .max_by_key(|(_, n)| *n)
            .map(|(name, _)| name.clone())
            .unwrap_or_default();
        let (a, b): (Vec<Move>, Vec<Move>) =
            moves.iter().cloned().partition(|m| group(m) == largest);
        if !a.is_empty() && !b.is_empty() {
            return (a, b);
        }
    }
    let (a, b): (Vec<(usize, Move)>, Vec<(usize, Move)>) = moves
        .iter()
        .cloned()
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);
    (
        a.into_iter().map(|(_, m)| m).collect(),
        b.into_iter().map(|(_, m)| m).collect(),
    )
}

/// A group this size is one family no cut can divide, so its rows are told
/// apart by the leaf's own first word instead. The 91 material rows were one
/// family, and six live rounds of a material bundle could not be cut at all.
const LARGE: usize = 8;

/// The leaf's first camelCase word: `barkRed` is bark, `plateCellScale` is
/// plate, `size` is size.
fn head(leaf: &str) -> &str {
    leaf.char_indices()
        .find(|(i, c)| *i > 0 && c.is_uppercase())
        .map_or(leaf, |(i, _)| &leaf[..i])
}

/// Which sub-family a dial row belongs to: below `/skeleton` the second path
/// segment, so envelope, habit, twigs, bias and growth are told apart; a group
/// of more than eight rows the leaf's own first word, qualified by the group
/// so that `element` and `material` do not share a `lobe`; anything smaller is
/// its own first segment.
pub fn family(path: &str, dials: &[Dial]) -> String {
    let segments = path
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let Some(first) = segments.first().copied() else {
        return String::new();
    };
    if first == "skeleton" {
        return segments.get(1).copied().unwrap_or(first).to_string();
    }
    let rows = dials
        .iter()
        .filter(|d| d.path.split('/').nth(1) == Some(first))
        .count();
    match segments.last().copied().filter(|_| rows > LARGE) {
        Some(leaf) => format!("{first}.{}", head(leaf)),
        None => first.to_string(),
    }
}

/// Two merged families, named together and each name kept once.
fn merged(a: &str, b: &str) -> String {
    let mut names = a.split('+').chain(b.split('+')).collect::<Vec<_>>();
    names.sort_unstable();
    names.dedup();
    names.join("+")
}

/// The bundle cut into at most four parts by dial sub-family, the smallest
/// families merged into each other until four remain. Deterministic: the
/// smallest by move count goes first, ties by name.
pub fn families(moves: &[Move], dials: &[Dial]) -> Vec<(String, Vec<Move>)> {
    let mut parts: Vec<(String, Vec<Move>)> = vec![];
    for m in moves {
        let name = dials
            .iter()
            .find(|d| d.id == m.dial)
            .map_or_else(String::new, |d| family(&d.path, dials));
        match parts.iter_mut().find(|(n, _)| n == &name) {
            Some((_, rows)) => rows.push(m.clone()),
            None => parts.push((name, vec![m.clone()])),
        }
    }
    while parts.len() > 4 {
        parts.sort_by(|a, b| a.1.len().cmp(&b.1.len()).then(a.0.cmp(&b.0)));
        let (name, rows) = parts.remove(0);
        parts[0].0 = merged(&parts[0].0, &name);
        parts[0].1.extend(rows);
    }
    parts.sort_by(|a, b| a.0.cmp(&b.0));
    parts
}

/// A part of a parent bundle, at the parent's own strength.
pub(super) fn part(moves: &[Move], strength: f64, base: &str, track: &str) -> Bundle {
    Bundle {
        strength,
        id: id(moves, strength, base, track),
        moves: moves.to_vec(),
        dropped: vec![],
    }
}

/// The wire patch for a subset of a bundle's moves.
pub fn overlay_of(moves: &[Move], dials: &[Dial]) -> Value {
    let mut overlay = json!({});
    for m in moves {
        let Some(dial) = dials.iter().find(|d| d.id == m.dial) else {
            continue;
        };
        let value = if dial.integer {
            json!(m.to as i64)
        } else {
            json!(m.to)
        };
        merge(&mut overlay, &patch(&dial.path, value));
    }
    overlay
}

pub(in crate::tuning) fn merge(into: &mut Value, patch: &Value) {
    match (into, patch) {
        (Value::Object(a), Value::Object(b)) => {
            for (key, value) in b {
                merge(a.entry(key.clone()).or_insert(Value::Null), value);
            }
        }
        (slot, value) => *slot = value.clone(),
    }
}
