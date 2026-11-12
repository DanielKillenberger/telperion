# fn-356-a-unit-lays-the-nodes-its-vigour-and.1 A unit lays min(1, size x presence) of its nodes

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Five designs built and measured; none kept. The last passed every test and moved held phytomers on the oak at 120 by 2.1%. The rule is reverted whole (code as at 5f07de93f, which keeps the two shed-bound corrections). The fault and what the builds established are in the spec's "Result" section and TASK1-STOP.md; the old oak's cost goes to fn-351's Phase 5.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 6da2ff9a3, 5f07de93f, 7983897b7
- Tests: narrow space+core suites 770 of 771 on the last design before its revert; after the revert the code equals 5f07de93f
- PRs: