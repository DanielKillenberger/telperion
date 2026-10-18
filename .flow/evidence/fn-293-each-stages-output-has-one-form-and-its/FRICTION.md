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

## 2026-10-18 - task 2's impl-review hit the two-round cap while converging

- **Doing:** task 2's codex impl-review loop (unattended, until SHIP).
- **Slowed by:** `review.maxIterations` is 2. Round 1 had two findings, round 2 one (both fixed, `08f23f9e0`, `067d5ea6a`); the re-review of the last fix was refused with `ESCALATE: impl-review round cap reached (2/2)`. The worker returned without `done`.
- **Host decision (recorded reason):** the review was converging (2, then 1, then 0 pending findings), not looping, so the host reset task 2's counter (`flowctl spec reset-review-rounds --task ...2`) and re-ran the review, as the owner allows the host to judge.
- **Cost:** one worker return and a conductor-run review, about 10 minutes.
- **Would have removed it:** a cap of three or more rounds for multi-finding tasks, or a cap that counts rounds without a decreasing finding count.

## 2026-10-18 - task 3: `cfg(test)` cannot reach what integration tests and examples read

- **Doing:** moving the scaffold's modules and `blend`'s size match under `cfg(test)`, as the task's approach says.
- **Slowed by:** an item gated `cfg(test)` is invisible to `tests/*.rs`, `examples/` and other crates' tests, which link the non-test library. The audit's finding 15 ("`blend::` reached only by `catalogue/walk.rs`, examples, suite", inventory only) missed eight core integration tests, two render examples and two render lib tests that call `blend::families` or `blend::centroid`; and `tests/migration/foundation.rs` (an integration test) reads the scaffold's `GrowthBias`. Gating either breaks the build, and gating only `blend/size.rs` would change what `blend::families` returns. Neither has a production reader.
- **Cost:** about 20 minutes of tracing readers, and both items left compiled pending the host's call.
- **Host decision (conductor, 2026-10-18):** `blend` (`families`, `centroid`, `size.rs`) and `GrowthBias` stay in the production build, kept for their named readers outside the lib's test build: `blend` for the core integration tests `catalogue_identity`, `space_presets`, `units_bound`, `space_walks`, `walk_sizes`, `walk_leaves`, `material_detail` and `walk_law`, the core examples `centroid`, `walk_size` and `walk_leaves`, the render examples `headless` and `space_walk` and the render lib tests `cpu_builds_tests` and `resident/organ_tests`; `GrowthBias` for `tests/migration/foundation.rs`. No cargo feature and no test moved here, as task 2 kept `invariants`; fn-288 R1 classifies those tests.
- **Would have removed it:** the audit counting integration tests, examples and other crates' tests as readers (a `cargo check --workspace --all-targets` after a trial gate), and the plan naming where code read only by them goes (moved into the suite, behind a feature, or kept).

## 2026-10-18 - task 3 stops: dropping the scaffold's row checks unbounds 39 wire rows

- **Doing:** `validate_skeleton` reduced to the checks whose rows a production reader needs (envelope, radii, twigs), the task's reading of R2.
- **Slowed by:** the scaffold's checks are also the only bounds on 39 wire rows (`/skeleton/attractors`, `samplingAttemptsPerAttractor`, `step`, every `/skeleton/habit/*` row but none, `/skeleton/bias/*`, `/skeleton/growth/*` but `maxNodes`). fn-123 R1 ("`Family::validate` refuses every value ... for every row") is pinned by `family::tests::every_row_past_its_bound_is_refused_on_every_family`, `suite::troll_limbs` (three wire/build refusal tests), `tests/catalogue_identity.rs::every_refusal_is_the_refusal_it_was`, and fn-113's dial walls in `telperion-jev/tests/dial_bounds.rs` judge dials by the same call. Which contract wins is a host call.
- **Cost:** one focused core test run (about 7 minutes) and the task stopped before commit; the diff is kept in `raw/t3-wip.patch`.
- **Would have removed it:** the plan checking fn-123 R1 against "keep only checks production rows need" before the task was written.
- **Host decision (conductor, 2026-10-18):** `validate_skeleton` and `Family::validate` stay exactly as on the base, every check kept (rows, `inner_envelope`, `resolved_growth`, `forks_placed`), so what they call (`specimen::rows`, `scaffold/fork.rs`'s `placed`, `slot`, `siblings`) stays compiled. Which rows validation bounds once the scaffold leaves production is an open item for the owner. `hold_tests::a_hold_adds_or_loses_no_leaf` is deleted with `Tree.pipe`: it pinned the pipe's effect on the scaffold's held trees, which no production tree has (`girthHold` is read by no stage).
