## Conversation Evidence

> owner (2026-09-25, fn-152's earlier draft): presets as data, so the species runner's Accept stage (fn-149) writes values, never code.
> Astra review of fn-152: generate typed Rust values at build time from the value files, keeping the core serde-free with no runtime parsing, and preserve exact numeric values and explicit defaults.
> owner (2026-09-26): "/flow-next:flow this to its completion in one stack with pr's for each spec"

## Goal & Context
<!-- scope: business -->

A species preset is a value file checked against the catalogue, not a Rust function. Adding or tuning a species then writes data. Fourth of four stacked specs. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists.** Presets are functions in `presets/species.rs` and `presets/materials.rs`. `presets::by_identity` and `Preset::parameters()` differ: `maxTurnPerStep` is `None` against `Some(35)` (`presets.rs:50`). [checked]
- **Value files.** One value file per shipped preset, holding only the rows that differ from the family default. A build step turns each file into typed Rust, so the core stays serde-free. The catalogue checks every key and value at build time. [paraphrase]
- **Writer.** One function writes a preset value file from an overlay; fn-149's Accept calls it. [paraphrase]
- **Kept as it is.** The `by_identity` and `parameters()` difference is preserved, not fixed; fixing it is its own spec. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Every shipped preset is a value file, and the Rust preset functions are gone. Every preset is byte-identical in mesh, field and metrics, with exact numeric values preserved. Errors: an unknown key or an out-of-bounds value fails the build with the file and row named. [paraphrase]
- **R2:** The writer round-trips: an overlay written as a value file builds the same family. [paraphrase]
- **R3:** The workspace gate and `npm test` are green. [paraphrase]

## Boundaries
<!-- scope: business -->

- Not fn-149's Accept stage itself. Not the `by_identity` defect.

## Strategy Alignment

- Serves "The catalogue": a species is data. [strategy:The catalogue]
