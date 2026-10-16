---
satisfies: [R1, R2]
---
# fn-142-select-keeps-the-reference-photographs.1 Implement fn-142-select-keeps-the-reference-photographs

## Description
Preserve recorded reference photographs across a select rerun.

## Acceptance
Spec R1-R2.

## Done summary
`select.rs` `write_packet` keeps an existing `references` array byte for byte and rewrites only `sources`. Test `a_select_rerun_keeps_the_recorded_references_and_only_rewrites_sources` red on base, green after. Gate green (1053 passed).

## Evidence
- Commits: a63d9e9fa6f4a1ab9b6d1e916ea23fc0eda6fd90
- Tests: cargo test -p telperion-jev --test judged_select; cargo test --profile ci --workspace --no-fail-fast (1053 passed)
- PRs:
