# fn-337 friction

## 9 Nov 2026, worker: writing the probe runner

- **Doing:** writing `probes/trunk.sh` with `mkdir && cat > file <<'EOF'` in one shell call.
- **Slowed by:** the dcg hook refused the heredoc redirect onto a new path under the worktree (its rule allows a literal absent target, but the combined `mkdir -p` of the parent in the same command left the parent missing when it checked).
- **Cost:** about 1 minute and one retry through the Write tool.
- **Would remove it:** nothing in the repository; the dispatch already says to use the Write tool. Noted so the next worker does.
