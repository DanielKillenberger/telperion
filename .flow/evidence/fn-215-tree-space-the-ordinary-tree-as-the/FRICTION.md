# Friction: fn-215

## 2026-10-08, the centroid at 80 is refused only after minutes of growth

- **Doing:** growing the ordinary tree's sheet at age 80 (seeds 1, 7, 42) with `space_ordinary`.
- **Slowed by:** every seed ran into the engine's 20,000,000-phytomer budget and was refused after growing: seed 7 took 288 s, seed 42 86 s, before `resource limit: engine phytomer budget`. The up-front predictor (`pipeline/space.rs`, `foreseen`) gives no expected count for a species whose buds read light, and the centroid's light extinction is 1/6, so nothing refused it early.
- **Cost:** about 8 minutes of wall time on three refusals; no sheet at 80.
- **Would have removed it:** a refusal before growth, or an early stop once the budget is plainly out of reach. Left to the host; not changed here.

## 2026-10-08, no way to read a tree's grown phytomer count

- **Doing:** checking that the sized ordinary tree at 80 stays under the 20M grown budget with margin.
- **Slowed by:** the grown count is `pub(crate)` (`grow::Grown`), so it was bracketed by regrowing each seed under lower budgets: nine extra 12 to 37 s grows. A first probe piped through `tail` showed nothing for ten minutes and was stopped, which cost a wasted ten-minute wait.
- **Would have removed it:** a public grown count beside the kept one, for example on the still runner's line.

## 2026-10-08, walk frames at 80 that neither finish nor refuse

- **Doing:** the walk strips at 80 from the ordinary tree.
- **Slowed by:** toward the beech, t = 0.5 and 0.75 ran to the 110 s timeout with no result, while the over-budget frames toward the oak and spruce were refused in 6 to 7 s.
- **Cost:** about 4 minutes.
- **Would have removed it:** the same early refusal as the first entry.
