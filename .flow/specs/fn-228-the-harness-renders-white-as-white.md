# The harness renders white as white

## Goal & Context

The silver birch's leaf rows were set to a measured linear colour (S2-Oslo's sunlit leaves, blue-to-green 0.46), yet render at 0.57-0.65; the white birch stem's bark rows (0.92) render at 1.06-1.19 (fn-213 SHEET-11.md, round 11b). So the scene's light, not the rows, adds blue, and every species likely reads cooler than its photographs. Owner (2026-10-08): "white-and-grey check ok".

## Requirements

- **R1 (measure):** a neutral white and a neutral mid-grey object (linear albedo stated) rendered in the harness scene under its default sun and sky, lit and in shadow; their rendered colour measured by code; the cause located (sky ambient colour, sun colour, tonemapping or exposure) by changing one at a time.
- **R2 (fix):** the default sun re-tinted so a neutral surface in full daylight (sun plus sky, a sunlit horizontal floor) renders neutral, each channel within 3% of the mean of the three, white and grey both; the sun keeps its current luminance. The drawn sky and the sky light are unchanged, and shade stays cool: it is measured and reported, not bounded. No new settings, no frame-wide balance, no species row changes. Host decision (2026-10-08): the reference photographs are white-balanced to daylight, where sun plus sky reads neutral and shade stays bluish, so the target is camera daylight balance, not neutral everywhere.
- **R3 (shown):** hero stills of every shipped species before and after, viewed; the birch leaf and bark ratios re-measured; the headless stills and the browser harness agree; gate and npm test green.

## Boundaries

The scene's lighting only; species colours are their own specs. Depends on fn-198 (F4).
