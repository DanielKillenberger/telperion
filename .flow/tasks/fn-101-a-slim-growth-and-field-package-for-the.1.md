---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-101-a-slim-growth-and-field-package-for-the.1 Implement A slim growth-and-field package for the homepage

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
A slim growth-and-field package: `crates/telperion-field` (the core without geometry or JSON: species id and seed in, field bounds and a batch occupancy query out, 101,417 brotli bytes against the 172,127 target), the typed entry point `src/field/index.ts` exported as `telperion/field` (1,067 brotli bytes of JavaScript), the example voxelizer `src/field/voxelize.ts` with the fn-100 rules tested on a fixture grid, `docs/field-package.md`, and the experiment rewritten on the entry point reproducing both accepted sheets byte for byte. R1, R2, R4, R5, R6 and R7 are met and the workspace gate is green. R3 is missed on every tree (warm medians 273 to 649 ms against 250 ms) at parity with the full binding measured in the same minute; the time is the generator's growth plus the plan query that fn-100 measured above 250 ms on this same code, so the decision is the host's: accept parity or open a query-speed spec. NEEDS_HUMAN is recorded in the task file; `flowctl done` was not run.

Decision: a separate crate rather than a feature of telperion-wasm, so the artifact keeps its own name, `default-features = false` on the core cannot unify with the full crate's features, and `cargo tree` answers R2 directly. `presets::by_identity` and the catalogue tables moved out from under the `json` feature (params re-exports them) so both bindings resolve a species through one path; the full module's exports are unchanged and every existing binding test runs unedited. Vite's library mode inlined the Wasm into `field.js` as base64 (142,551 brotli); the entry names the file through a constant so the URL stays a run-time resolution and `build-wasm.mjs` places the Wasm beside `dist/field.js`. Follow-ups not built: stripping the 35 KB function-names section from the release Wasm; a flags-only field query for consumers that read no counts or limbs.

Evidence: `.flow/evidence/fn-101-a-slim-growth-and-field-package-for-the/RESULTS.md` (R1 to R7 with the numbers), `FRICTION.md` (three entries: the sheets' location, the loaded machine, R3's unchecked ceiling), `measure.mjs`; raw runs under `raw/` (ignored). The four fn71 receipts the render tests rewrote were restored from HEAD before the evidence commit.

Tier: session (host judgment); actual model not exposed by the host.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 044074525cb833c48055dcd9d694f2b28532386a, 217c502ee08b04d76f1972c50ec79fd2cb14ceb9
- Tests: cargo test --profile ci --workspace --no-fail-fast (119 suites, 891 passed, 0 failed, 21 ignored, exit 0; raw/gate-cargo-test-ci.log), cargo test --profile ci -p telperion-field (4 passed), npx vitest run (11 files, 123 passed; src/field/field.test.ts and src/field/voxelize.test.ts among them), npm run catalogue:check (5 species pass), node scripts/test-wasm.mjs (tests/browser/bindings.mjs unedited and green, tests/browser/field.mjs green), npx tsc --noEmit (exit 0), cargo fmt --check -p telperion-field; cargo clippy --profile ci -p telperion-field --all-targets (clean), LIMB_ORDER=3 node experiments/voxel-field/voxels.mjs 64 (byte-identical to the accepted order-3 sheet, sha256 66bbd3ad), WOOD_ONLY=1 CUT_METRES=1 node experiments/voxel-field/voxels.mjs 64 0.03,0.02,0.015 (byte-identical to the accepted wood-only sheet, sha256 5bf0a5db), node .flow/evidence/fn-101-a-slim-growth-and-field-package-for-the/measure.mjs (R3 rows, raw/r3-timings.json; R3 missed, NEEDS_HUMAN), baseline: none (the spec lists no Quick commands; the CLAUDE.md gate ran once at the end)
- PRs:
## Blocker (NEEDS_HUMAN, 2026-09-22)

R3 is missed on all six trees: warm medians of 349, 388, 406, 649, 289 and 273 ms against a 250 ms ceiling (`.flow/evidence/fn-101-a-slim-growth-and-field-package-for-the/RESULTS.md`, R3). The slim entry point runs at parity with the full binding measured in the same minute, so the owner's "no slower than today" holds; the time is `branching::generate` plus the plan query, which fn-100 measured above 250 ms on this same code, and the binding adds 1 to 17 ms. Making the query faster is a core change (a flags-only early exit would drop the leaf counts and limb ids the voxel rules read), which is the host's design call: accept parity as the criterion, or open a query-speed spec. Every other criterion (R1, R2, R4, R5, R6, R7) is met and the gate is green.
