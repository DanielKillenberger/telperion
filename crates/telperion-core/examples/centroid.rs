//! Writes the ordinary tree's `/space` rows (fn-215 R1, fn-230 R3): the
//! passed branching species' point at equal weights, `blend::centroid`, at
//! `AGE`. The space's one blend rule gives it its shape, the centroid of every
//! row, and its size: one factor on the lateral probabilities so its
//! light-neutral expected kept phytomers at `AGE` are the corners' geometric
//! mean (host, 2026-10-08). `presets/ordinary.values` keeps every other row.
//!
//! cargo run -p telperion-core --example centroid
use std::{fs, path::Path};
use telperion_core::{
    blend,
    presets::{values, Preset},
};

/// The passed branching species. The date palm has no lateral branching to
/// average; the silver birch joins when fn-213 passes (fn-215 R4).
const CORNERS: [Preset; 3] = [
    Preset::EuropeanBeech,
    Preset::OregonWhiteOak,
    Preset::NorwaySpruce,
];
/// The age the size is matched at: the sheet's oldest.
const AGE: f64 = 80.0;

fn main() -> Result<(), String> {
    let corners: Vec<_> = CORNERS
        .iter()
        .map(|p| {
            let mut corner = p.parameters();
            corner.age = AGE;
            corner
        })
        .collect();
    let points: Vec<_> = corners.iter().map(|c| (c, 1.0)).collect();
    let space = blend::centroid(&points).map_err(|e| e.to_string())?.space;
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("presets/ordinary.values");
    let source = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut ordinary = values::read(&source)?;
    ordinary.space = space;
    let note = "The engine species (fn-215 R1, fn-230 R3): the corners' point at equal \
                weights, written by examples/centroid.rs.";
    let text = values::write(&ordinary, &source, note)?;
    fs::write(&path, text).map_err(|e| e.to_string())
}
