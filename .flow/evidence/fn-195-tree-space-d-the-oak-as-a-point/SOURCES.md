# Sources of the oak's values (round 4)

The rules 1 to 10, with their quotes and tags, are in MODEL-OAK.md (`/home/daniel/Projects/telperion/.worktrees/fn-190/.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/MODEL-OAK.md`). The source texts are in the fn-188 research worktree's `.firecrawl/lit/`:

- **BC07:** `bc2007.md`, Barthélémy and Caraglio 2007.
- **PR00:** `petri.md`, Prusinkiewicz and Remphrey 2000.
- **T83:** `miami.md`, Tomlinson 1983.
- **K19:** `biorxiv795286.md`, Kędra 2019.
- **CIR:** `cirad.md`, the CIRAD lecture.
- **HEIN:** `hein.md`, Kint et al. 2010 (abstract) and the citing passages on the same page.

Tags: **sourced** means the passage gives the rule or the number. **Rule sourced** means the passage gives the mechanism and the number is estimated. **Estimated** means no source gives it; the value was tuned on the stills.

| Value in `oak.rs` | Rule and source | Status |
|---|---|---|
| One growth unit a year; rhythmic units with bare nodes at the base and laterals towards the top | MODEL-OAK 1, 5. BC07: "growth and branching are rhythmic". BC07, Rauh 1939: "primary acrotony" | Rule sourced. Polycyclism (rule 1a) is not drawn |
| Every axis monopodial; the trunk erect (elevation π/2, tropism 0.6) | MODEL-OAK 2, 3. PR00: "The trunk is monopodial" | Sourced |
| Laterals repeat the trunk: limbs and boughs are orthotropic at first (elevation 0.85 and 0.8 rad) | MODEL-OAK 3. PR00: "Branches are orthotropic and morphogenetically equivalent to the trunk" | Rule sourced; angles estimated (MODEL-OAK: no source gives an oak angle) |
| A 2/5 spiral (divergence 2.513 rad) | MODEL-OAK 3: spiral phyllotaxis from the HOT axis table | Spiral sourced; the 2/5 fraction is estimated |
| No terminal flowering | MODEL-OAK 4. BC07, PR00, K19: lateral flowering | Sourced (nothing to draw) |
| Trunk: 11 years at about 0.36 m a year (4 to 5 bare, 2 to 3 lateral and 1 top node of 0.045 m), so about 3.6 m at 10 years | HEIN: Q. robur after 8 years in the field "varied between 1.3 and 4.2 m"; growth units of "116 and 242 mm" | Height checked against the range; the node counts and internode are estimated |
| Height at 40 and 80 years: 13.0 to 13.3 m and 19.2 to 21.7 m | E1, Jüttner 1955 oak yield table (Q. robur/petraea, stand-grown mean height), as transcribed in `.flow/evidence/fn58/validation/oregon-white-oak/fixtures/ertragstafeln.md`: 12.0 m (class II) to 15.6 m (I) at 40, 20.3 m (II) to 23.8 m (I) at 80 | Sourced as a stand-grown proxy; an open-grown tree's height is not given |
| Trunk laterals are temporary long shoots (`SPRIG`, about 0.35 m a year for 10 years, ascending at 0.7 rad as Rauh's laterals repeat the trunk, bearing twigs and branch systems), shed within about 30 years, so the sapling is a bushy young tree and the bole clears | MODEL-OAK 8. HEIN: "early branch dying and slow shedding for oak" | Rule sourced; lengths and lifespans estimated. Slow shedding (dead stubs kept) is not drawn |
| Fork over 2 years: each year's top cluster grows its 3 buds as limbs at p 0.55, so 3 or 4 limbs leave at two heights | MODEL-OAK 6. K19: "Not a single tree fully conformed to the original Rauh's model"; forks "formed since the early times"; main branches "of unequal insertion height" | Rule sourced. Fork age, years and counts estimated |
| Above the fork the stem carries on for 20 years as a leader (shoots of about 0.4 m a year), bearing limbs at p 0.08 and boughs at p 0.1 from each top bud, then becomes one more limb | MODEL-OAK 6 (a weak central stem may persist). K19: the trunk "contributes to the vertical tree extent", the lowest main branches to the horizontal | Rule sourced; years and rates estimated, for the height above and the dome |
| Limbs fork again: `LIMB` p 0.007 at each of the 3 top buds of a unit | MODEL-OAK 6. K19: "repeated forking of the axes coming from a fork below" | Rule sourced; rate estimated |
| Limb and bough apex death at 0.25 a unit, with a relay from the top cluster (epitony 0.3): the kinked, tortuous course | MODEL-OAK 6. CIR: abscissions "can be very frequent in an oak tree" | Rule sourced; rate and epitony estimated |
| Limb and bough sag (3e-4 and 5e-5, fn-200 and fn-203) | MODEL-OAK design question 3: the low limbs spread and hang in S1 and S2; no source says whether by lean or load | Estimated; the mechanism is the host's fn-200 answer for the spruce |
| Limbs leave at 0.45 rad from their bearer, ascend (elevation 0.95 rad, tropism 0.5) and settle outward by sag; wander 1.0, roll 0.8, straightening 0.2; pipe 0.009 at exponent 2.8, so the few limbs stay heavy. Boughs: elevation 0.75, tropism 0.6, wander 0.8, viability 0.999, so their lengths vary less (host round 4) | MODEL-OAK 10 (K19: forked arms turn upright away from the junction) for the straightening | Estimated (host, round 2: limbs more ascending, settling with sag) |
| Sleeping branch buds on every limb zone at p 0.1, waking after 6 years at a yearly hazard of 0.04 (fn-202): epicormic shoots | T83: reiteration "originates from a reserve bud—a dormant bud that has played no previous part in crown development"; BC07: delayed reiteration "from the development of a previously dormant bud"; HEIN: Q. robur parents "with crooked trunks and epicormic branches" | Rule sourced; share, delay and hazard estimated |
| Boughs: 60 years, viability 0.995, shed 4 years after death; borne at p 0.08 at each top bud of a limb; bearing branch systems at p 0.3 | MODEL-OAK 7, 8 | Estimated |
| Branch systems: 12 years then 4 as long shoots, viability 0.98, wander 0.8, internode 0.02 m; short shoots and twigs along the unit, twigs clustered at the top | MODEL-OAK 5 (acrotony); S4 shows laterals bunched at the shoot tip | Rule sourced; reference-visible only for the cluster; counts estimated |
| Twigs: one more order, 3 years of short growth (internode 0.015 m, wander 1.2) bearing short shoots | Host round 3: a fine twig web of many orders, as S1 and S4 show | Reference-visible; estimated |
| Short shoots: 3 to 5 crowded nodes, unbranched | none for the oak | Estimated; as the beech's GreenLab PA 4 |
| Pipes, exponents, internodes, ripening | none | Estimated from the stills; dbh 0.6 to 1.0 m at 80 years |
| Bark and leaf colours (preset) | none | Bark matched by render to S3; leaves darkened towards S2's crown (garryana) |
| Young wood dark grey-brown (0.08 / 0.066 / 0.05) under 30 mm, blending to the bark by 60 mm (preset) | none | Estimated, host round 2: fine twigs read dark, as in S1 |
| Stills rows: `shootRadius` 0.06, twig internode 8 mm | none | Estimated; see FRICTION.md for why `shootRadius` rose |
