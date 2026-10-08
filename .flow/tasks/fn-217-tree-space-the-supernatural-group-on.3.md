---
satisfies: [R1, R2, R3]
---
# fn-217-tree-space-the-supernatural-group-on.3 Floret form, horns and blossom at distance

## Description
DESIGN.md sections 6 and 7, release half (b): F1 eye tint, F2 stamens (width, fractional count, LOD stated), F3 organ lit as a surface; the horn-form check for Laurelin with existing values first (throat, flared mouth, silhouette, racemes as hanging clusters) and a stop for the host if unreachable; M0 measured per section 7.4, then M1 or M2 by the measurement, M2's coverage verified; crown coverage at legendary scales judged on stills. Every passed species unchanged where the rows are neutral.

## Acceptance
- F1-F3 built, neutral identity.
- Five-petal blossom and horn racemes on stills beside references, viewed.
- M0 measured; M1 or M2 by it; distance stills viewed.
- Gate and npm test green.

## Done summary
Task 3 of fn-217 (DESIGN.md 6 to 9, release half (b)) is built. Proof and readings are in .flow/evidence/fn-217-tree-space-the-supernatural-group-on/TASK3.md; HORN.md holds the first horn check.

What was built:
- 8.3: the bark's metre rows are drawn at the group's scale, the applied material to the bit at scale 1.
- 8.2: the margin band is set in leaf space and tends to its measured area share under a pixel. Width 0.1: near 0.115, far 0.117. Width 0.5: near 0.709, far 0.712.
- F3: the organ takes no canopy normal.
- 9.1: `/bloom/petalCurl`; positive flares the tips away from the axis, and zero is byte-identical.
- F1: the floret's eye, `/material/bloomEye*`.
- F2: stamens. The count is fractional; the stamens are kept on levels finer than their reach, `hypot(length, width / 2)`.
- Each new `/bloom` row has its `/supernatural/bloom` override, and the pins were re-proven by stripping the new rows.
- The organ mask, an instrument.

M0:
- Sampling keeps 1.004 to 1.005 of the reference.
- The organ's levels keep 0.923 to 0.959 of the finest geometry.
- M1 (values) by the measurement.

Stills viewed:
- curl;
- the 9.2 raceme beside Laburnum (it hangs as strands at middle distance);
- the blossom beside the cherry corymb;
- the margin before and after on the beech and the oak, with no visible change (the birch has no engine species);
- the gold rim at whole-tree distance (green, no longer gold);
- Laurelin's test tree at scale 3;
- the cherry-dense whole tree.

Open for the host:
- Blossom at whole-tree distance is not reached by values on the beech: florets are hidden by the outer leaves and read grey.
- The close-up raceme at a share of 1 is a dense curtain.
- The birch's margin cannot be checked until the birch has an engine species.
- FRICTION.md has two task-3 entries.

Tier: not provided by the conductor.
baseline: none (the spec defines no Quick commands).
Gate: the render half under the GPU lock had one failure, conformance on the word "blade", fixed and re-run focused. The rest of the workspace was green. npm test: 13 files and 143 tests, after regenerating presets.generated.ts. The review fixes were re-checked with focused tests.

stage: impl-review - ran (codex; NEEDS_WORK -> one fix pass -> SHIP; the re-review's one P2, the resident path, fixed in e8657ecd)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5bc4cde1c8ad700cac83862f51d9ccc3d020a88f, 649b03bf545e41d64ddd50b6e2ffc0b60af95fb7, dc840aa967edf41d0c7f2316b42db52e2e00569c, d8b68137f12298ac94e37b8354ba93bf60b2b085, 5606a4031d4e31322b95b8a8b06008bde7734205, f11c4ee4c196d228f986cf00767d1d1a9a5363e4, f8ada72f782ad94489e2852c5e588d69987bd8c3, 47cd0ee4c029c5a1c54afcc4d9004b6dca4626cd, 89c724049e4aa97a7bf7d9704f9122892a134cbd, 63209d856f6518f4b07061739554c08f9e6450f2, 30f09aabf33acbe8e6a3b3b4956d0152c319a881, d380fa988a2d24436eba3a3c461150b1d6fff319, e8657ecd2d9c8888507fb835bc5db92ff42acb25
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, npm test
- PRs: