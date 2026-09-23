---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-133-the-palms-literature-passes-the-last.1 Implement fn-133-the-palms-literature-passes-the-last

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The palm's last four literature stops are closed. R1: a field with any point never carries no_mature_size/no_growth_rate; below partial it takes Jev's most probable stated-shortfall gap (palm dbh -> bound_only, passes). R2: the palm row asks height_m at proxy_only; broadleaf/conifer unchanged. R3: the selection floor applies only when its labelled set holds >=20 cases and >=5 wrong picks (floors::calibrated); until then select fills the most probable span and records its probability in the select body (`pick_probability`), not in provenance, which an existing guard keeps probability-free. R4: an unstated variation trait (a level whose every range holds zero) takes that level (`uniform`) as a default with its reason (select body, profile `default` with no sources, provenance `defaults`); an unstated colour or roughness still files requirements-unmet. Tests: tests/palm_literature.rs (R1-R4). Changed by declared intent: judged_verify no_mature_size test (now the no-point field), judged_select floor and brightness tests, requirements.rs unmet count and palm height pin. Docs: docs/species-pipeline.md.

Tier: session (owner-approved design)
stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 00c9e689c1efe70140f1f6d0bdf0e1d8e213e875, 9f1ea345d8bcdb778392f33688a90fecf258e2d0
- Tests: cargo test --profile ci --workspace --no-fail-fast (gate_rc=0; 140 test result lines: 1027 passed, 0 failed, 21 ignored; first run red on pipeline_upstream provenance-probability guard, fixed in 9f1ea345), cargo test --profile ci -p telperion-jev --test palm_literature (red first: 4 failed for the intended reasons, then green)
- PRs: