# fn-287 friction

## 2026-10-16 - fn-287.1: the oracle's last dead item lies outside the task's Touches

- Doing: moving the comparison route (`begin_positions`, `complete_positions`, `emit_positions`, `prepare_whole`'s resident branch, `upload_wood`/`UploadedWood`) under `cfg(test)`.
- Hindered by: the only remaining new warning is `buffer::Lent::Borrowed` "never constructed" in the non-test build. The oracle was its only constructor; `buffer.rs` (the variant and its `Deref` arm) and `generation/compute.rs` (`compute_buffer_async`'s `matches!(ring_buffer, Lent::Borrowed(_))`) are not in the task's declared Touches, and the dispatch forbids edits outside them. The spec asks that such an item follow the oracle under `cfg(test)` with no new warning.
- Cost: one return to the conductor (about 10 minutes of build and test already spent are kept, committed as a coherent partial).
- Would remove it: planning Touches from a non-test build with the oracle stubbed out (the compiler names every item the cut orphans), or listing `buffer.rs` and `generation/compute.rs` in fn-287.1's Touches.

## 2026-10-16 - fn-287.1: gating `Lent::Borrowed` alone does not compile

- Doing: the conductor-approved fix, `cfg(test)` on `Lent::Borrowed`, its `Deref` arm and the `matches!` in `compute_buffer_async`.
- Hindered by: without the variant, `Lent<'a>` no longer uses its lifetime (E0392), so the gate the plan and the return described cannot build. `Lent` was removed instead: `compute_buffer_async` takes any `Deref<Target = wgpu::Buffer>` rings and the lent bytes explicitly (0 from `compute_async`, the buffer's size from the oracle and the tests), the same values the enum gave.
- Cost: one check build, about 2 minutes.
- Would remove it: checking a proposed `cfg` cut with `cargo check` before writing it into a return.
