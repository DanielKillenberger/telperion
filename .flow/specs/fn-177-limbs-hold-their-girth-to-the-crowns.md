## Goal

A London plane's limbs, and a beech's, stay thick almost to the crown's edge and then break into fine wood over a short distance; the generator's limbs thinned steadily from the fork. On fn-170's 8-seed plane candidate the owner confirmed the forks read and the limbs thin too early. This spec lets a table hold a limb's girth over most of its reach and taper it to leaf level near the crown's edge.

## Decisions

- owner (2026-09-27, fn-170 session, Basel Schützenmattpark photograph): "I feel our generator fundamentally lacks the ability to have strong limbs that quickly taper at the edge of the crown to leaf level."
- owner (2026-09-27, on fn-170's R7 London plane candidate): "birch looks good and the plane looks decent but yes it thins out too quickly for sure"
- The rule (host, 2026-09-27): one rule for all structural wood. Every part leaving the root, every lateral and every codominant sibling starts an axis; a fork's primary carries its axis on. Each axis holds its base girth (the pipe radius where it starts). A node's share of reach is t = d / (d + L), d measured from where the axis leaves its parent, L along the axis's own continuation to its tip (not through laterals, so a whorl-ended leader is not blunt). Radius is the larger of the pipe radius and the held girth, which holds to `girthHold` and then falls to the pipe radius by the tip as a smoothstep in log radius over `girthHold / girthFall` of the reach.
- Rows in `/radii`: `girthHold` 0 to 0.9 (0 is today's tree to the bit; the ceiling leaves a tenth of reach to fall), `girthFall` 0.5 to 8, default 2; catalogued, blended, on the dials; the hidden growth path ignores both.
- Given up: conservation at forks over the hold. The hold is applied in the final radius pass after twigs grow; foliage decisions read the pre-hold radii (`Tree::pipe`), so no twig or leaf is added or lost.
- Four refinements accepted by the host: the root is a point, axes are measured from where they leave the parent, leaf decisions read pre-hold radii, the rail stops at 0.9.
- R4 verdict, owner decision: merge the rows as a dial the beech needs (fn-62), and let the beech run's Tune and Gaps show whether the hierarchy is reachable or a generator gap.

## Acceptance Criteria

- **R1:** Radius along the plane's thickest limbs at 8 seeds, split into tip-count and length-taper parts — met: at mid-path the thickest limbs keep 14% of base girth; the pipe model explains 96% of the thinning (log terms), `lengthTaper` 4%. On 7 of 8 seeds the thick wood is codominant stems (not fn-61's limb systems), left at 43% of the path with 25% of the girth. `lateralShare` at its floor holds girth but ends blunt at 30% and regrows the tree; no existing row gives hold-then-fall.
- **R2:** `girthHold`, `girthFall` with rails, by name, wire, blend, metadata, dials; zero matches master on every preset at seeds 1 and 7 — met (16 of 16 identical; specimen schema 7).
- **R3:** Synthetic limb held within 1e-9 over the hold, pipe radius after the fall, whorl-ended leader not blunt, continuity under small steps — met (`crates/telperion-core/src/pipeline/radius/hold_tests.rs`, red first).
- **R4:** Plane candidate rendered with the rows at 8 seeds for the owner — met; rejected as the plane's fix (verdict below).
- **R5:** Gates green; cost against fn-170 — met: median build 771 vs 769 ms, peak 619 vs 618 MB, nodes and leaves identical.

## Result

The rows work as a dial and are neutral at zero. Owner verdict (2026-09-27, girthHold 0.75, girthFall 4): "it looks unnatural tbh. ... The structure of those trees is much different. Now we have 1 super thick center and then it tapers off to be thin very quickly anyway. The plane i sent has very thick multiple leaders and thick branches from that. Then there's actually very few branches off that secondary level that lead to many twigs with leaves." The missing piece is branching hierarchy (few strong scaffold limbs, sparse middle orders, dense twigs at the edge), taken up by fn-182. Reference photograph: `.flow/references/london-plane/schutzenmattpark-basel-2026-09-27.jpg`. Codex SHIP after a host-authorised review-round reset.

## Open

- A fork's primary keeps its axis's girth past the fork while a sibling starts thinner at its pipe share.
- Wood 10 cm and thicker grows only from 33 m to 41 m, because the pipe model still gives every limb a thin base.

## Friction

- Nothing read radius along a limb, and radius feeds growth -> `examples/limb_girth.rs` written; run R1 before marking a spec ready.
- Two new dial rows re-recorded the beech tape (8 Jev calls); `tape_trim` not idempotent; replay needs `headless` built by the gate -> fn-170's remedy again (tape key not hashing the dial table).
- The axis reach was defined twice (through laterals; restarting at a fork's primary) -> L along the continuation; whorl case added to the test.
- The review cap of 2 refused the re-review of the third fix -> host reset; a higher cap for pipeline-stage tasks proposed.
