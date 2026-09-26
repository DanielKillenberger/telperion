# fn-158 friction

## 2026-09-26: no tool compared a preset's artifacts before and after a refactor

- **Doing:** recording what every shipped preset builds on the base, so R3's byte identity could be checked after the refactor.
- **Slowed by:** no existing tool digests the pipeline's artifacts, the GPU executor's output at both deliveries and the growth path's mesh together; `generation_gpu` hashes only the GPU leaves, and the identity pins in `tests/identity.rs` cover a subset. The dcg hook also refused a redirect into a path held in a shell variable.
- **Cost:** about 10 minutes writing `crates/telperion-render/examples/generation_digest.rs` and one retry with literal paths.
- **Would have removed it:** a standing digest example for behaviour-preserving refactors (this spec adds one).

## 2026-09-26: timings taken beside another checkout's test run

- **Doing:** R4's release timings, 8 presets by 3 executors by 5 runs.
- **Slowed by:** a test binary from another checkout (`field_plan`, load average 7) ran during the first pass; its spruce resident median read +22% and its maxima reached 6.9 s. Nothing on the machine says a timing window is shared.
- **Cost:** a discarded 5.5-minute pass and an 11-minute interleaved rerun (base and head binaries alternating, order flipped each run).
- **Would have removed it:** a lock the timing and gate scripts take on the owner's machine, or a timing recipe that interleaves base and head by default (the one used here is `scratchpad/abtimings.sh`, worth promoting to `scripts/`).

## 2026-09-26: the visibility step needs host decisions the spec does not settle

- **Doing:** step 3 of the dispatch, making the stages private after the executor interface landed.
- **Slowed by:** the render crate's GPU tests build fixture trees and custom leaf boxes and call core stages on them; seven render integration tests build elements with `build_element`; eight core examples (`species_measure` among them) run their own stage chains. Making the stages private forces a choice between widening the interface and rewriting those callers, which AGENTS.md sends to the host.
- **Cost:** the task stops at NEEDS_HUMAN with the interface and typed inputs in, before the visibility migration.
- **Would have removed it:** the spec naming what the executor interface admits for tests and tools (a caller's own solved tree, an element from its rows) and what happens to evidence examples that chain stages.

## 2026-09-26: the species memory ceiling assumes the suite has its own process

- **Doing:** the workspace gate after the core's integration tests moved into its own test binary.
- **Slowed by:** `suite::species`'s ceiling reads the process's VmHWM and holds it to a budget sized for a binary of species tests alone. Under `cargo test` every suite test now shares that process, so the ceiling reads 21-25 GB against 11.5 GB; under nextest (CI) each test is its own process and the ceiling holds. The saturation run also named its test by its old integration-test path and selected nothing.
- **Cost:** a full gate run (about 6 minutes) and a second run of the species suite alone.
- **Would have removed it:** the ceiling reading the budget's own charges, or nextest as the local gate as it is in CI.

## 2026-09-26: the rebase onto master met a new stage caller

- **Doing:** rebasing onto master after fn-152 merged.
- **Slowed by:** #121's species runner called `branching::generate` for the palm's pins, which the boundary closes; the fix moved the palm's skeleton pin. The gate had to run a second time on the rebased code.
- **Cost:** about 15 minutes: the reroute, the base digests and example captures redone at the new base, and the second gate run.
- **Would have removed it:** the boundary landing before new consumers were written against the stages; nothing in this task could have avoided it.
