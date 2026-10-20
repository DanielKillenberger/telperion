---
satisfies: [R3, R4, R5]
---
# fn-299-a-walk-frame-grows-within-its-ends.3 Every shipped walk within its ends; ends unchanged; the stills

## Description
R3, R4, R5. Extend the repro to every shipped walk at the film's frames, prove the walk ends byte-identical to the base, render the oak-to-palm stills base against change, and run the gate and `npm test` once.

**Size:** M
**Files:** `crates/telperion-core/tests/units_bound.rs`, `.flow/evidence/fn-299-a-walk-frame-grows-within-its-ends/`
**Touches:** [crates/telperion-core/tests/units_bound.rs, crates/telperion-render/src/generation/cpu_builds_tests.rs, .flow/evidence/fn-299-a-walk-frame-grows-within-its-ends/**]

### Approach
- Film frames: t = ease((i + 1) / n) with the smoothstep at `crates/telperion-render/examples/headless/walk.rs:84-87`, n per walk 90, 90, 90, 120, 30, 90 (`WALKS` at `units_bound.rs:20-27`). The test must stay cheap enough for the ordinary suite; measure its time and record it.
- Ends: compare `families(a, b, 0)` and `families(a, b, 1)` grown species (or an artifact digest via `crates/telperion-core/examples/artifact_digest.rs`) against the base for every walk pair; record base and change digests in evidence.
- fn-298 device frames: if fn-298 has landed on the base, un-ignore its oak-to-palm frames (`cpu_builds_tests.rs`) and run them; otherwise assert `most_units` at most `FRAMES` (11) on frames 106 to 119 in the CPU test and record the un-ignore as open. The `unit_rounds[0]` frames 76 and 77 (oak to palm, seed 42, t = ease(76/120) = 0.69526 and ease(77/120) = 0.70681): build both on the device with the native wgpu test harness (`cpu_builds_tests.rs`, `every_tree_builds_on_the_device` with `SWEEP_ONLY`, or a scratch test kept out of the commit), under the limits on the branch, whether or not the rule changed their rows or count. A refusal is recorded with its message and its cause named (a census-projected `unit_rounds` limit, `build_size::limits`, is a different cause from the blend). If fn-298 has not landed, state that the build ran under the base's limits and that the result under fn-298's limits is open.
- Stills: `crates/telperion-render/examples/headless.rs` with the walk arguments in `headless/walk.rs`, oak to palm at age 40 at t = 0.5, 0.9 and 0.97, base build against change build in separate target dirs; at most four images viewed per capture; view every still the verdict rests on.
- Gate once at the end: `telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, then `npm test`.

### Investigation targets
**Required**:
- `crates/telperion-core/tests/units_bound.rs`
- `crates/telperion-render/examples/headless.rs:100-160`
- `crates/telperion-render/examples/headless/walk.rs`

**Optional**:
- `crates/telperion-render/src/generation/cpu_builds_tests.rs:53-140` - device sweep
- `docs/evidence-retention.md`

## Acceptance
- [ ] The test reads every shipped walk at the film's frames, three seeds, and passes.
- [ ] Every walk pair's t = 0 and t = 1 frames are byte-identical to the base, with the digests recorded.
- [ ] fn-298's oak-to-palm frames are un-ignored and build, or the CPU test holds them at most `FRAMES` and the un-ignore is recorded open; frames 76 and 77 at seed 42 were built on the device, and each either built or has its refusal and its different cause recorded.
- [ ] Stills of oak to palm at t = 0.5, 0.9, 0.97, base against change, are saved and viewed, with a one-line verdict each.
- [ ] The gate and `npm test` are green, run once.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
