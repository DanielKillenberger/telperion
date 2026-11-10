# fn-346: the host's answers to RENEWAL.md, 11 Nov 2026

Each is the host's design decision; none is the owner's ruling.

## What the measurement settles

- The crown is not renewed because of two things in today's rules, both written for a tree that is getting bigger, and neither is the charge (question 5: yes, the hollowing is the stills' bare limbs, and it is a failure of renewal):
  1. **No fine tip lives on in place.** A twig ends at 5 years and a short shoot at 3, with no successor, and leaves grow only on the year's wood. So every leaf depends on a framework axis laying new nodes that year. A crown that has stopped extending cannot keep its leaves.
  2. **Nothing answers light.** Light returns where the crown empties (0.61 against 0.33; 0.94 against 0.65) and no bud reads it. Sleeping buds are on the limb row only, wake on a timetable and expire at 16 years.
- Today's tree fails the same bar (question 6): it is renewed only by getting bigger. The bug hid that until 132.
- Question 1: yes, a steady state is read in framework axes held as well as tips. Question 2: it stays as it is; a branch the balance sheds did not pay. Question 4: not needed; a framework that slows as it stops paying is what a bounded tree does.
- **`L_max`.** The host's reading of it was wrong by the probe's own numbers. At `L_max` = the potential height the oak's crown starves at two thirds of that height, because a crown stands in about half light, not in full light: at 25 m of path a tip would need 0.72. The doubled instrument stopped at 41.6 m with its limbs alive from 4.7 m. So `L_max` is restated as **twice the species' potential height**, one factor for every species: the path at which a fully lit tip breaks even is about twice the height at which a clothed crown does. The factor 2 is the model's and is listed as an estimate for the owner; it is not moved again in this child.

## The two rules of renewal, each from the same economy as the charge

**A. A fine tip lives on while it lasts.** A twig at the end of its row moves on to the short-shoot row, as a branch already moves on at the end of its own. The short-shoot row has no end of life by a timer: a short shoot goes on year after year, making its few nodes and its leaves, until its survival draw fails or the balance sheds it. Nothing else about either row changes (nodes a year, internodes, survival, laterals).

**B. A sleeping bud wakes where a tip would pay.**
- Sleeping buds are held on the limb, bough and branch rows, at the limb row's existing values (0.1 per bud place, the 6-year delay, 0.04 a year). They do not expire while their bearer lives.
- A bud's yearly chance to wake is the existing rate times max(0, 1 − cost / light), where light is the lattice light at the bud's place and cost is what a tip there would be charged, its path over `L_max`. Where a species has no `L_max` the cost is 0 and the factor is 1, which is today's rule.
- A sleeping bud does not age while it sleeps: it wakes as the row it was laid as, whenever it wakes. (Corrected the same day on the worker's stop: today's `dormant::aged` ages a bud through its rows as it sleeps and gives none after 16 years, which contradicts a bud that does not expire. A dormant bud is resting tissue, and the shoots from one on an old oak are vigorous new branches.)

Use preset values where the preset can already say it (a row's life, what a row moves on to, which rows hold sleeping buds) and the probe's own code where it cannot; say which was which.

## The run: an ablation on one base

Base **Q** = P2 with `L_max` doubled (the charge by size and path, dead wood nothing, the main line tested). Then **QA**, **QB** and **QAB**. Probe kept as `probes/renew.patch`; the crates end as they began.

1. **Census to 300,** oak seed 1001, the four variants, stopping itself as before: height, spread, held phytomers, living tips by row, framework axes held (limbs, boughs, branches), lowest living limb, leaves, income and charge, births over deaths, sleeping buds held and woken, every 10 years.
2. **Stills,** today's beside the four variants: the oak leafless and leafed at 40, 80, 120, 200 and 300 on seed 1001, one sheet per view. A variant that did not reach an age has no still there.
3. **For QAB only:** the oak at 120 and 300 on seed 1007; the beech at 40 and 80; every other preset at its preset age; with the measures table as before, peak memory and one-thread seconds.
4. **Your plain read per tree and variant:** better, the same or worse than today's, and what is wrong where it is worse: which part, which age. Look in particular at whether the crown is clothed through its depth or only at its ends, whether the bole and the lower limbs grow a thicket of new shoots, whether short shoots pile into knots, whether a lost limb leaves a gap that refills, and whether the oak at 40 and 80 has moved.

If QAB outgrows the census stop, return with its curve and the stills it reached.

About 2.5 hours. Result as `RENEW-RESULT.md` with the sheets as JPEGs. Return then. The host views every sheet before anything else happens.
