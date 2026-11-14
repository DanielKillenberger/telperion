# An unstated leaf underside takes the front colour

## Goal

No admitted source described the palm's leaf underside, two search rounds found none, and `requirements-unmet/leaf_back_colour` reached the owner (fn-80, 2026-09-24). Many sources describe a leaf's colour without separating its faces. Owner (2026-09-24): "Default to front colour (Recommended)" — for every species, recorded as a default that tuning refines from the photographs.

## Decisions

- Host design: on fn-133's default path, an unstated `leaf_back_colour` with a sourced `leaf_front_colour` takes the front's level and ranges mapped onto the back's material fields, recorded as a default citing the front's source. No other colour trait defaults; bark never does.
- Host (2026-09-24, R4): a back dropped by a claim resolution (`replace-source`/`drop-value`) with no replacement found is an unstated one, so the same default applies while the front is sourced (live palm case: back dropped, front `grey_green`).

## Acceptance Criteria

- **R1:** Unstated back, grey-green front from F1 -> back `grey_green` as a default, no `requirements-unmet` — met.
- **R2:** Both faces unstated still file for both; bark never defaults — met.
- **R3:** Gate green — met (1052 passed).
- **R4:** A dropped back with no replacement defaults to the sourced front — met.

## Result

Landed in `03e1cc9a`, `1fb79862` (`pipeline/stages/appearance.rs` `default_to_front`, two-pass `drop_flagged`); tests in `tests/judged_select.rs`, each red first. Carried by the fn-149 rewrite (#121).

## Open

None.
