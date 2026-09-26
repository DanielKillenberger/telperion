# fn-167 friction

## 2026-09-26: no tool measures a blend's smoothness

- **Doing:** measuring whether a morph is smooth before rendering it (R4).
- **Slowed by:** nothing in the repository steps a blend and reads whole-tree
  metrics. `headless --to` renders a walk but measures nothing; the species
  measure tools read one preset. I wrote `video/dump` (a standalone crate over
  `pipeline::build` and `blend::families`) and two judging scripts.
- **Cost:** about 40 minutes and roughly 350 CPU builds (21 preset pairs at 21
  steps, then 200-step scans of the candidates).
- **Would remove it:** a core example, e.g. `blend_metrics <from> <to> <seed>
  [--span a,b] [--steps n]`, printing height, spread, node, wood and leaf
  counts a step, so any spec that shows or tunes a walk can measure it first.

## 2026-09-26: no preset morph measures smooth

- **Doing:** choosing the preset-to-preset morph for R4.
- **Slowed by:** every one of the 21 pairs of branching presets at seed 1
  steps in whole-tree counts. Some steps are one-unit count rows rounding
  (ordinary to Telperion jumps +82% nodes between t=0.7475 and 0.75, and again
  at 0.25). Others sit at an end of the walk: oregon-white-oak to Telperion
  goes from 125,087 nodes at t=0 to 4,078 at t=0.005; oak to beech jumps +30%
  near t=0.0125 and meets the 250,000-node cap at t=0.5 and 0.8. Even the
  writhe sweep on Telperion moves nodes 13% in one 0.005 step. Measurements:
  `BLENDS.md`.
- **Cost:** the morph shot is held out of the cut; R4's morph stays open.
- **Would remove it:** a host decision on what "measured smooth" means for a
  walk whose count rows step by one unit (STRATEGY.md allows that step), and a
  look at the end-of-walk cliffs, which read as a defect in `blend::families`.

## 2026-09-27: the field source does not load under Node's type stripping

- **Doing:** growing the oak's field from `src/field/index.ts` for the
  consumer shots (R9).
- **Slowed by:** `src/field/index.ts` imports `../wasm-source` with no
  extension, which Node's type stripping refuses, and the slim Wasm is only
  built by `npm run wasm:build`, which a worktree without `node_modules`
  cannot run. I built the slim crate with cargo by hand and added a resolve
  hook (`video/draw/ts-resolve.ts`).
- **Cost:** about 10 minutes.
- **Would remove it:** `.ts` extensions on the package source's relative
  imports, or a one-line note in `docs/field-package.md` on running the entry
  point from Node without a build.
