---
satisfies: [R6]
---
# fn-257-the-site-grows-its-trees-on-the-device.4 Folding the hierarchy's narrow steps: probed, kept only if measured

## Description
The owner: "go for it if we're sure that it does the saving". DISPATCH.md bounds all tiny dispatches at 22-27 ms a tree. The hierarchy's step (`plan_hier.rs::plan_step`: per round and placing pass `target`, a scan, `join_place`, `decide`, a scan, `assign`, `follow`) runs about 84 dispatches a cycle.

**Touches:** probe: `crates/telperion-render/src/generation/cycle/plan_hier.rs` and a probe-only kernel under `#[cfg(test)]`, `growth_profile_tests.rs`; evidence `DISPATCH.md`. If folded: WIDER, the hierarchy's kernels (`hier_*.wgsl`, `plan_hier.rs`).

The probe, before any fold: the hierarchy's passes are not idempotent (they add into counts), so they are not repeated. Instead the step's dispatch sequence is replayed as no-op dispatches (an empty kernel bound to a scratch word, the same workgroup counts, in the same compute passes, immediately after each real one) under a probe-only switch: the added time is the launch and barrier cost of that many dispatches, the most folding them can save. The tree's bytes are checked equal with and without the replay, so the probe changes no work. Oak at 40, seed 1, warm, ten alternated pairs in a quiet window.

Kept only past the noise: if the bound is under twice the pairs' spread, the fold is dropped with the numbers. Past it, the narrow steps (a round whose new axes and blocks fit one workgroup) are folded into one dispatch with workgroup barriers, every preset byte-identical, the saving measured the same paired way.

## Acceptance
- DISPATCH.md: each pair, the means and spreads, the bound per tree, and the decision.
- If folded: every preset's tree byte-identical, the saving measured paired, the gate green. If dropped: no production code changes.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
