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
