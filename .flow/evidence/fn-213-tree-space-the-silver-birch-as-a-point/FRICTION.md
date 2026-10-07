# fn-213 friction

## 2026-10-07: the Jev commands in docs/typesafe.md do not run

- **Doing:** screening the birch's height sources with Jev (task 1, R1).
- **Slowed by:** `cargo run -p telperion-jev -- screen ...`, as `docs/typesafe.md` "Commands" gives it, fails: the crate has three binaries (`jev`, `species`, `tape_trim`) and no `default-run`. The working form is `cargo run -p telperion-jev --bin jev -- screen ...`.
- **Cost:** one failed run, about 2 minutes.
- **Would remove it:** `default-run = "jev"` in `crates/telperion-jev/Cargo.toml`, or `--bin jev` in the doc's five commands.

## 2026-10-07: a redirect to a path held in a variable is blocked

- **Doing:** writing literature and screen logs under the spec's `raw/` folder from a loop.
- **Slowed by:** the shell guard refuses `> $DIR/file` and `> file-$(...)` (a dynamic redirect target); each loop had to be rewritten as a script with literal paths or piped through `tee`.
- **Cost:** two blocked commands, about 3 minutes.
- **Would remove it:** nothing in the repository; a local guard setting (reported, not specced).

## 2026-10-07: the GPU lock is held by another session's workspace test

- **Doing:** rendering sheet 1 (6 trees, 6 shots each) under the shared GPU lock.
- **Slowed by:** another session holds the lock for `cargo test --profile ci --workspace` (the whole gate, not a GPU step), so the stills wait behind a CPU-bound run.
- **Cost:** about 15 minutes of waiting so far.
- **Would remove it:** take the GPU lock only around the GPU-touching test binaries (the render crate's), not the whole workspace gate.
