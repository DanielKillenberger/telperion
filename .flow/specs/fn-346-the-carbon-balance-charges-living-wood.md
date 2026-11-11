# The carbon balance charges living wood by the pipes it carries

## Conversation Evidence

> owner, 10 Nov 2026, on the charge for dead wood: "ok well then we need to fix this bug properly. And we don't need byte identical trees to come from the fix."
> owner, 10 Nov 2026: "today's ok at 120 isn't that bad honestly."
> owner, 10 Nov 2026, on the oak at 300: "i mean wtf is that ok at 300..?"
> owner, 9 Nov 2026: "Just don't get stuck in a patch the current issue mode. Think high level how we can achieve our goal and make sure we move that direction."
> AGENTS.md, owner, 3 Nov 2026: "We don't want artificial limits like that i've said that before"

## Goal & Context

Child of goal fn-321. fn-344 measured the defect: a closed subtree's upkeep is frozen into its bearer's cost for good (`grow/cycle/working.rs`, `closed`), and stays there after the wood itself is let go (`grow/cycle/retire.rs`). On the oak that charge is 78 to 97% of all upkeep from 49 to 129, reaches the tree's light income at 109 and sheds the fork limbs at 132. With it off, living wood costs 9 to 12% of income at every age, the balance sheds almost nothing and the crown widens without end. [paraphrase]

fn-344's H and fn-345's N each put a rule over a tip's path in the charge's place and each failed the host's read: H knots the wood, N strips the crown. Both were geometry rules beside the balance, not a correction of the balance. [paraphrase]

This child corrects the balance itself. The owner has ruled that the defect is fixed properly and that the trees need not come out byte identical. [paraphrase]

## Acceptance Criteria

- **R1 (the measurement and the arithmetic, and back to the host):** a note giving, with no behaviour changed: what the balance charges today and in what units, from the code; what published work states about maintenance cost against living sapwood and about the pipe model's account of growth slowing with size, each claim checked against its text or marked unchecked; and, from a census of today's oak and of the oak with dead wood not charged, every living tip's path from the root and its own light by decade, with the arithmetic the Decision Context asks for. No behaviour is built before the host answers. [inferred]
- **R2 (the probe):** on the host's word, the corrected charge is built into the cycle, dormant at a neutral value. [inferred]
- **R3 (the read):** leafless and leafed stills, today's beside the probe: the oak at 40, 120, 200 and 300 on seeds 1001 and 1007, the beech at 40 and 80, every other preset at 40, with height, spread, trunk, lowest living limb, current-year leaf, held phytomers, peak memory and one-thread seconds, and the yearly census to 300. The worker views every still and says per tree: better, the same or worse than today's, and why. [inferred]
- **R4 (the host judges first):** the host views every sheet. The owner is shown the result only if the oak at 40 and 120 reads at least as good as today's, the oak at 200 and 300 reads as an old oak against photographs of real old open-grown oaks, and no other species is harmed. [paraphrase]
- **R5 (it lands):** where R4 passes and the owner accepts the trees, the correction stays in the crates, affected tests and baselines are updated with their evidence, the gate is green and the codex review ends SHIP. Where it does not pass, the probe leaves the crates as a patch in evidence. [inferred]

## Boundaries

- No cap: nothing stops growth at a count, a height or an age. A species value in the charge is a measured trait with its source, or is listed as an estimate with its reason. [paraphrase]
- No value fitted per species and none tuned to a sheet. [paraphrase]
- Girth, sag, the insertion angles and the foliage are not changed here. [paraphrase]
- Heavy work runs through `telperion-heavy`; every grow prints its yearly census and stops itself before the desk's memory. Nothing merges to master, nothing is tagged or published.

## Decision Context

Settled by the owner: the dead-wood charge is fixed properly; byte identity is not required. [paraphrase]

The host's direction, for R1 to test by arithmetic before anything is built: [inferred]

- Wood that has died costs nothing: only living tissue respires.
- Living wood costs by the pipes it carries. The generator's own girth rule gives every node a pipe from itself to the root. A living tip therefore keeps living sapwood along its whole path, and the upkeep it causes is proportional to that path. Today's charge is per metre of an axis and does not read how many pipes run through it, so the trunk under a whole crown costs what a twig of the same length costs.
- Under that charge the tree's whole upkeep is the sum, over living tips, of each tip's path times one species cost. A tip far from the root earns what a near one earns and costs more, so the balance falls with path by itself, in height and in spread, with no rule over geometry.
- The one species cost is restated from a measured trait: the path at which a fully lit tip's pipe costs what the tip earns is the species' potential height (oak 34.8 m).

Not settled, the host's after R1: whether a lateral's balance counts its tips' pipes over the whole path or only inside its own subtree; whether shedding alone carries the bound or a tip's growth must also read its balance; what the untested main line does at the top; how the young oak is held where it is today.

## Open Questions

None.

## Result

Closed 11 Nov 2026 as a measured finding with no code left in the crates; every probe is a patch under `probes/`. Now a child of goal fn-347.

- **The rule found (variant SR, `probes/square.patch`):** wood that has died is charged nothing; a living tip is charged its size times the square of its path from the root over the species' potential height; a lateral subtree's sums take only the child laterals that pay; an axis that continues or relays its bearer is tested as a lateral is. Under it the oak has a steady state for the first time: the longest path holds at 34 to 38 m from about 140, the height stops at 32 to 33 m, tips and held phytomers are level from about 220, and the grow to 300 costs 131 s and 2.2 GiB against today's 146 s and 2.7.
- **The host's verdicts for SR, against today's:** oak 40 and 80 the same; oak 120 better on three seeds; oak 200 thinner on seed 1001, neither on 1007, better on 1012; oak 300 worse on 1001 (the stem laid to the ground by sag), better on 1007 and 1012; every other preset the same. Jev's answers on 18 stills agree (`JEV-SR.md`).
- **Refuted on the way, each by arithmetic or an ablation sheet:** one pipe per tip (`R1-NOTE.md`); the straight charge at the potential height (starves) and at twice it (no bound on extent); rule A, fine tips living on (skirts and thickets); rule G as built, a part's largest radius held (stacked cylinders, no tree held up). Rule B, sleeping buds waking where a tip would pay, is not judged: as built a long-slept bud wakes at near-zero presence (`LAST-RESULT.md`).
- **What is left is outside the balance:** the girth falls with what a part bears (SR's trunk 137, 82, 36 cm at 120, 200, 300) and the settle's sag lays the old tree down on it.
- **Next:** fn-349 rewrites SR as product code off the integration branch; whether it merges before girth and sag are fixed is the owner's (goal log, 11 Nov).

The acceptance as written: R1 to R3 done; R4 judged by the host and not passed (one tree worse); R5 took its second branch (the probe leaves the crates as a patch).

## Friction

- **The shell hook refuses a heredoc that contains a redirect line, `git checkout -- crates`, and a redirect to a variable path** (twice, about 4 minutes). A local setup matter, reported and not specced; the dispatch now tells workers to write files with the file tool and revert a probe with `git apply -R`.
- **A still batch run under a timeout shorter than the batch was killed** (2 minutes). Worker's practice changed in the run: batches run in the background with no guard.
- **A probe build that panicked on every run was queued as a batch** (5 minutes). Worker's practice changed in the run: a one-year smoke run before any batch. Proposed as a line for the next child that draws stills: the still runner's preflight, already on the carried list from fn-344, also runs one year of the build.

