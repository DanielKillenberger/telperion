---
satisfies: [R1, R2, R3, R4]
---
# fn-214-the-field-from-the-structure-above-the.1 The coarse field probe

## Description
The spec's R1 to R4 as a probe: build the option, measure against 0.1.4 and the full field for oak, spruce and palm at the site's request, render the site's voxels beside today's for the host. Read FIELD-PATH.md (fn-198's evidence on branch fn-198-field-measure) first; its scratch harness is the measuring tool.

## Acceptance
- The option builds the structure above the cut with fine roots as expected occupancy; the full field unchanged.
- Time, memory and cubes against 0.1.4 and the full field.
- The site's voxel render beside today's, for the host.
- npm test and the gate green.

## Done summary
See c1-sum: the cut option built; spruce and palm pass the gate (spruce 242-249 ms and 43 MB against 0.1.4's 291-333 ms and 87 MB; renders viewed by the host); the oak waits on fn-211 (host option 1); the site keeps the full oak meanwhile.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 6381e130, 3be28d3c, 62c35fd7, 7253e19f, 251399ea, a128d0b0
- Tests: npm test, cargo test --profile ci --workspace --no-fail-fast
- PRs: