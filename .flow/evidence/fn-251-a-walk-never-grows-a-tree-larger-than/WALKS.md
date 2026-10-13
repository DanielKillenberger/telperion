# fn-251 walks: realized size along a walk, linear against natural-scale rows

Measured 2026-10-09 on cd4d380d with `crates/telperion-core/examples/walk_size.rs` (`--profile ci`, 4 threads). Each frame is grown by the build's executor (`grow_stepped`, cut 1024), beside both ends grown at the frame's seed, light and age, as the oracle asks. Raw lines are in `raw/` (ignored).

Cells are realized kept / grown phytomers in millions. An arrow marks a count outside the oracle's band, the ends' range widened by 10 %: ↑ above it, ↓ below it. "Factor on" is `blend::families`; "factor off" is the same frame with the engine's own walk in place of the sized species. `(f x)` is the size factor where it is below 1. The natural-scale rows are `natural-scale.patch`: probabilities (continuation, viability, abortion, zone lateral and dormant) on log-odds shifted by 1e-3, node counts and buds on log above 1, readiness on log above 0.01, the relay's share weighted by each end's stops as before. The patch is not applied on the branch.

## R1: the teaser's frame

The frame at t = 0.07504803866484144 (oak to birch, 80, seed 7) grows 20,021,484 phytomers against the 20M budget; `the_oak_to_birch_frame_the_teaser_refused_builds` reproduces it (red, 26 s). The size factor there is 0.996: the light-neutral prediction it solves on (7.28M kept, 34.8M grown) is near the ends' mean, so it barely moves. The cause is the light, not the solve:

- The ends' light-neutral predictions (oak 8.04M kept, 37.3M grown) are not what they grow. Lit at the frame's extinction the oak keeps 3.52M and grows 17.2M; the birch, which carries no light rows, keeps 3.37M and grows 6.40M at any light.
- The walk mixes the extinction linearly from the oak's 0.5 to the birch's 0, and the oak's light-response rows towards the birch's zeros. A frame is lit less and responds less than the oak, so it grows towards its light-neutral size, which the factor allows.
- The oak alone outgrows the budget under the frame's light from t = 0.5 on (22.6M grown at extinction 0.25, 30.6M at 0.1). Under the oracle's own definition the end it compares against is over budget, so R2's "meets the oracle" and "never fails on the budget" cannot both hold on this walk while the light walks linearly.
- The spec's "both ends far below the budget (2.19M, 2.93M)" counts drawn nodes; in phytomers the oak at 80, seed 7 grows 16.7M to 17.2M.

| walk | age | t | ends kept (M) | ends grown (M) | linear, factor on | linear, factor off | natural, factor on | natural, factor off |
|---|---|---|---|---|---|---|---|---|
| oak → birch | 80 | 0.01 | 3.41, 3.37 | 16.74, 6.40 | 4.17↑ / 18.90↑ (f 0.999) | 4.16↑ / 18.92↑ | 4.09↑ / 18.25 | 4.09↑ / 18.25 |
| oak → birch | 80 | 0.0750 | 3.52, 3.37 | 17.20, 6.40 | 4.36↑ / 20.02↑ (f 0.996) | 4.39↑ / 20.15↑ | 3.07 / 13.06 | 3.07 / 13.06 |
| oak → birch | 80 | 0.2 | 3.71, 3.37 | 18.32, 6.40 | 5.37↑ / 23.47↑ | 5.37↑ / 23.47↑ | 1.63↓ / 6.75 | 1.63↓ / 6.75 |
| oak → birch | 80 | 0.5 | 4.66, 3.37 | 22.61, 6.40 | 6.17↑ / 28.47↑ | 6.17↑ / 28.47↑ | 1.37↓ / 6.68 | 1.37↓ / 6.68 |
| oak → birch | 80 | 0.8 | 6.54, 3.37 | 30.64, 6.40 | 4.59 / 23.94 (f 0.942) | 6.23 / 32.07 | 2.22↓ / 11.62 | 2.22↓ / 11.62 |

## The fn-230 strips (seed 1) and fn-250's age walk (oak 5 to 80, seed 7)

| walk | age | t | ends kept (M) | ends grown (M) | linear, factor on | linear, factor off | natural, factor on | natural, factor off |
|---|---|---|---|---|---|---|---|---|
| oak → oak | 24 | 0.25 | 0.13, 0.13 | 0.24, 0.24 | 0.13 / 0.24 | 0.13 / 0.24 | 0.13 / 0.24 | 0.13 / 0.24 |
| beech → oak | 40 | 0.25 | 0.11, 0.47 | 0.12, 1.30 | 0.18 / 0.39 | 0.18 / 0.39 | 0.06↓ / 0.11↓ | 0.06↓ / 0.11↓ |
| beech → oak | 40 | 0.5 | 0.11, 0.42 | 0.12, 1.20 | 0.20 / 0.43 | 0.20 / 0.43 | 0.03↓ / 0.06↓ | 0.03↓ / 0.06↓ |
| beech → oak | 40 | 0.75 | 0.11, 0.39 | 0.12, 1.12 | 0.27 / 0.70 | 0.27 / 0.70 | 0.06↓ / 0.12 | 0.06↓ / 0.12 |
| beech → ordinary | 40 | 0.25 | 0.11, 0.10 | 0.12, 0.24 | 0.10 / 0.24 (f 0.962) | 0.15↑ / 0.32↑ | 0.07↓ / 0.15 | 0.07↓ / 0.15 |
| beech → ordinary | 40 | 0.5 | 0.11, 0.10 | 0.12, 0.24 | 0.12 / 0.26 (f 0.966) | 0.15↑ / 0.30↑ | 0.05↓ / 0.09↓ | 0.05↓ / 0.09↓ |
| beech → ordinary | 40 | 0.75 | 0.11, 0.10 | 0.12, 0.24 | 0.10 / 0.21 (f 0.978) | 0.10 / 0.22 | 0.03↓ / 0.06↓ | 0.03↓ / 0.06↓ |
| beech → spruce | 40 | 0.25 | 0.11, 1.18 | 0.12, 1.23 | 0.30 / 0.35 (f 0.987) | 0.31 / 0.37 | 0.00↓ / 0.00↓ | 0.00↓ / 0.00↓ |
| beech → spruce | 40 | 0.5 | 0.11, 1.18 | 0.12, 1.23 | 0.38 / 0.52 (f 0.862) | 0.62 / 0.82 | 0.00↓ / 0.00↓ | 0.00↓ / 0.00↓ |
| beech → spruce | 40 | 0.75 | 0.11, 1.18 | 0.12, 1.23 | 0.75 / 0.98 (f 0.771) | 1.88↑ / 2.46↑ | 0.06↓ / 0.08↓ | 0.06↓ / 0.08↓ |
| oak → ordinary | 40 | 0.25 | 0.38, 0.10 | 1.09, 0.24 | 0.27 / 0.67 | 0.27 / 0.67 | 0.20 / 0.50 | 0.20 / 0.50 |
| oak → ordinary | 40 | 0.5 | 0.40, 0.10 | 1.14, 0.24 | 0.16 / 0.37 | 0.16 / 0.37 | 0.08↓ / 0.16↓ | 0.08↓ / 0.16↓ |
| oak → ordinary | 40 | 0.75 | 0.42, 0.10 | 1.20, 0.24 | 0.08↓ / 0.21↓ | 0.08↓ / 0.21↓ | 0.03↓ / 0.07↓ | 0.03↓ / 0.07↓ |
| oak → spruce | 40 | 0.25 | 0.39, 1.18 | 1.12, 1.23 | 0.39 / 0.85↓ | 0.39 / 0.85↓ | 0.02↓ / 0.05↓ | 0.02↓ / 0.05↓ |
| oak → spruce | 40 | 0.5 | 0.42, 1.18 | 1.20, 1.23 | 0.38 / 1.01↓ | 0.38 / 1.01↓ | 0.02↓ / 0.04↓ | 0.02↓ / 0.04↓ |
| oak → spruce | 40 | 0.75 | 0.47, 1.18 | 1.30, 1.23 | 0.91 / 2.10↑ | 0.91 / 2.10↑ | 0.23↓ / 0.46↓ | 0.23↓ / 0.46↓ |
| spruce → ordinary | 40 | 0.25 | 1.18, 0.10 | 1.23, 0.24 | 0.69 / 1.49↑ (f 0.830) | 1.17 / 2.65↑ | 0.66 / 1.55↑ | 0.66 / 1.55↑ |
| spruce → ordinary | 40 | 0.5 | 1.18, 0.10 | 1.23, 0.24 | 0.39 / 0.91 (f 0.839) | 0.69 / 1.73↑ | 0.27 / 0.69 | 0.27 / 0.69 |
| spruce → ordinary | 40 | 0.75 | 1.18, 0.10 | 1.23, 0.24 | 0.20 / 0.47 (f 0.885) | 0.31 / 0.75 | 0.12 / 0.30 | 0.12 / 0.30 |
| oak → oak | 43 | 0.5 | 0.60, 0.60 | 1.72, 1.72 | 0.60 / 1.72 | 0.60 / 1.72 | 0.60 / 1.72 | 0.60 / 1.72 |
| oak → oak | 61 | 0.75 | 1.79, 1.79 | 6.32, 6.32 | 1.79 / 6.32 | 1.79 / 6.32 | 1.79 / 6.32 | 1.79 / 6.32 |
| oak → oak | 76 | 0.95 | 3.09, 3.09 | 13.98, 13.98 | 3.09 / 13.98 | 3.09 / 13.98 | 3.09 / 13.98 | 3.09 / 13.98 |
| beech → oak | 80 | 0.25 | 1.61, 2.72 | 1.93, 14.39 | 2.63 / 9.61 (f 0.987) | 2.81 / 10.30 | 0.46↓ / 1.78 | 0.46↓ / 1.78 |
| beech → oak | 80 | 0.5 | 1.61, 2.36 | 1.93, 12.60 | 2.02 / 7.66 | 2.02 / 7.66 | 0.18↓ / 0.73↓ | 0.18↓ / 0.73↓ |
| beech → oak | 80 | 0.75 | 1.61, 2.10 | 1.93, 11.21 | 2.22 / 9.63 | 2.22 / 9.63 | 0.30↓ / 1.23↓ | 0.30↓ / 1.23↓ |
| beech → ordinary | 80 | 0.25 | 1.61, 0.77 | 1.93, 2.69 | 1.17 / 4.44↑ (f 0.921) | 2.23↑ / 8.67↑ | 1.15 / 4.19↑ | 1.15 / 4.19↑ |
| beech → ordinary | 80 | 0.5 | 1.61, 0.77 | 1.93, 2.68 | 1.37 / 4.58↑ (f 0.900) | 2.45↑ / 8.05↑ | 0.89 / 2.60 | 0.89 / 2.60 |
| beech → ordinary | 80 | 0.75 | 1.61, 0.77 | 1.93, 2.68 | 0.84 / 2.60 (f 0.924) | 1.38 / 4.21↑ | 0.36↓ / 0.92↓ | 0.36↓ / 0.92↓ |
| beech → spruce | 80 | 0.25 | 1.61, 4.78 | 1.93, 5.40 | 2.37 / 3.03 (f 0.920) | 3.95 / 4.93 | 0.03↓ / 0.04↓ | 0.03↓ / 0.04↓ |
| beech → spruce | 80 | 0.5 | 1.61, 4.78 | 1.93, 5.40 | 2.98 / 4.40 (f 0.903) | 4.86 / 7.02↑ | 0.01↓ / 0.01↓ | 0.01↓ / 0.01↓ |
| beech → spruce | 80 | 0.75 | 1.61, 4.78 | 1.93, 5.40 | 3.53 / 5.57 (f 0.830) | 7.88↑ / 11.97↑ | 0.16↓ / 0.24↓ | 0.16↓ / 0.24↓ |
| oak → ordinary | 80 | 0.25 | 2.03, 0.77 | 10.83, 2.68 | 1.57 / 7.01 | 1.57 / 7.01 | 1.07 / 4.64 | 1.07 / 4.64 |
| oak → ordinary | 80 | 0.5 | 2.18, 0.77 | 11.65, 2.68 | 1.18 / 4.39 | 1.18 / 4.39 | 0.68↓ / 2.03↓ | 0.68↓ / 2.03↓ |
| oak → ordinary | 80 | 0.75 | 2.36, 0.77 | 12.60, 2.68 | 0.71 / 2.44 | 0.71 / 2.44 | 0.35↓ / 0.94↓ | 0.35↓ / 0.94↓ |
| oak → spruce | 80 | 0.25 | 2.10, 4.78 | 11.21, 5.40 | 3.28 / 12.43↑ | 3.28 / 12.43↑ | 0.07↓ / 0.21↓ | 0.07↓ / 0.21↓ |
| oak → spruce | 80 | 0.5 | 2.36, 4.78 | 12.60, 5.40 | 1.73↓ / 7.95 | 1.73↓ / 7.95 | 0.05↓ / 0.14↓ | 0.05↓ / 0.14↓ |
| oak → spruce | 80 | 0.75 | 2.72, 4.78 | 14.39, 5.40 | 2.67 / 13.01 | 2.67 / 13.01 | 0.43↓ / 2.10↓ | 0.43↓ / 2.10↓ |
| spruce → ordinary | 80 | 0.25 | 4.78, 0.77 | 5.40, 2.69 | 2.58 / 12.05↑ (f 0.893) | 3.83 / 18.13↑ | 1.79 / 8.95↑ | 1.79 / 8.95↑ |
| spruce → ordinary | 80 | 0.5 | 4.78, 0.77 | 5.40, 2.68 | 1.50 / 7.92↑ (f 0.917) | 2.13 / 11.30↑ | 0.61↓ / 3.57 | 0.61↓ / 3.57 |
| spruce → ordinary | 80 | 0.75 | 4.78, 0.77 | 5.40, 2.68 | 1.08 / 4.79 (f 0.948) | 1.39 / 6.10↑ | 0.32↓ / 1.68↓ | 0.32↓ / 1.68↓ |

## Tally (45 frames, the strips, the age walk and the repro)

| Rows | Oracle met, factor on | Oracle met, factor off | Over the 20M budget |
|---|---|---|---|
| linear | 30 | 22 | 4 (oak to birch, t ≥ 0.075) |
| natural | 10 | 10 | 0 |

## Reading

- Natural-scale rows alone do not meet the oracle. They remove every overrun on the high side but undershoot nearly everywhere: beech to spruce at 0.5 keeps 0.01M at 80 against ends of 1.61M and 4.78M. Log-odds pull a probability towards its smaller end, and the trees compound it over decades. With the factor on or off they are the same tree: the factor never binds on natural rows.
- Linear rows with the factor meet the oracle in 30 of 45 frames; they fail high on the oak to birch (the light, above), oak to spruce, and the grown count of the walks to the ordinary tree at 80 (the factor bounds kept and grown on light-neutral predictions, which shed differently than the realized trees), and low on oak to spruce and oak to ordinary at 40.
- The age walk (one species across ages) is the species grown at each frame's age in every variant, factor 1: `size::sized` already predicts the ends at the frame's age (`count.of_end(end, age)` with the frame's age). fn-250's claim did not reproduce on this base.
- The ordinary preset's rows were written by `blend::centroid` with the factor (`examples/centroid.rs`); deleting the factor changes what that example writes, not the shipped values file.

Not decided here (host): whether the floor and the row set of the natural scale are the design, whether the light walks with the rows or the ends are compared at their own light, and whether the factor stays. The sweep R3 asks (15 pairs, 2 ages, 3 seeds, fine t) costs hours of CPU a variant at this measurement's 4 grows a frame (FRICTION.md).

# Re-measured on F4 after fn-255 (2026-10-14)

Measured on af2d4ba8a, `--profile ci`, 6 threads, with `examples/walk_size.rs` ported to the one cycle (`grow_cycle`, whose `grown` counts every phytomer grown, the shed ones included). The oracle is as the spec writes it: the ends grown at the frame's seed, light and age; each realized count between the smaller and larger end, widened by 10 %. Every preset now reads the one site light (`presets/space.rs`, extinction 0.5), so the frame's light is the ends' light on every walk. Frames are R1b's 45 plus the release film's pairs (fn-272 R1a) at seed 7: beech → birch → spruce → oak → palm at 40, palm → Laurelin as seedlings (ages 1 and 3), Laurelin → Telperion at 80. Raw lines are in `raw/all.log`, `raw/probe.log`, `raw/probe2.log` (ignored).

## R1: the teaser's frame builds

`the_oak_to_birch_frame_the_teaser_refused_builds` passes in 18 s (it failed on the 20M budget on cd4d380d). At t = 0.07504803866484144 the frame keeps 3.26M and grows 15.99M phytomers against ends of 3.39M / 16.63M (oak) and 1.78M / 10.63M (birch), within the oracle. The size factor there is 0.996. No frame of the 80 or of the 37 probes failed on the budget; the largest grown count was 18.70M (oak → birch, t = 0.001), 6.5 % under it.

## The frames

Cells are millions of phytomers, `ph` whole phytomers. An arrow marks a realized count outside the oracle's band. `f` is the size factor where below 1. The last column is the light-neutral prediction the size factor solves on.

| walk | age | seed | t | ends kept (M) | ends grown (M) | frame kept / grown (M) | frame predicted kept / grown (M) |
|---|---|---|---|---|---|---|---|
| oak → birch | 80 | 7 | 0.01 | 3.39, 1.78 | 16.63, 10.63 | 3.95↑ / 18.18 (f 0.999) | 7.94 / 36.92 |
| oak → birch | 80 | 7 | 0.0750 | 3.39, 1.78 | 16.63, 10.63 | 3.26 / 15.99 (f 0.996) | 7.28 / 34.76 |
| oak → birch | 80 | 7 | 0.2 | 3.39, 1.78 | 16.63, 10.63 | 2.99 / 14.45 | 6.02 / 29.75 |
| oak → birch | 80 | 7 | 0.5 | 3.39, 1.78 | 16.63, 10.63 | 3.09 / 14.97 | 4.09 / 21.18 |
| oak → birch | 80 | 7 | 0.8 | 3.39, 1.78 | 16.63, 10.63 | 2.51 / 13.24 (f 0.942) | 3.12 / 17.02 |
| beech → oak | 40 | 1 | 0.25 | 0.13, 0.36 | 0.31, 1.05 | 0.17 / 0.40 | 0.26 / 0.60 |
| beech → oak | 40 | 1 | 0.5 | 0.13, 0.36 | 0.31, 1.05 | 0.18 / 0.41 | 0.30 / 0.72 |
| beech → oak | 40 | 1 | 0.75 | 0.13, 0.36 | 0.31, 1.05 | 0.24 / 0.66 | 0.45 / 1.15 |
| beech → ordinary | 40 | 1 | 0.25 | 0.13, 0.10 | 0.31, 0.24 | 0.12 / 0.27 (f 0.966) | 0.17 / 0.38 |
| beech → ordinary | 40 | 1 | 0.5 | 0.13, 0.10 | 0.31, 0.24 | 0.13 / 0.28 (f 0.973) | 0.15 / 0.35 |
| beech → ordinary | 40 | 1 | 0.75 | 0.13, 0.10 | 0.31, 0.24 | 0.10 / 0.22 (f 0.984) | 0.14 / 0.33 |
| beech → spruce | 40 | 1 | 0.25 | 0.13, 1.08 | 0.31, 2.68 | 0.31 / 0.73 (f 0.988) | 0.27 / 0.64 |
| beech → spruce | 40 | 1 | 0.5 | 0.13, 1.08 | 0.31, 2.68 | 0.41 / 1.01 (f 0.869) | 0.39 / 1.05 |
| beech → spruce | 40 | 1 | 0.75 | 0.13, 1.08 | 0.31, 2.68 | 0.79 / 1.71 (f 0.782) | 0.72 / 1.66 |
| oak → ordinary | 40 | 1 | 0.25 | 0.36, 0.10 | 1.05, 0.24 | 0.26 / 0.67 | 0.37 / 0.91 |
| oak → ordinary | 40 | 1 | 0.5 | 0.36, 0.10 | 1.05, 0.24 | 0.16 / 0.36 | 0.19 / 0.46 |
| oak → ordinary | 40 | 1 | 0.75 | 0.36, 0.10 | 1.05, 0.24 | 0.08↓ / 0.21↓ | 0.14 / 0.34 |
| oak → spruce | 40 | 1 | 0.25 | 0.36, 1.08 | 1.05, 2.68 | 0.37 / 0.81↓ | 0.33 / 0.78 |
| oak → spruce | 40 | 1 | 0.5 | 0.36, 1.08 | 1.05, 2.68 | 0.36 / 0.97 | 0.36 / 1.05 |
| oak → spruce | 40 | 1 | 0.75 | 0.36, 1.08 | 1.05, 2.68 | 0.86 / 2.00 | 0.76 / 1.93 |
| spruce → ordinary | 40 | 1 | 0.25 | 1.08, 0.10 | 2.68, 0.24 | 0.68 / 1.47 (f 0.830) | 0.67 / 1.51 |
| spruce → ordinary | 40 | 1 | 0.5 | 1.08, 0.10 | 2.68, 0.24 | 0.39 / 0.91 (f 0.839) | 0.37 / 0.94 |
| spruce → ordinary | 40 | 1 | 0.75 | 1.08, 0.10 | 2.68, 0.24 | 0.20 / 0.47 (f 0.885) | 0.20 / 0.55 |
| oak → oak | 24 | 7 | 0.25 | 0.13, 0.13 | 0.24, 0.24 | 0.13 / 0.24 | 0.16 / 0.30 |
| oak → oak | 43 | 7 | 0.5 | 0.60, 0.60 | 1.72, 1.72 | 0.60 / 1.72 | 1.24 / 3.13 |
| oak → oak | 61 | 7 | 0.75 | 1.79, 1.79 | 6.32, 6.32 | 1.79 / 6.32 | 3.71 / 13.07 |
| oak → oak | 76 | 7 | 0.95 | 3.09, 3.09 | 13.98, 13.98 | 3.09 / 13.98 | 6.96 / 30.65 |
| beech → birch | 40 | 7 | 0.05 | 0.22, 0.46 | 0.43, 1.28 | 0.28 / 0.51 (f 0.998) | 0.19 / 0.42 |
| beech → birch | 40 | 7 | 0.25 | 0.22, 0.46 | 0.43, 1.28 | 0.27 / 0.53 | 0.21 / 0.49 |
| beech → birch | 40 | 7 | 0.5 | 0.22, 0.46 | 0.43, 1.28 | 0.32 / 0.71 | 0.27 / 0.63 |
| beech → birch | 40 | 7 | 0.75 | 0.22, 0.46 | 0.43, 1.28 | 0.45 / 1.01 | 0.41 / 0.98 |
| beech → birch | 40 | 7 | 0.95 | 0.22, 0.46 | 0.43, 1.28 | 0.37 / 0.97 (f 0.997) | 0.53 / 1.27 |
| birch → spruce | 40 | 7 | 0.05 | 0.46, 1.05 | 1.28, 2.62 | 0.54 / 1.31 (f 0.986) | 0.58 / 1.40 |
| birch → spruce | 40 | 7 | 0.25 | 0.46, 1.05 | 1.28, 2.62 | 0.52 / 1.37 | 0.52 / 1.38 |
| birch → spruce | 40 | 7 | 0.5 | 0.46, 1.05 | 1.28, 2.62 | 0.52 / 1.53 | 0.53 / 1.62 |
| birch → spruce | 40 | 7 | 0.75 | 0.46, 1.05 | 1.28, 2.62 | 0.89 / 1.93 (f 0.877) | 0.96 / 2.25 |
| birch → spruce | 40 | 7 | 0.95 | 0.46, 1.05 | 1.28, 2.62 | 1.13 / 2.52 (f 0.886) | 1.11 / 2.54 |
| spruce → oak | 40 | 7 | 0.05 | 1.05, 0.49 | 2.62, 1.34 | 1.17↑ / 2.57 | 1.11 / 2.66 |
| spruce → oak | 40 | 7 | 0.25 | 1.05, 0.49 | 2.62, 1.34 | 0.74 / 1.75 | 0.76 / 1.93 |
| spruce → oak | 40 | 7 | 0.5 | 1.05, 0.49 | 2.62, 1.34 | 0.35↓ / 0.91↓ | 0.36 / 1.05 |
| spruce → oak | 40 | 7 | 0.75 | 1.05, 0.49 | 2.62, 1.34 | 0.26↓ / 0.60↓ | 0.33 / 0.78 |
| spruce → oak | 40 | 7 | 0.95 | 1.05, 0.49 | 2.62, 1.34 | 0.48 / 1.15↓ | 0.75 / 1.77 |
| oak → palm | 40 | 7 | 0.05 | 0.49, 480 ph | 1.34, 480 ph | 0.17 / 0.48 | 0.29 / 0.76 |
| oak → palm | 40 | 7 | 0.25 | 0.49, 480 ph | 1.34, 480 ph | 5862 ph / 0.02 | 3413 ph / 0.02 |
| oak → palm | 40 | 7 | 0.5 | 0.49, 480 ph | 1.34, 480 ph | 676 ph / 1152 ph | 610 ph / 1139 ph |
| oak → palm | 40 | 7 | 0.75 | 0.49, 480 ph | 1.34, 480 ph | 440 ph / 440 ph | 455 ph / 471 ph |
| oak → palm | 40 | 7 | 0.95 | 0.49, 480 ph | 1.34, 480 ph | 474 ph / 474 ph | 473 ph / 473 ph |
| palm → Laurelin | 1 | 7 | 0.05 | 12 ph, 7 ph | 12 ph, 7 ph | 11 ph / 11 ph | 12 ph / 12 ph |
| palm → Laurelin | 1 | 7 | 0.25 | 12 ph, 7 ph | 12 ph, 7 ph | 10 ph / 10 ph | 11 ph / 11 ph |
| palm → Laurelin | 1 | 7 | 0.5 | 12 ph, 7 ph | 12 ph, 7 ph | 9 ph / 9 ph | 10 ph / 10 ph |
| palm → Laurelin | 1 | 7 | 0.75 | 12 ph, 7 ph | 12 ph, 7 ph | 8 ph / 8 ph | 8 ph / 8 ph |
| palm → Laurelin | 1 | 7 | 0.95 | 12 ph, 7 ph | 12 ph, 7 ph | 7 ph / 7 ph | 7 ph / 7 ph |
| palm → Laurelin | 3 | 7 | 0.05 | 36 ph, 80 ph | 36 ph, 80 ph | 35 ph / 35 ph | 35 ph / 35 ph |
| palm → Laurelin | 3 | 7 | 0.25 | 36 ph, 80 ph | 36 ph, 80 ph | 31 ph↓ / 31 ph↓ | 33 ph / 33 ph |
| palm → Laurelin | 3 | 7 | 0.5 | 36 ph, 80 ph | 36 ph, 80 ph | 63 ph / 63 ph | 35 ph / 35 ph |
| palm → Laurelin | 3 | 7 | 0.75 | 36 ph, 80 ph | 36 ph, 80 ph | 56 ph / 56 ph | 45 ph / 45 ph |
| palm → Laurelin | 3 | 7 | 0.95 | 36 ph, 80 ph | 36 ph, 80 ph | 79 ph / 79 ph | 62 ph / 62 ph |
| beech → oak | 80 | 1 | 0.25 | 1.70, 1.88 | 8.24, 10.06 | 2.01 / 8.53 (f 0.990) | 3.60 / 14.28 |
| beech → oak | 80 | 1 | 0.5 | 1.70, 1.88 | 8.24, 10.06 | 1.30↓ / 5.77↓ | 3.61 / 13.95 |
| beech → oak | 80 | 1 | 0.75 | 1.70, 1.88 | 8.24, 10.06 | 1.61 / 7.70 | 4.15 / 17.80 |
| beech → ordinary | 80 | 1 | 0.25 | 1.70, 0.77 | 8.24, 2.68 | 1.38 / 5.10 (f 0.926) | 2.22 / 8.43 |
| beech → ordinary | 80 | 1 | 0.5 | 1.70, 0.77 | 8.24, 2.68 | 1.46 / 5.03 (f 0.907) | 1.79 / 5.97 |
| beech → ordinary | 80 | 1 | 0.75 | 1.70, 0.77 | 8.24, 2.68 | 0.86 / 2.72 (f 0.930) | 1.45 / 4.57 |
| beech → spruce | 80 | 1 | 0.25 | 1.70, 3.69 | 8.24, 17.27 | 2.12 / 9.28 (f 0.931) | 3.14 / 11.59 |
| beech → spruce | 80 | 1 | 0.5 | 1.70, 3.69 | 8.24, 17.27 | 2.66 / 12.56 (f 0.911) | 3.41 / 14.79 |
| beech → spruce | 80 | 1 | 0.75 | 1.70, 3.69 | 8.24, 17.27 | 3.24 / 15.03 (f 0.838) | 3.82 / 16.69 |
| oak → ordinary | 80 | 1 | 0.25 | 1.88, 0.77 | 10.06, 2.68 | 1.53 / 6.82 | 3.52 / 14.84 |
| oak → ordinary | 80 | 1 | 0.5 | 1.88, 0.77 | 10.06, 2.68 | 1.15 / 4.30 | 2.03 / 7.48 |
| oak → ordinary | 80 | 1 | 0.75 | 1.88, 0.77 | 10.06, 2.68 | 0.70 / 2.42 | 1.50 / 5.08 |
| oak → spruce | 80 | 1 | 0.25 | 1.88, 3.69 | 10.06, 17.27 | 2.22 / 9.95 | 3.96 / 15.17 |
| oak → spruce | 80 | 1 | 0.5 | 1.88, 3.69 | 10.06, 17.27 | 1.51↓ / 7.38↓ | 2.37 / 10.68 |
| oak → spruce | 80 | 1 | 0.75 | 1.88, 3.69 | 10.06, 17.27 | 2.51 / 12.35 | 2.51 / 12.85 |
| spruce → ordinary | 80 | 1 | 0.25 | 3.69, 0.77 | 17.27, 2.68 | 2.51 / 11.81 (f 0.893) | 2.80 / 12.86 |
| spruce → ordinary | 80 | 1 | 0.5 | 3.69, 0.77 | 17.27, 2.68 | 1.49 / 7.85 (f 0.917) | 1.77 / 8.77 |
| spruce → ordinary | 80 | 1 | 0.75 | 3.69, 0.77 | 17.27, 2.68 | 1.06 / 4.74 (f 0.948) | 1.29 / 5.98 |
| Laurelin → Telperion | 80 | 7 | 0.05 | 2.47, 2.47 | 12.42, 12.42 | 2.47 / 12.42 | 2.75 / 11.60 |
| Laurelin → Telperion | 80 | 7 | 0.25 | 2.47, 2.47 | 12.42, 12.42 | 2.47 / 12.42 | 2.75 / 11.60 |
| Laurelin → Telperion | 80 | 7 | 0.5 | 2.47, 2.47 | 12.42, 12.42 | 2.47 / 12.42 | 2.75 / 11.60 |
| Laurelin → Telperion | 80 | 7 | 0.75 | 2.47, 2.47 | 12.42, 12.42 | 2.47 / 12.42 | 2.75 / 11.60 |
| Laurelin → Telperion | 80 | 7 | 0.95 | 2.47, 2.47 | 12.42, 12.42 | 2.47 / 12.42 | 2.75 / 11.60 |

## Tally

| frames | met | failed |
|---|---|---|
| R1b's 45 (the repro walk, the strips, the age walk) | 40 (30 before fn-255) | 5 |
| the film's 35 | 30 | 5: spruce → oak at 40 (4), palm → Laurelin at 3 (1) |
| all 80 | 70 | 10, of which 0 on the budget |

The age walk and Laurelin → Telperion are the same tree at every frame (Laurelin and Telperion grow alike at seed 7).

## Why the 10 fail

Three causes, each checked against measurement.

1. **The walk's own prediction lies below both ends (5 frames: oak ↔ spruce at 40 and 80, t = 0.25 to 0.75 from the oak).** The light-neutral prediction at spruce → oak, 40, t = 0.75 is 0.33M kept against the ends' 1.15M and 0.97M; the realized trees follow it. The size factor is shrink-only by design (`blend/size.rs`), so it stands at 1 and cannot lift a frame the linear rows make smaller than both ends. Seed-independent: seeds 1 and 42 at t = 0.75 also fail (`raw/probe.log`).
2. **One realization's kept count moves more than the 10 % widening (4 frames: oak → birch at 80, t = 0.01; spruce → oak at 40, t = 0.05; beech → oak at 80, t = 0.5; oak → ordinary at 40, t = 0.75).** Their predictions lie within the ends. The walk is continuous at its ends: at t = 1e-9 the frame is the end's tree exactly (`raw/probe2.log`). But a row change of 1e-6 moves the kept count by 10 % at one seed while the grown count barely moves (beech → birch at 40: kept 0.220M → 0.242M, grown 431,806 → 431,871; spruce → oak: kept 1.05M → 1.21M), and the oak at 80 keeps 3.39M, 1.88M and 1.47M at seeds 7, 1 and 42. Kept is what light leaves standing, so every species now carries the light-driven spread fn-251's first measurement saw on the oak alone; one seed's end is a narrower reference than that spread.
3. **Whole phytomers at a seedling (1 frame: palm → Laurelin at 3, t = 0.25).** 31 against a floor of 32.4 (ends 36 and 80).

The cause is not light walking away from the ends any more: the frame's light equals the ends' on every walk.
