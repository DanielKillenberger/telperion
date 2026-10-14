# A walk between leaf forms changes the leaves by degree

## Goal & Context

fn-272's film (FILM.md, defects 2 and 5, 2026-10-14): walking from the Oregon white oak to the date palm at age 40, the leaves go at once: 871,168 leaves at t = 0.0098, 4 at t = 0.0127 (frames 486-487), and at t = 0.08 a bare oak-shaped crown stands on a palm stem. Leaf counts also step 25-38 % between consecutive frames of spruce to birch (t = 0.16) and birch to oak (t = 0.83). Every parameter must change the tree by degree, never switch between ways of building (AGENTS.md, design principles); a jump of five orders of magnitude in one frame is a switch. Which row or rule flips (a leaf-form row read as a threshold, a rosette or canopy placement chosen by a rounded value, a count rounding) is unknown until checked.

## Requirements

- **R1 (repro first):** the oak to palm leaf count across t in [0.009, 0.013] at fine steps, standalone, at seeds 1, 7, 42; the same for the two steps above; each as a test that fails today.
- **R2 (cause):** the row or rule behind each jump, checked against code, written down before any fix.
- **R3 (by degree):** the leaves change continuously along every walk: an oracle on leaf count between consecutive frames (its bound measured on the walks that pass today and stated), dormant where a species has no leaves of that form. No species branch.
- **R4:** the film's walk frames and fn-251's walk tests pass with it; gate, npm, review; stills of oak to palm at t = 0, 0.01, 0.05, 0.2, 0.5, 1 viewed.

## Boundaries

Release item (fn-237) because the film walks through it (fn-272 R3). Walk sizes from either side stay fn-273.
