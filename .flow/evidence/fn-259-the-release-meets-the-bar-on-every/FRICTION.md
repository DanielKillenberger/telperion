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

## 2026-10-19 06:58, task 5, a refused row keeps no error

- **Doing:** reading the matched-age refusals (42 rows) into the scorecard.
- **Slowed:**
  - `gpu-row.mjs` writes `probe: null` and `exit: 1` for a probe that fails, and the error itself goes only to stderr. Which preset refused with which error had to be read from the runner's log in job order.
  - The runner also needed a hand-written 30-minute admission watchdog, because `gpu-row.mjs` has no deadline (task 4's follow-up).
- **Cost:** about 5 minutes for the log mapping and about 5 for the watchdog. The sweep itself ran 85 minutes with no stop.
- **Would have removed it:**
  - `gpu-row.mjs` keeping the probe's last stderr line in the row when it prints no JSON;
  - an admission deadline in `gpu-row.mjs` itself.

## 2026-10-19 07:40, task 6, the audit's names had to be pinned by hand

- **Doing:** writing the name list for R2's audit (contract A, the stepped light executor, the fine-root cut, the lattice, the replaced growers) before searching the shipped modules.
- **Slowed:**
  - The task's investigation target for "the lattice that stays" is `pipeline/branching/lattice.rs`, the leaf bases' spiral cells. The decision it cites (fn-237, "Astra's release review, incorporated") means the light lattice (`telperion_space::light`, `cycle/light*.wgsl`). Both had to be traced to be sure which one the decision meant.
  - The shipped shader inventories (`shaders::all()` and `generation::sources::all()`) are `#[cfg(test)]`, so the WGSL a module embeds had to be read by matching every `.wgsl` file's text against the Wasm's bytes.
  - The main module's name section is v0-mangled (the renderer's is demangled by wasm-bindgen), so the names were read through `twiggy` rather than a plain name-section dump.
  - A redirect to a shell-variable path was refused by the dcg hook; the commands were re-run with literal paths.
- **Cost:** about 15 minutes in all.
- **Would have removed it:**
  - the plan naming each replaced thing by its symbol and the commit that deleted it;
  - a non-test list of the shipped WGSL sources.

## 2026-10-19 07:21, task 7, the sheet tool the task names is not a sheet tool

- **Doing:** composing the old-versus-new sheets, which the task says to make "with `scripts/contact-sheet.py`".
- **Slowed:** `contact-sheet.py` is the isolated Claude reviewer over a sheet (an envelope, hashes, a model call), not a compositor; no repository script tiles stills into a labelled sheet. The sheets were made with ImageMagick `montage` from a scratch script (`raw/t7/t7-sheets.sh`), and the 0.1.4 stills needed their own adapter (`raw/t7/v014-resident_stills.rs`), because task 4's `v014-probe.patch` carries only the frame and field probes.
- **Cost:** about 5 minutes; the scratch scripts are ignored, so the next old-versus-new sheet rebuilds them.
- **Would have removed it:** a committed sheet compositor (stills in, one labelled JPEG out), and the 0.1.4 stills adapter kept beside `v014-probe.patch`.

## 2026-10-19 10:20, CPU-peak probe, the 0.1.4 scratch tree was gone

- **Doing:** instrumenting the 0.1.4 `frames` route at matching stages.
- **Slowed:** the scratch export of `v0.1.4` that task 4 built from no longer existed (only its target directory did), so it was re-exported with `git archive v0.1.4` and `v014-probe.patch` reapplied. `gpu-row.mjs` also takes only a probe that prints a JSON row on stdout, so the stage probe ran under `telperion-heavy` alone, without the foreign-process check.
- **Cost:** about 3 minutes.
- **Would have removed it:** a script beside `v014-probe.patch` that recreates the 0.1.4 probe tree in one step.
