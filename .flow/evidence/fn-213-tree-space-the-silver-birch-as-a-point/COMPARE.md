# fn-213 COMPARE: the shipped (old generator) birch against round 7 (worker, 2026-10-07)

Facts for the host's diagnosis after the owner's verdict on round 7 (SHEET-7.md). No values were changed. The capability call is the host's.

## How this was made

- **Old birch:** today's shipped `silver-birch` on `master` (`ebd2e37b`), built in a scratch worktree of its own (`.worktrees/birch-old`, detached at master, its own `target/`). It goes through the old generator: space colonisation, the twig layer and its pendulous curtain. The old tree ignores `age` (its envelope is fixed at 18 m), so one tree per seed stands for both 40 and 80.
- **Round 7:** the branch tip `3ff611e9`, at 40 and 80.
- **Stills:** the same shots, camera and sun as the engine sheets. The old birch was rendered by a scratch example copying `space/still.rs`'s shot code, under the GPU lock. Sheets: `raw/compare/compare-40.png` and `compare-80.png`, each with an old row above the round-7 row for seeds 1, 7 and 42 (bare, in leaf, crown, limb). Viewed by the worker.
- **Measures:** by code, `birch_measure.rs` (this folder), run unchanged on both checkouts through `pipeline::build`; raw output in `raw/measure-old.txt` and `raw/measure-engine.txt`. Definitions:
  - A *run* is the nodes sharing one `branch` id.
  - *Fine* wood has a radius under 5 mm.
  - A *hanging strand* is a fine run at least 5 cm long whose chord falls more than 30° below the horizontal.
  - *Droop* is that chord's angle below the horizontal.
  - A leaf's host is the nearest node within the 3 × 3 × 3 grid of 25 cm cells around it.

  One caveat on the engine side: its runs are the conversion's (one per axis), so a hanging system (`BRANCH`) and the long shoots it bears (`SHOOT`) count as separate strands. A strand's length there is one axis, not the whole curtain.

## The measures

Seed 1 shown; seeds 7 and 42 follow the same pattern (raw files).

| Measure | Old (seed 1) | Round 7 at 40 (seed 1) | Round 7 at 80 (seed 1) |
|---|--:|--:|--:|
| Nodes | 79,806 | 420,906 | 1,573,152 |
| Hanging strands / fine runs | 32,630 / 44,225 (74%) | 3,394 / 40,868 (8%) | 12,248 / 139,495 (9%) |
| Strand length p10 / p50 / p90 / max, m | 0.45 / 0.45 / 1.32 / 3.00 | 0.11 / 0.30 / 0.46 / 1.23 | 0.11 / 0.28 / 0.37 / 1.29 |
| Strand droop p10 / p50 / p90, deg | 40 / 40 / 90 | 33 / 48 / 70 | 33 / 48 / 70 |
| Droop of all fine runs ≥ 5 cm, p50, deg | 40 | 25 | 26 |
| Fine runs leaving wood ≥ 1 cm radius, per metre of it | 12.5 | 4.3 | 1.6 |
| Fine wood (< 5 mm), total length | 30.1 km | 3.4 km | 10.1 km |
| of which 0.5 to 1 mm radius | 15.8 km | 0.5 km | 1.7 km |
| Fine-wood segment length p10 / p50 / p90, m | 0.41 / 0.45 / 0.48 | 0.002 / 0.003 / 0.028 | 0.002 / 0.002 / 0.024 |
| Leaves | 231,665 | 339,014 | 1,106,770 |
| Leaves nearest a hanging strand / a run under 5 cm / elsewhere | 67% / 0.04% / 33% | 25% / 18% / 57% | 27% / 15% / 58% |
| Height | 16.2 m | 16.8 m | 25.5 m |

## What the old birch does that round 7 does not

### In the curtains

1. **Many long strands.** Three quarters of the old tree's fine runs hang, against under a tenth of the engine's. Old strands are 0.45 m at the median and up to 3 m (p90 1.3 m); the engine's are 0.3 m at the median, 1.3 m at most.
   - **Old mechanism:** the twig layer's pendulous shoots. `/skeleton/twigs/hang` 2.4 (every shoot under a descending limb hangs), `pendulousLength` 3.0 m with `pendulousVariation` 0.95 (each strand its own share of 3 m), and `pendulousRadius` (which stations' shoots hang).
   - **Nearest engine rows:** the length of a `BRANCH`/`SHOOT` axis is lifespan × nodes per unit × `internode`. Round 7 has `BRANCH` 6 years × about 8 nodes × 0.035 m and `SHOOT` 3 years × 4 to 6 nodes × 0.03 m. Per-strand length variation comes from `viability` (apex death) and `internodeSpread`.
   - **Engine mechanism:** yes for length and its spread (values). There is no single per-strand "length share" draw; length varies by apex survival.
2. **Strands that fall straight down.** Old strands either hang at their departure angle (40°, the twig layer's `angle` row) or reach vertical (p90 90°). The engine's sit between 33° and 70°, median 48°.
   - **Old mechanism:** `/skeleton/twigs/sag` 1.0. A strand turns toward straight down along its length as an arc, steepest near its bearer, reaching vertical by the end of its pendulous length.
   - **Nearest engine rows:** `form.elevation` (the set-point: `BRANCH` −1.0 rad, `SHOOT` −1.4 rad), `form.tropism` (how fast an axis turns to it, per metre grown), and `form.sag` (a beam bending under the load beyond each phytomer).
   - **Engine mechanism:** yes. A set-point of −π/2 with a high tropism turns an axis to vertical. Round 7 does not set it so, and its strands are short, so they turn only part way.
3. **Unbranched, evenly spread strands in a curtain.** Old strands are single runs of 0.45 m segments, set `curtainSeparation` 9° apart, falling below the crown shell (`curtainDrop` 0.6) to a floor (`curtainClearance` 1.8 m). The engine's hanging systems branch: `BRANCH` bears `SHOOT` (0.25 medial, 0.4 at the top) and `SHORT`. The stills show tufts and sprays rather than straight strands.
   - **Nearest engine rows:** `BRANCH`'s lateral `SHOOT` probabilities (0 for an unbranched strand), `form.roll` and `divergence` for spacing.
   - **Engine mechanism:**
     - Unbranched strands: yes (values).
     - A minimum angular separation between neighbouring strands: no; strands are placed by the bearer's phyllotaxis and draws, never kept apart.
     - A floor no strand falls below: no; the engine has no crown shell or ground clearance on its axes.
4. **Leaves along the strands.** Two thirds of the old tree's leaves sit on hanging strands; only a quarter of the engine's do. More than half of the engine's sit on non-hanging wood (limbs, boughs and the ascending parts of hanging systems), and 15 to 18% on short runs.
   - **Old mechanism:** leaves are stationed along twig runs (`/skeleton/twigs/twig/internodeLength` 0.06 m on master), with clumps (`/canopy/clump` 8, `clumpSpan` 0.5).
   - **Engine:** the same station placement along every run thinner than `/canopy/shootRadius` (round 7: 12 mm spacing, `shootRadius` 0.03). Where the leaves end up follows where the fine wood is.
   - **Engine mechanism:** placement yes. The leaf share follows the strand share in point 1. Clustering at short shoots is fn-223.

### In the bare fine wood

5. **Ten times the fine wood, all of it long, thin and straight.**
   - The old tree carries 30 km of wood under 5 mm (52% of it 0.5 to 1 mm) in segments of 0.41 to 0.48 m. Each fine run is a straight hair.
   - Round 7 carries 3.4 km at 40 and 10 km at 80, mostly 1 to 5 mm. It is drawn as one segment per phytomer (2 to 3 mm, up to 3 cm), so it wanders and kinks every few millimetres. `wander` is 0.6 rad per metre on hanging axes and 0.4 to 0.6 on limbs.
   - **Old mechanism:** the twig layer (`/skeleton/twigs/laterals` 7, `lengthRatio` 0.55, `twig/length` 0.45 m, `twig/diameter` 0.003 m) grows its twigs as straight 0.45 m runs.
   - **Nearest engine rows:** `internode` and node counts per age (length), `form.wander` and `bendLength` (straightness), `form.pipe` and `exponent` (thickness).
   - **Engine mechanism:** yes for straighter, longer, finer axes (values). The engine's fine wood is drawn at its phytomer resolution, so for the same metres it costs about 200 times the segments of the old tree's 0.45 m segments.
6. **Dense twigs along the limbs.**
   - The old tree carries 12 to 13 fine runs per metre of wood thicker than 1 cm. Round 7 carries 4 at 40 and 1.5 to 1.6 at 80.
   - At 80, round 7's limb wood is 2.7 to 3.2 km, five times the old tree's 0.62 km, but it bears fewer twigs.
   - **Old mechanism:** the twig layer seats `laterals` 7 a station along every limb.
   - **Nearest engine rows:** each limb age's lateral probabilities (`LIMB`/`BOUGH` → `BRANCH` 0.1 medial, 0.22 to 0.25 at the top, `SHORT` 0.35) and node counts per unit. Lateral wood also dies and is shed: `BRANCH` lifespan 6 years, viability 0.95 a year, `shedding` 1 year. So only the last few years' laterals stand on old limb wood.
   - **Engine mechanism:** yes for density (values). Laterals on old wood depend on their lifespan, viability and shedding. Rounds 2 to 4 lowered these lateral rows to open the crown on host and Astra notes (SHEET-2, SHEET-4).

## In one line

The old birch is curtains: about 33,000 straight, unbranched strands up to 3 m long hanging at 40° to vertical, carrying two thirds of the leaves, and 30 km of straight 0.5 to 1 mm twigs. Round 7 is a branching crown: about 3,000 to 14,000 short (0.3 m), part-drooping strands that branch into sprays, and 3 to 10 km of finely kinked twigs. The engine has rows for strand length, vertical hang, unbranched strands and twig density; it has none for keeping strands a set angle apart or for a floor they stop at, and its fine wood is drawn per phytomer.
