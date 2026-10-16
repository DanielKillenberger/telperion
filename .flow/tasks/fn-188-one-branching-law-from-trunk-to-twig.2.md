---
satisfies: [R2]
---
# fn-188-one-branching-law-from-trunk-to-twig.2 Small ablations: whole-tree girth, no terminal extension, axis-relative subdivision

## Description
Ablate inside today's pipeline: whole-tree girth, no terminal extension, axis-relative subdivision.

## Acceptance
- R2 of the spec.

## Done summary
Deleting the tip continuation is a clean win (built as fn-189); whole-tree girth barely moves ratios; axis-relative forks give the mesh but run away (4 to 71 leaders). A topology defect remained; the owner approved the probe.

## Evidence
- Commits: da88cb3d, 40361290
- Tests: cargo test --profile ci -p telperion-core --lib --no-fail-fast
- PRs: