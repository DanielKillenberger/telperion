//! Compiles the preset value files in `presets/` into typed Rust tables, one
//! constant a file and the lookup from a preset to its table. The grammar is
//! judged here and the catalogue judges every row as the crate compiles
//! (`src/presets/table.rs`), each refusal naming the file and the line.
use std::path::Path;

#[path = "src/presets/grammar.rs"]
mod grammar;

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
    for name in &names {
        let text = std::fs::read_to_string(dir.join(name)).expect("a readable value file");
        let rows = grammar::parse(&text).unwrap_or_else(|why| panic!("presets/{name}:{why}"));
        let id = name.trim_end_matches(".values");
        let constant = id.replace('-', "_").to_ascii_uppercase();
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
    let path = Path::new(&std::env::var("OUT_DIR").expect("cargo sets it")).join("presets.rs");
    std::fs::write(path, out).expect("OUT_DIR is writable");
}

/// `date-palm` as its variant, `DatePalm`.
fn variant(id: &str) -> String {
    id.split('-')
        .map(|w| w[..1].to_ascii_uppercase() + &w[1..])
        .collect()
}
