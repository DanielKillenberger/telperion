**Verdict: accept with changes.** The device-resident direction is right. The draft needs a clearer contract and measurement boundary before implementation, particularly around conversion, leaf culling, contacts and memory.

1. **The scope is principled, provided it extends the pipeline’s executor interface.**

   Fine geometry should flow from the GPU grower into dressing and drawing without reconstruction as a CPU `Tree`. The CPU reference should define the same operations, and unsupported inputs should produce named errors. Small totals and the established per-root settle records are compatible with this; production per-node readback is not.

   “Produce today’s inputs on the device” is too weak as the architecture. Today’s conversion includes geometry-dependent merging, radius cutoffs and relay attachment selection. I checked these in `pipeline/space.rs` on the fn-198 branch. FINE-CONTRACT’s decision 2 instead preserves botanical identities and assigns omission to drawing. **Resolve that discrepancy before freezing the probe’s reference.** Matching the converter’s current bytes could preserve behavior the host has already rejected.

   The CPU portion also includes roots routed there for ground handling, not just structure above the cut. Replace the opening “95–99%” claim with measurements from the realized executor and its final routing.

2. **Keep fn-212 separate now; reconcile fn-125 and fn-171 after F4 acceptance.**

   [fn-198 R5](/home/daniel/Projects/telperion/.worktrees/fine/.flow/specs/fn-198-tree-space-f-the-engine-in-production.md:32) gives the order explicitly. Accept F4, then rewrite fn-125 before F4 merges. Do not make fn-212 depend on that rewritten spec if fn-212 supplies work needed to accept F4.

   Fn-212 can establish the device-resident bridge under the existing principles and numeric tolerances. Any compatibility with today’s station layout should be identified as transitional. After acceptance, fn-125 owns the remaining expansion contract for engine leaf-bearing nodes, including leaflets and clumping.

   **Do not land fn-171 unchanged first merely to obtain its ring implementation.** Its baseline, sources and dependency on fn-125 belong to the earlier pipeline. Transfer its useful obligations into fn-212: sequential-frame timing, contact correctness, precision errors and measured gain. At the required post-F4 reconciliation, supersede completed obligations and retain only distinct remaining work. Avoid two ring-preparation implementations.

   Fn-176 can remain later, but “unaffected” is inaccurate. F3 already supplies curve clustering and ring-level error data, and FINE-CONTRACT assigns thin-wood omission to drawing. Fn-176 must consume that representation and preserve identities.

3. **Probe first is right; the current R1 is too broad and its 3× denominator is ambiguous.**

   Start with a bounded dependency and deletion audit, then a vertical probe that reaches actual consumers. Implementing rings, stations and curves for every fine node before discovering what can disappear is almost the entire native feature.

   The historical spruce-80 figure consists of:

   | CPU work | GPU-PATH measurement |
   |---|---:|
   | Conversion and expansion | 497 ms |
   | Rings | 1,080 ms |
   | Stations | 1,184 ms |
   | Curve | 1,125 ms |
   | Total | 3,886 ms |

   Thus “3.9 seconds for three preparation passes” includes another half-second of conversion and expansion. That probe also did **not** execute GPU leaf placement for engine trees. Fn-198’s subsequent integrated timing reports 14.376 seconds to a resident spruce, with 3.056 seconds in stage 2. Different configurations and measurement boundaries prevent treating either number as the new acceptance baseline.

   Keep 3× as an **explicitly attributed proposed feasibility target**, measured against a fresh baseline on the same code, hardware, inputs and output fidelity. Define preparation as elapsed time from settled geometry to equivalent draw-ready resident artifacts, including remaining CPU work, GPU work, allocation, uploads and synchronization. Require it at both spruce-80 seeds; report the other fixtures and parameter-to-completed-frame time separately. A miss should end the probe with its profile before integration.

   Five omissions matter:

   - **Deletion before porting.** [Preparation currently discards ring positions after leaf seating and then builds the curve](/home/daniel/Projects/telperion/.worktrees/fine/crates/telperion-render/src/generation/preparation.rs:258). Rings and curves both perform path sampling and framing. Measure sharing that preparation, and emitting contact geometry only where consumed, before committing to three GPU equivalents.
   - **Leaf culling.** Fn-198 R1 removes the foliage cull. Today `place.wgsl` performs shell tests while computing bounds, ranks and compaction. Remove the obsolete rejection policy on both executors; preserve whatever bounds, packing and other filtering remain necessary. Count surviving consumers before retaining the scan and scatter work.
   - **Measured envelope.** Fine nodes on the device must contribute to the whole-tree measurements. Reduce them there and combine with CPU geometry through bounded summaries. Measuring only the coarse structure is wrong. The inspected fn-198 `grown_envelope` still inherits shoulder, irregularity and lobe scale from the family, despite decision 11 requiring measurement. Name where that unfinished work lands.
   - **Wood touch points and complete curves.** Positions alone do not establish correctness. Include contact ranges, neighbouring segments, fork/socket geometry and continuity across the partition. Curves also require runs, clusters, bounds, radius extrema and error ladders. Compare final seated leaves and surfaced wood against the CPU reference; do not spend the full tolerance independently at each intermediate stage.
   - **Memory and browser execution.** Avoiding CPU fine nodes does not prove browser fit. Count overlapping growth scratch, pooled buffers, descriptors, contact vertices, curves, leaf-placement scratch, final leaves and the previous tree. Check individual buffer/binding limits as well as total memory. The existing fine driver’s blocking waits and native threads make async WebGPU integration substantive work.

4. **I would revise the requirements into these concrete deliverables.**

   - A checked input/output contract covering identities, partition boundaries, contacts, envelope reductions and buffer ownership.
   - A deletion audit with measured costs, followed by the native vertical probe and precisely defined 3× gate.
   - Final-output parity across the fixture matrix, including contacts and partition seams, plus named failures without fallback.
   - Browser acceptance covering async completion, allocation limits, device errors and repeated large-to-small rebuilds. Replace “should fit” with a measured pass criterion.
   - Separate reporting of build latency and steady-frame cost.

   The last distinction matters to the original realtime question. GPU-PATH measured spruce-80 at **58.2 ms per steady frame**, including **38.68 ms in vegetation**. Faster dressing does not remove that drawing cost. Fn-212 can make rebuilding practical and unblock browser residency; it cannot by itself establish realtime rendering.

I reviewed source and recorded evidence only. I did not rerun benchmarks, inspect stills or modify files. The local FINE-CONTRACT contains numbered decisions through 8; I could not verify the requested decision 9 there.