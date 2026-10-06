# fn-211 friction

## 2026-10-06, task 1 worker: a pass's time has no report below the stage

- **Doing:** profiling each pass of a growth year (balance, allot, grow, wake; sketch, re-lay, shade; settle's parts).
- **What slowed it:** `grow_staged`'s `Stage` reports one `Grown` per cycle and one `Settled`; the passes inside them, and the share of each spent on spread work, have no report, and perf is not on the desk. Timing them meant a hand patch over seven files (`profile-instrumentation.patch`), kept out of `src` and reverted.
- **Cost:** about 15 minutes of patching and a rebuild; no rerun.
- **What would remove it:** a per-pass timer behind the stage reports (a `Stage::Pass(name)` or a timing sink the probe passes in), so the next profile is a flag and not a patch. A host decision, since it widens a public enum.

## 2026-10-06, task 2 worker: `cargo fmt -p telperion-space` rewrites seven files nobody touched

- **Doing:** formatting the crate before committing a lever.
- **What slowed it:** seven files in `telperion-space` (`examples/alone.rs`, `examples/field.rs`, `examples/junctions.rs`, `src/alone.rs`, `src/bounded.rs`, `src/closed_form/boundary.rs`, `src/light/tests.rs`) are not rustfmt-clean on the base, so `cargo fmt -p` dirties them; restoring them takes a reverse patch, since the desk's guard blocks `git checkout --` and a redirected `git show`. Twice.
- **Cost:** about 5 minutes.
- **What would remove it:** one formatting commit for the crate (or a `cargo fmt --check` in CI that holds it); until then, `rustfmt <file>` on the files changed.

## 2026-10-06, task 2 worker: freeing a grown tree costs a quarter second, and a helper does not hide it

- **Doing:** finding the oak's unattributed 7 % (PROFILE.md, "Other").
- **Finding:** with the pass reports, it is freeing: the grower's per-axis state when `run` returns (oak 80 s1 about 165 ms, spruce about 160 ms), the shed axes inside `shed` (oak about 95 ms) and the full lay's tree at each re-lay (about 45 ms in `RelayWrite`).
- **What slowed it:** freeing beside the work on a helper thread (a bin that drops what it is given, built and measured, not committed) made the passes it overlapped slower than the freeing it hid: the spruce's settle and lay lost about 300 ms, the oak's attachments 90 ms, and across the growth years the oak's balance, allot and sketch each lost 25 to 30 %. Reverted.
- **Cost:** about 20 minutes.
- **What would remove it:** fewer allocations per axis (each `Draws` holds five vectors; each `Axis` two), a layout question for the host, not an exact parallel lever.

## 2026-10-06, task 2 worker: a spread spawns its threads each call, about 15 us a thread

- **Doing:** spreading the growth year's smaller passes (the light's lattice, the rows balance and allotment read, the sketch's generations) over the cores.
- **What slowed it:** `cores::spread` spawns its helpers through `std::thread::scope` on every call: 60 us at 4 threads, 0.48 ms at 32 (200 spreads of 64 empty pieces, this desk). A growth year at 70 to 80 runs a dozen spreads whose work is 1 to 5 ms each, so the spawning eats most of what they gain: the light's lattice built on every core (bounds and deposit by layer, built, hash-identical) measured no faster (176 to 181 ms over 80 years) and was dropped.
- **Cost:** about 15 minutes.
- **What would remove it:** a pool of helpers kept for the process, handed scoped work. Without a dependency (rayon) that takes `unsafe` lifetime erasure in `cores.rs`: a host decision.
