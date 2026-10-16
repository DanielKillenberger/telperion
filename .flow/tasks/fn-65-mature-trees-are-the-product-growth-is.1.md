---
satisfies: [R1, R2, R3, R4]
---
# fn-65-mature-trees-are-the-product-growth-is.1 Implement mature trees as the product; growth hidden

## Description
The harness draws the direct build by default; the growth path is reachable only behind ?growth=1; the owner's rule is recorded in CLAUDE.md.

## Acceptance
- [x] R1 to R4 per the spec.

## Done summary
The harness draws the mature tree unless opened with `?growth=1`, which keeps the specimen path and its age controls as before. `growthFromQuery` in `harness/params.ts` with six unit cases. The owner's 2026-09-18 rule recorded in CLAUDE.md with the birch's 73,337 vs 590,410 nodes. No generator, renderer, preset, protocol or identity change.

## Evidence
- Commits: 3b635d6f3fa645914c621ad7b0730133bd5921c9
- Tests: npm run typecheck, npx vitest run (6 files, 83 tests)
- PRs:
