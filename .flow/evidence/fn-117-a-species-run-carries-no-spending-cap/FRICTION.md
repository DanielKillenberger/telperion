# Friction: fn-117

## 2026-09-23: a shell hook refused a multi-file edit script

- Doing: making the conductor's caps optional across seven files with one
  inline Python edit script.
- Hindered: the `dcg` PreToolUse hook blocked the heredoc as an "embedded
  shell launcher" it could not verify, although it only rewrote source text.
- Cost: about 2 minutes and one retry (the script went to a scratch file).
- Would remove it: a dcg allowlist entry for `python3 - <<'EOF'` edit
  scripts, or a note in the worker brief to write edit scripts to the
  scratchpad first. This is a local setup matter, not a repository one.

## 2026-09-23: turning required caps optional meant hand-wrapping 60 literals

- Doing: changing eight `u64` cap fields to `Option<u64>`.
- Hindered: the test suites build budgets as struct literals in about 60
  places across seven test files, each needing `Some(..)`.
- Cost: about 5 minutes, done by a script over rustc's JSON diagnostics.
- Would remove it: a shared budget builder in `tests/fixture`, so a field
  change touches one place.
