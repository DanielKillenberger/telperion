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
Bounded leaf readback uses one fallibly reserved final vector and <=4 MiB staging shared by CPU output and explicit verification. Exact word tests and specimen hashes pass; matched spruce warm CPU-output p50 improves 351.73→297.03 ms, oak 259.27→255.65 ms. Full evidence, allocation lifetimes, cold regression and RSS limits: `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/READBACK.md`.

baseline: green (seven generation tests, Wasm check, scoped formatting).
Final gates: ten generation tests, native example build, Wasm check and scoped formatting passed. Compile-red for the new API is recorded without claiming behavioral red. Host inspected the diff and found no blocker. Parent .1/spec remain open; parent R5 is not qualified.

Tier: session (jev moderate 0.24) (explicit IMPLEMENTER preserved).
stage: impl-review - skipped(config: REVIEW_MODE=none)

Included host-authorized owner visual evidence, wood-boundary note and task .3 lifecycle files in the checkpoint. Friction: candidate build queued behind the test compile for under one minute; see FRICTION.md.
## Evidence
- Commits: 6d5841d78bc39b0b6e7cb34829889f7ec96061ec
- Tests: baseline: green (7 native generation tests, Wasm check, scoped rustfmt), /tmp/telperion-fn91-tools/cargo-nextest nextest run --release -p telperion-render --lib -E test(generation::) — 10 passed, cargo check -p telperion-render --target wasm32-unknown-unknown — passed, cargo build --release -p telperion-render --example generation_gpu — baseline and candidate passed, rustfmt --edition 2021 --check crates/telperion-render/src/generation.rs crates/telperion-render/src/generation/io.rs crates/telperion-render/src/generation/readback_tests.rs crates/telperion-render/examples/generation_gpu.rs — passed, python3 .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/readback-measure.py — four fresh-process measurements plus four separate repeated hash checks passed, git diff --check — passed
- PRs: