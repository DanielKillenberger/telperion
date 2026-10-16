---
satisfies: [R1, R2, R3]
---
# fn-178-the-size-check-stops-what-the-owner.1 Implement The size check stops what the owner would refuse

## Description
Replace the release-plus-5% budgets with owner ceilings and a per-PR growth share.

## Acceptance
- R1-R3 of the spec.

## Done summary
Ceilings with reasons in scripts/artifact-budgets.json; 5% per-PR growth over the newest green ancestor master run's `package` artifact, passable when `## Decisions` names the artifact; a missing base warns and enforces ceilings only. Offline tests cover ceiling, fn-150 failing, fn-170 passing, declared growth, the 5% edge, paging and an absent base; docs/principles.md item 3 updated. Codex SHIP after 3 findings fixed.

## Evidence
- Commits: 0e53eaa5cefa52655697e410441d2ca10d38fd7f, 6ae1398dd66b2e9c16a289eb80139cd08267265c, 712e319121605436d74548f47ebb65f62d3e096a
- Tests: npm test (129 passed), cargo test --profile ci --workspace --no-fail-fast (1074 passed), npx vitest run scripts/artifact-budgets.test.mjs
- PRs:
