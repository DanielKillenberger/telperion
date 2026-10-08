# The supernatural group on the engine path: the design (fn-217 R1)

A design proposal for the host to decide, then for Astra to review, before anything is built (R1). No product code is changed. It covers each trait the brief names (`.flow/evidence/fn-198-tree-space-f-the-engine-in-production/VISION-TWO-TREES.md`) and the two blossom follow-ups fn-216 handed on. Every code claim names its file and line on this branch (`fn-217-tree-space-the-supernatural-group-on`, at `54e84fcd`).

Each claim carries one marker:

- **read**: checked by reading the code, not run;
- **inferred**: follows from code that was read;
- **estimate**: a cost from the pass structure, with nothing built or measured;
- **unknown**: not checked.

Frame costs are estimates. The only measured frame times used are fn-208's 80-year engine trees at 960 × 720 with 4 samples (`.flow/evidence/fn-208-tree-space-f3-wood-surfaced-on-the-gpu/STEP5.md:40-55`). The beech hero is 2.7 ms vegetation, 3.3 ms surfacing and 6.7 ms frame. The oak hero is 17.0, 2.6 and 20.2 ms.

## 0. The recommendations at a glance

| Trait | Recommendation | Neutral value | Frame cost (estimate) |
|---|---|---|---|
| Scale | Scale the converted tree's positions and radii in both executors, after growth | `scale = 1` | none per frame; leaves and florets per tree change with it (§3.1) |
| Leaf, silver beneath | A back-face reflectance and tint (a metallic highlight on the underside) | back reflectance 0 | under 2 % of the leaf fragment |
| Leaf, gold edge | A reflectance on the margin band the shader already draws | margin reflectance 0 | under 2 % of the leaf fragment |
| Emitted light, itself | An emission colour in the organ block, added to the floret's radiance | emission 0 | about 0 |
| Emitted light, surroundings | An analytic emitter per tree (the bloom's ellipsoid) in the shared ambient term | emitter reach 0 | +5 to 10 % of lit fragment time |
| Glow in the air | An analytic in-scattering term along each pixel's ray, no post-process pass | haze 0 | +0.05 to 0.2 ms at 960 × 720 |
| Cycle | A frame clock on the renderer; per tree a period, phase, depth and shape | depth 0 (steady light) | about 0 |
| Two trees in counterphase | Two renderers drawn into one target, each lit by the other's emitter | one tree | a second tree's frame |
| Falling dew | Stateless drops, a pure function of (floret, drop, time), drawn from the organ buffer | drops 0 | under 0.2 ms at 50k drops |
| Five-petal floret | A base tint (eye) and stamens in the organ element; organ lit as a surface, not a mass | eye share 0, stamens 0 | proportional to the organ's added triangles |
| Blossom at distance | Measure first; then keep a sub-pixel floret's coverage by alpha-to-coverage | none (a draw rule for the organ only) | +0.2 to 1 ms at millions of florets |
| fn-10's writhe and spiral | Leave them on the scaffold, and do not port them | as today | none |

## 1. What exists

### The group is read only by the scaffold

- **The rows.** fn-10's group is `SupernaturalParams` at `/skeleton/bias/supernatural`. Its rows are `enabled`, `writheAmplitude`, `writheWavelength`, `spiralRate` and `maxWritheMagnitude` (`crates/telperion-core/src/pipeline/bias.rs:12-94`). It has no `scale` row (read). fn-10 is still `open`, and its R6 scale row was never built.
- **What off means.** A disabled group reads as `NONE` (`bias.rs:100-108`, `:196-200`). The blend walks what each side applies, so a walk ramps the terms up from nothing (`crates/telperion-core/src/blend.rs:41-45`, `:126-143`). Every new row below follows this pattern.
- **Who reads it.** Only `GrowthBias::apply` reads the group (`bias.rs:194-249`), and only the scaffold grower calls it (`pipeline/branching/scaffold.rs:236`, `pipeline/branching/specimen.rs:68`).
  - The production skeleton stage is `space::grow` alone (`crates/telperion-core/src/pipeline.rs:174-180`).
  - The scaffold is `#[cfg(test)]` and "no production path reaches" it (`pipeline.rs:182-190`).
  - *Read, consequence:* on the engine path the group parses, validates and blends, and changes no tree.
- **The bias field is not on the engine path.** No file in `crates/telperion-space/src` names the bias field, writhe or supernatural (grep, 2026-10-08). The conversion takes the species, the age, the seed and the site's light, and nothing else (`crates/telperion-core/src/pipeline/space.rs:99-116`).
- **The Two Trees are not engine trees.** `telperion.values` and `laurelin.values` set a 148 m and a 132 m envelope and the writhe rows. Neither has a `/space` row (`crates/telperion-core/presets/telperion.values:66`, `:73-76`; `laurelin.values:15`, `:22-25`). A family with no `/space` row has no species (`presets/space.rs:57-60`, `:114-116`), and the pipeline refuses it by name (`space.rs:98-100`). Both ids are still in the catalogue (`presets.rs:37-38`). *Inferred:* building either by name fails today with "no engine species". Their values belong to fn-218 and fn-219.

### The renderer: one tree, one sun, no clock

- **One tree per renderer.** "A device with the room built on it, holding at most one tree" (`crates/telperion-render/src/lib.rs:101-129`). There is one wood, one leaf foliage and one organ foliage.
- **The frame's passes,** in order (`lib.rs:484-554`):
  1. wood surfacing (compute);
  2. leaf selection (compute) and organ selection (compute);
  3. the sun's shadow pass, wood and leaves only, with no organ (`:503-516`);
  4. the room: sky, ground and figure (`:517-536`);
  5. vegetation: wood, leaves, then the organ under its own block (`:537-551`).
- **One light model, in the prelude.** Every lit shader shares `ambient()` (a sky and ground hemisphere), `occluded_ambient()` and `key()` (the sun through one shadow map) (`crates/telperion-render/src/shaders/common.wgsl:147-175`). There are four lighting sites:
  - ground and figure: `scene.wgsl:37`;
  - wood: `wood.wgsl:231`;
  - leaves and florets: `foliage.wgsl:189-236`;
  - sky: `sky.wgsl:29`.
- **No high-dynamic-range target.** Every shader tone-maps its own fragment with an ACES fit and writes straight to the sRGB target (`common.wgsl:194-206`). No pass follows the vegetation pass, which resolves the samples (`lib.rs:537-541`, `pass.rs:142-145`). Multisampling is 4 (`pass.rs:17`), with alpha-to-coverage off and no blending (`pass.rs:251`, `:294`, `:311`).
- **No time.** No uniform holds a time (`common.wgsl:9-99`), and the frame block is filled from the camera, the light, the scene row and the material (`scene/frame.rs:8-215`). The harness already draws on every animation frame (`harness/rust-stage.ts:95-104`). *Inferred:* a frame clock costs no extra redraw there.
- **One build-time clock.** The pipeline's `Request.season` is the time of year flowers are drawn at (`crates/telperion-core/src/pipeline.rs:52-54`). It scales florets in and out at build (`pipeline/foliage/bloom.rs:65-84`). A change of season is a rebuild.
- **The organ's colour block.** The florets draw through `foliage.wgsl` under a copy of the frame block (`scene/frame.rs:218-245`):
  - with `bloomFront*` and `bloomBack*` as the two faces (`material/params.rs:208-257`);
  - with veins, mottle and margin set to zero (`frame.rs:234-236`);
  - with every other term inherited from the leaves, the canopy normal's bend toward the crown's outward direction included (`foliage.wgsl:174-178`).
- **Free lanes.** The frame block has several reserved lanes: `leaf_colour_detail.w`, `crown_shade.zw`, `lichen_detail.zw`, `reflectance.zw`, `plate_profile.yzw`, `leaf_dead.w` and `mature_radius.zw` (`frame.rs:138-143`, `:176`, `:183`, `:191`, `:198-199`, `:206`). *Inferred:* the new scalars below fit in them. A colour needs a new `vec4`.

## 2. Three decisions that cut across every trait

### C1. Where the group lives, and what "off" means

fn-218's gate asks for "the group off (an old beech)". Telperion's "dark green above, silver beneath" and Laurelin's gold rim are colours, and fn-10 put them in "the colour rows" of the preset (fn-10 R9). They cannot be both natural rows and off with the group.

- **X1: new leaf terms are natural material rows, and the group gates scale and light only.**
  - The legendary preset sets silver and gold in the natural rows.
  - Off is a beech in form and size, still with silver or gold leaves.
  - It is the cheapest option, and a whitebeam or white poplar can use the same rows. It does not give fn-218's "old beech" sheet.
- **X2: the group carries its own appearance rows, added on the natural ones while it is enabled (recommended).**
  - The back reflectance and tint, the margin reflectance, emission, dew and cycle are group rows, each dormant at zero. `enabled` gates them all to neutral through the existing `applied` path (`bias.rs:196-200`, `blend.rs:126-143`).
  - The natural front, back and margin colours stay natural rows (`material/params.rs:128-176`, `:503-533`), so Telperion's darker green front is a natural value.
  - Off is an old beech in every respect but a darker leaf.
  - The group's rows move out of `/skeleton/bias/supernatural` into a group of their own (`/supernatural/...`), with each row tagged by the stage it acts in (`=> [Grow]` or `=> [Draw]`, as `bias.rs:18` and `material/params.rs:12` do). The renderer takes the group's applied draw rows beside the material (`Renderer::set_material`, `lib.rs:305-308`).
  - *Inferred:* moving the writhe rows' paths changes the wire and the browser catalogue (`catalogue/mount.rs:123-124`). Leaving them where they are and adding the new rows beside them is the cheaper reading. The host picks one.
- **X3: off is the base preset itself.**
  - A legendary preset names its base, and the group's strength walks the family blend from base to legend. `blend::families` already interpolates every row (`blend.rs:24-45`).
  - It is the most general option: any row can be legendary.
  - It needs preset inheritance in the value-file grammar (`presets/grammar.rs`), which does not exist (read: presets are rows off the default family, `presets.rs:1-3`).

**Recommendation: X2.** It is the one option that gives fn-218's off-sheet without new grammar. It keeps the non-natural physics (a metallic rim, light from flowers) out of the natural rows.

### C2. Two clocks

FLOWERS-DESIGN.md §2 said "fn-217's light cycle reads the same clock" as the season. *Inferred:* it cannot be the same clock and stay cheap:

- the season is a build input that moves geometry (`bloom.rs:65-84`);
- a seven-hour light cycle has to move every frame.

**Recommendation:** a frame clock on the renderer, `Renderer::set_time(seconds)`, set by the harness's loop and by the headless still runner's flag. It sits beside the scene row and is not a property of a tree (`scene/row.rs:1-6` makes the same argument for the sun). The season stays a build input. Each tree's cycle rows read the frame clock.

### C3. Two trees in one frame

The brief's counterphase and mingling need two trees seen and lit together. The renderer holds one (`lib.rs:101`).

- **T1: a renderer holds N trees.**
  - Wood, leaf and organ foliage, crown uniforms and level deviations become per tree (`lib.rs:102-129`).
  - It is the right end state for any stand of trees. It is the largest change, and it reaches every pass.
- **T2: two renderers draw into one target (recommended for now).**
  - The first draws the room and its tree without resolving; the second loads the targets, skips the room and resolves.
  - Needs: a `room: bool` and a `resolve` choice on `draw_with`. Today the room always clears (`lib.rs:521-527`), and the vegetation pass always resolves and discards the samples (`pass.rs:142-145`).
  - Each tree keeps its own sun shadow map, so neither casts a sun shadow on the other. That is acceptable for a light-cycle strip.
- **T3: side-by-side stills, each lit by the other's emitter (§3.3, L2) off frame.**
  - No renderer change beyond the emitter list. The strip shows no shared ground.

The trees' light mixes through the emitter list (§3.3) whichever option is taken. **Recommendation: T2, with T1 recorded as its own spec when a stand is wanted.**

*Note on a word.* The brief's "mingling at the crossover" means the hour the two lights cross. It is not the pipeline's `Tree::crossover` (`space.rs:476`), nor the crown and botany crossover in AGENTS.md.

## 3. The traits

### 3.1 Scale

**Today.**

- No scale row exists anywhere (§1).
- On the engine path, height comes from the species' internodes (`PaState::internode`, metres, `crates/telperion-space/src/species.rs:90-91`) and the age.
- The conversion turns each phytomer tip into a node position (`space.rs:182-184`, `:401`) and its radius into the node's (`:403-408`).
- The device path turns its own fine roots z-up to y-up in `generation/fine/dress.wgsl:221`.
- Leaf stations are counted along a run's grown wood (`Tree::grown`, pushed at `space.rs:430`) in whole twig internodes (`pipeline/foliage/plan.rs:82-97`).
- Blooms are one per flowering drawn node (`space.rs:431-437`).

**What it needs.** One row, `scale`, neutral 1, in 0.1 to 100, refused by name off its range, read as 1 while the group is off (fn-10 R6). It also needs a decision on what grows with it. In fn-10's words, the wood scales and the leaves keep the base species' size.

**Options.**

- **S1: scale after growth, at the conversion (recommended).**
  - Multiply positions and radii by `scale` where the engine's output becomes the pipeline's: `up()` (`space.rs:182`) for the CPU, and `dress.wgsl:221` for the device.
  - The engine grows the natural tree, so its light, sag and girth act in natural metres, and off is the natural tree.
  - The merge and thinness thresholds (`FINEST`, `SHORTEST`, `space.rs:46-47`, `:373`, `:384`) are taken before the scale, so the topology is the natural tree's.
  - The trees are not byte-identical at 1 if the multiply is done in float. Applied only where `scale != 1`, or exactly as `x * 1.0`, which IEEE keeps, they are identical.
  - *Inferred, the choice inside S1:*
    - *grown wood scaled too:* leaf stations rise with `scale`, so the crown keeps its leaf density along the wood. Leaves rise about in proportion to `scale` (×5 for a 150 m Telperion on a 30 m beech), with the frame and build cost in proportion.
    - *grown wood not scaled:* the same leaves spread `scale` times thinner.

    Recommendation: scale `grown` with the positions.
  - Blooms do not rise with the scale (one per node). Florets per crown area fall by about `scale²`, so the legendary preset's `florets` row has to carry it. *Inferred.*
  - Cost: both executors, plus the identity pins. Two to three tasks (estimate).
- **S2: scale in the engine, before growth.**
  - `Species::scaled(s)` multiplies every length row (internode, sag and girth constants) before the engine grows.
  - Light, sag and girth then act at the new size. *Inferred:* the result is a different tree, not the natural one enlarged; a giant would sag more.
  - It is one change, in the engine. The off-sheet holds. Telperion at scale would no longer read as "an old beech, enlarged". Engine and fine-contract tests move. Whether the fixed-point geometry holds a 150 m tree is *unknown* (`crates/telperion-space/src/geometry.rs:20-31` is Q32).
- **S3: no row.**
  - The Two Trees get their height from age and internode values.
  - Off is then not species-sized, which breaks fn-218's off-sheet.

**Recommendation: S1, with the grown wood scaled and the legendary preset's `florets` set for the area.** Neutral 1. Cost estimate: per frame, proportional to the leaves and florets the scale adds; no new pass.

### 3.2 Two-sided leaf colour with an edge term

**Today.**

- Two-sided colour exists. The fragment takes `leaf_back` or `leaf_front` by the face the eye sees (`foliage.wgsl:132`), from `leafFront*` and `leafBack*` (`material/params.rs:128-176`, `scene/frame.rs:108-114`). Both are diffuse albedos clamped to 1 (`foliage.wgsl:166-168`).
- The highlight is front-only: "the back has no cuticle" (`foliage.wgsl:208-215`). The grazing sheen reads both faces (`:227-235`).
- An edge term exists. A margin band of `marginWidth` adds `margin*` as an albedo offset (`foliage.wgsl:160-168`; rows `material/params.rs:503-533`). It is diffuse only, so it reads as a yellow edge and not as a glint.
- There is no wind (no time, §1). *Inferred:* "the silver shows as leaves turn in wind" can be shown on a still only where the leaves already face away from the eye. Wind is its own gap, which this design does not take.

**What it needs.** A silver underside is a bright, low-roughness reflection tinted grey-white (a tomentose underside). A gold rim is a metallic reflection tinted gold. Neither is an albedo.

**Options.**

- **E1: values only.** A pale `leafBack*` and a yellow `margin*`. No code. It reads as pale green and yellow, not silver and gold. *Inferred* from the clamp and the diffuse-only use.
- **E2: two reflectance terms, dormant at 0 (recommended).**
  - `backReflectance` with `backTint*`: the existing `highlight()` (`canopy.wgsl:68`) evaluated on the back face too, with its own foot and colour (a metal's reflectance is its colour). The sheen term (`foliage.wgsl:227-235`) takes the same tint on the back.
  - `marginReflectance` with the margin colour as its tint: the same highlight weighted by the margin band already computed at `foliage.wgsl:162`.
  - Both are zero for every passed species, and the added arithmetic multiplies by zero. *Inferred:* identical pixels when built as `+ 0.0 * term`. To be checked by the R3 stills.
  - Cost: one more `highlight()` per leaf fragment, under 2 % of the leaf fragment (estimate).
- **E3: E2, plus a glint that sparkles.** A hashed per-leaf, per-band speckle on the margin, filtered to the pixel's footprint by the far-draw rule (flow memory, the far draw is the near draw minus what the eye cannot resolve). It gives "glittering". It adds a few percent more, with the risk of shimmer in motion.

**Recommendation: E2,** with the rows in the group (C1, X2). E3 waits for fn-218's sheet to say whether a smooth rim reads as gold. Neutral: both reflectances 0.

### 3.3 Emitted light from flowers

**Today.** No emission term exists in any shader or row (grep `emission`, `emit`, `glow` over `crates/telperion-render/src/shaders` and `material/params.rs`; `curve_emit.wgsl` is wood surfacing, not light). The organ is drawn by the leaf shader under its own block (§1). The tone map rolls radiance over about 3 to white (`common.wgsl:194-206`).

**What it needs.** Three things at three costs:

1. the flower is bright;
2. the tree and its ground are lit by it;
3. the air glows around it.

**Options, by part.**

- **L1, itself (recommended):**
  - `bloomEmission*` and `bloomEmissionStrength` in the organ block, added to the floret's radiance before `tone()` (`foliage.wgsl:225-236`), times the cycle (§3.4). The leaves' block carries zero.
  - Cost: one multiply-add per organ fragment, about 0 (estimate).
- **L2, the surroundings, analytically (recommended):**
  - Each tree's bloom is one emitter: the ellipsoid its florets fill, which the organ's mass grid already bounds (FLORETS.md, D5), with a radiance from L1's rows times the cycle, and a `reach` row.
  - Every lit fragment adds the emitter's irradiance through a sphere-light form factor beside `ambient()` in the prelude (`common.wgsl:147-152`). That one change reaches all four lighting sites (§1).
  - Inside the ellipsoid, the existing crown depth term (`common.wgsl:157-163`) blends it to the interior's even glow.
  - There is no shadow, so light passes through the trunk. *Inferred:* acceptable for a diffuse glow of thousands of sources.
  - The frame block takes an emitter list of two (C3: each tree lit by both). Dormant at reach 0.
  - Cost: about 20 to 30 arithmetic operations per lit fragment, against a leaf fragment of a few hundred. That is +5 to 10 % of fragment time (estimate): about +0.15 to 0.3 ms on the beech hero and +0.9 to 1.7 ms on the oak hero (fn-208's vegetation times).
- **L3, the surroundings, with shadow:**
  - An omnidirectional shadow map from the bloom's centre: six faces re-drawing wood and leaf casters.
  - Cost: about six times the sun's shadow pass (estimate; that pass's own time is not in fn-208's table, *unknown*). A frame cost of +50 to 150 % is the likely range.
- **L4, many lights:**
  - Florets as point lights in tiled or clustered lighting. Florets number 0.1M to 3.4M per tree (FLORETS.md, beech 40 s7: 3,369,006), so a sampled set and a binning pass would be needed.
  - The most faithful option and the most expensive one, never needed for a diffuse glow (inferred).
- **G1, glow in the air, analytic (recommended):**
  - Single scattering from L2's emitter along each pixel's ray, in closed form for a sphere light (the airlight integral). It is added before `tone()` in the sky, the room and the vegetation.
  - It needs no new pass and no new target. A `haze` row, dormant at 0.
  - Cost: about +0.05 to 0.2 ms at 960 × 720 (estimate).
- **G2, glow as a post-process:**
  - A floating-point colour target, a down and up blur chain and a final tone pass. `tone()` moves out of every shader (`common.wgsl:202`).
  - Every passed species' pixels then move by rounding. Keeping them requires the chain only when emission is on, which is two ways of drawing (principles: no switch between ways).
  - Cost: +0.5 to 1.5 ms, plus a 4-sample float target of about 22 MB at 960 × 720 (estimate).

**Recommendation: L1 + L2 + G1.** Neutral: emission 0, reach 0 and haze 0; all three terms then read zero. L3 and G2 are recorded as successors if the sheets show light leaking through the trunk or a halo that reads flat.

**Scene.** The text's Trees light a world without a sun. A dim scene row (sun near 0, a dark sky) is a scene value, not a tree value (`scene/row.rs:22-38` bounds the sun at 0 to 10). The strip's scene is the host's call.

### 3.4 The cycle, two trees in counterphase

**Today.** There is no clock (§1, C2).

**What it needs.**

- A frame clock (C2).
- Per tree:
  - a `period` (seconds of frame clock per cycle; the text's hours are a value);
  - a `phase` (share of the period);
  - a `depth` (how far the light falls from full: 0 steady, 1 to dark);
  - a `shape` (how long the light dwells at its peak).
- The cycle multiplies L1, L2, G1 and the dew's brightness.
- Counterphase is a value (Laurelin's phase is Telperion's plus one half). The crossover is where the two curves are equal.
- Mingling is the two emitters adding in L2 and G1. It needs no term of its own (inferred).

**Options.**

- **K1: a raised cosine to a power, `1 - depth · (1 - (0.5 + 0.5 cos 2π(t/period + phase))^shape)` (recommended).** It is smooth, so nothing pops. At depth 0 it is constant. `shape` widens or narrows the peak, so the lights can overlap for a long or a short mingling.
- **K2: a piecewise wax, hold and wane.** It matches the text's "waxed and waned" more literally. It has corners, which read as steps in a time strip.
- **K3: florets open and close with the cycle.** A vertex-stage size factor on the organ, by degree. It is cheap (a multiply in the vertex stage), but it is geometry that moves with a light. It is not in the brief, and is recorded only.

**Recommendation: K1.** Neutral: depth 0, and the rows are dormant where emission is 0. Cost: one uniform and a cosine per frame, about 0.

### 3.5 A falling dew of light

**Today.** No particle, drop or rain exists in the renderer (grep: `particle`, `drop`, `rain`, `dew` over `crates/telperion-render/src`). There is no blended pipeline (`pass.rs:294`). Selection keeps no state between frames (`shaders/select.wgsl:3-4`).

**What it needs.** Drops that fall from the florets to the ground, lit by their own emission, by degree.

**Options.**

- **D1: stateless drops (recommended).**
  - Drop `i` belongs to floret `hash(i) mod florets` of the organ's placement buffer, the one `foliage.wgsl` already reads (`:11`, `:52-55`). Its height is the floret's minus a fall speed times `(time + offset_i) mod fall_time`, ending at the ground.
  - Each is a small view-facing streak, drawn by an instanced draw with no compute pass and nothing remembered between frames.
  - Rows: `drops` (per floret, a share), `fallSpeed`, `dropSize` and colour from the emission, all dormant at `drops` 0. The cycle scales brightness and count by degree: a drop past the count's fraction is drawn at size zero.
  - Opaque and emissive, there is no new blend state. Additive blending would read softer, and is one pipeline variant.
  - Cost: at 50k drops, 100k triangles against a crown of millions, under 0.2 ms (estimate).
- **D2: drops placed at build,** as a second organ list animated in the vertex stage. It costs the same per frame and adds build memory. Nothing gained over D1 (inferred).
- **D3: a simulated particle buffer** carried across frames. It breaks determinism and the select pass's no-state rule. Not recommended.
- **D4: light pooling on the ground** ("light pooled", the brief). The ground under the crown takes an emissive pool that fills as the cycle waxes. It is a term in `scene.wgsl:36-37` reading L2's emitter. It is an optional addition to D1, dormant at 0.

**Recommendation: D1, with D4 as a value-gated option the host may take or leave.** Neutral: drops 0, pool 0.

### 3.6 Five-petal florets (fn-216 follow-up 1)

**Today.**

- The organ element is one petal turned `petals` times about the floret's axis, opened `open` degrees (`pipeline/foliage/bloom.rs:86-145`).
- The petal is a leaf blade with fixed rows (`bloom.rs:92-106`): base fullness 0.85, tip sharpness 1, curl 0, no lobes, 4 by 2 segments, and the cup the `petalCup` row sets.
- The floret has no centre, stamens or eye tint. The `bloomEye*` base tint FLOWERS-DESIGN.md §3 proposed was not built (`material/params.rs:208-257` holds only the two faces).
- The organ inherits the leaves' canopy-normal bend (`frame.rs:221-245`; `foliage.wgsl:174-178`), which lights a petal as part of the crown's mass. *Inferred:* that flattens the shading within a flower.

*Inferred causes of "flat white scraps" (STILLS.md):*

1. no centre;
2. one even colour per face;
3. petals lit as a mass;
4. florets close enough to fuse at the corymb.

**Options.**

- **F1: an eye tint (recommended).** `bloomEye*` and `bloomEyeShare`, the colour toward the petal's base along `coord.x`, the margin term's dual at the other end of the blade (`foliage.wgsl:160-163`). Dormant at share 0. Shader only, in the organ block.
- **F2: stamens (recommended).** `stamens` (count, by degree as `petals` is) and `stamenLength`. A ring of thin filaments added to the organ element, coloured by the eye tint through their coordinates. Element only (`bloom.rs:86-145`). Dormant at 0. Cost: the organ's triangles rise by the filaments' share, about +50 to 100 % at the finest level, less at distance (estimate).
- **F3: the organ lit as a surface.** The organ block sets the canopy-normal bend to zero (`frame.rs:221-245`, one more line beside `:234-236`). This is not a row. It is the organ's block choice, as veins off is. *Unknown* whether it reads better, until a still is taken.
- **F4: petal shape rows.** Expose curl and a tip notch (a cherry's petal is notched). The element has `curl` and `lobe_count` (`bloom.rs:100-101`), so a notch may be a two-lobe tip. Whether the lobe rule gives one is *unknown*.

**Recommendation: F1 + F2 + F3,** with F4 recorded until a sheet shows the outline is what fails. Neutral: eye share 0 and stamens 0. F3 changes the look of every flowering tree, which today is only the test overlays (no shipped preset flowers, FLORETS.md "space_hash").

### 3.7 Blossom at whole-tree distance (fn-216 follow-up 2)

**Today.**

- Selection picks for each floret the coarsest level under half a pixel of deviation. Nothing is dropped for size: only a floret outside the frame goes to the unseen bucket (`shaders/select.wgsl:67-101`).
- A sub-pixel floret is rasterised as is, with 4 samples and no alpha-to-coverage (`pass.rs:17`, `:251`, `:311`).
- *Inferred:* at the hero pose a 3 cm floret on a 25 m tree spans about one sample or less, so its coverage is a hit or a miss.
- The organ's coarsest level replicates the petal's three-point ladder (`bloom.rs:132-143`; the anchors are base, widest section and tip, `pipeline/foliage/levels.rs:171-186`). Whether that loses area at distance is *unknown*.
- A value cause is also possible: the beech test was a beech, whose flowers are inconspicuous (STILLS.md, host decision 2).

**Options.**

- **M0: measure first (recommended).** On a cherry-dense overlay (florets, size and flowering share at a cherry's), count the organ's covered pixels at the hero pose against the florets' projected area. It is one headless still and the frame's own counts, inside the token budget's four images. If the ratio is near one, the gap is values, not the renderer.
- **M1: values.** More and larger florets on the outer ages. No code.
- **M2: coverage kept by alpha-to-coverage (recommended if M0 shows a loss).**
  - In the organ's vertex stage, a floret projected under `k` pixels is enlarged to `k`. Its fragment outputs coverage equal to the true area over the enlarged area.
  - The organ pipeline turns alpha-to-coverage on, with the 4 samples already there. The expected coverage is the floret's own area, so the far draw is the near draw minus what the eye cannot resolve.
  - It applies to the organ only, so no passed species moves.
  - Cost: the enlarged florets' fragments, +0.2 to 1 ms at millions of florets (estimate).
- **M3: the unresolved blossom carried by the leaves.** Each leaf fragment mixes toward the bloom colour by its cell's floret-to-leaf area in a grid like the mass grid (`mass.rs:1-60`), weighted by how far florets there fall under a pixel. One grid read per leaf fragment, under 2 % (estimate). It is energy-correct only as the florets vanish, and it puts blossom colour on leaves.

**Recommendation: M0, then M1 or M2 by its result.** M3 is recorded as the alternative if M2's fragments cost too much on the oak-scale crowns. Neutral: none. M2 is a draw rule of the organ, not a row.

### 3.8 fn-10's writhe and spiral on the engine

The spec puts "fn-10's supernatural group ... on the engine path". The brief asks for none of the writhe. It asks that both trees be recognisable old beeches first (VISION-TWO-TREES.md, "The design").

- **W1: leave the rows to the scaffold and do not port them (recommended).** They stay test-only until fn-198 R0 removes the scaffold (`pipeline.rs:182-185`). At that point the rows' deletion is the host's or owner's call.
- **W2: carry them as the engine's own form.** Engine axes already wander by keyed turns (`Form::wander`, `species.rs:204`; `geometry.rs:1-8`). A legendary wander is then a value, not a field.
- **W3: apply the bias field to the converted tree.** A deformation after conversion, in both executors. *Inferred:* this is a second chain over geometry the engine laid, which moves sag and contact after the engine judged them. Not recommended.

**Recommendation: W1.** It is the "question, delete" reading of rows that no production tree reads.

## 4. Cost and the R3 measurement

All per-frame numbers are estimates against fn-208's measured beech and oak heroes. R3 measures them on the built terms with the existing timing record (`lib.rs:89-99`: vegetation, selection, shadow and surfacing pairs), plus a pair for the dew draw if D1 is built.

| Term | Pass it lands in | Estimate |
|---|---|---|
| E2 leaf reflectances | vegetation (leaf fragment) | < 2 % of the leaf draw |
| L1 floret emission | vegetation (organ fragment) | ≈ 0 |
| L2 analytic emitter (×2 trees) | room and vegetation (every lit fragment) | +5 to 10 % of fragment time; beech hero +0.15 to 0.3 ms, oak hero +0.9 to 1.7 ms |
| G1 analytic airlight | sky, room and vegetation | +0.05 to 0.2 ms at 960 × 720 |
| K1 cycle | a uniform | ≈ 0 |
| D1 dew, 50k drops | vegetation (a new instanced draw) | < 0.2 ms |
| F2 stamens | vegetation (organ) | organ triangles +50 to 100 % at the finest level |
| M2 coverage | vegetation (organ) | +0.2 to 1 ms at millions of florets |
| S1 scale | build; leaf and organ draws in proportion | leaves ×`scale` at the same density; no new pass |
| T2 second tree | a second tree's wood, selection, shadow and vegetation | about one more tree's frame |

**Work** (estimates, a task each of an hour or two of agent time):

| Step | Tasks |
|---|---|
| C1 group rows (X2), wire, catalogue, blend, `enabled` gate, identity pins | 2 |
| C2 frame clock (renderer, wasm, harness, still runner) | 1 |
| S1 scale in both executors, pins, ceilings | 2 to 3 |
| E2 leaf reflectances | 1 |
| L1 + L2 + G1 emission, emitter list, airlight | 2 to 3 |
| K1 cycle | with L1 |
| D1 dew (+ D4 optional) | 2 |
| F1 + F2 + F3 floret form | 2 |
| M0 measure; M2 if needed | 1 to 2 |
| T2 two renderers in one target | 1 |
| R2 stills and time strip; R3 identity and frame cost | 2 |
| **Total** | **≈ 17 to 21** |

The spec could split along the brief's own seams: (a) scale and leaves, (b) light, cycle and dew, and (c) floret form and distance. Whether to split is the host's call.

## 5. Decisions for the host

1. **C1:** X2, the group's own appearance rows gated by `enabled`, so off is an old beech; and whether the writhe rows move to the new group path or stay under `/skeleton/bias/supernatural`.
2. **C2:** a frame clock on the renderer, separate from the build's season.
3. **C3:** T2 (two renderers into one target) for the Two Trees' strip, with T1 left to a stand spec.
4. **Scale:** S1, after growth, with the grown wood scaled so leaves keep their density.
5. **Leaves:** E2 (back and margin reflectance); E3's sparkle only if the sheet asks for it.
6. **Light:** L1 + L2 + G1, with no shadowed glow and no post-process for now; and the scene row the strip is drawn under.
7. **Cycle:** K1's raised cosine with depth and shape.
8. **Dew:** D1 stateless drops, opaque or additive; D4's pool, yes or no.
9. **Florets:** F1 + F2 + F3.
10. **Distance:** M0 first, then M1 or M2 by the measurement.
11. **Writhe:** W1, not ported.
12. **Split:** one spec, or three along the brief's seams.

Not in this design: the Two Trees' values (fn-218, fn-219); wind; the organ in the sun's shadow pass (fn-125, FLORETS.md "Shadows").

## 6. Host decisions (2026-10-08)

Release lens: the owner's goal is a release PR; the old generator's Telperion and Laurelin never glowed ("leave luminosity for later", 2026-09-14), so light is not release parity.

1. **C1: X2.** The group's own appearance rows are gated by `enabled`, so off is an old beech. The new rows live in a new `/supernatural/...` group. The writhe and spiral rows stay where they are until R0 removes the scaffold. They change no engine tree, so they are recorded as dead rows for R0 to delete (decision 11).
2. **C2, frame clock:** yes, but built with the light spec (12c), not now.
3. **C3: T2,** with the light spec (12c); T1 is left to a future stand spec.
4. **Scale: S1,** after growth, with the grown wood scaled so leaves keep their density.
5. **Leaves: E2,** back and margin reflectance; E3's sparkle only if a sheet asks for it.
6. **Light: L1 + L2 + G1,** in the light spec (12c), after the release.
7. **Cycle: K1,** in the light spec.
8. **Dew: D1,** stateless and additive, in the light spec; no D4 pool.
9. **Florets: F1 + F2 + F3.**
10. **Distance: M0 first, then M1 or M2** by the measurement.
11. **Writhe: W1, not ported.** Its rows are listed for deletion with R0's scaffold removal.
12. **Split into three specs:**
    - (a) scale and leaves, needed for the release;
    - (b) floret form and blossom at distance, needed for the release;
    - (c) emitted light, cycle, dew and two trees in one frame, after the release.

    fn-218 and fn-219 depend on (a) and (b) only.
