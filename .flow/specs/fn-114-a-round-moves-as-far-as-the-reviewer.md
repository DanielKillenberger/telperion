# A round moves as far as the reviewer says the tree is off

## Goal

In the date palm's first tuning revision the reviewer said every round that the fronds were two to three times too short, yet frond length moved 3.5 to 3.75 m over seven rounds: a round's move was a fixed ladder of multiples of each dial's small step whatever the reviewer said. This spec lets the size of the named gap choose the stride. Host proposal 2 in fn-80's `GAPS.md` (2026-09-22): "Tuning loop: strides scaled by the reviewer's magnitude words, or a magnitude the owner's priority carries." Owner (2026-09-23): "i approve 1 & 2".

## Decisions

- Jev selects, never supplies a number: code offers a magnitude class set per priority or finding (near, clearly off, far off, no-match), Jev or the owner's priority selects, code maps the class to a strength ladder bounded by the dial range and the existing overshoot/rollback rules. No match falls back to today's ladder.
- Ranges are fn-113's.

## Acceptance Criteria

- **R1:** A large named gap draws a larger stride than a small one, from a code-offered class — delivered by fn-149.
- **R2:** Class coverage tested against labelled cases incl. the palm's "far too short" — delivered by fn-149.
- **R3:** A replay of fn-80's palm rounds reaches the frond target in fewer than seven rounds, or says why not — delivered by fn-149.
- **R4:** Overshoot steps back rather than oscillating — delivered by fn-149.
- **R5:** Gate green — delivered by fn-149.

## Result

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (`c2ac430b`).

## Open

None.
