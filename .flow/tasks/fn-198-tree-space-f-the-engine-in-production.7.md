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
The flaky fixed-species test's process ceiling is a stated 6 GiB, no longer a share of MemAvailable; the birch peaks at 1.98 GB against it. The audit (TEST-AUDIT.md) removed five duplicate test runs, each with its covering test cited, turned six tests of the scaffold on an engine preset's rows into skips or product builds, shared the walks suite's duplicate walk, and stopped the jev retry tests sleeping. The gate's limit guard, red on fn-198's tip, has the shoulder fit's nine sites classified. Gate 980 s red -> 508 s green (desk loaded both times); npm test red (slim field timeout) -> green. Candidates the host decides are listed in TEST-AUDIT.md.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 4512c233, 2a9acd71, 9dbb37a2, 2d7b96ef, 661a3a16, 693d40a3, 8bd00cfd, 99d724ad, a70b580c, 9c3bf57d, e67b51d6, 07fa0fda, cc8954ff, 4705a2a6, f5f212b0, 94aeeb85
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: