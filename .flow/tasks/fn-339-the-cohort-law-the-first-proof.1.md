---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-339-the-cohort-law-the-first-proof.1 Implement The cohort law: the first proof

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The cohort law's first proof ran through all stages: R1 (law on paper), R2 (published ranges), R3 (replay under the frozen protocol, twelve seeds per species), R4 (coupled probe, oak/spruce/beech at 40 on seeds 1001-1012, oak at 80 and 120 on 1001/1007/1012) and R5's verdict. By the goal's rule as written the approach stops: the coupled probe is no closer to R2's checked ranges (spruce shoots 28.4x over against the cycle's 27.3x, spruce LAI 1.23x against 1.15x, beech LAI 1% nearer at a fifth of a stand's) and it is 1.8 to 3.2 times cheaper on one thread, not five (oak 3.24 at 40, 2.52 at 80, 1.79 at 120; spruce 3.20; beech 1.95). The probe matches the cycle at 40 within 6 to 11% and drifts on the old oak (trunk 1.36x at 80, 1.91x at 120, leaf 1.21x and 1.89x).

Evidence: .flow/evidence/fn-339-the-cohort-law-the-first-proof/ RESULT.md (verdict), R1-LAW.md, R1-ANSWERS.md (host), R2-REAL-TREES.md, PROTOCOL.md (host), R3-RESULT.md, R3-TABLES.md, R4-TABLES.md, sheet-r4-{oak,spruce,beech}.jpg, probes/coupled.patch and probes/replay.patch, FRICTION.md. The crates end as they began.

baseline: none (probes only; crates restored)
stage: impl-review - skipped(policy: risk - no crate change stays; commits hold only .flow/ evidence, the probes kept as patches)
Tier: implementer (project routing block, AGENTS.md "Model routing": implementer: opus) (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: a8cb90fcf6b7bfe797f3c63465aa02781212e9f8, b86bb7cb47ac56fe035e223d260a2092b9cebc51, 7146357006c7b546ba04ba8ce693f567bc1c1e75, 2f1c5ed2036065d545a464988090d775442e81b4, d2baa9508217acb864e843bbdf7f1b315e9eac0b, 2b4b4e4ae735592777be6e99bf70b07493487c80
- Tests: probe: telperion-heavy 8 --threads 4 -- cargo test --profile ci -p telperion-space --lib replay (patch applied; 1 passed), probe: cohort_replay (R3) and cohort_coupled (R4) ignored measurements through telperion-heavy, patch applied, rc 0, GATE_SKIPPED:gate:no crate change stays - probes kept as patches, crates restored
- PRs: