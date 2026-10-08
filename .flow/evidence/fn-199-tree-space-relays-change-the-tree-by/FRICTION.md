# fn-199 friction

## 2026-10-07 12:24, the GPU lock held by a workspace gate

- **Doing:** R4's GPU parity check, `generation::fine` under the shared GPU lock, started 12:10.
- **Slowed by:** another session's `cargo test --profile ci --workspace --no-fail-fast` held the lock from 12:06, with headless stills and an fn-212 baseline queued behind it. The whole workspace gate runs under the GPU lock, though only the render crate's GPU tests need it.
- **Cost:** 15 minutes of waiting at the first try, and a rerun when `flock -w 900` gave up.
- **Would remove it:** the gate taking the lock only around the GPU test binaries (or a second lock for the GPU tests alone), so CPU-only suites never queue GPU work.

## 2026-10-07 12:24, the Codex findings are not on the branch

- **Doing:** reading the four findings the spec cites (`.flow/review-fanout/33e47f112c61497c9b41dddd668e36c5/`).
- **Slowed by:** the directory exists on no branch or checkout; only the receipt's finding ids sit in fn-193's spec JSON. fn-206's RESULT.md ("fn-199's four relay-continuity findings") and fn-202's RESULT.md carried enough to rebuild each repro.
- **Cost:** about 5 minutes.
- **Would remove it:** a spec that cites review findings copies their worked examples into the spec or its evidence, since the fan-out directory is ignored.

## 2026-10-07 12:40, /tmp is a shared 16 GB tmpfs

- **Doing:** base digests and a `contract dump` of the base (e9bb5b08), built from a `git archive` copy in the scratchpad with its own `target/`.
- **Slowed by:** the scratchpad sits on `/tmp`, a 16 GB tmpfs shared by every session. The base copy's `target/` and the dump filled the quota; the dump, the first GPU test run's output and its lock wait were lost, and other sessions writing to `/tmp` may have failed in that window. Both removed at once.
- **Cost:** one GPU run lost, about 15 minutes.
- **Would remove it:** a base checkout and its `target/` under the worktree's ignored `raw/` (home disk), never under the scratchpad; the guidance for agents to say so.

## 2026-10-07 13:35, the GPU lock again

- **Doing:** R1's GPU parity (`generation::fine`) on the work-in-progress.
- **Slowed by:** four sessions queued on the one GPU lock (a workspace gate among them); `flock -w 1200` timed out without the run starting.
- **Cost:** 20 minutes, and R1's shader edits stay unchecked at the timebox.
- **Would remove it:** as the first entry: the lock taken around GPU work only, not whole gates.

## 2026-10-07 14:00, /tmp's quota full, from other sessions

- **Doing:** the closed form for R1.
- **Slowed by:** `/tmp` is a 16 GB tmpfs, with 13 GB used, mostly by other sessions' scratch directories under this host's shared scratchpad (`out` 2.3 GB, `t5` 1.8 GB, `topo` 1.3 GB, `before` and `after` 1.2 GB each). The user's quota there is full, so every shell call errors on its working-directory write, and a background command's output file can be lost. This session holds only a few KB there now.
- **Cost:** a background test's output lost; every command since then reports an error.
- **Would remove it:** per-session scratch directories cleaned when their session ends, or the scratchpad on the home disk. A local setup problem: reported, not specced.

## 2026-10-07 evening, resuming after the crash: the walk suite's length

- **Doing:** checking R1's closed form against the walk tests after resuming the crashed worker's tree.
- **Slowed by:** `tests/walks.rs` runs 17 minutes on 8 threads, and one failing walk (`branches_are_made_and_unmade_at_vanishing_size`) can only be seen at its end; the crate's other binaries take under 3 minutes together.
- **Cost:** 17 minutes of wall time per check of the walks.
- **Would remove it:** a filter that runs one walk test, or the walks binary split so the vanishing-size check runs alone (`--test walks branches_are_made` works, but the suite's shared `walks(&all)` recomputes every setting's walk).

## 2026-10-07 evening, a measurement that grew without bound

- **Doing:** dumping the closed form's rows at 1, 2, 4 and 8 quadrature points to see how fast they converge.
- **Slowed by:** the rows took 2.5 s at one point and 22.5 s at two; at four they ran past 15 minutes and were stopped. Nothing warned that the nested relays' tables multiply with the points.
- **Cost:** about 20 minutes.
- **Would remove it:** time one point count on one species before looping over several (done afterwards with the predictor timing).

## 2026-10-07 late, a bisect that did not rebuild

- **Doing:** finding which commit turned R1's walk red, each commit extracted with `git archive | tar -x` over one copy with its own `target/`.
- **Slowed by:** `tar` keeps the archive's mtimes (the commit's time), older than the built artifacts, so cargo rebuilt nothing and every commit "failed" with the first one's numbers.
- **Cost:** about 15 minutes and a wrong first reading (that the crashed worker's tree was already red).
- **Would remove it:** `tar -x -m` (or `touch`) on every extraction; worth a line in the agent guidance next to "two checkouts never share a target".

## 2026-10-07 late, a gate failure that predates the branch

- **Doing:** the workspace half of the gate.
- **Slowed by:** `generation_limit_guard` fails on the base (`e9bb5b08`) as well. A red test inherited from the base hides whether a branch broke it, and here took a base run to rule out.
- **Cost:** about 5 minutes.
- **Would remove it:** the base's inventory fixed on master. Reported here, not fixed in fn-199, whose boundaries are engine continuity.
