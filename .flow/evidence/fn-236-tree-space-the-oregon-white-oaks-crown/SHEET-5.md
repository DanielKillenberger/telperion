# fn-236: the typical blade and round 5 (2026-10-14)

Host rulings after round 4 are recorded in the spec. **Round 5 stops the rounds: it refuses at 80, seed 42.** Rounds 6 and 7 were probed and not committed.

## The typical blade (`5ae542cd2`)

No source read gives a measured mean length for Q. garryana. The blade is set to the middle of FNA's "25-120(-140)" mm, 72.5 mm (`/canopy/size` 0.725).

On round 4's structure, leaf over hull lands at **2.2 / 2.8 / 1.9** at 40, seeds 1 / 7 / 42. That is below the sources' 3.2 to 4.4.

## Round 5: the trunk lives four years (`0fada95b6`)

O-WHOLE's trunk, measured from the photograph (`photo_crown.py`'s mask, the trunk's dark run by row), widens into its first scaffold limbs at 0.10 to 0.19 of the tree's height. SILV and OWIC give open-grown oaks "short boles". The trunk (PA 2) used to live 12 years and fork at 4.6 to 6.7 m, 0.44 to 0.6 of the height.

| Age, seed | Nodes | Height, m | Width, m | Width / height | Gaps | Leaf over hull |
|---|---|---|---|---|---|---|
| 40, 1 | 529,141 (280,037) | 9.1 | 10.9 | 1.20 | 0.381 | 3.0 |
| 40, 7 | 494,358 (371,251) | 10.0 | 9.7 | 0.97 | 0.336 | 3.5 |
| 40, 42 | 561,088 (284,355) | 10.8 | 10.8 | 1.00 | 0.398 | 3.3 |
| 80, 1 | 2,083,688 (1,405,560) | 20.5 | 23.3 | 1.13 | 0.421 | |
| 80, 42 | refused: `engine phytomer budget` | | | | | |

The nodes in brackets are round 4's. Seed 7 at 80 is left out (the host's spec).

- Leaf over hull lands inside 3.2 to 4.4 at 40.
- Width over height moves toward O-WHOLE's 1.31.
- Gaps are 0.34 to 0.42, against round 3's 0.20 to 0.26 at 40 and 0.14 to 0.16 at 80.
- At 80 the tree is over MODEL-OAK's range (height 14 to 24 m, crown 10 to 20 m wide). Seed 1 grows in 24.8 s, against 8.5 s for round 4.

**Stills, all viewed** (`raw/compare/compare5.png`: O-WHOLE, round 4, round 5).

- At 40, seed 1 is broad and low on a short trunk, the nearest any round has come to O-WHOLE's outline. Seeds 7 and 42 stay upright.
- At 80, seed 1 is a bushy, see-through crown with foliage to the ground and no clear trunk. Round 4's 80-year still, with its clear trunk and broad top, reads closer to O-WHOLE.

## Probes for rounds 6 and 7, at 40 on round 5 (not committed)

| Change | Width / height | Gaps | Nodes |
|---|---|---|---|
| Limb `form/elevation` 1.05 to 0.8 | 1.00 to 1.21 | 0.354 to 0.371 | +4 % |
| Limb `form/elevation` 1.05 to 0.6 | 1.05 to 1.23 | 0.331 to 0.351 | +8 % |
| Limb `insertion` 0.5 to 0.9 | 1.09 to 1.26 | 0.331 to 0.356 | +8 % |
| Leader lifespan 6 to 2 | 0.97 to 1.22 | 0.372 to 0.443 | -14 % |

- No structural row brings gaps near round 3's.
- Every probe that widens the crown also adds nodes.
