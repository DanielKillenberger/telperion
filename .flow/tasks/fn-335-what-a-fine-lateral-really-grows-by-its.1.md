---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-335-what-a-fine-lateral-really-grows-by-its.1 Implement What a fine lateral really grows, by its light

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The read ran under the frozen protocol, and the verdict is near. With span and light as inputs, a stand-in read from local state predicts the standing leaf, section and wood of every preset's fine laterals within 10% for the whole tree and 20% for the worst bearer-age group. Span alone is no-go, and so is one curve across species. The full result is `.flow/evidence/fn-335-what-a-fine-lateral-really-grows-by-its/RESULT.md`, with `cells.csv` beside it. No next step is proposed; that is the host's.

The recorder reads grown quantities before the shed and standing quantities after the settle's shed and scale. Its test holds the standing sums equal to the tree `grow_cycle` returns. The recorder is kept as `probes/recorder.patch`, and the crates end as they began. The read was reduced to cell sums through a pipe, so no record line reached the disk.

baseline: none (no Quick commands; narrow test only)

stage: impl-review - skipped(policy: risk - no crate change stays; only .flow/ evidence is committed, per AGENTS.md and R6)

Tier: implementer (project routing block, AGENTS.md "Model routing": implementer: opus) (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b2e606b23aa44648a6a72bd10dd306195d437457, 03fae16f41703f007a378ba5fd6d1683f70a7fd7, cf270d8e14513f8023b560ef7c55bdc3af3b9ccc
- Tests: telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-space --lib the_kept_laterals_are_the_full_cycles (patch applied): 1 passed, probes/read.py x4 presets, seeds 1001-1012 (lateral_records --ignored): exit 0, probes/verdict.py raw/read: VERDICT near
- PRs: