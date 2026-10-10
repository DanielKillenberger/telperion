//! Every shipped preset grown as the browser grows it, at its own seed and
//! age, against the browser's ceiling (fn-260 R2). fn-248 set Telperion's
//! age from `expected_kept`, a mean over seeds, and its seed kept 4.30M
//! against the 4M ceiling; only `npm test` caught it, after the gate.
use super::*;
use crate::presets::{by_identity, CATALOGUE};
use std::time::Instant;

/// Each shipped preset passes both of the browser's refusals: the
/// expected count before it grows and the kept count once grown. The
/// margin under the ceiling is printed for each.
#[test]
fn every_shipped_preset_keeps_under_the_wasm_ceiling() {
    let mut over = Vec::new();
    for &(_, id, _, _) in CATALOGUE {
        let started = Instant::now();
        let family = by_identity(id).unwrap();
        let input = GrowInput::of(&family);
        let (species, request) = asked(&input).unwrap();
        let foreseen = foreseen(species, request.age, Some(WASM_KEPT));
        let grown = telperion_space::grow_marked(species, request).map_err(engine);
        let (structure, _, _) = grown.unwrap_or_else(|e| panic!("{id}: {e}"));
        let kept: usize = structure.axes.iter().map(|a| a.phytomers.len()).sum();
        let margin = WASM_KEPT as f64 - kept as f64;
        println!(
            "{id}: seed {} age {} keeps {kept}, margin {margin:.0} ({:.1}%), {:.1} s",
            request.seed,
            request.age,
            100.0 * margin / WASM_KEPT as f64,
            started.elapsed().as_secs_f64()
        );
        if foreseen.is_err() || fits(&structure, Some(WASM_KEPT)).is_err() {
            over.push(id);
        }
    }
    assert!(over.is_empty(), "over the browser's ceiling: {over:?}");
}
