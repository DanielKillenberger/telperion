# Friction reports, fn-84 (the generator declares what it can express)

Entries follow the friction rule in CLAUDE.md; the owner decides which become specs.

## 2026-09-19 the session's machine had no toolchain the workspace compiles on

- **Doing:** picking the partial `d424494` up and running the gates it had not run — the `telperion-core` and `telperion-jev` suites — before touching anything.
- **Hindered by:** the only `cargo`/`rustc` on this machine were the system `1.85.0` at `/usr/bin`, with no `rustup` at all, so `rust-toolchain.toml`'s pinned `1.98.1` could not be honoured. Every build failed on `E0658` in files this spec does not touch (`foliage/levels.rs`, `surface.rs`, `surface/normals.rs` use `slice_as_chunks` and `is_multiple_of`), which reads as nine compile errors in the generator rather than as a missing toolchain.
- **Cost:** about eight minutes — one failed suite run, the diagnosis, and a `rustup` install of `1.98.1` from `sh.rustup.rs` before any gate could run.
- **What would remove it:** this is a local setup problem on this machine, not a repository one — it is reported here and not specced. The only repository-side thing that would have shortened it is the toolchain check the gates do not do: a first build against an unpinned `rustc` reports its own errors, never the pin it is missing.
- **Early return:** not taken; the install is a one-off and the gates ran after it.

## 2026-09-19 the workspace test gate is OOM-killed before it reaches its own verdict

- **Doing:** running `cargo test --release --workspace`, the gate `package.json` names as `rust:test`, over the finished fn-84 change.
- **Hindered by:** the `species` suite's test binary is killed by the OOM killer at its default thread count and again at two threads. `dmesg` names it: `species-48ef87b` at 5,174,684 kB anon RSS on a box with 15 GB of RAM and 9 GB already held by other processes. Cargo reports it as `signal: 9, SIGKILL` under `error: test failed`, which reads as a failing test rather than as a machine that ran out of memory; the run stops there and every suite after `species` never runs, so the gate returns 101 without a verdict on most of the workspace.
- **Cost:** about eighteen minutes — one killed full-workspace run, one killed two-thread rerun of the suite, one green single-thread rerun at 108 s, and a full re-run of the gate at `--test-threads=1`.
- **What would remove it:** one suite holding 5 GB in a single binary is repository-side, whatever the box: `rust:test` names no thread cap, so the gate's peak memory is the product of that suite and the host's core count and nothing bounds it. A thread cap on the gate command, or a bound on what the `species` suite holds at once, would make the gate's cost knowable before it runs. The box's size is local and is not specced; the unbounded peak and the `SIGKILL`-reads-as-test-failure are not.
- **Early return:** not taken; the single-threaded rerun is green and the gate had to produce a verdict for this spec.

## 2026-09-19 a two-crate change pays the whole workspace's fat-LTO price twice

- **Doing:** the two gates this spec needs — `cargo clippy --release --workspace --all-targets -- -D warnings` and `cargo test --release --workspace`.
- **Hindered by:** both run on the `release` profile, which `Cargo.toml` pins at `lto = true, codegen-units = 1`. Clippy alone measured 4 m 20 s of wall clock for 53 s of user time, almost all of it link. The same crate's two touched suites compiled and ran under `--profile ci` in seconds. The repository already ships that `ci` profile, and its comment says it exists "so the test binaries link in seconds" and that "every package script and local command stays on release" — which is the owner's decision, and this entry only records what the decision costs a change that touches two crates.
- **Cost:** roughly ten minutes of the session's wall clock in link time that produced no information the `ci` profile would not have produced.
- **What would remove it:** a package script for the local pre-PR gate on the `ci` profile, with the release run reserved for the captures and the numeric protocol that actually need the optimised build. The owner decides whether the pinned identity digests make that unsafe.
- **Early return:** not taken; both gates were required and neither was avoidable once started.
