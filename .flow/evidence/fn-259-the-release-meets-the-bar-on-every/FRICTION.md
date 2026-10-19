# fn-259 friction

- 2026-10-15 22:25, the release fork, gate on the R7 format branch: `telperion-heavy 20` (fn-271's declared gate size) waits for 20 GB of room on a 31 GB desk where Chrome, Steam and several Claude sessions hold about 16 GB; room read 6-12 GB for the first 30 minutes. Cost: the gate cannot start while the owner's desktop is in use. What would remove it: a gate whose whole-preset tests fit a smaller cap (fn-271's "a smaller core-lib peak, later work"), or the gate run on a quiet machine or CI.
- 2026-10-15 22:24, the site bench: F4's field package grew the spruce at seed 1234 in 7.0 s and was killed at the 6 GB cap on the next seed (Node holding several Wasm instances' memory between releases). The site-call bench needs one Node process per tree, as fn-245's PROFILE did.

## 2026-10-19 03:20, plan-review cap at two rounds

- **Doing:** codex plan-review of the seven-task plan.
- **Slowed:** `review.maxIterations` is 2, so the third round was refused with `ESCALATE` while findings were shrinking (6, then 1, the one a refinement of a prior finding). The counter was reset by the host (`flowctl spec reset-review-rounds`, which takes no reason argument; the reason is recorded here and in the commit), as the owner allows when findings shrink.
- **Cost:** about 2 minutes and one refused dispatch.
- **Would have removed it:** a cap of 3-4 for plan reviews, or a `--reason` on `reset-review-rounds` so the reset carries its reason in flow state.
- **Also:** `flowctl codex plan-review` without `--base` fails with "cannot resolve review snapshot: main...HEAD" in a repo with no `main`; every call needs `--base`. Cost: one failed call.

## 2026-10-19 04:00, task 3, the row runner's isolation rule cannot hold on this desk

- **Doing:** writing `scripts/gpu-row.mjs`, which per the task starts a row "only with no other compute process on the card".
- **Slowed:** `nvidia-smi --query-compute-apps` always lists the desktop's GPU clients (Chrome's and Grok's gpu-process, 1Password, Battle.net, voxtype; type C+G, 9-239 MiB each), and nvidia-smi cannot tell them from a compute job (a wgpu probe is C+G too). Task 4 says graphics clients are recorded, not killed, so the rule as written never lets a row start. Which processes count as foreign is a design call; returned to the host with NEEDS_HUMAN.
- **Cost:** one stop and a round trip to the host; the probe and the pixel helper were finished first.
- **Would have removed it:** the isolation rule checked against `nvidia-smi`'s listing on this desk before the task was written.

## 2026-10-19 03:55, task 3, the dcg hook and a shell variable in a redirect

- **Doing:** writing the probe's JSON rows to `raw/` with the directory in a shell variable.
- **Slowed:** dcg blocks any redirect whose target is a variable; the command was rewritten with literal paths.
- **Cost:** about 1 minute. Local setup, reported and not specced.

## 2026-10-19 04:10, task 3, impl-review cap at two rounds

- **Doing:** codex impl-review of the scorecard tooling.
- **Slowed:** the third round was refused by `review.maxIterations` 2 while findings were shrinking (5, then 2, one of the two the host's rule (a) that the reviewer re-raises). Reset with `flowctl spec reset-review-rounds --task`, as the host's brief allows when findings shrink.
- **Cost:** about 1 minute and one refused call. The first round also lost its correctness draw to "model at capacity".
- **Would have removed it:** a cap of 3-4, or a host decision the reviewer reads, so an owner-decided exemption is not re-raised every round.

## 2026-10-19 05:16, task 4, the matched-age rows waited on a memory cap the desk never had

- **Doing:** the oak's matched-age diagnostics (F4 at 100), asked of `telperion-heavy` at 14 GB because fn-258 measured the oak at 100 peaking at 9.1 GB.
- **Slowed:** the desk offered 13 GB of room (31 GB, the owner's desktop holding about 15), so the job waited from 05:13 without ever being admitted. The runner was stopped and the cap lowered to 12 GB, which still covers the 9.1 GB peak; the cap kills a job over it and changes nothing measured.
- **Cost:** about 4 minutes of waiting and one restart of the resumable runner.
- **Would have removed it:** a runner that prints the room it needs against the room the desk has and refuses at once when the cap cannot fit, rather than waiting on a number that will not come.
