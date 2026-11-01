# Local verification reruns only what a change can affect

## Conversation Evidence

> user: "pls look up before we start my grok bot should have made a write up on how to make ci more efficient. Make the first spec here to optimize all verification times as to make progres as fast as possible."
> user: "We don't need to run ci on this branch on github either. Run verification locally. Keep it light and focused. Don't rerun shit that's not been changed etc. Be efficient this should be the first step of the goal"

## Goal & Context

The first step of goal fn-321. The run's progress is set by how long each child waits on verification, and today every child pays the whole gate once whatever it changed. On goal branch `b2645a302`, the gate's test run lists 1,514 tests and has a measured median of 1,241 s (fn-304), and the two extra checks add up to about 4 minutes after a core change. So a child that changes only a TypeScript file or a table still waits over 20 minutes, and a rerun after a one-line fix pays it again.

The pieces already exist. `scripts/ci-key.mjs` hashes the git objects of everything a suite reads, for `rust-core`, `rust-render`, `rust-wasm`, `rust-jev` and `node`, and CI skips a suite whose key has a green receipt. The local gate does not use it: `scripts/gate.py` always runs everything. Moving that idea to the local gate is the change.

Source: the owner's CI speed audit, `gno://daniel-os/projects/engineering/ci-speed-audit-2026-10-31.md`, and its playbook. For Telperion it reports 11.2 minutes median and 22 at p90 on GitHub (re-measured here on 1 Nov 2026: 11.2 and 25.8, n = 7), `wasm, field and jev` as the long pole at 15.9 minutes, and 19 of 30 runs failing; 17 of those failures are the `package` job's artifact ceiling and growth step. Those are GitHub findings, out of scope here. From the playbook this spec keeps its rule: never weaken a check to go faster. <!-- Source: 40% user / 40% [paraphrase] / 20% [inferred] -->

## Acceptance Criteria

- **R1 (skip what cannot have changed):** the local gate reuses `scripts/ci-key.mjs`'s suite keys and keeps a local receipt per suite, written only after that suite passes. A suite whose key has a green receipt is skipped and printed by name with its key; every other suite runs. Keys read the commit, so the gate refuses a dirty tree outside `.flow/` by name, or states that it ran uncached. [paraphrase]
- **R2 (the skip is proven):** ordinary tests with fixture changes assert which suites run: an input of a suite re-runs it, a shared input (`Cargo.lock`, the toolchain, `.config/nextest.toml`, the key script) re-runs all, a `.flow/` or Markdown change runs none, a failed or interrupted suite leaves no receipt, and a suite with no receipt runs. Each test is red on a temporary break of the rule it guards. [inferred]
- **R3 (light and fast):** on this machine, with warm builds: a `.flow/` or Markdown-only change verifies in under 10 s; a rerun with nothing changed in under 10 s; a change only under `src/` runs only the node suite and the package check; a change only to `crates/telperion-render` runs no core or space suite. Measured by running each case, recorded in this spec's `RESULT.md` with the before figure. [paraphrase]
- **R4 (focused while working):** a child iterates on the named narrow test only, and verifies once when it closes; a fix after a failure reruns the failed tests first (nextest's rerun of the failed set), then the rest of that suite only if they pass. `AGENTS.md`'s gate section says so with the new command, replacing "run once, at the end of a task" with the measured figures. [paraphrase]
- **R5 (nothing weakened):** the same 1,514 listed tests stay in the suites, with no test deleted, ignored, retried or moved out, and the gate's per-test budgets and its total hold for what does run. A full run stays one flag away and runs once before any final pull request. A suite's list before and after is compared and recorded. [inferred]
- **R6 (reviews are light too):** a review covers the child's own diff, and a task with no code change, or only `.flow/` and evidence, skips review with the reason recorded, under the working rules' risk rule. Reviews still run inside the queue. [inferred]
- **R7 (one checkout):** the goal run works in one long-lived checkout with its warm `target/`, committing children straight onto the integration branch, so no child pays a cold build. A cold build is stated when one happens. [inferred]

## Boundaries

- No GitHub CI: no change to `.github/workflows`, no pull request, and no pushing to trigger a run. The goal branch's pushes trigger nothing today, since `tests.yml` runs only on pushes to master and on pull requests.
- No paid runner or remote cache. No check weakened, as R5 states.
- `scripts/ci-key.mjs` stays the one source of suite keys; CI's use of it is unchanged. It lists `.github` files among its shared inputs, and those do not change here.
- Time box: 2 hours. This exists to speed up the goal, so it stops there with what works. Anything left goes to the goal log.

## Decision Context

The base (host, 1 Nov 2026): the gate at `b2645a302` was stopped at 1,269 of 1,514 tests to save about 35 minutes, with 1,266 passed and 3 timed out while it shared the machine. Alone, those 3 passed in 7, 33 and 7 s, so they were load (`cycle::grown_det_tests`, `cycle::retire_tests`, `cycle::settle_det_tests`). The remaining 245 tests have not run on this base. This spec's first full run covers them and is the run's first full proof of the base. A failure there that reproduces alone follows `AGENTS.md`'s rule for base failures (owner, 1 Nov 2026): the host judges and fixes it, and it stops the goal only as that rule says.

## Parked unknowns

- How much of a core change's cost can be cut: `telperion-core` and `telperion-space` feed every suite, so a change there still reruns all of them. Test-level selection is not attempted here. If child work shows this dominates, it goes in the goal log.
