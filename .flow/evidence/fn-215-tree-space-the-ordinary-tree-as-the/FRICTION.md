# Friction: fn-215

## 2026-10-08, the centroid at 80 is refused only after minutes of growth

- **Doing:** growing the ordinary tree's sheet at age 80 (seeds 1, 7, 42) with `space_ordinary`.
- **Slowed by:** every seed ran into the engine's 20,000,000-phytomer budget and was refused after growing: seed 7 took 288 s, seed 42 86 s, before `resource limit: engine phytomer budget`. The up-front predictor (`pipeline/space.rs`, `foreseen`) gives no expected count for a species whose buds read light, and the centroid's light extinction is 1/6, so nothing refused it early.
- **Cost:** about 8 minutes of wall time on three refusals; no sheet at 80.
- **Would have removed it:** a refusal before growth, or an early stop once the budget is plainly out of reach. Left to the host; not changed here.
