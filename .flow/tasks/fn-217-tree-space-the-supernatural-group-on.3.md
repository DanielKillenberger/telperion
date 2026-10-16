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
Built per DESIGN.md 8-9: bark metre rows scale with S1; margin band in leaf space with area-weighted sub-pixel mix; organ takes no canopy normal; `/bloom/petalCurl`; floret eye and fractional stamens; every new row has its override. M0: sampling 1.004-1.005 of reference, levels 0.923-0.959, so M1. Stills viewed: curl flare, hanging racemes, five-petal blossom, invisible margin change, green crown with gold edges. Blossom at whole-tree distance not reached by values (moved to fn-234). Review: codex NEEDS_WORK, fixes, SHIP. npm test 143 passed.

## Evidence
- Commits: 5bc4cde1c8ad700cac83862f51d9ccc3d020a88f, 649b03bf545e41d64ddd50b6e2ffc0b60af95fb7, dc840aa967edf41d0c7f2316b42db52e2e00569c, d8b68137f12298ac94e37b8354ba93bf60b2b085, 5606a4031d4e31322b95b8a8b06008bde7734205, f11c4ee4c196d228f986cf00767d1d1a9a5363e4, f8ada72f782ad94489e2852c5e588d69987bd8c3, 47cd0ee4c029c5a1c54afcc4d9004b6dca4626cd, 89c724049e4aa97a7bf7d9704f9122892a134cbd, 63209d856f6518f4b07061739554c08f9e6450f2, 30f09aabf33acbe8e6a3b3b4956d0152c319a881, d380fa988a2d24436eba3a3c461150b1d6fff319, e8657ecd2d9c8888507fb835bc5db92ff42acb25
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, npm test
- PRs: