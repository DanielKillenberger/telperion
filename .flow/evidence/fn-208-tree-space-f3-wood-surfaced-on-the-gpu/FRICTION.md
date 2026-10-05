# fn-208 friction

## 2026-10-05, design research: timing today's wood path

- **Doing:** measuring the wood pass's GPU time for the 80-year engine spruce and oak, and today's presets, at the hero view, the limb close-up and a 5 cm twig close-up.
- **Slowed by:** no runner times a chosen tree at chosen views.
  - `telperion_render::measure` exists and is sound: it gives timestamp pairs around the vegetation, selection and shadow passes, and a contention verdict.
  - But nothing calls it on an engine tree, or at a camera other than the hero pose. The stills runner renders and does not time.
- **Cost:** about 10 minutes and one release build. I wrote a scratch example (`f3_frame`, not committed; a copy is in `f3-frame-probe.rs`).
- **Would remove it:** a `--time` flag on the stills runner (`examples/space/still.rs`) that runs `measure` at each shot it renders, `View::Bare` and `View::Whole`.

## 2026-10-05, design research: shell guard on a loop of logs

- **Doing:** running the four timing runs in a loop, each to its own log.
- **Slowed by:** the dcg hook refuses a `>` redirect to a path built from a loop variable.
- **Cost:** about 1 minute, rewritten as one appended log.
- **Would remove it:** nothing in the repository; it is a local hook setting (fn-198 FRICTION.md records the same hook).

## 2026-10-05, step (i): the session's temp directory is full

- **Doing:** reading the wood shader and pipeline code for the shading level of detail.
- **Slowed by:** every Bash call returns no output: "disk quota is full" on `/tmp/claude-1000/.../tasks`.
  - The session's scratchpad holds 6.2 GB, mostly earlier rounds' directories: `ref4` 3.7 GB, `base` 0.9 GB, `b` 0.3 GB, `fn188` 0.2 GB. None of it is mine.
  - I may not remove files outside the worktree, so I work by redirecting every command into new files under this spec's ignored `raw/` and reading them back.
- **Cost:** about 5 minutes so far, and one extra tool call per command from here on.
- **Would remove it:** clearing the old scratchpad directories, or a session temp directory on a larger filesystem (`CLAUDE_CODE_TMPDIR`). A local setup problem, for the owner, not a spec.

## 2026-10-05, step 1: the gate's beech memory ceiling and the limit inventory

- **Doing:** running the workspace gate once after steps 1 and 2.
- **Slowed by:** two failures.
  - `fixed_beeches_pass_geometry_and_profile_gates_with_repeatable_varied_specimens` read a peak resident of 5.91 GB against its 5.76 GB process ceiling. It passed when rerun alone, in 22 s. fn-206 recorded the same flake.
  - `generation_limit_guard` flagged the curve's nine `min`, `max`, `clamp` and constant sites, which were added to `docs/generation-limits-inventory.json` with reasons. That one was mine and expected, but nothing ahead of the 8-minute gate would have told me.
- **Cost:** about 10 minutes: one gate run and two reruns.
- **Would remove it:** run `generation_limit_guard` from the crate's own test command, cheap and early. Make the beech's ceiling test refuse to run beside other heavy tests (a nextest test group), rather than fail under load.

## 2026-10-05, step (i): the temp quota comes and goes; eight samples refused late

- **Doing:** measuring the shading changes before and after.
- **Slowed by:**
  - The temp directory's quota filled again and again during the step. Every command's output went to files under `raw/` and was read back. Background jobs that wrote their log into the worktree ran fine.
  - The 8-sample runs panicked at pipeline creation. `Gpu::supports_samples` reported 8 as offered, but wgpu needs `TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES` for it. That cost one wasted run of three trees.
- **Cost:** about 15 minutes in all.
- **Would remove it:**
  - The quota is local (see the entry above).
  - `supports_samples` should check what the device was given, not the adapter; a small fix to `device.rs`.

## 2026-10-05, the `supports_samples` fix had no red test

- **Doing:** fixing `Gpu::supports_samples` (host decision 12).
- **Slowed by:** the defect lived inside a method that needs a device, so nothing could test it before the fix. The test came with the extracted function (`sample_count_granted`), so it was never shown red against the old code.
- **Cost:** none in time; one red-first proof missing.
- **Would remove it:** extracting the decision from the device first, then writing the test red against the old rule, then fixing it.

## 2026-10-05, (iv): the curve compute costs latency, not work

- **Doing:** measuring the GPU passes' frame cost.
- **Slowed by:** the vegetation pass's GPU timer does not cover the new compute passes, so their cost showed only in wall time. Bisecting it took environment switches in a scratch build: about 20 minutes and four runs.
- **Would remove it:** a fourth timestamp pair around the curve's compute passes in `timing.rs`.
