# Rust surface benchmark prototype

## Goal

Measure whether a Rust surface core earns migration from optimized TypeScript for browser and native consumers, as an isolated experiment beside fn-6 (owner request for a new worktree carrying the updated STRATEGY.md and the parallel-work prospect, `.flow/prospects/parallel-work-alongside-fn-6-2026-09-05.md`, survivor 1). Surface stage only; production engine, presets and API unchanged.

## Decisions

- Accepted direction: an engine-independent generation/lifecycle core with browser and native bindings, Rust conditional on measured end-to-end latency and peak-memory benefits.
- Reference frozen at `019234b554a1b387e23b05dd3c7e2626f428f907`; compare reference TS, preallocated TS, and one Rust implementation compiled native and wasm32; coarse bindings (one crossing per mesh build).
- An honest negative result completes the experiment; surface CPU gains are never translated into GPU-frame or lifecycle claims.

## Acceptance Criteria

- **R1:** Isolation and provenance: everything under `experiments/rust-surface-benchmark/`, frozen inputs hashed, Telperion, Laurelin and small cases — met.
- **R2:** Reference TS, optimized TS, native Rust and Wasm implement the same algorithm with no simplification — met.
- **R3:** Correctness: finite positions, in-bounds indices, matching connectivity/winding, scale-aware tolerance, determinism — met (13 cases; zero positional difference on presets).
- **R4:** Measurement: release builds, warmup, at least five samples, median/p95, counts, bytes, environments labelled — met (10 samples per case).
- **R5:** Memory labelled honestly, unavailable metrics stated — met.
- **R6:** Reproducible commands, machine-readable results and a checked-in report — met.
- **R7:** A justified migration recommendation without GPU claims — met.

## Result

Browser Wasm cut surface-build latency well below preallocated TypeScript (Telperion compute median 22.17 ms Wasm vs 70.64 ms optimized TS vs 101.23 ms reference; Laurelin optimized TS 129.74 ms). This supported a bounded surface-core follow-up but not yet a broader Rust migration, which needed end-to-end lifecycle and memory evidence (taken up by fn-8). Report: `experiments/rust-surface-benchmark/REPORT.md` (raw samples archived at commit 1922505). Implementation 525a2a5; completion review SHIP.

## Open

None; whole-pipeline comparison belongs to fn-8.
