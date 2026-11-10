# fn-345 friction

## 10 Nov 2026, R1 worker: the shell hook refuses heredoc scripts with variable redirects

- **Doing:** writing `probes/build.sh` and `probes/runs.sh` with a single `cat > $E/... <<EOF` call.
- **What slowed it:** the local hook (dcg, `core.filesystem:redirect-truncate-dynamic-path`) read the redirect lines inside the heredoc body as commands and refused the whole call. The dispatch had warned about direct redirects; it did not say the hook also scans heredoc bodies.
- **Cost:** about 2 minutes and one retry. The scripts were then written with the editor tool.
- **What would remove it:** one line in the dispatch template: "write probe scripts with the editor, never through a heredoc".
