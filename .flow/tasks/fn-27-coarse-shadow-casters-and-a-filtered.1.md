---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-27-coarse-shadow-casters-and-a-filtered.1 Implement Coarse shadow casters and a filtered shadow

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Blocked:
NEEDS_HUMAN at the spec's early proof point, one owner decision away from the rest of the task.

The wood caster prefix is built, green and committed at 102769e: runs emitted in
descending largest-radius order with an additive run table whose spans tile the index
buffer, the four row values in the one field table, and the sun's depth pass drawing one
binary-searched prefix. On the oak at seed 7, 1600 by 1000, the default row and a valid
120-frame session, that took the shadow pass from fn-14's 1.813 ms to **1.0629 ms** and the
caster geometry from 8,255,000 to 90,760 triangles, with the ground shadow's silhouette and
dapple unchanged beside fn-14's hero at the same crop. The native total p50 is **4.2109 ms**,
down from 4.949 but still over R3's 3.8 ms. Because 1.0629 ms is above the early proof
point's "below about 0.9 ms" condition, that clause's stop was taken literally: the foliage
stride, the filtered comparison and normal offset, the record's caster counts, the browser
type declarations, the docs pass and both trees' clocks were neither implemented nor
measured, and nothing was retuned or retried. The remaining 1.0629 ms is 869,310 foliage
placements still casting in full, which is precisely what the unbuilt stride addresses, so
the question the owner owns is whether that floor was a stop before the stride or a health
check on the prefix. R1 is half met, R2, R4, R5 unmet, R7 half met; all four gates are green
and no pin or tolerance was loosened. Evidence, deviations and the two empty verdict slots:
.flow/evidence/fn27/REPORT.md. One defect to carry into whatever resumes this:
crates/telperion-core/examples/occupancy_audit.rs:152-181 reconstructs pre-permutation path
order and now reads each run's triangles from another run's span while its closing assert on
the total still holds; the run table is the fix and was not applied there.
## Evidence
- Commits:
- Tests:
- PRs:
