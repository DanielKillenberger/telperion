# Tree space F: the engine in production

## Goal & Context

The tree-space engine replaces stage 2 of the one pipeline for every species that passed C to E; today's generator keeps the rest until each passes. The PR carries every passed species for the owner's verdict. [user]

## Programme

This spec is one phase of the tree-space programme (`docs/tree-space.md`, STRATEGY.md track "The tree space"; owner, 2026-10-03). Read that document first: the four claims, the rule "stand on botany, invent the space", the phase table with each phase's gate and wrong-path stop, how a species is judged, and the rules for running unattended. The research behind it is in `.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/` (LITERATURE.md; source texts in the local research worktree `/home/daniel/Projects/telperion/.worktrees/fn-188-research/.firecrawl/lit/`) and `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/` (the eighteen failed probe rounds and why, the model specifications MODEL-OAK.md, MODEL-SPRUCE.md, MODEL-PALM.md, STAGE1-BEECH.md, and today's pinned baseline BASELINE.md, whose raw stills sit in `/home/daniel/Projects/telperion/.worktrees/fn-190/.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/baseline/`). [user, checked]

## Edge Cases & Constraints

- **Stand on botany, invent the space:** botanical mechanisms are reproduced against an oracle before use; external code and data are never copied, only run and compared against. [user]
- **Errors, not fallbacks;** a collapsed tree or wood below the ground is an error. Every setting changes the tree by degree; no switches (`docs/principles.md`). [principles]
- **Judging:** the host views every still first; a species passes only when the host is confident it reads as itself beside its reference photographs and Astra, reading the same sheet independently, agrees. Numbers explain a look and never decide it (memory: judge each generator round by its stills). [user]
- **Unattended:** this spec runs under `/loop /flow-next:flow --auto` overnight. Its gate is evidence, never the owner's sign-off; the owner's verdict comes at the PR in the morning. Nothing merges to master. A wrong-path condition stops with `NEEDS_HUMAN` and a report of what was tried. [user]
- **Agents:** design judgments go to the host (AGENTS.md, "Dispatch and escalation"); friction is written to this spec's FRICTION.md as it happens. [AGENTS.md]

## One pipeline (owner, 2026-10-05)

"In the end we want one pipeline. So if we choose f1-f3 the old one gets removed." F4 ends with today's grower removed: the engine is the skeleton stage's only producer, private to the pipeline, for every shipped preset. There is no temporary exception and no per-preset choice of grower. F4 therefore depends on every shipped preset existing as an engine species: the beech, spruce, oak and palm have passed (fn-193 to fn-196); silver birch, laurelin, telperion and ordinary are their own specs.

Architecture F1 to F3 must fit (host, 2026-10-05; **revised 2026-10-06** after fn-209's rescope and fn-211): the stages and their typed inputs stay. The skeleton stage is the engine: the CPU grows the structure above fn-209's cut, and each fine root (at most about 1,024 expected kept phytomers) is grown in the expansion stage by fn-209's executors, a CPU reference and a GPU executor, year by year in step with the CPU's pass where the species reads light (fn-211, DESIGN-INTERLEAVE.md). Mechanics are realized for every species: each fine root reports its realized totals to its bearer and the CPU settles as today's engine (fn-211's host decision 3); option 4's committed boundaries are not in production. Fine roots' laid geometry stays in device buffers; F4 with fn-171 defines the device-side readers (rings, stations, curves; fn-209 host decision 5). The surface stage's product is F3's curve data, surfaced per frame by the GPU executor, with the CPU wood kept as the reference build. `docs/pipeline.md` and the boundary tests are updated so nothing outside the pipeline grows a tree.

## Overnight scope (owner, 2026-10-06: "pls keep going over night. Try and get this integrated into the main engine and harness"; "yes please keep everything on a branch next to master")

R0 needs every shipped preset as an engine species; silver birch, laurelin, telperion and ordinary are not yet. The integration is built on this branch, never merged: the engine is the pipeline's only skeleton producer for every preset; the four passed presets (beech, spruce, oak, palm) build through it, natively and in the harness; a preset without an engine species fails with a named error, never a fallback to today's grower (no temporary exception, per the section above). In order:

1. **Species as preset values.** The engine's species (`telperion-space` beech, spruce, oak, palm) become value tables the shipped presets carry; the pipeline reads them; no species branch in code.
2. **The engine as stage 2.** `Generator::prepare` grows through the engine (today's whole-tree CPU grower first; fn-209's executors as they land); `tree::convert` leaves the examples and becomes the pipeline's private conversion; the handed-in path of the tree-space examples is closed; presets without an engine species refused by name.
3. **The harness and the Wasm binding.** `setTreeGpu` and the Wasm build reach the engine; the four species open in `npm run dev` at their default ages, with the 4 GB Wasm ceiling measured and stated per species and age (the spruce at 80 peaked at 3.5 GB CPU before the GPU, GPU-PATH.md).
4. **R2 measured** on the branch, and the four species' sheets for R4.
## Acceptance Criteria

- **R0:** Today's grower and its stages (the scaffold's grower, the twig layer, its thickness gate, the tip shoot, girth patches, the shell-based shedding) are removed; every shipped preset builds through the engine; the tree-space examples' handed-in path is closed. [owner]
  - With R0 (host, 2026-10-07, decisions 11a and 12a): the envelope's `irregularity` and `lobeScale` rows are removed from `Envelope` (the engine path holds irregularity at 0 and reads no lobe scale); `Node.stem` becomes a 0-1 value read by degree by its three readers (the grown envelope's crown base, `stem_radius`, `stem_apices`), the trunk ages become a 0-1 weight per age that a preset blend walks linearly, and a walk test across a blend of two presets shows no jump (decision 12's acceptance, moved here from task 6). [host]
- **R1:** The engine inside the pipeline's stage 2 for the passed presets, as values; the twig layer, its thickness gate, the tip shoot, girth patches and the foliage cull are removed for those presets' path (`docs/pipeline.md`). [user]
- **R2:** Growth time per passed preset within 1.5x of today's skeleton time at equal or greater fine wood, against the pinned baseline (BASELINE.md); full build no slower than today; memory reported. [user]
- **R3:** `cargo test --profile ci --workspace --no-fail-fast` and `npm test` green; the Codex review passes; every shipped artifact within its CI size budget. [AGENTS.md]
- **R4:** The PR (per `docs/pr-format.md`) shows every passed species' sheet beside today's and the photographs, for the owner's verdict; it is not merged without it. [user]
- **R5:** Once F4 is accepted, and before it merges, update fn-125 ("The generator hands engines a tree they only draw") to the engine. Keep its principles and checked findings: one expansion algorithm with a GPU executor and a CPU reference, every leaf a pure function of (source, index, seed), no fallbacks, leaflets on the GPU, limb clumping as a reduction pass, numeric domains as named errors, and its removal list. Replace its station sources (twig runs, short-shoot wood chosen by radius, rosette apices: today's grower's constructs) with the engine's leaf-bearing nodes, expanded per resolved axis; hand its wood-ring work to F3 (fn-208); then check fn-126, fn-171 and fn-15 against the rewrite. Not before: fn-125 is rewritten only once the engine's replacement of today's grower is certain. [owner, 2026-10-05]
- **Wrong-path stop:** the speed bar missed after measured optimisation. [user]

## Boundaries

- Only this phase's deliverable; later phases are their own specs. [user]
- Not the add-species pipeline's recalibration (its own spec after F). [user]
