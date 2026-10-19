---
satisfies: [R1, R3]
---
# fn-221-bark-that-matures-with-girth.1 Mature bark blended in by radius

## Description
The spec's R1 and R3: a mature-bark colour and fissure depth blended by a stated radius range, the way /material/shootRadius blends young wood; neutral at zero. R2 (the birch's values and its judged stills) is fn-213's round 5 on this capability; here, show the blend on one specimen with test rows and view the stills.

## Acceptance
- R1 built; neutral rows byte-identical in look (stills of the passed species unchanged).
- The blend shown on a specimen, stills viewed.
- npm test and the gate green.

## Done summary
Six /material rows (matureRed/Green/Blue, matureFurrowStrength, matureRadius, matureSpan) blend old bark into thick wood by radius in the wood shader. Inert at matureRadius 0 (test; 12 neutral stills byte-identical to the base). Blend shown on the oak with test rows, stills viewed.

## Evidence
- Commits: 44665344, 75a7656a
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, npm test
- PRs: