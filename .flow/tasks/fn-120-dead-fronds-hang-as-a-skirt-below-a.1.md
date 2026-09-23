---
satisfies: [R1, R2, R3, R4]
---
# fn-120-dead-fronds-hang-as-a-skirt-below-a.1 Implement fn-120-dead-fronds-hang-as-a-skirt-below-a

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Any rosette can now keep a skirt of dead fronds below its living crown. It takes three canopy rows (`skirt_fronds`, `skirt_pitch`, `skirt_length`) and three material rows (`leaf_dead_red/green/blue`). The dead fronds continue the crown's spiral and spacing below the oldest living frond, hang at the skirt pitch and are drawn at their share of a living frond's length. A count or length of zero draws nothing, so every shipped table except the palm keeps its bytes. The palm has a first skirt, and one small still shows it: `.flow/evidence/fn-120-dead-fronds-hang-as-a-skirt-below-a/date-palm-skirt-seed1.png`.

How the dead colour is drawn (the spec's unknown): it rides the existing crown draw. A leaf's scale is never negative, so the sign bit of the half float in word 2 marks a withered leaf. The core, `leaf.wgsl` and the browser's `leafScale` mask that bit, and the foliage shader draws a withered leaf in `leaf_dead` on both faces. Measured against giving the dead fronds their own draw: the bit costs no extra bytes per leaf, and the palm still renders in 6 draw calls both with and without the skirt. The bit also passes through the cull, clumping, the growth cache and the wasm buffers with no extra bookkeeping. A separate draw would need its own placement buffer, selection pass and level lists. A contiguous range of dead leaves would need to be tracked through the cull and every concatenation.

Decisions for the host to check:
- The palm's `shell_depth` is set to 1.0. At the family default of 0.45, the shell cull dropped 1,168 of the 1,760 dead leaflets, because a skirt hangs against the trunk, the deepest place inside the envelope. This setting also keeps 239 interior living leaflets that the cull used to drop. The rosette code is unchanged.
- The reference box grows with the skirt's deepest insertion. A skirted palm's living leaves therefore requantise by at most one box step: rotation and scale words are identical, positions move by no more than the step, and `tests/skirt.rs` checks this.
- The dead colour is stated directly in `leaf_dead_*`. The spec's phrase "aged from the back colour toward a dead colour" is not modelled as a blend, because a blend would need another row.
- The neutrals are 140 degrees and 0.8. They sit below each rail's end because the conformance jitter scales every value up to 1.25x.
- Palm values: 16 fronds, 165 degrees, 0.85 length, dead colour (0.34, 0.29, 0.22). They are first values. Source F1 says growers trim the lower leaves, which argues for a partial skirt. fn-82 owns the tuning.

Tests: `crates/telperion-core/tests/skirt.rs` checks the count, the withered bit, the living prefix, the droop, the length share, the neutral and the decode. The validation error cases (`skirt fronds` 129, `skirt pitch` 181, `skirt length` 1.5) are in `tests/rosette.rs` `every_new_row_is_refused_by_its_own_name`. The dead colour rails are in `material/tests.rs`. The capability test covers `dead-frond-skirt`.

Not verified: vitest and the TypeScript typecheck, because this worktree has no `node_modules`. The `leaf.ts` change is one mask.

Follow-ups: the FRICTION.md entries (a checklist for new rows, a unit test that every default times 1.25 stays on its rail, a slice in place of the fixed-length refusal array).

Baseline: none run before the edits. By project rule the gate runs once, at the end of the task. It was green on the third run: the first two runs failed on a fixed-length array and on the conformance jitter, and both are fixed.

Tier: session (actual model: claude-opus-5-5)

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: e1315e0a217e89e789e75c48dd5cfb3d1ce2a255
- Tests: cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-core --test skirt --test rosette --test capability --test sweep --test generation_limit_guard --test geometry_benchmark, cargo test --profile ci -p telperion-jev --test dial_table --test tuning_engine, cargo run --profile ci -p telperion-render --example headless -- --preset date-palm --seed 1 --size 768x768 --camera '{"fill":3.5,"targetHeight":0.88,"elevation":5}'
- PRs: