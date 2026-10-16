---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-127-the-literature-step-reads-mature-sizes.1 Implement fn-127-the-literature-step-reads-mature-sizes

## Description
Fix the three literature-step defects from the palm's live run.

## Acceptance
Spec R1-R5.

## Done summary
Mature fields need no age and are judged by a new mature-size question set (14 labelled cases); appearance values record source and span; the conductor pauses on open owner-only decisions first. Tests `mature_sizes.rs`, `requirements.rs`, `owner_first.rs`, each red on base. Gate green (958 passed).

## Evidence
- Commits: 28341393aa882013983965f77bd1639f52099d16
- Tests: cargo test --profile ci -p telperion-jev --test mature_sizes --test requirements --test owner_first --test sets; cargo test --profile ci --workspace --no-fail-fast (958 passed)
- PRs:
