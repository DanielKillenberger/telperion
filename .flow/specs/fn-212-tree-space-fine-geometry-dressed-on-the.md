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
