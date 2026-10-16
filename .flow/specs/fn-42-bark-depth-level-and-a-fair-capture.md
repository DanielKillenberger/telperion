# Bark depth, level and a fair capture

## Goal

fn-32 gave the bark a plate network that the owner judged "much much better. but still has much room for improvement", and the owner chose to carry the remaining gaps here. Three gaps had been measured in fn-32's round three: the oak trunk too light and cool (crop mean 140 144 133 against the reference's 118 119 114, because the level lives in fn-29's base colour row), furrows reading as crackle rather than valleys (relief capped at 4 mm by the anti-aliasing contract), and a dark fraction dominated by the trunk cylinder rather than bark. This spec added a bark-only capture at a known scale, reworked depth and relief under fn-71's one continuous footprint filter, and moved the oak and spruce colour rows, judged by numbers first and the owner's eye last.

## Decisions

- > user (turn 1, on fn-32's round-two stills): "it's much much better. but still has much room for improvement. Not sure if in this spec or not though"
- > user (turn 2, on carrying the remaining gaps to a new spec): "ok do it"
- > user (2026-09-19, on the performance budget): "i think we'll have to find performance improvements all over the place anyway so i think i'd allow extra cost for now and optimize later?" -> R4 requires comparable measurements and disclosure, not a ceiling.
- Reference decision (2026-09-19): the original photographs could not be recovered and replacements lacked physical scale and calibrated RGB; the owner chose option 1, "Continue with visual reference matching". Replacement sources: MONGO's Norway spruce bark detail (2816x2112, flash-lit, unknown width; Syrio's 4000x3000 close-up as secondary) and SelecTree's labelled oak bark (650x567, limits fine-detail judgment); originals in the local `.refs` cache, never redistributed.
- First candidate (2026-09-19): the owner said "colors look better" but expected added relief.
- Owner visual verdict (2026-09-20) on the stronger-relief candidate: "i feel the relief highlights more clearly the structural difference between the reference and our implementation. The small scales with fine lines are quite different to our deep and more wide valleys in the relief. There's also a lack of roughness around the edges." -> not accepted; redirect to shallower plate faces, narrow separations and irregular chipped edges. Continuation authorised: "ok so pls $flow-next-flow this to the end".
- The thin chipped profile is an explicit authored row, `plateEdgeShape` (0 rounded, 1 revised, blended), not the smooth-term optimisation flag, which would have changed shape when lichen or lenticels are on.
- Colour refinement (2026-09-20): the owner called the revised shape "much better" and asked for red/brown layered colour in this spec; directional overlapping flakes go to a separate spec (fn-90).
- Owner acceptance (2026-09-20): "hm forogt the birch materials but that's probably just tuning from here? if you confirm that i can attest the distance render looks good i'm happy to merge this (squash merge)". The host confirmed birch's controls were preserved; R5 resolved and squash merge authorised.

## Acceptance Criteria

- **R1:** Bark-only oak and spruce recipe, square-on, physical crop width, lighting and camera recorded; structure metrics as render diagnostics; photo-scale limits stated — met.
- **R2:** Scale and furrow relief matches the owner's structural feedback (flatter faces, finer separations, chipped edges) under one continuous footprint filter; all distance, resolution, close-up sweep and redraw tests at unchanged bounds — met, with the inherited historical hero reconstruction as an owner-accepted exception (it failed its absolute 0.03 gate before and after; no tolerance changed).
- **R3:** Oak and spruce colour and relief readability improved with restrained red/brown variation via existing rows; before/after render colour metrics — met (mean RGB shift oak +3.66/-0.59/-2.47, spruce +1.21/-0.32/-1.41).
- **R4:** Fresh baseline and candidate timings by the fn-26 protocol; browser orbit reported — met (cost disclosed, see Result).
- **R5:** Owner judges bark-only stills and re-rendered fn-32 stills beside references — met (accepted 2026-09-20).

## Result

- Parallax corrected; continuous shallow chipped scale profiles with `plateEdgeShape`; restrained warm recess colours on oak and spruce fissure rows; other species unchanged. A runtime-uniform GPU probe checks profile interpolation and specialisation parity at 1e-6.
- Timing (RTX 3080, 1600x1000, seed-7 oak): native whole-tree total p50 8.9231 -> 14.5405 ms (p95 14.9348); full-screen trunk vegetation 14.8695 -> 38.4031 ms (p95 38.8321); browser orbit 66.6 fps average, wall p50 10.1 ms, p95 30.0 ms. The regression is retained for later optimisation by owner decision.
- Landed as PR #47, squash-merged as `a63802a2` after all nine checks passed; QA SHIP.

## Open

Directional shingling and overlapping flakes (fn-90); birch material tuning; a reproducible evidence recipe (complete camera, dimensions, seed, hashes, mask and invocation; proposed for fn-78); a bark reference catalogue with useful resolution and scale.

## Friction

Twenty entries, all reviewed at close (dispositions; none promoted to a spec without the owner):
- Stale rendering design -> re-anchor older specs against later accepted contracts; resolved here.
- Missing original photographs; capture build cold; GPU timing contention; slow debug grazing mask; concurrent GPU probe crash; colour-metrics Python interpreter -> local setup, not specced.
- Capture radius contract; lighting orientation -> explicit-radius fixture and recorded patch normal, camera and light.
- Reference search stopped early; host stopped at worker checkpoints -> keep the objective across rejected candidates and checkpoints.
- Oak reference resolution -> possible evidence-tooling spec for original-resolution references.
- Incomplete historical hero recipe and mask -> proposed line in fn-78.
- Candidate distance regression; stronger relief aliasing -> existing gates caught them; keep the gates.
- Optimisation flag chose shape -> fixed by `plateEdgeShape` with parity coverage.
- Live QA locator; merge preparation -> inspect control roles first; `docs/pr-format.md` supersedes the old grouping proposal.

## Evidence

- `.flow/evidence/fn-42-bark-depth-level-and-a-fair-capture/colour-final/{oak-flat,spruce-flat}.jpg` (final bark-only verdict stills), `colour-final/calibration.json` (capture recipe), `colour-final/comparison.json` (R3 metrics)
- `.flow/evidence/fn-42-bark-depth-level-and-a-fair-capture/shape-final/{oak-trunk,oak-branch,spruce-trunk,spruce-branch}.jpg` (re-rendered fn-32 stills)
- `.flow/evidence/fn-42-bark-depth-level-and-a-fair-capture/spruce-reference-candidates.json` (reference sources and hashes)
