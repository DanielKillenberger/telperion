# Growth runs on ordinary floats

## Goal & Context

fn-245's first stage. The owner (2026-10-09): "doesn't sound very elegant to have hundreds of emulated inlined copies of 32 bit operations", then "yes" to the breakthrough and "convenve with astra about 3". Contract A (exact Q32/Q62 fixed point with emulated 64-bit products, so CPU and GPU agree to the bit) is the main cause of the `grow` shader's 13-17 s cold compile and a large share of the Wasm cost (fn-245 `PROFILE.md`). The experiment (`EXACT.md`, fn-245 evidence) nudged every decision boundary at 1e-7 and 1e-6: 0-2 of 3.5-19M decisions flipped, height moved at most 0.1 mm, no branch appeared or vanished. Astra's architecture review (fn-245 `ASTRA-ARCH.md` §3) agrees and names what stays exact. Release item (fn-237 R0: performance must be no worse than 0.1.4).

## Requirements

- **R1 (inventory, checked against code):** every biological quantity on contract A on both executors (`fixed.rs`, `allocation.rs`, `transcendental.rs`, `contract.wgsl` and their callers), and every place identity or ordering depends on it; written before code.
- **R2 (floats):** biological state and arithmetic in `f32` on the device and the matching CPU operations; `f64` allowed for CPU preparation and diagnostics. Q32/Q62 and wide products removed from biological quantities; the pinned transcendental wrapper removed unless R1 names a consumer that needs it.
- **R3 (exact where identity lives):** lineage hashing and the key registry, integer cycle and event indices (no time accumulated by adding small floats), stable sibling order, deterministic prefix allocation and fixed reduction trees stay exact; no float atomics. Draws compare a documented, exactly representable uniform from the lineage hash against the float probability, endpoints handled explicitly. Presence windows and fractional births unchanged: no hard topology thresholds.
- **R4 (four checks replace byte parity):** (1) exact repeatability within an executor: repeated builds, fresh contexts, reused buffers, varied scheduling; (2) exact discrete contracts: hashes, keys, parent links, event order, endpoints; (3) invariants: conserved vigour, normalised PA means, bounded light, finite geometry, non-negative radii, contacts, budgets; (4) CPU-GPU agreement by lineage: position and radius error, unmatched wood weighted by length and radius, crown dimensions, leaf area, major-branch survival, tolerances set from measured float runs on every shipped preset, never from EXACT.md's perturbation sizes.
- **R5 (proved):** cold `grow` shader compile time and Wasm stage time before and after, on every shipped preset; whole/mid/close stills of every preset, viewed, beside master's; baselines re-pinned with that evidence (AGENTS.md "Generator evolution"); Astra's agreement; gate and npm test green.

## Boundaries

Arithmetic only: the growers, light and the CPU/GPU cut are unchanged here (the next three specs). Release item for fn-237.
