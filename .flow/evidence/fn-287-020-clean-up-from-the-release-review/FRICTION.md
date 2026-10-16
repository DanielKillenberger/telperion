# fn-287 friction

## 2026-10-16 - fn-287.1: the oracle's last dead item lies outside the task's Touches

- Doing: moving the comparison route (`begin_positions`, `complete_positions`, `emit_positions`, `prepare_whole`'s resident branch, `upload_wood`/`UploadedWood`) under `cfg(test)`.
- Hindered by: the only remaining new warning is `buffer::Lent::Borrowed` "never constructed" in the non-test build. The oracle was its only constructor; `buffer.rs` (the variant and its `Deref` arm) and `generation/compute.rs` (`compute_buffer_async`'s `matches!(ring_buffer, Lent::Borrowed(_))`) are not in the task's declared Touches, and the dispatch forbids edits outside them. The spec asks that such an item follow the oracle under `cfg(test)` with no new warning.
- Cost: one return to the conductor (about 10 minutes of build and test already spent are kept, committed as a coherent partial).
- Would remove it: planning Touches from a non-test build with the oracle stubbed out (the compiler names every item the cut orphans), or listing `buffer.rs` and `generation/compute.rs` in fn-287.1's Touches.
