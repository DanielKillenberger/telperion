---
name: Telperion
last_updated: 2026-09-05
generator: flow-next-strategy
---

# Telperion Strategy

## Target problem

Trees in games are baked assets: modelled offline, branching stops a few orders down, and the twigs and leaves are painted cards. Nobody has seen a tree in a game that branches the way a real one does, from trunk to the twig a leaf hangs on, and nobody has seen that done for a tree that is supernatural on purpose. It is hard because a realistic 150 m tree is millions of elements, growth to twig scale is a different method at every scale, and it all has to stay one continuous structure that renders in a frame.

## Our approach

Grow the tree rather than model it: one continuous recursion from trunk to twig under botanical rules, with everything supernatural living in a single parametric bias field. Generated deterministically at runtime in the browser and measured on a named machine, so the tree is real by default and magical by dial.

## Who it's for

**Primary:** Game and real-time 3D developers — they're hiring Telperion to put trees in a scene that branch the way a real tree does, from a hero tree to a whole forest of a species, generated from a seed instead of bought as baked assets.

**Secondary:** The owner, judging the Two Trees in clay — the first user, and the one whose eye every preset is held to.

## Key metrics

- **Fidelity** — the leaf is a botanical multiple of the twig it hangs on, and the tree carries the leaf count its size implies (10^5 to 10^7 for the Two Trees); measured in the unit tests on both presets.
- **Frame** — a shipped hero tree renders inside 2 ms of GPU time at native pixel ratio on the named machine, an RTX 3080, and the whole vegetation layer of a thousand-tree forest inside 4 ms once a forest rig exists to measure it; GPU timer queries in the harness rig. A game shares its frame, so the tree gets a slice of it, not the whole.
- **Build** — time from a dial move to a finished tree, held to whatever keeps dragging usable; the harness's own build timer.
- **Attributability** — same seed and parameters give a byte-identical tree, and every change to a preset traces to a named dial; asserted in the unit tests.
- **The owner's eye** — Telperion reads as Telperion and Laurelin as Laurelin in clay with every supernatural term at its preset value; judged in the harness, recorded in the spec.

## Tracks

### Growth

The one recursion from trunk to twig: branch generations with real length and laterals, the twig as a fixed botanical anatomy, and continuity asserted across every change of method.

_Why it serves the approach:_ growing rather than modelling is only true if the structure is one structure all the way down.

### The supernatural field

The bias terms, the presets that state them, and the Two Trees they are judged on.

_Why it serves the approach:_ the magic has to be a parameter the whole tree obeys, or it is a hand edit like everyone else's.

### Rendering at scale

Twigs as instanced elements, a LOD ladder from full geometry to impostor, GPU or worker generation, species grown as a few dozen archetypes and instanced into forests, and export to engines. Millions of elements stay inside the frame, the build stays draggable, and a forest is a placement problem rather than a growth problem. The first proof of the forest path is a Valheim mod: biomes configured as species presets, trees spawned from seeds and grown by Telperion, in Valheim's own style, taking that world's procedural generation to the next level.

_Why it serves the approach:_ runtime generation is a claim about the frame and the build, and this track is where that claim is paid for, at one tree and at a thousand.

### The skin

The swept surface, forks, interpenetration and tips, and eventually bark and leaf texturing.

_Why it serves the approach:_ a grown skeleton is judged through its surface, and the surface is where a real tree and a pile of tubes part ways.
