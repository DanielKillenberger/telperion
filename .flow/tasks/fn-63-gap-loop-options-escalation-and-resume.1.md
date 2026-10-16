---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-63-gap-loop-options-escalation-and-resume.1 Implement Gap loop: options, escalation and resume

## Description
Build the gap loop: typed options, the gap question set, the route table, resume via expired keys, metrics, and the add-species skill.

## Acceptance
Every R-ID in the parent spec's acceptance is satisfied; judged against the spec's criteria directly.

## Done summary
R1 to R6 met and evidenced (`pipeline/gap/`, `data/questions/gap.json`, `data/cases/gap.json`, `data/gap-routes.json`, `.claude/skills/add-species/SKILL.md`). `jev cases --only gap` exits 0: change kind 24/24 and 13/13 held out, prior verdict 23/24 and 12/13, best match 9/10 and 4/5. R7 (one real species end to end) is open and not this task's to close; the machinery runs end to end against a fixture transport in `tests/gap.rs`.

## Evidence
- Commits: 5e382032c773414d6e73298f346a9f881d30c15b, 47dadeec2d38ef923ff299c9b25498761bdd6079, bb5dc4d3a8e82fcd9de50f154d3100c3a8097710, 72af189825cc8e4c58975cec45c363a54d3c9928, ebd1c8f7216ed04f1b6826de1f94d60422327565, 343e14a7f31a641af2bf056b4c676a645e73b47c
- Tests: cargo test --profile ci -p telperion-jev (168), cargo test --profile ci --workspace, cargo clippy --profile ci -p telperion-jev --all-targets, target/ci/jev cases --only gap
- PRs:
