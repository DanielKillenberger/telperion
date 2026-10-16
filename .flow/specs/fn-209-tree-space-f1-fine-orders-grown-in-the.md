# Tree space F1: fine orders grown in the expansion stage

## Goal & Context

Realtime generation, never baked offline (owner, 2026-10-05); retuning a tree in the harness near-instant (owner, 2026-10-06: "looking at trees in the harness and adjusting params should yield near instant results. That should really be the goal"; STRATEGY.md, Build track); and many trees in a game (owner, 2026-10-06: "we need to be able to generate many of these in a game environment right?"). The owner accepted progressive refinement as the mechanism (2026-10-06, "aight") and the integration of the engine into the existing GPU path (2026-10-06: "it should obviously be integrated into the fast gpu path that the existing pipeline uses").

**Rescoped 2026-10-06 (owner: "aye").** The first design, trees resolved on demand by the camera chain by chain (decisions 1 to 14, PROPOSAL.md), was probed and does not reach the goal for a near tree; its record stays in the evidence folder. What the probes established:

- **The cost is node count, not unseen detail.** The engine's trees carry 7 to 48 times the nodes of the shipped presets (spruce at 80: 4.64M against 96k), and every CPU stage costs about 0.2 µs a node (GPU-PATH.md). The shipped presets reach a first frame through the harness's GPU path in 153 to 174 ms because the CPU holds a coarse structure and the GPU expands the detail; the engine grows every shoot on the CPU (BASELINE.md: 2.2 to 8.2 s from parameters to first frame at age 80).
- **A near tree needs most of its detail at 40 m** under every rule tried (P3 rounds 1 to 3: 54 to 98 % resolved), so camera-driven laziness does not shorten a retune.
- **A chain can be grown and shed alone** from its committed boundary (P2: 35 to 78 µs a chain, GO for beech, spruce and palm); junctions committed by expectation never pop and a streamed tree equals a one-pass tree (option 4, the competition).
- **The closed-form tables cost 30 to 90 ms per species point** at age 80 (P1 rounds 1 to 3, from 9 to 140 s, values unchanged but for sag mass and moment within 0.55 %).
- **The finest orders hold the nodes** (SPLIT.md): with every subtree of at most 1,024 kept phytomers grown elsewhere, the CPU keeps 1.1 % of the spruce's nodes (about 51k), 5.2 % of the beech's, 4.1 % of the oak's.
- **The oak's cost is its yearly whole-tree passes** (P4: re-lays, light, allocation, balance, mostly serial), which no partition removes; that is F2b's work.

## Design (host, 2026-10-06; DESIGN-FINE.md, accepted with changes by Astra, ASTRA-REVIEW-FINE.md)

- **The CPU grows the structure above a cut; each fine root is grown in the pipeline's expansion stage** by the engine's own chain rules, then laid, thickened, ringed, stationed and dressed there, inside fn-125's expansion with its two executors: the CPU reference executor runs the same steps as the shaders and defines correct. One pipeline: the fine growth is a stage of the expansion, never a second chain.
- **Astra's four changes, all taken:**
  1. **A canonical fine-root record and numeric decision contract.** The record carries everything P2's `Bud` carries (birth draws, chain root, horizon, waking state) plus option 4's committed section, mass and first moment, the attachment frame, ancestor survival and termination, dormant and relay history, retained-pipe transfer, and for light-reading species DESIGN-LIGHT.md's history-dependent aggregate state. Every continuous-to-discrete decision (draws, survival and abortion thresholds, fractional cycle times, shedding, and the converter's merge and cull tests) is audited and given a specified deterministic representation, so the executors cannot disagree on topology.
  2. **Work partition separate from botanical approximation.** The cut decides where work runs, not what is computed. Where the two must coincide, DESIGN-LIGHT.md's complementary blending and walk tests apply, so moving the cut does not change the tree beyond the measured approximation.
  3. **Fine preparation stays on the device** through rings, stations, curves, foliage and the frame; nothing per fine node returns to the CPU.
  4. **An end-to-end gate:** a parameter change to a completed frame, including every invalidated preparation (tables, boundaries, aggregates, histories).
- **The cut** is by kept phytomers, chosen before growth by a light-neutral closed-form predictor of the expected kept count; N = 1,024 is the starting point from SPLIT.md. The predictor's agreement with realized sizes, empty roots and the work inside an item (generated organs, chains, cycles, memory) are measured, with bounded execution and an overflow rule that preserves the botanical result.

## Requirements

- **R1 (contract probe):** small adversarial cases (fractional lifespans, threshold-adjacent probabilities, relay and dormant buds, ancestor shedding, changing light, retained pipes, a loaded junction) give identical topology by lineage on both executors, then geometry and contacts within fn-125's screen; repeated with different item order, chunking and partition.
- **R2 (throughput):** beech, spruce and lit oak at ages 40 and 80, seeds 1 and 7, classified prospectively: every stage timed to a completed frame, cold and repeated, with growth-parameter edits; worst-item latency; memory.
- **R3 (fidelity):** the fine-root contract against whole-tree growth: kept phytomers per order, per-PA growth drift under 2 % (decision 4), junction radius, stills of the four species beside their passed sheets, judged by the host and the owner.
- **R4:** the age walk (owner, 2026-10-06, the age slider): a tree at age a + 1 continues the tree at age a; persistent lineages, no jumps.
- **R5:** workspace gate; Codex review.

Three comparisons stay distinct (Astra): CPU against GPU under the contract (R1); the contract against whole-tree growth (R3); the timing (R2). Passing one does not establish another.

## Boundaries

Depends on fn-206; amends fn-125's boundary that growth stays on the CPU (recorded in fn-125 when F4 is accepted, as fn-198's R5 says). The engine's integration into the pipeline and harness is F4 (fn-198). The coarse growth above the cut, the oak's yearly passes above all, is F2b. A spruce frame at 58 ms is fn-176's level of detail. Cheaper stand-ins for view-dependent resolution are a later spec (DESIGN-RULE.md, outcome).

Evidence for this spec, before and after the rescope, lives in `.flow/evidence/fn-209-tree-space-f1-trees-resolved-on-demand/` (the spec's id before its retitle).

## Closed as superseded (host, 2026-10-16)

Astra's release review (2026-10-16, `.flow/evidence/fn-237-release-the-tree-space-engine-replaces/ASTRA-RELEASE-REVIEW.md`): the old CPU-above-the-cut, GPU-fine-root design is superseded by fn-255/256's complete cycle; its measurements stand and any live performance obligation moves to fn-285. Its requirements are not implemented and are not to be executed.
