# fn-343: the host's answers to R1-NOTE.md, 10 Nov 2026

Each is the host's design decision; none is the owner's ruling.

The host's direction did not fit and is withdrawn. The lateral that takes the line is no continuation: it is a side branch born years earlier and well down the axis, and it is the line for one reason only, that the pipe model makes it the thicker. So nothing in the grower's events should change. What is wrong is narrower: girth says which wood is the line, and the geometry does not listen. A lateral stands at its insertion angle whether it carries a hundredth of the section past the joint or nearly all of it.

## The rule: a fork shares its angle by section

At a joint where a bearer of section `A_b` (its own wood past the joint) bears a lateral of section `A_l`, the angle between the two stays what it is today, the lateral's insertion angle `θ`. What changes is how that angle is shared about the bearer's line below the joint:

    lateral's turn from the line below  = θ * A_b / (A_b + A_l)
    bearer's turn from the line below   = θ * A_l / (A_b + A_l)      (to the far side, in the lateral's plane)

- A thin lateral on thick wood stands at almost its full insertion angle and the bearer barely notices, as today.
- A lateral that carries most of the section runs on nearly straight, and the thin wood past the joint is the one that stands off. With the measured radius ratios (the bearer's own wood at 0.13 to 0.14 of the lateral after a move-on, 0.43 to 0.52 after a relay) the thick line turns about 1 degree and about 9 to 12 degrees, where it turns 57 today.
- Two equal forks each take half, a symmetric Y.

Why this form: it has no setting. It reads two sections the pipe model already gives and the insertion angle the species already has, it changes the tree by degree, and it cannot fit a species or a sheet. The source the host has in mind is Murray 1927 (J. Gen. Physiol. 10), on branching angles following from the sections at a fork, unchecked. Check it against its text. If it states a different form (the host recalls a relation on the sines of the two angles weighted by section), report the stated form beside this one and build the stated one; if the text cannot be read, build this one and mark the source unchecked.

For the probe only, one blend `s` from 0 to 1 multiplies the change so the probe is dormant at 0. It is not a product setting.

## Before any code: the arithmetic on the joints already logged

From `raw/`'s tagged joints (each has both radii), recompute every joint's two turns under the rule and give, at 40 and 120:

- joints of 10 degrees or more per metre on wood 1 cm or thicker, today and under the rule;
- how many joints the rule makes worse: a bearer that today runs straight and would turn 10 degrees or more because of a lateral of comparable section;
- the same on limbs and boughs alone.

If the rule does not at least halve the sharp joints on wood 1 cm or thicker at both ages, stop and return with the table.

## Then the probe

- Where: the step that lays or settles the tree with final girth known. Sag already turns a subtree about its joint at settle; this is the same kind of turn. Say where you put it and why. If girth is not available where laterals are framed, apply it as a posture step at settle and say what that costs.
- The turn applies about the joint to the whole subtree on each side, in the plane of the lateral and the line below.
- Nothing else changes: girth, sag, the balance, the relay and take-over mechanism, every setting.

## Then R3's read, with these additions

- Leafless and leafed, today's beside the probe, as the spec says.
- For every probe tree say plainly: better, the same or worse than today's, and why. Look in particular for a bearer made to zigzag by its laterals, a crown that has changed outline, and any species whose character is lost (the birch's hanging boughs, the spruce's tiers).
- Return then. The host views every sheet before anything else happens.
