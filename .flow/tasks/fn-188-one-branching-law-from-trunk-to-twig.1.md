---
satisfies: [R1]
---
# fn-188-one-branching-law-from-trunk-to-twig.1 The judging lens: fixed camera, supersampled, coverage shown or ruled out

## Description
Build a fixed-camera judging lens and show or rule out coverage loss.

## Acceptance
- R1 of the spec.

## Done summary
Headless captures run at 4x MSAA; stills keep twig tone; a pinned beech seed-1 pose for every candidate.

## Evidence
- Commits: da88cb3d, 40361290
- Tests: cargo test --profile ci -p telperion-core --lib --no-fail-fast
- PRs: