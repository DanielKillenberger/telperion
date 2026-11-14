# The cohort law: the first proof

## Conversation Evidence

> goal, owner, 9 Nov 2026: "I want highest fidelity/performance ratio tradeoff. That's what we're maximizing. Realistic trees generated and rendered fast."
> goal, owner, 9 Nov 2026: "it just needs to be even better and grow larger trees more efficiently"
> goal, owner, 9 Nov 2026: "i don't think we want hand tuned rules. It's going against the engineering principles"
> goal R1: "The leafy shoots that cannot are carried as cohorts on the wood that bears them: whole numbers by age, drawn by lineage key, moved each year by the same species tables over a shoot's own life with the same hazards read at the cell."
> goal, the first proof: "The approach stops if the cohorts are no closer to real trees than the cycle and no cheaper by five times."

## Goal & Context

Child of goal fn-321, the first of its three proofs, serving R1, R3 and R5. The goal's approach counts the leafy shoots inside a cell of the light lattice as cohorts on the wood that bears them, where today's cycle grows each one bud by bud and sheds 65 to 98.6% of what it grows. Whether that law, written out, gives leaf and shoot numbers as near to real trees as the cycle's at a fraction of the cost is what this child settles before any part of the product is rewritten. Today's cycle is the instrument it is read beside, not a target to match. [paraphrase]

The law in outline, to be written exactly in R1 (the host's design, 10 Nov 2026):
- **What is a cohort.** A lateral whose whole subtree cannot outgrow a cell of the light lattice is not grown as an axis. It is counted on the phytomer that bears it, by its age on the reference axis and its years since birth.
- **Birth.** The laterals a bearing node would sprout, by its zone's buds and lateral probabilities, as today.
- **Ageing.** A count moves one year on; at its lifespan's end it stops or moves on by `continuation`, as the tables say, and what its own nodes bear joins the same stretch's cohorts.
- **Death and shedding.** Each year a count survives by `viability`, shaded by the cell's light to the species' `shade_hazard`, and what is dead drops after `shedding`.
- **Totals.** A cohort's leaf enters its cell of the lattice, its upkeep enters its bearer's carbon balance, and its section enters its bearer's girth.
- **Whole numbers.** Each step's count is drawn from its expectation by the bearer's lineage key.
[inferred]

Evidence in hand: a fine lateral grows 0.43 to 1.21 of its tables' potential over its own span and the settle sheds 55 to 66% of the finest (fn-335, one seed for the first, twelve for the second); growing only what outgrows a 0.5 m cell left 12,481 of the oak's roughly 240,000 axes (fn-334, D0 at 0.5 m); `closed_form` computes a bud's expected phytomers by age from the same tables (fn-334's patch holds `expected_bud_counts`). Evidence against: hand-written expectations ran 14 to 17% over on the oak with half its sheds (fn-245 COUNCIL.md, form (a)), judged against the cycle. [paraphrase]

## Acceptance Criteria

- **R1 (the law on paper, and back to the host):** a note in evidence writes each step above as a formula over the species' existing settings, names every setting it reads, and derives on paper, with arithmetic on the oak's and the spruce's own tables, what it predicts for one bearing node over ten years: shoots standing by age, leaf, section and wood. It says where the outline is ambiguous or cannot be written from the tables, and what the law leaves out. Which laterals count as cohorts is stated as a rule from the cell's size and each age's reach, with the oak's, spruce's and beech's ages listed under it at cells of 0.25, 0.5 and 1.0 m. No probe is built before the host answers this note. [inferred]
- **R2 (what real trees have):** a table, each number checked against its source's text, of published ranges for a pedunculate or Oregon white oak, a Norway spruce and a European beech near 40 and near 80 years where sources give them: leaf area index, leaves or needles, new shoots a year, shoot length, and the share of shoots shed a year. A number that could not be checked is marked unchecked. [inferred]
- **R3 (the replay, and back to the host):** on full trees grown by today's cycle with nothing retired (fn-335's recorder), oak, spruce and beech at 40 on at least six seeds from 1001 up: the law is run on each bearing phytomer of the wood that stays structural, with the cycle's own light there, and its cohorts' shoots, leaf, section and wood are set beside what the cycle grew on the same wood and beside R2's ranges, per tree and per cell of the lattice. The protocol's bins and thresholds are written after a dry run on one tree per species and before the read that counts. [inferred]
- **R4 (the coupled probe):** on the host's word after R3, a probe grows the structural wood through the cycle with its cell-confined laterals carried as cohorts whose leaf shades and whose upkeep is charged, for the same species and seeds at 40, and the oak at 80 and 120: the tree's height, spread, trunk, limb count, leaf area and leaf area index beside the cycle's and beside R2's ranges; stills of each at the whole view on one retained sheet beside fn-336's; one-thread seconds, buds grown a year, and peak memory. [inferred]
- **R5 (the verdict, by the goal's rule):** the result states, with the numbers, whether the cohorts are closer to real trees than the cycle on R2's measures, and how many times cheaper the coupled probe grows. The approach stops if they are no closer and no cheaper by five times; the result says so plainly either way and proposes nothing further. [paraphrase]
- **R6 (verification):** no product behaviour changes; the probe leaves the crates as a patch in evidence, as fn-334's and fn-335's did, and where any code stays the gate is green and the codex review ends SHIP. [paraphrase]

## Boundaries

- No value fitted per species, and none tuned to bring the cohorts nearer the cycle; a constant the law needs holds for every species and is stated with its reason. [paraphrase]
- Today's cycle is the instrument, not the target: no criterion here asks the cohorts to match it. [paraphrase]
- The structural wood's own rules (vigour, rank, posture, height) are the second proof's and are not changed here. [paraphrase]
- Nothing merges to master, nothing is tagged or published. Heavy work runs through `telperion-heavy`.

## Decision Context

Settled by the goal: one law run three ways; the boundary is the light cell; the host conducts and an Opus worker builds, stopping at any question about how the system should work; a rule's direction is derived on paper before it is built; a protocol runs on one tree per species before it is frozen and lists its bins one by one. [paraphrase]

Not settled, and the host's to decide on the worker's findings: how a lateral near the boundary passes from cohort to axis by degree; how a cohort member that would outgrow its cell is promoted; whether the survival settings in today's tables are themselves near real trees (the spruce grows about 50 to 70 times a measured spruce's shoots a year). The replay and the probe may use a plain boundary by an age's expected reach, with its sensitivity to the cell's size reported. [inferred]

## Open Questions

None.

## Result (host, 10 Nov 2026)

The worker's result is `.flow/evidence/fn-339-the-cohort-law-the-first-proof/RESULT.md`, with `R3-RESULT.md` and the sheets `sheet-r4-oak.jpg`, `sheet-r4-spruce.jpg` and `sheet-r4-beech.jpg`. The host read them and viewed the oak's sheet.

- **R5, by the goal's rule as written: the approach stops.** The coupled probe is no closer to real trees (spruce new shoots 28.4 times over the measured range against the cycle's 27.3; spruce leaf area index 12.9 against 12.0 and a measured 10.5; beech 1.21 against 1.20 and a measured 5.6 to 9.5) and is 3.2 times cheaper on the oak and spruce at 40, 1.95 on the beech, 2.5 on the oak at 80 and 1.8 at 120, where the rule asks five.
- **What the numbers do and do not show (host's reading, not a verdict).**
  - The first half of the rule could not have been met: the law reads the same species tables as the cycle, so it lands where the cycle does. The host wrote a rule whose first half tests the tables, not the cohorts.
  - The cost half read a probe whose law recomputes each cohort from its birth every year. That step is 68% of the oak's probe time and 31% of the spruce's; the rest of the probe runs 11.8 and 4.7 times under the cycle. Carrying a cohort's state forward a year at a time is not measured.
  - Peak memory fell 3.4 to 7 times, and the probe grows 4 to 17% of the cycle's yearly shoots.
  - The old oak drifts: trunk 1.36 times the cycle's at 80 and 1.91 at 120, leaf 1.21 and 1.89, and on seed 1007 at 120 it is visibly another tree. The cause is not isolated. The likeliest is that cohorts take no share of the vigour split, so the structure keeps what the twigs used to draw. This is the "averages are not dynamics" risk the goal named, and it is the result that weighs most.
- **R6:** no crate change stays; the probes are `probes/replay.patch` and `probes/coupled.patch`. No gate and no review, for that reason.

## Friction (host's dispositions)

- Shell hook refusals (redirect to a missing directory, a variable log path, a heredoc holding a Rust tuple type): the owner's local hook, reported and not specced. Later dispatches say to write files with the editor and to redirect only into existing directories.
- The wrong citation in the dispatch (Harmer for Buck-Sorlin and Bell 1998): the host's slip; R2-REAL-TREES.md carries the right source.
- Four missing doc lines, each of which cost a rerun: which cycle's leaves `Grower::year`'s light comes from; that the recorder's counts are unweighted axes; that `Phytomer::scale` holds the size once settled; that `leaf_girth` is normalised over the whole tree. They are doc comments in the crates, so they go in as a line of the next child that changes `telperion-space`, not as a gate run of their own.
