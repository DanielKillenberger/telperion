# fn-312 friction

- 2026-10-24, Unknowns on scratch clones: the dcg hook refused shell redirects to a variable scratch path (`> $S/out/...`) twice, and a heredoc into python that wrote under `$S`. Cost: about 3 minutes rewriting each probe as a script file. What would remove it: local setup (an allowlist for the session scratchpad); reported, not specced.
- 2026-10-24, flowctl probe: running `show`, `cat` and `spec chain` once per spec (about 750 flowctl calls, two checkouts) took over 10 minutes, about 0.4 s per call of Python start-up. What would remove it: a flowctl verb that dumps every spec's `show` in one call; the probe could also have sampled.
