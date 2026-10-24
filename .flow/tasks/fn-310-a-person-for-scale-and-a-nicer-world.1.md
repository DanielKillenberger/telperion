---
satisfies: [R3, R4]
---
# fn-310-a-person-for-scale-and-a-nicer-world.1 Variant C as the default scene: sky glow, ground noise and haze as row parameters

## Description
The environment half of the spec (D1, D4): new scene-row fields, their uniforms, and the sky and floor shaders, with variant C's values as the row's default. Comes first because the person's still is judged under it.

**Size:** M
**Files:** `crates/telperion-render/src/scene/row.rs`, `crates/telperion-render/src/scene.rs`, `crates/telperion-render/src/scene/frame.rs`, `crates/telperion-render/src/shaders/common.wgsl`, `crates/telperion-render/src/shaders/sky.wgsl`, `crates/telperion-render/src/shaders/scene.wgsl`, tests under `crates/telperion-render/tests/`, `README.md`
**Touches:** [crates/telperion-render/src/scene/row.rs, crates/telperion-render/src/scene.rs, crates/telperion-render/src/scene/frame.rs, crates/telperion-render/src/shaders/common.wgsl, crates/telperion-render/src/shaders/sky.wgsl, crates/telperion-render/src/shaders/scene.wgsl, crates/telperion-render/tests/**, README.md]

### Approach
- Add rows to the `fields!` table (`row.rs:19-41`): dry ground colour (3), ground variation, sky glow, haze distance. Each gets a range and a refusal name. Defaults are variant C's values (`scene-c.json` on branch `scene-concept`, quoted in the spec's D1), and C's sky and ground colours replace the old defaults. Sun position unchanged; sun colour retinted per D1 so `the_default_daylight_is_white_at_the_luminance_it_had` holds as written.
- Append whole `vec4` slots to `Uniforms` (`scene.rs:41-117`) and mirror them in `common.wgsl`'s struct; fill them in `frame.rs` beside the sky and ground colours (`frame.rs:83-98`).
- Rewrite, not copy, the concept's `sky_radiance`, value noise and `ground_field` (common.wgsl), `ground_albedo` and `ground_haze` (scene.wgsl), and the sky's dither (sky.wgsl). Reuse `vary` (common.wgsl:225) for the hash. Haze is weighted by the floor channel (`in.colour.a`) so only the floor fades. An explicit zero turns its effect off (spec, R3 errors), without a branch that switches ways of drawing where a factor of zero does the same.
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
- [ ] With glow, variation and haze explicitly zero, a GPU test shows each effect off: the sky matches the plain zenith-to-horizon gradient within the dither's one 8-bit step, the floor's albedo does not vary with place, no floor pixel is hazed, and no pixel is NaN (skipped without a GPU, as the suite does)
- [ ] Haze changes no subject pixel: a GPU test shows a frame with the default haze and one with haze 0 differ only on floor and sky pixels
- [ ] The same frame renders the same bytes twice on one device (GPU test)
- [ ] README's `--scene` field list names the new fields and defaults
- [ ] Focused render tests green through `telperion-heavy 5 --gpu 3 --threads 2`
## Done summary
Variant C is now `SceneRow::default()`. Six new row fields carry it: `skyGlow`, `groundDryRed/Green/Blue`, `groundVariation` and `hazeDistance`. Two appended vec4 uniforms (`atmosphere`, `ground_dry`) take them to the sky and floor shaders, which add the sun glow, value-noise ground patches on the `vary` hash, floor-only haze and a one-step sky dither. Each effect is off at an explicit zero.

- Defaults: C's sky, ground and dry ground; variation 0.5, glow 0.6, haze 500 m. The sun keeps 135/30 and is retinted to (3.064, 2.854, 2.374) per D1. `the_default_daylight_is_white_at_the_luminance_it_had` passes unmodified.
- Tests moved by design: `tests/neutral_light.rs` `ground()` now sets variation and haze to 0, because it measures the light and needs a flat, unhazed floor. No other existing test changed.
- New tests: `scene::row::tests` (refusal per new field, round trip, omitted field takes the default, explicit zero, the C default pinned). `tests/environment.rs` (GPU): zero glow is the CPU gradient within 1 step; with zero variation and haze the floor is one colour, and each default effect visibly bites; the floor under the eye is not black (no NaN); haze moves no subject pixel (tree and figure mask from a with-tree versus bare-room frame, sun off, one sample); the same frame is the same bytes.
- Decision: ground variation mixes the ground toward the dry colour by `variation * patch` (D4 as written). The concept also darkened lush patches by a literal 0.82; that is dropped under R3's no-literals rule, so dark-side contrast is slightly lower than the concept's sheet.
- Measured: the oak still at seed 7 (960x540, RTX 3080) shows the sky gradient, glow, ground patches and horizon haze. Harness panel: the new keys show automatically as number boxes (`Traits` over the default's keys); no range sliders exist for scene fields.
- Baseline: green (scene lib tests and shot/look/neutral_light/shadow).
- Follow-up, not in scope: the 4 m noise octave may shimmer at grazing distance in motion; judge on the film stills.

Tier: session (jev long_running 0.21) at default effort (model: claude-opus-5-5)

stage: impl-review - ran (codex, 3 draws, SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 91eddc36bfe59b45b9db970cb367aefffe5592bf
- Tests: telperion-heavy 5 --gpu 3 --threads 2 -- cargo test --profile ci -p telperion-render --lib scene, telperion-heavy 5 --gpu 3 --threads 2 -- cargo test --profile ci -p telperion-render --test environment --test neutral_light --test look --test shot --test shadow --test conformance --test bark_distance --test bark_occlusion --test colour_cavity --test canopy_light --test bark_resolution --test curve --test lobe_shade --test socket_cavity --test sun_cache
- PRs: