# fn-293 friction

## 2026-10-18 - `flow --auto` cannot select the spec

- **Doing:** starting `flow-next:flow fn-293 --auto` in the new worktree `.worktrees/f4-293`.
- **Slowed by:** the pilot snapshot reports the spec `eligible: false`, route `host` ("spec not ready"). The ready flag is a person's consent gate (`flowctl spec ready`), and a dispatched agent may not set it on another agent's word. The run drove the same stages directly (plan, plan-review, work) instead of through the auto driver's selector.
- **Also:** `flowctl init --json` rewrote `.flow/config.json` (added `review.fanoutExecution`), and every preflight prints a note about two removed keys (`artifacts.html.enabled`, `pipeline.chainStages`). The edit was reverted by hand.
- **Cost:** about 5 minutes and three extra flowctl calls.
- **Would have removed it:** the owner marking a spec ready when handing it to a host for an unattended run, or the dispatch carrying the owner's ready instruction; removing the two dead keys from `.flow/config.json`.
