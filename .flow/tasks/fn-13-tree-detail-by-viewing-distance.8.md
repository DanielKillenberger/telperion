---
satisfies: [R1, R3, R5, R6]
---
# fn-13-tree-detail-by-viewing-distance.8 Build and validate a reusable multi-needle span library

## Description
Implement consensus stage 1's geometry half. Produce actual reusable multi-needle parts and measure the source-to-canonical fit before any voxel approximation.

**Size:** M
**Files:** crates/telperion-core/src/foliage/placement.rs, crates/telperion-wasm/src/lib.rs, experiments/fn13-rendering/assembly-engine.js, experiments/fn13-rendering/assembly-pages.js, experiments/fn13-rendering/canonical-library.js (new), experiments/fn13-rendering/canonical-library-check.mjs (new)
**Touches:** [crates/telperion-core/src/foliage/placement.rs, crates/telperion-core/src/foliage.rs, crates/telperion-wasm/src/lib.rs, experiments/fn13-rendering/assembly-engine.js, experiments/fn13-rendering/assembly-pages.js, experiments/fn13-rendering/canonical-library.js, experiments/fn13-rendering/canonical-library-check.mjs, experiments/assembly-boundary/DESIGN.md, .flow/evidence/fn13/candidates/canonical-library/**]

### Approach
- Adapt assembly-pages.js only as needed to consume the core-owned versioned recipe while preserving existing experimental ABI behavior and recorded route semantics. Build the new experimental Wasm into the task evidence directory; do not replace the running viewer's assemblies.wasm or production artifact. Verify recipe-equivalence fixtures before any live consumer migration.
- Extend only the existing experimental assembly output. Move experimental botanical lean/divergence/variant semantics into a versioned core-owned recipe; leave the default exact generation/API unchanged. Keep source-recipe equivalence fixtures for the behavior moved across the boundary, and label deliberate approximation changes separately.
- Use run identities to distinguish transport segments from biological spans. Inventory length, taper, phase and prototype variation on two distinct seeds before locking bins; select a bounded canonical family with rigid/uniform-scale instances and small reproducible variants. Declare fitting/held-out samples, measured memory ceilings and rejection policy before candidate fitting.
- Reuse geometry for multiple needle stations per part, expanding only per canonical library element. Emit per-tree instance references and fingerprints without full per-tree needle matrices. Measure how instance/library sizes change under a second seed and a changed botanical parameter.
- Compare exact source against the canonical assembly tree independently from any future voxel gate, including resolved near anatomy, whole silhouette/gaps and source-space bounds. Measure cold library construction, warm reuse, parameter invalidation and a second prototype ID; do not claim two same-kind seeds prove all-species fit.
- Carry content/recipe/version identity and reject unsupported deformation. Test empty output, malformed/nonfinite descriptors, invalid bin/prototype reference, canceled construction and replacement while another tree still references a shared part.

### Investigation targets
**Required**
- crates/telperion-core/src/foliage/placement.rs:520 — experimental_assemblies descriptors
- crates/telperion-wasm/src/lib.rs:167 — experimental feature output
- experiments/fn13-rendering/assembly-engine.js — experimental Wasm adapter
- experiments/fn13-rendering/assembly-pages.js:27 — current renderer-owned placement law
- experiments/assembly-boundary/DESIGN.md — core/renderer semantics and exact recovery
- .flow/evidence/fn13/candidates/integrated/DECISION.md — no-expansion baseline and limits

### Quick commands
```bash
npm run typecheck
node_modules/.bin/vitest run
cargo test --release --workspace
node experiments/fn13-rendering/canonical-library-check.mjs
```
Exercise the experimental feature build and its local Wasm artifact without rebuilding/replacing the owner's production artifact; record exact commands and source-equivalence evidence.

## Acceptance
- [ ] The versioned core-owned recipe and canonical parts preserve the agreed source-fit bar on declared fit/held-out samples, with near and whole inspection; a failed fit cannot advance to voxel evaluation.
- [ ] Two fingerprint-distinct trees share multi-needle geometry and retain independent topology/placements without per-seed expanded needle arrays; library growth, cold/warm costs and allowed transforms are measured.
- [ ] Exact default output remains behaviorally pinned; approximation provenance, multiple prototype IDs, changed parameters and content invalidation are exercised.
- [ ] Empty/invalid/canceled builds are explicit and release partial allocations; removing/replacing one tree preserves parts referenced by another.

## Done summary
Implemented the core-owned circular-v1 recipe and a bounded, shared multi-needle triangle library with independent tree instances, content invalidation and transactional ownership. Two fingerprint-distinct spruce trees pass the declared source-space and inspected native whole/near botanical fit; durable acceptance mapping, measurements and limits are in `.flow/evidence/fn13/candidates/canonical-library/DECISION.md`.

Baseline: green after documented browser environment correction (the initial missing-Playwright red is retained); new canonical module baseline none. Final typecheck,119 Vitest tests,Rust workspace/experimental-feature checks, exact eight-buffer output pin against production and pre-task base, and canonical/legacy checks pass. Final native candidate images are byte-identical to the inspected pairs. Initial seed42 near evidence was inadequate and is preserved alongside the added source-selected held-out anatomy pair. A pin-harness overread is preserved as inconclusive, then corrected to inspect the actual full exported ranges.

The shared library grows4232→4958 parts and58.6→67.5MB for the second seed; warm rebuilds add zero parts. Changed parameters/prototypes and empty/invalid/canceled/replacement/shared-survival cases are exercised. Task9 may investigate voxel fit against this geometry; no voxel, noise, motion,2ms hero,4ms forest or finalR1–R6 pass is claimed. The owner viewer and Vite5190 service were preserved; additional owned browsers closed, GPU observations nonexclusive.

stage: impl-review - skipped(config: REVIEW_MODE=none; explicit owner disabled automatic reviews)
stage: plan-sync - skipped(config: planSync.enabled=false)

Runtime status restored on 2026-09-08 from this committed completion receipt; historical validation and scope above are unchanged.
## Evidence
- Commits: d8ba05aa43af0a7e35f26ba120129ea988dc8747
- Tests: baseline: green (typecheck,119 unit tests,Rust workspace); initial browser baseline red due missing Playwright module, resolved by documented environment and rerun green; new canonical command baseline none, npm run typecheck, node_modules/.bin/vitest run, cargo test --release --workspace, cargo test --release -p telperion-core -p telperion-wasm --features experimental-assemblies --lib, node experiments/fn13-rendering/canonical-library-check.mjs, node experiments/fn13-rendering/assembly-check.mjs, PLAYWRIGHT_MODULE=/tmp/fn13-browser/node_modules/playwright/index.mjs BROWSER_URL=http://127.0.0.1:5190 CHROMIUM_EXECUTABLE=/usr/lib/chromium/chromium npm run test:browser (configured baseline, software regression), node .flow/evidence/fn13/candidates/canonical-library/build.mjs, node .flow/evidence/fn13/candidates/canonical-library/capture.mjs, node .flow/evidence/fn13/candidates/canonical-library/capture.mjs --heldout, node .flow/evidence/fn13/candidates/canonical-library/capture.mjs --verify, node .flow/evidence/fn13/candidates/canonical-library/capture.mjs --heldout --verify, node .flow/evidence/fn13/candidates/canonical-library/evaluate.mjs, Native source/canonical scoped botanical fit inspected by worker and conductor; final candidate10 PNGs byte-identical to inspected originals; no voxel/noise/motion/performance acceptance
- PRs: