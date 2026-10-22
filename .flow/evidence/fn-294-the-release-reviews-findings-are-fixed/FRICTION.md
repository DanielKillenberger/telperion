# fn-294 friction

## 2026-10-19 — plan review: `main...HEAD` snapshot, then a two-round cap

- Doing: the first `flowctl codex plan-review` (flow-next 9.0.0).
- Hindered: it failed with `cannot resolve review snapshot: main...HEAD`; the repo has no `main` and the command needs `--base fn-198-tree-space-f-the-engine-in-production` passed by hand. Then the cap refused the third round at 2/2 verdicts (`MAX_REVIEW_ITERATIONS=2`) while the findings were shrinking (3, then 1, a wording slip in one acceptance line).
- Cost: one failed dispatch (seconds), one cap refusal and a counter reset (`flowctl spec reset-review-rounds`, which takes no reason flag, so the reason is recorded here: findings shrinking 3 → 1, owner allows a reset in that case).
- Would remove it: plan-review defaulting its base to the spec's base branch (or `git symbolic-ref refs/remotes/origin/HEAD`) instead of `main`; a `--reason` on `reset-review-rounds` so the record sits with the counter.

## 2026-10-19 — task 1 gate: queued about eleven and a half hours

- Doing: the task 1 gate, `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, started 11:48.
- Hindered: `telperion-heavy` waited for 12 GB of room while another session's film render held memory; the host later stopped it and re-queued the gate at the owner's 8 GB cap (`gates8.sh`), which started at 23:38. The run's own work had been done since 11:48.
- Cost: about 11 h 50 min of wall time with no gate, against a few minutes of compile and test.
- Would remove it: a gate request sized to what the gate needs (8 GB was enough), or a queue that reports who holds the room and for how long, so a waiting run can say so early instead of waiting silently.
- Also: a fresh worktree needed `npm ci` (the esbuild postinstall is skipped by the install-scripts guard, but vite still ran); a cold `target/` per worktree, as AGENTS requires, means a full compile per worktree.

## 2026-10-20 — `flowctl init` rewrote the project config

- Doing: the plan stage's preflight (`flowctl init --json`, flow-next 9.0.0).
- Hindered: init added `"fanoutExecution": "concurrent"` to `.flow/config.json`, and the plan commit (`git add .flow`) carried it. The completion review flagged it as an untraced change; the closing commit reverts it.
- Cost: one reverted line and a few minutes. Had it merged, it would have changed review execution for every later spec.
- Would remove it: `flowctl init` leaving an existing config untouched, or reporting the keys it adds.
