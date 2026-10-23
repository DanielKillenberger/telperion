---
satisfies: [R2, R5]
---
# fn-310-a-person-for-scale-and-a-nicer-world.5 The person casts a shadow through the existing shadow pass

## Description
Owner verdict on person-v2 (2026-10-23): "ok good. it should also cast a shadow then i think it's fine." The person (task .2) is drawn into the sun's map in the existing shadow pass, the caster path the tree uses, so its shadow falls on the ground. No new pass; the person's 627 vertices are a negligible addition to the casters (host, 2026-10-23).

**Size:** M
**Files:** `crates/telperion-render/src/scene.rs`, `crates/telperion-render/src/shadow/fit.rs`, `crates/telperion-render/src/sun.rs`, `crates/telperion-render/src/lib.rs`, the shadow pass's draw site and shader, tests under `crates/telperion-render/`, `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/`
**Touches:** [crates/telperion-render/src/**, crates/telperion-render/tests/**, .flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/**]

### Approach
- Find where the sun's map is drawn (the curve's sun view and the foliage's caster instances; `lib.rs` `caster_triangles`, `caster_instances`, `shadow_cached`; `sun.rs`) and draw the room's person indices (the range after `floor_index_count`, `scene.rs`) into the same pass, depth only. The floor itself is not a caster.
- The figure flag holds: with the figure off (`set_figure(false)`, `--no-figure`, photo frames) the person casts nothing. Clay draws no shadow today; keep it so.
- The sun's volume (`shadow/fit.rs` `light(row, bounds)`) must hold the person and the shadow it throws, since the person stands outside the crown (spec D2): widen the fitted bounds by the person's placed extent, without a change to the tree's own fit when the person is inside them.
- The sun's map cache (`sun.rs` plan/drew/forget) must redraw when the person moves (it moves on submit, which already forgets) and when the figure flag changes.
- Expose the person as a caster where a test can read it (for example the caster triangle or instance count, or a flag on the sun's plan), so the test reads it the way `caster_triangles` is read.
- Then re-render the final stills: oak at 40 and Telperion at 80, seed 7, 1280x720, under the film's sun (`--scene '{"sunAzimuth":115,"sunElevation":60}'`), with the person's shadow visible on the grass. Compose one sheet at `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/raw/stills/person-shadow.jpg` (write atomically: temp name, then `mv`) and a compressed committed copy at `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/person-shadow.jpg`. View them.

### Investigation targets
**Required:**
- `crates/telperion-render/src/sun.rs` - the sun's map plan and cache key
- `crates/telperion-render/src/shadow/fit.rs:36-110` - the sun's volume
- `crates/telperion-render/src/scene.rs` - room buffer layout, `draw`, the person's indices
- `crates/telperion-render/src/lib.rs:370-440` - the caster readbacks
**Optional:**
- `crates/telperion-render/tests/shadow.rs`, `tests/sun_cache.rs` - shadow and cache tests to extend

## Acceptance
- [ ] The person is drawn into the sun's map in the existing shadow pass; no new render pass is added
- [ ] A test shows the person is among the shadow casters with the figure on, and not with the figure off
- [ ] A GPU test (skipped without a GPU as the suite does) shows a floor point in the person's shadow is darker than with the figure off, and the tree's shadow is unchanged
- [ ] The sun's volume holds the person and its shadow; the tree's own fit is unchanged when the person is inside it
- [ ] The sun-map cache redraws when the person moves or the figure flag changes
- [ ] `person-shadow.jpg` (oak 40, Telperion 80, film's sun) exists at the raw path, with a committed compressed copy, the person's shadow visible on the grass
- [ ] Focused render tests (shadow, sun_cache, shot, look, environment, scene and camera libs) green through `telperion-heavy 5 --gpu 3 --threads 2`

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
