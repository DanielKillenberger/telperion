# Leaving angles and turning rates that vary axis to axis

## Goal & Context

The silver birch (fn-213) has Astra's 'comb' named on sheets 8 and 9: strands with 'repeated hooked starts and straight, parallel drops', in ranks. Cause, checked in code by the run (SHEET-9.md, 'Code check before any round 10'): every lateral of a physiological age leaves its bearer at one constant `insertion` (geometry.rs:183, :208; geometry/single.rs:89) and turns toward `form.elevation` at one constant `form.tropism` (geometry/lay.rs:148-150); only roll is drawn per axis (geometry.rs:177-178). Real shoots vary in both. Host decision (2026-10-07): a capability, the sibling of fn-222's internode spread, not more values.

## Requirements

- **R1:** Two per-PA settings, `insertion_spread` and `tropism_spread`: each axis's insertion angle and tropism scaled or offset by a factor drawn from its own lineage key (new keys), mean-preserving (insertion offset of mean zero, tropism factor of mean one), bounded, zero by default with no draw (every species byte-identical at zero); by degree (walk-tested within the bound of 30).
- **R2:** Both executors (CPU reference and GPU) draw them identically under the integer contract, the contract probe extended; the GPU lay applies them.
- **R3:** The closed form, which reads `insertion` as a constant (closed_form/events.rs:156, :236, :268; kept/path.rs:256), stays correct in expectation, or its approximation is stated and its error measured; the cut's predictor unchanged in counts.
- **R4:** Every passed species byte-identical; npm test and the gate green.

## Boundaries

The engine's layout only. Depends on fn-198 (F4) with fn-222. The birch's values are fn-213's.