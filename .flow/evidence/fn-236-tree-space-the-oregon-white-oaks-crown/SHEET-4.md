# fn-236 round 4: leaves on the current year's shoots (2026-10-14)

This round sets the rows the host ruled on after `DEFECTS.md`. They are in the preset and mirrored in `oak.rs`. **Neither acceptance is met, so the round stops here for the host.**

## The rows and their sources

- **`leafYears` 1 on every age.** FNA, Quercus garryana: "Trees or shrubs, deciduous".
- **`shootRadius` 0.007.** FNA: "Twigs ... 2-4 mm diam.", so the bearing limit is a 2 mm radius. Over the 40-year root's 0.29 m, that is 0.007, inside the dial's [0.005, 0.01]. Because the row is a share of the root's radius, the limit grows with the root: 4.2 mm at 80.
- **Twig `internodeLength` 0.008, unchanged.** One leaf a node (Go Botany, Quercus alba: "one leaf per node along the stem", `raw/lit/gobotany-quercus-alba.html`). On the current year's engine nodes the spacing measures:
  - 7.48 / 7.51 / 8.85 mm at age 40, seeds 1 / 7 / 42;
  - 7.84 / 8.06 mm at age 80, seeds 1 / 42.

## Measures

The acceptance is a leaf area index of 3.2 to 3.3 (Phillips et al. 2003, groves) or a mean of 4.4 (Karlik & McKay 2002, open-grown Q. douglasii).

| Age, seed | Leaves | Leaf over hull | Gaps, round 4 | Gaps, round 3 |
|---|---|---|---|---|
| 40, 1 | 47,248 | 8.4 | 0.378 | 0.256 |
| 40, 7 | 70,247 | 10.4 | 0.308 | 0.202 |
| 40, 42 | 47,921 | 7.2 | 0.421 | 0.248 |
| 80, 1 | 254,764 | 9.7 | 0.311 | 0.158 |
| 80, 42 | 214,119 | 8.8 | 0.302 | 0.143 |

- Every leaf now stands on wood of age 0.
- The leaf area index is 2 to 3 times the sources'. The drawn leaf averages 96 cm², at `/canopy/size` 1.4, FNA's largest blade. The host has not ruled on that row.
- Gaps roughly double at 80.
- At 80, seed 7 refuses with `invalid input: unsolved radii` on round 3's rows as well, so the refusal predates this round.

## Stills

`raw/sheet4/`, and `raw/compare/compare4-40.png` beside round 3 and O-WHOLE; all viewed.

- The comb along the limbs is gone, and the leaves stand in tufts at the shoot tips.
- At 40 the crown opens: limbs show through it, and seed 42's lower crown is sparse. It no longer reads as a dense dome.
- At 80, seed 1 still reads as a broad, rounded crown from the hero view, but its gaps double.
