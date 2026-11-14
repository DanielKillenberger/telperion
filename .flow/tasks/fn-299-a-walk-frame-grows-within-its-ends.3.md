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
R3 and R4 are met.
- **Film frames:** `every_film_frame_grows_within_its_ends` holds all 510 film frames of the six walks, at seeds 1, 7 and 42, within each seed's ends plus `SLACK` and at or below `FRAMES`.
- **Device frames:** `oak_to_palm_frames_build_on_the_device` builds fn-298's frames 76 and 77 (seed 42) and 106 and 119 (seed 1) on the device, matching the CPU.
- **Ends:** all 36 walk ends are byte-identical to the base.
- **Stills:** oak to palm at 0.9 and 0.97 is byte-identical to the base. At 0.5 it differs by one bare branch (T3.md, sheet `oak-palm-base-vs-change.jpg`).
- **Host option 1:** rule B now swaps only the eight timing rows.
- **Re-pins and rulings:** the walk digest is re-pinned, and three walk properties were updated on the host's rulings (palm middle within its ends; grown count binding at t = 0.8; oak-to-birch exception refreshed to 3,961,379).
- **Gate:** green, 1,425 passed and 111 ignored, in 2,399 s, before the three-seed test change, which passed focused in 46 s. `npm test` passed 177 of 177.
- **Open:** fn-298's ignored frames stay ignored here; the host un-ignores them on the release branch.
stage: impl-review - ran (codex panel: NEEDS_WORK on three seeds, then SHIP) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 7d0eb0fb7eccf11eda35e670657ec45b4c0f70ab, 2328161940fea7ceceaf240dcfefaa5c5c2497b7, e40fde44177b7a9061f851eeb105d0fea5001529, 060119518bdba88129d52f27faced9c0a01b4c5b, 6f94e76af81bc7011b3dcbfa3bc0692936d2f944, 5fd697b52ea059838a930e4b092cef0ece8d7380, b9685f28eeb5c91a8166d825f0786f752fdaf722, 29da7ecc375a2ddbe35c56cdc03a190585095386, ae5bd74940351e787147269e8a212d39a8a79b7b
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1425 passed, 0 failed); npm test (177 passed); cargo test --profile ci -p telperion-core --test units_bound (green after three seeds)
- PRs: