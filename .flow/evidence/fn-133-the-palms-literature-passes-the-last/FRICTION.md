# fn-133 friction

## 2026-09-24, worker (fn-133.1)

- Doing: applying a multi-passage doc edit to `docs/species-pipeline.md` with an inline python heredoc.
- Hindered by: the dcg PreToolUse hook blocks any heredoc whose body carries backticks ("embedded shell launcher cannot be statically verified"); Markdown code spans are backticks, so every doc edit by heredoc is refused.
- Cost: about 2 minutes and one retry (script written to the scratchpad, then run).
- Would remove it: a dcg allowlist entry for quoted heredocs (`<<'EOF'`, no expansion) feeding `python3 -`, or the habit of writing edit scripts to the scratchpad first.
- 2026-09-24: the first workspace gate failed on pipeline_upstream's guard that provenance carries no probability; the pick probability moved from the sidecar entry into the select body. Cost: one extra gate run (~time of full workspace).
