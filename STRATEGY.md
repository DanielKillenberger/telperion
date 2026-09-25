---
name: Telperion
last_updated: 2026-09-25
generator: flow-next-strategy
---

# Telperion Strategy

## Target problem

Game developers need trees that hold up from a close inspection to a forest, with convincing branching and the capacity to grow and respond to their surroundings. The hard part is maintaining a coherent living structure across millions of elements, including trees on Telperion's scale, within a game's generation, memory and frame budgets.

## Our approach

Build toward realistic simulations of procedurally generated trees, using one persistent botanical structure from trunk to leaf-bearing twig, with growth, surroundings and damage shaping its lifecycle. The generator is one continuous tree space: every parameter is a numeric trait defined for every tree, a template is just a point in that multi-dimensional space, age and growth rate are numeric traits too, the seed selects a reproducible specimen, and a shared parametric field supplies supernatural character, so a small change in any parameter makes a small change in the tree and smooth interpolation between any kind of tree is a standing requirement. A parameter may be dormant where the structure it shapes is absent, and it wakes smoothly as that structure appears; its description states where it is dormant. No parameter is a switch between ways of building, and a count steps only by one unit of the structure it counts. All rendering techniques must generalize across generated trees and leaves through shared geometry and data contracts, so changing supported generator parameters or adding a template requires no renderer code changes. Generation is one pipeline (grow, plan, expand, cull, draw) that every tree passes through; each family feature is a term inside a stage, never a route around it. The expansion is one algorithm with two executors, the GPU and a CPU reference that defines correct and serves consumers without one, agreeing within a stated tolerance; an input the pipeline cannot represent is an explicit error, never a fallback path. The project mantra is "Minimalist af, efficient af and beautiful": use lean code, explicit data flow and small interfaces, generate only the detail the consuming engine needs, and judge each advance through measured runtime costs and visual evidence. The same economy governs the work itself: a step that stops work must catch a defect the steps around it cannot. Code guards check every push against these principles (`docs/principles.md`); Jev review of specs and designs is planned in fn-156.

During active development, measured gains in fidelity, generation speed and rendering efficiency take priority over preserving historical generated output. Seeds identify specimens within a stated generator revision and backend; algorithms, random sequences, topology and encodings may change, and blanket byte-identical output across revisions or CPU/GPU backends is not a product requirement. Prefer byte-identical performance improvements when practical because they simplify verification; intentional improvements may still evolve tree structure and data representation. Performance improvements may also change bytes while preserving perceived appearance, provided measured gains come without perceptible visual regression or failure of relevant correctness requirements.

## Who it's for

**Primary:** Game and real-time 3D developers — they're hiring Telperion to put trees in a scene that branch the way a real tree does, from a hero tree to a whole forest of a species, generated from a seed instead of bought as baked assets.

**Secondary:** The owner, judging the Two Trees as they are lit — the first user, and the one whose eye every preset is held to.

## Key metrics

- **Fidelity** — the leaf is a botanical multiple of the twig it hangs on, and the tree carries the leaf count its size implies (10^5 to 10^7 for the Two Trees); measured in the unit tests on both presets.
- **Frame** — a shipped hero tree renders inside 2 ms of GPU time at native pixel ratio on the named machine, an RTX 3080, and the whole vegetation layer of a thousand-tree forest inside 4 ms once a forest rig exists to measure it; GPU timer queries in the harness rig, budgeting a slice of the game frame for vegetation.
- **Build** — fast generation is a core requirement for every use case, including interactive editing, browser presentation and native engine consumption. Measure end-to-end latency from parameters to the requested usable representation, including preparation and transfers, and separate cold initialization from repeated builds; mature-tree workloads are the first proof, not a website-only target. Monthly growth also records per-slice fixed cost beside a mature build, with cost proportional to changed wood as the design target.
- **Attributability** — record generator revision, backend, seed and parameters so a result can be investigated and compared. Use exact byte checks for outputs intended to remain unchanged, and tolerances where appropriate; historical hashes and CPU/GPU byte equality do not veto intentional improvements. Rebaseline changed specimens with botanical, geometric, visual and performance evidence, and keep each preset change attributable to its parameters.
- **The owner's eye** — Telperion reads as Telperion and Laurelin as Laurelin, lit, with every supernatural term and every appearance row at its preset value; judged in the harness, recorded in the spec.

## Tracks

### Growth and botanical fidelity

Species anatomy, branching and lifecycle behaviour form one continuous tree structure, judged against real trees as well as the Two Trees.

_Why it serves the approach:_ reference-based visual QA exposes structural mistakes that counts and continuity tests alone cannot catch, guiding the path toward realistic simulation.

### The catalogue

Every tree species in the world is the long-horizon goal, real and legendary alike. Each species is one spec generated from a template and implemented as a value table by a value-tier model, and the shared parametric field supplies a legendary tree's authored character on top of its real base species, with Telperion and Laurelin as the first demanding specimens. The generator's coverage is measured against the 23 Hallé and Oldeman architectural models, and a species that exposes an unsupported form or organ becomes a generator spec for the frontier tier.

_Why it serves the approach:_ a template is a point in one continuous tree space, so every species added is evidence that the space is complete, every gap found names the next generator spec, and the whole tree still answers to the same authored rules; the owner's eye is spent on the final round of each species and the value tier spends the rest.

### The core and integration

One lean Rust generation core serves browser Wasm and native integrations, with small boundaries between tree state, generation rules and output representations. Consumers request structure, meshes, instances or spatial fields as needed; a voxel world such as Minecraft can sample wood and foliage as a spatial field and stream trees in without paying to construct a surface mesh. Portable tree and rendering data stay separate from engine-specific drawing and material implementations, keeping future engines able to preserve the same visual fidelity without requiring an external-engine proof in every rendering pass. The generator owns the expansion from plan to drawn geometry as a contract: a versioned plan layout, the portable shaders that expand it, and the CPU reference. The Rust wgpu renderer on master is its first consumer and the rig that measures the frame metric, in the browser and in a headless native target; engines consume the same contract, and none needs to materialize or read back a full CPU mesh.

Integration proofs run in sequence. First, textured trees with wind, collision and chopping or destruction in Unreal Engine; then forests of individually generated specimens within measured generation, memory and frame budgets. A Valheim mod is a later candidate proof point, where each world tree has its own reproducible procedural structure and participates in the existing game's interactions, multiplayer and persistence. That experiment follows the Unreal and forest proofs; its game-specific constraints do not drive the immediate core architecture.

_Why it serves the approach:_ an engine-independent tree state keeps every representation optional, and proving functional trees in an integration we control establishes the foundation for adapting the same botanical state to an existing game's constraints.

### Surface and rendering at scale

Continuous surfaces, natural forks and tips, foliage and bark make the underlying structure legible from close views to the canopy. Simplification, aggregation, filtering, detail selection and streaming operate on generated geometry and measured error, without species-specific paths, needle-specific topology assumptions or hand-modelled foliage clusters. Validate this generality early on materially different tree and leaf shapes before committing to a rendering technique; geometry-dependent choices are allowed, and unsupported inputs remain explicit errors, never a second path. Hierarchy and continuous rendering representations keep generation, updates and rendering within game budgets from a hero tree to a forest, without visible stepping, thinning or shimmer; this fidelity direction applies across consuming engines, with concrete integrations validating it when undertaken.

_Why it serves the approach:_ visual fidelity and measured runtime cost jointly determine whether a generated tree belongs in a real-time world.
