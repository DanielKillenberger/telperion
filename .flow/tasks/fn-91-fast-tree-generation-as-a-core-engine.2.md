---
satisfies: [R2, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.2 Bound GPU leaf readback memory without changing output bytes

## Description
Continue the fn-91 checkpoint after the owner explicitly asked to keep going. This is a new bounded task under the unchanged parent acceptance; task .1 remains open and its exhausted budget is not reset.

Replace full-size GPU leaf readback staging plus intermediate byte Vec with a reusable bounded staging buffer and direct decoding into the final owned Vec<[u32; 3]>. Preserve exact GPU-produced words and ordering. Both CPU delivery and explicit resident verification readback use the shared helper. Do not change the compute shaders, placement, mass construction, renderer delivery or default backend. The staging chunk is at most 4 MiB (round down to a multiple of 12 bytes, also respecting device limits). The final result is reserved once with fallible allocation; no partial output escapes on allocation, mapping, cancellation or device errors. Zero leaves does no GPU work. Copy/map/unmap sequential chunks; retain browser-compatible asynchronous completion. Native wrappers continue using the same implementation.

**Touches:** crates/telperion-render/src/generation.rs, crates/telperion-render/src/generation/io.rs, crates/telperion-render/src/generation/*tests.rs, crates/telperion-render/examples/generation_gpu.rs, scripts/benchmarks/generation.md, .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/**

Use the existing baseline executable/protocol where provenance matches; otherwise preserve a focused baseline build before editing. Compare native oak/spruce seed 1 CPU-output GPU mode in fresh processes, same samples and hardware, including elapsed time and wait4 peak RSS. Account final output and bounded staging explicitly without equating that with whole driver/process/device memory. Do not run full unrelated workspace gates, repeated LTO builds or screenshots. Host separately inspects wood costs without competing timed work. Record friction as it happens. Do not alter parent targets or mark .1/spec complete.

Quick commands: native renderer library generation tests via the existing isolated nextest runner and explicit --lib build target; cargo check -p telperion-render --target wasm32-unknown-unknown; scoped rustfmt. Add a focused GPU readback test for zero, one, exact-chunk and multichunk/tail results, checked against known u32 words, using a small internal test chunk limit to avoid huge fixtures. Existing GPU geometry/ownership tests remain required. The host reviews the diff directly (review.backend=none).
## Acceptance
- Leaf readback has one final owned leaf vector, no full-size intermediate byte vector, and staging bounded to at most 4 MiB. Byte ordering and values match the current GPU result exactly.
- Zero and chunk-boundary/tail behavior are verified; allocation/map/device failure returns an error rather than a partial tree. Browser code has no native blocking poll or channel receive.
- Existing focused generation tests, native/Wasm compile checks and scoped formatting pass.
- Fresh-process before/after native oak and spruce seed-1 CPU-output measurements retain timing, peak RSS and explicit allocation limits. Report any latency or total-memory regression honestly; parent R5 remains open until all its own requirements pass.


## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
