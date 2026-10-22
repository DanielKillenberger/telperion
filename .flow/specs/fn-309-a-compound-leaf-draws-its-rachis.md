# A compound leaf draws its rachis wherever it is placed

## Goal & Context

Owner (2026-10-22), on the release film's cut 5: "i thought i still saw bare/flying leaves in the palm". This is the same symptom as the cut 4 verdict (2026-10-19: "some flying leaves that don't make sense").

Measured by the host's diagnosis on the film branch at 9daf4afa5 (`examples/walk_leaf_probe.rs`, uncommitted in `.worktrees/film`; scratch output `probe3.txt`), oak to palm at age 40, seed 7, frame 526, t = 0.34:
- **687 of 860 leaves stand on no drawn element.** They sit 0.17 to 1.19 m from the nearest drawn wood (p10 to p90), and 37 stand above the highest wood. The pattern holds at frames 506, 516 and 536.
- **They are the oak's twig placements.** With `cauline` set to 0 (rosette fronds only), no leaf stands on nothing. With `leafletCount` set to 1, none does either.
- **The cause, read in code:**
  - A twig placement fans through `rosette::fan` with girth 0 (`station.rs:128-133`, `cluster.rs:131-136`). Its rachis "draws no bark" (`rosette.rs:376-377`).
  - It takes the blended `leafletCount` (log, 4.99 at t = 0.34) and spaces those leaflets along the blended `rachisLength` (linear 0 → 7 m, 2.38 m at t = 0.34; `leaflet.rs:50-51, 172-173`).
  - `rachis::rachises` draws an axis only for rosette fronds (`rachis.rs:52-58`; fn-283).
  - So each twig leaf becomes a row of leaflets on a rachis up to 2.4 m long that is never drawn.

fn-283 drew the rachis of a frond. This spec draws the rachis of every compound leaf, wherever it is placed. Joins the release (fn-237): the film shows it.

## Requirements

- **R1 (the rule):** every leaf whose placement carries a rachis (`leafletCount` above 1 and `rachisLength` above 0) draws that rachis through the one path `rachis::rachises` already uses for fronds.
  - A rachis's girth and taper come from the same rows a frond's rachis reads, with no new row.
  - A placement with one blade or no rachis length draws nothing new. So shipped species whose leaves are simple are byte-identical, and the rows stay dormant where the structure is absent (principle 3).
  - Check before the plan which shipped species place compound leaves off a rosette today. Their trees change by design; measure them and show stills.
- **R2 (device and CPU):** the drawn rachis is the same on both executors, held by the existing rachis agreement tests.
- **R3 (proof):**
  - **No floating leaves:** the probe's class (e) count is 0 at film frames 506, 516, 526 and 536, and at fn-308's oak-to-palm frames t = 0.25, 0.5 and 0.75, seeds 1, 7 and 42.
  - **Shipped presets unchanged:** shipped presets with simple leaves keep byte-identical digests.
  - **Cost:** `build_time` on the oak-to-palm frame at t = 0.34 (`BUILD_FRAME`) and on the date palm at 40 is measured against the base.
  - **Stills:** frame 526 at the film's pose, base and change side by side, for the owner.
- **R4:** the probe is committed as an ignored test that reports the classes. The gate (`scripts/gate.py` as declared) and `npm test` are green, with a codex review per task, every review dispatched inside `telperion-heavy` per AGENTS.md.

## Boundaries

- Not the blend rules (fn-299, fn-308) and not the size factor.
- The birch-to-oak oracle overshoot in cut 5 (40.8 % over its ceiling, against fn-251's exception of 21 %) is not this spec. The host handles it separately.
