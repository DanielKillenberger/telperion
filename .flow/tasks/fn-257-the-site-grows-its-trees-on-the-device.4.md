---
satisfies: [R6]
---
# fn-257-the-site-grows-its-trees-on-the-device.4 Folding the hierarchy's narrow steps: probed, kept only if measured

## Description
The owner: "go for it if we're sure that it does the saving". DISPATCH.md bounds all tiny dispatches at 22-27 ms a tree. The hierarchy's step (`plan_hier.rs::plan_step`: per round and placing pass `target`, a scan, `join_place`, `decide`, a scan, `assign`, `follow`) runs about 84 dispatches a cycle over a few hundred to a few thousand new axes.

The probe, before any fold is written:
- **The bound, measured.** On the oak at 40 (seed 1), warm, paired against the base in a quiet window (each build alternated, ten pairs), the step's dispatches are recorded twice (a probe-only switch under `#[cfg(test)]` that repeats each of them; their writes are idempotent per pass, checked by the build's bytes being equal): the added time is what those dispatches cost, the most a fold could save.
- **Kept only past the noise.** If the bound is under twice the paired runs' spread, the fold is dropped with the numbers (DISPATCH.md). Past it, the narrow steps (a round whose new axes and blocks fit one workgroup) are folded into one dispatch with workgroup barriers, the trees' bytes unchanged on every preset, and the saving measured the same paired way.

## Acceptance
- DISPATCH.md: the paired measurement (each pair, the mean and spread of the base and the probe), the bound per tree, and the decision.
- If folded: every preset's tree byte-identical to the base (the existing agreement tests), the saving measured paired, the gate green. If dropped: no production code changes.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
