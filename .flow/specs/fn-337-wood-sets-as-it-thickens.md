# Wood sets as it thickens: a limb bends under the load added while it is thin

## Conversation Evidence

> goal, owner, on the full oak at 200: "yea that's bad"
> goal, owner, 3 Nov 2026: "But then are we able to realistically grow 300 year old trees? That's really the goal here."
> goal, owner, 9 Nov 2026: "it just needs to be even better and grow larger trees more efficiently"
> goal R4: "At 200 and at 300 the crown's mean sits within 3 m of the trunk's base on the median seed."
> goal R4: "Each rule added for this is a rule of the one cycle for every species, its rows sourced, with what it does to every preset at its preset age measured and shown."

## Goal & Context

Child of goal fn-321, serving R4, the first of the three rules its Decision Context expects. The old oak's trunk bends near level by 200 and loops on itself at 300 (fn-336's sheet). fn-332 measured sag as the largest single cause: with it off, the crown's mean at 200 sits 7.9 m off the base against 15.7 m. [paraphrase]

Read from code, 9 Nov 2026: sag is taken once, when the tree is laid in the settle, from the finished tree's whole load at each phytomer's final radius (`grow/settle.rs`, `sag.rs`: "Growth never reads it back"). So a tree at any age is bent as if all its present weight had been hung on its present wood at once. A real limb is not: wood laid down in a year is laid in the shape the limb then has, so only load added afterwards bends it, and by then the limb is thicker. Bending the whole final load at once is the same as that only where nothing changed in between. [paraphrase]

Premise, not yet run: a phytomer's bend is the sum, over the years it has stood, of each year's added moment over its radius to the fourth in that year. On a young tree this is close to today's bend, and on an old one far less, since a limb's early load met thin wood and its late load meets thick wood. Unknown: what a year's reading of load costs the cycle, and how much of the old oak's bend remains, since fn-332 found no single cause accounts for all of it. [inferred]

## Acceptance Criteria

- **R1 (sourced first):** before any code, a note in evidence states the rule and what it rests on in the published mechanics of growing trees (incremental bending of a beam that thickens while it is loaded; the place of reaction wood, which this child does not model), each claim checked against its source's text, and says in one line what the rule leaves out. A rule its sources do not support returns to the host. [inferred]
- **R2 (the rule, in the one cycle):** every species' sag follows the wood's history: each year's added load bends each phytomer at that year's radius, and what a year has bent stays. It is one rule with no new row and no value per species; `sag` keeps its meaning and its sourced values. With sag at nothing the tree is unchanged to the bit. [paraphrase]
- **R3 (a tree at any age):** the tree at an age is the same whether grown to it in one call or paused and resumed on the way, and the years on the way are states of the same build, as today. [paraphrase]
- **R4 (the old oak, measured):** fn-332's probe on the oak at 40, 80, 120, 200 and 300, seeds 1, 7 and 42, before and after: height, where the trunk's path ends, the crown's mean off the base, and the steepest lean over a metre. Stills of the same trees at 120, 200 and 300 on one retained sheet beside fn-336's. [paraphrase]
- **R5 (every preset, shown):** every preset at its preset age, seeds 1, 7 and 42, before and after: height, spread, trunk and the whole-tree still, on retained sheets, so what the rule does to trees the owner has already seen is visible. Pinned digests and baselines that change are updated with that evidence (`AGENTS.md`, "Generator evolution"). [paraphrase]
- **R6 (cost):** the full cycle's time at each preset's age and the oak's at 300, before and after, one thread, by alternating batches; a rise is stated with where it comes from. [inferred]
- **R7 (verification):** the gate is green and the codex review of the child's diff ends SHIP. [paraphrase]

## Boundaries

- No value fitted per species; no new preset row. [paraphrase]
- This child does not make limbs share the crown and does not limit height; those are the next rules, and their absence is not a failure here.
- The device executor and the CPU executor grow the same tree by the existing agreement rules; a stage only one of them can run is returned to the host. [inferred]
- Nothing merges to master, nothing is tagged or published. Heavy work runs through `telperion-heavy`.

## Decision Context

Settled by the goal: one grower; a rule is general and sourced; the host conducts and an Opus worker builds, stopping at any question about how the system should work. How the yearly load is read (an upward pass each year, or the passes the cycle already runs) and where a phytomer's standing bend is kept are design questions that return to the host with the worker's findings before the rule is built. [paraphrase]

Open specs fn-59 (branches bend under the leaves they carry) and fn-200 (branches sag under their load and rest on the ground) own sag as it stands; this child changes when the load is counted and adopts neither. [inferred]

## Open Questions

None.

## Result

Retired 9 Nov 2026, superseded by fn-338, before any code. Its own R1 stopped it: the sources support the rule's mechanics (each year's added load bends the wood as it then stands, and the bend is kept; Almeras and Fournier 2009, Jirasek, Prusinkiewicz and Moulia 2000, both read) and refute the effect the host wrote into the premise. A radius only grows, so the yearly sum is never less than today's single bend, and on a self-similar limb it is about 10 times today's at 40 and 18 times at 300. The host had the direction reversed. The same sources name what the rule leaves out, reaction wood, as the reason real trees do not weep, and fn-338 builds on that. Kept and still read: R1-NOTE.md, DESIGN-FINDINGS.md and BEFORE.md (the old oak before any change: the crown's mean 11.91 m off the base at 200 and 12.47 m at 300 on the median seed) under .flow/evidence/fn-337-wood-sets-as-it-thickens/. No crate changed; no gate or review.

## Friction

- The shell hook refused a heredoc write (a minute). Disposition: local setup, reported.
- The host's premise stated an effect its own formula does not give, found by the worker's source check at the first planned stop (about 7 minutes of worker time, no code lost). Disposition: a rule's expected direction is derived on paper in the spec, with its arithmetic, before a child is minted for it.

