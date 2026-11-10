# What bounds a tree: vigour falls with the path from the root, and dead wood is not charged

## Conversation Evidence

> goal, owner, 9 Nov 2026: "0.2.0 looks bad. But .2 is better conceptually with the botanically accurate way of growing it just needs to be even better and grow larger trees more efficiently"
> goal R2: "height slows as the path from the root lengthens."
> owner, 10 Nov 2026: "today's ok at 120 isn't that bad honestly."
> owner, 10 Nov 2026, on the oak at 300: "i mean wtf is that ok at 300..?"
> AGENTS.md, owner, 3 Nov 2026: "We don't want artificial limits like that i've said that before"

## Goal & Context

Child of goal fn-321. fn-344 measured what bounds today's oak: one artefact. A limb is charged upkeep for wood that has already died, which is 78 to 97% of the tree's whole upkeep from 49 to 129, overtakes its light income at 109 and sheds the fork limbs at 132; after that the oak is a bare pole. Light itself holds the crown's density steady (about 125 income and 650 tips per square metre of ground) and bounds nothing else: with the dead-wood charge off, the crown widens and rises without end. [paraphrase]

fn-344's instrument H (dead wood not charged, and each new internode shortened by 1 - L / 34.8 m, L the path from the root) levelled the oak at 25 to 27 m tall and 22 to 25 m wide with its limbs alive from 4.4 m at 260. The host viewed it: the silhouette at 200 and 260 is an old open-grown oak's where today's is a pole. It is not a candidate: only the internode's length was shortened, so the same number of nodes pile into a fixed space (tips per square metre rise from 1,470 to 3,040, limbs swell into knots, the device refuses the tree at 300); the oak at 120 loses today's open two-limb V and stands 18.8 m where today's stands 25.1; and the bole shows a girth step. [paraphrase]

This child turns those two findings into rules and proves them on today's structure. [inferred]

## Acceptance Criteria

- **R1 (the rules on paper, and back to the host):** a note giving, checked against the texts and with an unchecked claim marked: what published work states about extension growth declining with path length or height (hydraulic limitation; the standard height-growth forms and what their asymptote is taken from), and whether any source gives the decline as a function of path or only of age; in the grower, every place a shoot's yearly growth is set (the vigour split, node counts, internode length) and which of them a decline in vigour must reach so that fewer and shorter shoots are made and not the same nodes packed closer; what in H made the bole's girth step and the hollow tube at 4 m; and, from H's and A's logged years, how far H's oak at 40 and 120 falls below today's and what single rescaling of the species' growth values would hold the oak at 40 where it is today. No code is built before the host answers. [inferred]
- **R2 (the probe):** on the host's word, both rules are built into the cycle, each changing the tree by degree and dormant at a neutral value. [inferred]
- **R3 (the read):** leafless and leafed stills, today's beside the probe: the oak at 40, 120, 200 and 300 on seeds 1001 and 1007, the beech at 40 and 80, every other preset at 40. Beside each: height, spread, trunk, lowest living limb, current-year leaf, held phytomers, peak memory and one-thread seconds; and the yearly census to 300 showing whether held phytomers and living tips level off. The worker views every still and says plainly per tree: better, the same or worse than today's, and why. [inferred]
- **R4 (the host judges first):** the host views every sheet. The owner is shown the result as a candidate only if the oak at 40 and 120 reads at least as good as today's, the oak at 200 and 300 reads as an old oak, and no other species is harmed. [paraphrase]
- **R5 (verification):** the probe leaves the crates as a patch in evidence; where any code stays the gate is green and the codex review ends SHIP. [paraphrase]

## Boundaries

- No cap: growth slows by a rule over a tip's own path and never stops at a count or a height. The species' value in that rule is a measured trait with its source, or is listed as an estimate with its reason. [paraphrase]
- No value fitted per species and none tuned to a sheet. [paraphrase]
- Girth, sag, the insertion angles and the foliage are not changed here. [paraphrase]
- Heavy work runs through `telperion-heavy`; every grow prints its yearly census and stops itself before the desk's memory. Nothing merges to master, nothing is tagged or published.

## Decision Context

Settled by fn-344's measurements: the dead-wood charge is an artefact and is the cause of the old oak's collapse; without it something must bound the crown, and light does not. The host's direction, for R1 to check: a tip's vigour, and so both how many nodes it makes and how long they are, falls as its path from the root lengthens, by the species' measured potential height; and the species' growth values are restated so a young tree grows as it does today. [inferred]

Not settled, the host's on R1: the form of the decline; whether it acts on the vigour a tip is allotted or on what it does with it; how the young tree is held. [inferred]

## Open Questions

None.

## Result

Closed 10 Nov 2026 as a failed read; nothing went to the owner as a candidate. `R1-NOTE.md`, `R1-ANSWERS.md` and `RESULT.md` are in this spec's evidence with `sheet-oak-1001.jpg`, `sheet-oak-1007.jpg` and `sheet-others.jpg`. The host viewed the oak's sheet.

- **The rule as the host wrote it:** each growth unit's node count multiplied by 1 - L / L_max (L the tip's path, L_max 34.8 m for the oak), wood that has died not charged, every row's node counts restated by 1.192 so the oak at 40 holds its height (within 1.4% on three seeds at the first try).
- **It bounds the tree and strips it.** Held phytomers peak at 784,000 in year 99 and fall to 329,000 by 299; height levels at 27 m; the grow to 300 takes 22 s and 0.7 GiB. But the oak at 120 carries 65,000 leaves against today's 305,000 on a 69 cm trunk against 143, and at 200 and 300 it is a tangle of thin, curling, nearly leafless wood. The host's read agrees with the worker's: worse than today's from 120 on. The beech at 80 loses 72% of its leaf and the birch at 40 loses 67%.
- **Why, as far as it is known:** tips far along a path make one node or none, so the short-lived fine wood that clothes the crown is not replaced (inferred from the census, not traced). And the linear fall is paid from the first metre, on top of the slowing with age the rows already carry, so the tree's middle years are thinned twice.
- **What stands from this child:** no source states a decline against path length, only against height, and the one closed form has a fitted asymptote; a shoot's node count never reads its vigour and sizes are normalised per row, so nothing in the grower today can make a shoot grow less; the bole's girth step is today's behaviour (0.711 to 0.546 m across the fork's node).
- **What the host got wrong:** it wrote a rule to zero for every shoot where the measured decline is of the tree's height, and it kept the rows' own slowing with age beside it.
- R5: no code stays; the probe is `probes/n.patch`. No gate and no review ran.

## Friction

- **The local shell hook refused redirects written inside a heredoc** (2 minutes). Local, reported and not specced.

