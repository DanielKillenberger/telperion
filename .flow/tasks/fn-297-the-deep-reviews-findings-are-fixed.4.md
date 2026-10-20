---
satisfies: [R1, R5, R6, R7]
---
# fn-297-the-deep-reviews-findings-are-fixed.4 One Wasm initialisation, an honest ledger, and a one-turn rosette divergence

## Description
Three small, independent fixes with disjoint files, grouped so the gate runs once: R1 (TypeScript loader), R5 (renderer ledger), R6 (parameter bound and boots upload).

**Size:** M
**Files:** `src/field/index.ts`, `src/browser/render.ts`, a shared loader module and its vitest test; `crates/telperion-render/src/lib.rs`, `crates/telperion-render/src/curve.rs`; `crates/telperion-core/src/pipeline/foliage/canopy/params.rs`, `crates/telperion-render/src/generation/resident/boots.rs`, generated tables and `docs/parameters.md`, `crates/telperion-core/tests/catalogue_identity.rs`
**Touches:** [src/field/**, src/browser/render.ts, src/browser/*.test.ts, src/*.ts, crates/telperion-render/src/lib.rs, crates/telperion-render/src/curve.rs, crates/telperion-render/src/curve/**, crates/telperion-render/src/foliage.rs, crates/telperion-render/src/generation/resident/boots.rs, crates/telperion-render/src/generation/resident/boots_tests.rs, crates/telperion-render/src/generation/**_tests.rs, crates/telperion-core/src/pipeline/foliage/canopy/params.rs, crates/telperion-core/tests/**, docs/parameters.md, src/browser/parameters.generated.ts, src/browser/presets.generated.ts]

### Approach
- R1: both entries import the default `init` of `render/telperion_render.js`; move the one promise into a shared module both use, keeping the literal `new URL("./telperion-render.wasm", import.meta.url)` form `vite.config.ts:14-21` rewrites; reset on rejection. Test with `vi.mock` of the glue (pattern `src/browser/render.test.ts:15-40`): two overlapping calls instantiate once; a rejected first call lets a retry succeed. Red first against the current two promises.
- R5: `lib.rs:266-274` `submit_organ` releases (`foliage.rs:161-172` `release()`) and clears the scene's organ when there is none; `curve.rs:203-211` adds the two 16-byte feedback buffers (`curve/feedback.rs`). Test after the pattern of `grow_in_peak.rs:88-130`: Laurelin then oak, `gpu.ledger.live()` equals the reported tree bytes and the organ holds 0.
- R6: `params.rs:210-212` bound to `Bounds::closed(-360, 360)`; `boots.rs:111` reduces in f64 (idiom `stations.rs:372-380` / `498-503`) before narrowing. Regenerate `docs/parameters.md` and `parameters.generated.ts` by running the `parameter_reference` integration tests with `TELPERION_WRITE_REFERENCE=1` (`crates/telperion-core/tests/parameter_reference.rs`), and `presets.generated.ts` by `scripts/build-wasm.mjs:28-36` and update the digest in `catalogue_identity.rs:151-180`. Test CPU = device leaf bases at 359 deg (`boots_tests.rs:318` pattern), red first if it differs today.

### Investigation targets
**Required:**
- `src/field/index.ts:15-60`, `src/browser/render.ts:230-250`, `vite.config.ts:10-55`
- `crates/telperion-render/src/lib.rs:255-320`, `crates/telperion-render/src/curve.rs:90-215`
- `crates/telperion-render/src/generation/resident/boots.rs:100-120`, `crates/telperion-core/src/pipeline/foliage/canopy/params.rs:200-215`
**Optional:**
- `src/wasm-source.ts`, `crates/telperion-render/src/generation/resident/boots_tests.rs`

### Key context
`divergence` keeps its bound (spec Decision Context). A shipped preset outside +-360 would be a regression: check `presets.generated.ts` and the catalogue packets before narrowing.

## Acceptance
- [ ] R1: overlapping `createRenderer`/`prepareField` initialisations instantiate once; a failed init can be retried; vitest red then green
- [ ] R5: Laurelin then oak leaves the ledger equal to the reported tree bytes with the organ at 0; curve bytes count the feedback buffers
- [ ] R6: rosetteDivergence bounded to -360..360 everywhere it is generated; reduced in f64 before upload; CPU = device at 359 deg; shipped values unchanged
- [ ] `npm test` and the gate green once at the end under telperion-heavy; codex impl review SHIP

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
