---
satisfies: [R1, R2, R3]
---
# fn-228-the-harness-renders-white-as-white.1 Implement the harness renders white as white

## Description
TBD

## Acceptance
Every R-ID in the parent spec's requirements is satisfied; judge this task against the spec's criteria directly.

## Done summary
The default sun is re-tinted from (3.0, 2.85, 2.6) to (3.078, 2.848, 2.388) at the same luminance, so full daylight (sun plus sky) on a neutral floor renders neutral, white and grey both, within 3%. Shade stays cool, and the drawn sky and the sky light are unchanged (host decision, 2026-10-08; spec R2 amended). The R1 measure, the R3 stills and the birch ratios are in `.flow/evidence/fn-228-the-harness-renders-white-as-white/MEASURE.md`.

- **R1:** `tests/neutral_light.rs` renders a neutral floor from above, in sun and in the tree's shadow. The shade's blue is the sky's colour; in sun, the warm sun and the blue sky offset each other; the tone map adds nothing.
- **R2:** `neutral_light` asserts the sunlit bar on the GPU. `row.rs`'s `the_default_daylight_is_white_at_the_luminance_it_had` checks the balance and the luminance without a GPU.
- **R3, stills:** oak, spruce, beech and palm before and after, viewed. Silver-birch, telperion and laurelin are refused on this branch ("no engine species").
- **R3, birch:** re-measured on fn-213's `b64872a2` with only the sun defaults changed. The stem's b:g goes from 0.97-0.99 to 0.93-0.94, against its rows' 0.92. The leaves go from 0.57 to 0.54 up close and from 0.65 to 0.63 in the crown, against 0.46; the rest is the cool shade inside the crown.
- **R3, browser:** agreement with the headless stills is inferred from code. The page reads `SceneRow::default()` through `WebRenderer::scene()`, and both draw with the same WGSL. No browser frame was measured.
- **Follow-up:** `telperion-jev/src/tuning/matched.rs` still writes the old sun (3.0/2.85/2.6) into its own scene.

baseline: none (the spec defines no Quick commands)
stage: impl-review - ran (codex, one correctness draw): SHIP

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5714dd474d2e053623507a450f5f44df98b26009, dd3ac28500d2ef724b9f6066a31595c04402b7ee, 3fb1341df3bafdda5bf705414ba6b40045e0e269
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock, exit 0), cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render (exit 0), npm test (13 files, 143 tests)
- PRs: