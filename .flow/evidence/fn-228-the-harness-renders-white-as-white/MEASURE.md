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

**R2's bar:** a neutral surface stays neutral in sun and in shade, with each channel within 3% of the mean of the three (`TOLERANCE` in the test). The default fails three of its four readings. The test asserts this bar and is `#[ignore]`d until R2 lands.

## The fix is a design call (escalated, not made)

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
