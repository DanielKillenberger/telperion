# fn-294 friction

## 2026-10-19 — plan review: `main...HEAD` snapshot, then a two-round cap

- Doing: the first `flowctl codex plan-review` (flow-next 9.0.0).
- Hindered: it failed with `cannot resolve review snapshot: main...HEAD`; the repo has no `main` and the command needs `--base fn-198-tree-space-f-the-engine-in-production` passed by hand. Then the cap refused the third round at 2/2 verdicts (`MAX_REVIEW_ITERATIONS=2`) while the findings were shrinking (3, then 1, a wording slip in one acceptance line).
- Cost: one failed dispatch (seconds), one cap refusal and a counter reset (`flowctl spec reset-review-rounds`, which takes no reason flag, so the reason is recorded here: findings shrinking 3 → 1, owner allows a reset in that case).
- Would remove it: plan-review defaulting its base to the spec's base branch (or `git symbolic-ref refs/remotes/origin/HEAD`) instead of `main`; a `--reason` on `reset-review-rounds` so the record sits with the counter.
