---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-71-bark-relief-that-reads-the-same-at-a.1 Implement Bark relief that reads the same at a distance

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Two renderer terms in `wood.wgsl` give the far draw the shade of the slopes its footprint lost and read the crest and fissure tints as the box mean of a ramp across the shading cell; the oak's 4x distance series falls from 3.134 to 2.532 with the bound restored to 3.0 and the 3.25 interval's reason beside it, every resolution test writes its margins to `.flow/evidence/fn71/resolution/`, the beech carries a 2 mm bark grain at 0.3 (2.37 against 3.0), and the birch's grain stays held off, refused by the bound at 2.999 with no grain at all. Report: `.flow/evidence/fn71/REPORT.md`. Open against the spec: the spruce is not in the distance series, because its bare crown's twig shadows put plain wood at 6.2/255 at every strip the fixture can frame (the grazing fixture holds it at 2.707); the birch's grain value needs an exact box prefilter of the grain, the next step named in the report.

stage: impl-review - skipped(config: REVIEW_MODE=none)
stage: implement - skipped(reach: gpt-6-astra unreachable, codex usage limit until 2026-09-19 16:27; session model used)
## Evidence
- Commits:
- Tests:
- PRs:
