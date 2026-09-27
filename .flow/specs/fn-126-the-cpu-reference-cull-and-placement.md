# The CPU reference cull and placement, fast

## Conversation Evidence

> user: "does this not apply to the gpu path? why not? can capture anyway to improve test and reference implementation iguess?"
> user: "ok go"
> user (2026-09-24, on rewriting this spec to follow fn-125): "ok"
> user (2026-09-27, on keeping this spec to the CPU reference and giving GPU ring preparation its own spec): "Two specs"

## Goal & Context

<!-- Goal & Context: 30% [user], 70% [inferred] code reading and fn-125's feasibility check -->

After fn-125, every tree goes through one expansion with two executors (STRATEGY.md, amended 2026-09-24). The CPU reference executor defines correct, serves consumers without a GPU, and runs in every test that builds a tree. Today the same work is the slowest part of the CPU build. At seed 1, spruce placement takes 3,845 ms (82.5% of its build) and its cull 360 ms (fn-91 `CPU-CANDIDATE.md`). Birch placement and cull take 4.21 s together (`.flow/evidence/fn76/LOCAL.md`). This spec makes the reference executor fast once fn-125 has defined it. [inferred]

The first draft targeted today's CPU placed path. fn-125 deletes that path (`placement.rs:133` `place_on`) and rewrites station placement (`station.rs:70` `place_run`) as station sources, so the draft's changes B (cull while placing) and C (a forward cursor in `place_run`) have no target left. Rewritten 2026-09-24 to depend on fn-125. [user]

## Architecture & Data Models

The work is profile first, then fix in ranked order. [inferred]

1. **Profile the reference executor after fn-125.** Split it by stage: station sources, clumping reduction, cull and compaction. Record it for the oak, spruce, birch and beech at seeds 1 and 7. No candidate is chosen before the profile names where the time goes.
2. **Candidates carried from the first draft, each screened against the profile:**
   - **A. A per-leaf bound in the cull.** The cull (`pipeline/foliage.rs:255` on master c71ef3e1) transforms every element vertex of a leaf until one lies within the shell. A leaf deep in the crown pays for all its vertices, and each costs one `quadrant()` evaluation in `radius_at` (`envelope.rs:120, 269`; #108 replaced the two `powf_fixed` calls) and a 128-step scan in the profile distance (`envelope.rs:255, 276`). The cull already computes the element's extent for an overflow check (`foliage.rs:267-269`), so part of rho exists. Every vertex lies within rho = |M| times the element's extent of the leaf's origin, and the distance to the profile moves by at most the distance moved. So: reject a leaf when its origin is deeper than shell + rho + epsilon on both tests, accept it when even its farthest vertex is inside, and run the vertex loop only between. The `radius_at` test needs a slope bound for the envelope, and the profile scan can use a height-band index. The same bound may serve the GPU cull (`in_shell`, `place.wgsl:42` today); if it does, both executors take it together in this spec, held within fn-125's tolerance, so they stay one algorithm.
   - **E. A parallel reference executor on native.** Station sources and the cull are independent per leaf, and chunks joined in order give identical output. It runs serially in Wasm.
3. **New candidates** come from the profile, not from this list.

**The profiler is committed.** A native example in `telperion-core` builds each preset through `pipeline::executor` and times the reference executor at its stage boundaries (station sources, clumping reduction, cull, compaction), printing one JSON line per run with preset, seed, revision and stage times. No timer runs inside a per-leaf loop. fn-76 and fn-91 each built a throwaway timer that was then deleted; this one stays so the next speed spec starts from it. (host, 2026-09-27)

**How a change is judged.** Native is the gate: medians of five warm runs per preset and seed, base and candidate on the same machine in one session. A change passes when the stage it targets is at least 5% faster on the preset where the profile named that stage the largest cost, at both seeds, and no preset's stage or total at either seed is more than 2% slower. E is native-only by design and runs serially in Wasm; A's Wasm time is recorded, not gated, because the browser draws through the GPU. (host, 2026-09-27)

**The quoted timings are old.** The spruce and birch figures above were taken on 2026-09-21, before #100 (growth) and #108 (the envelope). They motivate the spec; the R1 profile replaces them. (checked 2026-09-27)

## Acceptance Criteria

- **R1:** A committed profile of the reference executor, split by stage, for the oak, spruce, birch and beech at seeds 1 and 7, taken on fn-125's merged code before any change. [inferred]
- **R2:** Each change keeps the reference executor's output byte-identical to fn-125's for every catalogue and in-work preset at seeds 1 and 7. A change shared with the GPU keeps the two executors within fn-125's tolerance. [inferred]
- **R3:** Stage medians of five runs are recorded on base and candidate for each change. A change that does not speed its stage by at least 5% on the preset it targets is reverted and reported. [inferred]
- **R4:** Native peak RSS and the wall time of `cargo test --profile ci --workspace --no-fail-fast` are recorded on base and candidate. [inferred]
- **R5:** The profiler is a committed native example that prints one JSON line per run (preset, seed, revision, and each stage's time) for any catalogue or in-work preset, and R1's profile and every R3 measurement come from it. Errors: an unknown preset id is refused by name. [inferred]
- **R6:** A change is kept only under the judging rule above: at least 5% faster on its target stage for the preset the profile named, at seeds 1 and 7, and no preset's stage or total more than 2% slower; otherwise it is reverted and reported with its numbers. Errors: no error surface beyond R3. [inferred]

## Boundaries

- No change to the algorithm's output. Speed only. [inferred]
- Growth speed is fn-124; the GPU expansion's own speed is fn-125's R4. [inferred]
- No full-forest capture. [CLAUDE.md]

## Decision Context

- Depends on fn-125, which defines the reference executor this spec speeds up. [user]
- The first draft's B and C are dropped because fn-125 deletes their code. A and E carry over. [inferred]
- The first draft's line references (`place_impl`, `foliage.rs:253`) predated renames: `place_impl` is now `place_on`, and the cull starts at `foliage.rs:233`. [inferred, checked 2026-09-24]
- Lower priority than growth speed and the GPU contract since the owner's decision of 2026-09-23 that the GPU is the drawn path. [user]
- The GPU executor's own speed moved on 2026-09-27: fn-125's R7 now guards it (its R4 is superseded), and moving wood ring preparation (sampling, frames, packing) onto the GPU is a separate spec. The Boundaries line naming fn-125's R4 predates this. [user]
- R6 sharpens R3: R3 left "the preset it targets" and the seeds open. [inferred]

## Resolved via Codebase

- Every reference re-anchored on master c71ef3e1: `place_on` `placement.rs:138`, `place_run` `station.rs:45`, the cull `foliage.rs:255`, `radius_at` `envelope.rs:120` through `quadrant()` `:269`, the profile scan `envelope.rs:255, 276`, the GPU `in_shell` `place.wgsl:42`. fn-125 rewrites the placement and cull code, so the R1 profile is taken on fn-125's merged code, as R1 already says.
- The evidence files hold what the spec quotes: fn-91 `CPU-CANDIDATE.md:21` (spruce 3844.62 ms, 82.5%, cull 359.57 ms) and `.flow/evidence/fn76/LOCAL.md:17` (birch place plus cull 4.21 s), both from 2026-09-21.
- The presets exist for R1 and R2: `params::CATALOGUE` and `IN_WORK`; oak, spruce, birch and beech are in `catalogue/`; `[profile.ci]` is in the workspace `Cargo.toml`.
