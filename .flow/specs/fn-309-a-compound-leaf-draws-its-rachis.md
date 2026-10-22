# A compound leaf is never placed along a rachis nobody draws

## Goal & Context

Owner (2026-10-22), on the release film's cut 5: "i thought i still saw bare/flying leaves in the palm". This is the same symptom as the cut 4 verdict (2026-10-19: "some flying leaves that don't make sense").

Measured by the host's diagnosis on the film branch at 9daf4afa5 (`examples/walk_leaf_probe.rs`, uncommitted in `.worktrees/film`; scratch output `probe3.txt`), oak to palm at age 40, seed 7, frame 526, t = 0.34:
- **687 of 860 leaves stand on no drawn element.** They sit 0.17 to 1.19 m from the nearest drawn wood (p10 to p90), and 37 stand above the highest wood. The pattern holds at frames 506, 516 and 536 (1,912, 2,644 and more of the frame's leaves).
- **They are the oak's twig placements.** With `cauline` set to 0 (rosette fronds only), no leaf stands on nothing. With `leafletCount` set to 1, none does either.
- **The cause, read in code:**
  - A twig or cluster placement fans through `rosette::fan` with girth 0 (`station.rs:128-133`, `cluster.rs:131-136`). Its rachis "draws no bark" (`rosette.rs:376-377`).
  - It takes the blended `leafletCount` (log, 4.99 at t = 0.34) and spaces those leaflets along the blended `rachisLength` (linear 0 → 7 m, 2.38 m at t = 0.34; `leaflet.rs:50-51, 172-173`).
  - `rachis::rachises` draws an axis only for rosette fronds (fn-283).
  - So each twig leaf becomes a row of leaflets on a rachis up to 2.4 m long that is never drawn.

Drawing that rachis was weighed and declined (host ruling, 2026-10-22, below): it would need a second placement replay in the clothing step and the device's stations moved into the boots stage, about fn-283's scale (1,200 lines), for a structure that exists only inside walk frames. Question and delete first: the structure is not needed, so its row goes dormant. This spec takes fn-295's second branch and supersedes fn-295.

Who it affects: the owner's verdict on the film's oak-to-palm walk, and every consumer of placed leaves on both executors. No shipped preset changes: only the date palm sets compound leaves, and its `cauline` is 0, so it has no twig or cluster placements (`presets/date-palm.values`); every other preset has `leafletCount` 1 and `rachisLength` 0. Joins the release (fn-237): the film shows it.

## Acceptance Criteria

- **R1 (the rule; host ruling 2026-10-22, replacing the first draft's "draw every rachis"):** a compound leaf is never placed along a rachis that no axis draws. A placement that draws no rachis (a twig or cluster station, which `rosette::fan` places with girth 0) has no rachis structure, so `rachisLength` is dormant there (principle 3): its leaflets stand at the placement point, one tuft of `leafletCount` blades, each with its own heading as the fan already gives it (its pitch, side, arch-turned run, terminal closing and spine share unchanged), never spread along the undrawn line.
  - **Fronds are unchanged.** A frond that draws its rachis (fn-283) keeps its leaflet spacing to the bit.
  - **No new stage, no new row, no new pass.** The change is in the leaflet spacing on both executors (`leaflet::leaflets` and its device mirrors), keyed on the fact `fan` already receives: whether the placement is a frond, which stands on its own rachis (`rachis::girth`), or a placement on wood, whose rachis "draws no bark" (today's girth 0). The CPU makes that fact explicit in `fan`'s girth argument (a frond's girth, or none); the device's station call site is already separate from its frond call sites.
  - Errors: no error surface beyond the existing leaf rows' refusals. Boundaries: a placement of one leaflet, or with `rachisLength` 0, is placed exactly as today; the leaf plan's and reference's reach bounds may stay conservative (a tuft stands inside the reach a rachis had).
- **R2 (device and CPU):** the device places the same tufts as the CPU, held by the existing placement agreement tests (station leaves, cluster leaves, frond leaflets, the field executor) on a walk frame that bears station compound leaves, within their existing tolerances. Errors: a disagreement fails the agreement test, never a fallback to the CPU.
- **R3 (proof):**
  - **No floating leaves:** with the classes below, class (e) is 0 and class (d) is 0 at the film's frames t = 0.129, 0.227, 0.34 and 0.463 (frames 506, 516, 526, 536; seed 7, age 40), and at fn-308's oak-to-palm frames t = 0.25, 0.5 and 0.75, seeds 1, 7 and 42, all at age 40. Classes, per leaf base, against the drawn curve (each ring's sphere and between the rings of one run its tapered capsule), `seat` 0.1 m as the diagnosis took it:
    - (a) on wood at least 1 px wide; (b) on wood under 1 px;
    - (c) on a drawn rachis (wood the frond's axis adds);
    - (d) a station- or cluster-borne leaf (told from the frond's by a second build at `cauline` 0) standing more than `seat` off drawn wood: a leaf on a rachis nobody draws;
    - (e) any leaf standing more than `seat` off every drawn element.
    - The pixel split of (a)/(b) and of (c) is reported, never held; it uses a camera at the distance the film's frame stood (28.03 m and 26.8 mm a pixel at the stem for frame 526, from the diagnosis), overridable.
  - **Shipped presets unchanged:** every shipped preset keeps byte-identical CPU and device digests (the committed `suite/species` digests and the device digest test).
  - **Cost:** `build_time` on the oak-to-palm frame at t = 0.34 (`BUILD_FRAME=oregon-white-oak,date-palm,0.34`, age 40, seed 7) and on the date palm at 40 is measured against the base; a tuft must not cost more than the row it replaces (reported; a rise over 5 % goes to the host).
  - **Stills:** frame 526 (oak to palm at 40, seed 7, t = 0.34), base and change side by side at the hero pose, for the owner. If a tuft of blades at one point looks wrong, the report says so plainly; the owner judges.
- **R4:** the diagnosis probe is committed as an ignored test that reports the classes per frame; the gate (`scripts/gate.py` as declared) and `npm test` are green, with a codex review per task, every review dispatched inside `telperion-heavy` per AGENTS.md.

## Early proof point

Task fn-309-a-compound-leaf-draws-its-rachis.1 validates the core approach (on the CPU, a placement with girth 0 stands its leaflets at its point; class (e) and (d) fall to 0 on the oak-to-palm frames while every shipped preset's digest is byte-identical). If it fails, re-evaluate what keys "draws a rachis" before continuing with fn-309-a-compound-leaf-draws-its-rachis.2+.

## Boundaries

- Not the blend rules (fn-299, fn-308, fn-277's log scale) and not the size factor.
- No rachis is drawn for a station or cluster placement (declined, host ruling 2026-10-22); fn-295's first branch is not built.
- The leaf plan's reach bounds are not tightened.
- The birch-to-oak oracle overshoot in cut 5 (40.8 % over its ceiling, against fn-251's exception of 21 %) is not this spec. The host handles it separately.

## Strategy Alignment

Active tracks served by this plan:
- **The tree space** — a walk between leaf forms changes the leaves by degree without placing structure the walk frame does not draw.
- **Surface and rendering at scale** — every leaf the renderer draws stands on drawn structure, with no species path and no second stage.

## Decision Context

- **Host ruling (2026-10-22).** Draw the rachis of every compound leaf (the first draft's R1) was declined: it needs a placement replay in `clothe` (which runs before placement, without the seed, the twig stations or the `kept()` filter) and the device's twig stations moved before the sweep into the boots stage, for a structure only walk frames have. The row goes dormant where its structure is absent instead.
- **The key is the placement kind, not the girth's value.** Keying on a positive girth would also collapse a frond whose `rachisWidth` is 0, which the host's "fronds are unchanged" and fn-283's dormancy (a frond at width 0 places its leaflets as today; `the_axes_move_no_leaf` and the rosette suites build such fronds) both forbid. So a frond keeps its spacing at any width, and only a placement on wood (station or cluster) stands as a tuft. Flagged to the host in the plan report.

## Quick commands

```bash
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-core leaflet
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-core --test walk_rachis
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-core --test walk_flying -- --ignored --nocapture --test-threads 1
```

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | (the rule; host ruling 2026-10-22, replacing the first draft's "draw every rachis"): a compound leaf is never placed along a rachis that no axis draws. A placement that draws no rachis (a twig or cluster station, which `rosette::fan` places with girth 0) has no rachis structure, so `rachisLength` is dormant there (principle 3): its leaflets stand at the placement point, one tuft of `leafletCount` blades, each with its own heading as the fan already gives it (its pitch, side, arch-turned run, terminal closing and spine share unchanged), never spread along the undrawn line. | fn-309-a-compound-leaf-draws-its-rachis.1, fn-309-a-compound-leaf-draws-its-rachis.2 | — |
| R2 | (device and CPU): the device places the same tufts as the CPU, held by the existing placement agreement tests (station leaves, cluster leaves, frond leaflets, the field executor) on a walk frame that bears station compound leaves, within their existing tolerances. Errors: a disagreement fails the agreement test, never a fallback to the CPU. | fn-309-a-compound-leaf-draws-its-rachis.2 | — |
| R3 | (proof): | fn-309-a-compound-leaf-draws-its-rachis.1, fn-309-a-compound-leaf-draws-its-rachis.3 | — |
| R4 | the diagnosis probe is committed as an ignored test that reports the classes per frame; the gate (`scripts/gate.py` as declared) and `npm test` are green, with a codex review per task, every review dispatched inside `telperion-heavy` per AGENTS.md. | fn-309-a-compound-leaf-draws-its-rachis.1, fn-309-a-compound-leaf-draws-its-rachis.3 | — |
