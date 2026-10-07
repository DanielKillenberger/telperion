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
