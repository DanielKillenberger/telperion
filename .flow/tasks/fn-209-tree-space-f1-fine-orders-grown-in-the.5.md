---
satisfies: [R1, R2]
---
# fn-209-tree-space-f1-fine-orders-grown-in-the.5 Fine growth stays on the device, and two open items

## Description
Host decisions after task 4 (2026-10-06; GPU-GEOMETRY.md). (1) The growth wave loop moves onto the device (task 4's option a): the next wave's buds expanded on the device by prefix-sum compaction and indirect dispatch, the overflow re-run as a device queue; nothing per wave read back (Astra change 3), only a completion count. (2) The sag lay's frame correction does nothing for fine chains in the CPU reference (Bounded::settled runs before their phytomers are laid), and the GPU mirrors it. Check it against whole-tree growth (what today's engine does at the same junction); if it is an accident, fix it in both executors and show its size; if it is right, record why. (3) Host decision 3, built: a fine root whose conservative extent bound from its attachment can reach the ground is grown on the CPU reference executor; the GPU's D19 flag becomes a test that fails if any such root reaches it.

## Acceptance
- No per-wave readback on the GPU path (shown by the code path and a counter); topology identical by lineage and geometry within the screen, as task 4. - Spruce and beech at 80, seeds 1 and 7: the whole fine stage warm and cold against the CPU executor on 1 and all cores. - The frame correction's finding recorded, with its size on the four species if changed. - Ground-reaching roots routed to the CPU, with a test. - Workspace gate green.

## Done summary
Fine growth now stays on the device (host decision 6, recorded in FINE-CONTRACT.md).
- Runs are recorded, scanned and packed on the device, and chains that overflow a region are queued and gathered there.
- Geometry items and the next wave's buds are written there too.
- Only totals cross to the host: four per run (packed axes, packed phytomers, scratch words, queued chains) and three per wave (kept axes, their phytomers, borne buds).
- `Cost::trips` counts the reads, and the tests hold that it equals runs plus waves: 10 to 21 round trips per tree.

Results:
- Topology is identical by lineage, and geometry stays inside the screen: at most 34.2 um, RMS at most 2.3 um.
- Laid floats are identical (by digest) across reversed batches, and across batches of 64 at age 25. The batches of 64 caught a queue-offset bug, now fixed.
- Timing at load 12 to 20:
  - Spruce 80 runs 0.43 s warm and 0.6 to 0.9 s cold on the GPU, against 1.1 s on 32 cores and 19 to 25 s on one.
  - Beech 80 runs 0.11 to 0.16 s warm, against 0.12 to 0.19 s on 32 cores.

Ground routing (host decision 3) is built: roots whose reach bound comes within the landing zone are grown on the CPU only. A test holds that no other root comes near the ground, and the GPU tests fail on the D19 status bit.

The frame-correction accident is recorded with its size in FRAME-CORRECTION.md and not fixed, per the host's direction. GPU-DEVICE.md lists what the realized-totals change would touch in the GPU executor.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: a2b18180, d0b3c0a0, f86ceeba, 13987193, 80370cda
- Tests: cargo test --profile ci --workspace --no-fail-fast, FINE_GPU_LOCK=<lock> cargo test --profile ci -p telperion-render fine_geometry_at_40_and_80 -- --ignored --nocapture
- PRs: