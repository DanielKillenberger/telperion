# Coarse shadow casters and a filtered shadow

## Conversation Evidence

> user (2026-09-11): "should we optimize performance after fn-14, before 26? Or do we have budget to render fn-26 at 60fps for one hero tree"
> user (2026-09-11): "ok let's capture that shadowcaster optimization spec then. Would it be fitting to also fix the noise that's in the canopy atm? look at the crown it looks like dithering/aliasing still."
> user (2026-09-11): "omg this is so much better" (on the page rebuilt at four samples a pixel)
> owner (2026-09-12, planning): gate the oak, record the spruce; needle aggregation becomes a named follow-on.

Captured 2026-09-11 at the end of fn-14's run and planned 2026-09-12 at short depth with no review backend, per the owner's routing. fn-14's report records the oak's native total p50 at 4.949 ms against the 3.8 ms bound, of which the sun's depth pass is 1.813 ms, and the spruce's browser orbit at a wall p50 of 30.0 ms, of which the same pass is 13.51 ms. Halving the map in fn-14 bought half a millisecond because the pass submits the full-resolution wood and every needle: the cost is geometry, not raster. The crown's self-shadow is one hardware comparison, four taps at a 1,024-texel map over the whole crown, so each leaf gets a near-binary verdict at leaf scale.

## Overview

Recover fn-14's frame budget on the oak before fn-26 by drawing less into the sun's depth pass and filtering what the crown reads back from it. The oak's native frame returns under its bound; the spruce's numbers are recorded beside fn-14's and its shadow pass is expected to fall, but the spruce's 60 fps is not this spec's gate; the crown's self-shadow reads as dapple rather than salt. fn-26 depends on this spec.

## Architecture & Data Models

- **Runs ordered by radius, casters as a prefix.** The wood surface builder already emits vertices and indices one path run at a time. It now emits runs in descending order of each run's largest radius and records a run table beside the mesh: for each run, its first index, its index count and its largest radius. The sun's depth pass then draws one prefix of the wood index buffer: every run whose largest radius is at or above the caster threshold. One draw, no second index buffer, no wood level ladder; a threshold change is a binary search over the table on the host. The threshold in metres is the row's caster texels times the world size of one map texel at the fit, so one number serves a young tree and an old one. The trunk's run keeps its thin apex; only whole side runs drop. [planning]
- **A coarse foliage caster.** The depth pass draws every k-th placement of the buffer at the element's coarsest level, k the row's caster stride, with each kept leaf's caster quad scaled about its centre by the square root of k so the map's coverage is preserved. The subset is fixed by buffer index, never by the per-frame selection, so orbiting cannot change which leaves cast. The blade and the needle go through the same rule. [planning]
- **A filtered comparison.** The shared shadow read widens from the hardware four taps to a square kernel of hardware comparisons, its radius in texels on the row, and the receiver's sample point moves along its normal by the row's normal offset in texels before the lookup. Taps that fall outside the map count as lit. Both crown shaders and the ground disc read through the same function; the depth pipeline's constant bias stays. [planning]
- **Timing unchanged, counts added.** The third timestamp pair still times the pass. The renderer exposes the caster's triangle and instance counts, and the timing record carries them beside the full counts; they are counts, not percentiles, so they are present on an invalid record too. The sun's pass stays out of the frame stats. [planning]

## API Contracts

- **Four scene row values**, in the row's one field table with wire name, range and refusal words: `casterTexels` (0 to 8, 0 draws every run), `casterStride` (1 to 64, 1 draws every placement), `shadowFilterTexels` (0 to 3, 0 is the hardware four taps), `shadowNormalOffset` (0 to 4 texels). They travel through the headless scene flag and the page's scene setter like every other value; the browser row type is generic, so no line lands there. Validation rejects a value outside its range naming the field; an unknown field is refused naming the real ones. [planning]
- **The run table** joins the wood mesh contract as an additive field: per run, first index, index count, largest radius in metres, runs in descending radius. The wasm slots are unchanged and do not expose it. [planning]
- **Timing record** keeps every field and adds `caster_triangles` and `caster_instances`; the browser record type declares them together with the fields fn-14 left undeclared. [planning]

## Edge Cases & Constraints

- **No species path.** Threshold, stride, kernel and offset are numbers on a row; nothing in the pass knows a needle from a blade. The no-anatomy-word gate extends to the shadow, wood and shared shader files. [strategy:Surface and rendering at scale]
- **No shimmer.** The caster prefix is fixed at submit and the stride is fixed by buffer index; a row change re-derives both deterministically. Orbiting changes neither. [strategy:Surface and rendering at scale]
- **No thinning.** A stride whose holes show in the ground shadow, or a threshold that drops a limb's shadow, is too coarse; the caster quad scale and the kernel are the two levers before lowering either value. Judged in the hero still and under orbit beside fn-14's stills. [planning]
- **Degenerate values are legal, not errors.** A threshold above the trunk's radius or a stride above the placement count empties the caster set inside its range; the default row never does, and the shadow test pins that the default set casts. [planning]
- **The filter costs per pixel.** The kernel's cost lands in the vegetation pass and counts against the same bound; the pass split in the record shows where the time moved. [planning]
- **Coupling named.** The stride is coupled to placement order; a later reordering of the placement buffer changes which leaves cast but not their share. The prefix is coupled to the surface builder; a later change to run radii re-tunes the threshold. Vertex motion added later must move the caster set by the same rule. [planning]
- **Identity pins re-pinned once.** Emitting runs in radius order permutes wood vertices and indices; the core's identity pins move once with that reason recorded, counts unchanged. The clay still's pin is judged at its existing tolerance and is not tightened. [planning]
- **Budget discipline.** Small before large, one timing run per change, at most four images viewed, no forest captures, per the project's evidence rules. [planning]

## Quick commands

```bash
cargo test --release --workspace                                                   # run table, prefix, stride, kernel, record, pins
cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --size 1600x1000 --out /tmp/fn27-oak.png --timing /tmp/fn27-oak-timing.json
cargo run --release -p telperion-render --example headless -- --preset norway-spruce --seed 7 --size 1600x1000 --out /tmp/fn27-spruce.png --timing /tmp/fn27-spruce-timing.json
cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --scene '{"casterTexels":0,"casterStride":1}' --out /tmp/fn27-oak-full.png
```

## Boundaries / non-goals

- Sub-pixel leaf coverage noise, many leaves under a pixel at hero distance, is a coverage problem, not a shadow problem; it gets its own spec if it remains at four samples. [user]
- Needle aggregation for the spruce is a named follow-on; this spec records the spruce and does not gate it. [owner]
- No second light, no cascade, no second shadow map, no second index buffer. [planning]
- No wood level ladder; the prefix is the caster, not a simplification of the drawn wood. [planning]
- Bark relief, veins and transmission are fn-26's. [user]

## Strategy Alignment

Active tracks served by this plan:
- **Surface and rendering at scale** — shadow casting joins simplification and filtering as a coarsening chosen by measured radius and a fixed stride, with no species path and no stepping, thinning or shimmer under orbit.
- **The core and integration** — the run table is an additive extension of the engine-neutral mesh contract; the wasm slots are untouched.

## Decision context

- Fix before fn-26 rather than raise the bounds: fn-26's budget rule inherits fn-14's 3.8 ms and would stop on its first timing run; fn-26's transmission also reads the shadow, so it must call this spec's filtered function, and its note that the map is not re-timed there is superseded. [planning]
- Runs in radius order over a per-run draw list or a second caster index buffer: a prefix is one draw and zero bytes; a draw per kept run is thousands of calls a frame; a second index buffer doubles the oak's 99 MB of wood indices. The price is one permutation of vertex and index order and a re-pin. [planning]
- Stride with a scaled caster quad over a hashed subset: a hashed subset needs a compacted instance list, which is the second selection dispatch the sun's pass exists to avoid; the stride keeps one instanced draw, and the square-root scale keeps the map's coverage where the stride thinned it. [planning]
- Filter in the read over a larger map: fn-14 showed map size is not the lever, and a kernel is a bounded per-pixel cost the record can show, measured. [planning]
- Gate the oak, record the spruce: the spruce's frame is 28.5 ms of GPU with 13.5 ms of shadow, so a stride alone cannot hold 60 fps and no aggregation spec exists; the owner chose to record it and name the follow-on rather than stop this spec on it. [owner]
- The strategy's 2 ms shipped-frame metric is not this spec's bound; 3.8 ms is fn-14's working bound and the number this spec returns under. Reconciling the two is the strategy's, not this spec's. [planning]

## Acceptance Criteria

- **R1:** The sun's depth pass draws a wood caster set culled by branch radius against a row threshold and a foliage caster set at the coarsest level strided by a row value; both are values on the scene row with no species branch. [paraphrase] Errors: a threshold, stride, radius or offset outside its range is rejected naming the field; a degenerate value inside its range empties the set without error, and the default row's set is non-empty on every shipped preset.
- **R2:** The shadow comparison filters over a kernel at leaf scale with a normal-offset bias, both on the row, and the crown's self-shadow in the lit oak hero still reads as dapple rather than per-leaf speckle, judged by the owner beside fn-14's still. [user] Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker.
- **R3:** The oak's native total p50 of vegetation, selection and shadow is at or under 3.8 ms on the RTX 3080 with a valid session at four samples a pixel, seed 7, whole view, hero pose, 1600 by 1000, the default scene row, recorded beside fn-14's 4.949 ms with the per-pass split. [user] Errors: an unavailable, disjoint or contended session does not count and is re-run once; a number over the bound stops the spec with the number rather than another attempt.
- **R4:** The spruce's native timing at the same protocol and its browser orbit are recorded beside fn-14's 30.0 ms wall p50 and 13.51 ms shadow pass, with the caster counts; the oak's browser orbit holds a wall p95 under 16.7 ms with no frame over 33 ms. [owner] Errors: an invalid session is re-run once and recorded as such; the spruce's numbers stop nothing, and a spruce still over 60 fps names needle aggregation as the follow-on in the report.
- **R5:** The ground shadow and the crown's self-shadow show no visible thinning, stepping or shimmer against fn-14's stills and under orbit, judged by the owner. [strategy:Surface and rendering at scale] Errors: a rejecting verdict stops the spec as R2.
- **R6:** fn-14's pins hold at their existing tolerances: the clay still, the leaf and bare views, the lit-sky and leaf-offset pins; the core's identity pins are re-pinned once for the run order with the reason recorded, counts unchanged. [planning] Errors: no error surface beyond the existing pins.
- **R7:** The wood mesh contract carries the run table as an additive field and the timing record carries the caster's triangle and instance counts; the wasm slots and every existing record field are unchanged, and the browser record type declares the new fields with the ones fn-14 left undeclared. [planning] Errors: a run table that does not cover the index buffer exactly is rejected by mesh validation naming the run.

## Early proof point

The wood caster prefix comes first inside the one task: with the prefix alone at the default threshold, the oak's shadow pass p50 falls from 1.813 ms toward the raster floor fn-14 measured, and the ground shadow keeps its silhouette. The prefix alone is a health check, not a gate: it passed on 2026-09-12 (1.813 to 1.063 ms, 8,255,000 to 90,760 caster triangles, silhouette intact) and the residue is the foliage the stride addresses, so the stride and the kernel follow. Only a silhouette that loses limbs at every threshold that moves the number stops the task early; otherwise R3's final oak number is the one stop.

## Approach, in one pass

One task builds the whole spec in the order below; the owner chose one task over three because a strong implementer keeps the design in one head. File and line references are as of the plan on 2026-09-12.

1. **Runs in radius order and the run table.** `surface/paths.rs:3-11` holds `Run{start,end,trunk}`; the run table type (first index, index count, largest radius) belongs there, keeping `surface.rs` (352 lines) under the rule. Sort runs by largest radius descending before the emit loop at `surface.rs:223-290`; the radius is `Sample.r` (`surface.rs:75-80`), largest over the run's samples; record each run's index span as it is emitted. Mesh validation (`mesh.rs:28-43`) rejects a table whose spans do not tile the index buffer exactly, naming the run. The permutation moves `tests/identity.rs` and `tests/surface.rs:70,106` once; re-pin with the reason in the test's doc comment and assert counts unchanged. `tests/submit.rs:107` gains a `fits` entry for the table.
2. **The four row values.** Add `casterTexels` (0..8, default 1), `casterStride` (1..64, default 4), `shadowFilterTexels` (0..3, default 1), `shadowNormalOffset` (0..4, default 1) to the one `fields!` table at `scene/row.rs:17-64`; struct, default, validate, to_json and parse all walk that table. Extend the row tests at `:184-223` with the four names and one out-of-range refusal each. The browser row type is generic (`src/browser/render.ts:84`) and the panel renders the row through `harness/dials.tsx`, so no browser line lands.
3. **The wood caster prefix.** The table stays host-side on the wood subject (`wood.rs:75-111`); the texel world size comes from the fit (`shadow/fit.rs:49-103`, the ortho extent over `RESOLUTION`); threshold is casterTexels times texel size; the prefix is a binary search over the table's radii; `draw_shadow` (`wood.rs:136-144`) draws `0..prefix_index_count`. Expose the caster triangle count on the renderer without entering `FrameStats` (`lib.rs:286-289` keeps the sun's pass out of it). Measure once here: the oak still with the timing record at the default row; record the shadow pass p50 beside 1.813 ms and the caster triangles beside 8,255,000. This is the early proof point.
4. **The foliage stride with a scaled caster quad.** `foliage.rs:256-270` draws `0..select.instances()` at level 0; draw `instances.div_ceil(stride)` and let the vertex stage read `placements[instance * stride]` (`shadow.wgsl:15-23`). The light uniform is a bare `mat4` (`shadow.rs:116-121`, `shadow.wgsl:5`); widen it to the matrix plus stride and quad scale, both entry points and the layout together. Scale the caster quad about its centre by the square root of the stride in the same vertex stage; the peg is not scaled. The stride is fixed by buffer index; the shadow draw already ignores the level lists.
5. **The filtered comparison with a normal offset.** `common.wgsl:57-71` is the one read; take the receiver normal, move the world point along it by shadowNormalOffset times the texel world size, then average a square of hardware comparisons of radius shadowFilterTexels; a tap outside the map counts as lit. `key(n, world)` at `:86-89` already receives the normal, so the three callers (`wood.wgsl:33`, `foliage.wgsl:77`, `scene.wgsl:37`) change only if the signature does. The depth pipeline's constant bias (`pass.rs:215-219`) stays. Two uniform slots join the block at `common.wgsl:9-45`, mirrored at `scene.rs:37-66` and filled at `:255-335`; `scene.rs` is at 390 lines, so move the row-to-uniform fill into `scene/` if it grows past the rule. The clay view never reads the shadow; keep it that way so `tests/look.rs:77` holds without a tolerance change. A device without linear comparison filtering degrades each tap to a point sample; the kernel still averages and the record's adapter line says which.
6. **Tests for the casters and the kernel.** `tests/shadow.rs:18-46` compares coverage shares from a 1,024-square readback; replace the absolute inequalities with claims that survive the casters: the default row's set covers a non-zero share, casterTexels 0 and casterStride 1 each cover at least as much as the default, the bare view's share is below the whole view's, and a filtered read at radius 1 is never darker than the unfiltered read averaged over a 3 by 3 block. `tests/conformance.rs:246-285` adds `shadow.rs`, `wood.rs`, `common.wgsl` and `shadow.wgsl` to the no-anatomy list. `tests/look.rs` pins hold unchanged.
7. **Caster counts in the record.** Follow `with_levels` at `timing/report.rs:132-150` for `with_casters(triangles, instances)`; the counts are present on an invalid record like `multisample` (`:95-102`); `to_json` at `:254-297` adds `caster_triangles` and `caster_instances`. `timing.rs` is at 397 lines; if the counts push it over, move the report assembly out of `collect` (`:340-397`) into `timing/`. `tests/timing.rs:138-179` lists the full record's fields and the total-equals-sum identity; add the two counts, the identity is untouched. `src/browser/render.ts:49-76` declares the record; add the two counts with `shadow_p50_ms`, `shadow_p95_ms` and `multisample`, and add `clay` to the view type at `:78`.
8. **The clock on both trees.** Oak native at the default row (R3) with the per-pass split; spruce native at the same protocol, the first native spruce clock; oak and spruce browser orbits through the rig with `RENDER_EVIDENCE=.flow/evidence/fn27`, on a port other than 5173. One run each; an invalid session is re-run once and recorded as such. Record every number beside fn-14's: 4.949 total, 1.813 shadow, 10.00/10.10/10.20 oak orbit, 30.00/30.20/30.40 spruce orbit, 13.51 spruce shadow. Stills: the lit oak hero and the lit spruce hero at seed 7, the default row, and the clay oak still for the pin; at most four images viewed across the task, one of them a crown crop beside fn-14's hero as the R2 aid.
9. **The report and the docs.** `.flow/evidence/fn27/REPORT.md` in fn-14's shape, with a section that states what each lever bought against fn-14's numbers, the caster counts beside the full counts, and a follow-ups section that names needle aggregation for the spruce if it is still over 60 fps. Rewrite the headers the casters made stale (`shadow.rs:6-9,18-27`, `shadow.wgsl:1-3,12-14`, `common.wgsl:57-60`, `foliage.wgsl:73-75`, `scene.rs:1-5`, `scene/row.rs:1-11`, `timing/report.rs:37-71`, `fit.rs:9-11`), README lines 59, 98 and 130, and fn-14's three stale strings (`examples/headless/walk.rs:14`, `view.rs:1-2`, the browser record type). Evidence reports are dated and not rewritten; fn-14's follow-ups close here and in the README link list.
10. **The verdict slots.** Write the R2 and R5 slots in the Owner verdict section below, leave them for the owner, and end with the aids named. The task ends NEEDS_HUMAN by design; R3's number is the stop if it is over the bound, with the number, not another attempt.

Gates at the end: `cargo test --release --workspace`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `npm run typecheck`. Budget rules from CLAUDE.md bind throughout: one timing run per capture, at most four images viewed, no forest captures, never open receipts or frames.

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
|-----|-------------|---------|-------------------|
| R1 | Coarse wood and foliage casters from row values, no species branch | the one task | — |
| R2 | Filtered comparison with normal offset; dapple judged | the one task | — |
| R3 | Oak native total p50 at or under 3.8 ms, recorded | the one task | — |
| R4 | Spruce recorded natively and in the browser; oak orbit holds | the one task | — |
| R5 | No thinning, stepping or shimmer, judged | the one task | — |
| R6 | fn-14's pins hold; identity pins re-pinned once | the one task | — |
| R7 | Run table and caster counts additive; browser type declares them | the one task | — |

## Owner verdict

R2 and R5 are the owner's judgment of the lit oak hero still and the orbit beside fn-14's, recorded here in the owner's own words. The spec closes only on two accepting verdicts and R3's number at or under the bound.

### The crown's self-shadow, R2

> _verdict (owner, ):_

### Thinning, stepping and shimmer, R5

> _verdict (owner, ):_

## References

- fn-14 spec and `.flow/evidence/fn14/REPORT.md`: the numbers this spec records beside, the report shape it follows, and the follow-ups it closes.
- fn-24 evidence: the orbit protocol and the clay hero the pins are judged against.
- fn-26 spec: the follow-on whose transmission reads through this spec's shadow function.
- fn-4, fn-20, fn-21, fn-15: the open specs whose changes re-tune the threshold, the stride or the caster motion, named in Edge Cases.
