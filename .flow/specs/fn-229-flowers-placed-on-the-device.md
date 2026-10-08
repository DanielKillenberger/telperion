# Flowers placed on the device

## Goal & Context

fn-216 computes flowers through the CPU's `Expansion::organ` from `Tree::blooms`. On fn-212's device path the fine roots are dressed on the device and never enter the CPU's tree, so their blooms are missing: on a flowering overlay the device path drew 3.7-6.2 % of the florets the whole tree has (MERGE-F216.md in fn-198's evidence). The F4 merge refuses flowering families from the device path as an interim named gap (host, 2026-10-08), so a flowering share crossing 0 switches executors, which the principles do not allow to stay. fn-125 names this pass ("Flowers from fn-216"). Laurelin and Telperion are defined by their blossom (fn-217-219), so it gates them.

## Requirements

- **R1:** Florets placed on the device from the dressed fine nodes, following the pattern fn-212 built for leaves (host, 2026-10-08): no added readback or round trip, and extra kernels in the same submission are allowed. They are keyed by the lineage registry's bloom keys (BLOOM, BLOOM_ROLL, FLORET, FLORET_SIZE, FLORET_OPENING) under the phytomer's own 64-bit key, which the dress writes into the drawn record. The flowering share joins the device's per-PA table. The WGSL mirrors `bloom.rs`. The counts are scanned deterministically in fn-212's existing stations scan, with no atomics. Placement is a second entry point in `place.wgsl`, run in the same submission as the device's leaves. The florets are packed against the tree's combined leaf box, padded by the largest floret's reach, and the CPU block's florets are repacked against that box. They go into an organ buffer with a mass grid through the resident handoff: the CPU block first, the device's after it, as fn-212's handoff does for leaves.
- **R2:** Parity: florets per tree equal to the CPU whole tree's on beech and spruce at 40 and 80 with a flowering overlay, positions within fn-125's screen, residuals by named class.
- **R3:** The interim refusal in `executor::apart` deleted; flowers cast shadows if fn-216's organ draw supports it, else recorded.
- **R4:** Gate and npm test green; every shipped species unchanged (flowering 0).

## Boundaries

Placement only; floret form and blossom at distance are fn-217's follow-ups. Depends on fn-198 (with fn-216 merged).