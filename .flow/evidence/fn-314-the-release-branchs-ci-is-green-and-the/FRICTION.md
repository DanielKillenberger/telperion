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
