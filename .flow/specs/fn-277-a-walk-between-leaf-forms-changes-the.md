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

## Host rulings after the first review (host, 2026-10-14)

5. **The oracle tests the law and the draw apart** (host, 2026-10-14, replacing the per-bearer wording). The defect class is a switch in the law. (a) *The law is continuous:* each form's expected count per bearer, computed from the walked rows alone, unrounded (fronds per apex by leaflets per frond, leaves per station by the cauline share, cluster leaves per tip, and so on), changes by at most the measured bound per step of 0.001 in t, swept over every film walk at seeds 1, 7 and 42; the bound is stated from the walks that pass. (b) *The draw follows the law:* at the film's frames each form's drawn count per bearer lies within the sampling noise of its expectation (binomial on the fractional parts, 4 sigma), dormant where a form has fewer than 30 organs in the frame; wood is measured by station, not by metre. Reason: the per-bearer count oracle refused oak to palm (crowns of 3 to 25 leaves, whole fronds; leaves per metre on sub-internode runs) and palm to Laurelin at the film's frame spacing (a steady 1.2 to 1.4 a frame of the log-walked leaflets, 1.005 per step of 0.001) (FILM-WALKS.md). The t 0.25-0.52 wood steps stay with fn-273.
6. **The device draws the same law.** One law on both executors, and the harness keeps every family it drew before: the keyed splits of fractional counts and the cauline share in WGSL, and the order of mixed wood and frond leaves, with the CPU/device agreement test on the default birch with `rosetteFronds` 12, a fractional-count family (`leafletCount` 1.3 on the oak) and oak to palm at t 0.01, 0.3 and 0.9. No CPU placement route for fractional values. Reason: the first review's finding that the birch with 12 fronds, drawn on the device before fn-277, was refused after it.

