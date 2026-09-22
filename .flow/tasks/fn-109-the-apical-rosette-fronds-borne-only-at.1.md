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
The apical rosette as a shared generator capability, off by default. `foliage/rosette.rs` is a third placement source at every stem apex (`Tree::stem_apices`, the last node of the stem run) bearing N fronds on a phyllotactic spiral; `fan` is the one expansion of a placement into leaflets along a rachis that the station walk, the short shoots and the rosette all call, so pinnate-compound is drawable without a rosette; `terminal_leaflet` is a 0 to 1 blend of the last leaflet's pitch onto the rachis so the count stays stems times fronds times leaflets. Ten canopy rows with validated ranges, doc comments, wire lines, blend buckets and dial-table rows. `branching::clear_apical_twigs` and `mesh::grow` make an apex that bears a rosette bear no twig wood on both draw paths. apical-rosette, pinnate-frond and pinnate-compound moved to the expressed list with derivable clauses. R1 vocabulary and derivation green (`tests/capability.rs`, `stages_downstream.rs`); the palm run's gate rerun is the host's after landing. R2: species digests over four species and the catalogue pins unchanged and un-repinned, with a positive control. R3: eight tests in `crates/telperion-core/tests/rosette.rs` and one still at seed 1 showing a rosette of fronds on a bare stem; leaflet size and the trunk's habit are fn-82's tuning; the owner's look is pending. R4: dial-table coverage green; the workspace gate's one failure, the tuning loop's 24 KiB proposal-state cap with 358 bytes of headroom, was a size discipline set without a measured limit and was raised to 32 KiB by the host (fn-68's ledger holds Jev answering 34,000-token batches). Implemented on the strong tier at low effort as fn-80's dispatch-7; two corrections to the design handoff are recorded (the apex is the stem run's last node, not a childless tip; the terminal leaflet is a blend, not an extra instance).
## Evidence
- Commits: 90b61031, 9c08aa55, 31f1f763
- Tests: env -u TYPESAFE_API_KEY cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-core --test rosette, cargo test --profile ci -p telperion-jev --test tuning_engine
- PRs:
## Resolution of the blocker (host, 2026-09-22)

The proposal-state cap was a size discipline the fn-68 implementer set at 24 KiB without a measured limit; the fn-68 ledger holds proposal batches of 34,000 input tokens that Jev answered. Raised to 32 KiB in 31f1f763 with that rationale in the doc comment, and the digest test made cap-relative. Scoping the proposal menu to the priorities a round is tuning is the structural fix and is proposed in fn-80's friction, not built here. Gate rerun once at the end: EXIT=0, 894 passed.
