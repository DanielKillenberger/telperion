---
satisfies: [R1, R2]
---
# fn-142-select-keeps-the-reference-photographs.1 Implement fn-142-select-keeps-the-reference-photographs

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Select's `write_packet` (`crates/telperion-jev/src/pipeline/stages/select.rs`) now reads any existing `packet/references.json` and preserves its `references` array byte for byte across a rerun; only `sources` is rewritten from the manifest, matching what select actually owns. Fixes fn-80's bug where a stage rerun overwrote the palm's three recorded reference photographs with an empty list.

A red test first confirmed the bug (`a_select_rerun_keeps_the_recorded_references_and_only_rewrites_sources` in `crates/telperion-jev/tests/judged_select.rs`): pre-loading `packet/references.json` with three recorded shots and stale sources, then rerunning `quality::run` + `select::run`, wrote `"references": []` over them on the base commit. After the fix the same test is green: the recorded shots survive unchanged while `sources` reflects the manifest's current five source ids.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: a63d9e9fa6f4a1ab9b6d1e916ea23fc0eda6fd90
- Tests: cargo test -p telperion-jev --test judged_select, cargo test --profile ci --workspace --no-fail-fast (1053 passed, 0 failed, 21 ignored; GREEN_RECEIPT: .flow/tmp/green-receipts/a63d9e9f-unittest.json)
- PRs: