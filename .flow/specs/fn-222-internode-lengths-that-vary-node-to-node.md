# Internode lengths that vary node to node

## Goal & Context

The silver birch (fn-213) reached its wrong-path stop on Astra's trait (j): 'repeated comb-like arrangements of drooping shoots along long, sparsely subdivided branches', a constructed regularity, after two rounds aimed at it (SHEET-3.md, SHEET-4.md). The cause, found in code: every internode of a physiological age within a grown unit has the same length (`geometry/lay.rs:148`), so the shoots borne along a bough sit at even spacing; no value varies it. Real shoots vary internode length node to node. Host decision (2026-10-07): a capability, not more values, and not light for the birch.

## Requirements

- **R1:** A per-PA setting for internode length variation: each internode's length scaled by a factor drawn from its phytomer's lineage key (mean one, so a unit's expected length is unchanged), the spread zero by default (every species byte-identical at zero); by degree (walk-tested within the bound of 30).
- **R2:** Both executors (fn-209's CPU reference and GPU) draw it identically under the integer contract (FINE-CONTRACT), the contract probe extended.
- **R3:** The closed form's expected lengths and the cut's predictor unchanged in expectation (shown).
- **R4:** Every passed species byte-identical; npm test and the gate green.

## Boundaries

The engine's layout only. Depends on fn-198. The birch's values are fn-213's.