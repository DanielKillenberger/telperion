# Wind and structural motion

## Conversation Evidence

> user: "what about wind for the harness renderer? it's just a nice to have for the view but I think it'd improve it a lot. And we implemented it in killenberger.com didn't even take a lot of effort."
> user: "We don't need that for engines that import telperion right? they'd simulate wind themselves? or do we provide it for them?"
> user: "No I want it done properly. Fn-15 already models this? and exports the right attributes to consumers? in the harness renderer the consumer would just be that renderer. Correct?"
> user: "ok"
> user (2026-09-27, on the maximum bend): "it should based on reality. Can we do a /refine --scope=research pass on this. But also it should be configurable. If we need to be able to exaggarate the effect somewhat for supernatural. That should be fine? We should definitely be able to configure the wind to be a storm."
> user (2026-09-27, on the harness wind control, selected): "Strength + direction"
> user (2026-09-27): "there should also be a variance slider or smth on how much and how often the wind oscillates around the average speed?"
> user (2026-09-27): "How do we integrate this nicely into the pipeline? i guess it depends on fn-125? and can we parallelize generating the motion data next to another part of the pipeline? a naive implementation i assume would add generation time if motion data is requested?"
> user (2026-09-27): "yes but also we need to enable lod for this motion data. [...] So simpler trees should also have simpler motion data."
> user (2026-09-27, on adding in-pass production, the cost gate and motion levels, selected): "All three"

## Goal & Context

<!-- Goal & Context: 30% [user], 70% [inferred] from code read 2026-09-24 -->

A tree moves in the wind as one connected body. The trunk leans, limbs sway on their own periods, and leaves flutter about their stalks. Nothing tears or detaches. The owner wants this done properly: in the generator's contract, with the harness renderer as its first consumer. [user]

Engines run their own wind: forces, gusts, timing, and often one system shared with grass and cloth. They cannot animate a tree well without data only the generator has: which piece of wood each vertex belongs to, where that piece pivots, how stiff it is, and a phase so pieces do not move in lockstep. SpeedTree and Unreal's Pivot Painter ship exactly this, per-vertex motion data plus a reference shader. Telperion ships the same. The tree carries a motion stream and a reference motion function; the consumer supplies wind and time. [inferred]

killenberger.com's wind (`src/lib/grove/wind.ts`, 56 lines) is the precedent for the wind itself: a breeze with a slow swell plus gust fronts that roll across, as pure functions of time and place. It drives one damped spring per tree over a 2D raster, so its code does not carry over; its wind field does, as the harness's input. [inferred]

## Architecture & Data Models

<!-- scope: technical, checked against origin/master 123c4261 on 2026-09-24 -->

**Bones are the wood's runs.** The surface already divides the wood into runs, each a path that continues through the thickest child (`pipeline/surface/paths.rs:15`), and the renderer expands wood per run (`generation/wood.wgsl:10-43`). A bone is one run. Its record holds:
- the pivot: where it leaves its parent;
- the rest axis and length;
- the base radius;
- the parent bone and the distance along the parent where it attaches;
- a phase hashed from the run's first node's stable `NodeIdentity`, so a tree keeps its motion across rebuilds.

The bone table is part of stage 3's plan layout (fn-125) and is versioned with it. [inferred]

**The motion stream.** It is built only when a consumer asks for motion, so a static consumer pays nothing:
- **Wood:** one u32 per vertex, the bone index. Distance along the bone comes from the vertex's existing `coord.x`, the distance along the branch from the root (`pipeline/surface.rs:246-247`), less the bone's base distance, so no second field is needed. The oak at seed 1 drew 3,724,874 wood vertices at 36 bytes each (fn-91 `browser-baseline.json`, before fn-157 changed species values), so the stream adds about 15 MB, 11%; R5 re-measures it.
- **Leaves:** two words per leaf, carried beside the three packed words and never inside them: the bone index (u32) and the distance along the bone (f32, the same float the wood's `coord.x` carries), so a leaf and the bark under it evaluate the same bend at the same distance. The encoding is held to an attachment-error budget, not chosen first: at the largest bend any wind and flexibility allow, a leaf's attachment point and the bark point it was seated on differ by no more than fn-125's leaf position tolerance. (An 8-bit distance was rejected: at s/L = 0.9 on a 5 m bone bent 1 rad it misplaces a leaf by about 16 mm, against fn-125's 0.61 mm.) Today nothing that reaches the draw links a leaf to its wood:
  - the specimen view keeps only `p.leaf` (`specimen/view.rs:74`);
  - clumping and the cull remove leaves in place (`pipeline/foliage.rs`, `cull` `:255`, `retain` `:275`);
  - the GPU compaction copies only the words (`scatter.wgsl:9-11`).

  **Ownership per fn-125 source.** A twig-station leaf belongs to its segment's distal node's run; a short-shoot cluster's leaves to the run of the wood node the cluster clothes; a rosette frond's leaflets to the stem run at the apex, carried as one rigid frond from its base with each leaflet's rest offset along the rachis. Each source writes the same two words.

  The GPU path has the link at the right moment. Station preparation knows each segment's distal node (`pipeline/foliage/prepared.rs:188`), a node maps to its run, and `StationSegment.range.w` is unused padding (`generation/data.rs:96`, field at `:29`) that can hold it. `scatter` then writes the word beside each surviving leaf, and the CPU path carries it through clumping and the cull in the same order as the leaves. At 8 bytes a leaf this was 5.7 MB on the oak and 59 MB on the spruce (7,353,754 leaves), counts from before fn-157; R5 measures the stream on the current counts. [inferred]

**The motion function.** It is stateless: a pure function of the bone table, the wind input and time, so every frame, every consumer and every replay agree without a simulation to keep.
1. **Bone frames.** A small compute pass runs each frame. Each bone walks its ancestor chain and composes the bends at each attachment, so a bone's frame is its parent's bent frame at the point where it attaches. There are only as many bones as runs, far fewer than vertices; the count per preset is measured, not assumed.
2. **Bend.** A bone bends in the plane of the wind across its axis, by a static lean plus an oscillation.
   - Stiffness follows the wood as a cantilever: radius to the fourth over length cubed for the lean, and radius over length squared for the natural frequency. A thin long limb leans further and swings slower than a thick short one, with no species constant.
   - A bone bends as a circular arc of its own length: a bend of β bends the centreline through angle β·s/L at distance `s` along a bone of length `L`, so the bent centreline point is closed-form (pivot plus `L/β·(sin(βs/L)·axis + (1 − cos(βs/L))·bend direction)`, the straight axis as β → 0), and each ring turns with the centreline's tangent there. Arc length is preserved exactly, so the wood never stretches. (Turning each point by a different angle about the pivot, the first draft's rule, keeps pivot distance but stretches the wood between points by about 17% at the tip at 0.3 rad.)
3. **Wind input.** A field of direction and strength sampled at each bone's pivot, so a gust front reaches the lower crown before the upper, plus time. A consumer may pass its own field. The reference field is killenberger.com's breeze, swell and rolling gust fronts, lifted to 3D: the fronts travel along the wind's direction in metres. It reads four inputs (owner, 2026-09-27):
   - **Strength:** the mean wind speed in m/s, 0 to 33, the Beaufort scale's calm to hurricane.
   - **Direction:** the compass bearing the wind blows toward, 0 to 360 degrees.
   - **Gustiness:** turbulence intensity, the spread of speed around the mean divided by the mean, 0 (steady) to 0.6. The default is 0.33, the low end Milne measured at a spruce canopy's top, whose peak gusts ran about twice the mean.
   - **Gust interval:** the mean seconds between gust fronts, killenberger.com's 6.5 s as the default.
4. **Leaves.** A leaf takes its bone's transform at its stored distance along the bone. It then flutters about its own base: the stored position is the petiole base (`pipeline/foliage/element.rs:190`, `pipeline/foliage/station.rs:102-118`), so a rotation applied before `leaf_transform` pivots there with no new data. `coord.x` weights the flutter toward the tip, and the leaf's index gives its phase, as `vary(id)` already does for colour (`common.wgsl:208`).

5. **The bend follows reconfiguration and stops at failure.** Force on a crown grows with the square of wind speed at low wind, and crowns streamline as the wind rises (Vogel exponent near −1 for dense crowns, frontal area down to 20 to 54% at 20 m/s), so a bone's lean grows about linearly with speed in strong wind, not with its square. A bone's bend is capped at its failure bend, which shrinks with its stiffness: a few degrees for a trunk (a modelled tip slope near 0.1 rad at breakage), tens of degrees for a limb (17° measured on one maple limb), more for twigs. Wind above failure saturates the bend; nothing breaks, since breakage is fn-16's.
6. **Frequencies land in the measured bands.** Trunk natural frequencies fall at 0.2 to 0.5 Hz for 12 to 24 m trees, with 5 to 10% damping in leaf and less bare. Branches in leaf sway near the trunk's own frequency and damp it (mass damping), so a limb's oscillation is not the fast independent swing the bare cantilever law gives. Leaves flutter at 3 to 8 Hz, starting near 1.35 m/s and saturating near 2.6 m/s, and branch motion dominates above about 5 m/s.
7. **Flexibility is a family row for the supernatural** (owner, 2026-09-27). `motion.flexibility`, 1 by default, is the tree's physical response; above 1 it scales every bone's bend and failure cap and lowers its frequencies, so a supernatural preset can move more than wood would. It is one row blended like any other, dormant for a consumer that asks for no motion.

**Produced inside the passes that already run** (owner, 2026-09-27). Every piece of motion data is known at a moment the pipeline already reaches, so none gets a pass of its own:
- the bone table is filled in the run loop that orders and samples runs (on the CPU today, on the GPU if fn-171 passes), one record per run from data that loop already holds, and overlaps the GPU work in flight as station preparation does;
- the wood bone index is written by the emit pass, which works one run at a time; whether it can instead be derived in the vertex shader from the vertex index (every ring in a tree has the same segment count, the end caps being the open question) is probed first, and chosen if it holds, so the wood carries no stream at all;
- the leaf words are written where a leaf is seated from its station or source, and the scatter pass copies them beside each survivor.
A separate sweep over vertices or leaves after the build is ruled out.

**Motion levels** (owner, 2026-09-27). The bone table is ordered by significance, trunk first, then limbs by girth, then fine branches, parent before child, with nested levels addressing its prefixes, as the leaf element's levels do. A consumer evaluates the bones of the level it picks, and every bone past it rides its nearest evaluated ancestor rigidly at its rest offset. Levels follow the far-draw rule (`.flow/memory/knowledge/decisions/the-far-draw-is-the-near-draw-minus-2026-09-18.md`): a bone drops out at the distance where its largest bend under the current wind and flexibility moves nothing more than half a pixel, never earlier. The reference renderer picks the level per tree per frame by that rule. Wood and whole-tree LOD are not this spec's; motion levels are prefixes of the bone table so a later tree LOD that drops fine runs drops their bones with them.

**Where it lives.** The bone table, the stream encoders, a CPU reference evaluator and the WGSL motion function live in fn-125's `gpu` module in `telperion-core`. The core stays wgpu-free. The CPU evaluator defines correct; the WGSL matches it within fn-125's tolerance; tests and GPU-less consumers use the CPU one. [inferred]

**The harness renderer as consumer.**
- A time and wind block joins the frame uniforms. No time reaches any shader today (`common.wgsl:9-95`).
- One displacement function is applied in every pass that places geometry: wood (`wood.wgsl:17-31`, which today draws the vertex position unchanged), foliage (`foliage.wgsl:55-56`), and both shadow casters (`shadow.wgsl:15-40`). The shadow module has no prelude or uniforms today (`shadow.rs:150-159`) and needs the motion bindings; its own comment says "future motion must match it" (`shadow.wgsl:22`).
- Selection (`select.wgsl:76`) keeps reading rest positions and widens its bounds by the largest displacement the current wind and flexibility allow, so nothing is culled while moving into view.
- The shadow light volume, fitted today to the rest bounds with a fixed 1 m margin (`Renderer::draw_with` passing `self.bounds()` to `shadow::light`, `shadow/fit.rs`), is fitted to the same motion-widened bounds, so a storm-bent crown and its ground shadow stay inside it.
- The harness redraws every animation frame already (`harness/rust-stage.ts:99-107`) and redraws the shadow map every frame (`lib.rs:335`, the shadow pass at `:342-386`), so motion adds no new redraw path. [inferred]

## Edge Cases & Constraints

- **Zero wind is the rest tree.** With wind off or zero, every vertex is where it is today to the byte, and a build that asks for no motion is byte-identical to fn-125's output. Headless stills, species QA and every owner verdict render with wind off. [inferred]
- **Harness default.** Wind is on in the interactive harness at a moderate breeze (Beaufort 4, about 7 m/s) with the default gustiness and interval, off under `?wind=0`, and off when the viewer prefers reduced motion, as killenberger.com does (`TreeCanvas.tsx:50`). The panel carries four wind controls: strength, direction, gustiness and gust interval, each on the input's range; strength at 0 shows the rest tree. [paraphrase]
- **Attachment is a contract.** A leaf's base and the bark under it stay together under any wind, within fn-125's position tolerance scaled by the bend. A child run's first ring stays seated in its parent (`pipeline/surface/samples.rs:107-118`). [inferred]
- **Bounded motion.** No bone passes straight down, and no bend exceeds the stated maximum however strong the input. An invalid wind input (non-finite, negative strength) is refused by name, not clamped silently. [inferred]
- **Every tree moves.** fn-125 leaves no CPU fallback, so every catalogue and in-work preset goes through the one pipeline and carries the same stream; the CPU reference evaluator serves GPU-less consumers. [inferred]
- **Growth path** (`?growth=1`) stays buildable and unanimated. [AGENTS.md]

## Acceptance Criteria

- **R1:** The bone table and both motion streams are in the versioned plan layout document (fn-125 R1). A test round-trips them for every catalogue preset. A build without motion is byte-identical to today's on every catalogue preset at seeds 1 and 7. [inferred]
- **R2:** The CPU evaluator, on a synthetic three-bone family, meets each of these as its own failing test:
  - zero wind moves nothing;
  - a thicker bone under the same wind bends less and swings faster;
  - a child's pivot follows its parent's bent frame;
  - the bent centreline keeps its length along the curve: between any two ring parameters `s_i < s_j` on a bone, the centreline's arc length, measured by summing at least 1,000 sub-segments of the evaluated curve, equals `s_j − s_i` within 1e-5 relative at every tested bend (the straight chord between ring centres shortens as the bone bends, as an arc's chord does, and is not the quantity tested);
  - an invalid input is refused by name. [inferred]
- **R3:** The WGSL motion function matches the CPU evaluator within fn-125's tolerance, scaled by bend, for every catalogue preset at seeds 1 and 7 at three wind states. It translates through naga to HLSL and SPIR-V without error. [inferred]
- **R4:** In the harness, wood, foliage and both shadow casters move by the one function. At a fixed time and wind, and at the largest bend any wind and flexibility allow, a test finds: each directly attached leaf (twig stations, short-shoot clusters) within fn-125's leaf position tolerance of the bark point it was seated on; each compound leaflet (a rosette frond's leaflets along its rachis) at its rest offset from its source's attachment within that tolerance, without requiring leaflets to touch bark; no leaf culled that the rest bounds plus the displacement bound would draw; and every shadow caster inside the fitted light volume across sun directions. Errors: no error surface beyond R2. [inferred]
- **R5:** Cost is measured and reported, not gated. For the oak and spruce at seed 1, native and browser:
  - completed-frame medians with wind off and on;
  - the per-frame bone pass time;
  - the stream memory. [inferred]
- **R6:** The owner judges the motion in the running harness on the oak, spruce and birch, and the verdict is recorded. A rejecting verdict stops the spec with the owner's words. [user]
- **R7:** The wind input carries strength (m/s, 0 to 33), direction (degrees), gustiness (0 to 0.6) and gust interval (seconds, above 0), and the harness panel sets all four. Errors: a value outside its range, or non-finite, is refused naming the field. [paraphrase]
- **R8:** The CPU evaluator meets the research's landmarks on the oak and birch at seed 1, each its own test: at 0 m/s nothing moves; leaves flutter at 3 to 8 Hz once the wind passes about 1.35 m/s; the trunk's natural frequency lies in 0.2 to 0.5 Hz; the trunk's bend at 33 m/s stays within its failure cap while a thin limb's bend is at least ten times the trunk's; and bend grows about linearly, not quadratically, between 20 and 33 m/s. Errors: no error surface beyond R2. [paraphrase]
- **R9:** `motion.flexibility` is a family row, 1 by default, with a rail of 0.25 to 4, validated by name, on the wire, blended and in the browser metadata, with the snapshot schema and catalogue wire test updated; at 2 a bone's bend and cap double and its frequency falls; at 1 every preset's motion is R8's. Errors: a value outside the rail is refused naming the field. [paraphrase]
- **R10:** Motion data is produced inside the existing run, emit, seat and scatter passes, with no separate pass over vertices or leaves, and a build that asks for motion takes at most 5% longer (prepare total, `BASELINE.md`'s method, medians of five warm runs) than the same build without it, for the oak and spruce at seeds 1 and 7. The report states whether the wood bone index was derived from the vertex index or stored, with the probe's result. Errors: a cost above 5% stops with `NEEDS_HUMAN` and the profile. [paraphrase]
- **R11:** The bone table carries nested motion levels as prefixes ordered parent before child; evaluating any level leaves every bone past it at its nearest evaluated ancestor's frame times its rest offset, and the finest level is R8's full motion. In the harness, the level is picked per tree per frame so no dropped bone would have moved more than half a pixel, and a sweep of stills across a walk from the base to the hero pose under a fixed wind shows no visible step, judged by the owner with R6. Errors: no error surface beyond R2. [paraphrase]

## Boundaries

- No physical simulation, no collision between branches, no breakage or growth response (fn-16). [inferred]
- No engine adapter; fn-17 proves the stream in an engine. [inferred]
- One preset row only: `motion.flexibility`, for exaggerating a supernatural tree. Stiffness otherwise comes from the wood's own geometry, and the wind is a runtime input. (Owner, 2026-09-27, replacing "no preset rows".) [paraphrase]
- No breakage: wind above a bone's failure cap saturates its bend; breaking is fn-16's. [inferred]
- Captures stay under the budget rules: motion is judged in the live harness, never through a full-forest capture. [AGENTS.md]

## Decision Context

- Rewritten 2026-09-24 from the 2026-09-05 roadmap placeholder, which named no attributes, consumer or contract. [user]
- Depends on fn-125, whose plan layout and `gpu` module this spec extends. Designing the layout twice is the cost of building motion first. fn-9 and fn-23 are done and stay as dependencies. [inferred]
- A harness-only sway keyed on height was considered and rejected by the owner: "I want it done properly". [user]
- Refined 2026-09-27: the maximum bend is physical (failure cap by stiffness) rather than a chosen angle, the wind reaches storm strength, a flexibility row exaggerates it for supernatural trees, and the harness sets strength, direction, gustiness and gust interval (owner). References re-anchored to master c71ef3e1's `pipeline/` paths; the CPU-fallback edge case is replaced, since fn-125 removes the fallback. [paraphrase]
- Research gaps that stay open: no accessible measurement of trunk lean angle against wind speed, of leaf flutter amplitude, or of the honami gust period, so those are tuned by the owner's look (R6), not pinned. [inferred]

## Resolved via Research
<!-- provenance: refine --scope=research (literature scout for tree biomechanics, memory-scout) on 2026-09-27; the docs-scout and docs-gap-scout were not run, since the spec names no new library -->

### literature
- **Trunk frequency tracks size:** conifers linear in DBH/H², damping usually under 0.05. Source: Moore & Maguire 2004, Trees 18:195, https://link.springer.com/article/10.1007/s00468-003-0295-6
- **Broadleaf damping and frequency:** 12 to 24 m trees damp 8.6 ± 2.2% in summer, 3.9 ± 1.3% in winter; leaves raise f0 by 18%; a modelled sycamore sways at 0.26 Hz. Source: Jackson et al. 2019, J. R. Soc. Interface, https://royalsocietypublishing.org/doi/10.1098/rsif.2019.0116
- **f0 follows height across 243 trees;** conifers fit the cantilever model, broadleaves the pendulum. Source: Jackson et al. 2021, Biogeosciences 18:4059, https://bg.copernicus.org/articles/18/4059/2021/
- **Branches sway with the trunk and damp it:** a 19.7 m silver maple swayed at 0.33 Hz trunk and limbs together; with limbs removed, 1.0 Hz and damping 5% to 1%. Source: James et al. 2014, AUF 40(3):125, https://auf.isa-arbor.com/content/40/3/125; James, Haritos & Ades 2006, AJB 93:1522
- **Crowns streamline:** at 20 m/s frontal area falls to 20 to 37% in three hardwoods and by 36 to 54% in three conifers; Vogel exponent near −1 for dense crowns. Sources: Vollsinger et al. 2005, CJFR 35:1238; Rudnicki, Mitchell & Novak 2004, CJFR 34:666; Gosselin 2019, https://arxiv.org/pdf/1905.08055
- **Limb deflection:** about 17° measured at 1.8 m along a Norway maple branch (wind speed not stated). Source: Yang et al. 2021 review, https://felix-rz.github.io/pdf/2021_Review.pdf
- **Failure:** no damage reported below 20 m/s and most trees broken above 42 m/s gusts in storm Klaus; a modelled tip slope near 0.1 at breakage (contested, PRE 94:067001). Wytham broadleaves' critical speeds 23 to 56 m/s. Sources: Virot et al. 2016, PRE 93:023001; Jackson et al. 2019, Front. For. Glob. Change, https://www.frontiersin.org/articles/10.3389/ffgc.2018.00013/full
- **Leaf flutter:** 3 to 5 Hz in aspen and cottonwood canopies; a cherry's leaves at 6 to 8 Hz, flutter starting at 1.35 m/s and saturating at about 2.6 m/s, branch motion dominating above about 5 m/s. Sources: Roden & Pearcy 1993, Oecologia 93:201; Tadrist et al. 2018, J. R. Soc. Interface 15:20180010
- **Beaufort land scale:** 2 (2 to 3 m/s) leaves rustle; 4 (6 to 8 m/s) small branches move; 6 (11 to 14 m/s) large branches in motion; 7 (14 to 17 m/s) whole trees in motion; 8 (17 to 21 m/s) twigs break; 10 (25 to 28 m/s) trees uprooted; 12 at 33 m/s and above. Sources: https://weather.metoffice.gov.uk/guides/coast-and-sea/beaufort-scale, https://www.rmets.org/metmatters/beaufort-wind-scale
- **Gusts:** a 3 s gust is 1.66 × the 10 min mean over open land; at a spruce canopy top, turbulence intensity 0.33 to 0.54 and gust factor 2.1 on average; canopy motion is dominated by intermittent sweeps and honami waves (their period not found). Sources: Harper, Kepert & Ginger 2010, WMO/TD-1555; Milne 1992, AFM 61; Gardiner 1994, BLM 67:161; Finnigan 2000, Annu. Rev. Fluid Mech. 32:519

### memory-scout
- **a-new-habitparams-row-changes-the-2026-09-26:** a new family row changes the snapshot schema, so the catalogue wire test and the pinned harness test move with R9. Source: .flow/memory
- **a-mid-task-instruction-that-adds-a-2026-09-04:** harness controls are independent and each dormant at its neutral value, which the four wind controls follow. Source: .flow/memory
- **a-slack-band-around-an-orbit-pivot-2026-09-04:** bounds come from the subject as drawn, not estimates, which R4's displacement bound follows. Source: .flow/memory
