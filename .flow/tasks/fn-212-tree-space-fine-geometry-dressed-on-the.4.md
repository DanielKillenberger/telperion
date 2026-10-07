---
satisfies: [R3]
---
# fn-212-tree-space-fine-geometry-dressed-on-the.4 Fine nodes in the tree's frame, and the whole-tree reductions, on the device

## Description
CONTRACT.md's probe, first part: the device turns fine roots' bearer-relative geometry into the tree's frame and reduces the whole-tree quantities (envelope and the eight reductions, the shoulder's front as a bounded summary) combined with the CPU's; host decision D3 option (a): a conservative ground bound computed before upload, without laid geometry, routes roots to the CPU; measure how many it routes.

## Acceptance
- Device frame and reductions agree with the CPU reference within the screen; reductions combined exactly.
- D3(a) built; the routed share measured.
- Two-part gate green.

## Done summary
The device draws fine roots in the tree's frame, as stage 2's conversion does (dress.wgsl, from Tree::attachments). It folds the height, the crown's lowest node, the widest reach, the shoulder's bins, the sifted crown nodes and the leaf box (fold.wgsl). Combined with the pipeline's own folds (executor::Folds, exact over any split) they come within 4.6 um of the CPU reference: beech and spruce at 40 and 80, seeds 1 and 7. Node strays are 0 to 1 per tree, D1''s residual. D3 was amended to (c) by the host: landed roots are masked on the device by the LANDED flag, about 7 % of the spruce's phytomers; option 1's estimate is 65 to 70 % device share. Gate: CPU half 988 passed, render half 159 passed. TASK4.md.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 63e8b272, eab7785a, 5a4122ca, 7afeb6c8
- Tests: cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci -p telperion-render dressed_at_40_and_80 -- --ignored
- PRs: