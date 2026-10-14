---
satisfies: [R1, R1b, R2, R3]
---
# fn-251-a-walk-never-grows-a-tree-larger-than.1 Repro, natural-scale rows, the size factor questioned, the sweep

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Blocked:
Waits for fn-255 (one light-reading grower); host decisions in the spec, 2026-10-09. Measurement and red repro on branch walksize (d4cc14c4, 28b0200f).

Blocked:
NEEDS_HUMAN (host design call). Re-measured on F4 after fn-255 (958654d48, WALKS.md "Re-measured on F4"): the repro builds; 70 of 80 frames meet the oracle and none fails on the budget. The 10 that fail have three causes: the linear walk predicts below both ends (oak and spruce, 5 frames, out of reach for a shrink-only factor); one seed's kept count spreads by more than the 10 % widening (4 frames); whole phytomers at a seedling (1 frame). The fix needs a design decision first.

Blocked:
NEEDS_HUMAN (host decision 1, fail-early). The lateral rows the size factor moves have room for only x1.07 to x1.21 before a zone's laterals sum to 1. At full room, oak <-> spruce at 40 stays 47 % (t = 0.25/0.75) and 32 % (t = 0.5) short of the predicted geometric-mean target, and at 80, t = 0.5, 6 % short. Nothing is built. Evidence: WALKS.md "The two-sided factor's room" (52160a8f2).

Blocked:
NEEDS_HUMAN (a film pair fails). Under the seed-spread oracle 86 of 95 frames meet it. Film pairs: birch -> oak at 40 keeps 0.58-0.65M against a ceiling of 0.537M around t = 0.5-0.6 (seeds 7 and 42). Palm -> Laurelin at 3 fails at seeds 1, 7 and 42 (31, 28 and 30 low; 138 high against 97); at age 2 and age 1 every frame meets it. The other failures are oak -> birch 80 t = 0.01, oak -> ordinary 40 t = 0.75, beech -> oak 80 t = 0.5, and the named oak <-> spruce exception. WALKS.md "The seed-spread oracle" (e4088f133).

Blocked:
BLOCKED: TOOLING_FAILURE. The workspace gate is OOM-killed inside telperion-core's unit tests (src/lib.rs, untouched by this task) at 10 GB/6 threads, 9 GB/4 threads and 9 GB/2 threads; every_preset_grows_the_same_tree_again_and_on_any_cores alone needs more than 9 GB. The task's work is committed (8cf010423); walk_sizes passes alone in 53 s and goes red when a record is tightened. Needs a larger gate cap or one-at-a-time heavy unit tests; then npm test, the codex review and done. FRICTION.md 2026-10-14.

Blocked:
BLOCKED: TOOLING_FAILURE. The workspace gate was OOM-killed at 13 GB with 4 threads (190 s, 13G peak) in telperion-core's unit tests (src/lib.rs, untouched by this task), with every_preset_grows_the_same_tree_again_and_on_any_cores running. The earlier caps were 10 GB/6, 9 GB/4 and 9 GB/2. The task's work is committed (8cf010423). npm test, the codex review and done wait on a gate that fits.
## Evidence
- Commits:
- Tests:
- PRs:
