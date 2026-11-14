# fn-340 result: the cohort law carried forward, the first proof's second read

Drafted by the worker on 10 Nov 2026. The work behind it:
- R1 and R2: `R1-R2-NOTE.md`, answered in `R1-R2-ANSWERS.md`;
- the balance trace: `R1B-TRACE.md`, answered in `R1B-ANSWERS.md`;
- the ledger: `R1C-LEDGER.md`, answered in `R1C-ANSWERS.md`.

R3 and R4 follow below. The verdict is the spec's R5 rule as written. Nothing further is proposed.

## R5, the verdict

**The approach stops by the goal's reworded rule.** The oak is five times cheaper at none of 40, 80 and 120. Its median trunk is not within a tenth of the cycle's at any of them. Its spread and current-year leaf are not within a tenth at 120. Either failure alone stops it, and both fail.

**1. Five times cheaper?** These are one-thread seconds by fn-339's method: five alternating batches per seed on one pinned core, each grow over 0.5 s run once.

| oak, age | seeds | cycle median s | probe median s | cycle over probe, median per seed (range) | five times? |
|---|---|---|---|---|---|
| 40 | 1001 to 1012 | 1.556 | 0.407 | **3.88** (3.40 to 4.49) | no |
| 80 | 1001 to 1006 | 22.30 | 8.95 | **2.49** (2.14 to 3.18) | no |
| 120 | 1001 to 1006 | 76.17 | 29.16 | **2.63** (2.39 to 2.79) | no |

At 80, seeds 1001 to 1003 were timed again on a quiet machine (load average 2), because the first run met an outside load of 32 (`FRICTION.md`). Seeds 1004 to 1006 are from the first run, whose batches agree within 16%.

**2. Within a tenth of the cycle?** Each figure is the probe's median over the cycle's median, with the median of the per-seed ratios in brackets.

| oak, age | height | spread | trunk | current-year leaf |
|---|---|---|---|---|
| 40 (12 seeds) | 1.004 (0.999) | 1.014 (1.013) | **1.103** (1.135) | 1.058 (1.073) |
| 80 (6 seeds) | 1.013 (1.010) | 0.967 (0.981) | **1.346** (1.361) | 1.012 (1.054) |
| 120 (6 seeds) | 0.981 (0.999) | **0.875** (0.857) | **1.212** (1.258) | **0.860** (0.812) |

- The trunk misses at every age: by a hair at 40, and by a third at 80.
- At 120 the spread and the leaf also miss, both on the low side.
- The same cells miss whether "median" is read as the ratio of the medians or the median of the per-seed ratios.
- **Reported beside the rule, not entering it:** the spruce and the beech at 40 are within 2% of the cycle on height, spread, trunk and leaf, except the beech's trunk at 1.07. Each is 2.9 to 3.0 times cheaper.

## R3, the probe as corrected and carried

`probes/coupled2.patch` applies at `577831cf0` (the crates are unchanged since fn-339's `d2baa9508`). It holds fn-339's coupled probe with the following, each made because the cycle's own rule implies it:

1. **The split (S1a, R1-R2-ANSWERS answer 1).** Each cohort class stands in the vigour split as a lateral at its unit's middle node. Its weight is its expected living apexes' lit presence, and its share is let go. On paper this is dormant at λ 0.5 (the spruce and the beech). Measured, it moved the oak by 2% or less (`R1-R2-NOTE.md`).
2. **Frozen upkeep (R1B-ANSWERS answer 1).** A class that stops standing leaves its wood's upkeep on its bearer for as long as the bearer is held. That is the cycle's rule for a closed subtree (`retire.rs`: its "upkeep stays frozen in its bearer's balance"). Marked choice: the frozen wood is the most wood the class stood with.
3. **Retirement reads the cohorts (R1B-ANSWERS answer 2).** A closed subtree's living, when retirement judges it, includes its axes' cohorts' living, as the settle's shed reads it. A subtree whose cohorts lived lately is not let go early. This also removed the settle panic at `geometry.rs:63` (`R1C-LEDGER.md`).
4. **The leaf-girth mean keeps the cohorts' pipes (R1-R2-ANSWERS answer 4).** As fn-339 had it.
5. **The carried state (R1-R2-ANSWERS answer 2, exact).**
   - `law/carried.rs` carries each class a year at a time: its chance of having grown each unit, its balance memory, its units' inner members (each carried the same way) and their recent living.
   - **One thing the note missed.** The recomputation reads a member's living at a cycle with the light of the cycle after, which settles its next unit's survival. So each carried member also gives its contents a cycle back, revised by this cycle's light, and the balance and the shedding window read those.
   - **The test.** Its unit test, `the_carried_state_is_the_recomputed_expectation`, holds the carried member equal to `Unit::member` within 1e-9 every year to 30. It covers the oak's twig (balance on) born at 0 and 4, and its short. It runs under a constant light of 0.26 and under a light that varies from 0.08 to 0.98. It passes.
   - **The probe is unchanged by it.** The oak at 40 on seed 1001 reads identically to the recomputing probe in every printed figure (`raw/sanity-r3.log` against `raw/sanity-c1.log`).

**The upkeep hour (R1C-ANSWERS item 2): no defect.** R1c's read had summed the probe's cohort upkeep on every bearer, closed ones included. The cycle's side was read on open structural bearers only, and the frozen upkeep sits largely on closed bearers.

| seed 1001, probe over the cycle | 20 | 30 | 40 | 60 | 80 | 100 |
|---|---|---|---|---|---|---|
| read as R1c read it, every bearer | 0.82 | 0.97 | 1.18 | 1.67 | 1.90 | 1.70 |
| read as the cycle is read, open structural bearers | **0.81** | **0.85** | **0.87** | **0.90** | **0.95** | **0.90** |

Read the same way on both sides, the probe charges 0.81 to 0.95 of the cycle's upkeep (`raw/trace-1001-u.log`). Nothing was corrected.

**Not corrected, named:** the law's open-loop bias (`R1C-LEDGER.md`, (c)). On the cycle's own tree and light it has a fifth fewer living tips than the cycle's twigs and shorts. On young wood it hands the lattice 5% more leaf at 40 and 17% more at 80. It has no standing twig wood on units past ten years. It is the one measured difference that rises with the tree's age without feedback.

## R4, the read

The full tables are in `raw/r4-tables.txt` (ignored), printed by `probes/r4.py`. The runs are `probes/r4-runs.sh`.

### The oak, probe over the cycle

Each figure is the median of the per-seed ratios, with the range in brackets. "Cycle spread" is the cycle's largest over its smallest across the same seeds: how much one oak differs from another.

| measure | 40, 12 seeds | cycle spread | 80, 6 seeds | cycle spread | 120, 6 seeds | cycle spread |
|---|---|---|---|---|---|---|
| height | 0.999 (0.994 to 1.007) | 1.08 | 1.010 (0.981 to 1.017) | 1.19 | 0.999 (0.946 to 1.029) | 1.13 |
| spread | 1.013 (0.954 to 1.177) | 1.40 | 0.981 (0.921 to 1.015) | 1.72 | 0.857 (0.691 to 0.987) | 1.53 |
| trunk | 1.135 (1.055 to 1.166) | 1.65 | 1.361 (1.224 to 1.542) | 2.28 | 1.258 (1.105 to 1.633) | 1.58 |
| current-year leaf | 1.073 (1.026 to 1.106) | 1.78 | 1.054 (0.888 to 1.094) | 2.40 | 0.812 (0.453 to 1.121) | 2.16 |
| leaf area index | 1.110 (0.996 to 1.133) | 1.40 | 1.157 (1.124 to 1.229) | 1.19 | 1.141 (0.652 to 1.219) | 1.74 |
| new shoots, weighted | 0.906 (0.854 to 0.954) | 1.65 | 0.856 (0.707 to 1.001) | 2.20 | 0.749 (0.423 to 1.024) | 2.04 |

**Absolute values, cycle / probe (medians):**

| oak, age | trunk diameter, m | current-year leaf, m² | height, m | spread, m |
|---|---|---|---|---|
| 40 | 0.79 / 0.87 | 183 / 193 | 10.5 / 10.5 | 8.3 / 8.4 |
| 80 | 2.04 / 2.74 | 912 / 923 | 19.4 / 19.6 | 23.6 / 22.8 |
| 120 | 2.00 / 2.43 | 1,005 / 864 | 27.5 / 27.0 | 28.4 / 24.9 |

What the oak's figures show:
- **At 40 and 80**, height, spread and leaf stay within the cycle's own seed-to-seed spread, and mostly within a few percent of the cycle.
- **The trunk runs 14 to 36% thick.** The cycle's trunk differs 1.6 to 2.3 times from seed to seed, so this is under one oak's difference from another, but it is a consistent bias.
- **At 120 the probe is narrower, and carries less leaf on four of six seeds:** 0.45 to 0.84 of the cycle's on those four, 1.11 and 1.12 on seeds 1001 and 1003. The cycle's own leaf differs 2.2 times across those seeds.

### The spruce and the beech at 40, seeds 1001 to 1012

Each figure is the median of the per-seed ratios, with the range in brackets.

| species | height | spread | trunk | current-year leaf | leaf area index | new shoots, weighted |
|---|---|---|---|---|---|---|
| spruce | 1.000 | 0.996 (0.994 to 0.996) | 0.982 (0.978 to 0.987) | 1.003 (1.000 to 1.007) | 1.010 | 1.002 |
| beech | 1.000 | 0.999 (0.922 to 1.001) | 1.055 (0.980 to 1.085) | 1.009 (0.993 to 1.038) | 1.045 | 1.008 |

Against R2's checked ranges, as fn-339 read them (a multiple of the nearer end; 1 is inside):

| species | measure | cycle | probe |
|---|---|---|---|
| spruce | new shoots a year | 27.3 | 27.3 |
| spruce | leaf area index | 1.15 | 1.17 |
| beech | leaf area index | 0.214 | 0.216 |

### Cost

**One-thread seconds** (fn-339's method):
- spruce at 40: cycle 4.495 s, probe 1.536 s, **2.99** times (2.87 to 3.16);
- beech at 40: cycle 0.247 s, probe 0.086 s, **2.94** times (2.40 to 3.22);
- the oak at each age is in the verdict above.

**Peak memory**, one process per tree and mode, seed 1001 (fn-330's `peak.py`). Each process grows the tree twice, once for the census and once for the pipeline's shape.

| tree | cycle, GiB | probe, GiB |
|---|---|---|
| oak 40 | 0.29 | 0.08 |
| spruce 40 | 1.36 | 0.37 |
| beech 40 | 0.07 | 0.03 |
| oak 80 | 1.33 | 0.39 |
| oak 120 | 2.18 | 0.61 |

**The share of the probe's time in the cohort step.** These are census phases, seed 1001, one thread; the cohort step is the `Grown` stage.

| tree | cycle total, s | probe total, s | cohort step, s (share) | cycle over the probe without the step |
|---|---|---|---|---|
| oak 40 | 0.987 | 0.204 | 0.100 (49%) | 9.5 |
| spruce 40 | 4.734 | 1.524 | 0.392 (26%) | 4.2 |
| beech 40 | 0.143 | 0.049 | 0.019 (40%) | 4.9 |
| oak 80 | 13.55 | 4.140 | 2.617 (63%) | 8.9 |
| oak 120 | 34.48 | 11.98 | 8.051 (67%) | 8.8 |

**Carrying the state did not make the step cheap.** fn-339's recomputing step took 68% of the oak's probe at 40; the carried one takes 49%.
- The oak's cohorts live about ten years, so the recomputation was never long.
- The carried member does about as much per year as the recomputation did:
  - it steps every inner member;
  - it evaluates itself twice, now and a cycle back;
  - it reads the shedding window.
- Even with the step free, the probe would be 8.8 to 9.5 times cheaper on the oak at seed 1001 (arithmetic from the phases). It is not free.

### Stills

**`sheet-r4-oak.jpg`:** the oak at 40, 80 and 120 on seeds 1001 and 1007, the cycle on the left and the probe on the right, whole view. It was drawn with fn-336's still examples (`probes/r4-runs.sh stills`, `probes/r4-sheet.py`).
- **The probe draws its wood and the structure's own leaves.** The cohorts' leaf is counted, not drawn, so the probe's crowns look bare. The sheet says so.
- **What the worker saw, an observation and not a verdict:**
  - At 40, and at 80 and 120 on seed 1001, each probe tree has the cycle's frame: the same fork, limbs and crown outline, bare where the cycle's twigs are.
  - On seed 1007 the probe's trunk is visibly thicker and flared at 80. At 120 its crown is upright and broad, where the cycle's leans to one side, so it is a different tree. fn-339's sheet showed the same on this seed.

## R6, verification

- **No product behaviour changes.** The probe stays a patch, `probes/coupled2.patch`, and the crates end as they began.
- **Its tests passed under the patch:** the carried-state test (`cargo test --profile ci -p telperion-space --lib carried`) and fn-339's law test.
- **No crate change stays,** so no gate runs, and the codex review is skipped with that reason.

## What was isolated and what was not

**Isolated (measured):**
- Retirement not reading the cohorts, together with the cohorts dropping their frozen upkeep, was most of fn-339's runaway at 120. Correcting both took seed 1007's trunk from 3.66 to 1.50 at 120, and removed the settle panic (`R1C-LEDGER.md`).
- The split, the leaf's place within its unit, and the pipes in the mean are not the drift (`R1-R2-NOTE.md`).
- The balance gap closed by a fitted factor leaves the drift (`R1B-TRACE.md`).
- The upkeep the probe charges, read as the cycle's is read, is 0.81 to 0.95 of the cycle's. The 1.1 to 2.3 times seen in R1c was a read that took every bearer.

**Not isolated:**
- the trunk's bias, 10 to 36% thick at every age;
- the narrower, leaner oak at 120;
- why branch tips in the probe stop in growth more often at more light (`R1C-LEDGER.md` (b));
- why the law's leaf and growth per living tip rise with the tree's age on the cycle's own tree and light (`R1C-LEDGER.md` (c)).

The last is the one measured difference that rises with age without feedback.

## Friction

Every entry is in `FRICTION.md`, with its cost and what would have removed it:
- the settle panic;
- the shell hook's variable redirect;
- the replay over 8 GB;
- the ledger's first year;
- the old oak's hour of timing;
- the outside load during the timing.

