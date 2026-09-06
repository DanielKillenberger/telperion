# GPU-accelerated skeleton and field generation

## Conversation Evidence

> user: "those specs are excellent can you $flow-next-capture them?"

This approved the roadmap investigation. The owner subsequently authorized standard-depth planning and implementation in a separate worktree, without plan or implementation review.

## Goal & Context

Investigate GPU acceleration of skeleton generation and field construction/querying, then implement only workloads with worthwhile end-to-end gains. Ordinary and giant trees must retain their botanical identity and field semantics; a measured negative result is useful completion, not a reason to ship a slower or approximate default.

## Overview and Scope

Three sequential tasks establish reproducible CPU workloads, evaluate a bounded GPU candidate, and make the measured adoption decision. Developers get reproducible cost and correctness evidence; consumers retain the current synchronous CPU API unless a separately explicit accelerated entry point qualifies. No deployment or renderer changes are involved.

## Architecture & Data Models

Keep the Rust CPU core authoritative and engine-independent. Profile skeleton growth, foliage prerequisites, field-index construction and batched query stages separately. Investigate skeleton acceleration by measuring its sequential rounds/data dependencies and feasible parallel substeps; record why a candidate is or is not worth prototyping. The first compute proof is a full wood-and-foliage indexed field-query batch, since queries are independent and existing construction can supply equivalent inputs.

Use a small owned field snapshot boundary when necessary for the experiment, with explicit numeric layout, bounds and ownership; do not couple botanical state to WebGPU or Three.js. CPU and GPU measurements use the same browser, specimen, query inputs and returned occupancy bits. Query-only resident timings are supplemental: charge snapshot extraction, packing, GPU upload, synchronization and readback to cold and amortized end-to-end results. Construction/prerequisite costs remain visible even when shared.

## API Contracts

Existing synchronous build and CPU field queries retain their contracts. No automatic backend switching. An experimental asynchronous query candidate consumes owned snapshot data, returns complete wood/foliage occupancy or an explicit failure, and disposes resources deterministically. A production accelerated entry point is conditional on task 3 qualification and must preserve revision/lifecycle behavior. Snapshot extraction must be optional and cannot impose copies on existing consumers.

## Approach

Use fixed Ordinary and Telperion presets/seeds, full field output with no render meshes, and reproducible 32^3/64^3 grids plus outside, point and contact-boundary queries. Record hashes, counts, caps/completeness, requested outputs and raw timings. Use one warmup and at least five measured paired samples, report medians and tails, cold preparation separately, memory by domain, and hardware/browser/adapter metadata. Native CPU profiling can inform stage selection but is not the browser GPU speedup denominator.

Raw WebGPU compute is the minimal experimental backend; it does not migrate the renderer. Optional timestamps supplement wall-clock submit-to-copied-result timings. Predeclare qualification: at least 20% median end-to-end gain with repeatable non-overlapping sample evidence on a named workload, no worse tail behavior, and explicit memory cost. A workload-specific path must leave nonqualifying workloads on CPU. An inconclusive result remains inconclusive.

## Edge Cases & Constraints

The CPU uses f64 and WGSL f32 cannot promise identical floating point behavior. Occupancy is discrete: any unexplained mismatch, false negative or repeatability failure rejects uncorrected adoption; if boundary correction is used, include its cost and verify exact final flags. Empty/outside/zero-extent cells, touching tapered wood and foliage boxes, invalid/nonfinite/negative cells and large coordinates must be covered. Unsupported adapter/features, limits, allocation/shader/submission/readback failure, device loss and timeout return explicit failure or documented whole-request CPU fallback; partial results never count as success. GPU memory includes source, packed, staging and result buffers alive together. No concurrent GPU measurements with fn13.

## Acceptance Criteria

- **R1:** Profile representative ordinary and giant skeleton, field construction and query workloads; compare CPU and GPU candidates on equivalent inputs/outputs including transfer, synchronization, preparation and memory. Report unsupported hardware, incomplete specimens, invalid samples and inconclusive results explicitly. [paraphrase]
- **R2:** Integrate accelerated paths only where measurements justify them; preserve geometry, field behavior and repeatability against the core reference. Mismatch or noisy/negative benchmarks reject adoption and produce a documented result; no silent precision weakening or reduced botanical workload.
- **R3:** Document supported hardware, precision and determinism guarantees. Missing devices, execution/resource failures and stale/disposed data have a documented whole-request fallback or explicit recoverable failure, never partial success.

## Quick commands

```bash
cargo test --release -p telperion-core --test field
npm run typecheck
```

The final task also runs workspace Rust, harness and browser/Wasm checks appropriate to changed bindings, plus the reproducible GPU experiment on the named adapter.

## Boundaries

Rendering optimization belongs to fn13. No WebGL-to-WebGPU renderer migration, external-engine proof, universal compute abstraction, GPU-every-stage mandate, botanical simplification or thousands-of-identical-tree draw benchmark. Future distinct-specimen scale informs ownership and memory reporting only.

## Strategy Alignment

- **The core and integration** — preserve a lean portable core, optional representations and measured binding costs.
- **The supernatural field** — preserve shared authored field semantics and reproducibility.

## Decision Context

Repo and docs scouts confirm batched field queries are independent while growth mutates settled attractors and parent order sequentially; stage profiles decide whether a growth substep merits GPU work. Existing indexed CPU queries are the baseline, not a deliberately naive scan. WebGPU is isolated to the experiment because current consumers render through WebGL. R2/R3 capture inferences are confirmed by existing f64 field semantics, deterministic geometry tests and resource-error APIs. No new spec dependencies: current core contracts are available; future growth/legendary work is not a prerequisite.

## Early proof point

Task 1 proves equivalent reproducible workloads and bounded transferable field data. Task 2 proves measured GPU feasibility; if it fails precision, support or speed gates, task 3 records that result and keeps CPU production behavior.

## Requirement coverage

| Requirement | Tasks |
|---|---|
| R1 | 1, 2, 3 |
| R2 | 1, 2, 3 |
| R3 | 1, 2, 3 |

## References

- [WebGPU buffer mapping, limits and timestamps](https://www.w3.org/TR/webgpu/)
- [WGSL floating-point evaluation](https://www.w3.org/TR/WGSL/#floating-point-evaluation)
- [Official WebGPU compute and timestamp examples](https://github.com/webgpu/webgpu-samples)
- [Three.js compute portability limits](https://threejs.org/docs/pages/Renderer.html)
