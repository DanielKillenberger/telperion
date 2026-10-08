# fn-228 R1: what tints a neutral surface (worker, 2026-10-08)

**Source:** `crates/telperion-render/tests/neutral_light.rs`. Raw log and stills are in the ignored `raw/r1.log` and `raw/r1/`.

**Setup.** The floor is the neutral object. Its shader is only the light: albedo × (ambient + sun). Its albedo is a scene-row value, set to 0.8 for the white and 0.18 for the grey. The scene is the default sun (30° elevation) and the default sky. The ordinary tree stands on the floor, and the camera looks straight down at it.

- **Floor pixels:** every pixel that comes back exactly black when the ground row is black.
- **Sun and shade:** the sunlit sample is the brightest 5% band of those pixels. The shaded sample is the darkest band, which is the tree's cast shadow.
- **Values:** linear means of 8-bit sRGB pixels, after the tone map. The browser takes its default row from the same Rust `SceneRow::default()`, so the stills and the harness see the same light.

## One light at a time

| Light | White in sun | White in shade | Grey in sun | Grey in shade |
|---|---|---|---|---|
| Default | 0.783/0.783/0.799 (b:g 1.02) | 0.259/0.347/0.485 (**1.40**) | 0.301/0.309/0.328 (1.06) | 0.031/0.047/0.087 (**1.85**) |
| Sky's light grey (same luminance) | 0.791/0.783/0.768 (0.98) | 0.342/0.342/0.342 (1.00) | 0.319/0.305/0.287 (0.94) | 0.045/0.045/0.045 (1.00) |
| Sun grey (same luminance) | 0.768/0.783/0.807 (1.03) | as default | 0.292/0.309/0.347 (1.12) | as default |
| Both grey | 0.783/0.783/0.783 (1.00) | 0.342/0.342/0.342 (1.00) | 0.309/0.309/0.309 (1.00) | 0.045/0.045/0.045 (1.00) |

## The cause

- **Shade: the sky's colour.**
  - `ambient()` in `common.wgsl` lights every surface with the mean of the zenith and the horizon, (0.365, 0.48, 0.71).
  - Making that grey makes the shade exactly neutral.
  - This is the blue in the birch's crown and on its stem.
- **Sun: the warm sun and the blue sky offset each other.**
  - The sun is (3.0, 2.85, 2.6), so with a grey sky a sunlit grey goes warm (0.94).
  - With a grey sun, the sunlit grey goes cool (1.12).
  - The default's near-neutral white in sun (1.02) is the two cancelling. The grey still reads 1.06.
- **Tone map and exposure: no part.** With both lights grey, every reading is neutral to the byte. The curve is the same on every channel, so it cannot add a tint, only change how much an existing one shows.

## Host decision (2026-10-08): daylight balance, shade stays cool

**Host:** "My spec's R2 was wrong to ask for neutral shade. Real shade under a blue sky is bluish, and the reference photographs were taken with a camera white-balanced to daylight: sun plus sky together reads neutral, and shade stays cool. So the target is camera daylight balance, not neutral everywhere."

**The decision:**
- The drawn sky and the sky's light stay as they were.
- The default sun is re-tinted so that full daylight on a neutral surface reads neutral. Full daylight is the sun plus the sky, measured on a sunlit level floor.
- The sun keeps its luminance.
- No new settings, no frame-wide balance, no species rows.

The spec's R2 is amended to match. The section at the end of this file comes from before the decision. It stays as the record of what was weighed.

## R2: the sun re-tinted

**The solve.** Full daylight on a level floor is the sky's mean colour plus sin(30°) times the sun. The new sun is the one that makes that sum grey at the old sun's luminance (2.864):

- The sun goes from (3.0, 2.85, 2.6) to **(3.078, 2.848, 2.388)**, in `scene/row.rs`.
- Daylight comes to 1.904 on every channel.
- `the_default_daylight_is_white_at_the_luminance_it_had` checks both the balance and the luminance. It needs no GPU.

**Rendered** (`neutral_light`, now unignored; log in `raw/r2.log`):

| | White in sun | White in shade | Grey in sun | Grey in shade |
|---|---|---|---|---|
| Before | 0.783/0.783/0.799 (b:g 1.02) | 0.259/0.347/0.485 (1.40) | 0.301/0.309/0.328 (1.06) | 0.031/0.047/0.087 (1.85) |
| After | **0.783/0.783/0.783 (1.00)** | 0.259/0.347/0.485 (1.40) | **0.309/0.309/0.309 (1.00)** | 0.031/0.047/0.087 (1.85) |

- **Sun:** the test asserts the sunlit bar, each channel within 3% of the mean of the three, on white and grey. Both now read neutral to the byte.
- **Shade:** reported, not bounded. It is unchanged because the sky is unchanged.
- **Balance holds at the default elevation only.** A row that moves the sun's elevation moves the balance with it, because sin(elevation) weights the sun against the sky.

## R3: the stills and the birch

### Hero stills of every shipped species

`headless --seed 1 --size 1200x800`, before with `--scene` set to the old sun, after on the default. Stills and side-by-side pairs are in `raw/r3/`. The worker viewed all four pairs.

- **Shipped:** oak, spruce, beech and palm. On this branch, silver-birch, telperion and laurelin are refused with "no engine species".
- **Change:** a slight warming of the sunlit parts. No change in structure or exposure.
- **Frame means, linear, before → after:**
  - Red rises by about 0.0024 on every species.
  - Green is unchanged.
  - Blue falls by about 0.005.
  - Example, the oak: 0.199/0.231/0.181 → 0.201/0.231/0.176.

### The birch (fn-213 round 11b)

**Setup:**
- Rendered in a temporary checkout of `b64872a2` with fn-213's `space_birch` and `leaf_closeup`.
- Before is that commit as it stands. After is the same commit with only the three sun defaults changed.
- The pairs are in `raw/birch/`, and the worker viewed all six.
- The leaf numbers come from fn-213's own `leaf_colour.py --rendered`. Its before values reproduce SHEET-11b's exactly.

**Leaves** (the rows state b:g 0.46):

| Still | Before | After |
|---|---|---|
| Crown @80 | 0.282/0.302/0.197 (0.93:1:0.65) | 0.286/0.302/0.189 (0.95:1:0.63) |
| Close-up @40 | 0.213/0.233/0.133 (0.92:1:0.57) | 0.217/0.233/0.125 (0.93:1:0.54) |
| Close-up @80 | 0.262/0.282/0.161 (0.93:1:0.57) | 0.265/0.281/0.151 (0.94:1:0.54) |

**Stem** (the bark rows are 0.78/0.76/0.70, b:g 0.92):

- **Measure:** `stem_colour.py`, beside this file, on the bare base shot. It reads the trunk's columns below the horizon, so the lower trunk only.
- **Not SHEET-11b's figure:** SHEET-11b's 0.406/0.476/0.564 (1.19 at 80) was taken with a mask that sheet does not record. This one does not reproduce it.

| Still | Before | After |
|---|---|---|
| Base @40 | 0.550/0.539/0.523 (1.02:1:0.97) | 0.554/0.537/0.503 (1.03:1:0.94) |
| Base @80 | 0.127/0.128/0.127 (1.00:1:0.99) | 0.130/0.128/0.119 (1.02:1:0.93) |

**What is left:**
- The sunlit stem now renders at its rows' ratio: 0.94 and 0.93 against 0.92.
- The leaves move from 0.57 to 0.54 up close and from 0.65 to 0.63 in the crown, still above the rows' 0.46.
- The rest of the leaves' blue is the cool shade inside the crown, which the host's decision keeps. The leaf rows are fn-213's to judge against it.

### The browser harness

The page reads its scene row from the renderer: `WebRenderer::scene()` returns `SceneRow::to_json()` of the default, and the shaders are the same WGSL. So the browser and the headless stills take the same sun. This is inferred from the code; no browser frame was measured.

### Gate

- **Render half first, under the GPU lock:** `cargo test --profile ci -p telperion-render --no-fail-fast` exits 0.
- **Then the workspace:** `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` exits 0.
- **Logs:** `raw/gate-*.log`.

### Not changed

`crates/telperion-jev/src/tuning/matched.rs` builds its own scene from the old sun (3.0/2.85/2.6, dimmed for overcast). That is outside this spec's scope, so it is left alone.

## Before the decision: R1's first bar and the options

**The first bar:** a neutral surface stays neutral in sun and in shade, with each channel within 3% of the mean of the three. The default failed three of its four readings.

Under the bar, both the sky's light and the sun have to be neutral. The sky backdrop (`sky.wgsl`) is drawn from the same two row colours the ambient light reads. So greying the light means choosing between these:

1. **Grey the row defaults** (zenith, horizon and sun at their own luminance).
   - No code change.
   - The drawn sky turns grey as well.
2. **The ambient takes the sky's luminance, not its colour,** and the sun's default goes grey.
   - The backdrop keeps its blue.
   - A row that tints the sky (a sunset) would no longer tint the light. That is a lost capability in an existing row field.
3. **New scene-row fields for the sky's light** (`skyLightRed/Green/Blue`), separate from the backdrop, defaulting to a grey of the current luminance, and the sun's default goes grey.
   - Keeps both the blue backdrop and the tint capability.
   - Adds three row fields, which the panel shows through its generic controls.
4. **A camera white balance at the tone step.**
   - It can make the sunlit side or the shade neutral, but not both: the two are lit by differently coloured light.
   - So it cannot meet R2 as written.

Every option is a scene-row default or shader change only. None touches a species row.
