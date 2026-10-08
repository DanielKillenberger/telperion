# Tree space: fine geometry dressed on the device

## Goal & Context

Owner, 2026-10-07: "Is realtime perf feasible? are we stuck in any way?" and "yes" to starting this spec. The engine's trees carry 7-48 times the nodes of the shipped presets (spruce at 80: 4.6M against 96k), and every CPU preparation pass after stage 2 (rings, stations, curve points) costs about 0.2-0.26 µs a node (fn-209 GPU-PATH.md): about 3.9 s for the spruce at 80, and its 3.5 GB of CPU memory before the GPU breaks the browser's 4 GB ceiling (fn-198 BROWSER.md). fn-209 grows, sheds and lays every fine root (up to about 1,024 expected kept phytomers; 95-99 % of a tree's nodes) on the GPU, identical by lineage to the CPU reference, its laid geometry left in device buffers (fn-209 host decisions 5 and 6: no per-node readback). Nothing after stage 2 reads those buffers yet (fn-198 host decision 16), so the GPU's speed does not reach a drawn tree.

This spec makes the fine roots' laid geometry reach the frame on the device: the preparation the CPU does today for a node (its ring words, its leaf stations, its F3 curve points) is done on the GPU for every fine-root node, written into the buffers the existing GPU stages (expansion, cull, placement, curve surfacing) read, while the structure above the cut (1-5 % of nodes) keeps the CPU path. One pipeline: the CPU reference executor still computes the same preparation for every node and defines correct (fn-125's principle); the GPU executor agrees within fn-125's tolerance.

## Relation to open specs (owner to decide)

- **fn-125** (the GPU expansion executor, station sources from today's grower) is rewritten to the engine only once F4 is accepted (fn-198 R5, owner 2026-10-05). This spec does not rewrite it: it produces, on the device, the per-node inputs the expansion consumes, in the layout fn-198's conversion defines today. If the owner prefers, this spec is folded into fn-125's rewrite instead.
- **fn-171** (wood ring preparation on the GPU, probe first) computes ring words on the GPU for today's presets. This spec's ring step is the same computation for fine-root nodes; it supersedes fn-171 for engine trees, or fn-171 lands first and this spec reuses it (owner's call).
- **fn-176** (whole-tree level of detail) orders data for prefixes; unaffected, later.

## Requirements (draft)

- **R1 (probe first):** ring words, leaf stations and curve points for every fine-root node computed on the GPU from fn-209's device buffers, against the CPU reference within fn-125's tolerance, for beech and spruce at 40 and 80, seeds 1 and 7; the first measurement is the stage-3 CPU preparation it removes, by GPU-PATH.md's method. The probe passes only when the spruce at 80's preparation falls by at least 3x end to end.
- **R2:** the pipeline's stage 3 reads fine roots from the device on the GPU executor (natively), the structure above the cut from the CPU; a drawn spruce and beech at 80 within the tolerance of the CPU reference build; no per-node readback.
- **R3:** the browser: the same path through WebGPU in the harness (`setTreeGpu`), async, with the memory ceiling re-measured (the spruce at 80 should fit when fine nodes never reach the CPU).
- **R4:** timings from a parameter change to a drawn tree in the harness and natively, before and after; screenshots viewed.
- **R5:** workspace gate, npm test, Codex review.

## Boundaries

Depends on fn-209 (the device-resident fine roots) and fn-198 (the engine as stage 2). The oak waits on the owner's decision on its GPU arithmetic (fn-211 task 5). Not fn-176's level of detail.

## After Astra's review (2026-10-07, fn-209 evidence ASTRA-REVIEW-READERS.md): accept with changes, all taken

These replace R1 to R4 above; R5 stands.

- **R1 (contract):** a checked input/output contract: botanical identities (fn-209 decision 2: the converter's merges and radius cutoffs become draw-time choices, resolved before the probe's reference is frozen), the partition boundary including ground-routed roots, wood contacts across the seam, the whole-tree envelope reduced on the device from fine nodes and combined with the CPU's (closing fn-198 decision 11's shoulder, irregularity and lobe scale), and buffer ownership. The share of nodes on the device measured from the realized executor's final routing.
- **R2 (delete first):** an audit with measured costs of what can disappear before porting: rings and curves share path sampling and framing; contact geometry only where consumed; the foliage cull's rejection policy removed on both executors (fn-198 R1), keeping only the bounds, packing and filtering a consumer reads.
- **R3 (native vertical probe, gate):** from settled geometry to equivalent draw-ready resident artifacts (remaining CPU work, GPU work, allocation, uploads, synchronization), against a fresh baseline on the same code, hardware, inputs and fidelity; the proposed 3x (host's feasibility target) at both spruce-80 seeds; a miss ends the probe with its profile. Final-output parity (seated leaves, surfaced wood, contacts, partition seams) against the CPU reference, the tolerance spent once on the final output.
- **R4 (browser):** async completion, allocation and binding limits, device errors, repeated large-to-small rebuilds; a measured pass criterion for the spruce at 80, not "should fit".
- **R5a:** build latency and steady-frame cost reported apart. The spruce at 80 draws at 58.2 ms a frame (38.7 ms vegetation, GPU-PATH.md); realtime drawing of mature trees needs fn-176's level of detail, which consumes F3's curve data and preserves identities.
- **Relations:** fn-212 stays separate; fn-125 and fn-171 are reconciled after F4's acceptance (fn-198 R5); fn-171 does not land first; its obligations (sequential-frame timing, contact correctness, precision errors, measured gain) move here; one ring implementation.
