# fn-334 protocol, frozen 9 Nov 2026 before any derived tree grows

## The derivation (candidate D1)

Inputs: a species' tables and a fidelity f in metres. No value per preset. A species at f = 0 is itself.

**1. Which ages coarsen, and how far.** Merging the nodes of a unit drops laterals, so an age coarsens only as far as the laterals it bears are smaller than f.

- reach(j), the expected length in metres of the axis a bud of age j grows, from the tables alone: over each cycle of its lifespan, its survival so far times its unit's mean nodes times its internode, plus, where it moves on, its continuation times reach(j + 1). Abortion, relays and vigour are left out.
- borne(k), the reach of the laterals age k bears: the mean of reach(j) over the lateral ages j, weighted by the unit's expected laterals of each. An age that bears no lateral has borne = 0.
- s_k = clamp(f / borne(k), 1, cap_k), with cap_k = min(f / internode_k, the unit's mean nodes), and cap_k at least 1. Where borne(k) = 0, s_k = cap_k.

So the trunk and limbs, whose laterals reach past f, keep s = 1, and no `first` row exists. Every s moves continuously with f.

**2. The coarse unit.** As `Species::coarsened` does today: internode times s_k, each zone's node law divided by s_k. Lateral probabilities per node are unchanged (no `bud` row).

**3. What the missing laterals carried.** With N the full species, C the coarse one after step 2, and E[.] the expected phytomers by age that a bud of age k grows over the tree's age (`closed_form`'s walk, `Carried::default()`, no woken first unit), three multipliers per coarsened age are solved so that C's expectation equals N's for a bud of every coarsened age:

- leaf: sum over ages of E_C(pa) * leaf_area(pa) * L_pa = sum of E_N(pa) * leaf_area(pa)
- section: sum of E_C(pa) * (pipe(pa) * P_pa)^exponent(pa) = sum of E_N(pa) * pipe(pa)^exponent(pa)
- wood upkeep: sum of E_C(pa) * internode_C(pa) * upkeep(pa) * U_pa = sum of E_N(pa) * internode(pa) * upkeep(pa)

Each is solved from the last age down. An age with s = 1 has every multiplier 1 and is not solved for. A multiplier is at least 1; a solution under 1, or an age with no leaf, pipe or upkeep of its own, takes 1 and is reported. Then leaf_area times L, pipe times P, upkeep times U. `hazard` and `shedding` stay at 1: the expectation holds no light and no shedding, and that is the gap this child measures.

**Candidate D0** is D1 with U = 1 everywhere, read to see what the upkeep term does. No other candidate is read.

## The read

- Presets: all eight of the catalogue, each at its preset age.
- Fidelities: 0.25, 0.5 and 1.0 m, and 0 as the reference (the full cycle).
- Seeds: 1001 to 1012. Seeds 1 to 50 are not grown.
- Statistics: `fidelity_tests::read`'s six (height, spread, trunk, limbs, shed share, leaf area), each the median over the seeds as a ratio to the reference's median on the same seeds; the worst departure from 1.
- Cost: one thread, pinned, native `--profile ci`, five repeats less the first, a grow over 0.5 s not repeated; the median seconds, axes and apex-cycles.
- Beside each preset that has `/space/coarse` rows, the same read of those rows at the same fidelities.
- Every run goes through `telperion-heavy`.

## The verdict rule

Per preset at f = 1.0 under D1: "holds" at a worst departure of at most 5%, "near" at most 10%, "misses" past it, each naming the statistic that is worst. A derived tree that costs more seconds than the full cycle is named. A preset that fails to grow is listed with its refusal. The result states which statistics miss on most presets, since those name what a general correction or a small trained model would have to supply. A mistake found in the probe's code is fixed and the read repeated; a formula above is not changed after the first read, and a further candidate is a new child.
