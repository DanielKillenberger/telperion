# fn-339 result: the cohort law, the first proof

Drafted by the worker on 10 Nov 2026. R1 is `R1-LAW.md`, answered in `R1-ANSWERS.md`; R2 is `R2-REAL-TREES.md`; R3 is `R3-RESULT.md` under `PROTOCOL.md`. R4 below is the coupled probe built on the host's word of 10 Nov. The verdict is the goal's rule as written. Nothing here is proposed.

## R5, the verdict

**The approach stops by the goal's rule.** The rule stops it only if both halves fail, and both fail.

**1. Closer to real trees? No.**

These are R2's checked measures. Each figure is the median over seeds 1001 to 1012 at 40, as a multiple of the range's nearer end, where 1 means inside.

| species | measure | R2's range | cycle | coupled probe | nearer |
|---|---|---|---|---|---|
| spruce | new shoots a tree a year, weighted by presence | 691 to 2,372 (Bellan 2019) | 64,650: 27.3× over | 67,270: 28.4× over | cycle |
| spruce | leaf area index over the tree's own columns | 10.5, projected (Bolstad and Gower 1990) | 12.03: 1.15× | 12.94: 1.23× | cycle |
| beech | leaf area index over the tree's own columns | 5.6 to 9.5 (Leuschner 2006) | 1.196: 0.214× | 1.209: 0.216× | probe, by 1% |

- The probe is farther from the range on two measures and nearer by 1% on the third, where it still holds a fifth of a beech stand's index.
- The oak has no checked measure. Ages 80 and 120 have none on any species.
- R3's replay reached the same within 2% (`R3-RESULT.md`).
- A lone tree's index is not a stand's. The comparison is read as the protocol froze it.

**2. Five times cheaper? No.**

These are one-thread seconds by fn-327's habit: five alternating batches per seed, each grow over 0.5 s run once, otherwise five repeats less the first. The cycle and the probe ran on the same seeds and the same pinned core.

| tree | seeds | cycle, median s | probe, median s | cycle over probe, median per seed (range) |
|---|---|---|---|---|
| oak at 40 | 1001–1012 | 1.521 | 0.464 | **3.24** (2.96 to 3.68) |
| spruce at 40 | 1001–1012 | 4.277 | 1.326 | **3.20** (3.18 to 3.26) |
| beech at 40 | 1001–1012 | 0.201 | 0.100 | **1.95** (1.64 to 2.32) |
| oak at 80 | 1001, 1007, 1012 | 13.29 | 5.275 | **2.52** (2.00 to 2.64) |
| oak at 120 | 1001, 1007, 1012 | 43.70 | 24.38 | **1.79** (1.17 to 1.86) |

The probe is cheaper everywhere and five times cheaper nowhere, and its edge narrows with the oak's age.

## R4, the coupled probe

### What was built

`probes/coupled.patch` applies at `d2baa9508` (its crates are as at `2f1c5ed20`). It holds R3's replay probe and the coupled grower, and leaves the crates as they began.

With a boundary set (`telperion_space::set_cohorts`, or `TELPERION_COHORTS=0.5`), every age outside the cohort set at 0.5 m grows through the cycle unchanged. The cohort set is the oak's twig and short, the spruce's shoot, and the beech's shoot and short. A lateral of a cohort age is not sprouted. It is counted, at the presence its draws make, on its bearing growth unit, and moved each year by the law as R3 left it: the species' shedding rule at the bearing unit's size (`subtree-size`, nearest the cycle in R3). Each year:

- **Leaf.** A cohort's current-year leaf is deposited into the lattice at its unit's place. Its light is read there, from the same lattice the cycle's apexes read.
- **Balance.** Its living apexes' light and its wood's upkeep enter its bearer's balance, as its subtree's would. A cohort on an axis the balance sheds dies with it.
- **Working set.** A living cohort keeps its bearer open in the working set.
- **Shed.** Its last living cycle enters its bearer's fade at the shed.
- **Girth.** At the settle, its root radius joins its bearer's girth by the bearer's exponent, and its pipes enter the leaf term's mean.

### Choices the code forced, each small and marked in the patch

1. A cohort joins its bearer at its unit's middle node. Its leaf deposits and its light reads at its unit's mean node position.
2. Its light enters the bearer's balance sum. It is not entered in the vigour split (`allocation.rs`), which has no bud for it. So the split routes no vigour to cohorts, where the cycle routed some to their buds.
3. Sleeping buds of a cohort age are refused by name. The oak, spruce and beech hold none at 0.5 m, so the probe never meets one.
4. A cohort's living enters its bearer's fade in whole cycles from the bearer's birth, alive at the tree's age at the bearer's presence.
5. A cohort on an axis the balance sheds takes light 0 from then on. Its survival is then 0, since every cohort age's viability is under 1.
6. **The leaf term's mean.** Every phytomer a cohort grows enters the mean at its root age's pipe and at the light of the year it grew in. Its root radius is then lit by its last light. Without this, the oak's trunk read 19% under the cycle's at 40, because the tree-wide mean held only the structure. With it, the trunk reads 5% over on seed 1001.
7. `Phytomer::size` is made public for the read alone.

None of these changes what a tree is beyond what the table below shows. The sanity run on one oak at 40 grew leaf, a living crown and no refusal: height 10.45 m against 10.46, leaf 125.7 m² against 121.6.

### The read

Median over seeds, with the range; the last column is the probe over the cycle per tree. The full tables are `R4-TABLES.md`.

| species, age | height | spread | trunk | limbs | current-year leaf | leaf area index | new shoots, weighted |
|---|---|---|---|---|---|---|---|
| oak 40 | 0.999 | 1.023 | 1.115 (1.05–1.17) | 0.815 | 1.086 | 1.102 | 0.914 |
| spruce 40 | 1.000 | 0.993 | 1.006 | 1.000 | 1.031 (standing 1.048) | 1.063 | 1.040 |
| beech 40 | 1.000 | 1.000 | 1.055 | 0.806 | 1.009 | 1.046 | 1.008 |
| oak 80 | 1.018 | 1.004 | **1.359** (1.33–1.69) | 0.980 | **1.206** | 1.184 | 1.073 |
| oak 120 | **1.112** | **1.150** | **1.906** (1.68–3.66) | 1.076 | **1.893** (1.64–3.79) | 1.341 | **1.621** |

Absolute values, in the cycle's units:

| tree | trunk diameter, m: cycle / probe | leaf, m²: cycle / probe | height, m: cycle / probe |
|---|---|---|---|
| oak 40 | 0.79 / 0.89 | 183 / 197 | 10.5 / 10.5 |
| oak 80 | 1.50 / 2.04 | 585 / 684 | 17.7 / 18.0 |
| oak 120 | 1.50 / 3.36 | 793 / 1,783 | 24.9 / 26.4 |
| spruce 40 | 0.36 / 0.36 | 82 (standing 442) / 85 (463) | 11.8 / 11.8 |
| beech 40 | 0.32 / 0.34 | 31.6 / 32.0 | 10.1 / 10.1 |

- **At 40 the probe grows the cycle's tree.** Height, spread and trunk are within 6% on the spruce and beech, and the oak's trunk is 11% over. Leaf is within 9%.
- **Limbs read 0.81 on the oak and beech.** fn-323's limb count names limb systems by branch order read from the tips. A tree without its twig axes reads its orders differently. Inferred, not isolated.
- **The oak drifts with age.** Its trunk is 1.36 times the cycle's at 80 and 1.91 at 120. Its leaf is 1.21 and 1.89 times, and at 120 its height is 11% over and its spread 15% over. The spread across seeds widens too: at 120 the trunk runs 1.68 to 3.66.
- **The drift's cause is not isolated.** Choice 2 (no vigour to cohorts in the split) and the cohorts' leaf deposited at one point per unit are candidates. Neither was tested; the goal's "averages are not dynamics" names this risk.
- **Work.** The probe grows 4 to 17% of the cycle's apex-cycles in the last year (oak 4.7%, spruce 17.4%, beech 5.9% at 40; oak 4.2% at 80, 6.9% at 120). It grows 7 to 19% of the cycle's phytomers. It steps its cohorts, about 19,500 classes on the oak at 40 (169,000 class-years over its life), 87,000 on the spruce, 4,400 on the beech, and 194,000 on the oak at 120 (6.4 million class-years).
- **Where the probe's time goes.** Census phases on seed 1001, one thread; the cohorts' yearly step is the `Grown` stage.

  | tree | cycle total | probe total | probe's cohort step | probe without it | cycle over the probe without it |
  |---|---|---|---|---|---|
  | oak 40 | 0.941 s | 0.252 s | 0.172 s (68%) | 0.080 s | 11.8 |
  | spruce 40 | 4.620 s | 1.421 s | 0.446 s (31%) | 0.975 s | 4.7 |

  As written, the law recomputes each cohort's expectation from its birth every year (`law.rs`, `Unit::member`). On the oak that step takes two thirds of the probe's time. This is measured, and the verdict reads the probe as it stands.

**Peak memory**, one process per tree, seed 1001, with fn-330's `peak.py`. Each process grows the tree twice, once for the census and once for the pipeline's shape.

| tree | cycle, GiB | probe, GiB |
|---|---|---|
| oak 40 | 0.29 | 0.04 |
| spruce 40 | 1.36 | 0.34 |
| beech 40 | 0.07 | 0.02 |
| oak 80 | 1.31 | 0.22 |
| oak 120 | 2.26 | 0.56 |

**Stills,** whole view, seeds 1001 and 1007, with the cycle beside the probe: `sheet-r4-oak.jpg` (40, 80, 120), `sheet-r4-spruce.jpg` and `sheet-r4-beech.jpg`. They were drawn with fn-336's still examples (`probes/r4-stills.sh`, `probes/r4-sheet.py`).
- **The probe draws its wood and the structure's own leaves.** The expansion seats leaves on axes, and a cohort has no axis, so its leaf is counted and not drawn. Drawing it needs twig axes built from the cohorts, the third proof's work. Each sheet says so.
- **What the worker saw, an observation and not a verdict:**
  - At 40, each probe tree has the cycle's frame, bare where the cycle's twigs are.
  - At 120, seed 1007, the probe's oak is a different tree: a broad, dense crown of thick limbs on a flared base, where the cycle's leans to one side.

## R6, verification

- No product behaviour changes. The probe stays a patch: `probes/coupled.patch`, which holds R3's `probes/replay.patch` and the coupled grower.
- The crates end as they began. No crate change stays, so no gate runs, and the codex review is skipped with that reason.
- The probe's law test passed. Its replay check read the cycle's light to the bit at every apex.

## Friction

Every entry is in `FRICTION.md`, each with its cost and what would have removed it.
