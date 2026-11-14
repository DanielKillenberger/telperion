# A dial's range is its validated bound; the preset span is a stride, not a wall

## Goal

The date palm's first tuning revision (fn-80, seven rounds, ~1.64 M tokens) could not reach a palm's proportions: 75 of 219 dial rows took their `min`/`max` from the span the shipped presets occupy, capping `leaf_length` at 0.171, `leaf_width` at 0.11175, `trunk_radius` at 0.023 and `leaf_size` at 1.575 where a palm leaflet is 20-40 cm. A species outside every shipped preset cannot be tuned toward itself. Host proposal 1 in fn-80's `GAPS.md` (2026-09-22): "Tuning loop: dial ranges from validated bounds, the preset span as a stride only." Owner (2026-09-23): "i approve 1 & 2" (proposals 3-5 not approved).

## Decisions

- A row's walls become the generator's validated bound; the preset span stays on the row as a non-limiting field (stride hint, prior), never a clamp. A parameter with no validated bound gets one in the generator, with a test, before its row widens; otherwise it stays capped and says so.
- How far a round moves is fn-114's.

## Acceptance Criteria

- **R1:** No row takes `min`/`max` from the preset span; each reads a validated bound or records why it stays capped — delivered by fn-149.
- **R2:** Preset span kept as a non-limiting field; a move past it is accepted — delivered by fn-149.
- **R3:** The palm's leaflet length/width, trunk radius and leaf size can reach measured values — delivered by fn-149.
- **R4:** Gate green — delivered by fn-149.

## Result

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (`c2ac430b`).

## Open

None.
