---
satisfies: [R6, R7]
---
# fn-8-lean-rust-tree-generation-core.7 Measure the migration and remove the superseded production core

## Description
Measure the migration against fn-6 and remove the superseded production core.

## Acceptance
- Paired latency and memory measurements including binding costs; one production core; docs for rule and output changes.

## Done summary
Builders 2.91x (Telperion), 2.71x (Laurelin), 2.81x (comparison) faster with unchanged work; a squared-distance profile kernel removed the initial foliage regression. Giant Wasm high-water 526 MiB; total browser peak not measurable. Report: `.flow/evidence/fn8/REPORT.md`.

## Evidence
- Commits: fbb0728ca6715129edb7a7153871019ea917ed15, 48b66ce0f9edd5839ef49d8604b82092f7fdf7bc
- Tests: cargo test, npm test, paired headless Chromium measurement
- PRs:
