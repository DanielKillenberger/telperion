# R5 stills: whole, mid and close of every preset

Each preset was rendered at seed 42, 640×480, by `headless` (release) on three trees, under the GPU lock:
- master `ebd2e37b`;
- the base `59e627e7` (F4, contract A);
- this branch (f32).

The shots are whole (the hero pose), mid (fill 2.2, aim 0.55) and close (fill 7, aim 0.65). The sheets are in the ignored `raw/stills/`: `<preset>-f4.png` sets base against f32, and `<preset>.png` sets master against f32.

## Against the base, where f32 is the only change

| Preset | Whole RMSE | Mid RMSE | Close RMSE |
|---|---|---|---|
| european-beech | 0.22% | 0.46% | 0.50% |
| oregon-white-oak | 0.29% | 0.48% | 0.47% |
| norway-spruce | 1.45% | 1.96% | 1.37% |
| date-palm | 0.33% | 0.63% | 0.72% |
| silver-birch | 0.75% | 0.75% | 1.10% |
| ordinary | 0.21% | 0.40% | 0.53% |
| laurelin | 4.73% | 6.06% | 4.36% |
| telperion | 6.93% | 5.78% | 4.38% |

**Viewed (one sheet a preset):**
- **Telperion and Laurelin:** the same crown, trunk, limbs and silhouette. Only the scatter of blossoms and leaves within the crown moves, which is where the RMSE comes from.
- **Norway spruce:** whorls, tiers and the leader are the same.
- **Silver birch:** the hanging curtains and the trunk cannot be told apart.
- **Oregon white oak:** the same leaf by leaf at every view.

No perceptible regression was seen at any of the three views.

## Against master
Master still draws the pre-engine tree, so its sheets show a different generator: the beech is a fuller, paler crown on a short white trunk. They do not measure fn-254. The base comparison above is the one that isolates the floats.
