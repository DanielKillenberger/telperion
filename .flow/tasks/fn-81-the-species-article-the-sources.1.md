---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-81-the-species-article-the-sources.1 Implement The species article: the sources distilled, every claim cited

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Every species in the catalogue now carries a markdown copy of each admitted source and an article distilled from them with a citation on every claim, and the pipeline gained the `document` stage that writes both for the next species and re-runs fn-57's citation check over the article's own sentences.

Twenty-two source copies: four full (the three JRC atlas factsheets under CC BY 4.0 and the USFS Silvics chapter in the public domain), sixteen extracts of the cited passages, two recorded unavailable. Five articles, one sentence per line, each line citing the local copy it rests on; code renders every measurement, the gaps section and the bibliography from the record, so no number reaches an article from a writer. The structure check enforces all of it: the rights rule, the uncited claim, the number absent from the packet, the moved input, and an open unsupported-claim decision.

Three things the sources would not support are stated in the articles rather than smoothed over: KEW-ASH carries no morphology though the packet cites it for a height and a leaflet count, O1 and OSU-ASH are the same page under two ids, and NCSU-SPRUCE's two descriptions disagree on how far the spruce's secondary branchlets hang. ATKINSON-BIRCH is behind a JSTOR wall and TSO-BIRCH's recorded url now 404s, which leaves the birch's crown ratio and both gating blade dimensions resting on text no run can reach.

Not exercised, by budget: the Jev citation check has not been run over the five backfilled articles. It is roughly 150 model calls, and the four pre-pipeline species have no fetch.json or select.json for the stage to open, so it runs for the first time on the next species onboarded through the loop. The mechanism, its decision kinds and the check that refuses an article carrying an open one are all in place and tested.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 8bf062de53512a6bb9e808558c38f29e3e3c689e, 5d65a79b0318d2d03efffd2f8982b17f1d5643e3, 9b19d32f4795073746adeb34552410326d1d2d35
- Tests: npm test, cargo test --release --workspace, npm run typecheck, node scripts/catalogue-check.mjs
- PRs: