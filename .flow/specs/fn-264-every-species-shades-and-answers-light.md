# Every species shades and answers light by its sources

## Goal

COUNCIL.md (fn-245, 2026-10-11) makes the lattice the one light model for every species, species differing only by their rows; fn-255 measured that only the oak and the ordinary tree carried any leaf area in it, so spruce, birch, beech, palm, Laurelin and Telperion neither shaded nor were shaded. The owner (2026-10-09): "i'm still not comfortable with only the oak growing with light". Give every shipped species sourced light rows, one shared site light, and prove each response at its consumer. Species rows only; no engine change. Release item (fn-237).

## Decisions

- Owner (2026-10-09): "i'm still not comfortable with only the oak growing with light".
- Host (2026-10-11, R1a): the site's light is one shared environment setting, not a per-preset row: extinction k = 0.5 (Bréda 2003, spherical leaf-angle distribution), confirming the oak's previously unchecked 0.5.
- Host (2026-10-12): birch leaf area derived (Flora of China leaf dimensions × 0.67 ovate shape factor, Schrader et al. 2021) = 13.19 cm² a node; spruce derived (Bellan et al. 2019 needles/cm × Gebauer et al. 2019 sun needle area) = 0.033565 m² per metre of current-year shoot; palm the whole frond, 2.81 m² a node (Abdel-Sattar et al. 2024), with `MAX_LEAF_AREA` raised to 5 m².
- Host (2026-10-12): the Two Trees take the beech's rows (their engine species is the beech); beech leaf area 15.723 cm² (Hladká and Čaňová 2005, sunlit).
- Host (2026-10-12): hazard rows from one monotone mapping of Niinemets and Valladares's shade tolerance T, anchored on the oak (T = 2.4); upkeep 0.35, tolerance 0, leaf girth 1.0 shared physiology; apical control stays per species (architecture, not light).
- Host (2026-10-12, birch ruling): the size response ψ is not mapped from tolerance; it stays each species' pre-fn-264 value (dormant) until a source or owner verdict sets it — mapped ψ ran the birch's long shoots bare. Leaf girth stays 1.0 (disused pipes persist as heartwood). Lying wood belongs to fn-252.
- Host: retained (evergreen) foliage is a stated approximation; the deposit counts the year's foliage alone; retention is its own spec.

## Acceptance Criteria

- **R1:** inventory of every row the lattice and light response read, per preset, with consumer; environment rows named as environment — met.
- **R1a:** one shared site light every preset reads non-zero, sourced, tested — met (k 0.5).
- **R2:** each species' rows from literature screened by Jev, copied by code, checked against source; Two Trees by named analogue; no invented numbers — met, with derived values ruled by the host (SOURCES.md).
- **R2a:** per preset, foliage lowers sampled light and each populated response changes growth through its consumer, or the neutral expectation is tested — met.
- **R3:** every preset grows through fn-255's cycle; stills beside F4's viewed; build time and peak memory; fn-251's frame re-measured; owner's verdict — met.

## Result

Final R3 (tip `c116854f3`, ψ dormant), CPU reference at seed 42: kept phytomers birch −33 %, Laurelin and Telperion −33/−34 %, spruce −9 %, beech −6 %; oak and palm unchanged. Device warm build: Telperion 1,201 → 1,980 ms (+65 %), Laurelin 1,108 → 1,808 ms (+63 %), about +0.6 GB peak each, all in the light phase; other presets 0-13 %. Stills: oak and palm pixel-identical; birch's ground limb back to a thin line and the white whips gone; other crowns slightly more open. fn-251's frame (oak → birch, t 0.0750, age 80, seed 7) grows 15,989,657 and builds in 18.4 s. Gate: workspace 1,073 passed, renderer 273 passed, `npm test` 162; codex SHIP.

Owner verdict (2026-10-13), on the sheets: "yea looks good the artifact".

## Open

- Retained evergreen foliage in the deposit (host: its own spec).
- The birch's ground-running wood (fn-252; finding filed there).
- Better birch leaf-area sources are paywalled (Kull and Niinemets 1993, Niinemets 1994, Atkinson 1992).

## Friction

- flowctl not on the worker's PATH -> dispatch names the path.
- `jev select` recalls lengths and ages only, not coefficients or areas -> `select --units area,ratio` proposed for `telperion-jev`.
- fn-251's repro lived only on its branch; no `/usr/bin/time` -> merge the repro as an ignored measurement; shared timing script proposed.
- jev replay tests need examples only the renderer run builds -> fn-237 R10.
- CPU timings voided by another session's memory -> machine-wide memory gate proposed (fn-259 R4).
- A game took 6 GB of the card mid-gate: 22 spurious OOM failures -> per-test free-memory check proposed.

## Evidence

- `.flow/evidence/fn-264-every-species-shades-and-answers-light/SOURCES.md` (sources, Jev picks, host decisions)
- `.flow/evidence/fn-264-every-species-shades-and-answers-light/BIRCH.md` (birch ground limb and whips isolated; host rulings)
