# fn-340 R1b: why the cohorts' upkeep is half, the diagnostic, and the settle panic

Drafted by the worker on 10 Nov 2026, for the host, as `R1-R2-ANSWERS.md` ordered. It holds measurements and arithmetic only. There is no R3 code, and the crates end as they began.

**Sources.** The probe and every instrument are in `probes/coupled2.patch`, updated to carry the class dump, the labelled diagnostic and the panic check.
- Scripts: `probes/r1b.py` (the decomposition), `probes/diag-ratios.py` and `probes/r1b-diag.sh` (the diagnostic).
- Raw logs are in `raw/` (ignored): `dump-1001.log`, `r1b-1001.txt`, `trace-<seed>-d.log`, `diag-*.txt`, `panic-1007.log`.

## 1. The upkeep gap: what it is

**Setup.** Oak seed 1001, at cycles 20 and 30, at the balance.
- In the cycle, every cohort-age lateral subtree on a structural bearer is read: 2,603 subtrees at 20, 8,374 at 30. For each, the read takes its upkeep (the open subtree's total, or its frozen value once closed), its light, its pencils' wood, and its phytomers' size and presence.
- In the probe, every class is read: 1,150 and 3,455.
- Both are keyed by the bearer's lineage and birth and the cycle its bearing unit was laid.

| | cycle 20 | cycle 30 |
|---|---|---|
| upkeep handed to structural bearers, cycle | 210.4 | 595.9 |
| from open subtrees | 91.0 (43%) | 204.0 (34%) |
| **from closed subtrees, frozen** | **119.4 (57%)** | **391.9 (66%)** |
| upkeep handed, probe | 113.7 | 263.2 |
| probe over the cycle's open part | 1.25 | 1.29 |
| probe over the cycle's whole | 0.54 | 0.44 |
| light handed, cycle / probe | 1,905 / 1,647 | 3,924 / 3,369 |

**By the bearing unit's age (cycle 30, seed 1001).** Columns are the cycle's open subtrees, the cycle's closed subtrees, and the probe:

| unit age | cycle open | cycle closed | probe |
|---|---|---|---|
| 2 to 3 | 28.7 | 0.5 | 31.4 |
| 4 to 6 | 95.3 | 18.9 | 117.2 |
| 7 to 8 | 80.1 | 20.4 | 77.2 |
| 9 to 10 | 0 | 82.7 | 37.4 |
| **11 to 28** | **0** | **269.5** | **0** |

At 20 the pattern is the same: units 11 to 18 years old hold 61.7 of the cycle's frozen upkeep, and the probe has nothing there (`raw/r1b-1001.txt`).

**The cause, measured.** The gap is the cycle's frozen upkeep of closed cohort-age subtrees. Every cohort-age subtree on these units is closed by year 9.
- The cycle keeps charging their bearer for them for as long as the bearer stands: units 28 years old still pay.
- The probe drops a class once it no longer stands (its subtree shed, about year 10 at most). From then on it charges nothing.

**The rule behind it, read in the code.** `retire.rs` says a retired subtree's "upkeep stays frozen in its bearer's balance (`working.rs`), as the balance charges it today". `working.rs` says a closed subtree's "upkeep is frozen once, as the sum the whole tree's pass takes". So the cycle's balance charges a lateral for all the wood its subtree ever laid, dead and shed included, until the lateral itself goes. The `wood` switch of R1 kept only the class's own peak wood while the class stood. That is why it closed only an eighth of the gap.

**Over the open part, the probe charges more:** 1.25 to 1.29 times, rising to 1.20 to 1.30 times on units 4 to 6 years old. Its wood at size 1 times presence is 1.33 to 1.35 times the cycle's open subtrees. A probe class stands, and charges, up to its shedding, so its expected wood includes members that are dead but standing. In the cycle those members sit in subtrees already closed and frozen. The two sides are not split finer than that here.

## 2. The host's hypothesis: a mean size hides the spread

**The powers of size, read in the code.**
- `draw.rs` lays each phytomer at scale × size.
- The leaf the lattice takes is `light::leaf_area(scale, size, area)` = scale × size × area: **power 1**.
- The wood the balance charges is `pencil.wood += internode × scale × size`: **power 1**.
- So upkeep is upkeep × wood: **power 1**.
- The pipe is `form.pipe × scale` with scale sized at the settle, so the section is **power 2** (at the exponent 2 of every cohort age).
- Size does not change a unit's node count (`advance.rs`, `stop.rs`: size enters `draws.shade` only).

So leaf and upkeep take the same power of size. With power 1, the mean over a spread equals the value at the mean. The spread cannot open a gap between them unless it correlates with the weights.

**Moments, measured** on the cycle's open cohort-age subtrees, weighted by wood at size 1:

| | cycle 20 | cycle 30 |
|---|---|---|
| mean size | 1.013 | 1.010 |
| second moment (variance) | 1.100 (0.074) | 1.139 (0.119) |
| third moment | 1.272 | 1.425 |
| variance of subtree means between subtrees | 0.076 | 0.110 |
| probe, wood-weighted bearer-unit size | 1.005 | 1.010 |
| **the share of the open-part ratio size accounts for** | **0.992** | **1.000** |

**The hypothesis does not hold for the balance.** The spread is real (variance 0.07 to 0.12, mostly between subtrees), but at power 1 it moves the upkeep by under 1%. It would matter at power 2, in the section, where the second moment of 1.10 to 1.14 is 10 to 14% above the square of the mean. The girth takes it, the balance does not.

**Presence.** The cycle's wood at size 1 times presence against the probe's count × the unit's mean presence is folded into the 1.33 to 1.35 above. It is not separated from the dead-but-standing wood. Not isolated.

## 3. The diagnostic: closing the gap, never a correction

**What ran.** The probe's cohort upkeep handed to the balance was multiplied each year by the cycle-over-probe ratio R1's trace measured for that seed and year (`probes/diag-ratios.py`). It ran on seeds 1001 and 1007 to 120 (`probes/r1b-diag.sh`, switch `diag`, `DIAG_UPKEEP`). This is a fitted factor, labelled so in the code. It is not in the law and stays out of R3.

**The gap closes.** The probe's upkeep over the cycle's is 0.99 to 1.00 every year on 1001, and 0.96 to 0.99 on 1007.

**The drift stays.** Each figure is over the cycle, the probe as fn-339 left it / the probe with the gap closed:

| seed, age | trunk | current-year leaf | height | structural apexes |
|---|---|---|---|---|
| 1001, 40 | 1.080 / 1.080 | 1.043 / 1.042 | 1.001 / 1.001 | 1.005 / 1.004 |
| 1001, 80 | 1.278 / 1.270 | 1.197 / 1.191 | 1.014 / 1.013 | 1.150 / 1.139 |
| 1001, 120 | 1.684 / 1.673 | 1.893 / 1.870 | 1.058 / 1.058 | 1.908 / 1.855 (119) |
| 1007, 40 | 1.031 / 1.026 | 1.076 / 1.071 | 1.002 / 1.002 | 1.006 / 0.994 |
| 1007, 80 | 1.612 / 1.496 | 1.436 / 1.383 | 1.108 / 1.094 | 1.405 / 1.320 |
| 1007, 120 | 3.656 / 3.569 | 3.785 / 3.709 | 1.170 / 1.165 | 3.951 / 3.849 (119) |

**With the gap closed, the probe sheds more structural laterals than the cycle,** and its structure still outgrows the cycle's. Cumulative sheds by 60 are 2,041 against the cycle's 1,618 on 1001, and 8,135 against 5,291 on 1007.

**So the balance is not the cause.** It accounts for at most about 1% of the drift at 120 on seed 1001 and about 2 to 8% on seed 1007. The search is still open. What R1 measured parting after the balance remains unexplained, and no switch has tested it:
- the cohorts carry 4 to 8% more leaf than the cycle's cohort-age axes at 40, and 10 to 13% more at 50;
- the leaf deposited into the lattice parts at 46 to 54;
- the structure's apex count parts at 62 to 78.

## 4. The settle panic at `geometry.rs:63`, about 15 minutes

**What it met.** The probe was grown on seed 1007 to 96 with a check before the settle's shed (`TRACE_PANIC`). The check lists every lateral whose bearer holds no phytomer at its node. At 95 the panic was `the len is 0 but the index is 24` (and 34, and 60). That is a lateral's node index into a bearer whose phytomers had been let go.

**Where it came from (inferred from the printed values and the code; no fix tried).** The bearers in those lines are branches inside a retired subtree: `inside true`, phytomers let go by retirement. Yet each carries a cohort's living, for example `more Some((15.0, 0.0))`: lived 15 cycles, alive 0. The chain:
1. Retirement judged the subtree closed and its fade spent from the axes' own apexes alone. It let the phytomers go, and the probe's `cohort_rename` dropped those cohorts.
2. The settle's copy still holds the cohorts' last living for those axes (`cohort_living`).
3. `shed::fades` applies `more(i)` to every axis, gone ones included (`shed.rs`, the `more` line).
4. The fade of an axis already let go is lifted above 0. The shed keeps it with no phytomers, and the scale pass indexes a node it no longer holds.

**What it is not.** Nothing points to a size or presence out of range. It is a disagreement between retirement, which does not read the cohorts, and the settle's shed, which does.

**Why only some modes.** Lines with `more Some` appear from age 55 and grow to 47 at 96. The settle panics only where such an axis survives the shed. That happened on seed 1007 at 95, and at 55 with the `split` switch. With `spread`, the tree differs and it did not happen at 95.

**Whether this touches growth** (not tested): the cohorts dropped at retirement stop shading and stop charging their bearer during growth too.

## Questions for the host

1. **The frozen upkeep:** the gap is the cycle's rule that a lateral pays for all its subtree's wood, dead and shed, for as long as it stands. A law carrying that rule would keep each class's wood, frozen at its last value, charged to its bearer after the class stops standing, until the bearer goes. Is that the correction? The diagnostic says it will not remove the drift by itself.
2. **The search:** the trace's next mover is the cohorts' leaf per unit of structure, high from 40. Should the next trace read the cycle's cohort-age leaf beside the law's, class by class, as this one read the upkeep? Its candidates are survival under shade at the cell's light against each tip's, and the size and presence factors on the leaf.
3. **The panic:** fix it by making retirement read the cohorts' living, or `fades` skip `more` on gone axes? Or leave it, since R4 settles only at the final age?
