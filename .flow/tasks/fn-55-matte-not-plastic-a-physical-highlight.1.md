---
satisfies: [R1, R2, R3, R4]
---
# fn-55-matte-not-plastic-a-physical-highlight.1 Implement Matte, not plastic: a physical highlight and a pixel grain

## Description
The physical highlight and the grain rows, implemented in-host (the routed
gpt-6-astra bridge refused on quota). Report: `.flow/evidence/fn55/REPORT.md`.

## NEEDS_HUMAN: the 4x distance bound, and the grain's value
The bark distance series reads 3.134/255 at 4x against its bound of 3.0
(base 2.904); with the highlight removed entirely it reads 3.167, so the
cause is the old 15%-of-sun sheen that sat the lit trunk on the tone curve's
shoulder and compressed the relief's own cross-resolution error, not the new
term. The bound cannot hold without that sheen and the spec names it
unchanged; the owner decides whether the byte-domain bound is recalibrated
(3.25 covers it) or the relief's filtering is its own spec. The bark grain
rows ship held off: at any value visible in S-BARK or B-BASE a cell sits on
the half-size still's Nyquist edge and the smooth-bark bound reads 3.4 to
4.5 against 3.0.

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
