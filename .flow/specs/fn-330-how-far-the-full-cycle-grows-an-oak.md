# How far the full cycle grows an oak, unlimited

## Conversation Evidence

> owner: "can we not just generate the trees offline without shoot limit?"
> owner: "how can a tree be that memory expensive..? there must be a way to make that compress 100x at least?"
> owner: "ok" (to measuring first, then compressing if the numbers support it)

## Goal & Context

Child of goal fn-321, serving R7 and the route from 3 Nov 2026, whose learned stand-in needs full cycles to train on and to check against. The full cycle refuses the oak past about age 80 because the phytomer budget, 20 million, counts every phytomer ever grown, the retired ones included, while retired subtrees already leave memory. The device refuses past 90 on 2 GiB buffers. So whether full oaks reach 300 offline on this machine is unknown. [paraphrase]

## Acceptance Criteria

- **R1 (a frozen protocol):** the ages, seed, budget, thread count and what is recorded are committed in evidence before the first grow. [inferred]
- **R2 (the reach):** the full cycle on the CPU grows the oak at seed 1 to ages 80, 100, 120 and further in steps until it is refused or passes 300, with the budget raised so it does not refuse first, each in its own process. For each age: wall seconds, peak resident memory, phytomers live at the end, phytomers grown in all, and the share in the busiest cycle. A grow that runs out of memory or time is reported with the age and how it ended. [inferred]
- **R3 (where the memory goes):** the results state the bytes held per live phytomer and per live axis, how the live count and the grown count rise with age, and from those an estimate for age 300 of memory and time, marked as an estimate, with the share each of the three savings proposed on 3 Nov could take: a smaller phytomer, dropping shed wood, merging settled wood. [inferred]
- **R4 (the next step):** the results end with a recommendation: full oaks at 300 on this machine as they stand, after named compression, or not here, with the numbers it rests on. [inferred]
- **R5 (verification):** where product code changed, the gate is green and the codex review of the child's diff ends SHIP; a measurement-only child records its review skip with the reason. [inferred]

## Boundaries

- No threshold of the goal lowered. Nothing merges to master, nothing is tagged or published. [paraphrase]
- Heavy work runs through `telperion-heavy`, its memory claim sized to the grow, and keeps the desktop's reserve. [paraphrase]
- The product's budget stays as it is; only the measurement raises it. [inferred]

## Decision Context

The owner approved the route of 3 Nov 2026: the scaffold grown in full, a learned stand-in for the rest, age 300 first. A result is information for the next attempt and does not stop the run. [paraphrase]

## Open Questions

None.

## Result

Closed 3 Nov 2026. The full cycle grows the oak, seed 1, to 300 on one thread in 210 s at 1.95 GiB peak, after fn-333 let go of shed branches' records; before it, 200 needed 9.12 GiB and 300 was killed at 10 GB. Recommendation: full oaks at 300 grow on this machine as they stand; their form, not memory, now limits the route. Review skipped: no code of this child remains. Evidence: `.flow/evidence/fn-330-how-far-the-full-cycle-grows-an-oak/RESULT.md`. No friction recorded.

