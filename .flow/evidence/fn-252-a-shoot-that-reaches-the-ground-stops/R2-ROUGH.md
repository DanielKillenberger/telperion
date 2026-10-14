# fn-252 R2's open point: does growth's rough lay see the ground? (worker, 2026-10-14)

`rough.rs` grows each tree twice:
- `telperion_space::grow` gives the final lay.
- `telperion_space::sketch` gives the same tree with every phytomer where the rough layout grown with it put it. That layout is `grow/draw.rs`, which runs `Layer::step` with no sag load.

How it compares them:
- **Matching:** axes are paired by lineage, age and birth, since the two paths can keep different axes. No resting axis lacked a twin.
- **Resting:** a non-stem node at z < 1 mm.
- **"Within one internode":** the axis's lowest rough node is no higher than its own age's internode.

The raw log is `raw/rough.log`, seeds 1 and 7. The ordinary preset is lit (extinction 0.5), so growth lays it roughly, as it does the birch.

| Tree | Age | Axes resting (final) | In the rough lay: touch / within one internode / held above | Held above: lowest rough node p10/p50/p90/max (m) |
|---|--:|--:|---|---|
| birch | 10 | 71 / 80 | 80-86% / 3-4% / 10-17% | 0.01-0.02 / 0.05-0.07 / 0.08-0.09 / 0.09-0.14 |
| birch | 20 | 485 / 441 | 49-59% / 0-1% / 40-51% | 0.10 / 0.67-0.70 / 1.57-1.70 / 2.22-2.57 |
| birch | 40 | 70 / 13 | 59% and 0% / 0% / 41% and 100% | 0.15-0.21 / 0.38-1.17 / 0.82-2.51 / 1.01-3.20 |
| birch | 80 | 0 / 99 | seed 7: 0% / 0% / 100% | 5.17 / 8.48 / 9.89 / 11.31 |
| ordinary | 40 | 0 / 0 | | |
| ordinary | 80 | 278 / 154 | 0% / 0% / 100% | 0.27-0.49 / 0.40-0.79 / 0.52-1.29 / 1.86-2.64 |

The rough lay also rests far fewer nodes than the final lay:

| Tree | Age | Resting nodes, final | Resting nodes, rough |
|---|--:|---|---|
| birch | 10 | 433 / 499 | 370 / 444 |
| birch | 20 | 4,491 / 3,946 | 2,477 / 2,486 |
| birch | 40 | 797 / 153 | 499 / 0 |
| birch | 80 | 0 / 593 | 0 / 0 |
| ordinary | 80 | 1,955 / 1,278 | 0 / 0 |

## Reading

- **The young birch rests in growth's own lay.**
  - At 10, 83-90% of the resting axes already touch the ground in the rough lay, or come within one internode of it. `Layer::step`'s resting clamp runs in the rough lay too, because it is the same function, so growth's layout holds those strands on the ground as they grow.
  - The rest are held 1-14 cm up.
- **By 20, about half reach the ground only by sag**, held 0.7 m up at the median and up to 2.6 m.
- **At 40, it is half or all.**
- **At 80, and in the ordinary tree at 80, it is all.**
  - The rough lay holds the birch's resting wood 5-11 m up and the ordinary tree's 0.3-2.6 m up.
  - The contact there comes from the old limbs and branches bending under their load in the settle (`sag`). The strands' own sag barely moves them.
- **The birch strands' hang is in the rough lay, by tropism.**
  - Strand headings have a median vertical component of −0.70 to −0.95 in the rough lay, against −0.83 to −0.98 in the final lay.
  - A strand's drop from base to lowest node has a median of 0.15-0.82 m rough, against 0.27-0.91 m final.
  - The set-point (−π/2, tropism 3) does the hanging, and the strands' `sag` (2e-4) adds about 0.1 m at the median.
  - The large moves from rough to final are the bearers' sag: whole limbs and branches lowered, carrying their strands down.
