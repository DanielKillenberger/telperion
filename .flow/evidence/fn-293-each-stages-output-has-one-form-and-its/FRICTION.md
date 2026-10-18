# fn-293 friction

## 2026-10-18 - `flow --auto` cannot select the spec

- **Doing:** starting `flow-next:flow fn-293 --auto` in the new worktree `.worktrees/f4-293`.
- **Slowed by:** the pilot snapshot reports the spec `eligible: false`, route `host` ("spec not ready"). The ready flag is a person's consent gate (`flowctl spec ready`), and a dispatched agent may not set it on another agent's word. The run drove the same stages directly (plan, plan-review, work) instead of through the auto driver's selector.
- **Also:** `flowctl init --json` rewrote `.flow/config.json` (added `review.fanoutExecution`), and every preflight prints a note about two removed keys (`artifacts.html.enabled`, `pipeline.chainStages`). The edit was reverted by hand.
- **Cost:** about 5 minutes and three extra flowctl calls.
- **Would have removed it:** the owner marking a spec ready when handing it to a host for an unattended run, or the dispatch carrying the owner's ready instruction; removing the two dead keys from `.flow/config.json`.

## 2026-10-18 - task 1: the resident digest cannot read the curve's sections inside the declared Touches

- **Doing:** writing the resident digest test (`resident/digest_tests.rs`), which hashes `Prepared.wood`'s point, cluster and section buffers.
- **Slowed by:** `CurveGpu.sections` is private to `curve.rs` and had no test accessor, and the CPU-uploaded curve blocks (`curve.rs`, `filled`) were made `STORAGE` only, so a copy out failed wgpu validation (`COPY_SRC` missing). Neither file is in the task's Touches; the fix is a `#[cfg(test)]` accessor and `COPY_SRC` on the uploaded blocks, as the device's own handoff blocks already carry.
- **Cost:** about 10 minutes and one failed GPU run.
- **Would have removed it:** the plan checking each buffer the digest reads for a reader and a copy usage, and listing `crates/telperion-render/src/curve.rs` in the task's Touches.

## 2026-10-18 - task 1: a GPU declaration one gigabyte too large queued the resident digest for half an hour

- **Doing:** re-running the resident digest after the task's edits under `telperion-heavy 10 --gpu 7`.
- **Slowed by:** the wrapper admits a GPU job only when free VRAM covers the declaration plus 1 GB; with the desktop's browsers holding 2 GB of the 10 GB card, 7 + 1 never fitted, and the wrapper printed `waiting (room 15 GB ...)` without saying the GPU was the blocker. The same command had run at once an hour earlier with less on the card.
- **Cost:** about 35 minutes of wall time waiting; the run itself takes 15 seconds under `--gpu 6`.
- **Would have removed it:** the wrapper naming which condition holds a job (RAM or VRAM, with the free VRAM it saw), and the resident digest's declared VRAM measured once and written beside its command.

## 2026-10-18 - task 2: the render pipeline digest outlives a foreground call

- **Doing:** re-running `generation_digest` (every preset at seeds 1, 7, 42) after the space crate's deletions, to compare with the baseline.
- **Slowed by:** the run takes over ten minutes on the CPU, past the 600 s ceiling of one foreground tool call, so the harness moved it to the background mid-run; the digest's runtime is written nowhere beside its command in BASELINE.md.
- **Cost:** about 12 minutes of wall time, and one extra wait on a background handle.
- **Would have removed it:** BASELINE.md naming each digest's wall time, so a worker launches the long one in the background from the start; or a `DIGEST_PRESETS` filter to compare the presets a task can move first.
