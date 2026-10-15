//! Compiles the preset value files in `presets/` into typed Rust tables, one
//! constant a file and the lookup from a preset to its table. The grammar is
//! judged here and the catalogue judges every row as the crate compiles
//! (`src/presets/table.rs`), each refusal naming the file and the line. An
//! engine species' `/space` rows are judged here, by their place and form
//! and by the engine itself (`src/presets/space.rs`).
use std::path::Path;

#[path = "src/presets/grammar.rs"]
mod grammar;
// The writer half is the crate's; the build only reads.
#[allow(dead_code)]
#[path = "src/presets/space.rs"]
mod space;

fn main() {
    println!("cargo::rerun-if-changed=presets");
    let dir =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").expect("cargo sets it")).join("presets");
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .expect("crates/telperion-core/presets holds the value files")
        .map(|e| {
            e.expect("a readable entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.ends_with(".values"))
        .collect();
    names.sort();
    let mut out = String::new();
    let mut arms = String::new();
    let mut spaces = String::new();
    for name in &names {
        let text = std::fs::read_to_string(dir.join(name)).expect("a readable value file");
        let rows = grammar::parse(&text).unwrap_or_else(|why| panic!("presets/{name}:{why}"));
        let id = name.trim_end_matches(".values");
        let constant = id.replace('-', "_").to_ascii_uppercase();
        let (engine, rows): (Vec<_>, Vec<_>) = rows.into_iter().partition(|r| space::holds(r.path));
        judge(name, &engine);
        out +=
            &format!("pub const {constant}_SPACE: &[(&str, crate::presets::space::Cell)] = &[\n");
        for row in &engine {
            out += &format!(
                "    ({:?}, crate::presets::space::Cell::{:?}),\n",
                row.path,
                space::Cell::of(row)
            );
        }
        out += "];\n";
        spaces += &format!("        Preset::{} => {constant}_SPACE,\n", variant(id));
        out += &format!("crate::preset_values!({constant}, {name:?}, [\n");
        for row in rows {
            // The shortest text that reads back as the same double.
            let value = row.number();
            out += &format!(
                "    ({}, {:?}, {:?}, {value:?}),\n",
                row.at, row.path, row.form
            );
        }
        out += "]);\n";
        arms += &format!("        Preset::{} => {constant},\n", variant(id));
    }
    out += &format!(
        "pub fn rows(preset: Preset) -> &'static [super::Value] {{\n    match preset {{\n{arms}    }}\n}}\n"
    );
    out += &format!(
        "pub fn space(preset: Preset) -> &'static [(&'static str, crate::presets::space::Cell)] {{\n    match preset {{\n{spaces}    }}\n}}\n"
    );
    let path = Path::new(&std::env::var("OUT_DIR").expect("cargo sets it")).join("presets.rs");
    std::fs::write(path, out).expect("OUT_DIR is writable");
}

/// Refuses, naming the file and the line, a `/space` row off its place or
/// form, and a species the engine cannot draw.
fn judge(name: &str, rows: &[grammar::Line]) {
    let cells = rows.iter().map(|r| (r.path, space::Cell::of(r)));
    let read = space::read(cells).unwrap_or_else(|why| {
        let at = rows
            .iter()
            .find(|r| why.starts_with(r.path))
            .map_or(0, |r| r.at);
        panic!("presets/{name}:{at}: {why}")
    });
    if let Some(species) = &read.species {
        if let Err(why) = species.validate() {
            panic!("presets/{name}: the engine refuses its species: {why}");
        }
    }
}

/// `date-palm` as its variant, `DatePalm`.
fn variant(id: &str) -> String {
    id.split('-')
        .map(|w| w[..1].to_ascii_uppercase() + &w[1..])
        .collect()
}
