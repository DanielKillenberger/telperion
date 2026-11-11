# Phase 2 design on paper: wood that lasts, 11 Nov 2026

The host's design, from `PHASE2-SURVEY.md`. Nothing here is built. It is the base of a probe, and the probe is read before any product code.

## What the survey changes

- **The host's proposed measurement of 11 Nov is withdrawn before it ran.** Keeping every pipe and scaling the pipe down overshoots real oaks at 80 and 120 on paper (up to 1.9 times), because a pipe is laid per held phytomer and not per leaf: 2.37 M phytomers for 305,000 leaves at 120.
- **Girth and sag were fitted against each other.** Limbs are drawn about twice as thick as leaf-sized wood would be (fork limbs 0.39 to 0.71 m against 0.24 to 0.27 m at 80), and the sag rows are 10 to 60 times softer than green oak. Deflection goes with 1/r⁴, so a limb twice too thick needs wood about 16 times too soft to bend the same. Changing one without the other is what bent fn-341's limbs.
- **No girth or sag row in any preset has a source.** Two measured traits exist for the first and two for the second.

## The rule

A section is heartwood plus sapwood.

- **Sapwood is what the leaf above needs:** `S = A / R`, with A the leaf area the section carries above it this year and R the species' leaf area per sapwood area. Measured: oak 0.23 to 0.37 m² per cm² (Phillips et al. 2003), beech 0.331 (Bartelink 1997), spruce 0.36 (McDowell et al. 2002).
- **Each year one Nth of last year's sapwood becomes heartwood,** N being the species' count of sapwood rings. Measured: European oak 16 to 26 (Hollstein, through Haneca et al. 2009), beech 25 to 73 conducting rings (Gebauer et al. 2008).
- **The tree lays the sapwood it is short of, and wood laid is never lost:**

      kept   = S_before × (1 − 1/N)
      laid   = max(0, S_now − kept)
      T_now  = T_before + laid          (T is the section's whole area; radius = sqrt(T / π))
      S_held = kept + laid              (carried to next year as S_before)

- **A new phytomer starts at its primary wood,** the radius of the shoot as it extends (today's "established" radius), until its leaf-sized section passes it.

What follows without further rules:

- A trunk never thins. When the crown is shed, the section stops growing until the leaf above needs more sapwood than is left.
- A shed branch leaves in its bearer exactly the wood the bearer laid to serve it. There is no `retained` share and no moment of entry, so the mechanism behind the stepped rings and the pipe entering at a relay's tip is gone.
- Taper: every section below a branch carried that branch's leaf, so the trunk is thicker below each limb, as Shinozaki's piled cones.
- It replaces six rows per growth stage (`pipe`, `exponent`, `ripening`, `secondary`, `retained`, `leaf_girth`) with two per species (R, N), both measured.

**Expected on paper** (the survey's arithmetic, seed 1001, the leaf trace of the cycle before the budget fix): trunk 30, 72, 103, 144, 175 cm at 40, 80, 120, 200, 300, against White's open-grown 28 to 40, 56 to 79, 83 to 109, 121 to 152, 156 to 192. On the fix branch, whose crown falls to a fifth after 120, the trunk would end near 125 cm and not thin; that crown is Phase 3's.

**Estimates in the rule, named:**

- The form "one Nth retires a year, the tree lays what is missing" has no source as a formula. It is the simplest reading of two measured facts (sapwood area follows leaf area; an oak holds N rings of sapwood). LIGNUM uses the same shape with a fixed 0.07 a year.
- R is taken as one value per species. For the oak that is 0.30, the middle of Phillips' two groves; R's change with height (rising in oak and spruce, falling in beech) is left out.
- N for *Q. garryana* is European oak's. Spruce N and birch R and N were not found and are estimates until Phase 4 looks again.
- Primary wood per shoot is today's value.

## State and cost

- The rule is not linear (the `max`), so it needs two numbers per phytomer, T and S_held. The phytomer's `radius` field is an f64 that growth leaves at 0; two f32 fit in its place, so memory per phytomer does not grow.
- Each year, one pass from the tips down: leaf above each phytomer (its own, its axis's beyond it, its children's at their nodes), then the three lines above. It is the shape of the pass the settle's girth already makes, without a settle. fn-341 never built it and paid 2.2 times for a yearly settle of a copy instead. Cost unknown until measured; the bar is the goal's (no tree slower than at the last landing unless it is better and the cost is stated).
- Girth becomes state the grower carries from year to year, which is what "a year is a state" (P4) needs of it.
- Leaf is read as it stands in the growing tree that year. How far that is from the settled tree's leaf is unknown.

## Sag reads that wood, with wood's own stiffness

- The sag formula is an elastic beam's with `sag = 4ρg / E`. With measured green wood: oak 5.0e-6, beech 4.1e-6, birch 3.7e-6, spruce 6.1e-6 per metre (E and density from Lavers through Niklas & Spatz 2010 and Hamilton 1975). One value per species replaces the sag row of every growth stage.
- **Estimate, named:** a real limb bends more than the final load over the final r⁴ (wood carries load only from when it is laid: about 5 times at the poplars' exponent in Alméras & Fournier 2009) and holds most of that back with reaction wood (their poplars: +0.040 against −0.044 m⁻²). The two are taken to cancel, so the plain elastic value stands. If the probe shows limbs too stiff or too soft, the next step is Alméras & Fournier's increment and reaction terms from fn-338's note, not a fitted coefficient.
- Fine wood is younger and softer (juvenile wood 0.45 to 0.75 of mature E in lumber; no branch value found). Left out; noted.
- The species whose look rests on soft rows today (the spruce's drooping branches at 9e-4, the birch's hanging twigs) are the test of this. If measured stiffness cannot draw a weeping birch, that habit needs its own sourced trait, and the probe says so.

## What the drawing needs

- The root flare multiplies the radius by up to 2.1 at the ground; with a real trunk it draws a real flare. Unchanged.
- A limb's shoulder: the bearer is thicker below a limb than above it, by the wood laid for that limb, over one internode. While the limb lives, the limb explains it. After it is shed, the shoulder stays and later rings cover it evenly. Whether that reads as a real shoulder or as a step is for the probe's close view to show before any easing rule is designed.
- The drawn tree's cuts that read the root's radius (structural from 0.05 of it, the codominant fork from 0.4 of the parent) move with the trunk. Watched in the probe.

## The beech

Its leaf is a fifth of a real beech's, so wood sized from its leaf would be a wire. Girth from leaf cannot land for a species before that species' leaf is right. So Phase 2 takes the leaf half of Phase 4 with it: each real species' leaf area against its published leaf area index, fixed at its cause before the girth rule lands for that species. The shoot-count half (the spruce's 27 times) stays in Phase 4.

## The probe (before any product code)

On the fix branch after Phase 1 lands, as a patch kept in evidence:

1. The rule above with the yearly pass; R and N as stated.
2. Sag at today's rows and at `4ρg / E`, both.
3. Drawn and measured on the scoreboard's panel: the oak at 40, 80, 120, 200, 300 on three seeds for sheets and twelve for measures; the beech, birch and spruce at their preset ages; whole, mid and close views, the close view at a limb's shoulder and at the base.
4. Measured: trunk at 1.3 m against White; the base diameter of the largest limbs; height and spread; the limb's chord angle against today's; leaf area a year (recorded this time, for every seed); one-thread seconds and peak memory with and without the yearly pass.

**What would refute it:** a trunk outside White's three open classes at 80 to 300 on most seeds; young trees (40, 80) worse than today's by the host's eye under both sag settings; the yearly pass costing more than a tenth of the grow with no way to cut it.

## Order within Phase 2

1. Land the budget fix's CPU part (fn-349's commits without the device port), on the one grower.
2. The probe above. Read it.
3. The leaf of each species against its published leaf area index, cause found and fixed.
4. The girth rule and the sag value as product code, the six rows deleted.
5. The oak on seed 7 (L6) is judged on the scoreboard after 4.
