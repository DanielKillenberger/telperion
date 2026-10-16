---
satisfies: [R2]
---
# fn-9-real-species-profiles-and-procedural.2 Measure specimens against the botanical profiles

## Description
Measure generated specimens against the profiles with a CPU-only runner.

## Acceptance
- Actual wood/foliage geometry, DBH, branch axes/orders, retained foliage and area measured; per-case failures preserved; nonzero exit on unmet gates.

## Done summary
Profile comparison runner with durable JSONL, frozen profile data and machine/git metadata; analytic metric tests pass.

## Evidence
- Commits: df0119d
- Tests: metric tests, native tests (14), typecheck
- PRs:
