# fn-334 result: the derived dial, read on 9 Nov 2026

Drafted by the Opus build worker; R5 and the note on the code are the host's.

## What ran

- Probe: `crates/telperion-core/src/pipeline/derived_tests.rs` at commit `3d0973cb9`, since taken out of the crates and kept as `probes/derived.patch` (see the end). `derive` builds D1 or D0 from a species' tables alone, by PROTOCOL.md; the ignored `derived_rows` reads R3.
- Read: all eight catalogue presets at their preset ages (40; Telperion 82; Laurelin 80). Fidelities 0.25, 0.5 and 1.0, against the full cycle at 0. Seeds 1001 to 1012, five repeats less the first, and a grow over 0.5 s not repeated. Candidates D1 and D0, plus the tuned `/space/coarse` rows where a preset has them (oak, spruce, birch).
- Cost: one process per preset through `telperion-heavy`, `RAYON_NUM_THREADS=1`, each pinned with `taskset` to its own physical core (CPUs 8 to 15 of a Ryzen 9 5950X), all eight at once, native `--profile ci`. Runner: `probes/r3.py`. Tables: `probes/table.py`. Logs: `raw/r3-<preset>.log`.
- Wall time: 922 s for the longest preset (Laurelin).

Every ratio below is the derived tree's median over the 12 seeds divided by the full cycle's median on the same seeds. Worst is the largest |ratio - 1|. Speed-up is the full cycle's median seconds over the setting's.

Full-cycle seconds: ordinary 0.587, oak 2.292, spruce 5.834, beech 0.335, birch 1.613, date palm 0.0005, Telperion 16.41, Laurelin 14.71.

## R3: the read

| Preset | Read | f | height | spread | trunk | limbs | shed | area | Worst | Seconds | Speed-up | Axes | Apex-cycles |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ordinary | D1 | 0.25 | 0.996 | 0.999 | 1.007 | 1.032 | 0.796 | 1.045 | 0.204 shed | 0.5028 | 1.17 | 36334 | 83407 |
| ordinary | D0 | 0.25 | 0.996 | 0.999 | 1.007 | 1.032 | 0.796 | 1.045 | 0.204 shed | 0.4716 | 1.25 | 36334 | 83407 |
| ordinary | D1 | 0.5 | 0.997 | 0.967 | 1.611 | 0.893 | 0.738 | 2.993 | 1.993 area | 0.2075 | 2.83 | 14948 | 35264 |
| ordinary | D0 | 0.5 | 0.997 | 0.967 | 1.611 | 0.893 | 0.738 | 2.993 | 1.993 area | 0.2159 | 2.72 | 14948 | 35264 |
| ordinary | D1 | 1 | 0.997 | 0.983 | 1.692 | 0.724 | 0.694 | 3.219 | 2.219 area | 0.1132 | 5.19 | 6274 | 16442 |
| ordinary | D0 | 1 | 0.997 | 0.983 | 1.692 | 0.724 | 0.694 | 3.219 | 2.219 area | 0.1192 | 4.93 | 6274 | 16442 |
| oregon-white-oak | D1 | 0.25 | 0.998 | 0.315 | 0.399 | 0.134 | 1.438 | 0.060 | 0.940 area | 0.0410 | 55.88 | 8326 | 19622 |
| oregon-white-oak | D0 | 0.25 | 1.080 | 1.111 | 1.678 | 0.685 | 0.838 | 1.932 | 0.932 area | 0.3051 | 7.51 | 43515 | 86300 |
| oregon-white-oak | tuned | 0.25 | 1.012 | 1.033 | 1.047 | 0.947 | 0.996 | 1.024 | 0.053 limbs | 1.2731 | 1.80 | 154048 | 297928 |
| oregon-white-oak | D1 | 0.5 | refused: `states[8].upkeep: a rate lies in its bounded range` | | | | | | | | | | |
| oregon-white-oak | D0 | 0.5 | 1.099 | 0.946 | 1.692 | 0.468 | 0.821 | 1.578 | 0.692 trunk | 0.0814 | 28.17 | 12481 | 25646 |
| oregon-white-oak | tuned | 0.5 | 1.027 | 0.985 | 1.074 | 0.932 | 1.020 | 1.020 | 0.074 trunk | 0.6715 | 3.41 | 100113 | 175710 |
| oregon-white-oak | D1 | 1 | refused: `states[8].upkeep: a rate lies in its bounded range` | | | | | | | | | | |
| oregon-white-oak | D0 | 1 | 1.078 | 0.949 | 1.727 | 0.329 | 0.805 | 1.261 | 0.727 trunk | 0.0420 | 54.62 | 6645 | 14754 |
| oregon-white-oak | tuned | 1 | 1.030 | 0.959 | 0.975 | 0.985 | 0.965 | 0.990 | 0.041 spread | 0.2252 | 10.18 | 41134 | 55772 |
| norway-spruce | D1 | 0.25 | 1.000 | 0.949 | 1.278 | 0.976 | 0.850 | 1.771 | 0.771 area | 2.3766 | 2.45 | 334370 | 775418 |
| norway-spruce | D0 | 0.25 | 1.000 | 0.943 | 1.344 | 0.980 | 0.781 | 1.995 | 0.995 area | 2.6411 | 2.21 | 350050 | 816833 |
| norway-spruce | tuned | 0.25 | 1.000 | 1.036 | 0.936 | 0.993 | 1.048 | 0.826 | 0.174 area | 3.4149 | 1.71 | 467530 | 983142 |
| norway-spruce | D1 | 0.5 | 1.000 | 0.949 | 1.278 | 0.976 | 0.850 | 1.771 | 0.771 area | 2.4800 | 2.35 | 334370 | 775418 |
| norway-spruce | D0 | 0.5 | 1.000 | 0.943 | 1.344 | 0.980 | 0.781 | 1.995 | 0.995 area | 2.6844 | 2.17 | 350050 | 816833 |
| norway-spruce | tuned | 0.5 | 1.000 | 1.034 | 0.929 | 0.986 | 1.082 | 0.791 | 0.209 area | 1.9325 | 3.02 | 283564 | 616090 |
| norway-spruce | D1 | 1 | refused: `states[8].upkeep: a rate lies in its bounded range` | | | | | | | | | | |
| norway-spruce | D0 | 1 | 1.000 | 0.888 | 1.914 | 0.908 | 0.776 | 2.649 | 1.649 area | 1.6621 | 3.51 | 227772 | 531985 |
| norway-spruce | tuned | 1 | 1.000 | 1.011 | 1.009 | 1.010 | 0.953 | 0.998 | 0.047 shed | 0.7008 | 8.32 | 104697 | 245912 |
| european-beech | D1 | 0.25 | 1.000 | 0.960 | 1.178 | 1.057 | 0.702 | 1.242 | 0.298 shed | 0.1348 | 2.49 | 15628 | 50040 |
| european-beech | D0 | 0.25 | 1.000 | 0.968 | 1.178 | 1.056 | 0.701 | 1.242 | 0.299 shed | 0.1347 | 2.49 | 15640 | 50094 |
| european-beech | D1 | 0.5 | 1.000 | 0.795 | 1.410 | 0.710 | 0.898 | 1.433 | 0.433 area | 0.0390 | 8.59 | 5473 | 17219 |
| european-beech | D0 | 0.5 | 1.000 | 0.806 | 1.651 | 0.786 | 0.679 | 2.075 | 1.075 area | 0.0525 | 6.38 | 6820 | 22459 |
| european-beech | D1 | 1 | 1.000 | 0.704 | 1.117 | 0.463 | 0.898 | 0.896 | 0.537 limbs | 0.0165 | 20.27 | 2003 | 6176 |
| european-beech | D0 | 1 | 1.000 | 0.815 | 1.653 | 0.713 | 0.641 | 2.272 | 1.272 area | 0.0316 | 10.61 | 3980 | 13628 |
| silver-birch | D1 | 0.25 | 1.000 | 0.975 | 1.082 | 1.140 | 0.913 | 1.515 | 0.515 area | 0.3450 | 4.67 | 32366 | 121134 |
| silver-birch | D0 | 0.25 | 1.000 | 0.992 | 1.106 | 1.145 | 0.880 | 1.646 | 0.646 area | 0.3563 | 4.53 | 32800 | 123446 |
| silver-birch | tuned | 0.25 | 1.000 | 1.003 | 1.074 | 1.041 | 1.044 | 1.162 | 0.162 area | 1.1435 | 1.41 | 98445 | 344323 |
| silver-birch | D1 | 0.5 | 1.000 | 0.982 | 1.038 | 1.182 | 0.936 | 1.378 | 0.378 area | 0.2614 | 6.17 | 26934 | 100237 |
| silver-birch | D0 | 0.5 | 1.000 | 0.992 | 1.099 | 1.181 | 0.864 | 1.628 | 0.628 area | 0.2552 | 6.32 | 27739 | 104824 |
| silver-birch | tuned | 0.5 | 1.001 | 0.986 | 1.081 | 1.025 | 1.069 | 1.177 | 0.177 area | 0.7541 | 2.14 | 78251 | 250508 |
| silver-birch | D1 | 1 | 1.000 | 0.982 | 1.038 | 1.182 | 0.936 | 1.378 | 0.378 area | 0.2403 | 6.71 | 26934 | 100237 |
| silver-birch | D0 | 1 | 1.000 | 0.992 | 1.099 | 1.181 | 0.864 | 1.628 | 0.628 area | 0.2720 | 5.93 | 27739 | 104824 |
| silver-birch | tuned | 1 | 1.000 | 0.981 | 1.002 | 1.004 | 1.031 | 0.999 | 0.031 shed | 0.3280 | 4.92 | 43610 | 109979 |
| date-palm | D1, D0 | 0.25, 0.5, 1 | refused at every fidelity under both: `states[2].leaf_area: a rate lies in its bounded range` | | | | | | | | | | |
| telperion | D1 | 0.25 | 1.000 | 1.001 | 1.004 | 1.016 | 0.874 | 1.030 | 0.126 shed | 6.2584 | 2.62 | 432455 | 1517158 |
| telperion | D0 | 0.25 | 1.000 | 1.001 | 1.009 | 1.016 | 0.841 | 1.090 | 0.159 shed | 5.8669 | 2.80 | 449529 | 1584300 |
| telperion | D1 | 0.5 | 0.985 | 0.645 | 0.797 | 0.824 | 1.031 | 0.148 | 0.852 area | 0.8908 | 18.42 | 53193 | 180692 |
| telperion | D0 | 0.5 | 1.000 | 1.000 | 1.079 | 1.048 | 0.846 | 1.337 | 0.337 area | 3.3494 | 4.90 | 171310 | 621134 |
| telperion | D1 | 1 | 0.935 | 0.457 | 0.562 | 0.668 | 0.923 | 0.000 | 1.000 area | 0.2046 | 80.19 | 8817 | 28974 |
| telperion | D0 | 1 | 1.000 | 0.998 | 1.055 | 1.048 | 0.817 | 1.301 | 0.301 area | 2.3462 | 6.99 | 90622 | 342542 |
| laurelin | D1 | 0.25 | 1.000 | 1.000 | 0.999 | 1.004 | 0.874 | 1.062 | 0.126 shed | 5.7416 | 2.56 | 396042 | 1386918 |
| laurelin | D0 | 0.25 | 1.000 | 1.000 | 1.004 | 1.000 | 0.838 | 1.107 | 0.162 shed | 5.8877 | 2.50 | 406845 | 1430686 |
| laurelin | D1 | 0.5 | 0.992 | 0.651 | 0.815 | 0.828 | 1.031 | 0.212 | 0.788 area | 0.8624 | 17.06 | 50881 | 172704 |
| laurelin | D0 | 0.5 | 1.000 | 1.001 | 1.065 | 1.021 | 0.837 | 1.380 | 0.380 area | 3.0031 | 4.90 | 156549 | 566426 |
| laurelin | D1 | 1 | 0.955 | 0.457 | 0.584 | 0.686 | 0.930 | 0.000 | 1.000 area | 0.2096 | 70.21 | 8680 | 28680 |
| laurelin | D0 | 1 | 1.000 | 1.003 | 1.041 | 1.028 | 0.813 | 1.385 | 0.385 area | 2.2340 | 6.59 | 83514 | 315031 |

The refusals come from the engine's validation of the derived species, before growth. The oak's age 8 at f = 1.0 gets U = 68.7 on an upkeep of 0.35, so 24.1 against a bound of 10. The spruce's age 9 at f = 1.0 gets U = 62.5. The palm's single stem has borne = 0, so it coarsens to its cap. L then carries its own leaves, giving 20.7 to 33.7 m² a node against `MAX_LEAF_AREA` = 5.

Beech, Telperion and Laurelin have no `/space/coarse` rows, so they have no tuned read.

## Per-age multipliers (D1)

Only coarsened ages (s > 1) are listed. "Clamped" names each multiplier that took 1, either because its solution fell under 1 or because no solution exists. Ages 0, 1, 5, 10 and 11 of the oak, 0 and 10 to 13 of the birch, and 0, 3, 5, 6, 7, 13 and 14 of the spruce have borne 0 and every multiplier clamped. Each of them is an unused age, and all of them grow nothing.

**Oregon white oak** (age 40)

| f | age | borne m | s | L | P | U | clamped |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0.25 | 9 | 0.166 | 1.506 | 54.0 | 5.29 | 15.46 | - |
| 0.25 | 12 | 0.059 | 4.271 | 14.8 | 3.00 | 2.00 | - |
| 0.25 | 13 | 0.059 | 4.271 | 23.5 | 3.89 | 3.26 | - |
| 0.25 | 14 | 0 | 4.000 | 4.0 | 2.00 | 1 | - |
| 0.5 | 8 | 0.292 | 1.712 | 258.2 | 8.66 | 35.52 | - |
| 0.5 | 9 | 0.166 | 3.013 | 233.4 | 10.91 | 32.72 | - |
| 0.5 | 12 | 0.059 | 4.500 | 15.7 | 3.08 | 2.00 | - |
| 0.5 | 13 | 0.059 | 4.500 | 24.9 | 4.00 | 3.28 | - |
| 0.5 | 14 | 0 | 4.000 | 4.0 | 2.00 | 1 | - |
| 1.0 | 7 | 0.664 | 1.506 | 1 | 1 | 1 | L P U |
| 1.0 | 8 | 0.292 | 3.423 | 1011.8 | 17.07 | 68.73 | - |
| 1.0 | 9 | 0.166 | 5.000 | 428.0 | 14.76 | 36.05 | - |
| 1.0 | 12 | 0.059 | 4.500 | 15.7 | 3.08 | 2.00 | - |
| 1.0 | 13 | 0.059 | 4.500 | 24.9 | 4.00 | 3.28 | - |
| 1.0 | 14 | 0 | 4.000 | 4.0 | 2.00 | 1 | - |

**Norway spruce** (age 40). The rows at 0.25 and 0.5 are the same: age 10 sits at its node cap at both, and ages 8 and 9 have borne 0.651 m, so they stay at 1 below f = 0.651.

| f | age | borne m | s | L | P | U | clamped |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0.25, 0.5 | 10 | 0.068 | 3.000 | 16.1 | 3.12 | 5.37 | - |
| 0.25, 0.5 | 12 | 0 | 2.000 | 2.0 | 1.41 | 1 | - |
| 1.0 | 8 | 0.651 | 1.536 | 65.8 | 11.09 | 42.85 | - |
| 1.0 | 9 | 0.651 | 1.536 | 96.1 | 11.09 | 62.54 | - |
| 1.0 | 10 | 0.068 | 3.000 | 16.1 | 3.12 | 5.37 | - |
| 1.0 | 12 | 0 | 2.000 | 2.0 | 1.41 | 1 | - |

**Silver birch** (age 40). The rows at 0.5 and 1.0 are the same: ages 2 to 8 bear laterals reaching 1.2 to 5.1 m and stay at 1, and age 9 is at its node cap of 5 from f = 0.5.

| f | age | borne m | s | L | P | U | clamped |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0.25 | 9 | 0.047 | 4.167 | 33.6 | 4.00 | 1.49 | - |
| 0.25 | 14 | 0 | 2.500 | 2.5 | 1.58 | 1 | - |
| 0.5, 1.0 | 9 | 0.047 | 5.000 | 46.4 | 4.68 | 1.65 | - |
| 0.5, 1.0 | 14 | 0 | 2.500 | 2.5 | 1.58 | 1 | - |

The rows of every preset, candidate and fidelity are the `ROWS` lines of `raw/r3-<preset>.log`.

## R4: the verdict, by the frozen rule (D1 at f = 1.0)

| Preset | Verdict | Worst | Seconds against full |
| --- | --- | --- | --- |
| ordinary | misses | area 3.219 (2.219) | 0.113 against 0.587 |
| oregon-white-oak | fails to grow | `states[8].upkeep` out of range | none |
| norway-spruce | fails to grow | `states[8].upkeep` out of range | none |
| european-beech | misses | limbs 0.463 (0.537) | 0.017 against 0.335 |
| silver-birch | misses | area 1.378 (0.378) | 0.240 against 1.613 |
| date-palm | fails to grow | `states[2].leaf_area` out of range, at every fidelity under D1 and D0 | none |
| telperion | misses | area 0.000 (1.000) | 0.205 against 16.41 |
| laurelin | misses | area 0.000 (1.000) | 0.210 against 14.71 |

No preset holds or comes near. Five miss and three fail to grow.

No derived setting that grew was slower than the full cycle. The smallest speed-up is the ordinary preset under D1 at 0.25, at 1.17.

**Statistics that miss on most presets.** Leaf area is the worst statistic in 4 of the 5 misses at 1.0 (ordinary, birch, Telperion, Laurelin). The beech's area is also off, at 0.896. Area moves in both directions:

- Under D1 it collapses where the upkeep term is large. Telperion and Laurelin reach 0.000 at 1.0 and 0.15 to 0.21 at 0.5. The oak reaches 0.060 at 0.25, with spread 0.315 and limbs 0.134.
- Where upkeep does not bite, area overshoots: ordinary 3.2, spruce 1.8, birch 1.4 to 1.5.

The shed share departs 12 to 30% low at 0.25 on every preset that grew there (ordinary 0.796, beech 0.702, spruce 0.850, birch 0.913, Telperion and Laurelin 0.874). Height stays within 10% everywhere: 0.935 to 1.000 under D1, and up to 1.099 under D0 (the oak at 0.5).

**D0 against D1.** D0 never fails to grow apart from the palm. Without the upkeep term:

- Area overshoots on every preset: 1.26 to 3.22 at 1.0.
- The trunk overshoots on the oak, spruce and beech: 1.65 to 1.91 at 1.0.

With the upkeep term (D1), the crowns of the oak, Telperion and Laurelin die back, and the oak and spruce leave the engine's bounds at 1.0. On the ordinary preset D1 and D0 read the same: it has upkeep rows but no `balanceHazard` row, so nothing reads the upkeep.

**Beside the tuned rows.** At 1.0 the tuned rows hold on the oak (0.041), spruce (0.047) and birch (0.031). The speed-ups are 10.2, 8.3 and 4.9.

## What surprised the worker, for the host to judge

1. **The multipliers are very large on ages just below the limbs.** Examples are the oak's L = 1012 and U = 68.7 at age 8 (f = 1.0) and the spruce's U = 62.5 at age 9. The equation for age k matches a bud of age k grown for the tree's whole age. Most such buds are born late and live a short time, but they still get the multiplier fitted to a 40-cycle subtree. This reading is inferred from the numbers and was not tested.
2. **A coarsened age can get every multiplier clamped while its older ages carry large ones.** At 1.0 this happens at the oak's age 7 (s = 1.506) and the ordinary preset's age 8 (s = 1.199). The older ages' multipliers already make the bud's expectation exceed the full one, so the solution for the age's own multiplier falls under 1.
3. **The palm's stem coarsens.** Its only age bears no lateral (borne = 0), so by the protocol s = cap = min(f / internode, mean nodes). The stem's leaves then go to L = s, past `MAX_LEAF_AREA`.
4. **The requested ordinary test fails.** That test checks that the derived species validates on every preset at 0.25, 0.5 and 1.0. By the frozen formulas it does not: at age 24 the oak and spruce fail at 1.0 under D1, and the palm fails at every fidelity. The test stays in the file, ignored, with this reason. Un-ignoring it, rewriting it or bounding the derivation is the host's decision.

## Readings the worker made inside the protocol

- **reach(j).** The continuation term is `continuation * reach(j + 1)`, as written, with no survival factor over the lifespan. The protocol does not apply survival so far to the move-on. If it should, s rises a little on ages whose lifespan has viability below 1.
- **Lifespan.** The probe uses `lifespan` as stated, not the engine's tick grid `life()`, which is crate-private. The difference is at most half a tick.
- **Step 2 adds no pipe factor of its own.** The protocol's step 2 lists only the internode and the node law, and step 3's P equation is written on the uncoarsened `pipe(pa)`. So P carries the whole section, including what `coarsened` writes as s^(1/exponent). The P column above is that total.
- **Expected laterals.** These are counted over `zone.lateral`, times `buds`, `readiness` and the zone's mean nodes, without `rhythm` and without sleeping buds, following the host's note.

## R5: the next step (host)

**The derivation is refuted as frozen.** What a missing lateral would have carried cannot be read from the species' tables alone. The tables give the wood a bud could grow with nothing shading it, and the tree grows a small share of that: the tables' answer for the oak's age 8 is 1,012 times a node's leaf, where the rows a search found use 16 for the whole dial. Light decides how much of the potential a twig realises, and the expectation holds no light. Both candidates show it from opposite sides: with the potential wood's upkeep charged (D1) the crown starves to nothing, and without it (D0) leaf area overshoots by 1.26 to 3.22 times.

**What held.** The rule for which ages coarsen (an age coarsens only as far as its laterals are smaller than the resolution) needs no row and no `first`, and it is where the speed is: D0 at 1.0 m grows the oak 55 times faster, Telperion 7 times and the beech 11 times, with height within 10% everywhere and spread within 12% except the beech (0.82). It loses limbs (oak 0.33, beech 0.71), so its reach measure counts as small some laterals the limb count reads as limbs. The tuned rows show the target is reachable: 3 to 5% on three species at 5 to 10 times.

**So the missing quantity is the realised wood of a lateral, given its light.** That is the gap a small model would fill, as the owner proposed on 9 Nov ("unless we can generate the twigs and stuff with a NN?"). Before any model, one measurement settles whether it can be learned at all:

- From full cycles of the oak, spruce, birch and beech, record for every lateral of a fine age the light at its birth and what its subtree really carried over its life (leaf by year, section, wood length, years lived), beside the tables' potential.
- Read how tightly the realised share follows light and age. A narrow band means a curve or a model of a few inputs can supply L, P and U as the tree grows, read at each node from the light the grower already holds. A wide one means the stand-in needs more than local state, which is the council's objection, measured.

The reach rule is kept for that child, with its limb loss to be fixed by measuring reach on the ages the limb count reads.

**Not proposed:** bounding or re-fitting the frozen formulas. A horizon shorter than the tree's age, or a cap on the multipliers, would be a tuned constant standing in for light.

## The code

No product code stays from this child. The probe, its tests and `telperion_space::expected_bud_counts` are in `probes/derived.patch` (505 lines, applies at `10fd9ae00`); the crates are back as that commit has them, so the gate and the review have nothing to read (fn-322's rule). The next child applies what it needs from the patch, rewritten and tested as the code rules ask. The worker's four ordinary tests and the measurement passed on the binary it measured with; its last reformat was not rerun.

