# A blend between species solves in about a second

## Goal & Context

The owner's aim for the harness: "looking at trees in the harness and adjusting params should yield near instant results." A walk frame between species (`telperion_core::blend`, fn-230's size factor) solves before it grows, and the solve is slow at mature ages: fn-230 measured up to 87 s at 80 for blends near the ordinary tree, and fn-231's R5 table (`.flow/evidence/fn-231-the-engines-walk-is-continuous-at-its/ENDS.md`) has spruce to ordinary at 0.5 and 80 taking 76.0 s and beech to spruce at 0.5 and 80 taking 4.9 s, with one prediction costing what it did before. fn-231 found the slow frames are the ones whose size moved, because the solve now has to match the costly grown count as well as the kept one. Host (2026-10-09): a release item, its own spec, as fn-231's R5 deferral named.

## Requirements

- **R1 (measure first):** where the solve's time goes (how many predictions a solve makes, the cost of each, kept against grown), on the slow frames fn-230 and fn-231 named, at 40 and 80.
- **R2 (fix):** the solve made cheap by the cause R1 names (for example a cheaper grown count, fewer predictions through a better first guess or bracketing, or reusing the frame's predictions), stated before code; the solved factor equal to today's within the solve's own tolerance.
- **R3 (proved):** every walk frame of fn-230's strips at 40, and at 80 every frame without the ordinary tree as an end, solves in at most 1 s natively on one core (best of 3, load stated), against today's times; frames at 80 with the ordinary tree as an end measured and reported (its single kept prediction alone is 0.88 s; the faster kept predictor is its own post-release spec); the walk digest re-pinned only with every factor's move inside the solve's tolerance and the walk strips viewed unchanged; the shipped species byte-identical (amended 2026-10-09, host).
- **R4:** gate and npm test green.

## Boundaries

The blend solve only. Depends on fn-231.
