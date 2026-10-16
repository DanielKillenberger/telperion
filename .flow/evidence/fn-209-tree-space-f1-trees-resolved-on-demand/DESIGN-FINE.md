# Fine orders grown in the expansion stage (host design draft, 2026-10-06)

Status: draft for Astra's review; the cut level waits on SPLIT.md.

## Why

GPU-PATH.md: the shipped presets reach a first frame in 153-174 ms through the harness's GPU path because the CPU handles about 96k nodes; the tree-space engine's trees have 7-48 times more (spruce at 80: 4.64M), and every CPU pass costs 0.2-0.26 µs a node (rings, stations, curve), on top of growth (spruce 3.3 s, oak 5.6 s at 80) and 3.5 GB of CPU memory for the spruce. The old pipeline's speed is its split: a coarse structure on the CPU, fine detail expanded on the GPU. The engine grows every shoot on the CPU. Owner goals: realtime, never baked; near-instant retuning; one pipeline (`docs/pipeline.md`); every parameter by degree (`docs/principles.md`).

## The design

**1. One cut, one rule.** A bud whose expected kept subtree is below a cut is a *fine root*. The cut is a measured quantity (SPLIT.md), not a species value, and is the same rule F1's stand-ins use: a light-neutral predictor (DESIGN-LIGHT.md's classification) so which buds are fine never depends on what they grow into.

**2. The CPU grows the structure above the cut** with the engine as it is, and records each fine root's boundary: lineage key, PA, birth cycle, frame, base scale, committed section and load (option 4's contract), and for light-reading species its recorded light history (DESIGN-LIGHT.md section 5; P4 measured one read per fine subtree adequate within a subtree at 0.5-1 m).

**3. The expansion stage grows each fine root** by the engine's own chain rules (P2's grower: grow and shed alone from the committed boundary, 35-78 µs a chain on the CPU), then lays, thickens and dresses it, inside fn-125's expansion with its two executors: the CPU reference executor runs the same steps as the shaders and defines correct. Fine roots are independent work items, thousands per tree.

**4. Determinism.** Draws are lineage-keyed integer hashes, identical on both executors. Every discrete decision (a unit's node count, abortion, a stop outcome, shedding) is taken in integer or exactly representable arithmetic so f32 against f64 cannot flip topology; continuous geometry is compared within fn-125's screen (50 µm max, 10 µm RMS). This is the risk fn-12 failed on (its field queries failed contact precision), and the first thing the probe tests.

**5. Order and camera.** The cut is camera-independent, so the drawn tree is the same at every view; fn-176's level of detail chooses what to draw of it, and F1's on-demand resolution becomes "expand the fine roots in view first".

## What it changes

- Growth on the CPU covers only the structure above the cut; light, allocation and balance (the oak's yearly passes) run over that structure, with fine roots as committed aggregates (DESIGN-LIGHT.md section 6 and Astra's change 1).
- CPU preparation, conversion and memory scale with the structure, not the tree.
- One pipeline stays one: the engine's fine growth is a stage of the expansion, never a second chain.

## Open, for the probe

1. The cut (SPLIT.md): the CPU's remaining share and the number of fine roots.
2. Topology equality between executors on every species, seeds 1 and 7, ages 40 and 80.
3. The GPU expansion's time per fine root and for the whole tree, cold, including upload of boundaries and readback where a consumer needs it.
4. What fine subtrees lose against today's whole-tree growth: sag of a fine subtree under its own load only (its bearer's sag is above the cut), and shedding by committed extinction (P2: beech order 4+ about 10 % short at 80).

## The cut, from SPLIT.md (2026-10-06)

Cut by kept phytomers, age 80, seed 1 (seed 7 within about 2 points): at N = 1024 the CPU keeps spruce about 1.1 % of nodes (about 51k, 72k fine roots), beech 5.2 % (79k, 7.3k roots), oak 4.1 % (36k, 4.4k roots); below every shipped preset's whole node count (96-213k), where the GPU path reaches a first frame in 153-174 ms. The spruce has a natural break at 256 (whole order-2 shoots); beech and oak have none. One cut, N = 1024, serves all three. The finest PA alone holds 45-58 % of every tree's nodes.

The cut is by count, not reach: a short reach does not bound an item's work (spruce subtrees within 0.25 m hold up to 16,654 phytomers). For the design's rule (classification before growth), the predictor is the closed form's expected kept phytomers per bud, light-neutral; SPLIT.md measured realized counts, so the predictor's agreement with them is a probe item. Item sizes at N = 1024: p50 65-94, p95 100-716, max about 1k phytomers.
