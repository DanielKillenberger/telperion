---
satisfies: [R1, R2, R3, R4]
---
# fn-230-points-between-species-stay-viable-trees.1 Implement points between species stay viable trees

## Description
TBD

## Acceptance
Every R-ID in the parent spec's requirements is satisfied; judge this task against the spec's criteria directly.

## Done summary
Every blend between engine species is now sized. `blend::centroid` (N points, with `blend::families` as N = 2) mixes the rows, then multiplies the lateral probabilities by the smaller of two factors: one solved against the light-neutral predicted kept phytomers and one against the grown phytomers, each at the ends' weighted geometric mean. The factor only shrinks and is 0 where no factor reaches the target. The ordinary tree is re-derived through this rule at age 80: factor 0.74192690392932 against fn-215's 0.7419269039293579. `space_walk` sizes each frame at its own age. All 30 walk frames at 40 and all 30 at 80 build within the budget, oak to spruce at 0.75 included (22.6 m). The slowest frame at 80 is spruce to ordinary at 0.25, 87.4 s, most of it the solve.

R2's visible-pop clause moves to fn-231 (host, 2026-10-08). The solve's cost is open for the harness, and the order-dependent rounding of count rows in an N-way centroid is a follow-up (declined Codex P2). Both are in SHEET.md.


stage: impl-review - ran (codex: fan-out NEEDS_WORK, one fix pass, re-review SHIP)

## Evidence
- Commits: 4636e459e7d85143a8a6ec7ec7ba782f7fe069f9, a63ab490ca9eac4bb0896cb9e6765de5966c2a52, 9fae300de73a3a0489969390017eef902656a4bd, 51d92b1733aec405ef4f971280bfdb37cd7116e7, 088bb1377f5e08e0d175519ee982b51ad4964fc9, 5c8ed3ff504f60318cc74aca147fe39e8398515f, 0289de2d29af56a65fc77152958fa8c493657eb4, f52b2c0695e779292b5116615241c8b4fdb143c6
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (render half, GPU lock), cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast (1 failure: generation_limit_guard, fixed in 088bb137; rechecked focused), cargo test --profile ci -p telperion-core --test space_walks --test space_presets --test catalogue_identity --test generation_limit_guard (after review fixes), npm test
- PRs: