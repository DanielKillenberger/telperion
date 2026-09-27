---
satisfies: [R1, R2, R3]
---
# fn-178-the-size-check-stops-what-the-owner.1 Implement The size check stops what the owner would refuse

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The size check now holds two limits: the owner's ceilings (with reasons) in scripts/artifact-budgets.json, and a 5 percent per-PR growth share over the base, the newest green master run of tests.yml that is an ancestor of the PR head and still holds its `package` artifact; a PR whose `## Decisions` section names the artifact's file passes above the share, a missing base prints a `::warning::` and enforces ceilings alone, and any other lookup failure fails the job. Tests in scripts/artifact-budgets.test.mjs run offline on recorded sizes behind an injected reader (R1 ceiling with reason; R2 fn-150 +48.4 percent fails, fn-170 passes with no budget edit, a Decisions-declared growth passes, exactly 5 percent passes and one byte more fails, base found past the first page, absent base); docs/principles.md item 3 describes both limits (R3).

Live path: the GitHub reader was exercised locally read-only and found base ed23f773 (run 36320657176) with its six dist sizes; the package job's own output appears only once a PR runs it (tests.yml does not run on a branch push). Follow-up for the host: the reviewer's non-blocking FYI that the search stops at the first ancestor whose package is missing (chosen deliberately: artifacts expire by age, and continuing would page every older run through the API).

stage: impl-review - ran (codex fan-out NEEDS_WORK, 3 findings fixed in 6ae1398d, re-review SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 0e53eaa5cefa52655697e410441d2ca10d38fd7f, 6ae1398dd66b2e9c16a289eb80139cd08267265c, 712e319121605436d74548f47ebb65f62d3e096a
- Tests: npm test (13 files, 129 passed), cargo test --profile ci --workspace --no-fail-fast (1074 passed, 0 failed, 22 ignored, 108 suites), npx vitest run scripts/artifact-budgets.test.mjs (8 passed), baseline: green (npx vitest run scripts/artifact-budgets.test.mjs, 1 passed pre-edit)
- PRs: