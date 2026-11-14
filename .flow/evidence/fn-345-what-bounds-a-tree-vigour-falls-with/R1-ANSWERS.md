# fn-345: the host's answers to R1-NOTE.md, 10 Nov 2026

Each is the host's design decision; none is the owner's ruling. The note changes the direction in two places: vigour cannot carry a decline, since sizes are normalised per row and the node count never reads it; and no source states a decline against path. Both are taken as found.

## The rule

A tip's yearly growth unit makes fewer nodes as its path from the root lengthens:

    nodes made = nodes the zone's rule gives  *  max(0, 1 - L / L_max)

- `L` is the path along the wood from the root collar to the tip at the start of the year, as H carried it.
- `L_max` is the species' potential maximum height, a measured trait with its source. Check the oak's 34.8 m against its source's text before the probe runs; if it cannot be read, use it and mark it unchecked.
- The product is a real number. It becomes a count by a keyed draw (the whole part, plus one with the probability of the fraction), so the rule changes the tree by degree and is dormant where `L_max` is unset.
- **Internode length is not touched.** H shortened the length and kept the nodes, and the note shows what that does: the same nodes, laterals and pipes in less space. A slowed shoot here is a shorter shoot of fewer nodes, with every node as it is today.
- A unit whose count comes out 0 makes nothing that year and its tip lives on; say what the grower does with such a tip today (a zone with no nodes) and use that. If there is no such path, stop and return.

Why this form, and what is an estimate:

- The linear fall to nothing at a maximum is what the measured work states against height: traits fall linearly with height and are extrapolated to a limit (Koch 2004), and the monomolecular form is growth proportional to 1 - H / A. Both are abstract-level reads; marked so.
- Reading the tip's **path** in place of its height is the host's extension, an estimate: for a leader the two are the same; for a limb the proposed mechanism (Ryan and Yoder) is path. No source measures it. It is taken because fn-344 measured that something must bound the crown's spread as well as its height, and height alone cannot. Yoshimura's top shoots being the longest in a crown does not contradict it (the top is also the best lit), and is named in the result.
- Acting on the node count and not the length is the host's reading of where a short shoot differs from a long one. Unchecked; listed as an estimate.

## With it

- **Wood that has died is not charged**, as fn-344's B.
- **The young tree is held by restating the species' node counts once.** Today's counts were set to give the oak its sourced young growth with no decline, so they are the counts after a decline nobody had written down. One factor on every row's node counts (through the same keyed draw) such that the oak's height at 40 on seeds 1001, 1007 and 1012 matches today's within 3%. Find it by arithmetic first, then two or three runs; it is a restatement to a measured quantity and is reported with its value. It is not moved to make any sheet look better.
- **The oak at 120 is allowed to stand lower than today's.** The arithmetic says about 20.5 m against 25.9; the published forest figure is 24 m at 95 to 135, and an open-grown oak stands below a forest one. What must hold at 120 is how it reads.
- **Today's slowing with age stays as it is.** One thing changes at a time.
- **Other species:** each takes its own `L_max` where you can read a sourced potential height within fifteen minutes (beech, spruce, birch); one you cannot source runs with the rule dormant, and the sheet says so. Dead wood is not charged on any. Their node counts are not restated here; report how far each falls below today's at its preset age.

## The other questions

- **The girth step** is today's behaviour made larger and is not this child's. Report its size under the probe.
- **The open tube:** no separate run. If it shows in the probe's stills, say so and at which join.
- **The memory reading:** enough as it stands. Recorded as not reproduced.

## The run

1. The restating factor, by arithmetic and then runs at 40.
2. The census to 300 on seed 1001, stopping itself as before (eight times the most today's tree had held, or 20 GiB): do held phytomers and living tips level off? If they are still rising at 300 at a rate that will not fit the device, stop and return with the curve before any stills.
3. R3's stills and measures as the spec lists them, today's beside the probe.
4. Your plain read per tree: better, the same or worse than today's, and why. Look in particular at whether the oak at 120 keeps an open crown of few heavy limbs, whether shoots at the crown's edge read as stubby or bare, and whether any limb swells or knots.

Return then. The host views every sheet before anything else happens.
