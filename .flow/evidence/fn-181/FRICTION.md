# fn-181 friction

## 2026-10-02 — species metrics baseline for R2

Doing: recording master's species metrics at seeds 1 and 7 for every preset (R2).
Slowed: `species_measure` takes one profile set per run and refuses the catalogue packets'
`profile.json` for european-beech, silver-birch and date-palm ("invalid or unready profile
manifest"); the fn-9 set carries only oak and spruce. Three probe runs, about 5 minutes.
Would remove it: a `species_measure` mode that measures a preset with no profile gate (metrics
only), or a `--profiles` that accepts a packet's unfrozen profile for measurement.
