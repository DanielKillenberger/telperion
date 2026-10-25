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
The person now casts a shadow on the ground. Its indices are drawn depth-only into the sun's map in the existing shadow pass, using the wood's entry point, and no pass was added. The fit is now `light(row, &[Bounds])`, which holds every caster and the shadow it throws. A caster inside the tree's volume leaves the light byte-equal, and a test shows it. The sun-map key carries whether the person casts. A new tree is the only thing that moves the person, and it already forgets the map. `Renderer::figure_casters` reads the person's triangles in the map as last drawn.

First, as its own commit (fc36bb605), the two failures from the first gate were fixed:
- `lattice_value` in common.wgsl now writes `(a * b) ^ (c * d)`, the precedence naga already took, so drawing is unchanged (inferred from the parse order, not measured).
- The every-preset bounds table, the in-frame test and the ignored canopy test moved from camera.rs to `tests/person_frame.rs`. The library now names no family. They read the person through a new `figure_points` export, and the conformance test is untouched.

Tests:
- `shadow::the_person_is_a_caster_only_while_the_room_draws_it`: whole and bare views with the figure on, and not with the figure off or in the leaf view.
- `shadow::the_persons_shadow_darkens_the_floor_and_leaves_the_trees_alone`: under a sun behind the person, a floor point in its shadow is below 0.85 of its figure-off value. Every map texel is equal or nearer, and fewer than 1% change, so the tree's map is unchanged.
- `sun_cache`: a figure toggle redraws the map.
- `fit::a_second_caster_widens_the_map_only_where_it_stands_outside`.
- `sun`: the cache key with the figure flag.
- The existing empty-map assertion in `shadow.rs` now turns the figure off, because by design the person now casts.

Stills: `person-shadow.jpg` (committed copy, plus `raw/stills/person-shadow.jpg`) holds the oak at 40 and Telperion at 80, seed 7, under the film's sun, with crops 3x and 6x. I viewed it. The shadow is short and visible beside the feet; at Telperion's framing it is a few pixels and shows only in the crop.

Baseline: none run pre-edit. The conductor's gate log at a1b170043 showed the two static failures, which are now green.
Focused runs, all green (`telperion-heavy 5 --gpu 3 --threads 2`, `--profile ci`):
- `--lib`: tint_rule_tests, compile_size, camera, scene, sun, shadow::fit
- `--test`: shadow, sun_cache, shot, look, environment, neutral_light, conformance, person_frame

Feature map: no route changed.
Follow-ups (not this task's):
- scene.rs (550 lines) and lib.rs (677) were already over the 400-line rule and grew slightly.
- The harness and package use Wasm, which was not built here; the conductor's gate covers it.

Tier: session at default effort (model: claude-opus-5-5)

stage: impl-review - ran (codex, three reviewers, SHIP, 0 findings)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: fc36bb605892cc44944cdf469aee7a99ce6ff5ea, a192d41bdc03a5c9f486317f06dea97901e8a61b, 5247e51e0b13ce30d197c80ed19032d8e272eaaf
- Tests: cargo test --profile ci -p telperion-render --lib -- tint_rule_tests compile_size camera scene sun shadow::fit, cargo test --profile ci -p telperion-render --test shadow --test sun_cache --test shot --test look --test environment --test neutral_light --test conformance --test person_frame
- PRs: