# fn-203 result

## What is built

- **The lever turns with the bends before it.** `sag::levers` keeps, on the unbent tree, each phytomer's moment (the mass it carries times its centre's offset from the far end) and the chord from that end to its axis's tip. When the tree is laid again, base to tip, both are turned rigidly by the rotation from the phytomer's unbent frame to its frame now.
- **Each phytomer's turn is integrated exactly** (`sag::turn`). The load's fall from hanging closes as dfall/ds = −c·min(reach·sin(fall), cap), where c = sag / r⁴, so the turn cannot overshoot hanging. The 0.15 rad stop short of straight down stays as a backstop.

## Host decision 1 (2026-10-05): the correction only ever shrinks the lever

- The moment is the smaller of the turned lever's and the unbent lever's, in the turned direction.
- This is continuous: a minimum of continuous functions. It never feeds back to grow the lever, so upright trunks do not buckle. Buckling is out of scope, because this pass models no reaction wood or tropic response.
- **Result:** fn-200's walks pass over their full range, including the trunk's `form.sag` up to 2e-4, which toppled before this decision. Every other crate test passes, except the landing-corner test below.

## Host decision 2 (2026-10-05): the ground carries the load

- Wood past the point where it rests on the ground adds no lever to the wood before it, and its direction stays on the ground.
- **Built, three parts:**
  1. On the unbent tree, the load a phytomer passes to the wood before it is weighted by how far it stands free of the ground (`geometry::support`, the landing zone's ease).
  2. When the tree is laid again, the turned chord shows where the wood beyond would go below the ground. The ground carries that share, and the lever and its cap shrink by the square of the share left free (`sag::held`).
  3. The sag turn itself fades by the same ease as wood reaches the ground.
- The beech has no sag, so it is unaffected; the landing ease's arithmetic is unchanged.

## Host decision 3 (2026-10-05): R1 reworded

- **(i)** `a_heavy_branch_converges_towards_hanging`: a 3.5 m branch on a 10 m pole, at sag 2e-3. Where it hangs within 0.3 rad of straight down, its last turn per phytomer is below 1e-3 rad at three seeds. The old sag also passes; that is recorded.
- **(ii)** `a_heavy_low_bough_on_the_ground_does_not_swing_round`: round 9's fault in miniature, a bough 0.4 m up at the spruce's branch values (sag 6e-4, tropism 0.8 to elevation 0.7, wander 1.0). The level-going internodes' winding must stay below 1.2 rad.
  - Red on the old sag: it winds 1.61 rad at seed 1, and 1.5 to 1.6 rad at every seed.
  - Green now: 0.02 to 0.17 rad. The bough comes within 0.007 m of the ground.
- **R2:** `a_light_branch_bends_as_the_small_deflection_beam_does`. Doubling a light sag doubles every internode's bend to within 0.00005 rad (bend up to 0.036 rad).

## R4: the spruce at round 9 (`raw/r4b/sheet.png`, viewed)

- **Loops:** none. Boughs sweep down from the trunk and spread out radially along the ground. Nothing wraps round the base.
- **Nothing toppled.** Height 21.9 m. Crowns are 17.2 × 19.1 m at seed 1 and 16.7 × 14.4 m at seed 4.
- **The hook is lost,** as expected (host decision 4). Round 10 retunes.
- `raw/r4/sheet.png` is the earlier render, with the turned lever only: boughs hung steeply beside the trunk.

## Host decision 4, revised (2026-10-05): one landing bound, 0.8 rad, for every sag

**Why one bound.** Under the bent-lever sag, any wood that reaches the ground arrives near vertical and bends where it lands, as a heavy rope reaching a floor does. A light-sag bound of 0.35 measured a regime that no longer exists. The numeric bound guards against regressions; the stills judge the look. Measured on the walk tree's limbs:

| Sag | New: limbs landing | New: sharpest | Old (fn-200): limbs landing | Old: sharpest |
|---|---|---|---|---|
| 3e-4 | 1 | 0.21 | 0 | — |
| 6e-4 (the spruce's) | 4 | 0.72 | 0 | — |
| 1e-3 | 4 | 0.43 | 4 | 0.18 |
| 2e-3 | 14 | 0.49 | 1 | 0.40 |
| 1e-2 | 19 | 0.56 | 0 | — |
| 5.0 | 18 | 0.52 | 6 | 0.13 |

**Built.**

- `wood_lands_on_the_ground_without_a_corner`: at sag 6e-4, 1e-3 and 2e-3, at least 4 limbs land and every bend is under 0.8 rad. The worst is 0.717 at 6e-4.
- `heavy_wood_meets_the_ground_with_a_bounded_bend`: at sag 5.0, 18 limbs land, sharpest 0.517 rad, under its 0.6 bound.

**The spruce's base close-ups** (`raw/r4b/spruce-80-1-base.png`, `spruce-80-4-base.png`, viewed full size):

- **On the ground:** no visible kinks. Boughs come down in smooth arcs and run out along the ground.
- **At the trunk:** a few of the lowest boughs on seed 1 make a rounded knee just out from the trunk, where they drop. That is the drop under load, not a corner.

## R3

- The landing walk now runs limb sag up to 6e-4, the spruce's heaviest value (it was 3e-4). It holds the bound of 30 and its jump check.
- All 63 crate tests pass.
- The beech stills are byte-identical (`raw/beech`).
