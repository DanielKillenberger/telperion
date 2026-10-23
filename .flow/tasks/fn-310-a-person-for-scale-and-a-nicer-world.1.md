---
satisfies: [R3, R4]
---
# fn-310-a-person-for-scale-and-a-nicer-world.1 Variant C as the default scene: sky glow, ground noise and haze as row parameters

## Description
The environment half of the spec (D1, D4): new scene-row fields, their uniforms, and the sky and floor shaders, with variant C's values as the row's default. Comes first because the person's still is judged under it.

**Size:** M
**Files:** `crates/telperion-render/src/scene/row.rs`, `crates/telperion-render/src/scene.rs`, `crates/telperion-render/src/scene/frame.rs`, `crates/telperion-render/src/shaders/common.wgsl`, `crates/telperion-render/src/shaders/sky.wgsl`, `crates/telperion-render/src/shaders/scene.wgsl`, tests under `crates/telperion-render/tests/`
**Touches:** [crates/telperion-render/src/scene/row.rs, crates/telperion-render/src/scene.rs, crates/telperion-render/src/scene/frame.rs, crates/telperion-render/src/shaders/common.wgsl, crates/telperion-render/src/shaders/sky.wgsl, crates/telperion-render/src/shaders/scene.wgsl, crates/telperion-render/tests/**]

### Approach
- Add rows to the `fields!` table (`row.rs:19-41`): dry ground colour (3), ground variation, sky glow, haze distance. Each gets a range and a refusal name. Defaults are variant C's values (`scene-c.json` on branch `scene-concept`, quoted in the spec's D1), and C's sky and ground colours replace the old defaults. Sun position unchanged; sun colour retinted per D1 so `the_default_daylight_is_white_at_the_luminance_it_had` holds as written.
- Append whole `vec4` slots to `Uniforms` (`scene.rs:41-117`) and mirror them in `common.wgsl`'s struct; fill them in `frame.rs` beside the sky and ground colours (`frame.rs:83-98`).
- Rewrite, not copy, the concept's `sky_radiance`, value noise and `ground_field` (common.wgsl), `ground_albedo` and `ground_haze` (scene.wgsl), and the sky's dither (sky.wgsl). Reuse `vary` (common.wgsl:225) for the hash. Haze is weighted by the floor channel (`in.colour.a`) so only the floor fades. Every strength at zero is an exact no-op, without a branch that switches ways of drawing where a factor of zero does the same.
- Keep `row.rs` under about 400 lines; move the tests to a sibling file if needed.

### Investigation targets
**Required:**
- `crates/telperion-render/src/scene/row.rs` - the field table and its tests
- `crates/telperion-render/src/shaders/sky.wgsl`, `scene.wgsl`, `common.wgsl:9-110,162-230` - the uniforms, `ambient`, `key`, `tone`, `vary`
- `crates/telperion-render/src/scene/frame.rs:60-120` - uniform assembly
- scene-concept: `git -C /home/daniel/Projects/telperion/.worktrees/scene-concept diff 0af2cbc17 HEAD -- crates/telperion-render/src/shaders` - the design to rewrite
**Optional:**
- `crates/telperion-render/tests/look.rs:267`, `tests/neutral_light.rs`, `tests/shadow.rs` - default-row-dependent tests
- `README.md:131` - the `--scene` field list

### Key context
- WGSL uniform layout must match the Rust struct byte for byte; append `vec4`s only.
- Tests that read `SceneRow::default()` may move by design (R5): update only where the sky or ground changed, and say which in the task summary.
- The harness panel builds controls from the default's keys (`harness/GrowerDev.tsx:206-214,362`): check the new fields show with sensible ranges; `npm test` covers the package's `SceneRow` type.

## Acceptance
- [ ] `SceneRow::default()` carries variant C's sky, ground, dry ground, variation, glow and haze values; the sun position is unchanged and the white-daylight test passes unmodified
- [ ] Each new field is refused by name outside its range; a JSON omitting it takes the default; round-trip JSON test covers the new names
- [ ] At zero, glow, variation and haze each draw the same frame bytes as the base shader would for that row (GPU test, skipped without a GPU as the suite does)
- [ ] Haze changes no subject pixel: a GPU test shows a frame with the default haze and one with haze 0 differ only on floor and sky pixels
- [ ] The same frame renders the same bytes twice on one device (GPU test)
- [ ] README's `--scene` field list names the new fields and defaults
- [ ] Focused render tests green through `telperion-heavy 5 --gpu 3 --threads 2`

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
