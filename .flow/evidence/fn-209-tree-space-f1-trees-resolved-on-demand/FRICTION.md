# fn-209 friction

## 2026-10-05, the light lattice is private to its crate (worker, R0)

- **Doing:** measuring, for the proposal, how much bud light changes when each small subtree's leaves are gathered at its centroid (a stand-in's light coupling).
- **Slowed:** `light::Field` is `pub(crate)`, and an example sees only the public API, so the probe rewrote the lattice (deposit, sweep, read; about 150 lines) and checked the rewrite against `bud_light` (most apart 0.0). The alternative, a temporary test module inside `src/`, meant editing the crate for evidence.
- **Cost:** about 10 minutes and one extra build.
- **Would remove it:** a `light::field(light, leaves, reads) -> impl Fn(Vec3) -> f64` in the public API beside `bud_light`, or an evidence feature that exposes it to examples.

## 2026-10-05, one test binary of the crate's suite runs 404 s (worker, step 1)

- **Doing:** running `cargo test --profile ci -p telperion-space` once after the closed-form refactor.
- **Slowed:** every other binary of the crate finishes in under 9 s; one (11 tests, 1 ignored) took 404 s, most of the suite's wall time. The quick red/green loop for the envelope ran on `--lib envelope` instead.
- **Cost:** about 7 minutes of waiting, once.
- **Would remove it:** naming the slow binary's tests and moving the long walks behind the gate's own run, or splitting them, so a crate-level check stays under a minute.

## 2026-10-06, the competition's contenders share one desk (contender 4, option 4)

- **Doing:** timing chain resolution and the closed form's rows for the competition's performance numbers.
- **Slowed:** the 1-minute load reached 150 on 32 cores while other contenders and a walk test ran; the same beech 40 run went from 0.07 s to 0.39 s one-pass and its rows from 3.3 s to 16 s. Timings taken then are not comparable between contenders.
- **Cost:** one perf run thrown away, about 5 minutes of waiting for a quieter window.
- **Would remove it:** the host running every contender's `perf` itself, one after another, on a quiet machine (or a shared lock like the GPU lock for timing runs).

## 2026-10-06, the closed form's rows cost O(m³) per species and age (contender 4, option 4)

- **Doing:** reading every committed boundary a bud can need (each borne PA at every age 0 to the tree's) up front.
- **Slowed:** each kept table of a bud m cycles old works every threshold t ≤ m, and each m is its own memo entry, so the beech's limb rows alone took 3.3 s at 40 years on one core (the seed's table 0.03 s).
- **Cost:** the test bed's trees could not each read their own rows; they now share one read per species and age, spread over the cores.
- **Would remove it:** a kept table that carries every m at once (one recursion yielding the row at each age), or rows kept between runs on disk.

## 2026-10-06, a still at age 80 holds the GPU lock through the closed form's rows (probe P2)

- **Doing:** rendering the P2 stills (beech, spruce, palm, oak at age 80, seed 1) under the shared GPU lock.
- **Slowed:** the stills runner reads the closed form's rows inside the locked command, and the rows take 87 to 103 s at age 80 on this branch (P1's subject), so the GPU lock is held through a minute and a half of CPU work per alone still, and the other probes wait on it.
- **Cost:** about 3 minutes of lock time held without the GPU, once.
- **Would remove it:** rows read outside the lock (a `--rows` file the runner loads, or the stills runner taking the lock only around `render`), or P1's faster tables.

## 2026-10-06, no sampling profiler on the desk (probe P1, tables)

- **Doing:** profiling where the closed form's table reads spend their time, as the probe brief asks first.
- **Slowed:** `perf` is not installed and `samply` refuses to record because `kernel.perf_event_paranoid` is above 1; the profile had to be taken by hand with timers and call counters patched into the code and taken out again.
- **Cost:** about 10 minutes and two extra builds.
- **Would remove it:** `perf_event_paranoid` at 1 on the desk (a local setup matter, reported not specced), so `samply record` runs without sudo.

## 2026-10-06, timing the base beside a change needs a second copy (probe P1, tables)

- **Doing:** timing the closed form's tables on the base commit beside the changed code, with the same timing example.
- **Slowed:** the example is new, so it does not exist on the base; the shell guard refuses `git checkout <ref> -- <path>` and redirects to a computed path, so the base ran from a `git archive` copy of the crate in the scratchpad with its own target directory and a hand-written `[workspace]` and `ci` profile. The base read itself took 4 minutes (oak 140 s at 80).
- **Cost:** about 10 minutes.
- **Would remove it:** a timing example that lands with the code it times, so a later change has a base to run it on.

## 2026-10-06, `cargo fmt` reformats files nobody touched (task 1, fine roots)

- **Doing:** formatting the fine-root executor before committing (`cargo fmt -p telperion-space`).
- **Slowed:** the crate holds files rustfmt disagrees with on the base (`examples/alone.rs`, `examples/field.rs`, `examples/junctions.rs`, `src/light.rs`, `src/light/tests.rs`), so every package-wide format rewrites them, and the unrelated hunks had to be reversed by hand (`git diff | git apply -R`, since the shell guard refuses `git checkout --`) twice before each commit.
- **Cost:** about 5 minutes and two near-misses of committing unrelated formatting.
- **Would remove it:** one formatting commit of those five files on master, and a `cargo fmt --check` step in CI so the crate stays formatted.

## 2026-10-06, measuring a contract candidate on the base needs a throwaway worktree (task 1, fine roots)

- **Doing:** dumping the four species' trees on the base and on two candidate patches (pinned `libm`; waking times in ticks) to measure how far each moves the trees.
- **Slowed:** the dump example is new, so the base needed a temporary `git worktree` with its own `target/` (the scratchpad's stale `base` worktree entry from another session blocked the obvious name); reverting the first patch there to apply the second needed `git apply -R` because the guard refuses `git checkout --`.
- **Cost:** about 6 minutes, three builds of the crate (1 minute each with the dumps).
- **Would remove it:** the dump example (`examples/contract.rs`) now lands with this task, so the next candidate is a patch and one run on this branch.

## 2026-10-06, the shell guard refuses edits through heredocs and dynamic paths (task 2, GPU executor)

- **Doing:** applying multi-line source edits with short Python scripts, and copying raw evidence into the evidence folder.
- **Slowed:** the shell guard refused a heredoc'd Python script whose body it read as a dynamic shell launcher, `mkdir -p` with a redirect into a not-yet-made directory, `>` to a path built from a variable, and `mv` with a variable path. Each needed the script written to the scratchpad first, or literal paths, or `cp`.
- **Cost:** about 6 minutes over five refusals.
- **Would remove it:** nothing in the repository; a local guard setting (reported, not specced).

## 2026-10-06, a WGSL reserved word costs a locked GPU run each (task 2, GPU executor)

- **Doing:** compiling the contract and chain shaders.
- **Slowed:** WGSL reserves many plain English words (`from`, `target`, `get`, `pass`, `move`, `set`, ...). Each one is found only when wgpu builds the shader, which is inside a test run under the GPU lock; four runs failed this way, and naga also wants a `return` after every loop that returns.
- **Cost:** about 4 minutes and four short lock holds.
- **Would remove it:** a CPU-side `naga` parse of every `.wgsl` under `src/generation` in an ordinary test (no device needed), so a shader that cannot parse fails before the GPU is asked for.

## 2026-10-06, the GPU lock held by another worker's workspace gate (task 2, GPU executor)

- **Doing:** re-running the fine roots' timing at 40 and 80 with smaller regions.
- **Slowed:** fn-211.2's workspace gate took the GPU lock at 20:37 for its whole run, CPU tests included; my measurement waited behind it.
- **Cost:** about 20 minutes of waiting (20:37 to 20:57), spent writing; and a wait loop of mine that matched its own command line and never ended, until stopped by hand.
- **Would remove it:** a gate that takes the GPU lock only around the tests that use the GPU (a test filter or a separate GPU test binary), so CPU-only test time does not block GPU work on the desk.

## 2026-10-06, the GPU tests hold the lock through their CPU reference (task 2, GPU executor)

- **Doing:** the fine roots' equality and timing tests, which grow the CPU reference (`fine_cases`, `chain_cases`, `grow_fine`) and then the GPU's, in one test process.
- **Slowed:** the GPU lock is taken around the whole `cargo test`, so the CPU reference's growth (up to about 40 s of the 72 s measurement at 80) holds the lock that other workers wait on.
- **Cost:** about a minute of lock time per measurement run on the desk, and for the 25-year equality test in the gate about 30 s.
- **Would remove it:** CPU references written once to disk by an unlocked step and read by the GPU test, or the lock taken inside the test only around the dispatches.
