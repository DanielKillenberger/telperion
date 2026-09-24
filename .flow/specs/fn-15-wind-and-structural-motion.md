# Wind and structural motion

## Conversation Evidence

> user: "what about wind for the harness renderer? it's just a nice to have for the view but I think it'd improve it a lot. And we implemented it in killenberger.com didn't even take a lot of effort."
> user: "We don't need that for engines that import telperion right? they'd simulate wind themselves? or do we provide it for them?"
> user: "No I want it done properly. Fn-15 already models this? and exports the right attributes to consumers? in the harness renderer the consumer would just be that renderer. Correct?"
> user: "ok"

## Goal & Context

<!-- Goal & Context: 30% [user], 70% [inferred] from code read 2026-09-24 -->

A tree moves in the wind as one connected body. The trunk leans, limbs sway on their own periods, and leaves flutter about their stalks. Nothing tears or detaches. The owner wants this done properly: in the generator's contract, with the harness renderer as its first consumer. [user]

Engines run their own wind: forces, gusts, timing, and often one system shared with grass and cloth. They cannot animate a tree well without data only the generator has: which piece of wood each vertex belongs to, where that piece pivots, how stiff it is, and a phase so pieces do not move in lockstep. SpeedTree and Unreal's Pivot Painter ship exactly this, per-vertex motion data plus a reference shader. Telperion ships the same. The tree carries a motion stream and a reference motion function; the consumer supplies wind and time. [inferred]

killenberger.com's wind (`src/lib/grove/wind.ts`, 56 lines) is the precedent for the wind itself: a breeze with a slow swell plus gust fronts that roll across, as pure functions of time and place. It drives one damped spring per tree over a 2D raster, so its code does not carry over; its wind field does, as the harness's input. [inferred]

## Architecture & Data Models

<!-- scope: technical, checked against origin/master 123c4261 on 2026-09-24 -->

**Bones are the wood's runs.** The surface already divides the wood into runs, each a path that continues through the thickest child (`surface/paths.rs:15`), and the renderer expands wood per run (`generation/wood.wgsl:10-43`). A bone is one run. Its record holds:
- the pivot: where it leaves its parent;
- the rest axis and length;
- the base radius;
- the parent bone and the distance along the parent where it attaches;
- a phase hashed from the run's first node's stable `NodeIdentity`, so a tree keeps its motion across rebuilds.

The bone table is part of stage 3's plan layout (fn-125) and is versioned with it. [inferred]

**The motion stream.** It is built only when a consumer asks for motion, so a static consumer pays nothing:
- **Wood:** one u32 per vertex, the bone index. Distance along the bone comes from the vertex's existing `coord.x`, the distance along the branch from the root (`surface.rs:182-185`), less the bone's base distance, so no second field is needed. The oak at seed 1 draws 3,724,874 wood vertices at 36 bytes each (fn-91 `browser-baseline.json`), so the stream adds about 15 MB, 11%.
- **Leaves:** one u32 per leaf, carried beside the three packed words and never inside them. It holds the bone index in 24 bits and the distance along the bone in 8. Today nothing that reaches the draw links a leaf to its wood:
  - the specimen view keeps only `p.leaf` (`specimen/view.rs:71`);
  - clumping and the cull remove leaves in place (`foliage.rs:253`);
  - the GPU compaction copies only the words (`scatter.wgsl:9-11`).

  The GPU path has the link at the right moment. Station preparation knows each segment's distal node (`prepared.rs:184-186`), a node maps to its run, and `StationSegment.range.w` is unused padding (`data.rs:101`) that can hold it. `scatter` then writes the word beside each surviving leaf, and the CPU path carries it through clumping and the cull in the same order as the leaves. At 4 bytes a leaf this is 2.9 MB on the oak and 29 MB on the spruce (7,353,754 leaves). [inferred]

**The motion function.** It is stateless: a pure function of the bone table, the wind input and time, so every frame, every consumer and every replay agree without a simulation to keep.
1. **Bone frames.** A small compute pass runs each frame. Each bone walks its ancestor chain and composes the bends at each attachment, so a bone's frame is its parent's bent frame at the point where it attaches. There are only as many bones as runs, far fewer than vertices; the count per preset is measured, not assumed.
2. **Bend.** A bone bends in the plane of the wind across its axis, by a static lean plus an oscillation.
   - Stiffness follows the wood as a cantilever: radius to the fourth over length cubed for the lean, and radius over length squared for the natural frequency. A thin long limb leans further and swings slower than a thick short one, with no species constant.
   - A point at distance `s` along a bone of length `L` turns by the bend times `(s/L)^2` about the pivot. That is a rotation, so wood never stretches.
3. **Wind input.** A field of direction and strength sampled at each bone's pivot, so a gust front reaches the lower crown before the upper, plus time. The reference field is killenberger.com's breeze, swell and rolling gusts, lifted to 3D. A consumer may pass its own.
4. **Leaves.** A leaf takes its bone's transform at its stored distance along the bone. It then flutters about its own base: the stored position is the petiole base (`element.rs:90`, `station.rs:100-117`), so a rotation applied before `leaf_transform` pivots there with no new data. `coord.x` weights the flutter toward the tip, and the leaf's index gives its phase, as `vary(id)` already does for colour (`common.wgsl:205`).

**Where it lives.** The bone table, the stream encoders, a CPU reference evaluator and the WGSL motion function live in fn-125's `gpu` module in `telperion-core`. The core stays wgpu-free. The CPU evaluator defines correct; the WGSL matches it within fn-125's tolerance; tests and GPU-less consumers use the CPU one. [inferred]

**The harness renderer as consumer.**
- A time and wind block joins the frame uniforms. No time reaches any shader today (`common.wgsl:9-92`).
- One displacement function is applied in every pass that places geometry: wood (`wood.wgsl:17-31`, which today draws the vertex position unchanged), foliage (`foliage.wgsl:57`), and both shadow casters (`shadow.wgsl:15-40`). The shadow module has no prelude or uniforms today (`shadow.rs:150-159`) and needs the motion bindings; its own comment says "future motion must match it" (`shadow.wgsl:23`).
- Selection (`select.wgsl:76`) keeps reading rest positions and widens its bounds by the largest displacement the current wind allows, so nothing is culled while moving into view.
- The harness redraws every animation frame already (`harness/rust-stage.ts:99-107`) and redraws the shadow map every frame (`lib.rs:341-387`), so motion adds no new redraw path. [inferred]

## Edge Cases & Constraints

- **Zero wind is the rest tree.** With wind off or zero, every vertex is where it is today to the byte, and a build that asks for no motion is byte-identical to today's. Headless stills, species QA and every owner verdict render with wind off. [inferred]
- **Harness default.** Wind is on in the interactive harness, off under `?wind=0`, and off when the viewer prefers reduced motion, as killenberger.com does (`TreeCanvas.tsx:50`). [inferred]
- **Attachment is a contract.** A leaf's base and the bark under it stay together under any wind, within fn-125's position tolerance scaled by the bend. A child run's first ring stays seated in its parent (`surface/samples.rs:107-118`). [inferred]
- **Bounded motion.** No bone passes straight down, and no bend exceeds the stated maximum however strong the input. An invalid wind input (non-finite, negative strength) is refused by name, not clamped silently. [inferred]
- **Families on the CPU fallback** (fn-125's five) still move. The CPU path carries the same stream. [inferred]
- **Growth path** (`?growth=1`) stays buildable and unanimated. [CLAUDE.md]

## Acceptance Criteria

- **R1:** The bone table and both motion streams are in the versioned plan layout document (fn-125 R1). A test round-trips them for every catalogue preset. A build without motion is byte-identical to today's on every catalogue preset at seeds 1 and 7. [inferred]
- **R2:** The CPU evaluator, on a synthetic three-bone family, meets each of these as its own failing test:
  - zero wind moves nothing;
  - a thicker bone under the same wind bends less and swings faster;
  - a child's pivot follows its parent's bent frame;
  - no point's distance to its pivot changes;
  - an invalid input is refused by name. [inferred]
- **R3:** The WGSL motion function matches the CPU evaluator within fn-125's tolerance, scaled by bend, for every catalogue preset at seeds 1 and 7 at three wind states. It translates through naga to HLSL and SPIR-V without error. [inferred]
- **R4:** In the harness, wood, foliage and both shadow casters move by the one function. A test at a fixed time and wind finds every leaf base within tolerance of the bark it hangs on, and no leaf culled that the rest bounds plus the displacement bound would draw. [inferred]
- **R5:** Cost is measured and reported, not gated. For the oak and spruce at seed 1, native and browser:
  - completed-frame medians with wind off and on;
  - the per-frame bone pass time;
  - the stream memory. [inferred]
- **R6:** The owner judges the motion in the running harness on the oak, spruce and birch, and the verdict is recorded. A rejecting verdict stops the spec with the owner's words. [user]

## Boundaries

- No physical simulation, no collision between branches, no breakage or growth response (fn-16). [inferred]
- No engine adapter; fn-17 proves the stream in an engine. [inferred]
- No preset rows. Stiffness comes from the wood's own geometry, and wind strength is a runtime input. If the owner's verdict shows a species needs its own flexibility, that row is a follow-up the owner decides. [inferred]
- Captures stay under the budget rules: motion is judged in the live harness, never through a full-forest capture. [CLAUDE.md]

## Decision Context

- Rewritten 2026-09-24 from the 2026-09-05 roadmap placeholder, which named no attributes, consumer or contract. [user]
- Depends on fn-125, whose plan layout and `gpu` module this spec extends. Designing the layout twice is the cost of building motion first. fn-9 and fn-23 are done and stay as dependencies. [inferred]
- A harness-only sway keyed on height was considered and rejected by the owner: "I want it done properly". [user]

## Open before ready

- None held by the host. Marking it ready is the owner's gate.
