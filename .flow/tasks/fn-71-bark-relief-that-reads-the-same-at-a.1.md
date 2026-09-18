---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-71-bark-relief-that-reads-the-same-at-a.1 Implement Bark relief that reads the same at a distance

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Round 2, on the owner's R4 verdict: detail now leaves at the pixel, not an octave before it. The grain is box-averaged over the pixel by an exact integral of the lattice noise (`bark_noise2_box`), the ridge band's fade runs from one ridge width a pixel to two with the shortcut moved to match, the lichen fades from two pixels across to one, a lenticel dash leaves by its length, and the tints and cavity read over the range the band's fade took out of the height, scaled by the furrow row. The birch carries a 2 mm grain at 0.15 (2.90 against 3.0) and the beech's 2 mm at 0.3 reads 1.40. A footprint sweep (`.flow/evidence/fn71/sweep.py`, curves in `sweep/res1.md` and `sweep/res4.md`) shows the beech's 4x step of 0.62 at the 2 px band gone to 0.85, the mean within 2% along the walk; what is left at 4x to 8x is the lichen and lenticel discs integrated as one edge, named in the report. Every resolution receipt is green, redraws byte-identical, frame cost 7 to 17% over round one and recorded beside it. Report: `.flow/evidence/fn71/REPORT.md`, "Round 2". The owner judges the walk in the harness on 5174.

stage: impl-review - skipped(config: REVIEW_MODE=none)
stage: implement - skipped(reach: gpt-6-astra unreachable, codex usage limit until 2026-09-19 16:27; session model used)
## Evidence
- Commits:
- Tests:
- PRs:
