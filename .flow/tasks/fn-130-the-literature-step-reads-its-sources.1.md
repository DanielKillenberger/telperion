---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-130-the-literature-step-reads-its-sources.1 Implement fn-130-the-literature-step-reads-its-sources

## Description
Make the text the literature stages read the text the source holds.

## Acceptance
Spec R1-R5.

## Done summary
Markdown read as markdown, HTML converted once; one shared sentence splitter; fetch refuses interstitials and slivers, falls back to the raw body, files `unavailable-source` per source and continues; failed retries reopen; `screen` reads `extract.json`. Tests in `tests/literature_text.rs`, each red on base. Gate: one unrelated temp-path collision, fixed in the same commit.

## Evidence
- Commits: 4ea7a13c
- Tests: cargo test --profile ci --workspace --no-fail-fast (986 passed; objectives temp-path collision fixed)
- PRs:
