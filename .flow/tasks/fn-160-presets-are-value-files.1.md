# fn-160-presets-are-value-files.1 Presets are value files

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Every shipped preset is a value file in `crates/telperion-core/presets/`, holding only the rows off the default family with the functions' comments carried over. `build.rs` compiles the files into typed tables, and a const check against the catalogue fails the build on an unknown row, a value off its bounds or of another kind, naming the file and line. The Rust preset functions are gone. `presets::values::write` is the writer, and Accept calls it. Generation digests for all 8 presets at seeds 1 and 7 are byte-identical to base, and the field wasm is 357,103 bytes against 356,393 at base. The `by_identity` difference is kept.

stage: impl-review - ran (codex; round 1 NEEDS_WORK with 2 findings fixed, round 2 SHIP)
## Evidence
- Commits: f595faf399b59393043ae41ee10bbb8ff284df2f, 327f5eee01c4706595e4842b001a6129b7da2531, 5596c2724f732fce86062a67cae8b81f5e215fff
- Tests: cargo test --profile ci --workspace --no-fail-fast (1054 passed, 0 failed, 22 ignored), npm test (13 files, 110 tests), npm run build && node scripts/artifact-budgets.mjs (field wasm 357103 <= 362000), cargo run --release -p telperion-render --example generation_digest (16 lines byte-identical to base)
- PRs: