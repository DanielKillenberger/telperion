# Rust surface benchmark prototype

## Goal and context

Evaluate whether a Rust surface core earns migration from optimized TypeScript for Telperion's browser and native-engine consumers. This experiment runs alongside FN-6 in an isolated worktree. It measures the surface stage only and leaves the production growth engine, preset schema and public API unchanged.

The owner requested a new worktree and commits carrying the updated STRATEGY.md and parallel-work prospect. The accepted direction is an engine-independent generation/lifecycle core with browser and native bindings, with Rust conditional on measured end-to-end latency and peak-memory benefits. Source: .flow/prospects/parallel-work-alongside-fn-6-2026-09-05.md, survivor 1.

## Scope and architecture

Keep the experiment under experiments/rust-surface-benchmark/. Freeze the reference implementation and representative inputs from committed revision 019234b554a1b387e23b05dd3c7e2626f428f907, recording hashes and provenance. Include small synthetic geometry and both shipped preset skeleton/radius inputs; generated fixture data may be rebuilt deterministically rather than committing large blobs. Run identical inputs and surface parameters through the existing TypeScript algorithm, a preallocated-buffer TypeScript variant, and a shared Rust implementation compiled in release mode for native and wasm32-unknown-unknown. The optimized baseline must produce equivalent geometry and perform equivalent work.

Keep bindings coarse. Inputs cross once per complete mesh build and outputs are flat positions/indices. Record ownership and memory-growth rules. A dependency-light raw Wasm ABI is acceptable; wasm-bindgen is optional. The browser harness can be independent of the existing harness to avoid FN-6. Measure browser TypeScript and Wasm together; measure native separately and label the different environment.

## Requirements

- **R1 Isolation and provenance:** all implementation files live under experiments/rust-surface-benchmark/; production src, harness, package manifests and lockfiles remain unchanged. Frozen reference sources and fixture generation identify the exact source revision and hash inputs. Include Telperion, Laurelin and small geometry cases.
- **R2 Comparable implementations:** existing TypeScript, meaningfully optimized/preallocated TypeScript, and shared native/Wasm Rust implement the same surface algorithm and parameters, including transported frames, lobes/twist, flare, forks/sockets, caps and winding. No geometry simplification or disabling work to improve results.
- **R3 Correctness:** validate positions are finite, indices are in bounds, counts and connectivity/winding agree, and positional differences stay within a documented scale-aware tolerance selected independently of observed failures. Repeated runs are deterministic within a target; cross-target floating-point tolerance is explicit. Include forks and degenerate/minimal inputs, and check representative preset outputs.
- **R4 Measurement:** report release builds, warmup and repeated samples (at least five measured per representative case), raw sample values, median and p95, node/vertex/triangle counts, input/output bytes and machine/tool/browser versions. Separate compute-only and caller-visible build latency including binding/input/output transfer; native and browser timings must be labeled separately. Exclude process startup and fixture generation from steady-state compute, disclose cold-start separately if measured. Preserve equivalent output ownership when comparing end-to-end paths.
- **R5 Memory honesty:** collect and label supported measurements of peak memory or high-water usage; distinguish native process peak RSS, JS heap, Wasm linear memory and explicit buffer bytes. A memory metric unavailable in the browser is reported unavailable rather than guessed. Do not equate Wasm memory capacity with total browser peak memory. Document remaining uncertainty if no comparable total-memory measurement is available.
- **R6 Reproducibility:** provide documented commands to build, verify and run the experiment, explicit prerequisites, actionable errors, and machine-readable results plus a concise checked-in report containing actual measurements. Generated binaries and large intermediate fixtures stay ignored; dependency locks belong to the experiment if used. A clean checkout must reproduce fixture inputs without relying on uncommitted FN-6 files or an external working copy.
- **R7 Decision:** report evidence for or against a broader Rust migration, distinguish algorithm/layout improvements from language improvements, and explicitly avoid translating surface CPU gains into GPU-frame or full-lifecycle gains. Winning a speed ratio is not an acceptance condition; an honest negative result completes this experiment.

## Boundaries

No full core migration, production bindings, renderer rewrite, growth/lifecycle implementation, FN-6 changes, PR publication or merge. Preserve the strategy and prospect changes in this branch. Run through the configured final completion review; per-task implementation review is disabled by the repository's instruction.

## Quick commands

Baseline on the frozen checkout: npm run typecheck; npx vitest run src/mesh/surface.test.ts src/mesh/frames.test.ts src/mesh/paths.test.ts --maxWorkers=1 --testTimeout=60000.

After implementation: the experiment's documented build, correctness and repeated benchmark commands, plus the same baseline commands. The implementation must state exact runnable commands in its README and evidence. Check Rust formatting and compiler warnings; use targeted tests that validate actual output equivalence rather than timing thresholds.

## Task coverage

One implementation task covers R1-R7. The task includes the experiment, actual native/browser measurements, report and reproducibility docs. The conductor owns the strategy/prospect/spec initialization commit and final completion review.
