---
satisfies: [R1, R4]
---
# fn-259-the-release-meets-the-bar-on-every.5 The scorecard swept over every shipped species

## Description
Task 4's protocol over every shipped preset and seed, at the product default ages and at matched ages, F4 and 0.1.4, with one-build rows where only memory is wanted. Ends in `SCORECARD.md` with a regression list: every row worse than 0.1.4 that no owner tolerance covers.

**Size:** M
**Files:** `.flow/evidence/fn-259-the-release-meets-the-bar-on-every/SCORECARD.md`, `raw/`
**Touches:** [.flow/evidence/fn-259-the-release-meets-the-bar-on-every/SCORECARD.md, .flow/evidence/fn-259-the-release-meets-the-bar-on-every/FRICTION.md]

### Approach
- Run per species in the order small to large (palm, ordinary, beech, oak, birch, spruce, Laurelin, Telperion), each under `telperion-heavy <mem> --gpu <vram>` with free device memory recorded; resume from the progress file.
- A species 0.1.4 does not ship is reported F4-only with that reason.
- Tables: default ages; matched ages (diagnostic); per-tree field; download bytes. Each cell median [range] over the stated samples; failed rows marked.
- Regression list: row, measured gap (absolute and ratio), and whether an owner tolerance covers it, naming the tolerance; no other tolerance.

### Investigation targets
**Required:**
- `.flow/evidence/fn-259-the-release-meets-the-bar-on-every/PROTOCOL.md` (task 4)
- `.flow/specs/fn-237-release-the-tree-space-engine-replaces.md` - R0a and "The owner's release decisions (2026-10-16)"

### Key context
This is the capture budget's large run: one sweep, not repeated; a defect found mid-sweep is reported, not fixed (qualification only). Stop with FRICTION.md and NEEDS_HUMAN if a row exceeds 10 minutes or the GPU is held by another session.
## Acceptance
- [ ] Every shipped preset at seeds 1, 7, 42 at its default age on F4 and 0.1.4 (where shipped), plus matched-age rows, every R1 column filled or marked failed with the reason.
- [ ] Free device memory and other compute processes per row, contaminated rows rerun or marked invalid; one-build rows used for memory-only cells.
- [ ] The regression list names each worse row, its gap and the owner tolerance that covers it or "none".
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
