# fn-341: the host's answers to R1-MEASURES.md and R2-RULES.md, 10 Nov 2026

Each is the host's design decision; none is the owner's ruling. R1 orders the work: the trunk's girth is furthest from real trees, the old oak's bare trunk and its thinning come next, and both trace to how wood is counted and charged. So the probe takes the three rules that have a stated form and measured traits, and leaves the two that have neither.

## What enters the probe, in this order

### G. Girth by rings

A section's area is the sum of its yearly rings, and a year's ring is proportional to the leaf the section carries above it that year:

    ring(t) = A_L_above(t) / (R * N)        section(t) = sum of ring over the section's years

- `R` is the species' measured leaf area per sapwood area (Phillips 2003 for the oak; Bartelink 1997 and McDowell 2002 for the beech, which disagree, so both are run on paper and the checked one nearer its source's own trees is taken and the other named).
- `N` is the species' number of sapwood rings, a measured wood-anatomy trait. The worker sources it from the text for *Q. garryana* or its nearest measured relative and for *Fagus sylvatica*, and marks an unread one unchecked. Where a species has no heartwood boundary the source's own statement of its conducting rings stands.
- Why this form: Shinozaki's statement gives sapwood = leaf above over R at any moment. If the sapwood is the last N rings and the leaf above changes slowly over N years, each ring is that sapwood over N. Summed, it gives White's two statements at once: rings of constant area once the crown is at its optimum, and a stem that never thins, since a ring once laid stays. It is option (c) with its coefficient taken from two measured traits and nothing estimated.
- The leaf above is the standing leaf (the current year's on a deciduous species).
- It replaces every age's `pipe`, `ripening`, `retained`, `exponent` and `leaf_girth` in the probe. Fork sums then hold by construction, since leaf adds.
- **On paper before any code:** with the yearly leaf above the trunk at 1.3 m from a traced run of the oak (seed 1001, to 300) and the beech (to 80), compute the trunk at 40, 80, 120 and 300 by this formula and set it beside White's and Phillips' ranges. If it falls outside half to twice a range at any age, stop and return with the arithmetic. The host's own rough figure is 24 cm at 40 and about 60 cm at 80 for the oak, unchecked.

### P. Posture by gravitropic performance

Option (a), as a share by degree. For a section of diameter D bearing length H beyond it at lean φ, with eq. 39's `sin φ_max`:

    held = min(1, sin φ_max / sin φ)        the sag acts on the share (1 - held) of the moment

- So wood that can hold its lean keeps its growth angle, and only the share its reaction wood cannot hold bends, which is the source's "Pg below 1 bends downward" read by degree. This answers fn-338's three questions: the form is the limit and not the incremental sum (fn-337 showed the sum bends more than today); the share is scaled by the ratio above; one set of trait values for every species, Fig. 8's angiosperm means, listed as an estimate with that reason, and the source's conifer means for conifers if it gives them.
- With G before it, limbs are thinner than today's, so P reads G's diameters. That order is intended: girth earns posture.
- `straightening` and `erection` stay as they are in the probe and are named in the result.

### U. Upkeep on living wood

A branch's balance is charged for its sapwood, the last N rings of each section times its length, and not for all the wood it ever grew.

- Why: fn-340's trace found that 57 to 66% of the twig upkeep an oak branch pays at 20 to 30 is for twigs already dead, charged for as long as the branch stands (`retire.rs`). R1 finds the old oak with one living limb and a trunk bare to 21 to 30 m. Dead wood and heartwood do not respire. The worker sources this from the text before building (maintenance respiration against sapwood volume; Ryan 1990 and Sprugel 1990 are where the host would look, both unchecked), and if the sources do not state it, U is left out and that is said.
- The species' `upkeep` value stays as it is, applied to sapwood volume. It is an estimate today and is listed as one.
- U needs G's rings, so it is built after G.

## What stays out, and why

- **Vigour across orders.** Pałubicki's form is stated, but it replaces every age's node counts and internode, and option (b) against the whole tree's mean would size a leader by its vigour against a twig's. It needs its own derivation and is not a probe change. It is the next rule after the owner has seen these sheets.
- **Rank by vigour** and **height by path length.** No source gives a form. Option (c) for height: with G and U in, measure whether the oak at 300 still overshoots 34.8 m, and report it.

## How it is run

1. G's paper check, then G alone on one oak at 80 (seed 1001): its measured trunk and limb diameters beside the paper figure.
2. P added: the stated limb's chord angle and the crown's spread beside R2's paper direction.
3. U added: the lowest living limb and the crown ratio at 120 beside the cycle's.
4. If a rule's measured direction is opposite to its paper direction, or a tree fails to grow or loses its crown, stop and return with it. Otherwise go on to R4's sheets and read without returning, and draft RESULT.md as R5 writes it.
5. R4's "every other preset" runs with the oak's angiosperm values for P and each species' own `R` and `N` where the worker can source them in an hour; a species with no sourced `R` or `N` is run on the cycle's girth and named.

The probe is a patch; the crates end as they began.
