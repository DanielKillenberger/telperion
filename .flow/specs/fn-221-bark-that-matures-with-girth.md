# Bark that matures with girth

## Goal

The silver birch (fn-213) failed Astra's trait (b) on sheet 3: an old birch's lower trunk is dark and fissured, but the material rows gave one bark colour to all thick wood, so the 80-year base read "predominantly white with black speckling rather than a convincing dark, fissured lower trunk". The host deferred it as its own capability (fn-213 SHEET-1.md decision c, 2026-10-07). Many species' bark changes with girth. Renderer material and preset rows only; no engine change.

## Decisions

- R2 (the birch's values and judged stills) is fn-213's round 5 on this capability; this spec shows the blend on one specimen with test rows.
- A low/high radius pair was refused by the dial table (a first step from a zero high), so the range became a start (`matureRadius`) and a span (`matureSpan`), letting every dial step on its own.

## Acceptance Criteria

- **R1:** A mature-bark colour and fissure depth blended in by a stated radius range, no effect at zero — met.
- **R2:** The birch's 80-year base dark and fissured beside references, the 40-year stem white — moved to fn-213 round 5.
- **R3:** Every passed species' stills unchanged where the rows are absent; npm test and the gate green — met.

## Result

Six `/material` rows read by the wood shader: `matureRed/Green/Blue`, `matureFurrowStrength`, `matureRadius`, `matureSpan`; a smoothstep from `matureRadius` over `matureSpan`, young shoot colour still blended over it. At `matureRadius` 0 nothing changes (`tests/mature_bark.rs`); 12 neutral stills (beech, oak, spruce, palm; whole, bare, base close-up) byte-identical to the base. Shown on the oak at seed 1 (old bark 0.045/0.04/0.036, furrow 1.0, from 0.08 m over 0.12 m): the base dark and deeply fissured, thinner limbs pale. Notes for fn-213: fissure, crest and weathering tints are absolute offsets, so they still act on dark old bark (the oak's fissure reads purple); colour steps where radius steps at a fork. Gates green; npm test 143 passed.

## Open

None here; the birch's values are fn-213's.

## Friction

- `cargo fmt` and `rustfmt <file>` rewrite untouched files -> proposed a rustfmt-clean tree, or a fmt-changed script with `skip_children`.
- No way to print a preset's trunk radius at a seed (an extra render to correct a guessed range) -> proposed a small example printing root and breast-height radius.
- Shell guard blocks variable-path redirects -> local setup, reported only.
