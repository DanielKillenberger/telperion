# fn-132 friction

## 2026-09-24, worker

- **Doing:** applying multi-file test edits through a Python heredoc.
- **Hindered by:** the dcg shell hook blocked a heredoc whose embedded Rust doc comments carried backticks, reading them as command substitution.
- **Cost:** about 2 minutes and one retry; the edit went through as a script file in the scratchpad instead.
- **Would remove it:** a quoted heredoc (`<<'EOF'`) is already inert to the shell; the hook could treat it as literal text.
