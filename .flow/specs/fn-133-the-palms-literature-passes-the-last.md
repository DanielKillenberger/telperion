# The palm's literature passes: the last four stops

## Goal

Four stops remained on the palm's literature after fn-132 (2026-09-24): trunk diameter failed on `no_mature_size` at `proxy_only`; height scored `proxy_only` against a `partial` bar; select's uncalibrated floor rejected the leaflet picks; leaf brightness range was unstated in every source. Owner (2026-09-24), on closing all four in one last spec: "yes" — after this the host brings any further stop to the owner instead of speccing.

## Decisions

Host design (bar per the owner's 2026-09-23 choice):
- A field with any mature-size point never carries `no_mature_size`, at any level.
- Palm `height_m` at `proxy_only` like `dbh_m` (a mature range scores as a bound).
- No selection floor until its labelled set holds at least 20 cases with at least 5 wrong picks; until then select takes the most probable span and records its probability; verify's field-aware check is the guard.
- An unstated variation trait (hue/brightness range) takes the zero-width level as a recorded default with its reason; an unstated colour or roughness still stops.

## Acceptance Criteria

- **R1:** Palm `dbh_m` (`proxy_only`, 3 points) passes with no `no_mature_size` — met (takes `bound_only`).
- **R2:** Palm row asks `height_m` at `proxy_only`; others unchanged — met.
- **R3:** Below the calibration minimum, select fills the leaflet sizes from the most probable span with its probability — met (`pick_probability` in the select body, not provenance).
- **R4:** Unstated `leaf_brightness_range` becomes the zero-width default; an unstated colour still files `requirements-unmet` — met.
- **R5:** Gate green — met (1027 passed; first run red on a provenance-probability guard, fixed in `9f1ea345`).

## Result

Landed in `00c9e689`, `9f1ea345`; carried by the fn-149 rewrite (#121). Tests `tests/palm_literature.rs`.

## Open

None.
