# fn-197 step 5: retained pipes and thickening by leaves, 2026-10-05

## What was built

- **`retained`** (per PA, a share in 0 to 1, neutral 0): Shinozaki's disused pipes.
  - Before shedding, `girth::grown_radii` gives every grown axis its base radius as `geometry::scale` and `thicken` would size it, sizes included.
  - After shedding, each shed branch whose bearer was kept leaves `retained` × its section, raised to the bearer's exponent, at the node where it stood (`girth::Disused`). `thicken` carries it down like any other pipe.
  - Dormant where no PA retains: nothing is computed.
- **`leaf_girth`** χ (per PA, neutral 0): a phytomer's own pipe is scaled by light^χ over the tree's pipe-weighted mean of light^χ.
  - The tree's own pipe is conserved, and limbs whose leaves catch more light thicken.
  - Each unit's light is stored on its phytomers (`Phytomer::light`) only where some PA has χ > 0.
- **Tests:**
  - `girth/tests.rs`: on a trunk with a lit limb (0.9) and a shaded one (0.2), χ 1 keeps the tree's own pipe to 1e-12, thickens the lit limb and thins the shaded one. A retained pipe thickens its bearer by degree as its share runs 0 to 1 in 100 steps.
  - **Walks** (`light_changes_the_tree_by_degree`): `retained` slope 0.31 and `leaf_girth` 1.25, walked with the limbs sagging, since girth moves the shape measure only through sag. The whole light walk is green.
- **Neutral:**
  - `tests/light.rs` passes.
  - The 80-year hashes of the beech, the spruce and the oak at seeds 1 and 7 are unchanged against the pre-fn-197 base (`measures hash 80 1,7`).
  - `cargo test --profile ci -p telperion-space` is green (152 s).
- **Render flag:** `space_oak --girth <retained>,<χ>`.

## Renders (oak, 80 years, step 4c's settings plus retained 0.3, χ 1, under the GPU lock; I viewed every still)

Sheets in `raw/step5/`:

- `explore.png`: seed 1 at retained 0.3 / χ 0, retained 0 / χ 1 and both, beside step 4c, S1 and S3.
- `seeds-bare.png`, `seeds-whole.png`: the references, round 4, step 4c and step 5 for seeds 1, 7, 2, 3 and 4.
- `close-ups.png`: step 5's trunk bases and limbs beside S3.

| Seed | Step 4c: height, width (m), fine km | Step 5: height, width (m), fine km, leaves |
|--:|---|---|
| 1 | 21.7, 31.3 × 26.4, 9.33 | 22.5, 28.5 × 24.0, 9.58, 1.30M |
| 7 | 20.8, 24.8 × 26.0, 9.80 | 22.4, 22.5 × 22.8, 10.09, 1.37M |
| 2 | 21.3, 21.0 × 30.6, 14.44 | 21.5, 20.8 × 26.1, 14.58, 1.96M |
| 3 | 19.5, 26.4 × 24.8, 11.26 | 23.5, 24.1 × 21.7, 11.62, 1.58M |
| 4 | 18.8, 26.6 × 24.5, 6.90 | 21.6, 22.8 × 23.9, 7.36, 0.38M |

### Reading

- **Heavier wood.** Retained 0.3 is the visible term: the trunks are thicker and flare at the base, and the main limbs read heavier and darker in the bare crowns. χ 1 alone barely shows on seed 1.
- **The domes hold.** In leaf, seeds 1, 7, 2 and 3 are still rounded, closed crowns, and seed 4 a spreading one.
- **The crowns are 2 to 4 m narrower and 1 to 4 m taller** than step 4c's. Heavier limbs are stiffer against sag, so they spread less and stand higher. Nothing regresses that the sheet shows, but the domes are a little less broad.
- **Against S1 and S3: heavier, not yet massive.**
  - The trunk bases are thicker but read as smooth columns with a flare. The collar band is round 4's known gap.
  - They are not S3's massive, buttressed trunk dividing into a few huge limbs.
  - A larger `retained` or the limbs' own pipe exponent would push further. It is not tried here, as the host asked for a modest value.
- **The bole:** a short trunk before the crown, as S1 shows, unchanged from step 4c.
