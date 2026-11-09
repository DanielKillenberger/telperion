# fn-335 protocol: draft for the host, not frozen

Written by the worker on 9 Nov 2026 after R1's dry run. Nothing below counts until the host freezes it as `PROTOCOL.md`. Each choice is marked **proposed** where the spec's words leave it open; the questions at the end are the host's.

## The recorder (built, kept as `probes/recorder.patch`, applies at `41c425e5c`)

- `telperion_space::grow_laterals(species, request)`: the grower set up as `grow_capture_settled_at` sets it up (`Working::reference()`, every lateral grown), grown to the age, then `mark` and `assign` as `settle_shed` runs them. Every lateral is read there, before shedding, so the shed ones are included. The shed is then taken with `shed::shed` to flag the laterals the settle drops. The grown tree is not changed.
- The spec's open point, answered: the reference working set's tree before the settle holds every axis the settle drops. The test `the_kept_laterals_are_the_full_cycles` (oak, age 12, seed 1001) checks that the unshed laterals are exactly the lateral axes `grow_cycle` returns, by lineage, and that some laterals are shed.
- `telperion_space::expected_bud_counts(species, pa, cycles)`: fn-334's expectation, rewritten unchanged in meaning.
- `pipeline::lateral_tests::lateral_records` (core, ignored): the preset at its own age, fidelity 0, per seed. It prints `WEIGHTS` (per age: leaf area, internode, pipe, exponent), `POT` (per lateral age and span: the expectation's phytomers by age), `NOPOT` (a span the tables refuse) and one `REC` per lateral.
- The runner is `probes/dry.py`. It reads the lines for sense only.

## The record's fields (R2)

| Field | Read from |
| --- | --- |
| age | the lateral axis's `pa` |
| birth | `Axis::birth`, the cycle its bud was made |
| span | the tree's age minus birth: the cycles it could grow, and the span of its potential |
| light at birth | **proposed:** its first phytomer's `light`. The grower reads the light at each living apex's tip after every cycle, so this is the light at the bud's own place at the end of its birth cycle. Also recorded: the bearer's phytomer at its node (`bearer_light`). |
| apex lived | `apex_end - birth`, or the span where the apex lives at the age. Also recorded: the last cycle anything in its subtree grew. |
| subtree grown, by age | per age of the reference axis: the phytomers counted, and summed by their `scale` after `assign` |
| woken, bearer age, node, order | the origin, the bearer's `pa`, the lateral's branching order |
| shed | whether the settle drops it |

The four quantities, each a sum over the subtree's ages of phytomers times a weight per phytomer of that age (**proposed:** fn-334's weights, so the realised and the potential are on one scale): phytomers 1, leaf area `leaf_area`, section `pipe^exponent`, wood length `internode`. The realised share is the realised sum over the potential's sum for a bud of the lateral's age over its span.

## The dry run (one tree per preset, seed 1001, `raw/dry-*.log`, `raw/dry-read.txt`)

Every tree grew and every line parsed. Run time: 0.4 s (beech) to 7.4 s (spruce) for the whole probe on two threads.

| Preset (age 40) | Lateral ages seen | Laterals | Grew no phytomer | Span 0 |
| --- | --- | --- | --- | --- |
| oak | 6, 7, 8, 9, 13, 14 | 135,506 | 43,181 | 11,758 |
| birch | 6, 7, 8, 9, 14 | 129,351 | 25,671 | 10,090 |
| beech | 6, 7, 8, 9, 12, 14 | 19,581 | 4,045 | 2,283 |
| spruce | 8, 10, 12 | 810,017 | 139,156 | 53,566 |

The span-0 laterals are those born in the last cycle; the tables refuse a span of 0 (`NOPOT`), and they grow nothing. The rest of "grew no phytomer" are laterals with a span that never grew a unit (aborted, or a bud still asleep); they have no first light.

Medians of the realised share (leaf area, counted), over the laterals that grew:

| Preset | Per age |
| --- | --- |
| oak | 6: 0.43, 7: 0.64, 8: 0.75, 9: 0.74, 13: 0.76, 14: 1.03 |
| birch | 6: 0.72, 7: 0.80, 8: 0.86, 9: 0.87, 14: 1.10 |
| beech | 6: 0.47, 7: 0.86, 8: 0.97, 9: 1.01, 12: 1.04, 14: 1.21 |
| spruce | 8: 0.99, 10: 0.99, 12: 1.17 |

Counted and scale-summed medians differ by at most 0.06 (the birch's age 9, wood length). On the dry run's provisional light bins (edges 0, 0.2, 0.4, 0.6, 0.8, 0.9, 0.95, 1; for reading sense only), the leaf share's 90th over its 10th percentile within a bin runs from 1.0 (bins of one or a few laterals) to 10.3 (the oak's age 8).

## What the dry run says about the protocol's sense, for the host

1. **The shares sit near 1, not at 1/10 to 1/1,000.** Before shedding, a lateral grows 0.4 to 1.2 times the tables' potential over its own span. The spec's premise quotes fn-334's "10 to 1,000 times what the tree realises". That figure was a multiplier fitted to a bud grown for the tree's whole age, not a lateral's realised share over its own span. On this record, what the tree removes is mostly what shedding drops afterwards: the shed share of the finest ages is 0.55 to 0.66. Measured on one seed, not judged.
2. **Shares above 1 on the tip ages come from conditioning.** The expectation counts the chance that a bud grows nothing. A share taken only over laterals that grew is therefore biased up. The oak's age 14 has 41,389 of 121,857 laterals with no phytomer of their own. Inferred from the counts.
3. **Light varies little on some fine ages.** The oak's age 8 reads 0.967 to 0.988, and the spruce's age 8 reads 0.944 to 0.992. A light-bin read on those ages has one bin.
4. **The spruce's age 12 takes two values** (0.585 and 1.17). Its laterals grow one or two phytomers.

## Questions for the host

1. **Fine ages.** Which ages are read? Two options: the ages fn-334's reach rule coarsens at 1.0 m (oak 7 to 9 and 12 to 14, spruce 8 to 10 and 12, birch 9 and 14; the beech's set was not tabled in fn-334), or every lateral age.
2. **Light at birth.** Is it the first unit's light (proposed), or the bearer unit's light?
3. **Laterals that grew nothing but had a span.** Are they read as share 0 at their bearer's light, or counted, named and left out? Leaving them out biases the share up (sense note 2).
4. **Span 0.** Proposed: counted, named and left out.
5. **Realised.** Is the realised quantity everything grown before shedding (as R2's "the shed ones included" reads), or should the record also carry what the subtree still holds at the age? Sense note 1 suggests the gap is the shed, not the growth.
6. **Quantities and weights.** Proposed: fn-334's weights per phytomer, and counted rather than scale-summed. Is section correct as the sum of `pipe^exponent`?
7. **"By age".** The record is by age of the reference axis. Should it also be by year (fn-334's "leaf by year")? The recorder does not hold that today.
8. **The bearer's height.** The grower's axes hold no positions before the lay, and shed laterals are never laid. The bearer's age, node and branching order are recorded. Height would need the rough layout read at birth, which is a new hook. Drop height, or add the hook?
9. **Bins.** The light bins and the age (span) bins.
10. **Seeds.** Proposed: 1001 to 1012, the spec's minimum.
11. **The verdict rule.** What thresholds give go, no-go or unresolved? For example, the share of variance light and age explain, and the spread within a bin. How is "one curve fits all four presets" tested?
