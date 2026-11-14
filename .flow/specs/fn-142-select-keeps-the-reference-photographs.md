# Select keeps the reference photographs a run recorded

## Goal

The palm's fourth tuning revision paused "baseline infeasible" (no matched shots): the three reference photographs and matched shots recorded on 2026-09-22 (`0995dac9`) had been overwritten with an empty list by the 2026-09-23 stage rerun (`ba8a92e6`), because select rewrote `packet/references.json` wholesale. A literature rerun must never erase them. An obvious friction fix under the owner's standing rule of 2026-09-23.

## Decisions

- Select rewrites only the `sources` it owns and keeps existing `references` entries byte for byte; a first run writes an empty list as before.

## Acceptance Criteria

- **R1:** A references file with the palm's three recorded shots keeps them byte for byte after a select rerun with changed sources — met (red first).
- **R2:** Gate green — met (1053 passed).

## Result

Landed in `a63d9e9f`; carried by the fn-149 rewrite (#121).

## Open

None.
