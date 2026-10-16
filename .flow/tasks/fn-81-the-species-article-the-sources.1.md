---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-81-the-species-article-the-sources.1 Implement The species article: the sources distilled, every claim cited

## Description
Source copies and a cited article per species, a `document` pipeline stage, and structure-check enforcement.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
22 source copies (4 full under CC BY 4.0 or public domain, 16 extracts, 2 unavailable) and five articles, one cited sentence per line; code renders measurements, gaps and bibliography. The structure check enforces the rights rule, uncited claims, numbers absent from the packet, moved inputs and open unsupported-claim decisions. The `document` stage is built and tested; the Jev citation check was not run over the five backfilled articles (budget).

## Evidence
- Commits: 8bf062de53512a6bb9e808558c38f29e3e3c689e, 5d65a79b0318d2d03efffd2f8982b17f1d5643e3, 9b19d32f4795073746adeb34552410326d1d2d35
- Tests: npm test, cargo test --release --workspace, npm run typecheck, node scripts/catalogue-check.mjs
- PRs:
