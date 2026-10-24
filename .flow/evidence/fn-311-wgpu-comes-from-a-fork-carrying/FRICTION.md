# fn-311 friction

## 2026-10-24, task .1: dcg blocks every log redirect into the scratchpad

Doing: capturing `cargo check` output to a log under the session scratchpad, per the suite-output capture rule.
Hindered: dcg refused `> "$S/logs/x.log"` (dynamic path) and then the same redirect with a literal scratchpad path (home path, truncating). Only `| dcg create-new <path>` passed, which also changes how the exit code is read (`${PIPESTATUS[0]}`).
Cost: two blocked calls, about 2 minutes.
Would remove it: dcg treating the session scratchpad like `/tmp` for truncating redirects, or the worker brief naming `| dcg create-new` as the capture form for cargo logs.
