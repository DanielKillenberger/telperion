# Coarse shadow casters and a filtered shadow

## Goal

Captured 2026-09-11 at the end of fn-14 and planned 2026-09-12. fn-14 left the oak's native total p50 at 4.949 ms against its 3.8 ms working bound, 1.813 ms of it the sun's depth pass, because the pass submitted the full-resolution wood and every leaf (geometry, not raster). The crown's self-shadow was a near-binary four-tap verdict per leaf and read as salt. This spec recovered the oak's budget before fn-26 by drawing less into the depth pass and filtering what the crown reads back, so the self-shadow reads as dapple; the spruce is recorded, not gated.

## Decisions

- > user (2026-09-11): "should we optimize performance after fn-14, before 26? Or do we have budget to render fn-26 at 60fps for one hero tree"
- > user (2026-09-11): "ok let's capture that shadowcaster optimization spec then. Would it be fitting to also fix the noise that's in the canopy atm? look at the crown it looks like dithering/aliasing still."
- > user (2026-09-11): "omg this is so much better" (on the page rebuilt at four samples a pixel)
- > owner (2026-09-12, planning): gate the oak, record the spruce; needle aggregation becomes a named follow-on.
- Wood casters: surface runs emitted in descending largest radius with a run table; the depth pass draws one prefix of the index buffer (threshold = `casterTexels` x map-texel size). Rejected: a draw per run (thousands of calls) and a second caster index buffer (doubles 99 MB of oak indices).
- Foliage casters: every `casterStride`-th placement at the coarsest level, quad scaled by sqrt(stride), fixed by buffer index so orbiting cannot change which leaves cast. Rejected: a hashed subset (needs a second selection dispatch).
- Filter in the read (`shadowFilterTexels` kernel plus `shadowNormalOffset`), not a larger map; fn-14 showed map size is not the lever. Four row values, no species path.
- Sub-pixel leaf coverage noise is a coverage problem for its own spec; one task over three (owner).

## Acceptance Criteria

- **R1:** Wood casters culled by radius against a row threshold and foliage casters strided by a row value, no species branch; out-of-range values refused naming the field — met.
- **R2:** Filtered comparison with normal offset; crown self-shadow reads as dapple beside fn-14's, judged by the owner — met (accepted 2026-09-12).
- **R3:** Oak native total p50 at or under 3.8 ms (RTX 3080, valid session, 4x MSAA, seed 7, 1600x1000) — met, 3.4243 ms.
- **R4:** Spruce native and browser recorded beside fn-14; oak orbit wall p95 under 16.7 ms, no frame over 33 ms — met (spruce over 60 fps names needle aggregation as follow-on).
- **R5:** No visible thinning, stepping or shimmer against fn-14 and under orbit, judged by the owner — met (accepted 2026-09-12).
- **R6:** fn-14's pins hold; core identity pins re-pinned once for run order, counts unchanged — met.
- **R7:** Run table additive on the wood mesh contract; timing record gains `caster_triangles`/`caster_instances`; wasm slots unchanged — met.

## Result

- Oak, default row: native total p50 4.949 -> 3.4243 ms (p95 3.7484); sun pass 1.813 -> 0.2724 ms on 90,760 of 8,255,000 wood triangles and 217,328 of 869,310 placements. Early proof (prefix alone, 2026-09-12): pass 1.813 -> 1.063 ms, silhouette intact. Oak browser orbit 10.00 / 10.10 / 10.20 ms over 999 frames.
- Spruce: first native clock 23.4161 ms total, 4.2691 ms shadow; browser orbit wall p50 30.00 -> 20.00 ms, pass 13.51 -> 3.1918 ms; still short of 60 fps.
- Owner verdicts:
  - R2 > _verdict (owner, 2026-09-12):_ accept. "looking good", on the live page at the default row beside fn-14's crown aid.
  - R5 > _verdict (owner, 2026-09-12):_ accept. "looking good", on the live page under orbit beside fn-14's ground aid.

## Open

Needle aggregation for the spruce (still over 60 fps). The strategy's 2 ms shipped-frame metric versus the 3.8 ms working bound is the strategy's to reconcile.

## Evidence

- `.flow/evidence/fn27/REPORT.md` (linked from README; protocol, per-pass split, levers)
- `.flow/evidence/fn27/oak-crown-comparison.jpg` (R2 aid), `oak-ground-comparison.jpg` (R5 aid), `oak-final.jpg`, `spruce-final.jpg`
- `.flow/evidence/fn27/*-timing.json`, `*-browser-orbit.json` (final records)
