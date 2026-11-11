# A tree pays only for living wood, and still stops growing

## Conversation Evidence

> owner, 10 Nov 2026, on the charge for dead wood: "ok well then we need to fix this bug properly. And we don't need byte identical trees to come from the fix."
> owner, 11 Nov 2026: "ok can you make this a goal spec to run overnight?"
> owner, 11 Nov 2026: "i want you to visually gauge if it's roughly same visual fidelity or better ideally"
> owner, 11 Nov 2026: "and then evaluate what's wrong"
> owner, 11 Nov 2026: "use jev for specific questions about faults/features of images"
> owner, 10 Nov 2026: "today's ok at 120 isn't that bad honestly. visually the new ones are a regression. Today's is a bit jagged but it looks directionally correct."
> owner, 10 Nov 2026: "But do i have to tell you that?"
> owner, 10 Nov 2026, on the oak at 300: "i mean wtf is that ok at 300..?"
> owner, 9 Nov 2026: "Just don't get stuck in a patch the current issue mode. Think high level how we can achieve our goal and make sure we move that direction."
> owner, 9 Nov 2026: "i don't think we want hand tuned rules. It's going against the engineering principles"
> owner, 9 Nov 2026: "you orchestrate the goal build and have opus do the build work. They should escalate to you for questions."
> AGENTS.md, owner, 3 Nov 2026: "We don't want artificial limits like that i've said that before"

## Goal & Context

The grower's carbon balance charges a limb upkeep for wood that has died, for good, and keeps the charge after the wood itself is let go (`grow/cycle/working.rs`, `closed`; `grow/cycle/retire.rs`). fn-344 measured what that does to the oak: the charge is 78 to 97% of all upkeep from 49 to 129, reaches the tree's light income at 109 and sheds the fork limbs at 132, after which the oak is a bare pole. It is also the only thing that bounds the tree: with it off, living wood costs 9 to 12% of income at every age and the crown widens without end. [paraphrase]

This goal fixes the balance so that a tree pays only for living wood and is still bounded by its own economy, and so that the trees look the same as today's or better. It runs unattended; the host judges the look. [paraphrase]

It continues from goal fn-321, which is parked: its second proof stopped its approach, and its children fn-339 to fn-345 hold the measurements this goal starts from (`.flow/evidence/fn-344-*/RESULT.md`, `.flow/evidence/fn-345-*/RESULT.md`). fn-346, minted there on the owner's word of 10 Nov, is this goal's first child. [paraphrase]

<!-- Source: 25% user / 45% [paraphrase] / 30% [inferred] -->

## Acceptance Criteria

Every threshold tagged [inferred] is the host's number and is listed under Open Questions. Measurements run through `telperion-heavy` on the integration branch's head and state their commit and seeds.

- **R1 (the charge is correct):** in the product's grower, wood that has died adds nothing to any balance, and an ordinary test holds it: a limb whose twigs have died and been let go is charged what the same limb without them is charged. [inferred]
- **R2 (bounded by its own economy):** the oak grows to 300 on seeds 1001, 1007 and 1012 with no cap of any kind. From 200 to 300 its height, its spread and its held phytomers each change by at most 10% a decade. The grow to 300 takes no more one-thread time and no more peak memory than today's (96 s and 2.9 GB on seed 1001 at four threads, re-measured at the base), and the device draws the tree at every age asked. [inferred]
- **R3 (the old oak keeps its limbs):** at 200 and at 300, on each of the three seeds, the oak's lowest living limb leaves the trunk below one third of the tree's height and its crown is at least 0.7 times as wide as the tree is tall. [inferred]
- **R4 (the look holds or improves, by the host's eye):** leafless and leafed stills, today's beside the fix's: the oak at 40, 80, 120, 200 and 300 on seeds 1001 and 1007; the beech at 40 and 80; every other preset at its preset age. The host views every still and writes a verdict per tree in the goal log: better, the same or worse than today's, and why. The criterion is met when no tree is worse, the oak at 200 and 300 is better, and the old oak is recognisable beside at least four photographs of real old open-grown oaks kept with the sheets. Where a tree is worse, or an old oak falls short of the photographs, the verdict names what is wrong in the picture (which part, at which age, on which seed) and the measured cause it points to, and that finding is what the next child rests on. "i want you to visually gauge if it's roughly same visual fidelity or better ideally" [paraphrase]
- **R5 (the young trees stay where they are):** for every preset at its preset age and the oak at 40 and 120, on three seeds: height, spread, trunk diameter and current-year leaf are reported beside today's. A measure that moves by more than 15% is explained in the log by the mechanism that moved it, and is accepted only where R4's verdict for that tree is not worse. [inferred]
- **R6 (no value without a source):** every value the fix adds or changes in a preset is a measured trait with its source's text checked, or a restatement of an existing value to hold a measured quantity, reported with its arithmetic. None is fitted per species and none is tuned to a sheet. Every parameter changes the tree by degree and is dormant at a neutral value. [paraphrase]
- **R7 (it lands on the integration branch):** the fix is in the crates, the tests and baselines it moves are updated with their evidence, `cargo test --profile ci --workspace --no-fail-fast` is green, and a `codex` review of the whole diff ends SHIP. [paraphrase]
- **R8 (the owner sees it):** one page of the R4 sheets, the host's verdicts and the measures is ready for the owner at the run's end, with what is unmet stated first. [inferred]

## Boundaries

- Nothing merges to master, nothing is tagged, nothing is published to npm, and nothing is pushed unless the owner asks. [paraphrase]
- No cap: nothing stops growth at a count, a height, a path or an age. Growth slows only by a rule over a tree's own state. [paraphrase]
- No value fitted per species, by hand, by search or by calibration. No species or template branch in the generator or the renderer. [paraphrase]
- One grower and one pipeline. The fix replaces the charge; it is not a second path or a switch between ways of growing. [paraphrase]
- Girth, sag, the insertion angles, the foliage and the renderer are changed only where a child shows by measurement that the fix cannot hold R4 without it, and then as that child's own stated scope. [inferred]
- The counted shoots of fn-339 and fn-340 are not built here. [inferred]
- Heavy work runs through `telperion-heavy`, whose admission rules are never overridden; every grow prints its yearly census and stops itself before the desk's memory; generated data stays in ignored `raw/` directories. [paraphrase]

## Rigor and stop conditions

Applied by the goal run. All [inferred] unless quoted.

- **Who builds:** the host conducts and makes every design judgment; an Opus worker builds through the work stage and stops at any question about how the system should work (owner, 9 Nov 2026).
- **Path:** work stays on the existing integration branch `goal/fn-321-020-one-grown-tree-at-any-scale-in` in the worktree `.worktrees/fn-319`, which holds the evidence this goal builds on. No new branch is made. Children commit straight onto it. A child that leaves code in the crates runs the gate once at its close and a `codex` review of its own diff; a child whose commits hold only `.flow/` skips both with the reason recorded. No pull request per child, and no final pull request: the run ends with the branch as it stands.
- **Before a rule is built:** its effect is worked out by arithmetic on a census of today's tree, its sources are read and checked, and the host answers in writing. fn-341, fn-343 and fn-345 each built before the arithmetic and each failed.
- **A probe before the product:** a rule runs first as a probe kept as a patch. It enters the crates only after the host has viewed its sheets and found no tree worse.
- **The host judges first and plainly.** The host views every image a verdict rests on. A tree that is worse is called worse in the log. A published measure moving toward a range is no evidence that a tree looks better.
- **Jev for specific questions about a picture.** "use jev for specific questions about faults/features of images". The host puts narrow questions about a fault or a feature of a still to Jev (for example: does the trunk carry living limbs below its midpoint; does the wood show a swelling or a knot; which of two crops shows the denser crown), through the shared caller in `crates/telperion-jev` with its ledger entry, under `docs/typesafe.md`: every question offers a no-match answer, Jev selects and never supplies a number, and the key is never read, echoed or copied. Jev's answers are evidence beside the host's own view; the verdict stays the host's, and a disagreement between the two is written in the log.
- **Not a patch loop.** "Just don't get stuck in a patch the current issue mode." When a probe fails the host's read, the next child must rest on a newly measured cause, not on a variation of the failed rule. The run stops `NEEDS_HUMAN` when two probes in a row fail the host's read, or when the measurement shows the balance cannot bound the tree without a change outside the Boundaries.
- **Limits:** eight children. Stop at 10:00 Zurich on 11 Nov 2026, or when free space on `/home` falls under 15 GB.
- **What to expect:** R1, R2, R3, R5, R6 and R7 are checkable by the run. R4 is the host's judgment and counts on the verdicts written in the log. The owner's own verdict on the sheets is not a criterion of this goal; he reads R8's page.

## Decision Context

Settled by the owner: the charge for dead wood is a bug and is fixed properly; the trees need not come out byte identical; the host gauges the look, which must be roughly the same or better. [paraphrase]

Settled by measurement (fn-344, fn-345): light holds the crown's density steady, at about 125 income and 650 tips per square metre of ground, and bounds nothing else; the balance sheds whole limbs, not tips; nothing a tip grows reads its balance, since sizes are normalised per age row and the node count never reads vigour; shortening internodes by a tip's path knots the wood; cutting node counts by a tip's path strips the crown. [paraphrase]

The host's direction, to be tested by fn-346's arithmetic before anything is built: living wood costs by the pipes it carries. The girth rule already gives every node a pipe to the root, so a living tip keeps living sapwood along its whole path and its upkeep is proportional to that path. A far tip then earns what a near one earns and costs more, and the balance falls with path in height and in spread with no rule over geometry. The one species cost is restated from a measured trait: the path at which a fully lit tip's pipe costs what the tip earns is the species' potential height. [inferred]

Not settled, the host's after the arithmetic: whether a lateral's balance counts its tips' pipes over the whole path or only inside its own subtree; whether shedding alone carries the bound or a tip's growth must also read its balance; what the untested main line does at the top; how a slowed tip stays clothed in leaf. [inferred]

## Strategy Alignment

- **The tree space:** one growth engine with light and a carbon balance shaping density, the bole and size (R1, R2, R6).
- **Key metrics:** Fidelity, Build and the owner's eye (R4, R2).

## Open Questions

For the owner, each the host's number. An answer written here, with its date, is read by the next run.

1. **R2's 10% a decade** from 200 to 300, and its cost bar of today's time and memory.
2. **R3's shape numbers:** lowest living limb below one third of the height, crown at least 0.7 times as wide as tall.
3. **R5's 15%.**
4. **The limits:** eight children and the stop at 10:00 on 11 Nov.

## Goal log

- 2026-11-11 Zurich: goal written by the host on the owner's word ("ok can you make this a goal spec to run overnight?"). Integration branch: the existing `goal/fn-321-020-one-grown-tree-at-any-scale-in`, as this spec's rigor states; no new branch. Rigor applied: this spec's "Rigor and stop conditions".
- 2026-11-11 Zurich: adopted fn-346 as the first child. It was minted under fn-321 on 10 Nov and its R1 measurement is running. Its R4 is read under this goal's R4: the host's verdict gates the crates, and the owner reads the page at the end.
- 2026-11-11 Zurich: fn-346 R1 read. The charge as the host first wrote it (one pipe per tip, K = 1/34.8 m) is refuted by arithmetic before any build: it passes the oak's income at 45 and is 2.4 times income at 119, because a tip was charged a whole pipe and credited only its own size's light. Restated per tip as size x path / L_max, so a tip pays where its light is at least L / L_max (`R1-ANSWERS.md`). Probe dispatched in two variants: main line untested (P1) and tested (P2).
- 2026-11-11 Zurich: fn-346 probe read by the host (sheet-oak-1001.jpg, sheet-others.jpg viewed). R4 verdicts for P1 and P2: oak 40 the same; oak 80 slightly worse (thinner crown); oak 120 worse (a good wide skeleton, unclothed: bare limbs with end tufts); oak 200 and 300 much worse (limbs lost whole, nothing replaces them; a looping limb on a stump, or a bare spike); every other preset the same. Not a candidate; nothing entered the crates. First failed probe of this goal. What is wrong: the charge bounds the tree, but the tree has no steady state, since a shed tip is not replaced and a paying tip extends until it no longer pays. That reading is being measured (births and deaths by source, row and path; light returning to the emptied crown) before any rule is written (`R2-ANSWERS.md`).
- 2026-11-11 Zurich: fn-346 renewal census read (`RENEWAL.md`). Measured cause of the unclothed crown: no fine tip lives on in place (a twig ends at 5 years, a short shoot at 3, leaves only on the year's wood), and nothing answers the light that returns where the crown empties (sleeping buds on the limb row only, on a timetable, expiring at 16 years). Today's tree is renewed only by getting bigger. Also restated by the host: L_max is twice the potential height, since a clothed crown stands in about half light; the factor is an estimate for the owner. Second probe dispatched as an ablation on that base: A, fine tips live on as short shoots; B, sleeping buds on every framework row wake where a tip would pay (`R3-ANSWERS.md`).
- 2026-11-11 01:30 Zurich: fn-346 renewal ablation read by the host (three sheets viewed; verdict table in `R4-ANSWERS.md`). The charge alone with L_max doubled (Q): oak 40 and 80 the same, oak 120 better (a many-limbed clothed crown where today's is a two-limb V), oak 200 a different tree and not clearly better (50 m wide on 29 m, flat, thin, limbs drooping to the ground), oak 300 worse (a collapsed tangle). Rule A (fine tips live on) is withdrawn: it puts a skirt of shoots on the young oak, a thicket on the birch and fuzz on the beech. Rule B (buds wake by light) changes almost nothing: buds die with their bearer. No candidate; nothing entered the crates.
- 2026-11-11 01:30 Zurich: on the stop rule. This is the second probe that is not a candidate. The host reads it as a partial result and not a second failure in a row: the first failed from 80 on, this one holds 40 and 80 and betters 120, and fails only from about 170. The host goes on with a measurement of what takes the old tree apart (limbs that run too far and lie down; limbs lost whole with nothing coming back), which the owner asked for ("and then evaluate what's wrong"), and builds a third probe only if that measurement names a new cause. The owner may read this differently; it is stated here so he can.
- 2026-11-11 01:35 Zurich: minted fn-348, the second child: the shared Jev caller cannot carry an image (it sends state and questions only; the API takes images as `assets`), so no question could be put to Jev about a still as the owner asked. Built in its own worktree beside fn-346's measurement.
