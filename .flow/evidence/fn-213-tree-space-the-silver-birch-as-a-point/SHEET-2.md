# fn-213 sheet 2: round 2 beside the references (worker, 2026-10-07)

Round 2 aims at the host's points (1), (2), (4) and (5)'s lean by values (SHEET-1.md, "The host's reading"), plus decision (c) by the existing material rows. The values are in MODEL-BIRCH.md, "Round 2". The worker viewed every still named here; nothing below is a verdict.

## Decision (a): the merge and the parity test

`fn-198-tree-space-f-the-engine-in-production` (at `2fd97b73`: pinned math, F2b, faster tests) was merged first; the catalogue digests were re-pinned. `harness/parity.test.ts` then passes 8 of 8, the birch included: the 169-node native/Wasm gap of sheet 1 is gone with the pinned math. Re-run after round 2's values: see the gate below.

## The stills

Same layout as sheet 1, at the round-2 commit, in the ignored `raw/sheet2/`:

| File | What |
|---|---|
| `raw/sheet2/sheet-40.png` | References S1, S3-crown, S2-Oslo, S2-Uusimaa; then age 40, seeds 1, 7, 42: hero bare, hero in leaf, crown (in leaf), limb close-up (bare) |
| `raw/sheet2/sheet-80.png` | The same at age 80 |
| `raw/sheet2/bases.png` | Trunk bases: 40 seed 1; 80 seeds 1, 7, 42 |
| `raw/sheet2/birch-<age>-<seed>-<shot>.png`, `run.log` | Every still; sizes and times |

The references and their licences are SHEET-1.md's.

## The worker's reading, point by point

1. **Density.** The crown is open: sky shows between the limbs bare and in leaf at both ages, a third of sheet 1's wood. At 80 bare, S1's lattice of ascending limbs with fine haze reads. Seed 7's upper crown at 80 is still the densest.
2. **Limbs at 80.** Main limbs now ascend steeply from the stem and the strands hang from them (limb close-ups); no flat white spokes remain.
3. **Fine wood.** Wood under about 3 cm in radius is dark red-brown and grey; the stem and thick limb bases are white. The bare crowns read dark against the sky as S1's and S3's do.
4. **Leaves.** A bright yellow-green, warmer than S2-Oslo's, about S2-Uusimaa's hue; perhaps a shade too pale in full sun.
5. **Lean.** The stem curves at every seed and leans at 42 (40) and 7 (80), as S1 and S2-Oslo's do.

Still short of the references: the crown at 40 is narrower and thinner than S2-Oslo's (it is near S2-Uusimaa's); the 80-year in-leaf crown reads as fine strands rather than S2-Oslo's leafy mass; no stem forks (S2-Oslo), as the host decided (d) leaves it.

## Decision (c): bark by girth with the existing rows

- **White by girth, dark fine wood:** expressible. `/material/shootRadius` (wood under it takes the shoot colour, blending to the bark by twice it) is colour by radius; at 0.03 m it draws point 3.
- **Dark old base:** expressible in part. The relief's maturity ramps with radius over ridge width (`bark.wgsl`: `smoothstep(2, 5, 2r / ridgeScale)`), and the crest and fissure tints and the cavity follow the relief. With relief from 16 cm and a dark crest tint, the 40-year stem stays white and the 80-year flare darkens with dark fissures (`bases.png`); a ridge width of 12 cm darkened marks up the whole lower stem without making the base black. What the rows cannot draw is a base whose own colour turns black with girth: the bark colour is one row for all thick wood, and only the relief's tints vary with radius.
- **Options for the host** (not decided here): (i) accept the tint-based darkening; (ii) a capability spec: a mature-bark colour row blended in by radius (as `shootRadius` blends young wood), dormant at zero; (iii) the same blended by height above the ground, if the host reads the old birch's black base as a height trait rather than a girth one.

## Gate

After round 2's values, `cargo test --profile ci --workspace --no-fail-fast` passed every target but `preset_values`: the leaf-station row stated 0.02 m, the family's default, which the value writer drops. The row is now left at the default with a comment (the family is unchanged); `preset_values`, `catalogue_identity` and `space_presets` then pass. Sheet 1's one base failure (`generation_limit_guard` on fn-198's shoulder sites) is gone with the merge. `harness/parity.test.ts` passes 8 of 8 on round 2's values.
