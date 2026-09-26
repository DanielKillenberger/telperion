//! The accepted tree written into core as the species' preset value file,
//! `crates/telperion-core/presets/<id>.values`, by core's writer
//! (`presets::values::write`). A shipped species keeps its file's lines and
//! comments: each row the accepted tree moved takes its new value, or gains a
//! line under the acceptance's note. A new species gets a file of every row
//! that differs from the default family, and its registration in
//! `presets.rs`; the build compiles the file and checks it against the
//! catalogue.
use std::collections::BTreeMap;
use std::path::Path;

use serde_json::Value;
use telperion_core::{params, presets::values, presets::Preset, Family};

use super::start::flatten;

const PRESETS: &str = "crates/telperion-core/src/presets.rs";
/// Where the value files live, one `<id>.values` a preset.
const VALUES: &str = "crates/telperion-core/presets";

/// What a new preset registers under.
pub struct Names {
    pub id: String,
    pub common: String,
    pub scientific: String,
}

/// Every row whose value in `to` differs from `from`, by pointer.
pub fn moved(from: &Family, to: &Family) -> BTreeMap<String, Value> {
    let before = flatten(&params::metadata(from));
    flatten(&params::metadata(to))
        .into_iter()
        .filter(|(pointer, value)| before.get(pointer) != Some(value))
        .collect()
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

fn insert_before(source: &str, anchor: &str, text: &str) -> Result<String, String> {
    let at = source
        .find(anchor)
        .ok_or(format!("{PRESETS}: no `{}`", anchor.trim()))?;
    Ok(format!("{}{text}{}", &source[..at], &source[at..]))
}

/// `presets.rs` with `names` registered: the variant, the catalogue entry at
/// the next free ABI id, the profile and the id. The build pairs the variant
/// with its value file.
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
    insert_before(
        &s,
        "            \"telperion\" => Some(Self::Telperion),",
        &format!("            \"{id}\" => Some(Self::{variant}),\n"),
    )
}

/// Writes the accepted family as `names.id`'s value file under `root` and
/// says what it did. `note` heads the lines the acceptance adds.
pub fn write(root: &Path, names: &Names, accepted: &Family, note: &str) -> Result<String, String> {
    let file = format!("{VALUES}/{}.values", names.id);
    let path = root.join(&file);
    let read = |p: &Path| std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()));
    let (source, rows, registered) = match Preset::from_id(&names.id) {
        Some(shipped) => (read(&path)?, moved(&shipped.parameters(), accepted), None),
        None => {
            let header = format!("# {}, {}.\n", names.common, names.scientific);
            let presets = register(&read(&root.join(PRESETS))?, names)?;
            (header, moved(&Family::default(), accepted), Some(presets))
        }
    };
    let text = values::write(accepted, &source, note)?;
    std::fs::write(&path, text).map_err(|e| format!("{file}: {e}"))?;
    let Some(presets) = registered else {
        return Ok(format!("{} rows of {file} set", rows.len()));
    };
    std::fs::write(root.join(PRESETS), presets).map_err(|e| e.to_string())?;
    Ok(format!(
        "{file} written with {} rows and registered",
        rows.len()
    ))
}
