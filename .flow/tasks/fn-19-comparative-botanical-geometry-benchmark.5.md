---
satisfies: [R5]
---
# fn-19-comparative-botanical-geometry-benchmark.5 Provide reusable species onboarding and parallel-agent workflow

## Description
Provide the reusable species-onboarding template and parallel-agent workflow against the frozen admission contract.

## Acceptance
- The template separates species-level evidence and capability requirements from specimen-level seeds and receipts.
- The workflow names stage inputs/outputs, gates, file ownership and coordinated core/registry/measurement handoffs.

## Done summary
`docs/species-onboarding.md` and `templates/species-profile.md`, with separate oak and spruce packets and a later illustrative cohort under `.flow/evidence/fn19/onboarding-examples/`. Scots pine parses but admission reports `unsupported-anatomy`, `implemented=false`. `verify.py` runs 22 checks (identity, expansion, duplicates, missing references, reused holdouts, unsupported anatomy, ownership).

## Evidence
- Commits: 7550dd7fb66220f14cd65992babe1cd4331270da
- Tests: python3 -B .flow/evidence/fn19/onboarding-examples/verify.py; python3 -B -m unittest discover -s crates/telperion-core/examples/geometry_benchmark -p test_controls.py
