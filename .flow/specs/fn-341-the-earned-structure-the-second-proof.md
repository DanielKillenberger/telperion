# The earned structure: the second proof

## Conversation Evidence

> goal, owner, 9 Nov 2026: "0.2.0 looks bad. But .2 is better conceptually with the botanically accurate way of growing it just needs to be even better and grow larger trees more efficiently"
> goal, owner, 9 Nov 2026: "i don't think we want hand tuned rules. It's going against the engineering principles"
> owner, 10 Nov 2026, on the first proof's second read: "ok follow your recommendation. Just don't get stuck in a patch the current issue mode. Think high level how we can achieve our goal and make sure we move that direction. 3x cheaper is pretty good already"
> goal R2: "a lateral rises in rank by its vigour and not by a timer; a limb holds its lean while its wood's gravitropic performance allows and bends where it does not; height slows as the path from the root lengthens."
> goal, the second proof: "The approach stops if the owner does not find them better."

## Goal & Context

Child of goal fn-321, the second of its three proofs, serving R2, R3 and R4. The structure a viewer reads (trunk, limbs, boughs, branches) is today laid by prescribed settings: timers that raise a lateral's rank, set angles, a sag applied at the end. The goal's approach earns that structure from each branch's present state by published rules. This child shows whether trees grown that way read better to the owner and sit nearer real trees, before the grower's core is rewritten. [paraphrase]

It starts from measurement, not from a list of rules. fn-340's read gives the oak a trunk of 0.79 m at 40 and 2.0 m at 80 at heights of 10.5 and 19.4 m, which the host believes is two to three times a real oak's girth at those ages (unchecked). What is furthest from real trees is found first, and the rules are ordered by it. [inferred]

In hand: `.flow/evidence/fn-321-020-one-grown-tree-at-any-scale-in/RETHINK-2026-11-10.md` (the approach and its sources); fn-338's `R1-NOTE.md` (gravitropic performance, read against Alméras and Fournier 2009) and fn-337's `DESIGN-FINDINGS.md` and `BEFORE.md`; fn-336's sheets of the trees as they stand; fn-339's `R2-REAL-TREES.md`. [paraphrase]

## Acceptance Criteria

- **R1 (what is far from real trees, and back to the host):** on today's cycle, the oak at 40, 80, 120 and 300 and the beech at 40 and 80, seeds 1001, 1007 and 1012, a table of the structure's measures beside published ranges for those species, each number checked against its source's text and an unchecked one marked: trunk diameter at breast height for the age and for the height; crown spread for the trunk's diameter; a branch's diameter against its parent's and the summed section across a fork; the height of the lowest living limb; the number of limbs from the trunk; limb angle from the trunk by height; the trunk's taper. Each measure is ranked by its distance from the range. Beside each, the settings and code that set it today, saying which are prescribed (a timer, a set angle, a value chosen by eye) and which already follow from the tree's state. [inferred]
- **R2 (the rules on paper, and back to the host):** for each rule the goal names (vigour across orders setting how much a shoot grows; rank by vigour; posture by gravitropic performance; height slowing with path length) and for any R1 shows is needed (girth, if the trunk is as far off as it looks), a note gives: the published rule as its source states it, checked against the text; the formula over a branch's present state; the settings it replaces; the measured traits it needs and where their values come from; and, with arithmetic on one stated branch of the oak, the direction and rough size of its effect on R1's measures. It says where a source gives no functional form. No probe is built before the host answers this note. [inferred]
- **R3 (the probe):** on the host's word, the answered rules are built into the cycle as a probe, each changing the tree by degree and dormant at a neutral value, in the order the host gives. Each rule is first run alone on one oak and its measured direction set beside R2's paper direction before the next is added. [inferred]
- **R4 (the sheets and the read):** the oak at 40, 120 and 300 and the beech at 40, seeds 1001 and 1007, with leaves, at the whole and mid views, today's cycle beside the probe on retained sheets in the form of fn-336's; R1's table again under the probe; one-thread seconds beside the cycle's. Every other preset at its preset age on one seed at the whole view, so a rule that harms a species is seen. [inferred]
- **R5 (the owner's question):** the result ends with the sheets' paths and one question for the owner: are these better. It reports R1's measures before and after, says which moved toward real trees and which away, and proposes nothing further. The verdict is the owner's and only his written word counts. [paraphrase]
- **R6 (verification):** no product behaviour changes; the probe leaves the crates as a patch in evidence, and where any code stays the gate is green and the codex review ends SHIP. [paraphrase]

## Boundaries

- No value fitted per species and none tuned to a sheet or to a published range. A trait takes a measured value with its source, or is listed as an estimate with its reason. [paraphrase]
- No artificial limit on growth: height slows by a rule over the tree's state, never by a cap. [paraphrase]
- The foliage is not changed here: the probe runs on the full cycle so the sheets carry leaves. Counted shoots (fn-339, fn-340) are a separate question. [paraphrase]
- Nothing merges to master, nothing is tagged or published. Heavy work runs through `telperion-heavy`.

## Decision Context

Settled by the goal: form is earned from present state by published rules; a rule's direction is derived on paper before it is built (fn-337 built a premise whose direction was reversed); the host conducts and an Opus worker builds, stopping at any question about how the system should work. The owner asked on 10 Nov 2026 that the run keep to the goal's direction and not chase one defect; so this child reads all of the structure against real trees first and lets that order the work. [paraphrase]

Not settled, and the host's to decide on R1 and R2: which rules enter the probe and in what order; whether girth is a fifth rule; what stands in for a rule whose source gives no functional form (the height limit). [inferred]

## Open Questions

None.
