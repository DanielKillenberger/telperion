---
satisfies: [R1, R2, R3, R4]
---
# fn-109-the-apical-rosette-fronds-borne-only-at.1 Implement The apical rosette: fronds borne only at the apex of an unbranched stem

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Blocker — NEEDS_HUMAN (2026-09-22)

The rosette is built and every criterion but R4's last clause holds: R1's
vocabulary and derivation move, R2's species digests and catalogue pins are
untouched, R3's palm builds a frond crown at seed 1 with no foliage and no twig
wood below it, and the ten rows are in the dial table with meaning, range and
steps. The workspace gate stands at one failure,
`telperion-jev::tuning_engine::twelve_rounds_of_attempts_fold_into_a_digest_that_still_fits`:
the tuning loop's proposal state lists every score-visible dial in full, and
that list alone had 358 bytes of headroom under `PROPOSAL_CAP` (24,576) before
this spec — measured at 24,218 bytes with the ten rows removed and 25,888 with
them in. A score-visible dial costs about 160 bytes there, so the guard admits
two more geometry rows in total, and any capability spec that authors more
trips it. Every way out is a system-design call the spec and the design handoff
do not settle and that CLAUDE.md routes to the host: raise the cap to whatever
Jev's real limit is, group the proposal menu the way the run summary already
groups it (`the_routing_state_names_the_kinds_of_dial_rather_than_every_row`),
or scope `state.dials` to the priorities a round is actually tuning. Raising
the cap or dropping `score_visible` on rows that do move geometry would be
weakening the gate rather than answering it, so neither was done.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
