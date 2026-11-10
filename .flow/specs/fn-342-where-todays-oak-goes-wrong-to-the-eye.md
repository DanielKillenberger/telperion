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
