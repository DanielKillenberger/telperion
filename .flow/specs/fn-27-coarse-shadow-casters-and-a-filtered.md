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

Task fn-27-coarse-shadow-casters-and-a-filtered.1 validates the core approach: with the wood caster prefix alone at the default threshold, the oak's shadow pass p50 falls from 1.813 ms toward the raster floor fn-14 measured, and the ground shadow keeps its silhouette. If the pass does not fall below about 0.9 ms, or the silhouette loses limbs at any threshold that does, re-evaluate the prefix against a coarse wood proxy before continuing with fn-27.2.

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
|-----|-------------|---------|-------------------|
| R1 | Coarse wood and foliage casters from row values, no species branch | fn-27-coarse-shadow-casters-and-a-filtered.1, fn-27-coarse-shadow-casters-and-a-filtered.2 | — |
| R2 | Filtered comparison with normal offset; dapple judged | fn-27-coarse-shadow-casters-and-a-filtered.2, fn-27-coarse-shadow-casters-and-a-filtered.3 | — |
| R3 | Oak native total p50 at or under 3.8 ms, recorded | fn-27-coarse-shadow-casters-and-a-filtered.3 | — |
| R4 | Spruce recorded natively and in the browser; oak orbit holds | fn-27-coarse-shadow-casters-and-a-filtered.3 | — |
| R5 | No thinning, stepping or shimmer, judged | fn-27-coarse-shadow-casters-and-a-filtered.3 | — |
| R6 | fn-14's pins hold; identity pins re-pinned once | fn-27-coarse-shadow-casters-and-a-filtered.1, fn-27-coarse-shadow-casters-and-a-filtered.2 | — |
| R7 | Run table and caster counts additive; browser type declares them | fn-27-coarse-shadow-casters-and-a-filtered.1, fn-27-coarse-shadow-casters-and-a-filtered.3 | — |

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
