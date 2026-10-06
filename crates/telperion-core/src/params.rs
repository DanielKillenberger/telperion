//! JSON wire mirror of the parameter family, shared by the C-ABI binding and
//! the renderer, generated from the parameter catalogue: every row sits at
//! its catalogue path. Gated behind the `json` feature so the default core
//! stays serde-free; the same object is the browser's preset metadata.
use crate::{
    catalogue,
    presets::{space, Family},
    Error, Result,
};
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
    let rows = space::rows(&f.space).expect("a family's space is written as rows");
    for (path, cell) in rows {
        let mut slot = &mut v;
        for key in path[1..].split('/') {
            slot = &mut slot[key];
        }
        *slot = match cell {
            space::Cell::Number(n) => json!(n),
            space::Cell::Switch(b) => json!(b),
            space::Cell::Never => json!("never"),
        };
    }
    v
}

/// The engine species' rows of a wire object's `space`, flattened to their
/// paths; a value that is not a number, a switch or `never` is refused.
fn engine(v: &Value, at: &str, out: &mut Vec<(String, space::Cell)>) -> Result<()> {
    let refused = Error::InvalidInput("space row");
    match v {
        Value::Object(map) => {
            for (k, value) in map {
                engine(value, &format!("{at}/{k}"), out)?;
            }
        }
        Value::Number(n) => out.push((at.into(), space::Cell::Number(n.as_f64().ok_or(refused)?))),
        Value::Bool(b) => out.push((at.into(), space::Cell::Switch(*b))),
        Value::String(t) if t == "never" => out.push((at.into(), space::Cell::Never)),
        _ => return Err(refused),
    }
    Ok(())
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
    catalogue::check(Family::CHECKS, &f, catalogue::Site::Age)?;
    Ok(f)
}
/// The wire read into a family, every key known and every value its type,
/// with no row judged.
pub(crate) fn decode(v: &Value) -> Result<Family> {
    let mut f = preset(0)?;
    let schema = metadata(&f);
    fn known(v: &Value, schema: &Value, at: &str, unknown: &'static str) -> Result<()> {
        let map = v.as_object().ok_or(Error::InvalidInput("family object"))?;
        for (k, value) in map {
            let path = format!("{at}/{k}");
            // The engine species' rows are judged by their own reader.
            if path == "/space" {
                continue;
            }
            let s = schema
                .get(k)
                .ok_or_else(|| Error::InvalidInput(catalogue::retired(&path).unwrap_or(unknown)))?;
            if s.is_object() {
                known(
                    value,
                    s,
                    &path,
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
    known(v, &schema, "", "unknown family parameter")?;
    // Read in the wire's own order, so the first malformed value refused is
    // the one the wire always refused first.
    let mut rows: Vec<_> = catalogue::entries().collect();
    rows.sort_unstable_by_key(|entry| entry.wire());
    for entry in rows {
        if let Some(value) = v.pointer(entry.path()) {
            entry
                .set(&mut f)
                .decode(value.clone())
                .map_err(|_| Error::InvalidInput(entry.path()))?;
        }
    }
    let mut rows = Vec::new();
    if let Some(engine_rows) = v.get("space") {
        engine(engine_rows, "/space", &mut rows)?;
    }
    f.space = space::read(rows.iter().map(|(p, c)| (p.as_str(), *c)))
        .map_err(|_| Error::InvalidInput("space row"))?;
    Ok(f)
}

/// A family with some of its rows restated: `overrides` is a partial wire
/// object laid over the family's own wire, then read back through `parse`, so
/// an unknown key or a value off its range is refused by name. A value trial
/// states only the rows it moves.
pub fn overlay(f: &Family, overrides: &Value) -> Result<Family> {
    let mut wire = metadata(f);
    lay(&mut wire, overrides, "")?;
    parse(&wire)
}
/// Lays `over` onto `wire` at the path `at`; a key the wire lacks is refused,
/// by its replacement where the catalogue retired it.
fn lay(wire: &mut Value, over: &Value, at: &str) -> Result<()> {
    let map = over
        .as_object()
        .ok_or(Error::InvalidInput("family object"))?;
    for (key, value) in map {
        let path = format!("{at}/{key}");
        match (wire.get_mut(key), value.is_object()) {
            (Some(slot), true) if slot.is_object() => lay(slot, value, &path)?,
            (Some(slot), _) => *slot = value.clone(),
            // A space row a family leaves at its default is not on its wire:
            // laid as given, its reader judges it.
            (None, _) if path.starts_with("/space") => {
                wire[key.as_str()] = value.clone();
            }
            (None, _) => {
                let why = catalogue::retired(&path).unwrap_or("unknown family parameter");
                return Err(Error::InvalidInput(why));
            }
        }
    }
    Ok(())
}

// The wire's own tests - every control of every family through the schema in
// both directions, and every refusal by the name of what was refused - live
// beside this file rather than in it, so neither outgrows the line rule.
#[cfg(test)]
mod tests;
