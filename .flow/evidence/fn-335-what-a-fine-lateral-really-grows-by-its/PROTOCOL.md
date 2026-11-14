# fn-335 protocol, frozen by the host on 9 Nov 2026 after the dry run

The worker's draft and its dry run are `PROTOCOL-DRAFT.md`. The dry run chose nothing here but wording, with one exception stated plainly: it showed that a lateral grows close to its tables' potential over its own span (median 0.43 to 1.21 on one seed) and that the settle then drops 55 to 66% of the finest laterals. So the record must also hold what a subtree still carries at the tree's age, which is what a coarse node has to stand in for. The spec's R2 asked for what the subtree grew; the standing quantities are added to it, none removed.

## The host's answers to the draft's questions

1. **Fine ages.** Every lateral age is recorded. The verdict reads the ages fn-334's reach rule coarsens at 1.0 m (s above 1), computed for all four presets by that rule from fn-334's patch, the beech included.
2. **Light at birth.** The first unit's light. A lateral that grew no unit takes its bearer's light at its node. Both are kept in the record.
3. **Laterals with a span that grew nothing.** Read, at share 0, since the tables' expectation counts them.
4. **Span 0.** Counted, named and left out.
5. **Realised.** Both. Grown: everything the subtree grew before shedding. Standing: what the subtree holds at the tree's age after the settle's shed, summed by `scale` over the laterals the settle keeps.
6. **Weights.** fn-334's, counted for grown and scale-summed for standing: phytomers 1, leaf area `leaf_area`, section `pipe^exponent`, wood length `internode`.
7. **By year.** One addition only: the phytomers grown in the tree's last cycle, grown and standing, since a node bears its leaf in its year. Standing leaf is read from those.
8. **The bearer's height.** Dropped; no new hook. The bearer's age and the lateral's branching order stand for position.
9. **Bins.** Light: five equal-count bins per preset and lateral age, cut on seeds 1001 to 1006, with the raw range reported. Span: 1 to 8 each, then 9 to 16, then 17 and over.
10. **Seeds.** 1001 to 1012. Records are reduced to cell sums as they are read, and raw record lines are deleted once reduced; at most 2 GB of raw output at a time.

## The read

Oak, spruce, birch and beech at their preset ages, full cycle, one thread a process through `telperion-heavy`.

A cell is (preset, lateral age, span bin, light bin). For each cell: the laterals, and the mean and the 10th, 50th and 90th percentiles of four per-lateral values: grown leaf share (grown leaf over the potential's leaf for its age and span), standing leaf (last cycle's standing phytomers times leaf area), standing section and standing wood length, each also as a share of the potential's grown value.

## The verdict rule

A stand-in has to give a coarse node the right amounts, so the rule tests prediction of amounts, not per-lateral fit, which a branching process makes noisy by nature.

- **Fit and test.** Cell means are taken from seeds 1001 to 1006. For each tree of seeds 1007 to 1012, each fine lateral is given its cell's mean, and the tree's totals of standing leaf, standing section and standing wood length over its fine laterals are compared with the tree's own. The error of a preset and quantity is the median over the six trees of the absolute relative error.
- **By position.** The same totals split by the bearer's age, each group holding at least 5% of the tree's fine laterals, with the worst group's error reported.
- **Three predictors**, each read the same way: span alone; span and light; span, light and branching order.
- **One curve.** For each preset, cells are taken as shares of the potential from the other three presets' seeds 1001 to 1006, pooled by span bin and light bin over the fine ages, and the same totals are predicted.

Verdict for a stand-in read from local state, on the best of the three predictors:

- **go**: every preset's whole-tree error is at most 5% on all three quantities, and its worst position group at most 10%.
- **near**: whole-tree at most 10% and worst group at most 20%.
- **no-go**: past that on any preset.
- **unresolved**: a quantity cannot be read, or fewer than six test trees grow.

Reported beside it, each by the same thresholds and none changing the verdict: whether span alone already reaches the verdict (then no light is needed and a calculation may do), and whether one curve reaches it (then no value per species is needed).

A mistake in the probe's code is fixed and the read repeated. Nothing above changes after the first read that counts; a further question is a new child.
