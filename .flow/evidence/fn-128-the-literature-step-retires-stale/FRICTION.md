# fn-128 friction

## 2026-09-23 - the live evidence the spec cites is not on the branch

- **Doing:** writing R1's red test from the palm's second pass on fn-127, reading
  `.flow/evidence/date-palm/pipeline/` with `git show fn-80-the-gap-loops-first-live-run:<path>`
  as the dispatch said.
- **Hindered by:** the committed files are the first pass (ba8a92e6). There, `quality.json`
  scores crown width `none` and failing, which contradicts the spec's "quality passed crown
  width". The second pass that the spec describes is uncommitted in the fn-80 worktree
  (`quality.json`, `decisions.json`, `select.json` modified on disk, 19:59). I read it there
  without checking anything out.
- **Cost:** about 5 minutes and one detour to rule out a contradiction.
- **Would have removed it:** commit the run's evidence before capturing a spec that cites it,
  or name the working-tree path in the dispatch.

## 2026-09-23 - the command guard refuses a heredoc holding Rust generics

- **Doing:** applying a scripted edit through `python3 - <<'EOF'`.
- **Hindered by:** dcg read `Vec<(usize, usize)>` inside the heredoc as process
  substitution and blocked the command.
- **Cost:** about 2 minutes. I moved the script into a scratch file.
- **Would have removed it:** a dcg rule that skips quoted heredoc bodies fed to an
  interpreter, or writing edit scripts to files from the start.
