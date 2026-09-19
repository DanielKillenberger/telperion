# fn-74 friction

## 2026-09-18, the R4 comparison runs the whole workspace twice on the desk

What: proving that the `ci` profile moves no result (R4) means one full
`cargo test --release --workspace` and one `cargo test --profile ci --workspace`
on the same commit, then a diff of the per-binary tables.

Cost: 545 s and 479 s of wall time on a 32-core desk, about 17 minutes of a
120-minute timebox spent waiting, plus two warm-compile proxies after them.
The tokens were small: the runs were launched once in the background and read
back as two tables.

What would remove it: the comparison belongs on the runner, where the number
is the one that counts. A one-off workflow dispatch that runs both profiles
and uploads the two tables would give the R4 and R6 numbers in one push, and
the desk run would only need the focused suites. This shell cannot push, so
the runner numbers are the conductor's after the return either way.
