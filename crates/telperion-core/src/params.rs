//! JSON wire mirror of the parameter family, shared by the C-ABI binding and
//! the renderer, generated from the parameter catalogue: every row sits at
//! its catalogue path. Gated behind the `json` feature so the default core
//! stays serde-free; the same object is the browser's preset metadata.
use crate::{catalogue, presets::Family, Error, Result};
use serde_json::{json, Value};

pub use crate::presets::{by_identity, CATALOGUE, IN_WORK};
pub fn preset(id: u32) -> Result<Family> {
    let identity = CATALOGUE
        .iter()
        .find(|entry| entry.0 == id)
        .ok_or(Error::InvalidInput("preset id"))?
        .1;
    by_identity(identity)
}
/// The family as its wire object: every catalogue row at its path.
pub fn metadata(f: &Family) -> Value {
    let mut v = json!({});
    for entry in catalogue::entries() {
        let mut slot = &mut v;
        for key in entry.path()[1..].split('/') {
            slot = &mut slot[key];
        }
        *slot = entry.get(f).encode();
    }
    v
}
pub fn parse(v: &Value) -> Result<Family> {
    if let Some(id) = v.as_str() {
        return by_identity(id);
    }
    let f = decode(v)?;
    // The material row has no builder of its own to judge it: no mesh depends
    // on a colour, so nothing downstream would ever look. The wire is its
    // consumer, and the wire is where a value off its range or a range that
    // runs backwards is refused, by the name of the field that was wrong.
    f.material.validate()?;
    crate::growth::Age::from_years(f.age)?;
    f.growth.validate()?;
    Ok(f)
}
/// The wire read into a family, every key known and every value its type,
/// with no row judged.
pub(crate) fn decode(v: &Value) -> Result<Family> {
    let mut f = preset(0)?;
    let schema = metadata(&f);
    fn known(v: &Value, schema: &Value, unknown: &'static str) -> Result<()> {
        let map = v.as_object().ok_or(Error::InvalidInput("family object"))?;
        for (k, value) in map {
            let s = schema.get(k).ok_or(Error::InvalidInput(unknown))?;
            if s.is_object() {
                known(
                    value,
                    s,
                    match k.as_str() {
                        "habit" => "unknown habit trait",
                        "element" => "unknown element trait",
                        "canopy" => "unknown canopy trait",
                        "material" => "unknown material trait",
                        _ => unknown,
                    },
                )?;
            }
        }
        Ok(())
    }
    known(v, &schema, "unknown family parameter")?;
    for entry in catalogue::entries() {
        if let Some(value) = v.pointer(entry.path()) {
            entry
                .set(&mut f)
                .decode(value.clone())
                .map_err(|_| Error::InvalidInput(entry.path()))?;
        }
    }
    Ok(f)
}

/// A family with some of its rows restated: `overrides` is a partial wire
/// object laid over the family's own wire, then read back through `parse`, so
/// an unknown key or a value off its range is refused by name. A value trial
/// states only the rows it moves.
pub fn overlay(f: &Family, overrides: &Value) -> Result<Family> {
    let mut wire = metadata(f);
    lay(&mut wire, overrides)?;
    parse(&wire)
}
fn lay(wire: &mut Value, over: &Value) -> Result<()> {
    let map = over
        .as_object()
        .ok_or(Error::InvalidInput("family object"))?;
    for (key, value) in map {
        match (wire.get_mut(key), value.is_object()) {
            (Some(slot), true) if slot.is_object() => lay(slot, value)?,
            (Some(slot), _) => *slot = value.clone(),
            (None, _) => return Err(Error::InvalidInput("unknown family parameter")),
        }
    }
    Ok(())
}

// The wire's own tests - every control of every family through the schema in
// both directions, and every refusal by the name of what was refused - live
// beside this file rather than in it, so neither outgrows the line rule.
#[cfg(test)]
mod tests;
