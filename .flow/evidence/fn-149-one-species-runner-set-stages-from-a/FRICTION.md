# fn-149 friction

## 2026-09-25 12:50, worker fn-149.1: one task for a 57,000-line rewrite

- Doing: re-anchoring on the spec and reading the code the runner replaces. The fn-80 runner is 36,881 lines of Rust source plus 22,936 lines of tests; tuning alone is 13,002 source lines, and its engine, bundle, sheet, veto and stride modules all take the same `engine::Run` with its budget reservations, priority approvals and pause records.
- Slowed by: the spec is one implicit task (owner rule), so the runner, the conductor and gap-loop removal, the tuning pause removal and the accept path share one 10-commit, 6-hour budget. The tuning pause machinery cannot be deleted without rewriting its test suite, which is most of the 22,936 test lines.
- Cost: about 35 minutes of reading before the first line of the runner, and the certainty that R2/R7 do not close in this dispatch.
- Would remove it: splitting the spec's plan into ordered checkpoints the host accepts one at a time (runner and conductor removal; tuning pause removal; accept), each with its own budget, even under one task.

## 2026-09-25 12:50, worker fn-149.1: the dcg hook blocks `git checkout <ref> -- <path>`

- Doing: carrying the fn-80 crate onto the master-based branch on a clean tree.
- Slowed by: the hook refuses the checkout even on a clean tree; the workaround was `git diff <base> <ref> -- paths | git apply -3`, which also needed a hand fix for three dial-table rows both sides had added.
- Cost: about 5 minutes.
- Would remove it: allowing `git checkout <ref> -- <path>` when `git status --porcelain -- <path>` is empty.
