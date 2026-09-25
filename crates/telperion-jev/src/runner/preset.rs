//! The accepted tree written into core as a preset in the style the shipped
//! ones are: a function of `p.<row> = <value>;` lines in
//! `crates/telperion-core/src/presets/species.rs`, registered in
//! `presets.rs` beside the others. A species that already has a function
//! keeps its lines and comments; each row the accepted tree moved has its
//! line's value replaced, or gains a line under the acceptance's comment. A
//! new species gets a function of every row that differs from the default
//! family, and its registration. Every wire key is its Rust field in snake
//! case (the params table holds no exception), so a pointer names its line.
use std::collections::BTreeMap;
use std::path::Path;

use serde_json::Value;
use telperion_core::{params, presets::Preset, Family};

use super::start::flatten;

const SPECIES: &str = "crates/telperion-core/src/presets/species.rs";
const PRESETS: &str = "crates/telperion-core/src/presets.rs";

/// What a new preset registers under.
pub struct Names {
    pub id: String,
    pub common: String,
    pub scientific: String,
}

/// `/canopy/leafBases` as the field it names, `canopy.leaf_bases`.
pub fn field(pointer: &str) -> String {
    pointer
        .trim_start_matches('/')
        .split('/')
        .map(|key| {
            let mut out = String::new();
            for c in key.chars() {
                if c.is_ascii_uppercase() {
                    out.push('_');
                    out.push(c.to_ascii_lowercase());
                } else {
                    out.push(c);
                }
            }
            out
        })
        .collect::<Vec<_>>()
        .join(".")
}

/// A wire value as Rust source for its field: an f64 always carries a point,
/// and an optional row (null on the default family) is wrapped.
pub fn literal(value: &Value, optional: bool) -> Result<String, String> {
    let bare = match value {
        Value::Null => return Ok("None".into()),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) if n.is_f64() => format!("{:?}", n.as_f64().unwrap()),
        Value::Number(n) => n.to_string(),
        other => return Err(format!("no Rust literal for {other}")),
    };
    Ok(if optional {
        format!("Some({bare})")
    } else {
        bare
    })
}

/// Every row whose value in `to` differs from `from`, by pointer.
pub fn moved(from: &Family, to: &Family) -> BTreeMap<String, Value> {
    let before = flatten(&params::metadata(from));
    flatten(&params::metadata(to))
        .into_iter()
        .filter(|(pointer, value)| before.get(pointer) != Some(value))
        .collect()
}

fn lines(rows: &BTreeMap<String, Value>) -> Result<Vec<(String, String)>, String> {
    let defaults = flatten(&params::metadata(&Family::default()));
    rows.iter()
        .map(|(pointer, value)| {
            let optional = defaults.get(pointer).is_some_and(Value::is_null);
            Ok((field(pointer), literal(value, optional)?))
        })
        .collect()
}

fn snake(id: &str) -> String {
    id.replace('-', "_")
}

fn camel(id: &str) -> String {
    id.split('-')
        .map(|w| {
            let mut c = w.chars();
            c.next().map_or(String::new(), |f| {
                f.to_ascii_uppercase().to_string() + c.as_str()
            })
        })
        .collect()
}

/// The span of `fn <name>`'s body in `source`: after its `{`, up to its `}`.
fn body(source: &str, name: &str) -> Option<(usize, usize)> {
    let head = source.find(&format!("pub(super) fn {name}(p: &mut Family) {{"))?;
    let open = source[head..].find('{')? + head + 1;
    let mut depth = 1;
    for (i, c) in source[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some((open, open + i));
                }
            }
            _ => {}
        }
    }
    None
}

/// `source` with fn `name`'s rows set to `rows`: a line that sets the row
/// has its value replaced; any other row is added under `note`.
pub fn edit(
    source: &str,
    name: &str,
    rows: &[(String, String)],
    note: &str,
) -> Result<String, String> {
    let (start, end) = body(source, name).ok_or(format!("no fn {name} in {SPECIES}"))?;
    let mut text = source[start..end].to_string();
    let mut added = Vec::new();
    for (field, value) in rows {
        let target = format!("p.{field} = ");
        match text.lines().find(|l| l.trim_start().starts_with(&target)) {
            Some(line) => {
                let indent = &line[..line.len() - line.trim_start().len()];
                let new = format!("{indent}{target}{value};");
                text = text.replacen(line, &new, 1);
            }
            None => added.push(format!("    p.{field} = {value};")),
        }
    }
    if !added.is_empty() {
        let trimmed = text.trim_end().to_string();
        text = format!("{trimmed}\n    // {note}\n{}\n", added.join("\n"));
    }
    Ok(format!("{}{}{}", &source[..start], text, &source[end..]))
}

/// A new species' function: every row that differs from the default family.
pub fn function(name: &str, rows: &[(String, String)], note: &str) -> String {
    let body: Vec<String> = rows
        .iter()
        .map(|(f, v)| format!("    p.{f} = {v};"))
        .collect();
    format!(
        "\npub(super) fn {name}(p: &mut Family) {{\n    // {note}\n{}\n}}\n",
        body.join("\n")
    )
}

fn insert_before(source: &str, anchor: &str, text: &str) -> Result<String, String> {
    let at = source
        .find(anchor)
        .ok_or(format!("{PRESETS}: no `{}`", anchor.trim()))?;
    Ok(format!("{}{text}{}", &source[..at], &source[at..]))
}

/// `presets.rs` with `names` registered: the variant, the catalogue entry at
/// the next free ABI id, the profile, the id and the parameters branch.
pub fn register(source: &str, names: &Names) -> Result<String, String> {
    let variant = camel(&names.id);
    // Every entry, on one line or spread over several, opens `(<id>, "`.
    let entry = regex::Regex::new(r#"\(\s*(\d+),\s*""#).expect("a valid pattern");
    let taken = entry
        .captures_iter(source)
        .filter_map(|c| c[1].parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    let id = &names.id;
    let mut s = insert_before(source, "    Telperion,\n", &format!("    {variant},\n"))?;
    s = insert_before(
        &s,
        "    (1, \"telperion\"",
        &format!(
            "    ({}, \"{id}\", \"{}\", \"{}\"),\n",
            taken + 1,
            names.common,
            names.scientific
        ),
    )?;
    s = insert_before(
        &s,
        "            _ => None,\n        }\n    }\n\n    /// Native selection",
        &format!("            Self::{variant} => Some(\"{id}\"),\n"),
    )?;
    s = insert_before(
        &s,
        "            \"telperion\" => Some(Self::Telperion),",
        &format!("            \"{id}\" => Some(Self::{variant}),\n"),
    )?;
    insert_before(
        &s,
        "        if self == Self::Ordinary {",
        &format!(
            "        if self == Self::{variant} {{\n            species::{}(&mut p);\n            return p;\n        }}\n",
            snake(id)
        ),
    )
}

/// Writes the accepted family as `names.id`'s preset under `root` and says
/// what it did. `note` heads the lines the acceptance adds.
pub fn write(root: &Path, names: &Names, accepted: &Family, note: &str) -> Result<String, String> {
    let read = |p: &str| std::fs::read_to_string(root.join(p)).map_err(|e| format!("{p}: {e}"));
    let species = read(SPECIES)?;
    let name = snake(&names.id);
    let (text, word) = match Preset::from_id(&names.id) {
        Some(shipped) => {
            let rows = lines(&moved(&shipped.parameters(), accepted))?;
            let count = rows.len();
            (
                edit(&species, &name, &rows, note)?,
                format!("{count} rows of fn {name} set"),
            )
        }
        None => {
            let rows = lines(&moved(&Family::default(), accepted))?;
            let presets = register(&read(PRESETS)?, names)?;
            std::fs::write(root.join(PRESETS), presets).map_err(|e| e.to_string())?;
            let text = format!(
                "{}{}",
                species.trim_end_matches('\n'),
                function(&name, &rows, note)
            );
            (
                text,
                format!("fn {name} written with {} rows and registered", rows.len()),
            )
        }
    };
    std::fs::write(root.join(SPECIES), text).map_err(|e| e.to_string())?;
    Ok(word)
}
