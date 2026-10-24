# fn-314 friction

## 2026-10-24 14:05, conductor, plan: R3 places the new checks "beside fn-306's two guards" in gate.py, where none are

- **Doing:** planning R3 (add the two CI checks to `scripts/gate.py`'s pre-run guards).
- **Slowed by:** fn-306's two guards are not pre-run guards in `gate.py`; they are nextest priority overrides in `.config/nextest.toml` that start first inside the test run. `gate.py`'s only steps before the run are the allowlist, stale-entry and doc-test checks. The spec's architecture claim was not checked against the code, so the plan had to decide where a subprocess check goes and what "beside the gate" means for one that costs more than a minute.
- **Cost:** about 5 minutes of reading `gate.py`, `.config/nextest.toml` and AGENTS.md's gate line.
- **Would remove it:** checking a spec's named mechanism against the code before capture (AGENTS.md, "Gates and checked claims").

## 2026-10-24 14:08, conductor, plan: a cold target directory for the worktree

- **Doing:** reproducing R1 with CI's command on the base.
- **Slowed by:** the f4-314 worktree has no `target/`, so the repro is a cold build of the core crate's dependencies at two threads, while another worktree's gate holds the desk.
- **Cost:** 49 s wall for both CI commands from cold (`--no-run` for the tests), through telperion-heavy at two threads; small, so this entry records it rather than complains.
- **Would remove it:** nothing in the repository; checkouts must not share `target/` (AGENTS.md), so a new worktree pays one cold build.

## 2026-10-24 14:00, conductor, task .1: the worker skipped the impl-review the owner asked for

- **Doing:** collecting task .1's worker return.
- **Slowed by:** the owner asked for an impl-review on every task, but the worker prompt carried only `REVIEW_MODE: codex`, and the worker applied working-rules.md's risk rule and skipped the review for a one-attribute, one-row diff. The conductor ran the review itself after `done` and rewrote the task's stage line.
- **Cost:** one extra dispatch and about 3 minutes; the review itself took 43 s and found nothing.
- **Would remove it:** a dispatch field that forces review regardless of the risk rule when the person asked for one (a flow-next change, not this repository's); here the next workers get the request in their prompt.

## 2026-10-24 14:07, worker, task .2: the first cost measurement lost its timings

- **Doing:** timing CI's no-geometry check after a core change and warm, through `telperion-heavy 8 --threads 4`.
- **Slowed by:** the timing used `bc`, which this desk lacks, so both runs finished without a wall time and had to be repeated (the cargo logs alone gave the build and test times). The plan's 49 s figure covered only the build and `--no-run`; the check's lib tests take 139-172 s, which is what puts the check after the test run.
- **Cost:** about 6 minutes of desk time (two extra runs of the 142 lib tests).
- **Would remove it:** timing with shell arithmetic on `date +%s%N` from the start; a plan-time measurement that runs the check as CI does, not `--no-run`.

## 2026-10-24 14:26, worker, task .3: the core nextest run was OOM-killed at telperion-heavy's 5 GB cap

- **Doing:** running the `core` job's command (`cargo nextest run --cargo-profile ci -p telperion-core --no-fail-fast`) through `telperion-heavy 5 --threads 2`.
- **Slowed by:** `--threads 2` sets `RUST_TEST_THREADS`, `RAYON_NUM_THREADS` and `CARGO_BUILD_JOBS`, but nextest reads none of them for its test slots and starts one test per core, so the run held about eight tests at once (10 min 23 s CPU in 73 s wall), reached the scope's 5 GB `MemoryMax`, and the kernel OOM killer took one test; nextest then SIGTERMed the other 35 running. Result: 492 passed, 36 failed, all of them killed, none a test verdict.
- **Cost:** one 74 s run and its triage, about 5 minutes.
- **Would remove it:** `telperion-heavy` also exporting `NEXTEST_TEST_THREADS="$threads"` (a desk script outside the repository: reported, not specced). The rerun sets `NEXTEST_TEST_THREADS=2` by hand; it changes only how many tests run at once, not which.

## 2026-10-24 14:29, worker, task .3: two test slots still exceed the 5 GB cap

- **Doing:** rerunning the `core` command with `NEXTEST_TEST_THREADS=2` inside `telperion-heavy 5 --threads 2`.
- **Slowed by:** `suite::mesh::every_preset_builds_a_mesh...` (4,374 MiB measured) and `suite::packed_leaf::every_station_stands...` (4,344 MiB) are both on the gate's memory allowlist and were scheduled together; the pair passed 5 GB and the OOM killer ended the run at 344/532 (342 passed, the two killed). The machine rule's 5 GB cap fits one of the core's two 4.3 GiB tests at a time, never two.
- **Cost:** a 199 s run, about 4 minutes.
- **Would remove it:** a dispatch's cap sized from `.config/gate-budgets.toml`'s largest pair (about 9 GB for two slots), or one slot under 5 GB. The next run uses one slot (`NEXTEST_TEST_THREADS=1`), which keeps the machine rule and costs wall time.

## 2026-10-24 15:14, worker, task .3: the render run was OOM-killed at 5 GB after 23 minutes

- **Doing:** running the `render` job's command (`cargo nextest run --cargo-profile ci -p telperion-render --no-fail-fast`) with two test slots inside `telperion-heavy 5 --threads 2`.
- **Slowed by:** `conformance::every_shipped_family_renders_through_the_one_path` and `boundary::a_stage_outside_the_executor_interface_does_not_compile` (a compile-fail test that runs rustc) ran together and passed the 5 GB cap 1,285 s in; the OOM killer ended the run at 282/359 (280 passed, the two killed, 77 never started). Neither test is on `.config/gate-budgets.toml`, so nothing on the desk predicted it.
- **Cost:** about 24 minutes of desk time, most of it repeated by nothing: the 79 tests the run did not pass are rerun alone, one slot, by a filterset built from the run's PASS lines.
- **Would remove it:** a per-test memory figure for the render crate's heavy tests (the gate's runner measures peaks; the allowlist only names those over 4 GiB), so a dispatch can size its cap; or one slot for every crate under a 5 GB cap.

## 2026-10-24 18:57, worker, task .3: one space test alone needs more than the 5 GB cap

- **Doing:** running the `wasm, field and jev` job's command (`cargo nextest run --cargo-profile ci -p telperion-wasm -p telperion-space -p telperion-jev --no-fail-fast`) with one test slot inside `telperion-heavy 5 --threads 2`.
- **Slowed by:** `walks every_setting_changes_the_tree_by_degree_and_branches_come_and_go_at_vanishing_size` is on `.config/gate-budgets.toml` at 5,396 MiB measured (5,952 allowed), so it cannot pass under a 5 GB scope even alone; the OOM killer ended the run at 595/609 (594 passed, that one killed, 14 never started). The machine rule's 5 GB cap is below the gate's own allowlisted figure for this test.
- **Cost:** a 456 s run, about 8 minutes with triage.
- **Would remove it:** a dispatch cap of at least the largest allowlisted peak plus headroom (6 GB) for any run that includes `telperion-space`. The 15 tests this run did not pass are rerun alone at one slot inside `telperion-heavy 8 --threads 2` (the gate's memory size), a deviation from the dispatch's 5 GB rule the test's own measured peak forces; the result names it.
