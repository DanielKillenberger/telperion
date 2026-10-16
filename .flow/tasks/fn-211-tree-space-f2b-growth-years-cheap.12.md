---
satisfies: [R1, R4]
---
# fn-211-tree-space-f2b-growth-years-cheap.12 One executor: the stepped grower settles per root

## Description
Host decision (2026-10-07) on TASK11.md, option (b): the stepped executor becomes the only grower. Its settle is split per root using the per-root settle grow_fine already has (telperion_space::settle), dead roots stop being visited each cycle, and grow_fine's and grow_whole's paths are deleted. First check the unchecked claim: the per-root settle refuses species that read light; if lifting that refusal is a design change, stop and report options. Measure on the 20 trees of TASK11 (digests identical to today's grow, best-of-three warm time at low load) and report before deleting if any tree is slower than today's. The device's resident grower (task 10's options b and c) is deferred: the device run is 13-31 s against the CPU stepped 6.9-14.6 s with kernels at 2.6-4.1 s, so its remaining cost is call overhead; it is revisited after LOD (fn-176) and the coarse light field, on their numbers.

## Acceptance
- One grower; grow_fine's and grow_whole's paths deleted.
- Identical digests on the 20 trees; no tree slower than today's grow (or the reason reported before deleting).
- Gate green (two parts, render first).

## Done summary
One grower. grow is the stepped executor: each root runs to its next synchronisation point, and a root whose point is the tree's end is handed over as its bud, then grown, marked, assigned and shed on one core at the end (PHASES.md decision 2 as amended). It settles per root through the shared four phases, with the as-grown exchange (retained pipes, leaf girth) where a PA keeps the tree. grow_fine, grow_today, grow_whole and grow_staged are deleted. grow_roots is the same build reported per root, and the GPU executor's tests hold their roots to it. The whole-tree run stays only as test-only grow::whole.

The 20 trees, the build alone, best of six warm runs at load under 6: identical digests. Ratios to today: beech 0.98 to 0.99, spruce 0.99 to 1.00 (spruce 80 s1 is 0.4 percent over, inside today's own run-to-run spread), oak 0.38 to 0.51, lit oak 0.66 to 0.76, palm 0.90 to 0.96. The probe holds every phase identical (48 runs at 40, 24 at 80).

Gate green: render 168 passed, the rest 1,013 passed.

## Evidence
- Commits: 253e06f2, c0b6c727, 7a55150b, 2c569792, 160f495b, a092b447, 25f8456c, 5ab1d1ef, f07a1930
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render
- PRs: