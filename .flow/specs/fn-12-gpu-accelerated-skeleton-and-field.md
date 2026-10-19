# GPU-accelerated skeleton and field generation

## Goal

Approved with the roadmap ("those specs are excellent can you $flow-next-capture them?"), and run by the owner without plan or implementation review: investigate GPU acceleration of skeleton generation and field construction/querying, and ship only workloads with a measured, worthwhile end-to-end gain while ordinary and giant trees keep their botanical identity and field semantics. A measured negative result is a complete outcome.

## Decisions

- Rust CPU core stays authoritative and engine-independent; WebGPU is isolated to a benchmark-only experiment (consumers render through WebGL).
- First compute proof: a full wood-and-foliage indexed field-query batch (queries are independent; growth is sequential).
- Predeclared qualification: at least 20% median end-to-end gain, GPU maximum below CPU minimum, no worse tail, exact repeatable occupancy, memory accounted; cold costs include extraction, packing, upload, sync and readback.
- No automatic backend switching, no partial success, no epsilon relaxation or workload reduction.
- Owner, 2026-09-07: remove the rejected implementation and unused material; the optional owned CPU `field.snapshot()` stays (it serves fn-18).

## Acceptance Criteria

- **R1:** Profile ordinary and giant skeleton, field construction and query workloads; compare CPU and GPU on equivalent inputs including transfer and memory — met for queries; GPU growth and construction explicitly inconclusive.
- **R2:** Integrate accelerated paths only where measurements justify them — met: candidate rejected, CPU retained.
- **R3:** Document hardware, precision, determinism and whole-request failure behaviour — met.

## Result

No production GPU path qualifies. CPU medians: skeleton 11.3 / 312.0 ms, foliage prerequisite 25.2 / 640.8 ms, field index 23.0 / 662.5 ms (Ordinary / Telperion). Every cold GPU query batch is slower (e.g. Ordinary 64³ 21.3 ms CPU vs 101.1 ms GPU; Telperion 64³ 26.3 vs 261.4 ms), and f32 WGSL breaks occupancy: 65 and 54 contact-cell mismatches (with false negatives), 2 on Ordinary 64³. Only the giant 64³ source-resident subset passed timing (25.4 -> 11.8 ms), kept as a research direction for reused, precision-corrected data. Measured on one configuration (RTX 3080, Chromium 151, Vulkan/ANGLE). The rejected implementation is recoverable at commit `6f3adb2`. Report: `.flow/evidence/fn12/REPORT.md`.

## Open

- Per-round skeleton profiling for GPU growth and GPU field construction were never measured.

## Evidence

- `.flow/evidence/fn12/REPORT.md`, `gpu-tests.json`, `gpu-input-verification.json`
