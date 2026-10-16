# Tuning aims at every trait the generator can draw

## Goal

The date palm's three tuning revisions aimed only at the owner's three written priorities; the reviewer's inventory held sixteen traits (frond colour, trunk colour and weathering, fibrous matting, crown silhouette, leaflet arrangement...), none an objective, so the materials track found "no supported dial" in 13 rounds and kept nothing. A species is tuned toward everything its references show that the generator can draw, owner's priorities first. Owner (2026-09-23): "And why was there no goal? we should tune all params that are relevant to a species."

## Decisions

Host design and escalation decisions (2026-09-23):
- The proposed approval is the owner's priorities, then every `core` and `secondary` inventory trait not listed `unexpressed` (fn-116); `variation` traits are recorded, not objectives. The owner may reorder or remove.
- A gap may carry an optional `track`; unassigned gaps belong to every track. Each track's stride class comes from its own lead objective; a track with no objective is skipped with a note. No new Jev question.
- Per-request cap 8 -> 16 objectives, approval up to 32 gaps. Code turns an inventory trait into a gap (views from its reference shots, evidence from renders plus references).

## Acceptance Criteria

- **R1:** Proposed approval = owner priorities then every expressible core/secondary trait; others recorded — met.
- **R2:** Each track's stride judged on its own lead objective (palm materials track) — met; `frond-colour-range` is `variation` in the palm inventory, so the test assigns the other two traits.
- **R3:** A track with no objective is skipped with a route note — met.
- **R4:** Gate green — met (955 passed).

## Result

Landed in `98195188`, `0c4d8c86`, `872fd73b` (`tuning/objectives.rs`); carried by the fn-149 rewrite (#121). Approved inventory traits become `owner-priority:` required cells, so the reviewer grades them and they count toward machine readiness; the readiness rule is unchanged.

## Open

None.
