## Goal

The owner, on the round-5 silver birch pair, saw fn-37's curtain as rods: laterals left the limb in one fixed direction and ran straight. A weeping shoot is a slender stem bent by its own weight. This spec adds one row, a sag, that turns a hanging shoot toward vertical along its run as a smooth arc, neutral at today's straight rod, judged on the birch's matched pairs.

## Decisions

- user (2026-09-15, on the round-5 S-WHOLE pair of the silver birch): "I think the worst part is the hanging curtains are not affected by gravity or smth. It's clearly wrong."
- user (2026-09-15, on the beech's second stem): "Second trunk is missing but that's coming later that's fine."
- The sag is a per-step turn along the run (steepest at the attachment), not a departure angle; fn-37's droop, floor and separation stay. One fractional row over degrees per metre, so it reads the same on short and long curtains and blends.
- Boundaries: no wind or dynamics (fn-15), no change to shoot count, length or spacing (fn-37), no second stem (fn-38), no shoot colour (fn-40).

## Acceptance Criteria

- **R1:** `skeleton.twigs.sag` row, rail 0 to 1, neutral 0, refused by name, on the wire, blended, in browser metadata and harness; every preset byte-identical at neutral — met (birch re-pinned once with the reason).
- **R2:** with positive sag the heading turns monotonically toward down as a smooth arc steepest at the attachment — met.
- **R3:** the birch states a sag; matched pairs re-rendered beside the previous round; owner judges in fn-34 — met (sag 1, round 6b; rows `round6b-fn44` in `.flow/evidence/fn34/rounds.tsv`, 48/48 protocol).
- **R4:** tests for neutral identity, rail, monotone turn and end angles at sag 1 and 0.5, no step turning more than the last, floor, determinism, blend walk — met (`tests/sag.rs`).

## Result

The heading is carried beside the course so the branch law's turn limit does not bound weight. Measured limit: the sag is promised over the 3.5 m pendulous length but the birch's shoots ran 1.07 m on average before the floor or shell stopped them, reaching only 19 to 34 degrees below horizontal (addressed later by fn-47 and the shell work). The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62.

## Open

None.
