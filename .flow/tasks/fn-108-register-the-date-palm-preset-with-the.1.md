---
satisfies: [R1, R2, R3, R4]
---
# fn-108-register-the-date-palm-preset-with-the.1 Implement Register the date palm preset with the values the generator already reaches

## Description
Register the date palm preset with its four reachable value-table rows.

## Acceptance
Spec R1-R4.

## Done summary
`date-palm` registered in `params::IN_WORK` with `stems`=1, `lateral_orders`=0, `apical_dominance`=1.0, `length_taper`=0.0; no anatomy. `--print-family` and `--support` both exit 0 (`{"capabilities":["woody-axes"],"implemented":true,"profile_id":"date-palm"}`); gate green. R3's gate rerun is the host's after landing.

## Evidence
- Commits: 0364499996a52bbbef4b7c703830df52d3a12036
- Tests: species_measure --print-family date-palm; geometry_benchmark --support date-palm; cargo test --profile ci --workspace --no-fail-fast (118 suites ok)
- PRs:
