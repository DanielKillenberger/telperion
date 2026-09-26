# fn-158 friction

## 2026-09-26: no tool compared a preset's artifacts before and after a refactor

- **Doing:** recording what every shipped preset builds on the base, so R3's byte identity could be checked after the refactor.
- **Slowed by:** no existing tool digests the pipeline's artifacts, the GPU executor's output at both deliveries and the growth path's mesh together; `generation_gpu` hashes only the GPU leaves, and the identity pins in `tests/identity.rs` cover a subset. The dcg hook also refused a redirect into a path held in a shell variable.
- **Cost:** about 10 minutes writing `crates/telperion-render/examples/generation_digest.rs` and one retry with literal paths.
- **Would have removed it:** a standing digest example for behaviour-preserving refactors (this spec adds one).
