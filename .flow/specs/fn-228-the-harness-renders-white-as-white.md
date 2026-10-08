# The harness renders white as white

## Goal & Context

The silver birch's leaf rows were set to a measured linear colour (S2-Oslo's sunlit leaves, blue-to-green 0.46), yet render at 0.57-0.65; the white birch stem's bark rows (0.92) render at 1.06-1.19 (fn-213 SHEET-11.md, round 11b). So the scene's light, not the rows, adds blue, and every species likely reads cooler than its photographs. Owner (2026-10-08): "white-and-grey check ok".

## Requirements

- **R1 (measure):** a neutral white and a neutral mid-grey object (linear albedo stated) rendered in the harness scene under its default sun and sky, lit and in shadow; their rendered colour measured by code; the cause located (sky ambient colour, sun colour, tonemapping or exposure) by changing one at a time.
- **R2 (fix):** the scene's light corrected so a neutral object renders neutral (each channel within a stated tolerance of the others) in sun and in shade, by the cause R1 finds; no species row changes.
- **R3 (shown):** hero stills of every shipped species before and after, viewed; the birch leaf and bark ratios re-measured; the headless stills and the browser harness agree; gate and npm test green.

## Boundaries

The scene's lighting only; species colours are their own specs. Depends on fn-198 (F4).