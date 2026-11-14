## Goal

A species preset is a value file checked against the catalogue, not a Rust function, so adding or tuning a species writes data (fn-149's Accept writes values, never code). Fourth of four stacked specs.

## Decisions

- owner (2026-09-25, fn-152's earlier draft): presets as data, so the species runner's Accept stage (fn-149) writes values, never code.
- Astra review of fn-152: generate typed Rust values at build time from the value files, keeping the core serde-free with no runtime parsing, and preserve exact numeric values and explicit defaults.
- owner (2026-09-26): "/flow-next:flow this to its completion in one stack with pr's for each spec"
- One value file per preset holding only rows that differ from the family default; one writer function writes a file from an overlay. The `by_identity` vs `parameters()` difference (`maxTurnPerStep` None vs Some(35)) is preserved, its fix is its own spec.

## Acceptance Criteria

- **R1:** Every shipped preset a value file, Rust preset functions gone, byte-identical output; an unknown key or out-of-bounds value fails the build naming file and row — met.
- **R2:** The writer round-trips an overlay — met.
- **R3:** Workspace gate and `npm test` green — met.

## Result

Presets are `.values` files compiled to typed Rust at build time. Merged in PR #128.

## Open

The `by_identity` / `parameters()` defect remains for its own spec.

## Friction

- The field wasm budget (380,932 against 362,000) surfaced only after the gate -> proposal: a fast field-module budget check beside the core tests, or budgets recorded per commit.
