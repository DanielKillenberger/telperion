---
satisfies: [R1]
---
# fn-198-tree-space-f-the-engine-in-production.9 Release readiness

## Description
Owner goal (2026-10-08): a PR from this branch to master that is ready to make the release, driven by /flow-next:flow --auto. Bring the branch to that state: (1) every finished spec branch merged (fn-226, fn-230 at least; fn-223, fn-231, fn-214, fn-227 and the species specs as they finish), the full gate and npm test green; (2) measured release checks: every catalogue species's build time in the harness against master (settings change to drawn), killenberger.com's field call against 0.1.4, artifact sizes against their ceilings; (3) the catalogue: species with no engine species are refused by name today; the owner's scope decision (release now with the species the engine draws, or wait for the birch verdict, Laurelin and Telperion) is an open item on a draft PR, not a stop; (4) docs/pipeline.md and docs/tree-space.md current; the npm version and changelog per the repo's release convention; (5) the PR body per docs/pr-format.md with every open item listed.

## Acceptance
- The branch merges cleanly into master; gate and npm test green.
- The release checks measured and recorded.
- Open items (scope, birch verdict, size ceilings, remaining species) listed on the draft PR.
- The PR opened by make-pr; nothing merged without the owner.

## Done summary
Release PR #159 opened as a draft (F4 to master, not merged). Gate 1,413 passed, 0 failed; npm 179/179. Release checks in fn-259 RELEASE.md: the harness meets R0a (Telperion 1.33 s at seed 42 against about 1.25 s), the site does not (fn-257 unbuilt), all four artifacts over their ceilings; open items listed on the PR for the owner.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 4272cb810, 05425e58a, 14b726932
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: https://github.com/DanielKillenberger/telperion/pull/159