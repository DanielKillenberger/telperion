# Tree space: wander bends over a length

## Goal & Context

The owner judged the oak (fn-195, round 8) "structurally ok but very blob like and unnatural in subtle ways. Branches are a bit chaotic ... the reference looks calmer and more natural" (owner, 2026-10-05). The cause is checked in the code: `form.wander` draws an independent turn, of random angle and random direction, at every node (`geometry/lay.rs:83-90`). An axis's direction is therefore a random walk with no memory; wander can only add jitter at the scale of one internode, never a slow, deliberate bend. A real oak's limbs make a few large, slow turns while its fine wood stays even, and the rounds that raised wander to get "tortuous" limbs (fn-195 rounds 6 to 8) made every order jittery instead. The beech's known gap ("fewer, heavier limbs", its fan-shaped boughs) may share the cause (host, 2026-10-05).

## Design (host, 2026-10-05)

- **Curvature that persists.** Each axis carries a curvature vector (perpendicular to its heading) that evolves along the axis as a mean-reverting process keyed by lineage: at each node it relaxes toward zero over a correlation length and is kicked by a keyed draw. The turn applied at a node is that curvature times the internode's length.
- **Two settings per physiological age:** `form.wander` stays the amplitude (the stationary spread of the bend per metre), and a new `form.bend_length` (metres) is the correlation length. At `bend_length` 0 the process has no memory and reproduces today's per-node draw exactly (neutral: every existing tree byte-identical). Longer lengths give slow, smooth arcs of the same overall spread.
- **Continuity:** the curvature is a smooth function of the settings and the keyed draws; walk-tested within the bound of 30.
- **Inheritance:** a lateral starts with zero curvature (its own process); a continuation or relay carries its bearer's curvature on, so a limb's bend continues through its relays.

## Requirements

- **R1:** `bend_length`, neutral at 0 with every existing tree byte-identical (beech, spruce, oak, palm at seeds 1 and 7).
- **R2:** A test that at a long `bend_length` an axis's heading changes smoothly (successive turns correlated, with the expected correlation for the length) and the overall spread matches the neutral case's for the same `wander`; red first on the per-node draw.
- **R3:** Walk tests on `bend_length` and `wander` within the bound of 30.
- **R4:** The oak re-rendered with slow bends on the limbs and low wander on the boughs and fine wood, viewed by the host beside round 8 and S1.
- **R5:** Workspace gate; Codex review.

## Boundaries

Geometry only. No species values beyond the oak's R4 trial (the oak's values are fn-195's).
