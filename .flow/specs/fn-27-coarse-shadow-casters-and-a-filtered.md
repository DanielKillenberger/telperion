# Coarse shadow casters and a filtered shadow

## Conversation Evidence

> user (2026-09-11): "should we optimize performance after fn-14, before 26? Or do we have budget to render fn-26 at 60fps for one hero tree"
> user (2026-09-11): "ok let's capture that shadowcaster optimization spec then. Would it be fitting to also fix the noise that's in the canopy atm? look at the crown it looks like dithering/aliasing still."
> user (2026-09-11): "omg this is so much better" (on the page rebuilt at four samples a pixel)
> user (2026-09-11): "translucency is in the next spec yea?"

Captured 2026-09-11 at the end of fn-14's run, with its evidence on the branch. fn-14's report records the oak's native total p50 at 4.949 ms against the 3.8 ms bound, of which the sun's depth pass is 1.813 ms, and the spruce's browser orbit at a wall p50 of 30.0 ms, of which the same pass is 13.51 ms; fn-24 had the spruce at 10.1 ms. Halving the map in fn-14 bought half a millisecond because the pass submits the full-resolution wood and every needle: the cost is geometry, not raster. fn-26's budget rule inherits fn-14's bound, so it would stop on its first timing run. The crown's self-shadow is one hardware comparison, four taps at a 1,024-texel map over the whole crown, so each leaf gets a near-binary verdict at leaf scale.

## Goal & Context

Recover fn-14's frame budget before fn-26 by drawing less into the sun's depth pass and filtering what the crown reads back from it, so the oak's native frame returns under its bound, the spruce's browser orbit returns to 60 fps, and the crown's self-shadow reads as dapple rather than salt. fn-26 depends on this spec. [user]

## Architecture & Data Models

- **A coarse wood caster.** The depth pass draws only branches whose radius exceeds a threshold on the row, a value derived from what a texel of the map can resolve at the crown; a branch thinner than that casts nothing the filter could keep. One predicate over a value the generator already carries per branch, no new mesh and no wood level ladder. [paraphrase]
- **A coarse foliage caster.** The depth pass draws a strided subset of the placement buffer at the element's coarsest level, the stride on the row; the blade and the needle go through the same rule and differ only by their rows. [paraphrase]
- **A filtered comparison.** The shadow read widens from the hardware four taps to a kernel at leaf scale, its radius in texels on the row, with a normal-offset bias so a leaf does not shadow itself; both shaders and the ground disc read through the same function. [paraphrase]
- **Timing unchanged.** The third timestamp pair still times the pass; the record gains the caster's triangle and instance counts beside the full counts. [inferred]

## API Contracts

- **Shadow row values** join the scene row: caster radius threshold, caster placement stride, filter radius in texels, normal offset. They travel through the headless scene flag and the page's scene setter like every other scene value, with an outdoor default that meets the bounds; validation rejects a value outside its range naming the field. [inferred]
- **Timing record** keeps its fields and adds the caster counts; the report shape is fn-14's. [inferred]

## Edge Cases & Constraints

- **No species path.** Threshold, stride, kernel and offset are numbers on a row; nothing in the pass knows a needle from a blade. [strategy:Surface and rendering at scale]
- **No shimmer.** The caster subset is fixed from the placement buffer, never from the per-frame selection, so orbiting does not change which leaves cast and the ground shadow does not crawl. [strategy:Surface and rendering at scale]
- **No thinning.** A stride that opens holes in the ground shadow, or a threshold that drops a limb's shadow, is too coarse; the hero still and the orbit judge it, beside fn-14's stills. [inferred]
- **Filter cost is per pixel at four samples.** The kernel's cost lands in the vegetation pass and counts against the same bound; it is measured, not assumed. [paraphrase]
- **Budget discipline.** Small before large, one timing run per change, at most four images viewed, per the project's evidence rules. [paraphrase]

## Acceptance Criteria

- **R1:** The sun's depth pass draws a wood caster set culled by branch radius against a row threshold and a foliage caster set at the coarsest level strided by a row value; both are values on the scene row with no species branch. [paraphrase] Errors: a threshold, stride, radius or offset outside its range is rejected naming the field.
- **R2:** The shadow comparison filters over a kernel at leaf scale with a normal-offset bias, both on the row, and the crown's self-shadow in the lit oak hero still reads as dapple rather than per-leaf speckle, judged by the owner beside fn-14's still. [user] Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker.
- **R3:** The oak's native total p50 of vegetation, selection and shadow is at or under 3.8 ms on the RTX 3080 with a valid session at four samples a pixel, recorded beside fn-14's 4.949 ms. [user] Errors: an unavailable, disjoint or contended session does not count; a number over the bound stops the spec with the number rather than another attempt.
- **R4:** The spruce's browser orbit holds a wall p95 under 16.7 ms with no frame over 33 ms, recorded beside fn-14's 30.0 ms, and the oak's orbit stays inside the same bounds. [user] Errors: as R3.
- **R5:** The ground shadow and the crown's self-shadow show no visible thinning, stepping or shimmer against fn-14's stills and under orbit, judged by the owner. [strategy:Surface and rendering at scale] Errors: a rejecting verdict stops the spec as R2.
- **R6:** fn-14's pins hold: the clay still, the identity pins, the leaf and bare views. [inferred] Errors: no error surface beyond the existing pins.

## Boundaries

- Sub-pixel leaf coverage noise, many leaves under a pixel at hero distance, is out of scope; it is a coverage problem, not a shadow problem, and gets its own spec if it remains at four samples. [user]
- No second light, no cascade, no second shadow map. [inferred]
- No wood level ladder; culling by radius is the caster, not a simplification of the drawn wood. [inferred]
- Bark relief, veins and transmission are fn-26's. [user]

## Decision Context

- Fix before fn-26 rather than raise the bounds: fn-26's budget rule inherits fn-14's 3.8 ms and would stop on its first timing run, and the spruce already lost 60 fps in fn-14 while fn-26 judges it too. [paraphrase]
- Cull by radius over a wood level ladder: the cost is geometry submission, 8.25 million triangles for 1.8 ms on the oak and 13.5 ms on the spruce, and a threshold on a value every branch already carries is one predicate with no new mesh. [paraphrase]
- Filter in the read over a larger map: fn-14 showed map size is not the lever, and a wider kernel is a bounded per-pixel cost the budget can absorb, measured. [paraphrase]
- fn-14's R12 slot: the owner accepts fn-14's number there and points at this spec as the fix. [inferred]

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R6 | TBD during planning |
