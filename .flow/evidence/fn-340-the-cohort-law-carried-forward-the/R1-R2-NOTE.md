# fn-340 R1 and R2: where the probe leaves the cycle, and the share and the carried step on paper

Drafted by the worker on 10 Nov 2026 at `577831cf0`, for the host. It contains measurements, arithmetic and options. No design choice is made here, and R3 has no code yet.

**Sources.** The probe is fn-339's `coupled.patch` plus a year-by-year trace and five diagnostic switches, now `probes/coupled2.patch` (it applies at `577831cf0`; the crates are restored).
- Runs: `probes/r1-trace.sh`. Tables: `probes/r1.py`. Paper arithmetic: `probes/r2.py`.
- Raw logs are in `raw/` (ignored): `trace-<seed>[-b|-c|-w].log`, `r1-<seed>.txt`, `r2.txt`.
- The oak preset (`oregon-white-oak`) is grown one year at a time to 120 through `Paused`, with a settled copy every five years.
- With no switch named, the probe reproduces fn-339's seed-1001 read at 40 exactly, for both the cycle and the probe (`raw/sanity-oak40.log`).

## R1: the drift's cause

### What parts first

Each column is the probe over the cycle on the same seed. A year counts as parting when the ratio leaves 0.9 to 1.1 and stays outside for three reads in a row.

| quantity | seed 1001 parts at | seed 1007 parts at | how it moves |
|---|---|---|---|
| **upkeep the cohort ages hand their structural bearers' balance** | **15** (0.74), 0.54 by 20 | **15** (0.74), 0.57 by 20 | about 0.45 to 0.55 from 25 to 100 |
| light they hand that balance | not by this test (0.80 to 0.88) | not until 60 (0.73) | 0.85 at 20 on both, 0.73 to 0.91 later |
| structural laterals shed by the balance, cumulative | cycle 143, probe 45 by 40 | cycle 386, probe 339 by 40 | the probe sheds fewer up to 40 to 45 |
| cohorts' current-year leaf against the cycle's cohort-age axes (settled, five-yearly) | 50 (1.10) | 50 (1.13) | 1.045 / 1.082 at 40 |
| trunk at breast height (settled) | 45 | 50 | 1.08 / 1.03 at 40 |
| leaf deposited into the lattice (current year) | 54 | 46 | 1.04 / 1.08 at 40 |
| structural apexes growing | 78 | 62 | 1.005 / 1.006 at 40; 1.91 / 3.95 at 119 |
| height (settled) | never | 110 | within 6% to 120 on 1001 |
| vigour the structural buds get, as a share of the base | **5** | **5** | cycle 0.023 to 0.038, probe 1.000 every year |

**Order on both seeds:**
1. The structural buds' share of the split is the earliest difference, from year 5. It is a level, not a drift. The cycle hands 96 to 98% of the tree's vigour to cohort-age buds in every year, and the probe hands them none.
2. Next, the cohorts hand their bearers' balance about half the upkeep the cycle's cohort-age subtrees do, against about 0.85 of the light, from year 15 to 20.
3. Then the cohorts' own leaf rises above what the cycle's cohort-age axes hold, and the trunk and the leaf deposited follow (45 to 55).
4. Last, the structure's apex count (62 to 78).

**The balance's cost against its light** (the cycle / the probe):

| seed | 20 | 30 | 40 | 60 | 80 | 100 |
|---|---|---|---|---|---|---|
| 1001 | 0.111 / 0.069 | 0.156 / 0.079 | 0.199 / 0.113 | 0.227 / 0.127 | 0.232 / 0.133 | 0.244 / 0.142 |
| 1007 | 0.095 / 0.062 | 0.174 / 0.102 | 0.209 / 0.132 | 0.263 / 0.184 | 0.258 / 0.197 | 0.256 / 0.203 |

### Each switch alone

Each switch was run alone and is crude by design. Each figure is the probe over the cycle.
- `split`: the cohorts stand in the split as laterals at their unit's middle node, by their expected living apexes' lit presence. Their share is let go.
- `spread`: each cohort's current-year leaf is deposited over its unit's nodes.
- `nopipes`: the cohorts' pipes are left out of the leaf-girth mean.
- `wood`: each cohort charges its bearer for the most wood it has held, as a closed subtree's frozen upkeep stays with its bearer in the cycle. This candidate is not in the spec; it was added because of the table above.

| mode | trunk 40 / 80 / 120 | current-year leaf 40 / 80 / 120 | structural apexes, 119 |
|---|---|---|---|
| probe, seed 1001 | 1.080 / 1.278 / 1.684 | 1.043 / 1.197 / 1.893 | 1.908 |
| + split | 1.098 / 1.276 / 1.681 | 1.043 / 1.192 / 1.891 | 1.904 |
| + spread | 1.079 / 1.278 / 1.680 | 1.042 / 1.197 / 1.892 | 1.906 |
| + nopipes | **0.682 / 0.738 / 0.989** | 1.043 / 1.197 / 1.893 | 1.908 |
| + wood | 1.080 / 1.277 / 1.677 | 1.043 / 1.195 / 1.889 | 1.892 |
| probe, seed 1007 | 1.031 / 1.612 / 3.656 | 1.076 / 1.436 / 3.785 | 3.951 |
| + split | 1.029 / 1.601 / 3.704 | 1.067 / 1.413 / 3.739 | 3.917 |
| + spread | 1.030 / 1.617 / 3.639 | 1.076 / 1.437 / 3.789 | 3.934 |
| + nopipes | **0.590 / 0.775 / 1.810** | 1.076 / 1.436 / 3.785 | 3.951 |
| + wood | 1.028 / 1.536 / 3.479 | 1.074 / 1.403 / 3.635 | 3.688 |

- **`split`** works as intended: it takes the structural share from 1.000 to 0.031 to 0.041, against the cycle's 0.023 to 0.029. Beyond that it moves every read by 2% or less. R2 below gives the reason on paper.
- **`spread`** moves nothing beyond 0.5%. The lattice does not care where within the unit the leaf sits.
- **`nopipes`** sets the trunk's level: 0.68 and 0.59 at 40. It does not set the trunk's drift, which still rises 1.45 and 3.1 times from 40 to 120, as the probe's own trunk does. Growth is untouched, as expected: the mean is read only at the settle.
- **`wood`** closes about an eighth of the upkeep gap: 1001 goes from 0.43 to 0.50 at 30. On seed 1007 it removes about 7% of the overshoot at 120.

### What is isolated and what is not

**Isolated (measured):**
- None of the spec's three candidates causes the drift of the leaf or the structure. The split and the spread move nothing that matters.
- The pipes in the mean set the trunk's level at every age. The trunk's drift follows the structure's.

**Measured, not isolated as a cause:**
- The first quantity to part after the split's level is the balance input. From year 15 the cohorts hand their bearers about half the cycle's upkeep for about 0.85 of its light.
- The probe's structural laterals are shed less in years 20 to 45. This holds clearly on 1001 (45 against 143 by 40) and weakly on 1007 (339 against 386).
- Its cohorts then hold 4 to 8% more leaf than the cycle's cohort-age axes at 40, and 10 to 13% more at 50.

**Inferred, not tested:** a cheaper balance keeps more structural laterals, and a lusher cohort leaf feeds the structure's light and the trunk's pipes. Together they compound with age. No switch that matches the cycle's upkeep was run. The only switch that would close the gap without a cause is a fitted factor, which the boundaries forbid.

**Not found:** why the cohorts' upkeep is half the cycle's. The `wood` switch shows that only about an eighth of the gap is wood standing against wood ever grown. The candidates left are below. None was tested.
- The cohort's size: it takes its bearer unit's size. Its upkeep and leaf are multiplied by that one size, where the cycle sizes each twig by its own share.
- Its presence: count times the bearer unit's mean presence, where the cycle uses each phytomer's own.
- How the cycle's frozen upkeep of a closed subtree is kept or dropped as the subtree retires.

Nor was it found why the cohort leaf per unit of structure runs high from 40.

**Not isolated:** whether the balance gap alone drives the drift. That needs a correction whose rule the cycle implies, which is the host's call (question 3).

**Two observations outside the candidates:**
- The structural sizes' spread about 1 (Σp(s−1)²/Σp) is higher in the probe from 35: 0.39 against 0.34 at 40, and 0.97 against 0.61 at 80 on 1007. The `split` switch does not remove it (0.400 against 0.406 at 80 on 1001). So it follows the different structure; it does not come from the split.
- The probe's mid-growth settle panics at `geometry.rs:63` on seed 1007: at 95 in every probe mode, and at 55 with `split`. Those years read as `nan`. The cycle never does. This is a probe defect and was not chased (`FRICTION.md`).

## R2: the cohort in the split, and the cohort's year carried forward

### How a cohort enters the split

`allocation.rs` splits a bud's vigour at every branching point:
- the axis carries on with `v λ Q_m / (λ Q_m + (1 − λ) Q_l)`;
- each lateral at the node takes `v (1 − λ) Q / (λ Q_m + (1 − λ) Q_l)`;
- `Q` is a subtree's lit presence (Σ presence × light over its living apexes).

The base's vigour is the tree's light, and a bud's size is its vigour per presence against its own age's mean, `(r/r̄)^ψ`.

The buds a cohort stands for are lateral subtrees of a cohort age on the bearer's nodes. Their `Q` is their living apexes' lit presence. Their shares feed only their own ages' sizes. Cohort and structural ages are disjoint groups, so those sizes never touch a structural group's mean.

**The options and their consequences.** Arithmetic in `probes/r2.py` (`raw/r2.txt`) on the oak's settings: λ 0.45, ψ 1 on branch, shoot, twig and short; 1.5 on trunk, leader and sprig; 0.5 on limb and bough.

- **S0, absent (fn-339).** The structural buds share the structure's light alone.
  - **Exact at λ = 0.5.** There every split is in proportion to light, so a bud's vigour is its own lit presence, with or without the cohorts. Its size per presence is therefore identical: the absence changes no size.
  - So on the spruce and beech (λ 0.5, ψ 0) and on any species at λ 0.5, the split is dormant to cohorts.
  - It acts only on the oak (λ 0.45). Against the unbiased split, the cycle passes `(1 + x) / (1 + x (1 − λ)/λ)` past each cohort node, where `x = Q_lateral / Q_beyond`. That is 0.99 at x = 0.05, 0.97 at 0.15 and 0.90 at 1.
- **S1, as the cycle's own rule implies (marked).** Each cohort class stands as a lateral on its bearer's node or nodes, with `Q` its expected living apexes' lit presence: `living × count × presence × light`. It takes its share by the split's formula, and that share is let go, since cohorts are sized 1 (R1-ANSWERS answer 6). The structure's groups and normalisation are untouched. The `split` switch is this rule, crudely, at one node per class.
  - **S1a: Q at the class's middle node,** as the switch did.
  - **S1b: Q spread over the unit's nodes by their draws.** This is nearer the cycle, where each node bears its own laterals. On paper it differs only through `x` per node: a smaller `x` at more nodes, `(1+x)/(1+1.22x)` taken more times. That is second order.
- **S2, the cohorts also sized by their share,** against their own age's group as the cycle sizes twigs and shorts (ψ 1). This reverses R1-ANSWERS answer 6, and the boundaries leave the structural rules alone. It is named only because fn-339 R3 measured size 1 taking the oak's leaf to 0.77 in the replay.

**One bearing unit with stated numbers** (`r2.py`):
- **The unit:** an oak branch unit laid 3 years ago. It has 2.5 medial nodes, each bearing a short (0.6) or a twig (0.35).
- **The cohorts' lit presence:** the cell's light is 0.26. Per node it is the law's living at age 3 (1.366) × 0.26 = 0.355.
- **Beyond the unit:** three younger units (living 1.139, 0.724 and 0.95 new buds per node) and the apex, at presence 1 and light 0.5 (stated, not measured). The top clusters are left out.

| | Q beyond the unit | vigour passing the unit, of what enters it |
|---|---|---|
| S0, absent | 0.500 | 1.000 |
| S1, as the cycle (λ 0.45) | 2.328 | **0.677** |
| unbiased split (λ 0.5) on the same lights | 2.328 | 0.724 |

- **Direction:** entering the cohorts lowers the vigour reaching the structural buds beyond the unit by a third (×0.677). Of that, ×0.724 is the light share, which normalisation removes. Only ×0.935 is λ's bias, which it keeps.
- **For the apex:** with cohorts as buds it keeps 0.107 of what enters the unit, against a light share of 0.156 at λ 0.45. At λ 0.5 it keeps exactly 0.156, its light share. Absent, it keeps 1.0.
- **What reaches a size:** two branch apexes at one light, one on a branch one unit long and one four units long. In the cycle the old one's size is 0.80 of the young one's (ψ 1). In the probe it is 1.00. So S1 makes old, cohort-laden structural axes slightly smaller than young ones. The measured `split` switch shows that this moves the grown oak by 2% or less.

This is arithmetic for one unit with a stated apex light. The tree-wide effect is the switch's measurement.

### The cohort's year as a carried state

The law today (`law.rs`, `Unit::member`) rebuilds each class from its birth every year. It runs over the whole light history and memoises the inner members afresh, so the cost per class-year grows with the class's age.

**What a carried class holds.** Take the oak's twig, a 5-unit chain that bears shorts. Its root is the lateral of one age, born in one cycle, on one unit. The class holds:
- `alive`: the chance its root apex still grows. It also holds P(T = k), the chance the root stopped after exactly k units, for k up to the chain's length. This mixture is what the subtree shedding rule reads, because what stands depends on when the root stopped.
- The root's balance memory `m` (balance.rs's `m ← (m + (q − c)/(q + c))/2`), one scalar along the root-alive branch, as the recomputation keeps it.
- For each unit n the root has grown, that unit's inner members as their own carried classes. For the oak's shorts this is a 3-unit chain bearing nothing: an alive chance and P(T' = k').
- A ring of the last `shedding` years' expected living, per k branch: 2 to 3 values on the oak. `stands` reads its maximum.
- The class's place, node, size, presence and count, as now.

That is about 50 numbers for an oak twig class (arithmetic).

**One year moves it** with the light `L` the class read at its place from last cycle's lattice, as now:
1. Shade survival `σ = viability^(L^−shade_hazard)` on the alive branch.
2. The root's balance from its kids' living and wood: `q = living × L`, `c = upkeep × wood`. The memory and hazard are updated, and the alive mass the root loses joins P(T = k_now).
3. Units grow on the alive branch. Each unit's laterals are born as inner classes, by `lat[age][i]`. Inner classes step by the same rule, without the balance (inner, as now).
4. The ring shifts. A branch whose ring's maximum and own timer both pass `shedding` stops standing.
5. LEAF_CUR, WOOD, SECTION and LIVING are read off the state as sums.

**Light it reads:** one value per class per year, at its unit's mean node, from the previous cycle's lattice. That is unchanged from fn-339. The `spread` switch shows that where the leaf is deposited within the unit does not matter.

**Where it differs from the recomputed expectation, and why:**
- **In exact arithmetic it does not differ, at any light path (inferred from the code).** The recomputation never revises the past:
  - each class's light history is fixed once read;
  - a balance shed (`dead`) zeroes only the years after it;
  - survival at cycle c reads only light at c;
  - the root's memory and `stands` read only years up to c;
  - the lump at `cap` is the alive branch, carried.
- **So they agree to rounding (f64, summation order),** under constant light and under varying light. R3's test, constant light, is the weaker case. A test under a varying path would hold the same.
- **A reduced state would differ.** For example, members pooled by (age, years in age, living), with no root-death mixture or per-unit kids:
  - it cannot shed by the subtree rule (R1-LAW item 9), and would fall back to per-member timers, which keep the spruce's shoots two years past their spur;
  - it cannot keep a per-root balance memory.
- **Cost (arithmetic):** today a class-year costs about `a × cap² × Q` plus its inner members' rebuild. Carried, it costs about the size of its state.
  - fn-339 measured the step at 0.172 of 0.252 s on the oak at 40. If carrying makes the step free, the probe's 40 lies between 0.080 and 0.252 s against the cycle's 0.941 s, so 3.7 to 11.8 times cheaper. Where in that range is not known until it runs.
  - Carrying does not touch the drift above.

## Questions for the host

1. **The split:** S0, S1a, S1b or S2? S1 is what the cycle's rule implies. Measured, it moves the oak by 2% or less, and on paper it is dormant at λ 0.5.
2. **The carried state:** exact, as above (equal to the recomputation to rounding at any light; about 50 numbers per oak twig class), or a reduced table that gives up the subtree shedding and the root's memory? And should R3's test hold equality under a varying light path as well as a constant one?
3. **The balance gap is the drift's first measured mover, and none of the named candidates is it.**
   - Should R3 correct the cohorts' balance inputs, the upkeep and the light? If so, by which of the cycle's rules: its frozen upkeep of closed subtrees, each member's own size and presence, or something else?
   - Or should the worker first trace why the upkeep is half? That would read the cycle's cohort-age subtrees' wood, size and presence beside the law's, one class at a time. It is about one more hour, diagnostic only.
4. **The leaf-girth mean:** keep the cohorts' pipes in it (trunk 1.03 to 1.08 at 40) or leave them out (0.59 to 0.68)? Either way the trunk's drift follows the structure's.
5. **The probe's mid-growth settle panic on seed 1007:** fix it in R3, or leave it, since R4 settles only at the final age?
