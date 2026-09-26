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
