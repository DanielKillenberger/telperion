# Bark that matures with girth

## Goal & Context

The silver birch (fn-213) fails Astra's trait (b) on sheet 3: an old birch's lower trunk is dark and fissured, while the material rows give one bark colour to all thick wood, so the 80-year base reads 'predominantly white with black speckling rather than a convincing dark, fissured lower trunk'. The host deferred it as its own capability (fn-213 SHEET-1.md decision c, 2026-10-07). Many species' bark changes with girth (the oak's young smooth bark against its old ridged bark; the birch's white against its black base).

## Requirements

- **R1:** A mature-bark colour and fissure depth, blended in by radius (a stated radius range) the way `/material/shootRadius` blends young wood, with no effect at zero (every species without it byte-identical in look).
- **R2:** The birch's 80-year base dark and fissured, judged on stills beside its reference photographs; the 40-year stem stays white.
- **R3:** Every passed species' stills unchanged where the rows are absent; npm test and the gate green.

## Boundaries

Renderer material and its preset rows only; no engine change. Depends on fn-198.