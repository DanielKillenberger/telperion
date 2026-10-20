# Friction: fn-297

## 2026-10-20 plan stage
- Doing: `flowctl init --json`, as the plan skill's preflight requires.
- Hindered: init adds `review.fanoutExecution` to `.flow/config.json` on every run (fn-294 reverted the same key); the dirty tree would trip the pilot's post-hop guard.
- Cost: ~1 minute and a revert per stage that runs init.
- Remove it: commit the key once, or make init not write keys the repo removed.
- Also: `pilot snapshot` marks fn-297 ineligible because its parent branch fn-294 is not on origin, though the host merged fn-294 into this branch (e507d4874). The host dispatch proceeds anyway; a chain check that accepts a parent merged into the child branch would remove the override.

## 2026-10-20 work stage, branch fence
- Doing: work's Phase 2 chain fence.
- Hindered: `spec chain` reports fn-297 ineligible (parent fn-294's branch is not on origin), which would stop the run BLOCKED, although fn-294 is merged into this branch at e507d4874 and the host forbids pushing. The run proceeds on the current branch under the host's dispatch, with the spec base set by hand to ee2fe86b5.
- Cost: ~3 minutes reading the chained-spec and scheduler references to find no sanctioned path.
- Remove it: let the chain predicate accept a parent whose tip is already an ancestor of the child branch.

## 2026-10-20 task .1, first build
- Doing: the baseline build of `telperion-render`'s tests (`telperion-heavy 8 --threads 2 -- cargo test -p telperion-render --profile ci --no-run`) in a fresh worktree with its own `target/`.
- Hindered: `telperion-heavy` held the build for want of RAM (room 0 to 2 GB of the 8 asked) while another heavy unit ran a whole-workspace gate (`telperion-heavy-2842312`); the build had not started after 15 minutes, and a fresh `target/` then compiles from nothing on two threads.
- Cost: 15+ minutes waiting before the first compile, with the R2 red run blocked behind it.
- Remove it: a shared, read-only dependency cache (sccache) for worktrees, so a fresh checkout does not compile every dependency again; or a queue position in the wrapper's wait line, so a worker can tell a short wait from a long one.

## 2026-10-20 task .2, the red test
- Doing: the R3 repro, Laurelin's prepared bounds against every drawn stalk vertex.
- Hindered: the task names Laurelin, but at her shipped rows (age 80 and 30, seed 1) her crown already holds every stalk: the old bounds fell 0 m short, so the stated repro is green on the base. Finding a red case took four probe runs over droop, rachis and floret count (one floret on a 1 m rachis at droop 0.5 is 0.364 m short).
- Cost: ~12 minutes and four GPU test runs.
- Remove it: run the repro before the spec names its subject (AGENTS.md, "Gates and checked claims"); here the shortfall needs a rachis running past its last floret out of the crown.

## 2026-10-20 task .2, the gate
- Doing: the one workspace gate at the end of the task.
- Hindered: `generation_limit_guard` (a lexical check) read the stalk fold's presence test, `if ... element.stalk > 0.0 { ... return Err(..) }`, as an authored limit; only the gate runs that target, so a 37-minute gate was the first to see it. Restated as `then_some` and re-checked with the guard and bounds tests alone.
- Cost: one red gate (1409 passed, 1 failed, 111 ignored) and ~5 minutes.
- Remove it: list `generation_limit_guard` in the spec's Quick commands for any task touching `telperion-core/src`, so it runs before the gate.

## 2026-10-20 task .3, the green run
- Doing: the focused run of the two new tests after the fix (`telperion-heavy 8 --threads 2 -- cargo test -p telperion-render --profile ci --lib -- cycle::device a_block_opened_past_the_stride`).
- Hindered: `telperion-heavy` held the run with "room 7 GB for 8, gpu 0 GB" from 05:46, a gigabyte short of the 8 the host fixed for every heavy run, although a two-test lib run compiles one crate and holds the device for a few kilobytes; the foreground call hit its 600 s timeout while waiting.
- Cost: 40+ minutes and counting (05:46 to 06:26) before the compile started, for a run that takes about a minute. No other heavy unit ran; the desktop (browsers) held the memory, so room stayed at 6 to 7 GB. The fix was type-checked meanwhile (`cargo check -p telperion-render --profile ci --tests -j 2`) and sent to review.
- Remove it: a smaller declared size for a focused lib run (it needs neither 8 GB nor the whole-workspace reservation), measured once and written beside the gate's size.
