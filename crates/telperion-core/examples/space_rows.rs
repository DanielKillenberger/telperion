//! A tree-space species as the `/space` rows a preset's value file states
//! (fn-198 host decision 1), for a species spec to lay into its preset:
//!
//!   cargo run -p telperion-core --example space_rows -- birch [trunk ages, e.g. 2,4,5]
use telperion_core::presets::space::{self, Space};

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let name = args
        .next()
        .ok_or("usage: space_rows <species> [trunk ages]")?;
    let species = match name.as_str() {
        "beech" => telperion_space::beech(),
        "birch" => telperion_space::birch(),
        "oak" => telperion_space::oak(),
        "palm" => telperion_space::palm(),
        "spruce" => telperion_space::spruce(),
        _ => return Err(format!("no species {name}")),
    };
    let trunk = match args.next() {
        Some(list) => list
            .split(',')
            .map(|k| k.parse().map_err(|e| format!("trunk age {k}: {e}")))
            .collect::<Result<_, _>>()?,
        None => Vec::new(),
    };
    let space = Space {
        species: Some(species),
        trunk,
        ..Space::default()
    };
    for (path, cell) in space::rows(&space)? {
        println!("{path} = {}", cell.text());
    }
    Ok(())
}
