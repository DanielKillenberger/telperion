# fn-341 R3: the rules run alone, then added, on one oak; the stop U calls

Drafted by the worker on 10 Nov 2026, following `R2-ANSWERS.md` "How it is run" and `R3-G-ANSWERS.md`.

**U stops the run.** With upkeep charged on living wood only, the oak at 300 cannot be grown within 24 GB: it is killed at that cap whether U runs alone or with G and P. At 120, U holds twice the cycle's structure. R2-ANSWERS's return condition, "a tree fails to grow", applies, so the worker returns before R4's sheets. G and P measured in their paper directions.

## How

- **The patch.** `probes/earned.patch` holds G, P and U, each switched by the environment the space crate reads at every grow (`girth/rings.rs`, `from_env`), plus the measuring tests.
  - The runs are `probes/r3-runs.sh`, with logs in the ignored `raw/r3-*.log`. The oak at 300 was run directly through fn-330's `peak.py` (`raw/r3-oak-300-*.log`).
  - Every figure below is the oak, seed 1001, on four threads, unless said otherwise.
- **The traits.** As `R3-G-PAPER.md`, the nearest measured values and not the species' own.
  - G: R from Phillips' two *Q. garryana* groves, read at the tree's height; N from European oak's sapwood rings by age (16, 20.4, 25.9).
  - Primary wood under the rings: each phytomer's `established` pipe, an existing setting and an estimate.
  - P: eq. 39's coefficient 1,241 m, from Alméras and Fournier's Fig. 8 angiosperm means, an estimate for oak. H is the straight chord from a section to the far tip of what it bears.
  - U: a closed subtree (no living apex or bud below it) costs its bearer no upkeep. Living wood is charged per metre as before, so its whole length counts as living wood.
    - The worker read the host's "sapwood volume" this way because the species' `upkeep` is per metre, and a volume needs a reference section no source gives. This is the worker's reading, for the host to confirm.
    - Ryan 1990 ("linearly related to live-cell volume") and Ryan et al. 1994 ("linearly related to sapwood volume") state the physiology. No text read says outright that heartwood does not respire.
- **The ring step.** Each year a copy of the tree is shed and scaled, unlaid, and every section lays its ring by phytomer key. The ring settle skips the as-grown pass (shed pipes, leaf mean), which the rings do not read; the structure is the same.
- **Carrying the leaf up the year loop instead was not attempted.** The time went to finding U's failure; the cost is the probe's, not the rule's (R3-G-ANSWERS item 7).

## Each rule, measured against its paper direction

| step | measure | the cycle | measured | paper direction (R2) | agrees |
|---|---|---|---|---|---|
| G alone, 80 | trunk at 1.3 m | 1.10 m | **0.588 m** | down, to 0.589 m (R3-G-PAPER's sum) | yes, to the millimetre |
| G alone, 80 | the fork's three limbs | 0.53, 0.71, 0.39 m | 0.27, 0.25, 0.24 m | down | yes |
| G alone, 80 | the stated limb's chord from vertical | 41° | **50°**; height 17.4 → 15.3 m; spread 16.1 → 18.1 m | none written: today's sag bends the thinner limbs more (curvature ∝ moment / r⁴) | — |
| P alone, 80 | the stated limb's chord | 41° | 35.5° (base 22°) | toward its growth angle | yes |
| P alone, 80 | crown spread | 16.1 m | 15.7 m; height 17.6 m | down | yes |
| G and P, 80 | the stated limb's chord | 41° | **18.5°** (base 22°) | toward its growth angle | yes |
| G and P, 80 | spread, height | 16.1, 17.4 m | 15.7, 17.6 m | spread down | yes |
| G and P, 120 | trunk, spread, height | 1.43, 25.9, 24.9 m | 0.92, 23.6, 26.2 m | trunk down; spread down | yes |
| U alone, 120 | lowest living limb; crown ratio | 4.72 m; 0.81 | 4.72 m; 0.82 | lower crown base | **unchanged**: the bole below the fork is bare under every rule |
| U alone, 120 | living limbs from the stem | 3 | 4 | more kept | yes |
| U alone, 120 | axes held; phytomers | 344k; 2.37M | **670k; 4.63M** | not on paper | twice the cycle |
| G, P and U, 120 | trunk, spread, crown ratio, limbs | 1.43 m, 25.9 m, 0.81, 3 | 0.99 m, 26.5 m, 0.82, 4; 146 s, peak 9.9 GB | as above | as above |
| U alone, 300 | grows? | 86 s on four threads (R1) | **killed at the 24 GB cap after about 3 minutes** | — | the tree fails to grow on this desk |
| G, P and U, 300 | grows? | as above | **killed at the 24 GB cap after about 7 minutes** | — | the same |
| G and P, 300 | trunk at 1.3 m | 1.20 m | **1.53 m** | 1.54 m (R3-G-PAPER's sum) | yes |
| G and P, 300 | height | 40.0 m | **51.9 m** | none on paper. 1.49 times *Q. garryana*'s potential maximum of 34.8 m and 1.42 times its record | away from the range |
| G and P, 300 | spread | 30.4 m | 27.5 m | down | yes |
| G and P, 300 | bare bole; living limbs of the limb age | to 25.5 m; 1 | to 33.0 m; 1 | — | the bole is longer |
| G and P, 300 | time, peak | 86 s; not measured | 370 s; 6.65 GB | — | grows |

**G and P grow the oak at 300**, at 4.3 times the cycle's time. The trunk falls where the paper sum put it. With P, wood that can hold its lean holds it, and at the oak's girth that is nearly all of it, so the oak at 300 stands 12 m taller than the cycle's. That moves it away from every published height. No rule here limits height, and the goal's height rule is out of this probe (R2-ANSWERS).

## What U does, as read (inferred from the code)

Today a closed subtree's upkeep stays frozen in its bearer's balance (`Working::closed`, `retire.rs`). Under U it is zero, so a lateral's remembered balance no longer falls as its dead twigs pile up. The balance hazard (`balance_hazard` 2.0 on the oak) then sheds less, and the laterals it would have shed go on growing.

That is the rule's own direction for dead wood. But in this cycle the frozen upkeep is what bounds the oak's held structure. At 120 the held structure doubles; at 300 it outgrows 24 GB.

**U's failure is in growth, not in the crown.** The crown base does not move at 120 under any rule. The bole is bared by the trunk age's timer and the sprigs' lifespan, which none of the three rules touches.

## For the host

Not decided here:
- whether R4 runs without U;
- whether U's reading (closed wood free, living wood per metre) is wrong, and what bounds the held structure under it;
- whether the oak at 300 is drawn under G and P alone.

## Time, as measured (four threads, one seed; the one-thread times of R4 not run)

| oak, seed 1001 | cycle | P alone | U alone | G alone | G and P | G, P and U |
|---|---|---|---|---|---|---|
| 80 | 6.3 s | 7.5 s | — | 25.4 s | 26.2 s | — |
| 120 | 20.5 s | — | 30.4 s | — | 83.4 s | 146 s (9.9 GB) |

G's ring step settles a copy every year, so it multiplies the grow's time by 4 at 80 and 120. That is the probe's cost, not the rule's (FRICTION.md).
