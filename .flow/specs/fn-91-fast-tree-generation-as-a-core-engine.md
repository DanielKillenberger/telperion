# Fast tree generation as a core engine capability

## Goal
Fast generation is a core requirement for every Telperion use case: bring mature-tree generation toward near-instant while preserving perceived fidelity, in shared generation and rendering boundaries serving browser and native consumers alike. Species rotation with random seeds on killenberger.com was the immediate use case, not the scope. Measure first; try compact structural inputs expanded on the GPU and consumed directly by the renderer, avoiding fn-12's CPU-build/upload/query/readback dead end; target 10x warm end-to-end on agreed desktop fixtures, 100 ms as a stretch.

## Decisions
- > user: "is there a path to near instant generation? I thought that a lot of gen time goes to mesh generation and leaf placement. If we're placing leaves 1 by 1 in series surely that must be optimizable. Ideally we could do it all in parallel on a gpu. That'd make it almost instant?"
- > user: "we have previously tried to optimize using gpu but couldn't make it can you check the learnings there and make sure we're not running against a wall again."
- > user: "The generator won't be locked in for a while, while we develop it further."
- > user: "yea i'm not saying never check for byte equivalence where it's useful but as we add more improvements we consciously want to improve the structure of the bytes over time."
- > user: "and if we have performance improvements that can't be done byte equivalent but have no perceivable visual regression that's fine too for example"
- > user: "it's not just for the website obviously" / "this is crucial for all usecases"
- > user: "i mean IF we can get the same byte identical tree for perf improvements that would make verification easier."
- > user: "so given the approaches we're taking in the spec what is a reasonable target to set? aim high" -> 10x accepted: "ok $flow-next-flow"
- Fixture matrix (accepted): mature oak and spruce at seeds 1 and 7, native CPU output and browser delivery, recorded desktop hardware; 10x warm target, 100 ms stretch, no increase in accounted peak memory.
- Owner, 2026-09-20, native seed-1 CPU/GPU hero images (oak and spruce, 1280x720, first native GPU foliage candidate): "they look the same to me". Scope: accepts no visible regression in those four views only; not attachment close-up parity, other seeds, other species, browser or phone behaviour; does not close fn-91 (source: archived OWNER-VISUAL.md).
- Owner, 2026-09-20, seed-1 browser close views: assessed as looking exactly the same; after the decoded RGBA pixel comparison (browser oak close 0.50%, spruce close 0.65%, native oak hero 0.37%, native spruce hero 1.09% of pixels differ): "ok to me that's fine". Scope: accepts the observed differences in the shown CPU/GPU pairs; not byte equality, geometry, motion, other seeds/devices, performance or memory (source: archived OWNER-VISUAL.md). Exact-output follow-ups reuse this evidence.
- Owner, 2026-09-20, resident GPU wood captures (shaded lower-trunk views, oak and spruce seed 1), rejected as unjudgeable: "i can't see shit from these renders. I'm in the needles of the spruce and everything is in shadow" (source: archived resident-wood/visual/VERDICT.md).
- Owner, 2026-09-20, replacement lit exterior bare-wood CPU/GPU comparison, accepted: "looks identical. Why do you butter me about 12 px difference mate". Scope: this candidate and these views; tiny pixel counts remain technical evidence and do not trigger another owner confirmation (source: archived resident-wood/visual-lit/VERDICT.md).
- Owner authorized continued work on fn-91; a finished intermediate task is not itself a stopping condition (source: archived GPU-POSITION-BOUNDARY.md).
- Owner challenged the missing finish boundary after successive bounded experiments ("when the spec would finish"); the host limited the run to the claimed .13 experiment plus validation and a completion assessment; any change to acceptance remains the owner's decision (source: archived FRICTION.md).
- Owner approved the demo and requested PR finalization; renderer defaults became sun elevation 30° and camera azimuth 115° (source: archived FINAL-VALIDATION.md).
- Owner, 2026-09-20: reaching 10x on both oak browser completed-frame fixtures (at or below 157.0/186.25 ms) is the stopping point if a concrete path exists.
- Owner, 2026-09-27: closed at the reached milestone; oak seed 1's 1.1 ms short of strict 10x accepted; CPU-owned output speed moves to fn-126, growth speed to fn-172 to fn-175; the memory non-increase claim and the cold, phone and full-memory qualification are dropped, not carried.

## Acceptance Criteria
- **R1:** Fresh baseline and stage breakdown for browser and native paths, desktop and phone, cold and warm, memory — partial: desktop native and browser baselines recorded; phone, full cold start and complete memory domains dropped (owner, 2026-09-27).
- **R2:** A first bounded GPU candidate replaces CPU expansion and keeps buffers resident, avoiding fn-12's path — met.
- **R3:** Byte identity preferred, not required; no perceptible regression; owner verdict recorded — met for the views the owner judged.
- **R4:** Botanical plausibility, attachment, valid geometry and repeatability preserved — met (an isolated preflight-normal rejection fixture stays unproven).
- **R5:** Integrated into the shared path for browser and native; 10x warm on each desktop fixture, no peak-memory increase — partial: browser oak 9.93x/10.30x, spruce 42.64x/44.97x; memory claim dropped.

## Result
Browser completed-frame medians (initialised renderer) went from 1,570.0-7,628.6 ms to oak 158.1/180.8 ms and spruce 178.9/164.2 ms (seeds 1/7). Retained changes, by task: bounded leaf readback (.2), resident GPU wood expansion (.4), nested admission traversal (.5), shared canonical contact rings (.6), GPU positions in the qualified domain with canonical fallback (.7, .8), parallel native CPU surface expansion, bitwise identical and 2.57-2.70x faster (.13), shared station streaming with zero-contact overlap (.14), and the interactive harness building through `setTreeGpu` (.15). Screened and reverted: radius-only ordering (.10), inactive growth-bias specialization (.11), prepared envelope admission (.12). Station records are byte-identical in the mature matrix; oak seed 7's Wasm linear-memory high-water rose 26,279,936 bytes while seed 1 fell 24,641,536. PR #50.

## Open
CPU-owned output speed (fn-126); growth speed (fn-172 to fn-175).

## Friction
Host review (2026-09-20) dispositions: reuse the retained baseline stage runner; benchmark queue fence added; iterate on the non-LTO `ci` profile and keep release for qualification (later the gate); run inventory and architecture checks before expensive suites; explicit nextest targets; navigation guard and canvas-settle step in the capture harness; proposed caller-selected test artifact output so tests stop overwriting receipts, and a typed benchmark result schema; Vulkan instance crash left to fn-92; missing nextest, GNU time and image opener were local setup.

## Evidence
- `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/README.md` (index and recovery of the archived raw runs)
- `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/READBACK.md`, `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/resident-wood/REPORT.md`, `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/shared-rings/REPORT.md`, `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/position-integration/REPORT.md`, `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/stations/REPORT.md` (cited by `scripts/benchmarks/generation.md` and README)
