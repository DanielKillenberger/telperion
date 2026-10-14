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

## Host rulings (host, 2026-10-14)

After R2 (`.flow/evidence/fn-277-a-walk-between-leaf-forms-changes-the/CAUSE.md`):

1. **Fractional counts.** `rosetteFronds`, `leafletCount`, `skirtFronds`, `leafBases`, `acanthophylls` and the engine's `cluster_leaves` walk unrounded. Wherever a count sets how many organs a placement, axis or apex bears, the fraction is split by a keyed draw (`leaf_years`' pattern, fn-248), so the expected count moves with the row. Whole-valued presets draw byte-equal to today.
2. **The rosette takeover.** A new continuous row, the share of leaf-bearing wood that keeps its leaves (named botanically; every shipped dicot and conifer 1, the palm 0), applied by a keyed draw per leaf-bearing unit, replaces "any rosette frond makes the rosette the tree's only foliage". The rosette's presence is its frond count alone. The palm's 0: a palm bears its leaves only in the crown (cited as the palm's existing rows cite, or FAO 156). Default 1, dormant where absent.
3. **Leaflets: one law, no form switch.** A leaf with a rachis bears its leaflets wherever it grows, so mid-walk the oak's wood leaves become compound by degree. `leafletCount` walks on the log scale (fn-251 R1b: log for magnitudes). Before building further, the peak instance count along oak to palm at seeds 1, 7, 42 under 1-3 together is measured against the instance budget and reported to the host; if any frame exceeds it, stop and report the frames and numbers.
4. **The R3 oracle** is the ratio of leaf counts between consecutive frames, its bound measured on the walks that pass and stated, dormant where both frames bear none of that form.
