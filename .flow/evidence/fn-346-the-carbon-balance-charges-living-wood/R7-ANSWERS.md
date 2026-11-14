# fn-346: the host's read of the fourth probe and what is probed next, 11 Nov 2026

Each is the host's design decision; none is the owner's ruling.

## The host's verdict (viewed: square-oak-1001-whole.jpg, square-oak-1007-whole.jpg)

| tree | SR (square charge, `L_max` the potential height, with R) | PR (straight charge, same `L_max`) |
|---|---|---|
| oak 40, 80 | the same as today's | 40 the same; 80 worse, thinner |
| oak 120, both seeds | **better**: the many-limbed clothed crown; on 1007 a broad full oak on a short bole | worse: open, leaf at the ends |
| oak 200, seed 1001 | **worse**: a straggly thin crown, a third of today's leaf, limbs to the ground on one side | much worse |
| oak 200, seed 1007 | not worse, not better: an upright 23 m tree with a real trunk and limbs clear of the ground, thinly clothed | much worse |
| oak 300, seed 1001 | **worse**: the stem itself arcs over to the ground from 8 m | much worse |
| oak 300, seed 1007 | **better than today's**: a small upright old tree on a straight trunk, thin but standing, where today's is a twisted pole with a limb on the ground | dead |

- **The square charge is the first that gives the tree a steady state.** The longest path holds at 34 to 38 m from about 140, the height stops at 32 to 33 m, living tips and held phytomers are level from about 220 with births over deaths at 0.99 to 1.00, and the grow to 300 is cheaper than today's (131 s against 146 s, 2.2 GiB against 2.7).
- **The straight charge at the same `L_max` is refuted** and is dropped.
- **The source** supports the direction and not the exponent: the abstract of McDowell et al. 2002 states that leaf area per sapwood area falls as height rises, in a meta-analysis of 13 whole-tree studies, as a mechanism that "partially compensates". Full compensation, the square, is the host's estimate and stays listed as one. Whether an oak is among the species is unchecked.
- Not a candidate yet. Nothing enters the crates.

## What is wrong now: two things, each seen in the measures

1. **The wood shrinks.** A fork limb's base radius under SR goes from 34.8 cm at 160 to 13.1 cm at 240 as its outer parts are shed, and the drawn tree at 300 on seed 1001 is a thin stem bent to the ground, while the census holds a 30 m crown. The girth is read from the pipes a part carries now, so a part that loses what it bore gets thinner, and sag reads that radius to the fourth power. Wood that has been laid does not go away. This is why the settled tree stands at half the census height or less from 200.
2. **The steady crown is thin.** It holds 43 to 59% of its peak tips and a quarter to a third of today's leaf at 200. The limbs' tips stop from 200, so their fine wood is renewed only by what still extends. Rule B (a sleeping bud wakes where a tip would pay) was built for this and could not be read while limbs were lost whole or ran to 40 m.

## The two rules

**G. Wood that has been laid stays.** A part's radius never falls from one year to the next: it is the larger of what it was and what its pipes give now. Sag, the drawn surface and everything else that reads a radius read that one. Nothing else about girth changes (the pipe per node, `retained`, the leaf girth).

- First say, from the code, how and when the girth is computed (each year, or once at the settle from the present state) and what a running largest radius needs: what is stored, per what, and what it costs. If the girth is only computed at the settle, the probe may keep one stored radius per held phytomer and update it each year; report the cost. If it cannot be built without a decision of the host's, stop and return.
- Before building, write down from the existing logs the trunk's radius at its base and at 5 m, and each fork limb's base radius, every 20 years from 100 to 300 under SR and under today's, so the shrinking is on the page as a measurement.

**B, as built:** sleeping buds on the limb, bough and branch rows, never ageing, waking at the existing rate × max(0, 1 − cost / light), with the cost now the square charge at the bud's place.

## The run

Variants on SR: **SRG** and **SRBG**. Probe kept as `probes/stays.patch`; the crates end as they began.

1. **Census to 300,** oak seeds 1001 and 1007, both variants, stopping itself as before, with the same columns as last time and each decade the settled tree's height beside the rough layout's where that is cheap (otherwise at 120, 200 and 300).
2. **Stills,** leafless and leafed, today's beside SR, SRG and SRBG: the oak at 40, 80, 120, 200 and 300 on seed 1001 and at 120, 200 and 300 on seed 1007 (SR's stills reused).
3. **The other presets** under SRG and SRBG at each one's preset age and the beech at 80: one sheet beside today's, with the measures, trunk diameter among them.
4. **Each fork limb's shape** under SRG at 160, 200, 240 and 300, as before.
5. **Cost pair in one session:** today's and SRBG, one thread, the oak at 40, 80, 120, 200 and 300 on seed 1001, with peak memory.
6. **Your plain read per tree and variant,** with what is wrong where it is worse. Look in particular at whether the trunk and limbs stand at 200 and 300, whether the crown is clothed through its depth, whether a thicket grows on the bole or under the limbs, whether the trunk or a limb base swells out of scale, and whether the oak at 40, 80 and 120 has moved from SR's.

About 2.5 hours. Result as `STAYS-RESULT.md` with the sheets. Return then. The host views every sheet before anything else happens.
