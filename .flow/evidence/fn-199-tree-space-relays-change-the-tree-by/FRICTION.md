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
