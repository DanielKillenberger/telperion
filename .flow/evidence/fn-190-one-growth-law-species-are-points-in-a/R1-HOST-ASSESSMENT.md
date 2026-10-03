# fn-190 R1, host assessment after five rounds, 2026-10-03

The host stops the run after round 5 instead of spending round 6: the rounds are not converging. Each host decision fixed the defect it targeted and opened another one, and the two gaps that never closed (crown spread and time at equal fine wood) are the ones R1 cannot waive.

## Trajectory

| round | law change (host decision) | fixed | broke or still open |
|---|---|---|---|
| 1 | competition from the root, markers scaled by age | one law from root to tip | beech a pole; fine wood 0–3 km |
| 2 | light from the shadow grid, short shoots grown, bole by shedding | bole appears on the beech; spruce tiers | size runs away (49k vs 597k nodes by seed); shedding a cliff |
| 3 | carbon balance: upkeep, demand allocation, shedding on balance | size bounded (seeds within 1.5×) | no bole; seedling cliff; upkeep dial jumps 4–7× |
| 4 | Beer–Lambert light from above, reserve, perennial short shoots, smooth bole measure | bole passes beech and oak; both walks by degree; no seedling death in 40 runs | every crown columnar; time 1.8–3.5× over; fine wood 9–36% |
| 5 | five sky directions, phototropism, hydraulic height, short shoots as records | Troll trace passes at beech seed 1; light lookups 60 → 7 ms | bole lost (0.07); crowns still columnar; time 2.4–7.4× over; oak and spruce collapse with short shoots; walks collapse at their edges |

## What the five rounds established

- **Competition growth delivers what the patches faked, at the right settings:** a bole by shedding, size that limits itself, dials that move the tree by degree (round 4). These are real results and survive into any next attempt.
- **Crown spread never came,** from markers, a 45° shadow, Beer–Lambert, sky light or phototropism. Every broadleaf grew a column, spindle or cone. Today's scaffold rules make a spread crown; the law has not.
- **Time at equal fine wood is out of reach of incremental fixes:** rounds 4 and 5 are 1.8–7.4× over the bar with 9–40% of today's fine wood, so the gap at equal detail is roughly 5–30×. Today's twig layer makes fine wood by rule at almost no cost; the law pays per shoot for light, balance and allocation.
- **Grow-or-collapse recurs** (rounds 2, 3, 5): a whole-tree balance can go negative and the main line is then shed with the crown. That is a real tree dying, but it puts a cliff into every walk that reaches the edge of the viable region.

## Options for the owner

1. **Stop R1 here and rescope.** Keep fn-190's evidence and the three results that held (bole by shedding, self-limiting size, smooth dials); open the question of how crown spread is produced before any more law rounds. Recommended: five rounds show the law lacks a mechanism for spread, and adding mechanisms per round has not converged.
2. **Run round 6 on round 4's light** (where the bole held), with spread from branch angle and gravitropism per axis category and the main line's balance read against the tree's reserve. Low odds of meeting R1 in one round; the time bar would still fail.
3. **Change where the speed bar applies:** the probe proves form, and the time bar binds the production implementation (R5) instead of the probe. This is the owner's call: the bar is the owner's equal-detail rule.

Probe code: `crates/telperion-render/examples/growth_law/` on branch `fn-190-one-growth-law-species-are-points-in-a` (scratch, never merged). Round reports: `R1-ROUND1.md` to `R1-ROUND5.md`; host decisions per round in the spec.
