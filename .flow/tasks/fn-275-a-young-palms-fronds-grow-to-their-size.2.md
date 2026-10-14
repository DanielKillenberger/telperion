---
satisfies: [R2b]
---
# fn-275-a-young-palms-fronds-grow-to-their-size.2 Seedling leaves: entire fronds becoming pinnate

## Description
R2b, after fn-277 lands (fractional counts).

## Acceptance
R2b.

## Done summary
R2b, built to the host's two rulings. Each frond's leaflet count is leafletCount ^ f, where f is R1's establishment course at the crown's age when the frond emerged (f = ln n / ln N). The count goes from 1, an entire strap, to the mature 113, is split by fn-277's keyed draw, and is kept for the frond's life.

A young frond's leaflets are strips of one blade (`Split::segment` on the CPU, `leaflet` in pack.wgsl on the device):
- Each strip's base is mix(k/n, its station, f).
- Its length runs log from the remaining blade (base to tip) to the mature leaflet's.
- Its pitch runs from 0 to the mature pitch.
- Its spines are the mature share, 8·n/113.

No new rows were added. Mature fronds take the old code path, and the palm at 40 is byte-equal.

Measurements:
- Reach stays 0.81 to 1.21 blades, with no jump from one strip to two.
- Leaves at 1/2/3/4 years: 10/20/45/151, against 1,017/1,921/2,938/3,842 without the rows.

With no draws, the CPU and the device agree exactly at 3 and 4.

Gate: 1,406 passed and 1 failed. The failure was the generation-limits inventory, now classified, and its guard re-run passed. The palm at 1, seed 7, is re-pinned from 1,017 leaves to 10. The 3-year (FAO) against 6.75-year clock is accepted as a known simplification. Evidence is in SEEDLING.md.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 0e342e4b2, 19ab9885b, 11510978a
- Tests: telperion-heavy 10 --gpu 4 --threads 4 -- cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-core --test generation_limit_guard
- PRs: