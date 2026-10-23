# The person in the sun's map: what it costs the tree's shadow (fn-310 task 9)

The host ruled on task 9 that the shadow cost be measured before any further change. The quantity is the ground that one texel of the sun's map covers, with the person in the map's fit and without it. The bound is 10 %. The film breaks it. The catalogue stills do not.

## Method

The map is the renderer's own fit (`shadow::light`, 1,024 texels), given either the tree's bounds alone or the tree's bounds and the person's box. Each case reports the ground per texel along both map axes. Across the sun, the value is taken as it is. Up the sun's view, it is divided by the sine of the sun's elevation. The ratio is the larger of the two axes' with/without ratios, so it catches a refit of either axis. The person stands where the renderer stands it:

- For a still, at `figure_placement` under that sun.
- For the film, at `standing::stand` over the cached bounds pass `raw/film-bounds.tsv`, each crown the frame shows being taken in turn.

The code is `shadow-fit.patch` in this folder. It adds the `sun_light` re-export, the helpers in `tests/common` and one test in each of `person_frame.rs` and `film_placing.rs`. It is not applied on the branch.

## Every shipped preset at its catalogue age, and Telperion at 80

On both axes the ratio is 1.000 in every case: the person lies inside the volume the tree's own thrown shadow already fills.

| Tree | Harness sun 135°/30°, cm per texel | Film sun 115°/60°, cm per texel |
|---|---|---|
| Oregon white oak | 3.17 | 1.84 |
| Norway spruce | 3.33 | 1.75 |
| European beech | 3.14 | 1.81 |
| Silver birch | 4.40 | 2.27 |
| Date palm | 6.66 | 3.88 |
| Telperion | 15.54 | 9.70 |
| Laurelin | 15.15 | 9.44 |
| Telperion at 80 | 15.08 | 9.37 |
| Telperion at 80, seed 7 | 15.67 | 9.94 |

## Outside the ruling's list, for context

- **Seedlings at age 1, all seven presets:** 1.26 to 1.82 times coarser. Under the harness sun the texel goes from 0.41 to 0.64 cm to 0.74 cm, and under the film's sun from 0.23 to 0.33 cm to 0.37 to 0.39 cm.
- **The beech at age 10 in `rim_at_distance`:** 1.04 times, on the shorter axis.

## The film, under its sun of 115°/60°

The film breaks the bound in 469 tree renders (a crossfaded frame draws two trees, so each counts twice) across its 1,050 frames. Its coarsest frame is 719, the last frame of segment 6 (Laurelin at age 1): 5.66 times coarser, from 0.31 cm to 1.22 cm per texel.

| Segment | Tree renders over 1.1, frames | Coarsest frame | Ratio | Coarser axis, without to with (cm) | Crown height (m) |
|---|---|---|---|---|---|
| 0 (growth from age 1) | 118, frames 0-96 | 5 | 2.80 | 0.26 to 0.73 | 0.49 |
| 1 (walk at 40) | 11, frames 279-299 | 296 | 1.14 | 1.79 to 1.79 | 11.9 |
| 2 | 13, frames 300-352 | 306 | 1.16 | 1.74 to 1.74 | 12.1 |
| 3 | 49, frames 411-479 | 458 | 1.30 | 1.81 to 1.81 | 11.7 |
| 4 | 74, frames 480-553 | 524 | 1.59 | 1.55 to 1.55 | 11.6 |
| 5 | 14, frames 678-689 | 687 | 4.17 | 0.41 to 1.27 | 0.94 |
| 6 (walk at age 1) | 30, frames 690-719 | 719 | 5.66 | 0.31 to 1.22 | 1.14 |
| 7 | 160, frames 720-849 | 721 | 5.65 | 0.32 to 1.23 | 1.27 |

The failures come in two kinds:

- **Segments 1 to 4, age-40 crowns about 12 m tall.** The film stands the person up to 3.7 m out past the reach to get it out of the shadow, and that refits the map's shorter axis by up to 59 %. The coarser axis does not change, so the texel size the shaders read (`Light::texel_size`, the larger axis) does not change either.
- **Segments 0 and 5 to 7, crowns under 1.3 m.** The person's own box and its shadow are larger than the tree's, so both axes refit by up to 5.7 times. The coarsest texel is still 1.2 cm, finer than the 1.8 to 3.3 cm of any catalogue tree at its age.

## What this leaves to the host

The ruling's stop condition holds, so nothing in the renderer has changed. The weighing the ruling names is the host's: fit the map to the tree alone and give the person's shadow another route, or accept the film's refit on seedlings and wide stand-offs with a bound that names them.
