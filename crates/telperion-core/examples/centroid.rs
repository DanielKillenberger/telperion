//! Writes the ordinary tree's `/space` rows (fn-215 R1): the shape is the
//! equal-weight centroid of the passed branching species, on the space's own
//! metric (the walk between two families, `blend::families`, folded so each
//! corner weighs the same); the size is the corners' mean. Every age's
//! lateral probabilities are multiplied by one factor, solved so the tree's
//! light-neutral expected kept phytomers at `AGE` are the geometric mean of
//! the corners' (host, 2026-10-08). Light only removes growth, so the
//! light-neutral count bounds the real tree's size from above.
//! `presets/ordinary.values` keeps every other row.
//!
//! cargo run -p telperion-core --example centroid
use std::{fs, path::Path};
use telperion_core::{
    blend,
    presets::{values, Preset},
};
use telperion_space::{expected_kept, Species};

/// The passed branching species. The date palm has no lateral branching to
/// average; the silver birch joins when fn-213 passes (fn-215 R4).
const CORNERS: [Preset; 3] = [
    Preset::EuropeanBeech,
    Preset::OregonWhiteOak,
    Preset::NorwaySpruce,
];
/// The age the size is matched at: the sheet's oldest.
const AGE: u32 = 80;
/// Bisection steps on the factor: past a double's resolution, so a rerun
/// lands on the same factor to the bit.
const STEPS: usize = 64;

fn main() -> Result<(), String> {
    let mut corners = CORNERS.iter().map(|p| p.parameters());
    let first = corners.next().ok_or("no corner")?;
    // The k-th corner a share 1/k of the way from the mean of those before.
    let mean = corners.enumerate().try_fold(first, |mean, (i, corner)| {
        blend::families(&mean, &corner, 1.0 / (i + 2) as f64).map_err(|e| e.to_string())
    })?;
    let mut space = mean.space;
    let shape = space.species.take().ok_or("no centroid species")?;
    let mut logs = 0.0;
    for corner in CORNERS {
        let species = corner
            .parameters()
            .space
            .species
            .ok_or("a corner has no species")?;
        let count = kept(&species)?;
        println!("{corner:?} keeps {count:.0} at {AGE}, light-neutral");
        logs += count.ln();
    }
    let target = (logs / CORNERS.len() as f64).exp();
    let factor = solve(&shape, target)?;
    println!("target {target:.0} kept at {AGE}, lateral factor {factor:?}");
    space.species = Some(scaled(&shape, factor));
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("presets/ordinary.values");
    let source = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut ordinary = values::read(&source)?;
    ordinary.space = space;
    let note =
        "The engine species (fn-215 R1): the corners' centroid, sized by examples/centroid.rs.";
    let text = values::write(&ordinary, &source, note)?;
    fs::write(&path, text).map_err(|e| e.to_string())
}

/// The factor on every lateral probability at which the shape's
/// light-neutral expected kept count at `AGE` is `target`.
fn solve(shape: &Species, target: f64) -> Result<f64, String> {
    let at = |f: f64| kept(&scaled(shape, f));
    if at(1.0)? < target {
        return Err(format!(
            "the centroid keeps fewer than {target:.0}: no factor below 1"
        ));
    }
    let (mut low, mut high) = (0.0, 1.0);
    for _ in 0..STEPS {
        let mid = 0.5 * (low + high);
        if at(mid)? < target {
            low = mid;
        } else {
            high = mid;
        }
    }
    Ok(low)
}

/// The species' expected kept phytomers at `AGE` with its light rows at 0.
fn kept(species: &Species) -> Result<f64, String> {
    let mut neutral = species.clone();
    for state in &mut neutral.states {
        state.shade_hazard = 0.0;
        state.shade_size = 0.0;
        state.balance_hazard = 0.0;
        state.leaf_girth = 0.0;
    }
    let kept = expected_kept(&neutral, AGE).map_err(|e| e.to_string())?;
    kept.ok_or("a light-neutral species has no expected count".into())
}

/// `shape` with every age's lateral probabilities multiplied by `factor`.
fn scaled(shape: &Species, factor: f64) -> Species {
    let mut species = shape.clone();
    for zone in species.states.iter_mut().flat_map(|s| &mut s.zones) {
        zone.lateral.iter_mut().for_each(|p| *p *= factor);
    }
    species
}
