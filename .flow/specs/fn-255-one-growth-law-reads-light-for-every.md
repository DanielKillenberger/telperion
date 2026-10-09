# One growth law reads light for every species

## Goal & Context

fn-245's second stage. The owner (2026-10-09): "i'm still not comfortable with only the oak growign with light... seems like it breaks our design principles". It does: `Species::reads_light()` (`crates/telperion-space/src/species.rs:423`) is any of four light rows non-zero, and it selects a different grower (the stepped CPU executor, `fine/coarse.rs::lit`, `grow.rs::apart` keeping light-reading trees off the GPU's fine roots). Astra's architecture review (fn-245 `ASTRA-ARCH.md` §1-2) proposes one cycle for every species, with light read from a frozen summary of the previous cycle's foliage, and names the light model as the open research question. fn-226's failed outline-light experiment changed light and allocation at once (its `R3-R4.md:105`), so it does not show analytic light fails. Release item.

## Requirements

- **R1 (one cycle, on the CPU first):** every shipped preset grows through one cycle graph: light read at each bud from the previous cycle's immutable foliage summary; bottom-up sums of capture, upkeep and presence; top-down shedding and vigour split by the apical-control law; per-PA normalisation; per-bud survival, abortion, continuation, dormancy and branching; end-of-cycle geometry and summary update; final settle. `reads_light()` and every branch it drives are deleted. Zero light coefficients give neutral terms, never another grower; the tree at zero and arbitrarily near zero follows the same code.
- **R2 (allocation kept):** per-PA normalisation stays the actual one, `size_i = a_i × Σpresence / Σ(presence × a)` (`allocation.rs:153`); a light change and an allocation change are never tested together.
- **R3 (the light candidate, gated):** a hierarchy of local foliage volumes (leaf area, centroid, second moments; compact ellipsoidal kernels with a closed-form ray integral) read over the existing nine sky directions (`light.rs:23`), double-buffered, finer groups near and aggregates far with a blend across the refinement interval, no double counting, presence-weighted. Measured against the current lattice as oracle: light error per bud, summaries visited per bud including the worst tail, and whether moment updates fuse into existing reductions. If fidelity or cost fails, the spec stops with that evidence and the owner decides (`NEEDS_HUMAN`), rather than building on it.
- **R4 (proved):** every preset's stills (whole/mid/close) viewed beside the current branch; the oak's light response held (its shade-shedding and balance traits visible); species-to-species walks crossing zero light without a jump; build time per preset; the lattice and stepped executor removed once R3 passes; Astra's agreement; gate and npm test green; the owner's verdict.

## Boundaries

The CPU executor and the light model. Moving it to the GPU is the next spec. Release item for fn-237.
