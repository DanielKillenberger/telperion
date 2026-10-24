# fn-311 friction

## 2026-10-24, task .1: dcg blocks every log redirect into the scratchpad

Doing: capturing `cargo check` output to a log under the session scratchpad, per the suite-output capture rule.
Hindered: dcg refused `> "$S/logs/x.log"` (dynamic path) and then the same redirect with a literal scratchpad path (home path, truncating). Only `| dcg create-new <path>` passed, which also changes how the exit code is read (`${PIPESTATUS[0]}`).
Cost: two blocked calls, about 2 minutes.
Would remove it: dcg treating the session scratchpad like `/tmp` for truncating redirects, or the worker brief naming `| dcg create-new` as the capture form for cargo logs.

## 2026-10-24, task .1: impl-review scopes to telperion's range while the code is in the fork

Doing: the codex impl-review of a task whose code lives in a scratch clone of the wgpu fork.
Hindered: flowctl's review reads `--base..HEAD` in telperion, which held only an evidence note. Its `triage-skip` step would have passed that docs-only diff as SHIP without reviewing the fork, so the worker skipped triage and steered the reviewer to the fork through `--focus`. The reviewer could read the clone but not run cargo or reach GitHub from its read-only sandbox.
Cost: about 3 minutes of working out the route; no failed dispatch.
Would remove it: a review flag that names another repository and range as the artifact under review.
