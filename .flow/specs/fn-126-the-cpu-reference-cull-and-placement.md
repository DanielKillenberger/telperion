# The CPU reference cull and placement, fast

## Conversation Evidence

> user: "does this not apply to the gpu path? why not? can capture anyway to improve test and reference implementation iguess?"
> user: "ok go"
> user (2026-09-24, on rewriting this spec to follow fn-125): "ok"

## Goal & Context

<!-- Goal & Context: 30% [user], 70% [inferred] code reading and fn-125's feasibility check -->

After fn-125, every tree goes through one expansion with two executors (STRATEGY.md, amended 2026-09-24). The CPU reference executor defines correct, serves consumers without a GPU, and runs in every test that builds a tree. Today the same work is the slowest part of the CPU build. At seed 1, spruce placement takes 3,845 ms (82.5% of its build) and its cull 360 ms (fn-91 `CPU-CANDIDATE.md`). Birch placement and cull take 4.21 s together (`.flow/evidence/fn76/LOCAL.md`). This spec makes the reference executor fast once fn-125 has defined it. [inferred]

The first draft targeted today's CPU placed path. fn-125 deletes that path (`placement.rs:133` `place_on`) and rewrites station placement (`station.rs:70` `place_run`) as station sources, so the draft's changes B (cull while placing) and C (a forward cursor in `place_run`) have no target left. Rewritten 2026-09-24 to depend on fn-125. [user]

## Architecture & Data Models

The work is profile first, then fix in ranked order. [inferred]

1. **Profile the reference executor after fn-125.** Split it by stage: station sources, clumping reduction, cull and compaction. Record it for the oak, spruce, birch and beech at seeds 1 and 7. No candidate is chosen before the profile names where the time goes.
2. **Candidates carried from the first draft, each screened against the profile:**
   - **A. A per-leaf bound in the cull.** The cull (`foliage.rs:233` today) transforms every element vertex of a leaf until one lies within the shell. A leaf deep in the crown pays for all its vertices, and each costs two `powf_fixed` calls in `radius_at` and a 128-segment scan in `distance_to_profile` (`envelope.rs:221`). Every vertex lies within rho = |M| times the element's extent of the leaf's origin, and the distance to the profile moves by at most the distance moved. So: reject a leaf when its origin is deeper than shell + rho + epsilon on both tests, accept it when even its farthest vertex is inside, and run the vertex loop only between. The `radius_at` test needs a slope bound for the envelope, and the profile scan can use a height-band index. The same bound may serve the GPU cull (`place.wgsl:32-43`); if it does, both executors take it together so they stay one algorithm.
   - **E. A parallel reference executor on native.** Station sources and the cull are independent per leaf, and chunks joined in order give identical output. It runs serially in Wasm.
3. **New candidates** come from the profile, not from this list.

## Acceptance Criteria

- **R1:** A committed profile of the reference executor, split by stage, for the oak, spruce, birch and beech at seeds 1 and 7, taken on fn-125's merged code before any change. [inferred]
- **R2:** Each change keeps the reference executor's output byte-identical to fn-125's for every catalogue and in-work preset at seeds 1 and 7. A change shared with the GPU keeps the two executors within fn-125's tolerance. [inferred]
- **R3:** Stage medians of five runs are recorded on base and candidate for each change. A change that does not speed its stage by at least 5% on the preset it targets is reverted and reported. [inferred]
- **R4:** Native peak RSS and the wall time of `cargo test --profile ci --workspace --no-fail-fast` are recorded on base and candidate. [inferred]

## Boundaries

- No change to the algorithm's output. Speed only. [inferred]
- Growth speed is fn-124; the GPU expansion's own speed is fn-125's R4. [inferred]
- No full-forest capture. [CLAUDE.md]

## Decision Context

- Depends on fn-125, which defines the reference executor this spec speeds up. [user]
- The first draft's B and C are dropped because fn-125 deletes their code. A and E carry over. [inferred]
- The first draft's line references (`place_impl`, `foliage.rs:253`) predated renames: `place_impl` is now `place_on`, and the cull starts at `foliage.rs:233`. [inferred, checked 2026-09-24]
- Lower priority than growth speed and the GPU contract since the owner's decision of 2026-09-23 that the GPU is the drawn path. [user]
