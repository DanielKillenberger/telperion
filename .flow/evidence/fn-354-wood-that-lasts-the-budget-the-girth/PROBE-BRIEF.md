# fn-354 R4: the probe's brief, by the host, 11 Nov 2026

The rule and its reasons are `.flow/evidence/fn-351-020-the-release-candidate/PHASE2-DESIGN.md`. This file fixes what the design leaves open, for the probe only. Every value marked "probe estimate" is a stand-in so the probe can run; none is a preset value, and none lands.

## Where

A branch `fn-354-probe` cut from `fn-354-wood-that-lasts`'s head, in this worktree or its own. It is never merged. Commit the probe there as it grows; the product branch stays as task 1 left it.

## The rule, exactly

Per phytomer, two numbers carried from year to year: `T` (the section's whole area, m²) and `S` (the sapwood held, m²).

After each growth cycle, one pass from the tips down gives every held phytomer `A`, the leaf area above it this year: its own leaf, the leaf of its axis beyond it, and the leaf of every child at or beyond its node (a lateral at its node, a continuation or relay at the tip). Then:

    need = A / (R × 10⁴)                 R in m² of leaf per cm² of sapwood
    kept = S × (1 − 1/N)
    laid = max(0, need − kept)
    T   += laid
    S    = kept + laid

A phytomer laid this cycle starts with `T = π × established²` (today's established radius, unchanged) and `S = 0` before the pass.

At the settle, `radius = sqrt(T / π)`. `thicken`'s pipe sum, `exponent`, `ripening`, `secondary`, `retained`, `leaf_girth` and the disused pipes are not read. Relay joins (`Join::below`, `above`) are computed from these radii.

**Leaf, for the probe.** The leaf a phytomer bears is what the grower deposits into the light lattice for it (`leaf_area × scale × size`, `grow/draw.rs:224-229`), with the scale as it stands in that year. It counts while the phytomer is younger than the species' leaf life: one year for the oak, beech and birch; for the spruce, 6 years (probe estimate, from "half a cohort shed at 5.5 years", Muukkonen & Lehtonen 2004). Short-shoot clusters are not added. Record beside it the drawn leaf area the scoreboard already measures, so the two accounts can be compared per tree.

**A fading branch** is drawn at its section times its kept share, as its scale fades today; its bearer's `T` is untouched. **A retired subtree** keeps the radius its base had when it closed.

**Values.**

| Species | R, m² per cm² | N, rings |
|---|---|---|
| Oak | 0.30 (middle of Phillips et al. 2003's 0.23 and 0.37) | 20 (Hollstein's 20.4 at 100 to 200 years, European oak) |
| Beech | 0.331 (Bartelink 1997) | 50 (probe estimate, middle of Gebauer et al. 2008's 25 to 73) |
| Spruce | 0.36 (McDowell et al. 2002) | 30 (probe estimate, no source) |
| Birch | 0.331 (probe estimate, the beech's) | 40 (probe estimate, no source) |

Where they live in a probe is yours (constants keyed by preset are fine in a probe and nowhere else).

## Sag

Two settings, chosen at run time:

- **today's:** every row as the presets have it;
- **elastic:** every growth stage of a species takes `4ρg / E`: oak 5.0e-6, beech 4.1e-6, birch 3.7e-6, spruce 6.1e-6 per metre.

## Variants

- **B:** this branch as it stands (the budget fix, today's girth, today's sag).
- **G1:** the rule, today's sag.
- **G2:** the rule, elastic sag.

The scoreboard's `base-2026-11-11` is the 11 Nov build and is not redrawn.

## What to draw and measure

Stage 1, then stop and return:

- **Sheets,** seeds 1001, 1007, 1012: the oak at 40, 80, 120, 200, 300, whole view leafed and leafless; the beech, birch and spruce at 40, whole and mid. One sheet per species and view with columns base, B, G1, G2 and rows by age and seed, a label under every tile (variant, age, seed, height, spread, trunk at 1.3 m), a refused tree as a labelled empty tile with its error.
- **Close views,** oak seed 1001 at 120 and 300, variants B, G1, G2: the base of the trunk, and the junction of the largest limb with the trunk.
- **Measures** on those trees: the scoreboard's, plus the base diameter of the three largest limbs and the chord angle of the largest limb from vertical.
- **Yearly trace,** per seed, for the oak under G2 and for each other species: year, leaf above 1.3 m (the rule's `A` there), `T` and `S` at 1.3 m, height.
- **Cost,** one thread, seed 1001, the oak at 40, 120 and 300: seconds and peak memory for B and for G2, and the yearly pass's own share.

Use and extend the scoreboard's runner (`.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/scoreboard/`) on the integration branch's copy only by adding options (a panel subset, a variant label); it is an instrument and stays able to rerun the base.

Stage 2 comes on the host's word: twelve seeds of measures for the variant the host picks.

## What to return

`PROBE-RESULT.md`: the tables; the oak's trunk at 1.3 m per seed and age beside White's three open classes (28 to 40, 56 to 79, 83 to 109, 121 to 152, 156 to 192 cm at 40, 80, 120, 200, 300); a plain description per tree of what is drawn; what in the rule could not be built as written and what you did instead. No tree is called better or worse; the host views every sheet.

Stop and return early if: the rule needs a decision this brief does not make; a species cannot be drawn under a variant (return the error); the yearly pass more than doubles the grow time (return the profile).
