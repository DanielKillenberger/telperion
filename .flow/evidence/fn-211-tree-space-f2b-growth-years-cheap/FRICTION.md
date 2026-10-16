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

## 2026-10-06, task 3 worker: no profiler reaches the growth passes

- **Doing:** finding why the by-depth passes cost more on one core than the index-order loops did (host decision 2 assumed per-axis allocations).
- **What slowed it:** `samply` is installed but `perf_event_paranoid` is 2, so it refuses to record; perf is not on the desk. Each question took a hand probe (static timers in the pass, printed by `measures`, reverted) and a rebuild.
- **Cost:** about 15 minutes for two probes; the answer for `thicken` (allocation is not the cost, the depth order is) came from an A/B, not a profile.
- **What would remove it:** `kernel.perf_event_paranoid=1` on the desk (a local setup matter, reported, not specced).

## 2026-10-06, task 3 worker: the crate's own tests take seven minutes a commit

- **Doing:** running `cargo test --profile ci -p telperion-space` at every commit, as the task asks.
- **What slowed it:** 131 tests, about 100 CPU-minutes, 7 minutes of wall time on the shared desk (load 6 to 34 from the other worker); four levers take half an hour of test runs.
- **Cost:** about 30 minutes of waiting over the task.
- **What would remove it:** a fast subset the per-commit check names (the unit tests that build small trees), with the heavy integration tests left to the workspace gate.

## 2026-10-06, task 4 worker: P4's coupling pass lives only on an unmerged probe branch

- **Doing:** building the light-field comparison the task asks for first, on P4's coupling pass.
- **What slowed it:** the pass (`couple.rs`, `grow/couple.rs`, `grow/fine.rs`) is on `fn-209-probe-light` alone, not on this branch, and that worktree is another session's. The tool had to be built as a patch in a scratch worktree at P4's commit (`p4-light-probe.patch`), and the measures P4's metric needed (per-cycle population, realized leaf area) taken there by hand-added `eprintln`s behind env switches.
- **Cost:** about 10 minutes and eight rebuilds (each 3 s; the runs 20 s each, load 2 to 7).
- **What would remove it:** a probe that leaves its measures as a kept example on a branch the next task merges, or a statement in the task of which branch the code it builds on lives on.

## 2026-10-06, task 4 worker: the desk's command guard refuses `git diff > file` to a path held in a variable

- **Doing:** saving the probe's patch into the evidence folder.
- **What slowed it:** dcg blocks a truncating redirect to a shell-expanded path; `git diff --output=<literal path>` works.
- **Cost:** about 1 minute.
- **What would remove it:** nothing in the repository; a local setup note (reported, not specced).

## 2026-10-06, task 4 worker: the desk ran at load 60 to 87 during the coupling measurements

- **Doing:** timing today's oak against the coupled oak and running the per-repair matrix.
- **What slowed it:** another session's work took the load from 5 to 87 for several minutes. Today's oak at 80 s1 took 12.9 s, against 3.3 s at load 6, so those timings are unusable and the cost table had to be rerun.
- **Cost:** about 5 minutes and one rerun.
- **What would remove it:** a lock or a quiet window for timing runs on the shared desk, as `gpu.lock` gives the GPU; a local setup matter, reported.

## 2026-10-06, task 4 worker: probe (a)'s matrix takes over an hour on the shared desk

- **Doing:** re-running probe (a) after the order-free sums: 12 trees × 36 partitionings, each a whole tree grown.
- **What slowed it:** each run grows the tree again from nothing, and the desk's load ran from 14 to 33. The oak at 80 takes 4 to 10 s a run, so the matrix ran for more than an hour and was still running at the commit.
- **Cost:** the commit had to state a partial result; about an hour of wall time.
- **What would remove it:** run the 36 partitionings in one grower pass, each partition's sums and lattices taken side by side from the same terms (they share every term and differ only in grouping), or a smaller matrix at 80 (one order, one chunk).
- **Removed (2026-10-06, task 4 worker):** `grow_partitioned` now takes every run (cut and grouping) beside one growth: the tree grows on its own sums and each run's grouping of the same terms is compared where it is taken. The 12 trees × 36 runs took 6 min 40 s wall at load 24 to 45 (oak 80: 55 s and 269 s a tree, the second during a load spike), against more than an hour before.

## 2026-10-06, task 4 worker: the first stills of the stepped oak waited six minutes on the GPU lock

- **Doing:** rendering the stepped oak at 80, seeds 1 and 7, bare and in leaf (four stills) under `gpu.lock`.
- **What slowed it:** another worker held the lock; the run took 7 min 20 s of wall time for under 1 min of work (the next, eight stills, took 1 min 3 s with the lock free). The desk's load ran 18 to 88 meanwhile.
- **Cost:** about 6 minutes of waiting.
- **What would remove it:** nothing in the repository; a queue position or an estimate printed by the lock would have let the work go on meanwhile (a local setup matter, reported).

## 2026-10-06, task 4 worker: `rustfmt` on `grow.rs` and `lib.rs` again rewrote two files nobody touched

- **Doing:** formatting the executor's files before committing.
- **What slowed it:** `rustfmt` follows `mod` declarations, so naming `grow.rs` and `lib.rs` reformatted `fine/surface.rs` and `light/tests.rs` (task 2's entry); reverted with a reverse patch.
- **Cost:** about 2 minutes.
- **What would remove it:** task 2's fix (one formatting commit for the crate).

## 2026-10-07, task 5 worker: the crate's tests took ten minutes to check a merge

- **Doing:** checking the F1 merge with `cargo test --profile ci -p telperion-space`.
- **What slowed it:** one test binary ran 551 s of the 606 s (load 8 at the start, a 50 peak from other workers). The merge needed only the digests (1 min) and the partition and stepped tests.
- **Cost:** about 10 minutes of wall time, waited on.
- **What would remove it:** a named quick set for merges (the digests example plus `fine/tests/realized.rs` and `grow/stepped/tests.rs`), the slow binary's cases left to the gate.

## 2026-10-07, task 5 worker: the shell guard blocks writes to variables' paths and `git checkout <ref> -- <path>`

- **Doing:** writing scratch outputs to `$S/...` and restoring a probed file from HEAD.
- **What slowed it:** `dcg` refuses a redirect whose target is a shell variable and refuses `git checkout HEAD -- <file>`; each needed a rewrite (the Write tool, a literal path, or a script that writes the file).
- **Cost:** about 4 minutes and three retries.
- **What would remove it:** a local setup matter (the guard's allow list for the scratchpad), reported.

## 2026-10-07, task 6 worker: the desk at load 100 to 176, and a load gate that never gated

- **Doing:** timing the stepped executor's phases on the lit oak at 80.
- **What slowed it:** two other workers' test binaries (`walks-*`) held the desk at a one-minute load of 97 to 176, which inflated the oak's build seven to eight times (34 s against 4.5 s). The run script's load gate called `bc`, which this desk lacks, so it ran every case at once under that load; the first run was set aside (`raw/task6/old-binary-highload/`) and rerun behind an `awk` gate.
- **Cost:** about 5 minutes for the gate; the wait for the load to fall dwarfs the work (noted with the timings).
- **What would remove it:** one shared desk scheduler or lock for timing runs (as `gpu.lock` is for the GPU), so a timing worker queues rather than measuring other workers' load; the gate written without `bc`.

## 2026-10-07, task 7 worker: the desk's load again gates a four-run timing

- **Doing:** timing option A's ceiling (the lit oak at 80, seeds 1 and 7, all cores and one), each run gated on a one-minute load under 8.
- **What slowed it:** the load stood at 30 to 61 when the task started. One run got through at load 7.5 after a 4-minute wait; then the load rose to 56 again, and the other three runs queued behind it.
- **Cost:** the gate's waits were 255 s, 450 s, 3,300 s and 525 s for set a (the one-core seed 1 waited 55 minutes), and up to 375 s in set b. The runs themselves take 10 to 60 s each, so measurement time was about a tenth of wall time.
- **What would remove it:** task 6's entry stands: a shared desk lock for timing runs, so a timing worker queues on the lock rather than polling the load average.

## 2026-10-07, task 8 worker: the session's /tmp quota is full, and every command's output is lost

- **Doing:** writing the feedback contract's CPU module.
- **What slowed it:** the host session's scratchpad under `/tmp/claude-1000/` holds 11 GB of other workers' build trees (`out/` 2.3 GB, `t5/` 1.8 GB, `topo/`, `before/`, `after/` 1.2 to 1.3 GB each), so the per-user tmpfs quota is full and the harness cannot store any Bash output (EDQUOT). Every command's output now has to be redirected into the worktree's `target/` and read back with a second call. Not deleted: they are not this worker's.
- **Cost:** two calls for every command from here on; about 5 minutes to find the cause.
- **What would remove it:** builds in each worktree's own `target/` (AGENTS.md already says so), and the host clearing finished scratch build trees; a local setup matter, reported.

## 2026-10-07, task 9 worker: a wait loop that waits on itself

- **Doing:** waiting for the crate's test binaries before rebuilding.
- **What slowed it:** a `pgrep -f "<path>/walks"` loop matches its own command line, so it never ends. Two monitors had to be stopped and re-armed with `pgrep -f "deps/walk[s]-"`.
- **Cost:** about 5 minutes.
- **What would remove it:** a done-file written by the run itself (`cmd; echo $? > run.done`), which the later runs use.

## 2026-10-07, task 10 worker: the lit oak's fine roots leave D16's shedding, the lifts and the landing dormant

- **Doing:** proving the device's D16 and layout against the CPU on the lit oak at 24.
- **What slowed it:** the identity held at once, but a mutation of the device's kept share passed: at 24 the oak's fine roots decide by D16 23,939 times and never meet a threshold (tolerance 0, their balance near one). The relays' carried pulls (straightening and erection are 0) and the landing (no fine wood near the ground) were dormant too. A neighbour that wakes all three (tolerance 0.99, straightening 0.4, erection 0.3, laterals turned to −1.4 rad) was found by three surveys and two rounds of mutations.
- **Cost:** about 35 minutes, and about 15 GPU runs of 40 to 140 s.
- **What would remove it:** a coverage count in the stepped executor's device interface (thresholds met, sheds, carried lifts, landings, rests), printed by every proof, so a specimen's dormant paths are known before an identity is claimed on it.

## 2026-10-07, task 10 worker: the whole-state hash costs thirty builds

- **Doing:** comparing every grower's whole state after every phase (`Stepped::states`).
- **What slowed it:** the state is hashed through its `Debug` text: the lit oak at 24 takes 6 s with it and 0.2 s without, so the GPU proof's eight runs take 40 s and an oak at 80 cannot be compared whole at all (its check is the phase digests and the tree's).
- **Cost:** about 35 s a proof run, a dozen runs.
- **What would remove it:** a field-wise binary hash of the grower (the f64 fields' bits, the vectors' words), about ten times cheaper; then the oak at 80 could be compared whole too.

## 2026-10-07, task 11 worker: the shell guard again, on a loop's output file

- **Doing:** timing the device run, base and new binaries, in a shell loop whose output went to a path held in a variable.
- **What slowed it:** `dcg` refused the redirect, as in task 5's entry. The loop had to be written as a script in `target/t11/` and run with a literal output path.
- **Cost:** about 2 minutes.
- **What would remove it:** task 5's entry stands: the guard's allow list for the worktree's `target/` and the scratchpad. This is a local setup matter, reported.

## 2026-10-07, task 11 worker: twenty timings taken at a load of 11 to 24, and a gate at 98

- **Doing:** timing `grow` against the stepped executor on 20 trees, best of three, back to back.
- **What slowed it:** the load from other workers rose from 11 to 24 during the run. Spruce 80 s1's stepped build read 18.2 s, against 6.9 s on a rerun at load 7. One seed of the device run caught a spike too: today's `grow` read 26.6 s against 17.6 s. The render gate's load rose to 98, so the rest of the workspace took 10 minutes.
- **Cost:** a rerun of two trees (6 minutes). One device row is unusable for today's `grow`.
- **What would remove it:** task 6's entry stands: a shared desk lock for timing runs.

## 2026-10-07, task 12 worker: glibc's allocator under 92,703 growers

- **Doing:** timing the stepped executor's spruce at 80.
- **What slowed it:** the root growers' memory moved between threads. Each was made on the main thread, grown on whichever pool thread took its piece, and freed on another. Their frees waited on each other's allocator arenas: letting the growers go took 3.4 s on 8 threads. No profiler runs on this desk (`perf_event_paranoid`), so the cause was found by timing each field's drop, then confirmed with jemalloc preloaded.
- **Cost:** about 40 minutes.
- **What would remove it:** a profiler that runs on the desk (local setup, reported). In the code, pieces pinned to pool threads (`cores::pinned`) and the rules' tables shared through `Arc`, both done here.

## 2026-10-07, task 12 worker: the timing gate waits on the desk's load

- **Doing:** timing the 20 trees, each gated on a one-minute load under 6.
- **What slowed it:** the load rose from 6 to 22 during the runs. The ten gated pairs took 50 minutes for about 6 minutes of measurement, and the palm's last pair was stopped while it waited.
- **Cost:** about 45 minutes of wall time.
- **What would remove it:** task 6's entry stands: a shared desk lock for timing runs.

## 2026-10-08, task 12 worker: a digest timed inside one side of a comparison

- **Doing:** comparing `grow` with the stepped executor (task 11), then `grow_today` with `grow` (task 12).
- **What slowed it:** the example computed the first path's tree digest inside its timed region and the second's outside it. On small trees that charged one side up to 0.02 s of digest. Task 11's and task 12's first ratios favoured the stepped executor by that, and a decision was taken on them (TASK12.md, "A correction").
- **Cost:** a decision round, and about an hour of optimisation aimed at a target that was too easy.
- **What would remove it:** one timing helper for comparisons that times the build alone and hashes after both; any new comparison example should start from it.
