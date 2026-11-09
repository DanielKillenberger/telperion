# fn-339 R3 protocol, the worker's draft for the host to freeze

Drafted on 10 Nov 2026 after the dry run (`R3-DRY.md`). Every threshold here is the worker's proposal, marked for the host to keep, change or strike. Nothing is read under it until the host freezes it.

## What runs

- **Probe.** `probes/replay.patch` at the frozen head: the law as answered (`R1-ANSWERS.md`), replayed on the full cycle with the cycle's own lattice. The patch is applied, run and reversed; the crates end as they began.
- **Trees.** The oak (`oregon-white-oak`), spruce (`norway-spruce`) and beech (`european-beech`) at their preset age, 40.
- **Seeds.** Proposed: 1001 to 1012. R3 asks for at least six; twelve takes about 60 s per species on this desk at four threads (the dry run took 0.2 to 4.2 s a tree). Seed 1001 is the dry run's and is read again with the rest.
- **Boundary.** 0.5 m. 0.25 m and 1.0 m appear only in the shares of units.
- **Runs.** Each species through `telperion-heavy 8 --threads 4`, one process at a time. Raw logs go in `raw/` (ignored); the reduced tables go in `R3-RESULT.md`.

## The measures, each read on the settled tree at age 40

Counts are weighted by presence: a member counts its settled scale on the cycle's side, and its expected presence on the law's.

1. **New shoots in the last year:** units grown in the 40th cycle.
2. **Apexes living** at the end of the 40th cycle.
3. **Members standing.**
4. **Current-year leaf**, m²: what the lattice takes.
5. **Standing leaf**, m²: what the expansion draws, by `leaf_years` and clusters.
6. **Wood**, m.
7. **Section:** each cohort root's `r²` (`Σ (pipe · scale)²` on the cycle), joined to its bearer as `r^e_bearer`.
8. **Whole-tree measures** for R2: new shoots (weighted and unweighted), standing leaf, and the leaf area index over the tree's leaf-bearing 0.5 m ground columns.
9. **Shares of units:** the cycle's units grown inside the cohort sets at 0.25, 0.5 and 1.0 m, at tree ages 10, 20, 30 and 40, unweighted.
10. **Supporting reads**, which decide nothing:
    - the law with the cycle's own births;
    - the cycle's cohorts at size 1 (leaf and wood);
    - roots shed by the balance hazard;
    - the lattice check, apex reads and their largest difference.

## The bins, listed one by one

- **Per tree:** totals over all bearing units of the structure.
- **Per light bin:** the light at the bearing unit's place in the 40th cycle, from the cycle's lattice, in five fixed bins, totals per bin:
  1. 0 to under 0.2
  2. 0.2 to under 0.4
  3. 0.4 to under 0.6
  4. 0.6 to under 0.8
  5. 0.8 to 1.0
- **Per lattice node:** each 0.5 m node holding a bearing unit's mean position. This is read and reported as the median ratio, the 10th and 90th percentiles, the share within 25%, and the correlation, over nodes where either side holds leaf. It decides nothing, because a node holds a few units on the oak and beech and its whole-number cycle is noisy beside an expectation.
- **Per species:** the median over the twelve seeds of each per-tree ratio, with its range.

## What counts as agreeing

**With the cycle** (proposed). The law agrees with the cycle on a quantity for a species when:
- the median over seeds of its per-tree ratio lies in 0.9 to 1.1; and
- in every light bin holding at least 5% of the bearing units, the median over seeds lies in 0.8 to 1.25.

The result lists every quantity and species that agrees and every one that does not, with its ratio. Agreement with the cycle informs R4 and is not R5's test: the cycle is not the target.

**With R2.** A measure agrees when its median over seeds lies inside the checked range of the same species in `R2-REAL-TREES.md`. Otherwise its distance is stated as a multiple of the nearer end. A measure with no checked source is reported and counts for nothing. The checked ranges at hand:
- **Spruce, new shoots a tree a year:** 691 to 2,372, from about 36-year-old closed-stand trees 15.5 m tall.
- **Spruce, projected leaf area index:** 10.5, a 28-year plantation.
- **Beech, leaf area index:** 5.6 to 9.5 one-sided (Leuschner 2006), and 7.1 and 7.4 optical means (Glatthorn 2017).

The oak has no checked leaf or shoot measure.

The index over a lone tree's own columns is not a stand's index. It is the nearest this tree has, and the result names that. The host may prefer to drop the index from the R2 comparison and keep the shoot count only.

**Which is closer** (for R5, read here only as a preview). For each measure with a checked range, it is stated whether the law's whole tree or the cycle's lies nearer the range, by the ratio to the nearer end.

## Known gaps the read will show and not resolve

- **Section on the spruce** (dry run 1.43): the law carries no per-phytomer presence, and a section is quadratic in it.
- **The oak's size:** at the mean size of 1 (answer 6), the law's leaf sits about 0.77 of the cycle's. The cycle's size-1 values are read beside it.
- **Living and standing under answer 2's shedding rule** run 0.70 to 0.90 of the cycle's.

## Questions for the host before freezing

1. Twelve seeds, or six?
2. The agreement thresholds above (0.9 to 1.1 per tree, 0.8 to 1.25 per light bin), or others.
3. Whether the leaf area index over a lone tree's columns enters the R2 comparison.
4. Whether section is read under the agreement rule, given its definitional gap, or reported only.
5. Whether R1's corrected speed bound (17.9, 14.7 and 5.5 times by unit count at 0.5 m) changes what R4 reads.
