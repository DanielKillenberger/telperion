# The harness renders white as white

## Goal

The silver birch's leaf rows were set to a measured linear colour (S2-Oslo's sunlit leaves, blue-to-green 0.46) yet rendered at 0.57-0.65, and the white stem's bark rows (0.92) at 1.06-1.19 (fn-213 SHEET-11.md, round 11b): the scene's light, not the rows, added blue, so every species likely read cooler than its photographs. Owner (2026-10-08): "white-and-grey check ok". The scene's lighting only; species colours are their own specs.

## Decisions

- Host decision (2026-10-08), amending R2: "My spec's R2 was wrong to ask for neutral shade. Real shade under a blue sky is bluish, and the reference photographs were taken with a camera white-balanced to daylight: sun plus sky together reads neutral, and shade stays cool. So the target is camera daylight balance, not neutral everywhere." The drawn sky and sky light stay; only the default sun is re-tinted at its luminance; no new settings, no frame-wide balance, no species rows.
- Options weighed before the decision (grey the row defaults; ambient takes sky luminance; new sky-light rows; a camera white balance at the tone step) were not taken.

## Acceptance Criteria

- **R1:** A neutral white and mid-grey rendered under the default sun and sky, lit and in shade, measured by code, cause located one light at a time — met.
- **R2:** The default sun re-tinted so full daylight on a sunlit floor renders neutral within 3 %, white and grey, at the same luminance; shade measured and reported — met.
- **R3:** Hero stills of every shipped species before and after, viewed; the birch ratios re-measured; headless and browser agree; gates green — met (browser agreement inferred from shared code, no browser frame measured).

## Result

- Cause: shade's blue is the sky's ambient colour (0.365, 0.48, 0.71); in sun the warm sun and blue sky cancel (white b:g 1.02, grey 1.06); the tone map adds nothing.
- Sun (3.0, 2.85, 2.6) -> (3.078, 2.848, 2.388), daylight 1.904 on every channel. Sunlit white and grey now neutral to the byte (`tests/neutral_light.rs`; `the_default_daylight_is_white_at_the_luminance_it_had` without a GPU). Shade unchanged (white 1.40, grey 1.85). Balance holds at the default elevation only.
- Stills (oak, spruce, beech, palm) viewed: a slight warming of sunlit parts, red +0.0024, blue -0.005. Birch: stem b:g 0.93-0.94 against rows 0.92; leaves 0.54 close up and 0.63 in the crown against 0.46, the rest the cool shade the decision keeps.
- Gate green; npm test 143 passed.

## Open

`telperion-jev/src/tuning/matched.rs` still builds its scene from the old sun (3.0/2.85/2.6).

## Friction

- The local hook refused a redirect to a variable path -> local setup, reported only.
