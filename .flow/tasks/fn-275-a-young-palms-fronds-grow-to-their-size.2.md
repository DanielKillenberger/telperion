---
satisfies: [R2b]
---
# fn-275-a-young-palms-fronds-grow-to-their-size.2 Seedling leaves: entire fronds becoming pinnate

## Description
R2b, after fn-277 lands (fractional counts).

## Acceptance
R2b.

## Done summary
R2b to the host's two rulings: each frond's leaflet count is leafletCount ^ f (f the establishment course at the frond's emergence), from 1 (an entire strap) to 113, split by fn-277's keyed draw and kept for life; a young frond's leaflets are strips of one blade (`Split::segment`, `pack.wgsl` `leaflet`). No new rows; the palm at 40 byte-equal. Leaves at 1/2/3/4 years 10/20/45/151 (were 1,017/1,921/2,938/3,842); CPU and device agree exactly at 3 and 4. Gate 1,406 passed, 1 failed (generation-limits inventory, classified, guard re-run green). Evidence: SEEDLING.md.

## Evidence
- Commits: 0e342e4b2, 19ab9885b, 11510978a
- Tests: telperion-heavy 10 --gpu 4 --threads 4 -- cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-core --test generation_limit_guard
- PRs: