---
title: Parallel work alongside FN-6
date: "2026-09-05"
focus_hint: Parallel work alongside FN-6
volume: 15
survivor_count: 5
rejected_count: 10
rejection_rate: 0.67
artifact_id: parallel-work-alongside-fn-6-2026-09-05
promoted_ideas: [1]
promoted_to: {"1": [fn-7-rust-surface-benchmark-prototype]}
status: active
---

## Focus

Parallel work alongside FN-6

## Grounding snapshot

git_log_30d: active branching and test-budget changes
recent: fn-6.1 branch law and fill baseline completed
active: fn-6.2 and fn-6.3 combined gate
active_paths: src/skeleton, src/radius.ts, src/presets, src/index.ts
active_paths: harness/skeleton-view.ts, harness/params.ts
later_fn6_paths: src/canopy/place.ts, harness/stage.ts, README.md
surface: renderer-independent positions and indices output
surface_cost: millions of triangles; number arrays converted to typed arrays
open_specs:
  - fn-1-the-canopy-real-leaf-geometry-culled-to: The canopy: real leaf geometry, culled to a shell, in one instanced draw
  - fn-4-limbs-that-pass-through-each-other-and: Limbs that pass through each other, and the blunt ends they stop at
  - fn-5-branch-until-the-tips-bear-leaves-one: Branch until the tips bear leaves: one recursion, trunk to twig
  - fn-6-branch-generations-below-the-crossover: Branch generations below the crossover, and one fixed twig
changelog_recent: none (no CHANGELOG.md)
memory_matches: Zero width is not no constraint [envelope,invariants]
memory_audit_stale: none (audit not run)
strategy_name: Telperion
strategy_updated: 2026-09-05
target_problem: Trees in games are baked assets: modelled offline, branching stops a few orders down, and the twigs and leaves are painted cards. Nobody has seen a tree in a game that branches the way a real one does, from trunk to the twig a leaf hangs on, and nobody has seen that done for a tree that is supernatural on purpose. It is hard because a realistic 150 m tree is millions of elements, growth to twig scale is a different method at every scale, and it all has to stay one continuous structure that renders in a frame.
approach: Build living procedural trees for real-time worlds around one persistent branching structure, continuous from trunk to the twig a leaf hangs on, with output representations chosen for the consuming engine. Parameters define a family of trees, with species presets providing named sets of botanical traits; the seed determines an individual specimen, and its growth, surroundings and damage shape what it becomes over its lifecycle. Botanical rules guide its natural form, with supernatural character authored through the shared parametric bias field. Generate deterministically at runtime, proving the approach in the browser, and use the tree's hierarchy to update and render only the detail needed within measured game budgets, from a tree inspected up close to a whole forest.
tracks:
### Growth

The one recursion from trunk to twig: branch generations with real length and laterals, the twig as a fixed botanical anatomy, and continuity asserted across every change of method.

_Why it serves the approach:_ growing rather than modelling is only true if the structure is one structure all the way down.

### The supernatural field

The bias terms, the presets that state them, and the Two Trees they are judged on.

_Why it serves the approach:_ the magic has to be a parameter the whole tree obeys, or it is a hand edit like everyone else's.

### Rendering at scale

Twigs as instanced elements, a LOD ladder from full geometry to impostor, GPU or worker generation, species grown as a few dozen archetypes and instanced into forests, and export to engines. Millions of elements stay inside the frame, the build stays draggable, and a forest is a placement problem rather than a growth problem. The first proof of the forest path is a Valheim mod: biomes configured as species presets, trees spawned from seeds and grown by Telperion, in Valheim's own style, taking that world's procedural generation to the next level.

The generation and lifecycle engine should share an engine-independent core across browser and native-game bindings, with Rust the leading candidate subject to a prototype demonstrating gains in end-to-end latency and peak memory against optimized TypeScript, including binding and transfer costs.

Integration adapters should be able to request branch structure, surface meshes, instances or spatial fields describing wood and foliage at the resolution their engine needs, generating only the requested representation. A Minecraft adapter could turn field samples into blocks while a mesh renderer requests surfaces from the same tree state, with lifecycle changes exposed so either adapter can update affected regions.

_Why it serves the approach:_ runtime generation is a claim about the frame and the build, and this track is where that claim is paid for, at one tree and at a thousand.

### The skin

The swept surface, forks, interpenetration and tips, and eventually bark and leaf texturing.

_Why it serves the approach:_ a grown skeleton is judged through its surface, and the surface is where a real tree and a pile of tubes part ways.

## Survivors

### High leverage (1-3)

#### 1. Rust surface benchmark prototype
**Summary:** Compare release Wasm and native meshing against TypeScript using frozen skeleton/radius fixtures.
**Leverage:** Small-diff lever because the experiment consumes frozen inputs outside the production pipeline; impact lands on the Rust adoption decision.
**Size:** L
**Affected areas:** experiments/rust-surface
**Risk notes:** Binding and transfer costs may dominate; production integration waits.
**Persona:** senior-maintainer
**Next step:** /flow-next:interview

#### 2. Spatial field and block-output prototype
**Summary:** Sample wood occupancy and foliage density from synthetic trees without constructing a surface.
**Leverage:** Small-diff lever because a synthetic-tree sampler needs no growth or mesh integration; impact lands on field and block consumers.
**Size:** M
**Affected areas:** experiments/tree-field
**Risk notes:** Cell resolution must preserve thick wood connectivity; no Minecraft runtime integration yet.
**Persona:** first-time-user
**Next step:** /flow-next:interview

#### 3. Consumer output contract examples
**Summary:** Specify optional outputs, units, buffer ownership and region updates using mock consumers.
**Leverage:** Small-diff lever because mock adapters document a boundary without changing public APIs; impact lands on browser and native integrations.
**Size:** S
**Affected areas:** docs/integration-contract.md
**Risk notes:** Keep illustrative and provisional until FN-6 settles; no public API changes.
**Persona:** senior-maintainer
**Next step:** /flow-next:interview

### Worth considering (4-7)

#### 4. Lifecycle continuity decision experiment
**Summary:** Use tiny synthetic trees to compare persistent branch identities and radius changes across age samples.
**Leverage:** Small-diff lever because tiny lifecycle fixtures isolate identity and radius assumptions; impact lands on future growth contracts.
**Size:** M
**Affected areas:** experiments/lifecycle-contract
**Risk notes:** Tests assumptions only; does not implement FN-7 or alter growth.
**Persona:** senior-maintainer
**Next step:** /flow-next:interview

#### 5. Species reference fixtures
**Summary:** Organize sourced species traits and visual references into data for later preset validation.
**Leverage:** Small-diff lever because reference data can be collected without modifying presets; impact lands on later species validation.
**Size:** S
**Affected areas:** research/species
**Risk notes:** Local evidence may be sparse; production preset tuning waits for FN-6.
**Persona:** first-time-user
**Next step:** /flow-next:interview

### If you have the time (8+)

_(none)_

## Rejected

- Replace the entire engine with Rust — out-of-scope-vs-strategy: A complete Rust migration bypasses the required measured prototype gate and overlaps changing modules.
- Implement live growth now — too-large: Production lifecycle growth overlaps FN-6 and lacks settled structural contracts.
- Ship a Minecraft mod now — too-large: A playable mod combines unresolved fields, bindings and runtime integration.
- Retune the Two Trees now — duplicates-open-epic: FN-6 owns preset tuning and acceptance.
- Add the native-resolution GPU sweep — duplicates-open-epic: FN-6.7 already owns the native-resolution sweep.
- Rebuild branch junction surfaces — duplicates-open-epic: Junction and interpenetration changes belong to open FN-4.
- Implement full forest LOD backend — too-large: The combined forest renderer depends on unfinished twig representation and active harness code.
- Add a production oak preset — other: A shipped species preset depends on the schema and radius rules changing under FN-6.
- Rewrite foliage placement for per-leaf seeds — other: FN-6.5 owns canopy placement changes; a concurrent RNG rewrite would collide.
- Add mobile application packaging — out-of-scope-vs-strategy: No demonstrated mobile-app consumer need in the strategy.
