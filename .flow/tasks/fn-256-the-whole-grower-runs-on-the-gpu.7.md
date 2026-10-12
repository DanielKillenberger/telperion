# fn-256-the-whole-grower-runs-on-the-gpu.7 Roofline: where the GPU cycle's time goes per phytomer

## Description
Measurement only, no optimisation. For Telperion 82 (seed 42), the oak 40 and the beech 40 on the device cycle: per phase, GPU time, records touched (axes, phytomers, units, lattice nodes), bytes read and written, and the ratio of time to the RTX 3080's memory bandwidth and ALU limits (roofline: achieved GB/s and ops/s against peak); per cycle, how many records each phase visits that did not change since the last cycle; the share of grown phytomers on laterals that live under 3, 5 and 10 years and the share of total GPU time they cost. Report as ROOFLINE.md for the council.

## Acceptance
ROOFLINE.md with the per-phase table, roofline ratios, unchanged-record shares and the short-lived-lateral shares for the three presets, each figure measured.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
