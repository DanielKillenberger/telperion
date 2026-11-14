# The palm's trunk organs: persistent leaf bases and acanthophylls

## Goal

After fn-108 and fn-109 the date palm's gate named three missing capabilities; two are on the trunk: `persistent-leaf-base` (retained petiole bases clothing the trunk in the diamond pattern) and `acanthophyll` (basal leaflets modified as spines). The host's 2026-09-19 assessment read both as unsupported anatomy. This spec adds both, off by default, as fn-82's dependency, built in parallel with the palm's first tuning (gap loop option `trunk-organs-leaf-bases-and-acanthophylls`, Jev 0.14). Owner (2026-09-22): "we can do both no? tune and build the missing capabilities?" — the capability gate was waived for the first pass.

## Decisions

Host decisions on the strong-tier design handoff (2026-09-22):
- The leaf-base lattice's axial resolution comes from splitting the stem polyline for the bases, not a finer `stepDistance` (which would change every preset's skeleton).
- `persistent-leaf-base` requires the rosette: the bases are the rosette's own history and take its spiral from `rosette::frame`.
- The acanthophyll's cross-section stays the leaflet's; the owner's verdict on the trunk still decides whether it reads.

## Acceptance Criteria

- **R1:** The palm gate names both as expressed and halts on `infructescence` alone — vocabulary and derivable clauses met; gate rerun the host's.
- **R2:** Every shipped preset byte-identical with both organs absent — met (digests and pins unchanged; positive control).
- **R3:** Palm at seed 1 shows bases spiralling down from the crown and spines at each frond's base; trunk still for the owner — still rendered; owner's eye left to fn-82.
- **R4:** Rows in the dial table; gate green — met (eight rows, four on `SWITCHES_AT_ZERO`; 120 test binaries).

## Result

Eight canopy rows: retained bases of shed frond hung on every stem as wood after the radius solve, carrying the crown's spiral down the trunk, and basal spines on fronds. The palm's table states first values; fn-82 tunes. Departures from the handoff: a base is two nodes (the split needs an attach point on the axis); the crown frame is carried onto each local axis by rotation, not projection (projection turns the phase with the trunk's wander). Commits `07ab8dc0`, `040f0f6c`.

## Open

- A base's taper is the weathering term, not its own row.
- The growth path builds its own tree, so a grown specimen carries no bases; whether parity is owed is the host's call.

## Friction

- The spiral test named no frame to measure in (~15 min) -> verification lines must name the frame of a claimed angle.
- Two size assertions held a literal 24,576 after fn-109 raised `PROPOSAL_CAP` (~10 min) -> both now read the constant.
- `generation_limit_guard` only visible from a full gate run (~6 min) -> name it in the verification table for any new module.
- Merging master after the `canopy.rs` move and cfg-gating conflicted wholesale (~35 min) -> land move-only commits separately from gating; rebase promptly.
