# fn-212 friction

## 2026-10-07 10:28, task 1 (worker): the branch tip does not compile

- **Doing:** building the baseline probe on `1419a832` (fn-198's tip with F1 through task 9 merged).
- **Hindered:** `telperion-space` fails to compile: `fine/expansion.rs:405` assigns `std::time::Instant::now()` to the `f64` field `Stages::last`, a merge seam between F1 task 9's `Instant` and fn-198's `crate::clock` (host decision 8). The same merge put `std::time::Instant::now()` and `std::thread::scope` into `sow` (`expansion.rs:333-380`, `:342`), which `telperion_space::grow` reaches for the beech and spruce on every target, wasm32 included, where `Instant::now()` traps and threads cannot be spawned (not run in a browser here: unknown whether it traps there, but fn-198 decision 8 names exactly this).
- **Cost:** about 5 minutes and one failed build.
- **Removal:** the one-line fix (`self.last = crate::clock::seconds();`) is applied in the working tree for the measurement only, saved as `build-fix.patch`, and not committed: it is product code. The host should fix the seam on the branch, and decide `sow`'s clock and threads on wasm32 before R4.

## 2026-10-07 10:55, task 1 (worker): the baseline under a load of 38 to 60

- **Doing:** the baseline's first pass (four trees, three builds each), GPU lock held.
- **Hindered:** first the lock was held about 12 minutes by another worktree's workspace gate; then, during the pass, other workers' tests (a 9-core `walks` test, a `contract dump`) kept the one-minute load at 38 to 60 on 32 threads. The spruce's CPU stations ran 11 to 12 s against 5.8 to 7.2 s in the same step at load 10 to 12 an hour earlier, so the pass is not a clean baseline.
- **Cost:** about 15 minutes of waiting plus a repeated pass (about 6 minutes).
- **Removal:** a desk-wide quiet window for timing passes (a second lock, or the GPU lock's holders refraining from heavy CPU work while a timing pass runs). The figures carry their load and a second pass is taken.

## 2026-10-07 11:25, task 2 (worker): the GPU lock queued behind two workspace gates

- **Doing:** the base digests (the pipeline's hashes at 40 and 80) before the first deletion.
- **Hindered:** the lock was held by another worktree's workspace gate with a second gate and two GPU captures queued; my run queued at 11:07 and had not started at 11:25. `flock -w 900` gives up after 15 minutes, so a queued run can silently time out with empty output.
- **Cost:** about 20 minutes of wall time on the first proof (worked on code meanwhile); one driver rewritten to retry on `flock -E 99`.
- **Removal:** a lock with a visible queue (who holds it, who waits, since when), or gates that build first and take the lock only for their GPU tests.

## 2026-10-07 12:35, task 2 (worker): a proof run every half hour under a load of 70 to 100

- **Doing:** the per-deletion digests and timings, each process taking the GPU lock alone so other workers interleave.
- **Hindered:** two of the session's processes ran in the first hour (11:30 to 12:35); each waited 20 to 25 minutes for the lock, and the one-minute load reached 99 on 32 threads, so any timing taken now carries a load several times AUDIT.md's quietest pass.
- **Cost:** about an hour of wall time so far for two 40-year digests; the timings will need re-taking or wide error bars.
- **Removal:** the quiet window FRICTION's first timing entry asked for; or a per-desk queue that runs timing jobs only below a load threshold.

## 2026-10-07 14:10, task 2 (worker): /tmp full, then the machine out of memory

- **Doing:** the interleaved per-deletion timings and the CPU half of the gate.
- **Hindered:** the shared scratchpad in `/tmp` (tmpfs, 16 GB) filled to its quota with seven workers' dumps (11 GB), so tool output was lost (EDQUOT); then the machine ran out of memory and the host stopped the workers and deleted the scratch dumps, my probes' binaries and drivers with them, mid-session (the 80-year digest of the last state and every timing run lost).
- **Cost:** the session's remaining two hours of queued work; about 15 minutes to rebuild the probes and drivers inside the worktree.
- **Removal:** build outputs and dumps inside each worktree (as the host now requires), a memory ceiling per worker, and `CARGO_BUILD_JOBS`/`RUST_TEST_THREADS` set by default for agents.

## 2026-10-07 14:35, task 2 (worker): the split gate fails jev's replay tests

- **Doing:** the gate's CPU half, `--exclude telperion-render`, as the host now asks.
- **Hindered:** `telperion-jev`'s two replay tests read the render crate's examples (`headless`, `species_measure`, `geometry_benchmark`) from the target directory, which only the render half builds, so they fail in the CPU half on a fresh target ("missing: run the workspace gate"). Also, `generation_limits_are_classified` fails on this branch before any of my commits: nine limit sites in `pipeline/input/shoulder.rs` (fn-198's shoulder) are not in `docs/generation-limits-inventory.json`.
- **Cost:** one false red each; the replay tests are re-run after the render half.
- **Removal:** run the replay tests in the render half (or build the render examples before the CPU half); classify the shoulder's sites in the inventory on fn-198's branch.

## 2026-10-07 16:00, task 4 (worker): `cargo fmt` rewrites 20 files nobody touched

- **Doing:** formatting the task's two crates before the first commit.
- **Hindered:** `cargo fmt -p telperion-core -p telperion-render` reformatted about 20 unrelated files on this branch (`build.rs`, the catalogue, the scaffold, the mesh), so the branch is not fmt-clean. Reverting them took a saved reverse patch, because the shell guard refuses `git checkout --` on a dirty file.
- **Cost:** about 5 minutes.
- **Removal:** run `cargo fmt` once on fn-198's branch and keep a `cargo fmt --check` in CI, so a worker's fmt touches only its own lines.

## 2026-10-07 16:05, task 4 (worker): the limit guard does not see a `pub const`

- **Doing:** classifying the shoulder's sites after the folds moved.
- **Hindered:** making `BINS` public dropped it from the guard's scan (`const BINS` matched, `pub const BINS` does not), so its inventory entry had to be deleted as stale. A public constant that bounds generation escapes the guard.
- **Cost:** two guard runs, about 3 minutes.
- **Removal:** the guard's pattern for constants matching `pub`, `pub(crate)` and `pub(super)` too.

## 2026-10-07 16:55, task 5 (worker): two GPU probes in one test process run out of device memory

- **Doing:** the render half of the gate, then the 40 and 80 probes.
- **Hindered:** the libtest harness runs a crate's GPU tests on eight threads at once. Two fast probes (each a settled spruce, drawn and swept) beside the realized suite ran the 10 GB card out of memory, and so did the dress and sweep probes at 80 run in one command. Each red needed a rerun under the lock: about 3 minutes and one 70 s probe pass.
- **Removal:** GPU tests that hold whole trees take a process-wide device semaphore (or run under `--test-threads=1` by a nextest group), so concurrency is bounded by device memory, not by threads.

## 2026-10-07 19:45, task 6 (worker): the shell guard refuses a redirect to a variable path or an existing file

- **Doing:** saving each probe's output beside the evidence, `> $R/file.txt` and reruns into the same file.
- **Hindered:** dcg blocks a truncating redirect to a dynamic path, and to any existing file under home. Every rerun needed a new literal file name: `diag-1` to `diag-5`, `stationed-80-diag`, `-diag2`.
- **Cost:** two refused commands and about 3 minutes, plus a scatter of raw files.
- **Removal:** `tee` or an allowlisted evidence directory for probe output (`.flow/evidence/*/raw/`).

## 2026-10-07 19:50, task 6 (worker): classifying a residual run takes a full 40/80 probe pass each time

- **Doing:** finding why 20 spruce keys count differently at 80, one diagnostic print at a time.
- **Hindered:** every question about a residual meant rebuilding and rerunning the whole ignored probe, eight trees at about 55 s plus build, under the GPU lock. The test cannot select one tree, and core's `plan::runs` is not reachable from the render crate's tests (the contract exports only `Run`, `Plan`, `Descriptor`), so runs were rebuilt from segment endpoints.
- **Cost:** five passes, about 15 minutes.
- **Removal:** an env-selected tree and age for the ignored probes, and a probe dump of the residual runs (both sides' node lists) written once and read offline.
