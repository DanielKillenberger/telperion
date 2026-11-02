# fn-326 search rounds

Frozen before each grows a tree. The rule is `probes/search.py`'s docstring.

## Spruce and birch, round 1

First ages are those each preset's value file gives a lifespan, from its fourth on, so its trunk and first limbs keep their phytomers. 400 candidates on two seeds, then 3 generations of 100 on five. Fidelity 1.0, each preset's age. The bar is the oak's: all six R4 medians within 5%.

## Spruce, judgement 1

Round 1's search on tuning seeds 1001 to 1005 gave its cheapest rows within each departure. The three below go to held-out seeds 1, 7, 42, 1234, 5678 at five repeats. The rule, fixed now: the cheapest of them whose six medians all hold within 5% on the held-out seeds ships in the preset's rows; if none holds, the climb goes on from the tuning seeds, and held-out seeds still choose nothing but this pass or fail.

- S1: first 8, factor 2.417, leaf 13.725, pipe 1.834, bud 1.022, hazard 1.512, shedding 0.067
- S2: first 8, factor 2.608, leaf 16.014, pipe 2.11, bud 1.0, hazard 1.277, shedding 0.056
- S3: first 8, factor 2.775, leaf 17.965, pipe 2.136, bud 1.092, hazard 1.135, shedding 0.055

## Birch, round 2

Round 1 stopped at worst 0.072 and 96 ms against the full cycle's 992 ms, on tuning seeds, with factor at the search's floor of 1.1. It has not reached the 5% bar, so the climb goes on from its last three logs under the same rule: 5 generations of 100.

Result on held-out seeds, five repeats, against the full cycle's 5,172 ms: S1 worst 0.035 at 776 ms; S2 worst 0.054 at 620 ms; S3 worst 0.081 at 614 ms. S1 holds and ships.

Result of round 2 on tuning seeds: worst 0.069 at 93 ms, unchanged for four generations, with first 9 and factor at the floor. That climb is stuck.

## Birch, round 3

Each first age the climb left behind gets its own climb from its best rows in the wide pass and round 1: first 5, 6, 7 and 8, separately, 4 generations of 100, same score. Nothing else changes.

Result of round 3 on tuning seeds: first 5 reaches worst 0.042 at 845 ms; first 6 and 7 stall above 5%; first 8 reaches worst 0.034 at 178 ms and 0.048 at 167 ms.

## Birch, judgement 1

The same rule as the spruce's. Two rows go to the held-out seeds at five repeats, and the cheapest whose six medians hold within 5% ships.

- B1: first 8, factor 1.925, leaf 10.813, pipe 2.127, bud 2.559, hazard 2.274, shedding 0.734
- B2: first 8, factor 2.032, leaf 12.616, pipe 2.191, bud 3.041, hazard 2.19, shedding 0.499

Result on held-out seeds against the full cycle's 1,014 ms: B1 worst 0.117 at 199 ms; B2 worst 0.168 at 187 ms. Neither holds. Height, trunk, limbs, shed and area hold on both; spread is the one out, at 0.883 and 0.832, where the tuning seeds read 0.966 and 0.978. Five tuning seeds do not pin a birch's spread, so the climb overfitted them.

## Birch, round 4

The same climb from first 8's logs on twelve tuning seeds, 1001 to 1012 (`TUNE_SEEDS`), 4 generations of 100. Round 3's rows are read again on the twelve in the first generation only as its five best.

Result of round 4 on twelve tuning seeds: worst 0.031 at 209 ms and 0.046 at 199 ms.

## Birch, judgement 2

The same rule, second use of the held-out seeds.

- B3: first 8, factor 1.503, leaf 10.045, pipe 2.131, bud 1.534, hazard 2.366, shedding 0.65
- B4: first 8, factor 1.811, leaf 10.257, pipe 1.936, bud 1.886, hazard 2.244, shedding 0.58

Result on held-out seeds: B3 worst 0.118 at 226 ms, spread 0.882, limbs 0.904; B4 worst 0.104 at 212 ms, spread 0.897, limbs 0.896. Neither holds. Before another climb, the next read asks how much of that is five seeds' noise: B3 and B4 on tuning seeds 1001 to 1050, medians and each row's spread seed by seed. Those seeds choose nothing the held-out seeds or R4's 1 to 50 judge.

Result on tuning seeds 1001 to 1050 against the full cycle's 1,058 ms: B3 worst 0.041 at 215 ms, spread 0.989, limbs 0.959; B4 worst 0.056 at 209 ms. The same rows read spread 0.882 and 0.897 on the five held-out seeds, so most of that miss is five seeds' median, not the rows.

## Birch, decision

B3 ships in the preset's rows, and it did not pass the rule frozen for this judgement: on the five held-out seeds its worst is 0.118. That rule asked five seeds to settle what R4 asks fifty to settle. The 5% bar stands. It is read on R4's own seeds 1 to 50, which nothing has touched and which judge all three presets after this child.
