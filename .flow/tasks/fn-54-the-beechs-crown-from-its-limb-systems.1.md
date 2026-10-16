---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-54-the-beechs-crown-from-its-limb-systems.1 Implement the beech crown from its limb systems

## Description
Measure B-BARE and set the beech's architecture; add `canopy.limbClumping` and `material.lobeShade`; set the leaf material against B-WHOLE; render the pairs.

## Acceptance
- [x] R1 architecture measured and set.
- [x] R2 limb clumping row.
- [x] R3 lobe-shade depth term, cost recorded.
- [ ] R4 leaf material level (measured, unmet: 56 vs 80).
- [ ] R5 owner verdict (moved to fn-62).

## Done summary
R1 to R3 delivered and merged, all railed and neutral byte-identical. Grid build 62 ms at submission for 4.73 M leaves; beech hero 10.23 to 10.24 ms. R4 unmet and R5 not accepting; both carry into fn-62 with the owner's round-22 notes (outline too regular, twig taper toward the rim, too dense at the shoulder and everywhere). The owner kept depth shading at 0.7. Stopped 2026-09-16 at owner shutdown.

## Evidence
- Commits: a061b46d (branch: 4cba7a0d, 3b6f4e7f, fe9ebd4e)
- Tests: cargo test --release -p telperion-core/-p telperion-render, clippy, npm run typecheck, npm test, node tests/species.mjs --measure-only (48 cases)
- PRs:
