---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-133-the-palms-literature-passes-the-last.1 Implement fn-133-the-palms-literature-passes-the-last

## Description
Close the palm literature's last four stops.

## Acceptance
Spec R1-R5.

## Done summary
No `no_mature_size` on a field with points; palm height at `proxy_only`; selection floor only when calibrated (>=20 cases, >=5 wrong), otherwise most-probable span with `pick_probability`; unstated variation traits default to zero width with a reason. Tests `tests/palm_literature.rs` (red first). Gate green (1027 passed).

## Evidence
- Commits: 00c9e689c1efe70140f1f6d0bdf0e1d8e213e875, 9f1ea345d8bcdb778392f33688a90fecf258e2d0
- Tests: cargo test --profile ci -p telperion-jev --test palm_literature; cargo test --profile ci --workspace --no-fail-fast (1027 passed)
- PRs:
