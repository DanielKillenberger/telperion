---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8]
---
# fn-86-one-leaf-in-twelve-bytes.1 Implement One leaf in twelve bytes

## Description
Store each leaf as three `u32` words against a parameter-derived box, read by the three shaders.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Encoding in 5b49227c (R1, R3, R5, shader half of R6). Second run: growth half of R6 (cached leaves decode at a later age), R4 (96 `species:qa` cases pass; retained-count moves at most 0.00177%), R7 (spruces 1,763 MB), R8 (four stills on an RTX 3080 after the gates), R2 confirmed against the owner's amended bound, and a red `catalogue:check` fixed by regenerating the pages (50f7e9d5). Gates green at e61c9ae8.

## Evidence
- Commits: 5b49227c21c06f9ef656f2cf6d16f2a6540557a0, 4dbe6124d0dbebb2b9b1db30c6d7a379ce7c2e40, a5c1624345e92702c5f752b60e6117c32ea04e54, ca78691f29071c15a3335358fe52f467cefb4ee7, 138427a8dd2e18040db15345bbecd0ddce97ab59, 50f7e9d5034c614b76ab8029c4172d6be214ce5d, 1817b29d48fa787f8030f75aa544b57c24feaedb, e61c9ae845c0201fccdc7fd08945f575b1ff1151, 96c91c6dd11a55dd5170086329d92f77b95f7af2
- Tests: cargo test --profile ci --workspace, cargo clippy --profile ci --workspace --all-targets, npm test, npx tsc --noEmit, node tests/species.mjs --measure-only (fn9 and fn34 profiles)
- PRs:
