---
satisfies: [R2]
---
# fn-291-the-cpu-entry-builds-every-shipped.2 Settle R2 as the host decided: a measured sizing or memory fix, or a named documented refusal

## Description
Implement the R2 choice the host made from task 1's PROBE.md (recorded in this task's spec before start). Either (a) a sizing or memory fix in the wood stage so the failing presets build in the wasm32 entry, re-measured with task 1's probe, or (b) a wasm32-gated refusal predicted before allocation, named with its cause and remedy in the pattern of the phytomer ceiling, and stated in the README and the entry's types. A refusal of a shipped preset at its preset age needs the owner's word, quoted in the done summary.

**Size:** M
**Files:** decided by the host's choice; likely `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md`, `crates/telperion-core/src/pipeline/surface/build.rs`, `crates/telperion-core/src/pipeline/surface.rs`, `crates/telperion-core/src/pipeline/space.rs`, `src/browser/core.ts`, `README.md`
**Touches:** [.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md, crates/telperion-core/src/pipeline/**, crates/telperion-wasm/**, src/browser/core.ts, README.md, docs/**]

### Approach
- A refusal follows `OVER` / `foreseen` / `fits` in `crates/telperion-core/src/pipeline/space.rs` (wasm32-gated, predicted before the allocation), never a fallback that draws a different tree.
- A sizing fix re-runs task 1's probe and records the new table beside the old in PROBE.md.
- Red/green: a test that fails on the base for the chosen behaviour (the probe row, or a unit test on the prediction), then passes.

### Investigation targets
**Required**:
- `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md` — task 1's table and cause
- `crates/telperion-core/src/pipeline/space.rs:200-240` — the refusal pattern
- `crates/telperion-wasm/src/generate/tests.rs:185-210` — entry coverage test
**Optional**:
- `docs/principles.md:19` — the every-preset rule

## Acceptance
- [ ] The host's R2 choice is recorded in the task spec before work starts
- [ ] (fix) every probed case that failed now builds in the dist entry, re-measured with memoryBytes; or (refusal) the case is refused before allocation with a message naming cause and remedy, stated in README and the entry's types, with the owner's word quoted for any shipped preset
- [ ] A test fails on the base and passes after the change

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
