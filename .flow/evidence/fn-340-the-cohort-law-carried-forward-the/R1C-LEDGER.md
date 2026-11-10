# fn-340 R1c: the corrected probe, the structure's ledger, and the law open-loop

Drafted by the worker on 10 Nov 2026, for the host, as `R1B-ANSWERS.md` ordered. It covers answers 1 and 2, built as corrections to the probe, then reads (a), (b) and (c). The carried state is not built, and the crates end as they began.

**Sources.**
- Code: `probes/coupled2.patch`, updated with both corrections, the ledger and the replay's read by decade.
- Scripts: `probes/r1c.py` (the ledger), `probes/r1c-replay.sh` and `probes/r1c-replay.py` (the open-loop read).
- Raw logs in `raw/` (ignored): `trace-<seed>-e.log` for (a); `trace-<seed>-m.log` and `r1c-<seed>.txt` for (b); `replay-<seed>-<age>.log` and `r1c-replay.txt` for (c).
- The cycle is unchanged: its seed-1001 read at 40 is identical to fn-339's (`raw/sanity-c1.log`).

## The two corrections, as built

1. **Frozen upkeep.** A class that stops standing leaves its wood's upkeep frozen on its bearer, for as long as the bearer is held. It is dropped when retirement or compaction lets the bearer go.
   - Marked choice: the frozen wood is the most wood the class ever stood with. That is the nearest the law holds to the cycle's frozen sum of every wood the subtree laid. The law's "last wood" is the wood left once its members are shed, which is smaller.
2. **Retirement reads the cohorts.** `retire.rs` evaluates a closed subtree's living with the cohorts' living added, through `shed::subtree_living_with`, as the settle's shed does. A subtree whose cohorts lived lately is then neither closed early nor let go.
   - The shed is not patched.
   - **The settle panic is gone:** no settle failed in any five-yearly read on either seed (0 `nan`).

## (a) The oak to 120 with both corrections

Each figure is over the cycle, the probe as fn-339 left it / the probe with both corrections:

| seed, age | trunk | current-year leaf | height | spread | structural apexes |
|---|---|---|---|---|---|
| 1001, 40 | 1.080 / 1.079 | 1.043 / 1.040 | 1.001 / 1.001 | 0.964 / 0.963 | 1.005 / 0.999 |
| 1001, 80 | 1.278 / **1.194** | 1.197 / 1.080 | 1.014 / 1.011 | 1.003 / 0.988 | 1.150 / 1.017 |
| 1001, 120 | 1.684 / 1.122 | 1.893 / **0.765** | 1.058 / 1.007 | 1.074 / 0.947 | 1.908 / **0.776** (119) |
| 1007, 40 | 1.031 / 1.023 | 1.076 / 1.068 | 1.002 / 1.002 | 1.033 / 1.031 | 1.006 / 0.994 |
| 1007, 80 | 1.612 / **1.213** | 1.436 / **1.135** | 1.108 / 1.002 | 1.119 / 0.940 | 1.405 / 1.023 |
| 1007, 120 | 3.656 / **1.501** | 3.785 / **1.248** | 1.170 / 1.010 | 1.738 / 1.099 | 3.951 / 1.083 (119) |

- **The corrections take away most of the runaway.** Height is within 1% at every age on both seeds. The structure's apex count is within 2% at 80. Seed 1007's trunk falls from 3.66 to 1.50 times the cycle's at 120.
- **They do not bring the drift inside a tenth.**
  - The trunk is 1.19 and 1.21 at 80.
  - The current-year leaf is 1.08 and 1.14 at 80.
  - At 120 the two seeds part in opposite directions. Seed 1001 now falls below the cycle: leaf 0.77, structure 0.78. Seed 1007 is still over it: trunk 1.50, leaf 1.25.
- So (b) and (c) were run.
- **The balance gap is now reversed, and that is not explained.** The probe now hands its bearers 1.1 to 2.3 times the cycle's cohort upkeep from 40 on, where before it handed 0.45 to 0.55 times. Its light handed is 0.72 to 0.86 of the cycle's. A frozen sum taken from the most wood each class held should not exceed the cycle's every-wood frozen sum. That it does was not traced.

## (b) The structure's own population, ledger by age

**What was counted.** Every structural apex, per physiological age, year by year, cycle beside probe:
- living after the balance;
- born by sprouting, born by waking;
- grown on (continuation), relayed;
- stopped at the lifespan's end, stopped in growth (survival or abortion);
- shed by the balance;
- retired (subtree roots let go).

For the apexes the allotment sized it also counts their presence, presence × light, presence × size and vigour. Year 1 is left out, because the first run of the ledger carried the cycle's final year into the probe's year 1 (`FRICTION.md`).

**The first line that parts** is the cumulative count, outside 0.9 to 1.1 for five years running, with at least 30 counted:

| seed | year | age | line | cycle / probe |
|---|---|---|---|---|
| 1007 | **33** | branch | relayed | 879 / 976 |
| 1007 | 35 | branch | stopped in growth | 1,095 / 1,211 |
| 1007 | 41 | branch | shed by the balance | 58 / 93 |
| 1007 | 42 | shoot | shed by the balance | 28 / 57 |
| 1007 | 59 to 70 | shoot, bough, branch | living | 0.85 to 0.89 |
| 1001 | **79** | shoot | shed by the balance | 261 / 226 |

On seed 1001 nothing else parts by this rule before 79. Two lines move early but stay inside a tenth:
- branches relayed in 31 to 40: 437 against 395 (+11% in that decade, within a tenth cumulatively);
- branches stopped in growth in 31 to 40: 408 against 373.

**The first process is the same on both seeds: branch apexes stopping in growth.** On the oak's branch (abortion 0, relay 1, relay on failure 1) that means its survival draw failing and a bud relaying it. The probe stops 9 to 11% more branch apexes than the cycle from 21 to 40, on equal living. The probe's living branches then fall behind: 0.95 of the cycle's in 51 to 60 and 0.93 in 61 to 70 on seed 1007, and 0.96 in 111 to 120 on seed 1001. That is the structure the corrected probe ends short of on 1001.

**That process's inputs, side by side.** Branch apexes, by decade, the cycle / the probe; light, size and deaths per living are presence-weighted:

| seed, years | light at the tip | size | deaths in growth per living |
|---|---|---|---|
| 1001, 21 to 30 | 0.600 / 0.603 | 1.000 / 1.000 | 0.0346 / 0.0369 |
| 1001, 31 to 40 | 0.437 / 0.445 | 1.000 / 1.000 | 0.0592 / 0.0645 |
| 1001, 41 to 50 | 0.394 / 0.399 | 1.000 / 1.000 | 0.0716 / 0.0770 |
| 1007, 21 to 30 | 0.481 / 0.499 | 1.000 / 1.000 | 0.0563 / 0.0626 |
| 1007, 31 to 40 | 0.382 / 0.401 | 1.000 / 1.000 | 0.0853 / 0.0948 |
| 1007, 41 to 50 | 0.320 / 0.339 | 1.000 / 1.000 | 0.1038 / 0.1165 |

- **The probe's branch tips are brighter by 1 to 6%, and die 6 to 12% more often.** The branch's survival is `0.98^(L^−1)` at its tip's light, so brighter tips should die less, not more. Light does not explain the deaths.
- **Size is the same.** Its presence-weighted mean is 1.000 on both, as normalisation sets it.
- **Vigour per presence does not compare across the two.** It reads about twice as high in the probe only because the probe's base is the structure's light, without the cohorts'. It is left out of the reading.
- **Nodes laid** were not counted per apex.
- **Not isolated:** what else stops a branch apex in growth in the probe more often than its light allows. Abortion is 0 on the branch, so the remaining candidates in `advance.rs` are these, none tested:
  - the kept share the balance hands the apex (`survive × kept`);
  - the relay's own draw;
  - a branch counted as stopped where the cycle continues it on another axis.

## (c) The law open-loop on the cycle's own tree and light

**What ran.** fn-339's replay grows the cycle with every axis open and keeps every year's lattice. On each structural growth unit it sets the law (the species' shedding rule, at the bearing unit's size, the probe's reading) beside the cycle's own twig and short laterals, at the tree's age, with no feedback. The read was extended to group the units by their age in decades. Each figure is the law over the cycle, with the cycle's sum in brackets:
- "lit" is living × the cell's light in the last cycle;
- upkeep is upkeep × wood on both sides, so its ratio is the wood column's;
- the replay counts the cycle's roots unweighted and the law's by presence, so the roots column does not compare and is left out.

| seed, tree age, units 0 to 9 years old | grew | living | lit | current-year leaf | wood (and upkeep) | standing |
|---|---|---|---|---|---|---|
| 1001, 40 | 0.984 (14,400) | 0.800 (23,200) | 0.849 | 1.053 (114 m²) | 0.979 | 0.928 |
| 1001, 80 | **1.089** (52,900) | 0.777 (91,600) | 0.823 | **1.172** (416 m²) | **1.067** | 0.888 |
| 1007, 40 | 1.010 (20,400) | 0.794 (37,800) | 0.837 | 1.126 (166 m²) | 1.028 | 0.925 |

- **Units 10 years old and older:** the cycle still has standing cohort-age wood there (64 m at 40 and 325 m at 80 on 1001; 1,780 members at 80). The law has none, since its classes are shed by about 10 years.
- **The law drifts on its own, with no feedback.** On the cycle's own tree and light, the current-year leaf it would hand the lattice goes from 1.05 at 40 to 1.17 at 80 on seed 1001. What it grows goes from 0.98 to 1.09, and its wood from 0.98 to 1.07. Its living apexes stay at 0.78 to 0.80 and its lit presence at 0.82 to 0.85 throughout.
- So the law under-counts living apexes by a fifth while it over-counts what they grow and the leaf they carry. The over-count grows with the tree's age. Over the same years the cell light the law reads is the old crown's (`Cell::light`). This is the gap fn-339's replay read at 40 (living 0.73 to 0.79, leaf 1.14) and did not isolate.
- **Not read:** 80 on seed 1007 and 120 on both seeds. The replay was killed at the 8 GB cap there (`FRICTION.md`). Whether the open-loop leaf keeps rising past 80 is not known.

## What is isolated and what is not

**Isolated (measured):**
- Retirement not reading the cohorts was most of the runaway, together with the frozen upkeep (both built at once; not separated). It is also what made the settle panic.
- With both corrected, height and the structure's count hold to 80. The trunk and the leaf are still 8 to 21% over at 80.

**Measured, not isolated as a cause:**
- In the closed loop, the first structural process to part is branch apexes stopping in growth: +9 to 11% from about 30, at brighter tips and equal size.
- Open-loop, the law alone grows 9% more units and 17% more leaf than the cycle's twigs and shorts on 80-year-old wood, from 20% fewer living apexes. Its error grows with the tree's age.

**Not isolated:**
- why branch tips die more at more light;
- why the law's leaf and growth per living apex rise with the tree's age;
- why the corrected probe's frozen upkeep exceeds the cycle's.

**Inferred, not tested:** the law's open-loop surplus of leaf on older wood is the bias the loop then amplifies. It is the only measured difference that rises with age without feedback.

## For the host

The three hours are spent. In the host's terms:
- (a) the drift is not inside a tenth;
- (b) names the first process (branch apexes stopping in growth) and shows its inputs do not explain it;
- (c) says the law itself drifts with age on the cycle's own tree and light.

The host decides between building R3 as it stands and ending the child.
