# Tree space: dormant buds wake along old branches

## Goal & Context

The spruce (fn-194) has the frame of a Norway spruce, but seven rounds and three Astra gates end on the same fault: "the boughs read as thin, flat shelves rather than substantial, drooping sprays with hanging comb branchlets". Values have been pushed as far as they go (needle retention, spur sprays, hang; RESULT.md rounds 5 to 7). What is missing is a mechanism, not a value. Along an old spruce bough, buds that formed years earlier and slept wake later and release new branchlets: the draperies that fill the curtain (MODEL-SPRUCE.md F4 and M10). Today every lateral is born in the year its node grows (`grow.rs`, `grow_unit`), so nothing can appear later along old wood. The same mechanism is how oaks and many broadleaves put out epicormic shoots along their limbs and trunk, so the oak (fn-195) can use it too (host, 2026-10-04).

## Design (host, 2026-10-04)

- **A dormant lateral per zone.** Each zone gains a second lateral table, `dormant`, of the same shape as `lateral`: the probability that a node carries a sleeping bud of each PA. Each sleeping bud also has a release law: `delay` (years before it can wake) and `rate` (a continuous yearly waking hazard after the delay). Neutral values (all probabilities 0) change nothing, and every existing tree stays byte-identical.
- **Draws keyed by lineage.** A sleeping bud's existence and its waking time are drawn from the node's lineage under a new key (`DORMANT`), never from a sequence. The waking time is continuous: delay plus an exponential draw at `rate`, from one uniform draw by the inverse CDF.
- **Waking by degree.** A bud wakes in the cycle its time falls in, and the fraction of that cycle left scales its first growth, as relays already blend (`blend` in `sprout`). Moving `rate` or `delay` therefore moves growth continuously; crossing a cycle boundary does not add a whole year at once (the fault fn-199 records for relays must not be repeated here).
- **Only on living wood.** A bud wakes only while its node is present and its axis alive; a bud whose bough has been shed never wakes. Its presence comes from the same pass as births (expected-wood windows), so it grows in by degree.
- **Closed form.** The closed form (`closed_form.rs`) adds the expected waking of dormant buds: the probability that a bud born at cycle t has woken by cycle c, times its expected subtree. The oracle against GreenLab (phase A) is unchanged at neutral; a new test checks the simulated mean counts against the closed form for a species with dormant buds, in distribution.

## Requirements

- **R1:** `dormant`, `delay` and `rate` as above, neutral at 0 with every existing tree byte-identical (beech stills, A's oracle, B's walks).
- **R2:** Lineage-keyed draws: a test that adding a sleeping bud elsewhere moves no other draw.
- **R3:** Walk tests on `rate`, `delay` and a `dormant` probability, within the bound of 30, with a test that is red first on a naive integer-cycle waking.
- **R4:** The closed form matches the simulation in distribution for a species with dormant buds (seeds 0 to 3,999, as phase A did).
- **R5:** One still strip of the spruce walking a dormant probability from 0 up on its main branches, viewed by the host.
- **R6:** Crate tests and the workspace gate green; Codex review.

## Boundaries

Engine only. The spruce's values are fn-194's work. No light-driven release (phase E may later make the hazard depend on light).
