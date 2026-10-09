# fn-241 friction

## 2026-10-09: a fresh worktree builds everything cold

- **Doing:** starting R1 to R3 in `.worktrees/found`, which has its own `target/` (as AGENTS.md requires), no `node_modules` and no built Wasm.
- **Slowed:** the first `cargo test --profile ci -p telperion-render --no-run` compiles the whole dependency tree, and npm test needs `npm ci` plus both Wasm builds before vitest can start.
- **Cost:** minutes of wall time before the first measurement; numbers recorded below as they land.
- **Would remove it:** nothing in the repository; a separate target per checkout is the rule, so a cold first build is its price.

## 2026-10-09: the first R1 instrument ran out of device memory

- **Doing:** measuring every shipped species at 40 and 80 on both paths in one test.
- **Slowed:** holding the CPU mesh and both prepared trees at once ran the spruce at 80 out of device memory ("Out of Memory", `DeviceRefused`) after 80 s.
- **Cost:** one 80 s GPU run and a rerun (91 s).
- **Would remove it:** nothing to change in the repository; the instrument now drops each tree before the next is made.

## 2026-10-09: `cargo fmt -p` rewrote 26 files the change never touched

- **Doing:** formatting the R1 test.
- **Slowed:** the workspace is not rustfmt-clean, so `cargo fmt -p telperion-render -p telperion-core` reformatted 26 unrelated files and reordered `fine.rs`'s module list. Reverting them took a reverse `git apply`, because `git checkout --` is blocked here.
- **Cost:** about 5 minutes.
- **Would remove it:** a rustfmt-clean workspace, or a CI format check that keeps it clean; until then, `rustfmt --check` on the changed files only.

## 2026-10-09: an instrument hung holding the GPU lock

- **Doing:** the R1 stills and cost probe, with the fine kernels and tools made before the first `prepare`.
- **Slowed:** the process sat in futex waits with the GPU idle for 10 minutes, holding the shared GPU lock, while another session's cold-build test waited on it. I killed it by PID. With the kernels made after `prepare` it ran in 19 s.
- **Cost:** about 12 minutes of mine and of the other session's lock.
- **Would remove it:** a `timeout` on every GPU instrument run under the lock (used from here on). The hang itself is not diagnosed.

## 2026-10-09: the eight-tree bounds test ran the spruce at 80 out of device memory

- **Doing:** the R1 test, every shipped species at 40 and 80 in one process.
- **Slowed:** two of four runs failed at the spruce at 80 with `DeviceRefused { "Out of Memory" }`, a flake.
- **Cost:** two runs, about 3 minutes.
- **Would remove it:** a device poll after each tree is dropped, now in the test; it passed the next three runs.
