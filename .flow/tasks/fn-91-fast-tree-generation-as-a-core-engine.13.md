---
satisfies: [R3, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.13 Parallelize native CPU surface expansion within the memory budget

## Description
Address the separately measured CPU-owned bottleneck: ordinary native wood construction remains about185ms on oak in .8 GPU-assisted CPU-output controls. Evaluate bounded parallel expansion of independent surface runs into final CPU arrays. Source analysis is in CPU-OWNED-BOUNDARY.md; it is a hypothesis, not proof. This serves native consumers requesting actual CPU geometry, not a website shortcut or a claim about Wasm CPU output.

Before code, send host a concrete phase/lifetime/peak-capacity design, run partitioning, canonical fallback/error behavior and smallest timed experiment. Admission requires a credible accounted peak no larger than the existing serial surface build, including worker stacks/scratch and metadata. No per-worker full meshes followed by flattening, full GPU readback detour, unbounded threadpool/cache, default new dependency or unsafe uninitialized-output scheme. Prefer safe disjoint slices of final arrays and a bounded worker count with explicit stacks. A two-phase design may emit positions/coords, release paths/distance/sampling storage, then allocate normals/indices. Any reused normal/index implementation must retain ordered f64 cross accumulation and float32 writes. Do not duplicate botanical sample/frame formulas. Thread availability/creation failure must keep a usable serial path. No serialized engine state or threading requirement on Wasm.

Preserve run sort/ties, positions/coords/normals/index order, caps, bounds, dropped-triangle accounting, contact/PreparedSurface APIs and existing invalid/resource-limit semantics. Canonical facing for zero normals depends on transported frames; do not allocate one facing vector per vertex. A conservatively qualified fast path may abandon candidate arrays and join workers before serial fallback for collapsed/zero-normal cases, rather than change geometry or overlap a second full mesh. Bound fallback frequency explicitly. Public surface build remains synchronous; existing callers receive the same CPU-owned representation. Native parallel path must be size/capability gated, while small builds, single-core and Wasm stay supported. Preserve future parameter generality; no species-specific behavior.

Use a cheap native release screen first: baseline/candidate mature oak/spruce1/7, first+3warm full CPUwood, all fields bitwisecompared outside timing. Aim>=6x wood speed on oak as a route toward100ms CPU-owned delivery; a candidate must at least halve each oak wood time without material regressions and meet accounted memory before integration is worth considering. Keep targets/misses explicit. One bounded implementation/pairedscreen, no iterative worker-count tuning. Separate untimed memory accounting from timing; include thread stacks, all phases, outputs, scratch and coherent fallback. Reuse .10 full-field mesh comparison infrastructure and existingbaseline semantics. If architecture/memory fails, report rejection without forcing implementation. If timing/memory fails, archive and revert. Hostreviews outcome before broader qualification.

Qualifying candidate integrates shared native CPU surface construction, then fresh native CPU-output and GPU-assisted CPU-output all4 first+3warm establishes actual requested-output delivery versus immediatebaseline and originalbaselines, with initialization/RSS/counts/fallbacks explicit. Native GPUresident/browser paths must remain unchanged in behavior; run focusedrenderer checks and Wasmbuild, but do not repeatbrowserGPU timings for an untouchedalgorithm. Full-output exactness reuses .8visual acceptance; no new images. No claim of browserCPU improvement or whole-process memory qualification from boundedcapacity accounting.

Touches: docs/generation-limits-inventory.json (reviewed scheduling/capability classifications only), crates/telperion-core/src/surface.rs, crates/telperion-core/src/surface/**, crates/telperion-core/tests/surface*.rs, crates/telperion-render/examples/generation_gpu.rs (diagnostic metrics only ifneeded), .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/**. Quick: pinned nextest release explicit surface/prepared/collapse/attachmenttargets; focused newparallel/error/fallback/unit tests and renderergenerationtests onretainedcode; scopededition2021format; Wasmbuild. No fullworkspace/debugallpreset or conflictingGPUtests. Existing .8baselinegreen pluslaterretainedtaskgates. Serializetimings/builds. Five ticks/10commitsmax,noresetparent. Frictionimmediate; hostdirectreview, reviewbackendnone. Commitgitadd-A,no push/rebase. Hostretentionbeforedone/showreceipt. Handovers .flow/tmp/fn91-parallel-summary.md andfn91-parallel-evidence.json.
## Acceptance
- Host-approved bounded ownership/parallel design demonstrates no larger accounted surface peak, including worker resources and fallback, before implementation.
- Native CPU mesh expansion preserves full output and correctness/fallback contracts without requiring threading on Wasm or small/single-core requests.
- One native screen records6x oakwood aspiration and at-least2x admission target, exactall4outputs, rawperformance/capacityprovenance; failed design/screen is rejected and reverted.
- Retained code receives actual nativeCPU-owned delivery qualification, focusednative/Wasm tests and explicit parenttarget/distinct-representation limits; hostdecision precedes verified Flowcompletion/receipt.


## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
