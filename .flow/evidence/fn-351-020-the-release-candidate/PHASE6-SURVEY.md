# Phase 6 survey: drawn fast, the website and the entry

Filed 14 Nov 2026 by a Fable survey agent, read-only. Nothing was built, tested or committed. One light measurement was taken on build outputs that already existed (file sizes, `brotli -q 11`, `twiggy top`); it is marked where used.

Tags: **[code]** read in the source, with the file; **[record]** read in a spec, log or evidence file, with the place; **[derived]** worked out here from code or records, with how; **[unknown]** not found or not checked.

Code was read in `.worktrees/fn-364` at `8f6b017e2` (the stack fn-360 > fn-362 > fn-363 > fn-364). Against the integration branch that stack changes, under `src`, `harness`, `scripts`, the render crate and the Wasm binding, only the generated presets and parameters and six render test files [derived: `git diff --stat goal/fn-321...fn-364 -- src harness crates/telperion-render crates/telperion-wasm scripts package.json`]. So everything below about the package, the harness, the renderer and the binding holds on both.

Phase 6's text: "Levels of detail from the structure's own order; the voxel grove read from the tree's cells; growth off the main thread with or without WebGPU; the 250 KB budget; the native entry with a headless test; the forest rig and the frame measures." [record: PLAN-2026-11-11.md, section 5]

## 0. What stands under the whole phase

- Phase 5 has not landed. The run is in fn-363 and fn-364 (how the old oak is charged and extends); counted foliage is not in the grower, no twig is built from a key, and the per-species `/space/coarse` rows are still in the presets [record: fn-351 goal log, 13 Nov 03:25 to 14 Nov 01:11; code: `crates/telperion-core/src/presets/space.rs`, ten mentions of `coarse`].
- Three of the six items are worded on Phase 5's result: "the leaf its cohorts count" (levels of detail), "the tree's own cells" (the grove) and "its twigs for any region asked" (the entry) [record: fn-321 R8, R9, R11]. What those mean on a grower without counts is not written anywhere [unknown].
- The website is not in this repository. "killenberger.com's repository is not edited" [record: fn-351 Boundaries]. What the repository holds of it is the package entry `telperion/field`, the example voxelizer and a probe page [code: `src/field/index.ts`, `src/field/voxelize.ts`, `src/probe/field/`].
- The plan's own state lines for this phase: "P10: Entries take parameters (fn-317). No native entry with a headless test. P11, P13: Not measured." [record: PLAN-2026-11-11.md, section 2]
- Speed in the browser is the wide gap. Full growth in Wasm, with a 36-cell field: oak 3,584 ms, birch 1,230 ms, spruce 11,947 ms, against 0.1.4's 233 to 260, 239 to 304 and 203 to 212 ms; growth is 88 to 95% of each build and the field read 4 to 8%; Wasm is 10 to 18% slower than native [record: fn-329 RESULT.md, 3 Nov]. Not re-measured since Phases 1 to 4 changed the trees [unknown].

---

## 1. Levels of detail from the structure's own order

### 1.1 What is required

- "A hero tree renders within 2 ms of GPU time and a forest of 1,000 within 4 ms on the RTX 3080." [record: PLAN P13; STRATEGY.md, Key metrics, "Frame"]
- fn-351 P13 asks for a measurement and a report, not a pass: "the hero oak's GPU time and a forest of 1,000 oaks' are measured in the harness against STRATEGY.md's 2 ms and 4 ms on the RTX 3080 and reported." [record: fn-351 Acceptance]
- "A far tree is drawn from less of the same tree, by the order of its structure and the leaf its cohorts count, and a change of level moves nothing already drawn." [record: fn-321 R8, a paraphrase]. The phrase "the structure's own order" is used in the plan and in fn-321 and defined in neither [derived: searched `.flow/specs`, `.flow/evidence`, `docs`, `STRATEGY.md`].
- Owner, 18 Sep 2026 (P): the far draw is "the near draw minus what the eye cannot resolve at that distance, and nothing more, so a walk from base to hero pose shows no step in fidelity". [record: RULINGS.md, "Levels of detail"; fn-176]
- Owner, 3 Oct 2026: "growth decides where leaves are, so expansion places them without culling." [record: RULINGS.md, "No cull after growth"; fn-125]. The goal log added both rulings "because they bind Phase 6" [record: fn-351 log, 11 Nov 13:05].
- STRATEGY: "Simplification, aggregation, filtering, detail selection and streaming operate on generated geometry and measured error, without species-specific paths, needle-specific topology assumptions or hand-modelled foliage clusters." [record: STRATEGY.md, line 58]

### 1.2 What exists today

Wired:

- **Leaf shape levels.** Nested simplifications of the one leaf element, a doubling ladder chosen by outline deviation [code: `crates/telperion-core/src/pipeline/foliage/levels.rs`].
- **Per-leaf selection on the GPU each frame.** One compute thread per placement takes the coarsest level under half a pixel, or the unseen bucket outside the frustum; no state between frames [code: `crates/telperion-render/src/select.rs`, `shaders/select.wgsl`].
- **Thinning of small leaves and florets.** Leaves and flowers under two pixels are thinned by a share that follows their size and the kept ones enlarged [code: `crates/telperion-render/tests/thinning.rs`, module doc; fn-246].
- **The floret card.** Below 4 px a floret is one card; an owner decision of 9 Oct [code: `crates/telperion-render/src/card.rs`].
- **Wood by screen error.** The wood is a curve; each cluster keeps the coarsest of its ring levels (every 2^k-th point) under half the error budget, sides are `3·2^k` per ring, and a view past its budget draws at a coarser scale of 2, 4 or 8 and says so [record: docs/pipeline.md, "The wood's curve"; code: `crates/telperion-render/src/curve.rs`, `shaders/curve_walk.wgsl`, `curve_emit.wgsl`; CPU reference `Curve::tessellate`].
- **The CPU mesh's ring sides.** Since fn-362 a run is cut with the sides its widest ring needs within 0.2 mm; one count a run [record: fn-351 log, 13 Nov 03:25 and 04:26]. It is a property of the mesh, not of a view.

Partial:

- **Runs carry an order already.** The curve's runs are stored "widest first" with each run's largest radius [record: docs/pipeline.md, table `runs`; code: `crates/telperion-core/src/pipeline/surface/curve.rs`, `rings::rank`]. Nothing draws a prefix of them [derived: no reader of the order as a cut found in `curve.rs` or the shaders' module docs].
- **Identity that a level could key on exists.** Every drawn node has a key from its axis's lineage and place [code: `crates/telperion-core/src/tree.rs`, `node_key`].
- **A close view after a camera jump** draws one frame with wood missing: the first close frame of the ordinary tree draws at scale 8 with a cluster left out (tube capacity 29,160 against a demand of 255,492), the second is whole. "On the owner's list." Recorded for Phase 6 [record: fn-351 log, 13 Nov 02:43, 13:25, 13:28; code: `crates/telperion-render/src/curve/feedback.rs`].

Absent:

- **Any level for a whole tree.** Every leaf and every run of wood is drawn (or selected against) at any distance; a distant tree costs its full selection pass and its full curve walk [record: fn-176 Goal; code: `select.rs` and `curve.rs` module docs describe per-leaf and per-cluster work only].
- **Any level in what the CPU hands over.** `TreeMesh` is the whole tree; the CPU entry returns every chunk and every leaf [code: `crates/telperion-core/src/mesh.rs`; `src/browser/core.ts`, `TreeEngine.build`].
- **Twigs on demand.** "Nothing today builds wood on demand from a key." [record: fn-351 log, 13 Nov 03:25, item 4]

### 1.4 What earlier specs tried

- **fn-13** (tree detail by viewing distance, closed 8 Sep as a prototype): a WebGPU hero path with cross-part detail, worker preparation and streaming; a 1,024-tree capture peaked at 1.73 GB. "Not delivered: a usable forest, acceptable appearance, a stable viewer ... or a qualified performance number." Not merged [record: fn-13 Result].
- **fn-23** (done 9 Sep): per-leaf levels. Oak hero 1.750 ms native on the RTX 3080 at 1600x1000, spruce 32.6 ms; owner: "side by side they look exactly the same so we achieved the goal". That was the old generator's oak [record: fn-23 Result].
- **fn-27** (done 12 Sep): coarse shadow casters, to recover the sun pass's cost [record: fn-27 Goal].
- **fn-176** (whole-tree level of detail as prefixes of ordered data): **still open, never built**. Its design: each cluster's pruning order as an index list, runs in significance order, nested ring lists; drawn leaves a prefix with area held within 1% and colour within 2%. Its R1 pinned bytes that the engine replaced. Four parked unknowns, among them whether shadows and flutter follow the kept fraction and the wood's ring error metric [record: fn-176 Goal, Acceptance, Parked unknowns].
- **fn-208** (done 5 Oct): wood surfaced on the GPU at the screen's error; frames 1.3 to 3 times faster, wood memory spruce 7.49 to 1.59 GB. Open from it: surfacing walks each cluster six times a view, 2 to 10.5 ms [record: fn-208 Result, Open].
- **fn-246** (done 9 Oct): thinning, the card, the sun map's reuse. At 1920x1080 4x MSAA, fps whole / close: oak 234 / 72, spruce 122 / 130, beech 589 / 330, Telperion 50 / 34. Open: cluster cards for the Two Trees; texture bark (fn-247, open) for the wood's shaded pass, about 30 ms at close views [record: fn-246 Result, Open; fn-247 Goal].
- **fn-292** (done 18 Oct): level lists laid end to end, the spruce's 80.8 MB toward about 15 MB [record: fn-292 Goal].
- **fn-323** (1 Nov): keeping only low branch orders for the field failed its frozen rule. Order 5 keeps 85% of nodes at 40; lower orders lose foliage and limbs [record: fn-323 RESULT.md]. This is a field result, and the only measurement found of "cut by branch order" on the current engine.

### 1.5 What must keep holding

- `levels_nest_from_the_widest_section_down_to_the_whole_element`, `a_level_list_that_is_not_nested_detail_is_rejected_by_name` (core, foliage levels).
- `the_levels_share_one_list_the_crown_fills_once`, `an_empty_level_starts_where_the_next_does_and_a_forced_one_at_zero`, `a_forced_level_stays_within_each_kind_s_ladder`, `the_selection_path_names_no_family_and_no_anatomy` (render, selection).
- `a_crown_of_small_leaves_draws_fewer_and_covers_the_same`, `far_florets_drawn_as_cards_cover_what_the_florets_cover` (`tests/thinning.rs`); `a_floret_gets_a_card_of_its_petal_area_in_front_of_its_levels` (`card.rs`).
- `the_device_surfaces_the_wood_the_cpu_reference_does`, `a_view_past_its_finest_scale_is_drawn_at_the_coarser_scale_that_fits`, `the_second_slices_hold_what_the_first_could_not`, `wood_resting_on_the_ground_surfaces_off_it_on_the_device` (`tests/curve.rs`); `a_twig_stands_within_half_a_pixel_and_gains_detail_as_the_eye_nears` (core curve tests).
- `every_ring_of_a_run_has_its_widest_rings_sides_within_finest` (fn-362).
- `every_buffer_selection_needs_is_judged_and_named_before_the_upload`, `a_ladder_longer_than_selection_can_tally_is_refused_by_its_length`, `a_submitted_tree_is_the_tree_the_core_counted` (`tests/submit.rs`).
- The two rulings above: no step on a walk, and nothing culled after growth. No test holds "no step" by name [unknown: none found].
- `bark_distance` is failing on the phase's top already (oak 4x mean 6.00 to 6.64 against its bound) [record: fn-351 log, 13 Nov 13:03, 13:28]; a level that changes the far trunk meets it.

### 1.6 Open

Owner's:

- Whether P13's 2 ms and 4 ms are bars or reported measures. STRATEGY states them as metrics; fn-351 says "measured ... and reported" [record: both].
- Whether P13 needs texture bark (fn-247); the plan lists this as not read [record: PLAN section 7].
- The one frame with wood missing on a camera jump is "on the owner's list" [record: fn-351 log, 13 Nov 13:28].

Host's:

- What "the structure's own order" is: the engine's physiological age, the branch order from the trunk, the curve's run rank by radius, a node's lineage depth, or the cells of the light lattice. Each exists in the code today [code: `telperion-space/src/species`, `tree.rs`, `curve.rs`, `light.rs`].
- Whether the level lives in the generator's output (an ordered buffer a consumer reads a prefix of, as fn-176 drew it) or in the renderer only (as today's selection does). P10 points at the first; today's code is the second.
- How "nothing culled after growth" and "the near draw minus what the eye cannot resolve" read together for a tree drawn from fewer leaves.
- Whether it waits for counted foliage ("the leaf its cohorts count").

Options, unranked: (a) a prefix of the curve's runs by their rank, with the wood's existing error rule deciding the length; (b) a per-tree level chosen once per frame before selection, so a far tree skips its per-leaf pass; (c) fn-176's ordered index lists in the plan layout; (d) counted cells drawn as aggregate leaf at distance, once Phase 5 has them; (e) measure first and find the hero oak already inside 2 ms, leaving only the forest.

Measure first:

- The hero oak's GPU time on today's build at native pixel ratio, with `examples/frames.rs`; no number on the current trees was found [unknown].
- Where a far tree's cost sits: selection, the curve walk, the shadow pass or the leaf draw; `frames.rs` gives whole, mid, close, and the timing report gives per-pass medians [code: `crates/telperion-render/examples/frames.rs`, `src/timing.rs`].

---

## 2. The voxel grove read from the tree's cells

### 2.1 What is required

- "Its voxel grove is read from the tree's own cells. Cold page load to the first visible tree and the grove's completion are each at or under 0.1.4's" [record: fn-321 R9, a paraphrase].
- "no i want 0.2.0 to be strictly better. It should be efficient to use in a website coarse environment." [record: RULINGS.md, owner, 31 Oct]
- STRATEGY: "a voxel world such as Minecraft can sample wood and foliage as a spatial field and stream trees in without paying to construct a surface mesh." [record: STRATEGY.md, line 50]
- "Nothing per species" binds it: the per-species coarse rows "are what makes the website fast today (coarsest oak 164 ms in Wasm)" and are ruled out [record: PLAN section 2].
- fn-351's own P11 line does not repeat the grove sentence; Phase 6's text does [record: fn-351 Acceptance P11; Build order].

### 2.2 What exists today

Wired:

- **The field reads the drawn tree.** Each solved node's tapered sweep and each leaf-bearing record of the leaf plan write the cells they reach; the grid is `n` cells a side, 3 to 256, over the tree's box. No leaf is placed for it [code: `crates/telperion-core/src/pipeline/field.rs`, `field/splat.rs`; record: docs/pipeline.md, "The field"].
- **The package entry.** `growField(family, n, { limbOrder, source })` grows on the CPU through `telperion.wasm` and returns the grid: flags, wood radius, leaf count, limb id, floret count. It runs in a page, a worker or Node, with or without WebGPU; `compileField` shares one compiled module between workers [code: `src/field/index.ts`, `src/field/types.ts`].
- **The example voxelizer.** Wood by a radius cutoff in metres, foliage thinned by one of five rules (coin, density, mass, tuft, gap), flowers [code: `src/field/voxelize.ts`].
- **A probe page** that grows each site species in a worker beside 0.1.4's time; it is opened by the owner, not by an agent [code: `src/probe/field/main.ts`, `worker.ts`, `field-probe.html`].
- **A browser smoke test in a module worker** [code: `tests/browser/field.mjs`].

Absent:

- **A field read from the grower's own cells.** The grower has a light lattice at 0.5 m, leaf area deposited at its nodes [code: `crates/telperion-space/src/light.rs`, `CELL`]. Nothing reads a field from it; the field needs the whole drawn tree first, and without the `geometry` feature it is refused [record: docs/pipeline.md, "The field"].
- **A way to get a field without growing every twig.** Growth is 88 to 95% of the field call [record: fn-329 RESULT.md].
- **A grove.** No code places several trees; the grove is the site's [derived: no match for "grove" or "forest" in `src`, `harness`, `crates`, `scripts`, `tests`].
- **A still of the grove for a verdict.** "the grove on the website's voxel style is missing, since nothing draws it without a window" [record: fn-321 log, fn-336, 9 Nov].

### 2.4 What earlier specs tried

- **fn-100** (done 22 Sep): the field reads the plan, not placed leaves; spruce 9,002 to 233 ms. Owner: "looks good to me, seems like we have all the info"; coin dropping loses the crown's structure; how foliage is thinned is the consumer's problem; the wood cutoff is in metres, about 2 cm for broadleaves and 1 cm for the spruce [record: fn-100 Result].
- **fn-101** (done 22 Sep) and **fn-150** (done 27 Sep): a slim field module of its own, 107,758 Brotli bytes at fn-150; fn-150 removed its second build chain after the date palm could not be served [record: fn-101 Goal; fn-150 Goal, Result].
- **fn-214** (the field from the structure above the cut) and **fn-227**: closed as superseded, "not implemented and are not to be executed" [record: both specs' closing sections].
- **fn-257** (done 19 Oct): the site grew its trees on the device. A coarse-record route failed: acceptable only with nearly full expansion, then slower than growing everything. Device field: oak about 0.1.4's time, birch under it, spruce about 0.51 s, palm about 80 ms against 17 [record: fn-257 Goal; RESULTS.md]. Deleted with the GPU grower in fn-353 [record: fn-353].
- **fn-259 RELEASE.md** (the CPU field on the full tree, before fn-257): oak 6 to 10 times 0.1.4, birch 3 to 5, spruce 33 times, one spruce killed at 6 GB [record: fn-259 RELEASE.md, "The site's per-tree call"].
- **fn-316** (done): the field answers flowers. **fn-317** (done): the field takes parameters; the two executors' voxelizations differed by up to 5% of cubes (beech) and 10 to 24% (palm) [record: fn-318 Why]. **fn-318** (pregenerate on the GPU): closed as moot with the GPU grower [record: PLAN section 3].
- **fn-323** (1 Nov): a field from a scaffold cut at a branch order plus a density stand-in: stop. No candidate passes at both ages, and the scaffold that keeps the shape costs about 9.7 s at 80 against 50 ms [record: fn-323 RESULT.md].
- **fn-326, fn-328, fn-329**: the fidelity dial and its per-species rows gave oak 164 ms and birch 219 ms in Wasm, inside 0.1.4's; spruce 726 ms, 3.4 times 0.1.4; Laurelin and the beech slower at the coarsest than whole [record: fn-329 RESULT.md]. The owner then ruled the rows out [record: RULINGS.md, 3 Nov].
- **fn-334, fn-335** (9 Nov): deriving the rows from the species' tables was refuted; a stand-in read from a node's own state was a no-go [record: fn-321 log].

### 2.5 What must keep holding

- Core: `the_grid_is_the_one_the_voxelizer_draws`, `a_field_the_pipeline_cannot_read_is_refused_by_name`, `the_palm_field_places_nothing`, `the_limb_order_selects_how_finely_the_crown_parts`, `the_field_names_each_limb_by_its_densest_cell`, `the_field_counts_the_florets_the_tree_hangs`, `a_floret_outside_the_field_is_refused`, and the splat's `a_subtree_reaching_no_cell_centre_goes_whole_to_the_cell_it_stands_in`, `a_wide_subtree_is_spread_over_the_cells_it_reaches_and_sums_to_one`, `deposited_leaves_can_take_a_cell_from_the_kept_wood_s_limb`.
- Binding: `a_field_request_reads_the_plan_and_names_its_stages_for_every_preset`, `the_field_request_selects_the_limb_order_and_defaults_to_the_family`, `a_preset_id_is_refused_and_its_parameters_build`.
- Package (`src/field/field.test.ts`, `compile.test.ts`, `voxelize.test.ts`): "returns the main entry's field, with and without WebGPU", "refuses a preset's id and a malformed request by name", "is the core's grid for the same box", the five thinning rules' tests, the three compile tests.
- Principles: the field is a consumer of the drawn stage with "no stage, format or index of its own" (fn-257 R8); one pipeline, no second chain; a family it cannot read is refused, never placed another way [record: docs/pipeline.md; docs/principles.md].
- The field's shape as the site reads it: `FieldGrid` with `flags`, `woodRadius`, `leaves`, `limbs`, `flowers` [code: `src/field/types.ts`]. 0.2.0 is unreleased, so a change is not a break of a shipped API, but the site's port to the 25 Oct prerelease reads this shape [record: fn-317 Goal; fn-353 decision 3].

### 2.6 Open

Owner's:

- Whether "at or under 0.1.4's" for the grove is a bar or an aim. fn-351 made speed an aim ("never a stop") and dropped R9's grove sentence from P11 [record: fn-351 P1, P11, Rigor].
- Which species the grove must serve, and at what grid and count of trees; the site's numbers are its own [unknown].

Host's:

- What "the tree's cells" are: the grower's 0.5 m light lattice, Phase 5's counted cells, or the field's own grid. A field read from the lattice would be a second read-out beside `pipeline/field.rs`, which the fn-257 R8 sentence and the one-pipeline rule both touch.
- Whether a grove tree may stop growing short of its twigs once their leaf is counted, and whether that tree is "the same tree" under P4.
- Whether the limb id, which the voxelizer's thinning rules need, survives a read from cells.

Options, unranked: (a) keep the field as it is and let Phase 5's counting make growth cheap enough; (b) read wood from the drawn structure and leaf from the counted cells; (c) read the whole field from the grower's lattice, resampled to the request's grid; (d) grow the grove's trees in several workers from one compiled module, which the entry already allows, and report the wall time.

Measure first:

- The site's per-tree call on today's build, by fn-329's frozen protocol (`fn-329-…/PROTOCOL.md`, `probes/wasm.mjs`), for the species 0.1.4 has, at the field's 36 cells. The last numbers are from 3 Nov.
- How far the light lattice's leaf area agrees with the field's leaf count on the same tree, before any design rests on it [unknown].

---

## 3. Growth off the main thread, with or without WebGPU

### 3.1 What is required

- "the growth path downloads at most 250,000 Brotli bytes, and growth runs off the main thread with or without WebGPU. Owner, 1 Nov 2026." [record: fn-351 P11]
- "growth runs on the CPU off the main thread for every visitor, with or without WebGPU." [record: fn-321 R9]; "growth on the CPU for every visitor, choice A" [record: RULINGS.md, 1 Nov (P)]
- Phase 1's plan line: "The harness and the website draw CPU-grown trees, in a worker in the browser." The host moved the worker to Phase 6: "In the browser, Phase 1 grows on the main thread through the existing `setTree`. A Worker is Phase 6's, with the wire it needs." [record: PLAN section 5; fn-353, host decision 2]

### 3.2 What exists today

Wired:

- **The field entry runs in a worker and without WebGPU.** It loads only `telperion.wasm` through `TreeEngine`; "checked in Node without WebGPU" [code: `src/field/index.ts`, `src/browser/core.ts`; record: fn-351 log, 11 Nov 13:48].
- **`TreeEngine.build` is plain Wasm with one import** (`env.now`), so it runs in any worker; `TreeEngine.create` takes a compiled `WebAssembly.Module` [code: `src/browser/core.ts`].
- **A worker is exercised twice:** the probe's worker and the browser smoke test [code: `src/probe/field/worker.ts`, `tests/browser/field.mjs`]. "the only Worker in the repo" [record: PHASE1-SURVEY.md].

Partial:

- **The renderer grows on the page's thread.** `Renderer.setTree(family)` and `beginGrowth` / `growTo` / `draw` run `mesh::build` and `pipeline::growth` inside `telperion-render.wasm`; "Each call runs to its end before it returns" [code: `src/browser/render.ts`, `Growth`; `crates/telperion-render/src/web.rs`, `web/growing.rs`]. The cost was stated at Phase 1: "the main thread held while a tree grows" [record: fn-353 RESULT.md, item 5].
- **Growth by year exists only in the renderer's module.** `telperion.wasm` exports `build` and no begin, grow-to or settle [code: `crates/telperion-wasm/src/lib.rs`, exports; record: PHASE1-SURVEY.md, "The Wasm side"].

Absent:

- **A wire from a worker's tree to the renderer.** "A Worker running `TreeEngine` cannot feed the renderer today: `Output` carries no `Curve`, and `WebRenderer` accepts only a family's JSON. `WebRenderer::create` takes an `HtmlCanvasElement`; `web-sys` is built without `OffscreenCanvas`." [record: PHASE1-SURVEY.md, line 43; code: `src/browser/render.ts` header, "No geometry crosses this boundary"]
- **Threads inside the Wasm.** The target is `wasm32-unknown-unknown`; the build runs one thread, though rayon is compiled in [code: `scripts/build-wasm.mjs`; derived: `rayon_core` in the module, section 4.3].
- **A worker in the library or the harness.** The package ships none; the harness calls `setTree` directly [code: `harness/rust-stage.ts`, `harness/grow-in.ts`].

### 3.4 What earlier specs tried

- **fn-13**: "worker preparation and streaming" in the prototype; not merged [record: fn-13 Result].
- **fn-91** (done 27 Sep): browser medians from 1.6 to 7.6 s down to about 160 to 180 ms, by expanding on the GPU; all of that path is deleted now [record: fn-91 Result; fn-353].
- **fn-225** (growth off one core in the browser): closed as superseded, not implemented. It measured the spruce 40 to 70 at 14.2 s, about 8.9 s of it growth on one core [record: fn-225].
- **fn-243** (done 9 Oct): compiling GPU programs in the background, after Chrome's watchdog killed its GPU process on a synchronous compile [record: fn-243 Goal]. `createRenderer` still awaits the wood's programs before it resolves [record: fn-353 RESULT.md, task 3].
- **fn-256, fn-257, fn-276, fn-298 to fn-303**: growth on the device; deleted by the owner's ruling [record: RULINGS.md, "One grower"].
- **fn-353** (done 11 Nov): CPU growth bound for the browser on the main thread; `telperion/field` on the CPU [record: fn-353].
- **fn-291** (done 18 Oct): the CPU entry builds every shipped preset or refuses it by name inside 32-bit Wasm's 4 GiB [record: fn-291 Goal].

### 3.5 What must keep holding

- `every_shipped_preset_builds_through_the_main_entry`, `flowers_cross_the_binding_where_the_pipeline_grows_them`, `a_reduced_tree_reports_the_detail_it_kept` (binding).
- `every_shipped_preset_keeps_under_the_wasm_ceiling` (core, `pipeline/space/ceiling_tests.rs`): **failing on fn-363's branch** for Telperion and Laurelin at seed 42, 6.1 M and 5.4 M phytomers against 4,000,000; "Not loosened." [record: fn-351 log, 13 Nov 21:30]
- `scripts/test-dist.mjs`: every shipped preset at three seeds through `dist/telperion.js`, each under 3.5 GiB, and one case refused by name [code: its header].
- `src/browser/render.test.ts`: "a growth begins on the CPU, then grows and draws through its handle (fn-353)"; "resolves only once the renderer exists with the wood's programs compiled".
- `src/browser/module.test.ts`: "instantiates once when two renderers start together".
- The two compile-fail boundary tests: `crates/telperion-wasm/tests/boundary.rs`, `crates/telperion-render/tests/boundary.rs`.
- P4 across the wire: `a_year_is_a_state_not_a_horizon`, `the_tree_at_a_year_is_the_horizons_prefix`, `every_year_is_the_horizons_prefix` (space and core).
- "Typed Rust and TypeScript only under `src` and `crates`" [record: AGENTS.md, Code rules]. "Agents do not drive WebGPU in a browser (P)." [record: RULINGS.md]: a worker that feeds the renderer cannot be verified by an agent in a browser.

### 3.6 Open

Owner's:

- Whether the requirement covers only the site's field path (already in a worker, already without WebGPU) or also the package's renderer and the harness. P11 names "the growth path"; fn-353's decision 2 reads it as the renderer's too [record: both].
- Whether the package should ship the worker or document one; the site's worker is the site's [unknown].

Host's:

- What crosses from the worker: the `TreeMesh` (curve tables and leaf instances) as transferable buffers, the grown structure with the dress done on the page, or parameters and a handle with the renderer itself in the worker on an `OffscreenCanvas`.
- Where growth by year lives once growth is off the page: `telperion.wasm` has no year calls and the renderer's module has them.
- Whether one module or two do the work: today a page with a renderer downloads `telperion-render.wasm` (534,938 Brotli bytes), which holds a second copy of the core [derived: the render crate depends on `telperion-core`, `crates/telperion-render/Cargo.toml`].
- 32-bit Wasm's 4 GiB in a worker against the trees Phases 2 to 4 grow (the ceiling test above).

Options, unranked: (a) nothing new for the site, state that its path already meets the sentence, and move only the harness; (b) a curve and instances output on `TreeEngine.build` and a `submit` on `WebRenderer` that takes them; (c) the renderer in a worker with `OffscreenCanvas`; (d) growth by year exported from `telperion.wasm`, the renderer's copy removed; (e) several workers from one compiled module for many trees.

Measure first:

- How long the page's thread is held today per preset in the harness (`setTree`), on today's build [unknown since fn-353's oak 1.8 s native].
- The bytes a `TreeMesh` would carry across for each preset, since a transfer's cost follows them [unknown].

---

## 4. The 250 KB budget

### 4.1 What is required

- "The website's growth path downloads at most 250 KB (Brotli)." Owner, 1 Nov; the owner's words are "At most 250 KB", selected from options [record: PLAN P11; RULINGS.md; fn-321 Conversation Evidence].
- fn-351 P11: "at most 250,000 Brotli bytes" [record: fn-351 Acceptance].
- "`telperion.wasm` unchanged at 1,538,240 raw and 388,544 Brotli, which is over P11's 250,000 and is Phase 6's to bring down" [record: fn-351 log, 11 Nov 15:34].
- L2: "shipped artifacts inside `scripts/artifact-budgets.json`" [record: fn-351]. "A ceiling is set by the owner from what the product's consumers need; crossing one is an owner decision, never a routine raise." [code: `scripts/artifact-budgets.json`, note]
- Owner, 24 Oct, on the repository: "it's still 250k loc added which is a bit crazy. can we do another cleanup?" [record: RULINGS.md]

### 4.2 What exists today

- **The check.** `scripts/artifact-budgets.mjs` holds each listed artifact to a ceiling in raw bytes and, on a pull request, to 5% growth over its base [code: `scripts/artifact-budgets.json`, `artifact-budgets.mjs`]. It reads no Brotli size [derived: no match for "brotli" in `scripts/*.mjs`]. **Nothing holds P11 today.**
- **Ceilings (raw bytes) and baselines at fn-353:** `telperion.wasm` 1,600,000 (1,538,240); `telperion-render.wasm` 3,700,000 (2,220,865); `telperion.js` 200,000 (3,993); `presets.js` 200,000 (187,035); `field.js` 16,000 (1,299); `module.js` 70,000 (60,291); `voxelize.js` 16,000 (4,573) [code: `scripts/artifact-budgets.json`]. The render module's and the scripts' ceilings were raised for this release "and a small margin ... fn-242 brings the modules down after it" [code: same file, reasons].
- **A chunk the budget does not list.** The build at fn-353 emitted `dist/core.js` (4,092 bytes), which `field.js` and `telperion.js` import [derived: `ls` and import lines of `.worktrees/fn-353/dist`]. Whether the check passes over an unlisted file or fails on it was not read [unknown].
- **The build.** `cargo build --release --target wasm32-unknown-unknown -p telperion-wasm`, copied as built: no `wasm-opt`, no strip step [code: `scripts/build-wasm.mjs`]. The release profile is `lto = true`, `codegen-units = 1`, with no size setting and no `strip` [code: `Cargo.toml`].

### 4.3 What the 388 KB is

Measured here on existing outputs, nothing built. Sizes of `.worktrees/fn-353/dist` (built 11 Nov 15:04 to 15:30 at `83230d683`) and `twiggy top` on `.worktrees/fn-363/dist/telperion.wasm` (built 13 Nov 18:40, 1,541,239 bytes, 391,751 Brotli). All figures in this subsection are [derived] from those files unless tagged otherwise.

What the site's field path fetches, Brotli at quality 11:

| File | Raw | Brotli |
|---|--:|--:|
| `telperion.wasm` | 1,538,240 | 388,544 |
| `module.js` | 60,291 | 12,393 |
| `presets.js` (if the page uses presets) | 187,035 | 8,681 |
| `voxelize.js` | 4,573 | 1,703 |
| `field.js` | 1,299 | 587 |
| `core.js` | 4,092 | not measured |

So the path is about 403,000 Brotli bytes without the presets and about 412,000 with them, and the Wasm is 94% of it. `telperion-render.wasm` (534,938 Brotli) is not on the field path. Which files count toward "the growth path" is not written down [unknown].

Inside `telperion.wasm` (raw bytes, by section and by crate from the name section):

| Part | Raw bytes | Share |
|---|--:|--:|
| `telperion_space` (the grower) | 421,472 | 27% |
| `telperion_core` | 411,838 | 27% |
| the function-names section | 235,631 | 15% |
| `.rodata` | 214,450 | 14% |
| Rust `core`, `alloc`, `std`, `compiler_builtins` | about 125,000 | 8% |
| `serde_json` and `serde_core` | about 31,400 | 2% |
| `telperion_wasm` (the binding) | 28,890 | 2% |
| `libm`, `dlmalloc` | 33,592 | 2% |
| `rayon_core`, `crossbeam_epoch` | 16,899 | 1% |

Largest modules: `telperion_space::grow` 125 KB (with `grow::shoot` 37 KB and `grow::cycle` 21 KB); `telperion_core::pipeline::foliage` 124 KB; `pipeline::surface` 70 KB; `closed_form` (predictor, events, steps) about 46 KB; `catalogue` and `catalogue::mount` 58 KB; `geometry::place` 28 KB; `pipeline::drawn` 25 KB; `fine::canonical` 19 KB; `species` 18 KB; `pipeline::field` 11 KB.

One thing measured: with every custom section removed the module is 1,305,308 raw and **367,708 Brotli**. The names are 15% of the raw file and 24,000 Brotli bytes, because they compress well. Stripping alone leaves it 118,000 over.

Candidates to split or delete, from reading imports and exports; none is ranked and none is measured beyond the table:

- **The names section** is shipped. fn-150 stripped names in `build-wasm.mjs` for the slim module; today's script does not [record: fn-150 Result; code: `scripts/build-wasm.mjs`]. fn-101 left open "Strip the 35 KB function-names section" [record: fn-101 Open].
- **The `preset` and `catalogue` exports.** The module exports `preset(id)` and `catalogue()`; `TreeEngine` never calls them, and `scripts/build-wasm.mjs` calls `catalogue()` once at build time to write `presets.generated.ts` [code: `crates/telperion-wasm/src/lib.rs`; `src/browser/core.ts`, `Exports`; `scripts/build-wasm.mjs`]. They keep every preset's table and the catalogue's names and notes reachable; strings in `.rodata` show preset names, notes and parameter labels. How much of the 214 KB that is was not separated [unknown].
- **What a field request never runs.** A field request places no leaf and builds no surface [code: `crates/telperion-wasm/src/generate.rs`, header], yet the one module carries placement, the surface sweep, chunks, bloom and the leaf element (`pipeline::foliage` 124 KB and `pipeline::surface` 70 KB together 14% of the file). The core has a `geometry` feature that leaves those out, but "The field needs the drawn tree, so without the `geometry` feature it is refused" [code: `crates/telperion-core/Cargo.toml`; record: docs/pipeline.md].
- **rayon in a one-thread module** (about 17 KB) [record: fn-242 R2 named this lever: "rayon out of one-thread Wasm behind a feature"].
- **Formatting and JSON.** `core::fmt`, float formatting, `serde_json::Value` and the request and metadata as JSON text [code: `generate.rs`, `lib.rs`]. fn-242 R2: "formatting kept out of the release path" [record].
- **`closed_form`** (about 46 KB), the GreenLab count formulas behind the pre-growth size check (`foreseen`) [code: `crates/telperion-space/src/closed_form.rs`; `pipeline/space/growth.rs`]. Whether anything else on the build path reads it was not traced [unknown].
- **`catalogue` rows' prose.** `catalogue::mount` says "a build that never documents a row never carries its prose" [code: `crates/telperion-core/src/catalogue/mount.rs`]; whether the shipped module is such a build was not checked [unknown].
- **An optimiser pass.** fn-227's best build used `wasm-opt -O3` [record: fn-242 Goal]; `wasm-opt` is not installed on this machine [derived: `which`].
- **Phase 5 and Phase 7 deletions**: the fidelity dial and coarse rows, `path_end`, the `retained` setting [record: fn-351 Build order; log 13 Nov 21:30]. Their size is not known [unknown].
- **`module.js`** (60,291 raw) holds the renderer's wasm-bindgen glue; `field.js` imports it because the shared chunk also holds the serializer and the Wasm loader [code: `vite.config.ts`, `SHARED`; derived: `field.js` import lines]. It is 12,393 Brotli bytes of a field page's download that the field does not run.

### 4.4 What earlier specs recorded about size

- **fn-101** (22 Sep): the generator Wasm was 1,242,383 raw / 344,254 Brotli; a slim field module shipped [record: fn-101 Goal].
- **fn-150** (27 Sep): `telperion-field.wasm` 344,798 raw / 107,758 Brotli, the size 0.1.4 shipped; its first slim build had grown the module 48% in one PR [record: fn-150 Result; `artifact-budgets.json`, growthReason].
- **fn-178** (27 Sep): the two limits, ceiling and growth share, after three features used the old 5% margin in a day [record: fn-178 Goal].
- **fn-227** (9 Oct, closed as superseded): the field module had reached 1,171,132 bytes against a 632,580 ceiling; "it holds only about 4.5 KB of the old scaffold grower; the size is the new engine". Its SIZE.md is no longer in the repository's evidence [record: fn-227 Goal; derived: no `fn-227` directory under `.flow/evidence`].
- **fn-242** (re-scoped 16 Oct, closed 11 Nov as folded into this goal): "packaging alone cannot bring `telperion-field.wasm` near 0.1.4's size: the best speed-keeping build (the full tree compiled out, `wasm-opt -O3`) is 1,064,107 bytes stripped, 332 KB over the wire with brotli, against 0.1.4's 108 KB. Half the binary is the engine (`telperion_space`, 616 KB before the optimiser), its `grow` module 241 KB with the stepped grower 117 KB." [record: fn-242 Goal]. Its re-scope: "the final package's reachable modules and transfer sizes measured, unreachable dependencies and needless entry coupling removed, and the ceilings fn-237 raised for 0.2.0 brought back down." [record: fn-242 Re-scope]
- **fn-257** (16 Oct): the slim field module was removed; the package ships two modules [record: fn-242 Re-scope note].
- **fn-259 RELEASE.md** (19 Oct): the artifacts were "over every ceiling; the owner's decision" [record].
- **fn-317** (25 Oct): presets moved to their own entry so a page can read a preset "without the 1.5 MB CPU engine" [record: fn-317 Goal].
- **fn-353** (11 Nov): `telperion-render.wasm` 3,553,309 to 2,220,865 raw, 841,705 to 534,938 Brotli; `telperion.wasm` 389,561 to 388,544 Brotli [record: fn-353 RESULT.md].
- The engine's share today (421 KB of 1,305 KB of code and data, about a third) against fn-242's "half" is a different module and build, so the two are not the same reading [derived].

### 4.5 What must keep holding

- `scripts/artifact-budgets.test.mjs`: "fail an artifact over its ceiling, naming the ceiling and its reason", "fail an artifact the recipe did not build", "fails fn-150s rejected slim build, naming the base and the growth", "passes fn-170s fork rule under the budget file as it stands", "passes exactly the share and fails one byte more", "passes a growth the Decisions section names, and only that one".
- The package check in the gate: `npm run build`, then `node scripts/artifact-budgets.mjs` [record: AGENTS.md, Gates].
- `every_shipped_preset_builds_through_the_main_entry` and `scripts/test-dist.mjs`: every shipped preset's every artifact through each package entry [record: docs/principles.md, "How they are held"].
- One pipeline: a smaller module may not be a second build chain. fn-150 exists because the slim crate "kept its own build chain" and could not serve the palm [record: fn-150 Goal].
- The core without geometry builds and its lib tests pass [record: AGENTS.md, Gates].
- `presets.generated.ts` is held to what the Rust presets render, through the `catalogue()` export [code: `scripts/build-wasm.mjs`]; removing the export moves that check.
- Speed: fn-227's levers were judged "speed-keeping"; the plan's only blocking speed rule is "no step makes a tree slower or heavier than at the last landing" [record: PLAN section 4].

### 4.6 Open

Owner's:

- What "the growth path" counts: the Wasm alone, or every file a field page fetches, and whether `presets.js` is on it.
- Whether 250,000 holds for the site's field path only or also for a page with the renderer (534,938 for that module alone).
- The ceilings in `artifact-budgets.json` are the owner's; lowering them to hold the new size, or adding a Brotli limit, is his to set [code: the file's note].
- Whether a second, smaller published module is acceptable again. "The three published Wasm entries stay until their consumers and a migration justify a change." [record: fn-242 Re-scope]

Host's:

- Whether 250,000 is reachable on one module that also builds meshes, or only by a module that cannot. fn-242's best was 332 KB with the full tree compiled out; today's is 368 KB stripped with everything in.
- Whether a field-only build is one pipeline with outputs not compiled in, or a second chain (fn-150's lesson).
- Whether the check moves to Brotli bytes, and where the number is pinned.

Options, unranked: strip names and other custom sections; a size-minded build profile for the Wasm; `wasm-opt`; take the catalogue and preset exports out of the shipped module and generate the presets from a native tool; a feature that leaves placement, the surface and bloom out of a field module; rayon and formatting behind features; split `module.js` so the field does not fetch the renderer's glue; wait for Phase 5 and 7's deletions and measure again; return to the owner with the measured floor if it stays over.

Measure first:

- Each lever's Brotli bytes one at a time on the same commit, and the tree's build time beside it, before any is combined. Only the first (custom sections: 24,000) is measured.
- What `.rodata` holds, by symbol.
- The size after Phase 5's deletions, since the module being cut is not yet the module that ships.

---

## 5. The native entry with a headless test

### 5.1 What is required

- Owner, 1 Nov: "what we're builing must have a modular path to build a plugin or smth to be imported by game engines to generate trees at scale." and "Any imports to game engines are out of scope for this goal." [record: RULINGS.md]
- Owner, 25 Oct: "We shouldn't hand a species name to the generator and if it's not typed we can't do it." and "Remove everywhere" [record: RULINGS.md]
- fn-351 P10: "a native entry with no browser, harness or engine dependency takes parameters and a seed, never a species name, and returns the tree's data at any age; a headless test grows the oak through it. No plugin is built." [record: fn-351 Acceptance]
- fn-321 R11 adds: "and its twigs for any region asked" [record: fn-321 R11, a paraphrase]. fn-351's P10 does not repeat it.
- STRATEGY: "One lean Rust generation core serves browser Wasm and native integrations, with small boundaries between tree state, generation rules and output representations. Consumers request structure, meshes, instances or spatial fields as needed" [record: STRATEGY.md, line 50].

### 5.2 What exists today

Wired:

- **`pipeline::build(&Family, Request) -> Built`**: the one build; a request names its outputs (surface, foliage, structure, field) [code: `crates/telperion-core/src/pipeline.rs`]. `mesh::build(&Family) -> TreeMesh` is the same build as one mesh [code: `crates/telperion-core/src/mesh.rs`].
- **`pipeline::growth`**: `Params::of(&Family)`, `begin(&params, seed, horizon)`, `grow_to(&mut state, age)`, `settle(&state, at) -> Tree`, `colouring`, `mesh(&state, at, &family)`; a tree held between years and drawn at any age within its last year, past the first horizon too [code: `crates/telperion-core/src/pipeline/space/growth.rs`].
- **The crate's dependencies** are `telperion-space`, `slotmap`, and `serde` only under the `json` feature: no browser, GPU or harness crate [code: `crates/telperion-core/Cargo.toml`].
- **Parameters, not names.** `Family` is the parameter table; `params::parse` reads it from JSON under `json`; the binding refuses a preset's id [code: `crates/telperion-core/src/params.rs`; test `a_preset_id_is_refused_and_its_parameters_build`].
- **Native callers already use it:** the film, `grow_in`, `unfold`, `frames`, `headless` examples [code: `crates/telperion-render/examples/`].
- **A C ABI exists for Wasm.** `request_alloc`, `request_ptr`, `build`, `metadata_ptr`, `buffer_ptr`, `chunk_ptr`, `release`: JSON in, buffers by slot out. The crate is `cdylib` and `rlib` [code: `crates/telperion-wasm/src/lib.rs`, `Cargo.toml`].

Partial:

- **The seed sits in the parameters** (`family.skeleton.seed`) and the age in `family.age`; `growth::begin` takes the seed as its own argument, `pipeline::build` reads both from the family [code: `growth.rs`, `mesh.rs`, `src/field/index.ts` doc]. "Takes parameters and a seed" is met in two different shapes.
- **Name lookups remain beside the entry.** The Wasm ABI exports `preset(id)` by numeric id and `catalogue()`; `examples/headless.rs` is "The only place in the renderer that resolves a named family to parameters" [code: `crates/telperion-wasm/src/lib.rs`; `crates/telperion-render/examples/headless.rs`]. Whether a numeric preset id in the ABI counts as "a species name" is not ruled [unknown].
- **"A headless test" has two candidates already.** `tests/headless.rs::a_tree_reaches_the_pixels` renders offscreen and needs a GPU [code: `crates/telperion-render/tests/headless.rs`, `src/headless.rs`]. The core's public-build tests need none [code: `crates/telperion-core/tests/`]. No test was found that states P10's sentence: the oak, from parameters and a seed, grown through one named entry to several ages, with no GPU [unknown: not found by name].
- **Data returned.** `Built` and `TreeMesh` hold the wood as a curve and as chunked meshes, leaf instances, structure and field; the tree-wide rows an engine would need (bounds, the leaf element, materials) are in the family and the mesh [code: `pipeline.rs`, `mesh.rs`]. No document names this as the engine's contract; fn-125's plan layout doc is the nearest [record: fn-125 R1].

Absent:

- **A stable native surface named as the entry**: no crate, module or header is marked as what an engine links, and no native (non-Wasm) C ABI build is produced or tested [derived: workspace members and package scripts].
- **Twigs for a region asked** [record: fn-351 log, 13 Nov 03:25, "nothing today builds wood on demand from a key"].
- **Any engine plugin**, by ruling.

### 5.4 What earlier specs tried

- **fn-17** (first external engine integration): open since 8 Sep, never started [record: fn-17].
- **fn-41** (the surface function an engine can light), **fn-125** (the generator hands engines a tree they only draw): fn-125 is open; its two-executor contract was built and its GPU executor since deleted [record: fn-125; fn-353].
- **fn-126** (the CPU reference cull and placement), **fn-291** (the CPU entry builds every preset or refuses by name): done [record].
- **fn-158** made the stages private so a second chain does not compile; `executor` is the door for tools and tests [record: docs/pipeline.md].
- **fn-317** (done): every entry takes parameters; ids refused by type and at run time [record: fn-317 R1, R2].
- **fn-318** (a native command that writes a field from a parameter file): closed as moot with the GPU grower [record: PLAN section 3].
- **fn-353** (done): `GrowthState` "owns what it reads, so a caller can hold it between calls"; **fn-355**: growth past the first horizon [code: `growth.rs` docs].

### 5.5 What must keep holding

- The boundary: `crates/telperion-render/tests/boundary.rs` and `crates/telperion-wasm/tests/boundary.rs` (compile-fail, each with a positive control).
- `a_preset_id_is_refused_and_its_parameters_build`; the package's "refuses a preset's id and a malformed request by name".
- P4: `a_year_is_a_state_not_a_horizon`, `the_tree_at_a_year_is_the_horizons_prefix`, `every_year_is_the_horizons_prefix`, `a_tree_drawn_within_its_year_unfolds_from_the_last`, `a_stem_drawn_within_its_year_never_dips`.
- Repeatability across builds: the fixed representations of FINE-CONTRACT.md, "so a native build and a Wasm build grow the same tree" [record: docs/pipeline.md, "The one cycle and its grower"].
- `generation_limit_guard` (no cap of the generator's own; a tree past the machine is refused by name) [record: AGENTS.md, "No artificial limits"].
- `catalogue_identity` and `parameter_reference` (the wire's rows and their document).
- The core without geometry builds: an engine that draws its own geometry reads the solved tree, the plan and the field [code: `Cargo.toml` feature note].
- `a_tree_reaches_the_pixels` (the existing headless render).

### 5.6 Open

Owner's:

- Whether today's Rust API (`pipeline::build`, `pipeline::growth`) is the "native entry" he means, or whether an engine-facing C ABI or a crate of its own is. The ruling says "a modular path", and "No plugin is built".
- Whether the ABI's numeric `preset(id)` goes under "Remove everywhere".

Host's:

- Whether fn-321's "twigs for any region asked" is still in scope; fn-351's P10 dropped the clause.
- What "the tree's data" is for an engine: structure, curve, chunks, instances, field, and which of them the headless test asserts on.
- Whether seed and age are arguments of the entry or rows of the family, since both forms exist.
- Whether the entry is the same surface the Wasm binding wraps, so the browser and a native engine share one contract.

Options, unranked: (a) name the existing `pipeline::build` and `pipeline::growth` as the entry, document it, and add one test that parses the oak's parameters and grows it to 40, 80 and 120 with no GPU; (b) a small facade crate with only the entry and its types; (c) build the existing C ABI natively and test it from a C-shaped caller; (d) an example binary that reads a parameter file and a seed and writes the tree's data, with the test driving it.

Measure first:

- Nothing must be measured to choose. A check that should precede the spec: compile the core alone with default features off and with `json`, and confirm no dependency beyond `telperion-space`, `slotmap` and `serde` [unknown until run; the gate already builds the core without geometry].

---

## 6. The forest rig and the frame measures

### 6.1 What is required

- STRATEGY, Key metrics, "Frame": "a shipped hero tree renders inside 2 ms of GPU time at native pixel ratio on the named machine, an RTX 3080, and the whole vegetation layer of a thousand-tree forest inside 4 ms once a forest rig exists to measure it; GPU timer queries in the harness rig, budgeting a slice of the game frame for vegetation." [record: STRATEGY.md, line 28]
- fn-351 P13: measured "in the harness ... and reported" [record]. fn-321 R8 added "holds under 1 GiB of device memory" and "on a forest rig this goal builds" [record: fn-321 R8]; fn-351 does not repeat the 1 GiB.
- Owner, 9 Nov: "Realistic trees generated and rendered fast. That's the goal. Strategy states the goal also about many trees in game engines etc." [record: RULINGS.md]
- P3's aim: "a thousand oaks at 40 in 10 s" across the cores, native [record: PLAN section 4, item 1; host's number].
- The capture rules: "Never start a 1,024-tree capture until the specific defect reproduces and is fixed on the 8-tree forest with a red/green test. At most one full-forest capture per commit." "Read summaries, not receipts." [record: AGENTS.md, Token and evidence budget]
- "Heavy and GPU work runs through `telperion-heavy`, one GPU job at a time" [record: fn-351 Boundaries].
- L3: "fn-237 R2's measured release checks, cold starts with the shader cache off" [record: fn-351].

### 6.2 What exists today

Wired, for one tree:

- **GPU timing with a verdict.** Timestamp pairs around the vegetation pass, the selection pass, the sun's depth pass and the wood's surfacing; p50 and p95; a verdict of valid, unavailable, disjoint or contended, and no millisecond figure without a valid one; the same session runs in a browser [code: `crates/telperion-render/src/timing.rs`, `timing/report.rs`, `timing/collect.rs`; `src/browser/render.ts`, `TimingReport`].
- **The harness shows it.** `stage.timing()` and `describeTiming`; an orbit session reports wall-clock frames too [code: `harness/GrowerDev.tsx`, `harness/timing.ts`, `harness/rust-stage.ts`; `web.rs`, `timing`, `orbit`].
- **A native frame probe.** `examples/frames.rs`: `fresh_ms`, `warm_ms`, frame time per view (whole, mid, close, bare), peak RSS and device memory, one JSON line, with `SEED` and `AGE` [code: `crates/telperion-render/examples/frames.rs`].
- **Levels in the record.** What each level drew, coarsest first [code: `TimingReport.levels`].
- **The scoreboard** (fn-352): a fixed panel drawn and measured at every landing [record: fn-351 Rigor; `fn-352-…/SCOREBOARD.md`]. Whether it carries frame times was not read [unknown].

Absent:

- **Any forest.** "One tree, one sun, one GPU." The renderer holds one submitted tree; `submit` "uploads one tree in place" [code: `crates/telperion-render/src/lib.rs`, module doc; `Renderer::submit`]. No code draws two trees, places instances of a tree, or shares buffers between trees [derived: no match for "forest" in the render crate, `src` or `harness`].
- **The 8-tree forest** the capture rule names [derived: same search].
- **A frame number on today's trees.** "P11, P13: Not measured." [record: PLAN section 2]
- **A named measure of a thousand trees' build**, P3's third aim: no tool was found that reports "1,000 oaks at 40" [unknown: not found by search; the scoreboard's tooling was not read].

### 6.4 What earlier specs tried

- **fn-13** (closed 8 Sep, prototype): a 1,024-tree capture existed; memory by ownership with farthest-first eviction, 4.28 to 1.73 GB retained. "Not delivered: a usable forest ... or a qualified performance number." Its task 5 "consumed a full weekly quota on 22 full-forest GPU captures", which is why the capture rules exist [record: fn-13 Result; AGENTS.md].
- **fn-22, fn-23** (9 Sep): the oak hero from 18 ms to 1.750 ms native; browser 2.315 ms GPU [record: fn-23 Goal, Result]. Old generator.
- **fn-14, fn-27** (11 to 12 Sep): the oak's total at 4.949 ms against a 3.8 ms working bound, 1.813 ms of it the sun's pass; coarse casters recovered it [record: fn-27 Goal].
- **fn-176**: written for "a forest, fn-86's and fn-6's target"; open, unbuilt [record: fn-176 Goal].
- **fn-208** (5 Oct): 80-year trees, frames 6.7 to 38.6 ms after the curve path; surfacing 2 to 10.5 ms of it [record: fn-208 Result].
- **fn-246** (9 Oct): the fps table in section 1.4; Telperion close 25.0 ms of vegetation (wood about 9, leaves and florets about 16); the sun map reused when nothing moves [record: fn-246 Result].
- **fn-259** (19 Oct): the release scorecard through `frames.rs`: builds per preset and device peaks (Telperion 2.6 GB) [record: fn-259 RELEASE.md].
- **fn-285** (large mature trees): asked for frame time whole and close per species against age; closed 11 Nov as folded into this goal [record: fn-285 R1; PLAN section 3].
- **fn-237 RELEASE-CHECKS.md**: cold compile of each shader entry with the driver's cache off (`compile_size::tests::every_entry_made_cold_alone`) and the inlining ceiling [record].
- None of these measured a hero tree of the current engine after Phases 1 to 4, and none since fn-13 drew more than one tree [derived].

### 6.5 What must keep holding

- `tests/timing.rs`: `every_verdict_is_reached_by_the_samples_that_define_it`, `a_valid_record_carries_its_numbers_and_an_invalid_one_carries_none`, `a_valid_record_carries_every_pass_the_levels_and_the_orbit`, `a_reused_sun_map_costs_zero_and_says_so`, `nothing_a_session_did_not_earn_reaches_a_record`, `the_wall_clock_is_the_one_number_the_gpu_verdict_does_not_govern`, `a_session_on_a_device_never_reports_a_percentile_it_did_not_earn`, `a_device_session_times_the_selection_pass_and_counts_what_it_chose`.
- `tests/sun_cache.rs`; `redraw.rs`'s `an_idle_canvas_draws_once_and_then_nothing`.
- `tests/submit.rs`: `a_tree_is_judged_on_the_allocation_it_needs_not_the_bytes_it_holds`, `every_buffer_that_will_not_fit_is_refused_by_name_and_by_size`.
- `compile_size`: `every_entry_inlines_under_the_ceiling`, `every_entry_made_cold_alone`.
- `tests/concurrent_request.rs::devices_asked_for_at_once_all_arrive`; the whole-tree GPU lock (`whole_tree.rs`), which makes whole-tree GPU probes take turns.
- The capture rules and the gate's budgets (a test's wall time 120 s, 4 GiB) [record: AGENTS.md]. A forest test inside the gate meets both.
- "Timing and memory are measured on named hardware by the spec that changes them, never per push." [record: docs/principles.md]
- "Agents do not drive WebGPU in a browser (P)." [record: RULINGS.md]. "In the harness" for an agent means the native probe on the harness's route (`mesh::build`, then `Renderer::submit`), as `frames.rs` says of itself.

### 6.6 Open

Owner's:

- What a "forest of 1,000 oaks" is for the measure: a thousand different seeds, or fewer distinct trees instanced; what spacing, camera and resolution; whether trees shade each other. STRATEGY says "forests of individually generated specimens".
- Whether a forest the renderer can draw ships in 0.2.0's harness and package, or is a measuring rig only.
- Whether the 1 GiB of device memory from fn-321 R8 still stands.

Host's:

- Whether the rig is a native example, a harness page, or both, and whether the renderer's "one tree" becomes many or the rig composes one buffer set from many trees.
- Memory: one oak at 40 is 6.0 M wood vertices on the CPU mesh and 272,095 nodes [record: fn-353 RESULT.md]; a thousand of them cannot be held whole, so the rig's design and item 1's levels are one question.
- What the 8-tree forest is, since the capture rule requires it before any 1,024-tree run.
- Where P3's "a thousand oaks at 40 in 10 s" is measured; today one oak at 40 takes about 1.8 s native on one thread [record: fn-351 log, 11 Nov 15:34], so a thousand is about 1,800 core-seconds before any change, against an aim of 10 s of wall time [derived: 1,000 × 1.8 s].

Options, unranked: (a) the measure first on one tree with `frames.rs` at whole, mid and far distances, and the forest's cost projected from them, stated as a projection; (b) an 8-tree native rig that submits several trees into one renderer, then scaled; (c) a rig of a few distinct trees instanced to a thousand, reported as that; (d) the report alone for the candidate, with the gap and the approaches, as the speed ruling allows for an aim.

Measure first:

- The hero oak's GPU time and device memory on today's build, native, at the harness's resolution and pixel ratio, valid verdicts only.
- The same oak at the distances a forest would put it, to see what a far tree costs today with no tree-level level of detail.
- The build of N oaks at 40 across the cores, for P3's third aim.

---

## 7. What the six share

- **Order.** Items 1, 2 and the "twigs" clause of 5 are worded on counted foliage, which is not built. Items 3 (the wire), 4 (the module) and 5 (the entry's contract) all turn on what one build hands over and in what form; a decision on one sets the other two [derived].
- **One module or several** runs through 3 and 4: the page's renderer carries its own copy of the core, and the field path carries the mesh code it never runs.
- **What no agent can check.** A worker feeding a WebGPU canvas in a real browser, and the site's own cold load, are the owner's to look at [record: RULINGS.md; fn-351 log, 11 Nov 13:48, "Not verified, because agents do not drive WebGPU in a browser, and carried to the candidate's list"].
- **Not read for this survey:** the 49 specs below fn-190 beyond those cited; `fn-246 FPS.md` and `fn-259 VERDICT.md` in full; the scoreboard's files; CI's workflow files; `tests/browser/render.mjs` and `growth.mjs` bodies; the render crate's `web.rs` and `lib.rs` bodies beyond their public functions and module docs.
