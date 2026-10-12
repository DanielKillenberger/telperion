# fn-256-the-whole-grower-runs-on-the-gpu.7 Roofline: where the GPU cycle's time goes per phytomer

## Description
Measurement only, no optimisation. For Telperion 82 (seed 42), the oak 40 and the beech 40 on the device cycle: per phase, GPU time, records touched (axes, phytomers, units, lattice nodes), bytes read and written, and the ratio of time to the RTX 3080's memory bandwidth and ALU limits (roofline: achieved GB/s and ops/s against peak); per cycle, how many records each phase visits that did not change since the last cycle; the share of grown phytomers on laterals that live under 3, 5 and 10 years and the share of total GPU time they cost. Report as ROOFLINE.md for the council.

## Acceptance
ROOFLINE.md with the per-phase table, roofline ratios, unchanged-record shares and the short-lived-lateral shares for the three presets, each figure measured.

## Done summary
The roofline for Telperion 82, the oak 40 and the beech 40 (seed 42) on the device cycle is in ROOFLINE.md on branch fn-256-the-whole-grower-runs-on-the-gpu (commit 457f1193d). It has per-phase GPU time, bytes and ops against the card's measured 705 GB/s and 16.5 Tops int32, unchanged-record shares, and short-lived-lateral shares. The measurement is test-only (`the_roofline_is_measured`) and changes no kernel.

- **Dominant phases.** Telperion: units 24%, light 24% (of which 81% is the unlit lattice's empty invocations), balance and allotment 22%, retirement 17%. The oak and the beech: balance and allotment 33% / 39%, units 26% / 24%.
- **Distance from the roofline.** Every phase but compaction runs 6x to 270x above its roofline time: at most 17% of peak bandwidth (an upper estimate) and at most 4% of peak ALU. The time is latency: small dispatches, single-workgroup folds (25% of the beech), and the unlit lattice (19% of Telperion).
- **Short-lived laterals.** On Telperion, 76% of grown phytomers sit on laterals living under 5 years, and they cost 61% to 77% of GPU time by records visited. On the oak the figures are 73% and 66% to 72%; on the beech, 62% and 67% to 69%.

stage: impl-review - skipped(policy: risk - measurement only: test-only instrumentation and an evidence file, no production path changed)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 457f1193d
- Tests: cargo test --profile ci -p telperion-render the_roofline_is_measured -- --ignored --nocapture (ROOFLINE_OUT=raw/t7, RAYON_NUM_THREADS=4, under the GPU lock): 1 passed, python3 roofline.py raw/t7
- PRs: