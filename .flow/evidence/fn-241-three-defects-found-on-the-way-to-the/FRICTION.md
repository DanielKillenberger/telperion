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

## 2026-10-09: R3's named route buys 0.2 s of the 1.4 s it needs

- **Doing:** restoring the cold build's 1.5 s margin besides `grow` after the F4 merge.
- **Measured** (beech at 20, cold, under the lock, two runs each):
  - Base: 4.87 and 4.94 s besides `grow`, so 0.13 s of margin. The other pipelines total 3.9 s: `girths` 1.07, `asks` 1.07, `bends` and `lays` 0.34 each, `dress` 0.26. The build itself is 0.85 s.
  - The named route (the place threshold's log-odds from the CPU table) takes `asks` from 1.07 to 0.99 s.
  - One `lead_q`/`presence` call site for the ask and its place takes it to 0.87 s. Besides `grow`: 4.77 s.
- **Slowed:** each try is a 40 s cold build under the shared lock, and the gain per inlined call site removed is a few hundred ms. Reaching 3.5 s means slimming `girths` as well (three `section_q` and two `radius_q` copies inlined).
- **Cost:** about 15 minutes so far.
- **Would remove it:** a per-pipeline cold-compile budget in the cold-build test, so a shader that grows its compile time is named when it lands, not when the first-tree total runs out of margin.

## 2026-10-09: the R1 test at 80 ran out of device memory in the gate

- **Doing:** the render half of the gate, with four test threads on the device.
- **Slowed:** `the_fine_path_stands_where_mesh_build_does` failed at the spruce at 80 with "Out of Memory" beside the other tests. docs/gpu-tests.md already says a tree at 80 on the device is a device-limits probe. I had not checked that before adding the test to the default suite.
- **Cost:** one render-half gate run, about 5 minutes.
- **Fix:** the test is split. The trees at 40 run by default; the trees at 80 are `#[ignore = "device limits: ..."]` and pass alone under the lock (`raw/r1-at-80.log`).
- **Would remove it:** `device_limit_probes_run_only_when_asked` could also flag a default test that names age 80.0, not only check the marked ones.
