//! Writes the ordinary tree's `/space` rows as the equal-weight centroid of
//! the passed branching species (fn-215 R1), on the space's own metric: the
//! walk between two families (`blend::families`), folded so each corner
//! weighs the same. `presets/ordinary.values` keeps every other row.
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

fn main() -> Result<(), String> {
    let mut corners = CORNERS.iter().map(|p| p.parameters());
    let first = corners.next().ok_or("no corner")?;
    // The k-th corner a share 1/k of the way from the mean of those before.
    let mean = corners.enumerate().try_fold(first, |mean, (i, corner)| {
        blend::families(&mean, &corner, 1.0 / (i + 2) as f64).map_err(|e| e.to_string())
    })?;
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("presets/ordinary.values");
    let source = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut ordinary = values::read(&source)?;
    ordinary.space = mean.space;
    let note = "The engine species (fn-215 R1): the corners' equal-weight centroid, by examples/centroid.rs.";
    let text = values::write(&ordinary, &source, note)?;
    fs::write(&path, text).map_err(|e| e.to_string())
}
