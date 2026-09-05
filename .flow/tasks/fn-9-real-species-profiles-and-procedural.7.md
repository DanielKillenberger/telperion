---
satisfies: [R3, R4]
---
# fn-9-real-species-profiles-and-procedural.7 Expose species and anatomy through generated Wasm bindings

## Description
Expose species and anatomy through generated Wasm bindings. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `harness/skeleton-view.ts`, `harness/params.ts`, `harness/skeleton-view.test.ts`, `harness/params.test.ts`, `tests/browser/integration.mjs`, `crates/telperion-wasm/src/params.rs`, `crates/telperion-wasm/src/lib.rs`, `src/browser/presets.generated.ts`, `src/browser/core.ts`, `scripts/test-wasm.mjs`
**Touches:** [harness/skeleton-view.ts, harness/params.ts, harness/skeleton-view.test.ts, harness/params.test.ts, tests/browser/integration.mjs, crates/telperion-wasm/src/params.rs, crates/telperion-wasm/src/lib.rs, src/browser/presets.generated.ts, src/browser/core.ts, scripts/test-wasm.mjs]

### Approach
- Propagate changed generated types through existing harness conversion callers/tests so the integrated tree compiles and roundtrips. Task 8 owns new viewer affordances. Replace historical exact Ordinary counts only with explicitly documented current invariants; retain meaningful determinism/ownership coverage.
- Propagate the finalized native parameters and species identities through the existing fields! schema and packed-output contracts. Regenerate browser metadata with wasm:build; never hand-maintain a second schema.
- Replace positional family lookup and the hardcoded Two Trees-only public list with explicit identity lookup/catalogue semantics while keeping a clearly named ordinary baseline.
- Carry foliage biological counts and any actually required geometry buckets through the existing ownership boundary; keep optional structure/surface/foliage/field requests independent.
- Extend Wasm checks for both species, new-parameter roundtrip and rejection, disabled supernatural behavior, unknown IDs and release/disposal. Use small fixtures so these remain cheap.

### Investigation targets
**Required:**
- `crates/telperion-wasm/src/params.rs:7-92` — schema and ID mapping
- `crates/telperion-wasm/src/lib.rs:140-240` — output packing
- `src/browser/core.ts:5-35` — selection and ownership
- `src/browser/presets.generated.ts:1` — generated file contract
- `scripts/build-wasm.mjs` — metadata generation
- `scripts/test-wasm.mjs` — binding gate

### Quick commands
```bash
npm run wasm:build
npm run rust:test:wasm
npm run typecheck
```

## Acceptance
- [x] Both species and Ordinary resolve by identity without relying on array order.
- [x] Generated metadata exposes all implemented anatomy controls; roundtrip/invalid parameter and unknown identity checks pass.
- [x] Foliage data and optional output requests retain correct ownership, release and bounds semantics.
- [x] wasm:build regenerates consistently, Wasm tests and typecheck pass.

## Done summary
Implemented the identity-based Ordinary/oak/spruce/Two Trees catalogue, generated tagged habit and foliage anatomy controls, and nested explicit supernatural settings. Preserved all native controls through harness roundtrips and the Two Trees comparison. Exposed owned foliage subset metadata and biological counts without coupling optional output stages.

Verified native roundtrip/rejection tests, 104 harness tests, typecheck, strict binding-only Clippy, formatting, byte-identical regeneration, and the headless Wasm/species/ownership/viewer-retry gate. Intentional historical fixture updates and environment fallback are documented in `.flow/evidence/fn9/wasm-bindings.md`.

Implementation: 3c1d43b (pushed). No PR or merge. No new whole PNGs; existing `.flow/tmp/oak-{1,2,3}-whole.png` confirmed foliage-on. Next job: fn-9.8.
## Evidence
- Commits: 3c1d43b59c44d1443a6e164840aa776b42306433
- Tests: npm run wasm:build: passed; generated TS and Wasm byte-identical on repeat, npm run rust:test:wasm: passed; headless species anatomy/ownership/independent outputs and viewer retries, npm run typecheck: passed, npm test -- --reporter=dot: 104 passed, npm test -- harness/params.test.ts harness/skeleton-view.test.ts -t "presetToParams|carried native controls": 8 passed after final comparison naming change, cargo test --release -p telperion-wasm: 2 passed, cargo clippy -p telperion-wasm --all-targets --no-deps -- -D warnings: passed, cargo fmt --all --check: passed, git diff --check: passed
- PRs: