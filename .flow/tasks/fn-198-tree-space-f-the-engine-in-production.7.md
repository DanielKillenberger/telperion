---
satisfies: [R3]
---
# fn-198-tree-space-f-the-engine-in-production.7 A lean, reliable test suite

## Description
Owner, 2026-10-07: 'Fix the flaky test. Maybe actually have a subagent evaluate superfluous tests and remove them, integrate into working branch to make work more smooth and fast.' (1) fixed_beeches_pass_geometry_and_profile_gates_with_repeatable_varied_specimens reads its memory ceiling from the machine's free memory at start, so it fails when other worktrees load the machine; give it a ceiling that does not depend on free memory. (2) Audit the workspace's tests for superfluous ones (duplicates of another test's assertion, tests of removed paths, tests subsumed by stronger ones, slow tests whose coverage a faster one gives), each removal justified in TEST-AUDIT.md (file:line, what covers it instead); never remove executable fixtures as if generated (AGENTS.md); the 25 ignored old-grower tests stay until R0. (3) The gate's wall time and slowest binaries before and after.

## Acceptance
- The flaky test's ceiling machine-independent; it passes under load.
- TEST-AUDIT.md listing every removal with its covering test.
- Gate wall time before and after; the gate green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
