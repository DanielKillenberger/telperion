---
name: Telperion
last_updated: 2026-09-05
generator: flow-next-strategy
---

# Telperion Strategy

## Target problem

Game developers need trees that hold up from a close inspection to a forest, with convincing branching and the capacity to grow and respond to their surroundings. The hard part is maintaining a coherent living structure across millions of elements, including trees on Telperion's scale, within a game's generation, memory and frame budgets.

## Our approach

Build toward realistic simulations of procedurally generated trees, using one persistent botanical structure from trunk to leaf-bearing twig, with growth, surroundings and damage shaping its lifecycle. Parameters define a family, species presets provide named botanical traits, and the seed selects a reproducible specimen; a shared parametric field supplies supernatural character. The project mantra is "Minimalist af, efficient af and beautiful", expressed in lean, elegant code with explicit data flow, small interfaces and abstractions earned by concrete needs, so botanical rules remain easy to change and grow. Generate only the detail and output representation the consuming engine needs, and judge each advance through measured runtime costs and visual evidence.

## Who it's for

**Primary:** Game and real-time 3D developers — they're hiring Telperion to put trees in a scene that branch the way a real tree does, from a hero tree to a whole forest of a species, generated from a seed instead of bought as baked assets.

**Secondary:** The owner, judging the Two Trees in clay — the first user, and the one whose eye every preset is held to.

## Key metrics

- **Fidelity** — the leaf is a botanical multiple of the twig it hangs on, and the tree carries the leaf count its size implies (10^5 to 10^7 for the Two Trees); measured in the unit tests on both presets.
- **Frame** — a shipped hero tree renders inside 2 ms of GPU time at native pixel ratio on the named machine, an RTX 3080, and the whole vegetation layer of a thousand-tree forest inside 4 ms once a forest rig exists to measure it; GPU timer queries in the harness rig, budgeting a slice of the game frame for vegetation.
- **Build** — time from a dial move to a finished tree, held to whatever keeps dragging usable; the harness's own build timer.
- **Attributability** — same seed and parameters give a byte-identical tree, and every change to a preset traces to a named dial; asserted in the unit tests.
- **The owner's eye** — Telperion reads as Telperion and Laurelin as Laurelin in clay with every supernatural term at its preset value; judged in the harness, recorded in the spec.

## Tracks

### Growth and botanical fidelity

Species anatomy, branching and lifecycle behaviour form one continuous tree structure, judged against real trees as well as the Two Trees.

_Why it serves the approach:_ reference-based visual QA exposes structural mistakes that counts and continuity tests alone cannot catch, guiding the path toward realistic simulation.

### The supernatural field

A shared parametric field gives botanically grounded trees their authored supernatural character, with Telperion and Laurelin as the first demanding specimens.

_Why it serves the approach:_ the whole tree responds to the same authored rules, keeping its form reproducible and its controls understandable.

### The core and integration

One lean Rust generation core serves browser Wasm and native integrations, with small boundaries between tree state, generation rules and output representations. Consumers request structure, meshes, instances or spatial fields as needed; a block world can sample wood and foliage without paying to construct a surface mesh. The migration replaces the young TypeScript core without a backwards-compatibility requirement, while latency, memory and binding costs remain measured obligations.

_Why it serves the approach:_ an engine-independent tree state gives simulation and integration room to grow while keeping each representation optional.

### Surface and rendering at scale

Continuous surfaces, natural forks and tips, foliage and bark make the underlying structure legible from close views to the canopy. Hierarchy and level of detail keep generation, updates and rendering within game budgets as scenes grow from a hero tree to a forest, with native game integration as the practical test.

_Why it serves the approach:_ visual fidelity and measured runtime cost jointly determine whether a generated tree belongs in a real-time world.
