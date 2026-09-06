---
satisfies: [R2, R3, R4, R5, R6]
---
# fn-9-real-species-profiles-and-procedural.9 Run cross-seed visual QA and document the species workflow

## Description
Run cross-seed visual QA and document the species workflow. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `tests/browser/species.mjs`, `package.json`, `README.md`, `tests/migration/README.md`, `.flow/evidence/fn9/REPORT.md`
**Touches:** [crates/telperion-core/src/branching.rs, crates/telperion-core/src/branching/**, crates/telperion-core/src/foliage/**, crates/telperion-core/src/presets.rs, crates/telperion-core/tests/**, harness/GrowerDev.tsx, tests/browser/integration.mjs, tests/browser/species.mjs, scripts/*species*, package.json, README.md, tests/migration/README.md, .flow/evidence/fn9/**]

### Approach
- Add a compact headless runner using the existing integration capture pattern. Render the parent protocol, record camera/browser/renderer metadata and output hashes, preserve per-case results on failure, and write bulk output outside the checkout by default.
- Draw and record the 12 fresh seeds per species after initial calibration; run fixed and fresh measurements, capture the required subsets plus all numerical failures, and personally inspect whole-tree/bare/foliage views against references.
- Report each trait and case, separating numeric pass, visual assessment, missing evidence and actual owner feedback. Fix observed mismatches and retain them as regressions; record unresolved failures rather than declaring fidelity complete.
- Recheck Ordinary, Telperion and Laurelin after shared-rule changes, with known historical defects distinguished from new regressions. Record native generation time/count/size costs on the same host; no software-renderer GPU speed claim.
- Document public species selection, seed semantics, profile provenance, CPU-only generation/capture prerequisites and replay commands. Keep source manifest and compact final report durable; wire relevant checks into package scripts and run final integrated gates once.

### Investigation targets
**Required:**
- `tests/browser/integration.mjs` — headless setup and image output
- `tests/browser/migration.mjs:32-110` — fixed capture environment
- `README.md:51-83` — public commands and cost claims
- `tests/migration/README.md` — evidence/replay conventions
- `.flow/evidence/fn8/REPORT.md` — compact evidence pattern
- `package.json` — check entrypoints

### Quick commands
```bash
npm run rust:test
npm run rust:test:wasm
npm test
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm run typecheck
npm run build
node tests/browser/integration.mjs
node tests/browser/species.mjs --help
```

### Resumed architecture correction (2026-09-06)
The owner requested cleanup and completion after fetching pass 7. R6 authorizes targeted shared-rule corrections. The measured seed-2 window cannot be reached by rotations of frozen upper descendants. Inspect the spreading habit's hardcoded structural shell shedding, which can remove already-grown internal subdivisions despite full foliage retention. Spruce places most needle-bearing terminal runs below its descending secondary span; allocate subordinate shoots along that span during growth instead of post-hoc fan transforms. Remove superseded occupancy repairs if the upstream correction replaces them. Add small geometric regressions and re-run frozen numeric seeds and all reference views.

Frozen botanical targets, seed identities and honest geometry/visual evidence remain required. Historical pass reports' unchanged-count/protected-hash/no-lower-infill conditions have no attributed owner requirement in this spec and are superseded by this evidence-driven architecture repair. Record changed counts and outputs; do not weaken profile dimensions, resource/finite checks or natural/supernatural separation. Keep prior evidence recoverable through Git and replace bulky iterative output with a compact reproducible final receipt.
## Acceptance
- [x] All 24 recorded seeds per species have explicit numeric results; required fixed/fresh image sets are inspected against source references. Final replay passes 48/48; 39 required mature views and nine shared views are inspected, with representative anatomy supplements.
- [x] Trait-level evidence supports R3–R6, or outstanding failures remain explicit and prevent task completion; no unearned owner approval is recorded.
- [x] Ordinary and both Two Trees are checked for shared-rule regressions, with same-host CPU/count/size results and renderer identity reported.
- [x] Documented CPU-only replay succeeds, or a concrete environment limitation remains unassessed; missing captures never silently pass.
- [x] Native workspace tests, Wasm checks, harness tests, formatting/clippy, typecheck, build and relevant headless integration all pass; evidence remains compact. Final native suite, Wasm/browser integration, 106 harness tests, fmt, strict Clippy, typecheck and build pass. The final mature capture runner requires manual inspection and intentionally does not award a visual verdict.


## Current evidence
Final production is `0a8f343`. The final report, 48 numeric results, all 84 capture receipts and 59 scoped per-view observations are durable under `.flow/evidence/fn9/`. Required trait evidence supports R3–R6; optional occluded seam views and numeric-only cases remain explicitly unassessed. A retained real blunt terminal (spruce node5978) was corrected and reinspected without moving nodes or changing topology. Frozen targets and seeds remain unchanged.

The requested full-foliage spruce is served at `http://127.0.0.1:5184/?species=norway-spruce&seed=1`. Unknown species links now show a recoverable error; browser regression passes. The small fixture anatomy is shared across binding/render/UI tests while binding-specific resource limits remain explicit.

Final native suite (68 passed, 6 historical/diagnostic ignored), 106 harness tests, Wasm/browser integration, build, typecheck, fmt, strict Clippy and numeric/capture execution pass. Independent implementation review remains required before flowctl done. Historical reports and images are preserved intact under `experiments/fn9-iterations/`.

## Done summary
Replaced the post-growth occupancy transforms with retained species crowns, reachable oak infill and spruce shoots distributed along descending secondaries. Corrected genuine childless structural tips while preserving positions, handoffs and shared templates. Added recoverable species/seed viewer links, fresh Wasm before harness tests, one compact browser fixture and robust peg-view selection. Preserved historical iterations intact in experiments/fn9-iterations.

Final production 0a8f343 passes 48 fixed/fresh numeric cases, 68 native tests (6 historical/diagnostic ignored), 106 harness tests, Wasm/browser integration, build, typecheck, rustfmt and strict Clippy. All84 captures succeeded; 39 required mature and9 shared views plus11 representative anatomy views were inspected. Frozen targets/seeds remain unchanged; hidden optional seams and numeric-only cases remain unassessed. The retained blunt-tip counterexample now tapers. Owner feedback remains null.

Independent implementation review reached SHIP on d388ca0 with R1–R6 met. The quality correctness audit found no defects; both standards Should Fix findings were addressed. Report and durable evidence: .flow/evidence/fn9/REPORT.md and final/. Full-foliage spruce remains served at http://127.0.0.1:5184/?species=norway-spruce&seed=1.

stage: wave-dispatch - ran
stage: impl-review - ran (model: gpt-6-astra)
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: fd4a87b, d8812e6, 17fd6e5, a2eddc5, ec75925, 9da2c5b, 9cd68be, 0a8f343, d388ca0
- Tests: npm run rust:test — PASS, npm test — PASS, npm run build — PASS, npm run typecheck — PASS, cargo fmt --all -- --check — PASS, cargo clippy --workspace --all-targets -- -D warnings — PASS, BROWSER_URL=http://127.0.0.1:5187 PLAYWRIGHT_MODULE=/tmp/fn9-browser/node_modules/playwright/index.mjs CHROMIUM_EXECUTABLE=/tmp/fn9-chromium-gpu node tests/browser/integration.mjs — PASS, 48 numeric passes;84 capture successes;59 scoped view inspections — .flow/evidence/fn9/final/, Wasm entrypoint scripts/test-wasm.mjs imports the same fully executed tests/browser/integration.mjs suite., Independent implementation review SHIP — .flow/evidence/fn9/final/implementation-review.json, stage: plan-sync - skipped(config: planSync.enabled != true)
- PRs: