---
satisfies: [R1, R2, R3, R4]
---
# fn-65-mature-trees-are-the-product-growth-is.1 Implement mature trees as the product; growth hidden

## Description
The harness draws the direct build by default; the growth path is reachable only behind ?growth=1; the owner's rule is recorded in CLAUDE.md.

## Acceptance
- [ ] TBD

## Done summary
The harness now draws the mature tree, the same one the headless stills, `species:qa` and every owner verdict build, unless the page is opened with `?growth=1`, which keeps the specimen path and its age controls exactly as before. The flag is parsed by `growthFromQuery` beside the seed parser in `harness/params.ts`, with six unit cases. The growth controls are absent on the mature path rather than disabled. The owner's rule of 2026-09-18 is recorded in `CLAUDE.md` with the two node counts that forced it: the birch at seed 1 is 73,337 nodes on the direct build and 590,410 on the growth path.

No generator, renderer, preset, protocol or identity change. The growth path stays buildable and pinned.

Gates: `npm run typecheck` clean; `npx vitest run` 6 files, 83 tests passed after `npm run wasm:build` and `npm run render:build` in the fresh worktree (the parity tests read the built Wasm).

stage: impl-review - skipped(config: review.backend none; the host wrote and checked the diff, 4 files, 62 lines)
## Evidence
- Commits: 3b635d6f3fa645914c621ad7b0730133bd5921c9
- Tests: npm run typecheck (clean), npx vitest run (6 files, 83 tests passed)
- PRs: