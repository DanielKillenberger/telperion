---
satisfies: [R3, R4]
---
# fn-9-real-species-profiles-and-procedural.7 Expose species and anatomy through generated Wasm bindings

## Description
Expose species and anatomy through generated Wasm bindings. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `crates/telperion-wasm/src/params.rs`, `crates/telperion-wasm/src/lib.rs`, `src/browser/presets.generated.ts`, `src/browser/core.ts`, `scripts/test-wasm.mjs`
**Touches:** [crates/telperion-wasm/src/params.rs, crates/telperion-wasm/src/lib.rs, src/browser/presets.generated.ts, src/browser/core.ts, scripts/test-wasm.mjs]

### Approach
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
- [ ] Both species and Ordinary resolve by identity without relying on array order.
- [ ] Generated metadata exposes all implemented anatomy controls; roundtrip/invalid parameter and unknown identity checks pass.
- [ ] Foliage data and optional output requests retain correct ownership, release and bounds semantics.
- [ ] wasm:build regenerates consistently, Wasm tests and typecheck pass.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
