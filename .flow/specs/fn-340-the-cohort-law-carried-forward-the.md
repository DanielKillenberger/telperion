# The cohort law carried forward: the first proof, second read

## Conversation Evidence

> goal, owner, 9 Nov 2026: "I want highest fidelity/performance ratio tradeoff. That's what we're maximizing. Realistic trees generated and rendered fast."
> goal, owner, 9 Nov 2026: "i don't think we want hand tuned rules. It's going against the engineering principles"
> host to owner, 10 Nov 2026: "One more child on this proof (my recommendation). It would carry the counted groups forward a year at a time, give them their share of the vigour split, and re-read cost and the oak at 80 and 120. It continues only if the oak stops drifting and the cost passes five times."
> owner, 10 Nov 2026: "1"
> goal, the first proof, as reworded on the owner's choice: "The approach stops if the oak is not five times cheaper at 40, 80 and 120, or does not stay within a tenth of the cycle's at those ages."

## Goal & Context

Child of goal fn-321, the second read of its first proof, serving R1 and R5. fn-339's coupled probe grew the cycle's tree at 40 with 4 to 17% of its yearly shoots and 3.4 to 7 times less memory, but ran only 1.8 to 3.2 times cheaper and let the old oak drift: its trunk 1.36 times the cycle's at 80 and 1.91 at 120, its leaf 1.21 and 1.89. Two things in the probe are known to be wrong and neither was measured apart: the law recomputes each cohort from its birth every year (68% of the oak's probe time at 40), and cohorts take no share of the vigour split, so the structure may keep what the twigs used to draw. This child finds the drift's cause, corrects both, and reads the proof again. [paraphrase]

It starts from `.flow/evidence/fn-339-the-cohort-law-the-first-proof/probes/coupled.patch` and that spec's `RESULT.md`, `R1-LAW.md`, `R1-ANSWERS.md`, `PROTOCOL.md` and `R4-TABLES.md`. [paraphrase]

## Acceptance Criteria

- **R1 (the drift's cause, and back to the host):** on the oak, seeds 1001 and 1007, the probe as fn-339 left it and the cycle are read year by year to 120: structural apexes growing, the vigour the structural buds are allotted in sum, current-year leaf, and trunk radius. A note in evidence names the year the two part and which quantity parts first, and tests each candidate by switching it alone: the cohorts absent from the vigour split; their leaf deposited at one point per unit; their pipes in the leaf-girth mean. It states what was isolated and what was not. [inferred]
- **R2 (the share on paper, and back to the host):** the same note writes how a cohort enters the vigour split as the cycle's own split would treat the buds it stands for, from `allocation.rs` and the species' existing settings, and derives on paper, for one bearing unit with stated numbers, the direction and size of the change to the vigour reaching the structural buds beyond it. It also writes the cohort's yearly step as a state carried forward one year (what state a cohort class holds, how one year moves it, what light it reads), and says where that differs from the recomputed expectation and why. No code for R3 is written before the host answers this note. [inferred]
- **R3 (the corrected probe):** on the host's word, the probe carries each cohort's state forward a year at a time and enters cohorts in the vigour split as answered, with any further correction R1 isolated. A unit test holds the carried state equal to the recomputed expectation where light is constant. [inferred]
- **R4 (the read):** the oak, spruce and beech at 40 on seeds 1001 to 1012, and the oak at 80 and 120 on seeds 1001 to 1006, each as the probe over the cycle per seed, median and range: height, spread, trunk, current-year leaf, leaf area index and new shoots; one-thread seconds by fn-339's method; peak memory on seed 1001; the share of the probe's time in the cohort step. Stills of the oak at 40, 80 and 120 on seeds 1001 and 1007 beside the cycle's on one sheet, wood and the structure's own leaves, saying so. [inferred]
- **R5 (the verdict, by the goal's reworded rule):** the result states, with the numbers, whether the oak is five times cheaper at each of 40, 80 and 120, and whether its median height, spread, trunk and current-year leaf each stay within a tenth of the cycle's at each of those ages. The approach stops if either fails; the result says so plainly and proposes nothing further. The spruce and beech are reported beside it and do not enter the rule. [paraphrase]
- **R6 (verification):** no product behaviour changes; the probe leaves the crates as a patch in evidence, and where any code stays the gate is green and the codex review ends SHIP. [paraphrase]

## Boundaries

- No value fitted per species, and none tuned to bring the probe nearer the cycle; a constant the law needs holds for every species and is stated with its reason. A correction is made because the cycle's own rule implies it, never because it closes a ratio. [paraphrase]
- The structural wood's own rules (vigour across orders, rank, posture, height) are the second proof's and are not changed here; a cohort entering the split uses the split as it stands. [paraphrase]
- Whether the species tables are near real trees is not read here; that belongs to the presets on sourced traits. [paraphrase]
- Nothing merges to master, nothing is tagged or published. Heavy work runs through `telperion-heavy`.

## Decision Context

Settled: the owner chose a second read over stopping the approach (10 Nov 2026). The rule's tolerance of a tenth is the agreement threshold fn-339's frozen protocol already used; it is the host's choice and the owner may change it. The cost rule reads the oak because the goal's R5 is written on the oak; the spruce's bound by shoot count is 5.6 times at the 0.5 m boundary and 99.8% of its shoots are cohorts at 1.0 m, which is reported and left to the build's boundary question. [paraphrase]

Not settled, and the host's to decide on R1 and R2: how a cohort enters the split; what a carried cohort state holds; whether a cohort's leaf is spread over its unit's cells. [inferred]

## Open Questions

None.

## Result (host, 10 Nov 2026)

The worker's result is `.flow/evidence/fn-340-the-cohort-law-carried-forward-the/RESULT.md`, with the three notes and the host's three answers beside it and the sheet `sheet-r4-oak.jpg`, which the host viewed.

- **R5, by the goal's reworded rule: the approach stops, on both halves.** The oak's probe is 3.88 times cheaper at 40, 2.49 at 80 and 2.63 at 120, where the rule asks five. Its median trunk is 1.10, 1.35 and 1.21 times the cycle's, and at 120 its spread is 0.875 and its current-year leaf 0.86, where the rule asks a tenth.
- **What the child established (measured).**
  - Two corrections the cycle's own rules imply (a shed cohort's wood stays charged to its bearer; retirement reads the cohorts) removed most of fn-339's runaway: seed 1007's trunk at 120 went from 3.66 to 1.50 times the cycle's.
  - Height holds within 2% at every age. At 40 and 80 the spread and leaf hold within 7%. The spruce and beech at 40 are within 2% (the beech's trunk 1.07) at 2.9 to 3.0 times cheaper.
  - Peak memory is 3.4 to 3.7 times lower on the oak and spruce.
  - Carrying each cohort's state forward did not make the cohort step cheap: it is 49% of the oak's probe time at 40 and 63 to 67% at 80 and 120. The structure alone grows 8.8 to 9.5 times under the cycle on the oak (arithmetic from stage timings on seed 1001).
- **What it did not find.** The trunk's bias; the narrower oak with less leaf at 120; why the probe's branch tips stop more often at more light; why the law, on the cycle's own tree and light, gives a fifth fewer living tips and a leaf excess that rises from 5% at 40 to 17% at 80.
- **The host's errors.** Its three guesses at the drift's cause (the vigour split, a spread of sizes behind the balance, the balance itself) were each refuted by measurement. Its estimate that carrying the state would free the cohort step was arithmetic and was wrong.
- **R6:** no crate change stays; the probe is `probes/coupled2.patch`. No gate and no review, for that reason.

## Friction (host's dispositions)

- The settle panic was the probe's own defect and is removed by the retirement correction.
- The shell hook's refusal and the outside load during timing are the owner's machine, reported and not specced. The 80-year timings on seeds 1004 to 1006 ran under that load and are marked in the result.
- The replay does not fit 8 GB past 80, so the open-loop read at 120 is missing. No fix is proposed while the approach is stopped.
- The ledger carrying a previous run's last year was the worker's defect, fixed in the patch.
- An hour of one-thread timing for the old oak: later timing protocols state one batch for a grow over 10 s.
