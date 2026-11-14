# Where today's oak goes wrong to the eye: jagged limbs and the tree at 300

## Conversation Evidence

> owner, 10 Nov 2026, on the second proof's sheets: "today's ok at 120 isn't that bad honestly. visually the new ones are a regression. Today's is a bit jagged but it looks directionally correct. The jaggedness is worse at 40."
> owner, 10 Nov 2026: "every probe is worse than today"
> owner, 10 Nov 2026, on the oak at 300: "i mean wtf is that ok at 300..?"
> owner, 10 Nov 2026: "Just don't get stuck in a patch the current issue mode. Think high level how we can achieve our goal and make sure we move that direction."

## Goal & Context

Child of goal fn-321. The second proof's verdict is in: trees whose girth and posture were earned from published allometry read worse than today's, so that approach has stopped. Today's structure is the baseline. The owner names two things wrong with it to the eye: limbs are jagged, worse at 40 than at 120, and the oak at 300 is a bare, snaking pole under a tuft. [paraphrase]

This child finds, by measurement on today's cycle, what in the grower produces each of the two, so the next design decision rests on causes and not on the host's guesses (three of which were refuted in fn-340). It changes nothing. [inferred]

In hand: fn-341's `R1-MEASURES.md` (a limb is a chain of relay axes about 0.2 m long; the oak's limb has abortion 0.15 and relay 1.0; at 300 one living limb, trunk bare to 21 to 30 m) and its probes `r1.py` and `probes/structure.patch`, which already walk a limb's thickest wood; fn-336's sheets. [paraphrase]

## Acceptance Criteria

- **R1 (the jaggedness, measured):** on today's oak at 40 and 120, seeds 1001 and 1007, every limb from the trunk and every bough is walked as one path along its thickest wood. For each path: the turn angle at every joint between successive segments, and the distribution of turns per metre. Each turn above a stated size is attributed to what made it in the grower: a relay taking over from a dead apex, a fork, the sag, the straightening, the per-phytomer heading noise, or another cause named from the code. The table gives each cause's share of the turns and of the summed turn angle, at 40 and at 120, and says why 40 is worse. [inferred]
- **R2 (the jaggedness, seen):** leafless stills of the same four trees at the whole and mid views, and one still per cause with that cause's joints marked, so the host can check the attribution against what the eye calls jagged. The worker views every still and says where the table and the eye disagree. [inferred]
- **R3 (the oak at 300, its history):** on today's oak grown to 300, seeds 1001 and 1007, a yearly record: height and its yearly gain; the living limbs from the trunk with each one's birth year, death year and the balance terms in its last five years (light income, upkeep of living wood, upkeep of wood already shed); the height of the lowest living limb; and, for the path that is the trunk at 300, which axis it was born as at each height and the turn angles along it. The note says, from that record, when and why the low limbs die, why nothing replaces them, why the trunk snakes and what yearly height gain does after 120. [inferred]
- **R4 (back to the host):** one note with R1 to R3, each cause named with the code that sets it and whether that code is a prescribed value or follows the tree's state. It proposes no fix. [paraphrase]
- **R5 (verification):** no product behaviour changes; any probe leaves the crates as a patch in evidence. [paraphrase]

## Boundaries

- No fix is built or proposed here; no value is tuned. [paraphrase]
- No sheet goes to the owner from this child. [inferred]
- Heavy work runs through `telperion-heavy`. Nothing merges to master, nothing is tagged or published.

## Decision Context

Settled by the owner's verdict: today's structure stays the baseline and published measurements moving toward a range are no evidence that a tree looks better. Settled by the host: the eye's defects order the work, and each is traced to its cause before anything is designed. [paraphrase]

## Open Questions

None.

## Result

Closed 10 Nov 2026. `RESULT.md` in this spec's evidence holds the tables; `sheet-bare.jpg`, `sheet-limb.jpg` and `sheet-marks.jpg` are the leafless stills, which the host viewed.

- **The jaggedness is a line changing hands.** On limbs and boughs, 71% (at 40) to 92% (at 120) of joints turning 10 degrees or more are relays of 9 to 15 degrees. On all wood 1 cm or thicker, 67 to 74% are a lateral taking its bearer's line at its insertion angle, 34 to 57 degrees; at 40, 1,025 of 1,382 such joints stand where the bearer's tip had stopped. Wander, tropism, sag and straightening make no sharp joint.
- **Why 40 reads worse** is not in the count: only the thickest wood kinks more at 40 (0.47 a metre against 0.26). The host's read of the stills: at 40 the elbowed branch wood is most of what is drawn, and two low limbs lie along the ground; at 120 the same elbows are hairlines beside smooth limbs.
- **The oak at 300.** Every limb on the trunk line died the year the balance shed it, carrying 8 to 10 times its living wood's upkeep in wood already dead. Limbs are born only at the growing top, so replacements start at 18, 22, 25 and 32 m. Above 7 m the trunk is a chain of 35 to 67 limb axes pulled toward 60 degrees and not toward vertical. The top gains 0.08 to 0.17 m a year to 300 and nothing ends it.
- No code stays; the probe is `probes/turns.patch`. No gate and no review ran, for that reason.

## Friction

- **Axis indices change when the grower compacts** (one wasted 300 run, 6 minutes). A doc comment at `Grower::compact` saying lineage is the identity goes into the next child that changes `telperion-space`.
- **A local shell hook refused a redirect to a variable path** (2 minutes). Local, reported and not specced.

