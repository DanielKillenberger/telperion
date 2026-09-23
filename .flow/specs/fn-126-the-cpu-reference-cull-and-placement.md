# The CPU reference cull and placement, fast

## Conversation Evidence

> user: "does this not apply to the gpu path? why not? can capture anyway to improve test and reference implementation iguess?"
> user: "ok go"

## Goal & Context

<!-- Goal & Context: 30% [user], 70% [inferred] code reading -->

With leaves drawn on the GPU, the CPU placement and cull serve tests, the reference expander and consumers without a GPU. They are the slowest stages of the CPU build: at seed 1, spruce placement 3,845 ms (82.5% of its build) and cull 360 ms (fn-91 `CPU-CANDIDATE.md`); birch placement and cull 4.21 s together (`.flow/evidence/fn76/LOCAL.md`). Every test that builds a tree pays them. [inferred]

## Architecture & Data Models

Four changes, each expected byte-identical, screened in this order against a split profile. [inferred]

- **A. A per-leaf bound in the cull.** `foliage::cull` (`foliage.rs:253`) transforms every element vertex of a leaf until one lies within the shell, so a leaf deep in the crown pays for all its vertices, each costing two `powf_fixed` calls in `radius_at` and a 128-segment scan in `distance_to_profile` (`envelope.rs:221`). Every vertex lies within rho = |M| times the element's extent of the leaf's origin, and the distance to the profile moves by at most the distance moved. Reject a leaf when its origin is deeper than shell + rho + epsilon on both tests, accept it when even its farthest vertex is inside the shell, and run the vertex loop only between. The `radius_at` test needs a slope bound for the envelope; the profile scan can use a height-band index.
- **B. Cull while placing.** `place_impl` (`placement.rs:149`) reserves and fills the whole crown before `retain` drops most of it. Testing each leaf as it is pushed keeps order and random draws and lowers peak memory.
- **C. A forward cursor in station placement.** `place_run` (`station.rs:71`) walks back from the run's end for every station, O(stations x segments) per run. Stations arrive in increasing distance, so a cursor makes it linear.
- **E. A parallel cull on native.** The cull is independent per leaf; chunks joined in order give identical output. Serial in Wasm.

## Acceptance Criteria

- **R1:** `examples/generation_stages.rs` times placement and cull separately (fn-102 R4 adds this; reuse it), and the base split is recorded for the oak, spruce and birch at seeds 1 and 7 before any change. [inferred]
- **R2:** Each change keeps output byte-identical for every catalogue preset at seeds 1 and 7, and the existing pinned hashes pass without edits. [inferred]
- **R3:** Placement and cull medians of five runs are recorded on base and candidate for each change; a change that does not speed its stage by at least 5% on the preset it targets is reverted and reported. [inferred]
- **R4:** Native peak RSS is recorded on base and candidate; B shows its reduction. [inferred]
- **R5:** The wall time of `cargo test --profile ci --workspace --no-fail-fast` is recorded on base and candidate. [inferred]

## Boundaries

- The GPU shaders are untouched; the GPU path already parallelises these stages. [inferred]
- No output changes. [inferred]
- No full-forest capture. [CLAUDE.md]

## Decision Context

- Lower priority than growth speed and the GPU contract since the owner's decision of 2026-09-23 that the GPU is the drawn path. [user]
- The birch's "cull costs 3.1 s of a 3.3 s build" line in the fn-102 draft had no measurement; R1 supplies one. [inferred]
- Depends on fn-102. Captured from the fn-102 review, items A, B, C and E. [inferred]
