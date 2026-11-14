# A resolved claim stays resolved while the claim is unchanged

## Goal

The owner's and the routine tier's resolutions on the palm's appearance and height claims were voided by every stage rerun and asked again, because each claim decision was keyed to the whole `select.json` checksum (fn-80, 2026-09-24). Owner (2026-09-24): "Accept, and fix (Recommended)" — a resolution is keyed to its claim, so reruns stop re-asking.

## Decisions

- Host design: verify keys each claim decision on a checksum of its pointer, selected value, source id and cited sentence; an unchanged claim keeps its resolution, a changed one voids it. Old-keyed decisions void once and refile.

## Acceptance Criteria

- **R1:** A resolved claim survives another field changing and voids when its own value, source or sentence changes — met (red first).
- **R2:** Gate green — met (1053 passed).

## Result

Landed in `db9364fb`; carried by the fn-149 rewrite (#121).

## Open

None.
