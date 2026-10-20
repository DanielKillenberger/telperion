# Friction: fn-297

## 2026-10-20 plan stage
- Doing: `flowctl init --json`, as the plan skill's preflight requires.
- Hindered: init adds `review.fanoutExecution` to `.flow/config.json` on every run (fn-294 reverted the same key); the dirty tree would trip the pilot's post-hop guard.
- Cost: ~1 minute and a revert per stage that runs init.
- Remove it: commit the key once, or make init not write keys the repo removed.
- Also: `pilot snapshot` marks fn-297 ineligible because its parent branch fn-294 is not on origin, though the host merged fn-294 into this branch (e507d4874). The host dispatch proceeds anyway; a chain check that accepts a parent merged into the child branch would remove the override.
