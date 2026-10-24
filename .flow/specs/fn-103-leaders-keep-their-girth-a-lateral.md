# Leaders keep their girth: a lateral share at forks in the radius law

## Goal

The beech reference shows two strong ascending leaders losing girth slowly almost to the top, with thin laterals that thin further each order. fn-68's best beech split the wood evenly at every fork, so three metres above the fork leaders were no thicker than their laterals (the reviewer's "exposed thick scaffold limbs" and "crossing branches" for 44 rounds). The radius law (ground radius, fork exponent, length taper, taper cap) has no knob for who gets the wood at a fork; this spec adds that asymmetry for fn-62.

Owner quotes (2026-09-22, on the beech stills):
> "If you look at the reference you have two massive strong stems that grow super high that only taper towards the top. And branches at the top taper more? (They're thinner than bottom ones). Currently it seems like the dials can only achieve thin branches if they taper evenly all over. This needs a new parameter i'd say."
> "Because the left most packet clearly attempted to have the strong center stem"
> "yes draft it"

## Decisions

- Design: one row `/radii/lateralShare` (0-1, default 1.0, byte-identical at default); a non-continuation child contributes `proximal^e * lateralShare` to the parent's sum and leaves the junction scaled by `lateralShare^(1/e)`. Continuation child: generation-time mark if it exists, otherwise least direction change. Structural forks only; no species branch.
- The overarching issue (host, code checked 2026-09-22): radius is the generator's only hierarchy signal — laterals, twigs, pendulous shoots and wood-borne foliage all switch on at radius thresholds. With an even fork split every branch crosses them at the same distance, so the veil, the ascending outer sprays and the same-gauge lattice are one symptom. Hence fn-104 and fn-105 measure this spec's tree before adding any parameter; fn-106 (bark) is independent.
- Closed as folded into fn-170 (2026-09-27, #131): the lateral share and continuation roles were built there as `lateralShare` and the codominant fork weight.

## Acceptance Criteria

- **R1:** `lateralShare` on `/radii`, default 1.0, range-checked, presets byte-identical at default — delivered by fn-170.
- **R2:** Red/green synthetic fork test of the split — delivered by fn-170.
- **R3:** One recorded deterministic continuation rule per structural fork — delivered by fn-170.
- **R4:** Beech bare-view still pair at 1.0 and a lower share for the owner — moved to fn-170/fn-62.
- **R5:** Dial table row and green gate — delivered by fn-170.

## Result

Not built under this spec; folded into fn-170 (#131).

## Open

None here.
