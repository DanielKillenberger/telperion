---
name: Telperion
last_updated: 2026-09-10
generator: flow-next-strategy
---

# Telperion Strategy

## Target problem

Game developers need trees that hold up from a close inspection to a forest, with convincing branching and the capacity to grow and respond to their surroundings. The hard part is maintaining a coherent living structure across millions of elements, including trees on Telperion's scale, within a game's generation, memory and frame budgets.

## Our approach

Build toward realistic simulations of procedurally generated trees, using one persistent botanical structure from trunk to leaf-bearing twig, with growth, surroundings and damage shaping its lifecycle. The generator is one continuous tree space: every generator parameter is a numeric trait that acts on every tree, a template is just a point in that multi-dimensional space, the seed selects a reproducible specimen, and a shared parametric field supplies supernatural character, so smooth interpolation between any kind of tree is a standing requirement and no family field is a switch between ways of building. All rendering techniques must generalize across generated trees and leaves through shared geometry and data contracts, so changing supported generator parameters or adding a template requires no renderer code changes. The project mantra is "Minimalist af, efficient af and beautiful": use lean code, explicit data flow and small interfaces, generate only the detail the consuming engine needs, and judge each advance through measured runtime costs and visual evidence.

## Who it's for

**Primary:** Game and real-time 3D developers — they're hiring Telperion to put trees in a scene that branch the way a real tree does, from a hero tree to a whole forest of a species, generated from a seed instead of bought as baked assets.

**Secondary:** The owner, judging the Two Trees as they are lit — the first user, and the one whose eye every preset is held to.

## Key metrics

- **Fidelity** — the leaf is a botanical multiple of the twig it hangs on, and the tree carries the leaf count its size implies (10^5 to 10^7 for the Two Trees); measured in the unit tests on both presets.
- **Frame** — a shipped hero tree renders inside 2 ms of GPU time at native pixel ratio on the named machine, an RTX 3080, and the whole vegetation layer of a thousand-tree forest inside 4 ms once a forest rig exists to measure it; GPU timer queries in the harness rig, budgeting a slice of the game frame for vegetation.
- **Build** — time from a dial move to a finished tree, held to whatever keeps dragging usable; the harness's own build timer.
- **Attributability** — same seed and parameters give a byte-identical tree, and every change to a preset traces to a named dial; asserted in the unit tests.
- **The owner's eye** — Telperion reads as Telperion and Laurelin as Laurelin, lit, with every supernatural term and every appearance row at its preset value; judged in the harness, recorded in the spec.

## Tracks

### Growth and botanical fidelity

Species anatomy, branching and lifecycle behaviour form one continuous tree structure, judged against real trees as well as the Two Trees.

_Why it serves the approach:_ reference-based visual QA exposes structural mistakes that counts and continuity tests alone cannot catch, guiding the path toward realistic simulation.

### The supernatural field

A shared parametric field gives botanically grounded trees their authored supernatural character, with Telperion and Laurelin as the first demanding specimens.

_Why it serves the approach:_ the whole tree responds to the same authored rules, keeping its form reproducible and its controls understandable.

### The core and integration

One lean Rust generation core serves browser Wasm and native integrations, with small boundaries between tree state, generation rules and output representations. Consumers request structure, meshes, instances or spatial fields as needed; a voxel world such as Minecraft can sample wood and foliage as a spatial field and stream trees in without paying to construct a surface mesh. Portable tree and rendering data stay separate from engine-specific drawing and material implementations, keeping future engines able to preserve the same visual fidelity without requiring an external-engine proof in every rendering pass. The Rust wgpu renderer on master is the first consumer of the core's engine-neutral mesh output and the rig that measures the frame metric, in the browser and in a headless native target.

Integration proofs run in sequence. First, textured trees with wind, collision and chopping or destruction in Unreal Engine; then forests of individually generated specimens within measured generation, memory and frame budgets. A Valheim mod is a later candidate proof point, where each world tree has its own reproducible procedural structure and participates in the existing game's interactions, multiplayer and persistence. That experiment follows the Unreal and forest proofs; its game-specific constraints do not drive the immediate core architecture.

_Why it serves the approach:_ an engine-independent tree state keeps every representation optional, and proving functional trees in an integration we control establishes the foundation for adapting the same botanical state to an existing game's constraints.

### Surface and rendering at scale

Continuous surfaces, natural forks and tips, foliage and bark make the underlying structure legible from close views to the canopy. Simplification, aggregation, filtering, detail selection and streaming operate on generated geometry and measured error, without species-specific paths, needle-specific topology assumptions or hand-modelled foliage clusters. Validate this generality early on materially different tree and leaf shapes before committing to a rendering technique; geometry-dependent choices are allowed, while unsupported inputs remain explicit and fallback geometry must still meet the applicable fidelity and performance requirements. Hierarchy and continuous rendering representations keep generation, updates and rendering within game budgets from a hero tree to a forest, without visible stepping, thinning or shimmer; this fidelity direction applies across consuming engines, with concrete integrations validating it when undertaken.

_Why it serves the approach:_ visual fidelity and measured runtime cost jointly determine whether a generated tree belongs in a real-time world.
