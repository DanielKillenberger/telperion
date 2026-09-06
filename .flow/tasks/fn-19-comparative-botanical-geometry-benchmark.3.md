---
satisfies: [R2, R3, R4, R5]
---
# fn-19-comparative-botanical-geometry-benchmark.3 Add matched anatomy captures and projected crown-gap diagnostics

## Description
Add an isolated benchmark capture adapter and image diagnostics for the frozen anatomical views.

Read species identities, view requirements and specimen cases from the protocol manifest. Species-specific anatomical targets are explicit data or adapters, not a hardcoded two-species inventory. Test an additional-species admission fixture and require an explicit unsupported-target/generator result until that species is implemented; never substitute oak/spruce geometry.

**Size:** M
**Files:** tests/browser/geometry-benchmark.mjs, tests/browser/geometry-benchmark-diagnostics.mjs, tests/browser/geometry-benchmark.test.mjs, .flow/evidence/fn19/visual-controls.json
**Touches:** [tests/browser/geometry-benchmark*.mjs, .flow/evidence/fn19/visual-controls.json]

### Approach
- Adapt the existing species runner's stage-based capture pattern in a benchmark-only script. Reuse installed harness functions without changing production viewer/generation behavior or factoring shared helpers while fn13 is changing them.
- Pin an available fn13 rendering-runner revision as the convergence-method reference. Inspect its actual file via the sibling worktree or Git history and record that revision; the file is not present in the fn9 checkout. Reproduce needed sampling/receipt semantics locally without requiring fn13's optimized renderer or importing a script with top-level execution side effects.
- Capture whole, bare, base, fork and connected-shoot views, using anatomy predicates and recorded selected targets. Existing fn13 anatomy targeting covers hero spruce only; add oak targets explicitly. Validate actual framing and connectivity, not only camera matrices.
- Emit native and converged beauty/foliage-coverage evidence with identical specimen/camera conditions. Apply task 1's crown ROI, threshold and connectivity definitions to coverage masks. Annotate projected gaps separately from surface/biological interpretation.
- Test with synthetic masks containing enclosed holes, exterior openings, clipped crowns and empty regions; include threshold/convergence sensitivity and mismatched protocol/camera/source rejection. Keep image threshold metrics diagnostic until inspected.
- Run small capture smoke cases in an isolated server/browser with explicit output directory. Schedule mature captures with fn13 and timings with fn13/fn18; do not kill unrelated processes or replace source foliage to save time.

### Investigation targets
**Required:**
- tests/browser/species.mjs:59-299
- tests/browser/species.mjs:303-379
- tests/migration/README.md:153-180
- harness/stage.ts
- .flow/evidence/fn9/final/captures.json
**Optional:**
- .flow/evidence/fn9/final/preview/

### Key context
Read tests/browser/rendering.mjs:65-189,234-269 from the fn13 worktree at an explicitly pinned revision; its manifest and convergence semantics are the reuse reference, not a new runtime dependency. A photograph lacking a controlled mask/calibration supports only qualitative comparison.

### Quick commands
- node --test tests/browser/geometry-benchmark.test.mjs
- node tests/browser/geometry-benchmark.mjs --help
- npm run typecheck

## Acceptance
- [ ] Analytic mask tests distinguish holes, exterior openings and outside-crown background, report projection/threshold units and reject crop/identity mismatch.
- [ ] A small oak and spruce capture smoke test inspects framing, actual connected anatomy and coverage/convergence receipts; unresolvable targets and failed convergence remain failed/unavailable.
- [ ] Controls detect clipping, changed camera, missing mask and incomplete artifacts without granting a visual or biological pass from numeric similarity alone.
- [ ] Native/converged images and diagnostics retain separate dispositions; no production generator, viewer or fn13 artifact is modified.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
