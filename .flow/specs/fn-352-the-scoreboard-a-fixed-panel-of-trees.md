# The scoreboard: a fixed panel of trees, drawn and measured

## Goal

One rerunnable instrument that draws and measures a fixed panel of trees on any build, and its first run on the build of 11 Nov 2026 with 0.1.4 beside it. Every later landing of fn-351 is judged against it. It changes no product code.

## Context

- Child of fn-351 (Phase 0). The host designed the panel; the builder builds the runner and runs it.
- Earlier instruments to reuse, not rewrite from nothing: the still runner and its `telperion-heavy` use (fn-336's `sheets.py` and `stills.sh`; fn-349's `r5.sh`, `cost.sh`; fn-350's `probes/run.sh`, `measures.py`, `sheets.sh`), the Wasm timing protocol (fn-329's `probes/wasm.mjs`, `PROTOCOL.md`), the published ranges already checked against their sources (fn-339's `R2-REAL-TREES.md`, fn-341's `R1-MEASURES.md`). Closed specs' evidence may have been pruned; say what is missing.

## Acceptance Criteria

- **R1 (the runner):** one command, `scoreboard/run.sh <label>`, kept under this spec's evidence, draws and measures the whole panel on the checkout it is run from and writes `scoreboard/<label>/` (tables, sheets) with raw stills under ignored `raw/`. It runs through `telperion-heavy`, never two heavy jobs at once, and states the commit, the seeds and the machine load. A second command compares two labels and prints every measure that moved by more than 5%.
- **R2 (the panel):** every shipped preset at its preset age (the oak, spruce, beech, birch, date palm, the ordinary tree, Laurelin, Telperion), and the oak at 40, 80, 120, 200 and 300.
  - Sheets: seeds 1001, 1007 and 1012, and seed 7 for the oak; whole, mid and close views, leafed; the oak's ages also leafless at the whole view. One JPEG sheet per species and view, a label under every tile (species, age, seed, height, spread, trunk), a refused tree as a labelled empty tile with its error.
  - Measures: seeds 1001 to 1012. Per tree: drawn or refused, height, spread on both axes, trunk diameter at 1.3 m, the height of the lowest living limb, leaf area and leaf area index, new shoots in the last year, drawn nodes. Median and range per species and age, beside the published range where fn-339 or fn-341 holds a checked one, each with its source.
- **R3 (cost):** native, one thread: seconds and peak memory for every preset at its preset age and the oak at 40, 120 and 300, seed 1001. A thousand distinct oaks at 40 across the cores: seconds. In Wasm by fn-329's protocol: milliseconds for every preset at its preset age.
- **R4 (0.1.4 beside it):** 0.1.4's Wasm times for the species 0.1.4 has, by the same protocol, from the published package or the `v0.1.4` tag in its own checkout with its own `target/`. Its trees drawn at the whole view on the same seeds if the 0.1.4 code can draw a headless still within 45 minutes of effort; if not, say what stopped it and go on without them.
- **R5 (reference photographs):** list what reference photographs of real trees the repository already holds per species (path and what each shows). Fetch nothing.
- **R6 (first run):** label `base-2026-11-11`, on the integration branch's head. `SCOREBOARD.md` holds the tables and the builder's plain read per tree.

## Boundaries

- No product code changes. A measure the product does not expose is taken by an example or a probe kept under this spec's evidence, never by a change to the crates that would land.
- No driving WebGPU in a browser. No full-forest capture.
- The host views the sheets and gives every verdict.

## Evidence

`scoreboard/` (runner, compare, tables, sheets as JPEGs), `SCOREBOARD.md`; raw stills under ignored `raw/`.
