# fn-158-stages-take-typed-inputs-behind-one.1 Stages take typed inputs behind one pipeline boundary

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Stages take typed inputs (GrowInput, PlanInput, SurfaceInput, LeafInput) derived once in pipeline::build, and live in private pipeline modules; only pipeline::build and pipeline::executor (grow, expand, element, Expansion's CPU steps) are public, with the data types through pipeline::contract. The GPU executor and the growth path build through the interface; 42 core integration tests moved into private suite modules; the eight core examples and jev's pins build through pipeline::build; trybuild compile-fail tests with positive controls hold the boundary from render and wasm. Every shipped preset is byte-identical (generation_digest), timings are within spread, the slim wasm shrinks 408 bytes. The date palm's species_measure/geometry_benchmark/measure/node_buffer outputs and its catalogue skeleton pin now include its leaf bases (host decision 3). Evidence: .flow/evidence/fn-158-stages-take-typed-inputs-behind-one/RESULTS.md.

Tier: session model (Opus 5.5)

stage: impl-review - ran (codex fan-out NEEDS_WORK x3 -> fixes d1c52a3e -> re-review SHIP)
## Evidence
- Commits: b307a2590dd646cdc6525985abbbe0c0e525afad, e7ed9129c1c6643d5e1e088235cff0cc88f1df06, 4a8295f33fe62686fcd30dbc1e1920a0aa6917a1, 301bf65f7e31d228b4d83341fe82d3bee58db568, dc5dfea3d7903b164f1eb475851483b485aae120, eb026c14764c3b4d44dcddffe53c6a979533096a, b6ce783acffa618e01fe74a7f1cd4f934a8d8b1b, 1ca4fda6452b5570b7ddb7e2f299f982c2c71d66, 19c3b5a0471076cace50677b42242236ec7b9c8d, 4ee081b12d5205909af2c383aa635302c5cbf17a, f27820f55e7095e223b7fb48d770286db69faa47, d1c52a3e4f7c452bb0164f2ae0d8cfd26650cc8e, ad5efb0d2725eda79e840f3a19f2bbd37e9deae2
- Tests: cargo test --profile ci --workspace --no-fail-fast (1046 passed, 0 failed, 22 ignored), npm test (130 passed), node scripts/artifact-budgets.mjs (pass), generation_digest: 16/16 lines byte-identical to digests-base.jsonl, cargo test --profile ci -p telperion-render --test boundary; -p telperion-wasm --test boundary (trybuild, positive controls)
- PRs: