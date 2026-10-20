# fn-299 friction

## 2026-10-20, plan-review: the round cap ended the review on a one-number fix

What: `flow --auto` plan-review with codex. Round 1 found task .3 did not require device builds of fn-298's `unit_rounds[0]` frames 76 and 77; fixed. Round 2 confirmed that fix and found the new text named `ease(77/120)` and `ease(78/120)` where the frames are `ease(76/120)` and `ease(77/120)`; fixed exactly as the reviewer suggested. The third dispatch was refused: `review.maxIterations` is 2, so `ESCALATE` and `NEEDS_HUMAN`.
Cost: about 12 minutes for two review rounds, and the run stops before work for a human counter reset.
What would remove it: a cap of 3 for plan review, or a cap that counts rounds whose findings are new rather than a fix's own slip. The reset (`flowctl spec reset-review-rounds`) is human-only under autonomy.

## 2026-10-20, pilot snapshot: dcg blocks the skill's snapshot fence

What: auto.md's fence writes `flowctl pilot snapshot` to `$(git rev-parse --show-toplevel)/.flow/tmp/pilot-snapshot.json`; dcg refuses a truncating redirect to a dynamic path, and also to the literal path (existing home path). Worked around by writing to the scratchpad.
Cost: two blocked calls, about 2 minutes.
What would remove it: a `flowctl pilot snapshot --out <path>` flag, or a dcg allowance for `.flow/tmp/` in a worktree. A local setup matter (dcg), reported, not specced.

## 2026-10-20, task .2: heavy-queue waits dwarf the CPU-only tests

What: task .2's focused tests (`-p telperion-space`, `-p telperion-core --test units_bound`, CPU only) waited 38 minutes for a `telperion-heavy 8` admission (19:15 to 19:53), while another worktree's suite held memory. The suite itself takes about 2 minutes. The rerun after a test fix queued again from about 19:57.
Cost: about 40 minutes of wall time per focused run, against 2 minutes of work.
What would remove it: a lighter admission class for CPU-only crate tests (for example `telperion-heavy 4`), or a reservation that lets a waiting agent's small job run beside a gate. This is within the host's expected queue range (up to 90 minutes), so the run continues.
