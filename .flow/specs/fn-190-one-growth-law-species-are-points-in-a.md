# One growth law: species are points in a continuous architecture space

## Conversation Evidence

> owner (2026-10-02): "We should have a branching rule that makes them grow organically according to the params set."
> owner (2026-10-03): "how do we hill climb towards the reference image ... Problem here is that we seemingly have a fundamental flaw with the growth algo that's holding back most species."
> owner (2026-10-03): "all with the aim to fix fundamental growth algo that is better at supporting wide array of trees"
> owner (2026-10-03): "can we have a new model that is able to smoothly transition between those models to get the continuous tree space we want?"
> owner (2026-10-03): "yes and let's make sure the new spec has a well defined achievable goal so i can then run /flow --auto"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 25% [user], 55% [checked], 20% [inferred] -->

Replace the generator's growth stage with one growth law that runs from trunk to twig, in which every catalogue species is a point in a continuous space of architectural settings, so a parameter change moves the tree by degree and a new species is new coordinates. The law must make the beech, an oak and the spruce read closer to their reference photographs than today, measured, and stay fast. [user, inferred]

**Why (checked, fn-188).** Today's growth is two systems joined at a crossover (a rule-built scaffold and a separate twig layer) held together by thresholds and patches: the twig layer's thickness gate (`local/seed.rs:133`), the tip shoot that bypassed it and carried whole twig layers (fn-189 `TRACE.md`: 100% of telperion's twigs at seed 7), girth patches, the crown wall (removed in fn-183). Thresholds make the parameter space jumpy, which is why value tuning kept nothing. fn-188's probe (`.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/`, R3-PROBE.md to R3-PROBE9.md, R3-SPEED.md, R3-ADAPTER.md) showed competition growth (Pałubicki et al. 2009, space colonization with vigour allocation) removes the patches and keeps every walked parameter continuous, but with every axis following one rule the beech reads as a ball or a comb, not its photograph. The literature (`LITERATURE.md` in the same folder) names the missing piece: axis differentiation by physiological age (Barthélémy & Caraglio 2007), and that Pałubicki 2009 itself omits branch reorientation, "essential" for Troll's model. [checked]

**The architecture space (from LITERATURE.md and the host, 2026-10-03).** What separates the Hallé–Oldeman models becomes continuous settings on one law: apical persistence (monopodial to sympodial), lean by physiological age with a straightening rate (orthotropic to plagiotropic, Troll's model), the lateral position profile along a shoot (acrotony to basitony), rhythm strength (continuous to tiered), shoot form by vigour (short non-branching shoots to long branching shoots), reiteration readiness, and branching probability (down to unbranched, Corner's model). Catalogue models: beech and birch Troll, oak and ash Rauh, spruce Massart, date palm Corner. [checked: literature; inferred: the mapping]

## Architecture & Data Models
<!-- scope: technical -->

- **Base:** the fn-188 probe's competition law as of round 9 (history: `01ba06a6` on branch `fn-188-one-branching-law-from-trunk-to-twig`, report R3-PROBE9.md): space markers in the crown, buds with perception cones, Borchert-Honda vigour allocation, pipe-model radii over the whole tree with leaf demand, the pipeline's own foliage. Known gaps it carries: every bud follows one rule; space-only resource gives every bud equal vigour at λ 0.5; shedding cannot be tuned in space mode (R3-PROBE9). [checked]
- **To add (R1):** a continuous physiological age per bud; smooth curves of it set shoot length and internodes, lateral development and its position profile, lean and its change over time (straightening), and bud fate (short shoot, long shoot); a graded light or resource signal so vigour differs and drives age; shedding by remembered resource or light, not by space alone. Every setting is one continuous parameter; integer outcomes (branch counts) are taken in expectation with keyed randomness so small changes give small changes. [inferred]
- **The scorecard:** `.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/ASTRA-BARE-TARGETS.md` and R3-PROBE9.md's implementation (`raw/probe9/score.py` on that branch's disk; re-implement in typed code): clear bole and division height, major axes, W(h) profile, secondaries per major and length ÷ remaining parent, fine wood in the outer shell vs interior and upper vs lower, junction thickness ratios, fine-wood local straightness, fine-wood length, laterals per metre, generations, terminal run lengths. [checked]
- **References:** `.flow/references/<species>/` holds 49 openly licensed photographs (Wikimedia Commons) at five scales (S1 whole bare, S2 whole in leaf, S3 mid-crown limbs, S4 bare twig spray, S5 leafy spray) for the beech, oak (Q. robur stand-in, Q. garryana where available), silver birch, Norway spruce, European ash and London plane, each with provenance, licence and what it shows in the folder's README; gaps are listed there. The beech's earlier B-BARE and B-WHOLE (fasy896, fasy951) carry no redistribution licence: they stay out of the repository and are not targets; the ASTRA-BARE-TARGETS bands read from fasy896 are re-checked against the committed beech S1 photographs in R1 before they are fixed. [checked]
- **What the production law deletes:** the twig layer as a separate system, its thickness gate and tip shoot, the girth patches (`girthHold`, `girthFall`, `lateralShare` as patches), shedding as structural cleanup, and remaining crown-wall checks in growth. Also the foliage cull (`pipeline/foliage.rs:255` and its GPU copy in `place.wgsl`), which places leaves on all twig wood and then deletes those deeper than `shell_depth` inside the authored outline, and limb clumping as after-the-fact thinning: with light or resource competition, shaded interior shoots get little vigour and carry few or no leaves, so leaves exist only where growth put them (owner, 2026-10-03: "shouldn't growth just work and not need culling later?"). Code that becomes dead is removed; rows that lose meaning are retired by name with a migration note. [inferred]
- **Contract with fn-125 (expansion):** whether short shoots are grown wood carrying leaves or foliage stations on existing wood is decided in R2 and written into this spec and fn-125's, so fn-125 optimizes the station types growth produces. The contract also states that expansion places leaves only where growth put them: no outline-based cull and no clumping thinning remain in expansion. [inferred]
- **Unchanged:** the pipeline's other stages and their contracts; the bias field (fn-175's curl becomes a term in the law's direction); the scaffold envelope rows as authored crown volume where they still apply. [inferred]

## Edge Cases & Constraints

- **No switches.** Every new parameter changes the tree by degree; no value selects a different way of building (`docs/principles.md`). A parameter's neutral value is stated; byte identity with today is not required (AGENTS.md "Generator evolution"). [principles]
- **No new thresholds or walls** where a continuous law can do the job; any remaining limit names its owner and reason (`docs/principles.md` step 1). [principles]
- **Speed is a product requirement** (owner, 2026-10-02: "in the long term it needs to be super fast"); comparisons are at equal or greater detail, never by drawing less. [user]
- **Today's renders are never a target** for a real or legendary species, only the baseline to beat (owner, 2026-10-03: "today's look is bad for telperion and laurelin so giving them new start from references is good"). [user]
- **Legendary trees** follow STRATEGY.md ("the shared parametric field supplies a legendary tree's authored character on top of its real base species"): each has a real base species judged against that species' photographs with the scorecard, plus an authored character from Tolkien's text, every quote verified against the source before use and turned into measurable properties (scale, lean, writhe and spiral through the existing supernatural bias field, colour). **Laurelin** takes the European beech as its base (to be confirmed by the verified text, which is recalled as likening its young leaves to a new-opened beech's). **Telperion**'s base species is chosen in R2 from the verified text, comparing candidates (the beech, the silver birch and any other the text supports) with renders and reasons recorded; the owner can overrule at R7 (owner, 2026-10-03: "Don't know about the birch for telperion but rest seems good"). [user]
- **The date palm** is a real species: it is measured against its own references (fn-82's onboarding), stays unbranched (Corner), and is not judged against today's render. [user]
- **Ordinary** is the generator's default family, not a species: it is the law's neutral point (every setting at its default) and is checked only for plausibility, with no reference target. [user]
- No full-forest capture; at most four images viewed per capture by any agent. [AGENTS.md]

## Acceptance Criteria

- **R1 (proof in the probe, before production code):** In the probe, the law with physiological age grows the European beech, an oak (Oregon white oak preset; Quercus robur photographs where Q. garryana ones are lacking) and the Norway spruce. Pass bands are written into this spec BEFORE tuning and reviewed by Astra: for the beech the ASTRA-BARE-TARGETS bands (clear bole 0.18–0.28 of height; division band 0.30–0.40; 4–6 major axes; 5–10 substantial secondaries per major with length ÷ remaining parent 0.3–0.8; outer-shell ÷ interior fine wood > 1.3 and upper ÷ lower > 1.1; first-order junction ratio 0.4–0.6; laterals per metre of fine wood ≥ 1.3 with generations p50 ≤ 3), and for the oak and spruce bands derived the same way from their reference photographs and architectural models (Rauh, Massart). Each species meets at least 80% of its bands at seeds 1 and 7, with bare and in-leaf stills beside the references; and walks between the species' points (beech to oak, oak to spruce, at least 9 steps each) show no jump: no measure changes by more than three times its median step in one step. Probe growth time per species is at most 1.5 times today's skeleton time (warm median, same machine) at equal or greater fine-wood length. Bounded effort: if R1 is not met after the documented attempts (each a recorded probe round), the run stops with `NEEDS_HUMAN` and the report. [inferred]
- **R2:** The production design is recorded in this spec before production code: where the law lives in `pipeline` (stage 2), the parameter set with neutral values and windows, the deletion list, the fn-125 leaf-station contract, and how each catalogue preset is expressed as coordinates, including Telperion's chosen base species and the Two Trees' authored-character properties with their verified quotes. Astra reviews it against `docs/principles.md` and the evidence; findings are folded in or answered. [inferred]
- **R3:** The law replaces the growth stage for every preset; the deletions are made; no direct-build code path still runs the twig layer, its gate, the tip shoot or the girth patches. A test walks every new parameter and shows the tree changes by degree; off-rail values are refused by name; rows are declared once in the catalogue, blended and on the dials. [inferred]
- **R4:** Every catalogue preset is expressed in the new space: the beech, oak and spruce at least as close to their references as today on the scorecard at seeds 1 and 7 (and meeting R1's bands); birch, ash and the date palm measured against their references and no worse than today; Telperion and Laurelin judged against their base species' references with the scorecard and against their authored-character properties (scale, colour, the bias field's terms) with the verified quotes cited; ordinary checked as the neutral point. Bare and whole stills of every preset beside today's and the references are in the evidence. [inferred]
- **R5:** Speed and size: growth time per preset at most 1.5 times today's skeleton time (warm median, seeds 1 and 7), full build at equal or greater leaf count no slower than today, peak memory reported, every shipped artifact within its CI size budget. [user, inferred]
- **R6:** `cargo test --profile ci --workspace --no-fail-fast` and `npm test` are green; the Codex implementation review passes. [AGENTS.md]
- **R7:** The PR is opened for the owner's visual verdict on the R4 stills and is not merged without it. [user]

## R1 pass bands (fixed 2026-10-03, before tuning)

Read from the committed S1/S2 photographs and the architectural models; every band's source, pixel reading and confidence is in `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/R1-BANDS.md`. The beech's ASTRA-BARE-TARGETS bands were re-checked against `S1-entzia-open-winter` and `S1-rostock-copper-winter`, and four were widened where the two photographs disagree. Astra's review (`R1-BANDS-ASTRA.md`, REJECT) is folded in below where it corrects a reading or a measure; its proposal to take most bands out of the acceptance count would change this criterion's shape and is put to the host, so until the host decides the bands stand as diagnostics **and** the checks Astra asked for are added as mandatory gates (a stricter bar, not a weaker one).

**Scorecard rules, the same for every tree measured (today's and the probe's):** the measures are fn-188's `score.rs` (R3-PROBE9). Wood is **structural** where its radius is at least 0.05 of the root's; laterals per metre and generations count from that boundary. **Major axes** are the laterals born at 0.4 of the root radius or more whose axis reaches 0.2 H (`root04`); the union with `local04` is reported, not scored. A species passes when it meets at least 80% of its scored bands at seed 1 and at seed 7 separately.

| measure | beech (Troll) | oak (Rauh) | spruce (Massart) |
|---|---|---|---|
| clear bole (lowest substantial trunk lateral ÷ H) | 0.18–0.30 | 0.14–0.32 | not scored |
| lowest lateral with an axis ≥ 0.05 H, ÷ H | – | – | ≤ 0.10 |
| division (`leaders2` ÷ H, projected envelope) | 0.24–0.56 | 0.15–0.35, and division − clear bole ≤ 0.10 | none below 0.85 |
| major axes (`root04`) | 4–7 | 4–7 | 0–1 |
| substantial secondaries per major (median) | 4–10 | 3–10 | not scored |
| secondary length ÷ parent remaining (p50) | 0.3–0.8 | 0.4–1.0 | not scored |
| shell ÷ interior fine wood | ≥ 1.3 | ≥ 1.1 | ≥ 1.2 |
| upper ÷ lower fine wood | ≥ 1.1 | 0.9–1.5 | 0.7–1.6 |
| first-order junction ratio (median) | 0.35–0.60 | 0.40–0.70 | 0.08–0.25 |
| laterals per metre of fine wood | ≥ 1.3 | ≥ 1.3 | ≥ 1.3 |
| generations p50 | ≤ 3 | ≤ 3 | ≤ 3 |
| first-order branch elevation (median; the share above 45° reported, unscored) | – | – | −5° to +30° |

Beech 10 bands, oak 10, spruce 9; a compound row counts once and needs every clause; a missing measurement fails. Generations are taken once per terminal. The 0.05 boundary is a measurement convention, never a generator cutoff; final rows also report it at 0.04 and 0.06. **Mandatory beside the count (Astra, findings 1 and 2):** at both seeds, (a) the bare and whole stills at matched scale beside the references show differentiated limbs reaching the outer crown, substantial secondary subdivisions and no repeated paired uprights (beech, oak), and a persistent leader with tiers of predominantly horizontal branch systems (spruce); (b) a probe trace shows the developmental model: the beech's first-order axes lean at birth and their bases turn up over the run (Troll), the oak's stay orthotropic from birth (Rauh). **Attempt bound:** six recorded probe rounds; R1 unmet after the sixth stops the run with `NEEDS_HUMAN`.

## Host decisions after R1 round 1 (host, 2026-10-03)

Round 1 (`R1-ROUND1.md`) stopped on four design questions. They are answered here; rounds 2 to 6 run on these answers, and the host steers them one round at a time.

1. **Light is the resource; space markers retire as the resource.** Every bud reads its light from the shadow-propagation grid (`shadow.rs`, Pałubicki 2009 §4.1) with one lookup and casts its shadow when it grows. Markers and perception cones no longer gate growth, so a bud with nothing in front of it still grows by its own vigour and its direction terms. The authored envelope stays only as an optional direction term (attraction towards the authored crown, weight neutral 0), never as a wall or a resource. Why: round 1 shows marker perception is what a seedling cannot grow through (the pole, the missing trunk), and per-bud marker queries are the cost that makes fine wood unaffordable.
2. **Short shoots are grown wood, from the same bud law.** A bud's fate (short or long shoot) follows its vigour and φ continuously; a short shoot is wood with its internodes and leaves that never branches, a long shoot can. Both read light the same way, so there is no second system and no threshold. For fn-125 the contract is: growth hands real short-shoot wood, and foliage places leaves on the wood growth made, with no cull. The fine-wood comparison counts all grown wood, short shoots included.
3. **The 0.05-root boundary is a measuring convention,** reported at 0.04 and 0.06 as well (Astra, finding 4). Tip radii come from the pipe model over the whole tree with leaf demand per tip and no floor.
4. **Ontogeny: the tree grows from a seedling in fixed space, and the clear bole emerges from shedding.** Lower branches lose light as the crown above them grows and are shed on remembered light, as on an open-grown tree. No marker scaling by age, and no authored crown base imposed on growth. The species' bole comes from its shade tolerance (the shedding rate and its memory), measured against the bole band. The `crownBase` row's fate is decided in R2.
5. **Which bands vote (Astra's REJECT, accepted).** Per species at both seeds, the **required checks** are: Astra's visual gate (a) and developmental trace (b) above; the clear bole and the projected division band (beech, oak); the persistent leader, the branch elevation median and no division below 0.85 (spruce); fine-wood length at least today's; and growth time within 1.5 × today's skeleton. The **supplementary bands**, of which at least two of three must pass, are the junction ratio, secondaries per major (reported per major too) and secondary length ÷ parent remaining (beech, oak). For the spruce, one of two must pass: the lowest lateral ≥ 0.05 H and the junction ratio. **Diagnostics only:** major-axis count, `leaders2` as a path count, the density ratios (renamed for what they measure: terminal-run density), laterals per metre and generations.

## Host decisions after R1 round 2 (host, 2026-10-03)

Round 2 (`R1-ROUND2.md`) showed one cause behind its four questions: the law has income (light) and no cost, so size runs away, shedding judges a branch by light alone, and every short shoot costs a long shoot its vigour. The answer is one carbon balance, continuous, with no new threshold.

1. **Net resource is light gathered minus upkeep.** Every living shoot costs upkeep in proportion to its wood (neutral: its volume); a subtree's net resource is its leaves' light minus its wood's upkeep, summed basipetally as light is now. Growth spends only the net. Size is then self-limiting: a tree grows until its upkeep meets its light, so the growth rate falls smoothly as it nears its size, and seed 1 and seed 7 cannot differ by ten times. The run's age sets how far along that curve a tree is; the authored height is the value age and the upkeep ratio are calibrated to, measured and never enforced. A bud that stays dormant dies with a probability that rises smoothly with its dormant years (the bud bank decays).
2. **Shedding by a branch's own balance.** A branch is shed with a probability that rises smoothly as its remembered net balance (its subtree's light minus its upkeep) stays negative. The main line carries the whole crown, so its balance is the crown's and it is never shed by a shaded moment; no special case spares it. Shade tolerance is how negative a balance a species tolerates, and for how long.
3. **Allocation follows demand.** A shoot draws resource in proportion to what it builds: a short shoot of a few internodes draws little, so making more of them no longer starves the long shoots, and they pay for themselves in light. Fine wood is then what the balance affords.
4. **The probe definitions are confirmed:** Troll means first-order axes are born at 45° elevation or less and their bases rise by 10° or more over the run; Rauh means they are born at 45° or more and fall by no more than 5°; the spruce's lowest lateral is an axis of at least 0.05 H that starts no higher than 0.10 H. A tree of fewer than 1,000 nodes is an error in the probe driver, never a scored tree.

## Host decisions after R1 round 3 (host, 2026-10-03)

Round 3 (`R1-ROUND3.md`) bounded size but grew no bole, kept a seedling cliff, and showed the upkeep dial jumping. Two causes: the shadow is local (6 layers), so nothing above darkens the seedling-era limbs; and light does not saturate with leaf area, so the balance is bistable (grow or collapse) instead of settling.

1. **Light is intercepted from above over the full height (Beer–Lambert).** A cell's light is exp(−k · leaf area above it within a sky cone), accumulated down each column once per cycle, replacing the per-bud shadow pyramids. k is the species' crown density (a shade-casting beech high, an open birch low). The lower crown darkens as the crown above fills, so seedling-era limbs fall into balance deficit and are shed, and the bole forms; light per leaf saturates as the crown thickens, so the balance has one stable size and the upkeep dial moves it smoothly. Expected to be cheaper too: one pass over the grid per cycle instead of a pyramid per bud.
2. **A reserve, not an age gate.** The tree carries a stored resource, starting at a seed reserve and topped up from surplus; a negative net draws on it before any balance counts as negative. It is a stock with a continuous size (neutral: the seed reserve), so the seedling no longer stands on a cliff, and no rule names an age.
3. **Short shoots persist and add a little each year.** A short shoot lives many years and extends by a few short internodes a year, their length following vigour; its upkeep is in proportion to its tiny volume, so fine wood accumulates as on a real beech. The fine-wood bar is unchanged: it is the owner's equal-detail rule for the speed comparison (owner, 2026-10-02).
4. **The bole is measured smoothly:** crown base is the 5th percentile height of fine-wood length ÷ H, reported beside the old lowest-substantial-lateral reading. The bole band applies to the new measure.

## Host decisions after R1 round 4 (host, 2026-10-03)

Round 4 (`R1-ROUND4.md`) formed the bole on the beech and oak and made size and both walks move by degree; every crown is columnar, every tree is 1.8–3.5× over its time bar at a tenth to a third of today's fine wood, and height swings with apical control.

1. **Crowns spread toward the open sky.** Light is gathered from a few sky directions down to a low elevation (neutral: five directions, the zenith and four at 30°), not a single 45° cone, so an open-grown crown's sides are bright and its interior dark. Every bud's direction gains a term along the horizontal part of the light gradient (phototropism, weight per species). The beech's Troll lean and straightening are switched on at its point. The envelope's pull stays at 0; the spread must come from light.
2. **Height by hydraulics, not by apical control.** A shoot's growth efficiency falls smoothly with its path length from the root, reaching zero near the species' authored height (the hydraulic limitation, Ryan & Yoder 1997). Height then settles near the authored row whatever λ is, and λ shapes the crown rather than its height.
3. **Speed: the same law, aggregated below a shoot.** A long shoot reads light once, and its short shoots are one record per shoot (count, internodes, leaf area, volume) whose balance and growth the law computes from that shoot's light and allocation, with geometry emitted only at the end. The whole-tree passes become incremental: only shoots whose subtree changed are recomputed. Same law, same fates, fewer evaluations. If round 6 meets everything except the time bar, the run stops with `NEEDS_HUMAN` and the measured gap, for the owner to decide; the bar is not lowered by an agent.
4. **Check the spruce's division reading** against the flared base before it votes; fix the measure if it is the flare.

## Rescope after R1 round 5 (host, by the owner's delegation, 2026-10-03)

The owner asked the host to apply the design principles and choose among the options in `R1-HOST-ASSESSMENT.md`. The choice is option 1, rescoped as below; Astra reviewed it (`R1-ASTRA-RESCOPE.md`, AGREE WITH CHANGES) and its changes are folded in.

**Question the requirement.** "A tree is grown from a seedling by per-bud competition" was the host's design assumption (fn-188's option A), not an owner requirement. The owner's requirements are: one rule from trunk to twig that grows a tree organically from its parameters, species as continuous coordinates, references matched, no thresholds or culling, fast at equal detail. Five rounds never produced crown spread from competition, and fn-188's markers only filled the authored envelope.

**Delete before optimising.** Per-bud competition is deleted, not optimised: per-bud light lookups (59.7 ms of the beech's 235 in round 4), the whole-tree allocation pass every cycle (70.7 ms) and marker perception go. Option 3 is rejected because it would defer an owner bar for a mechanism with no measured path to it; option 2 because one round on the same mechanism has low odds.

**The rescoped R1, in two steps. Both keep R1's species, seeds, voting rules (host decisions round 1 item 5), the 1.5× time bar at equal fine wood, and grown short shoots (the fn-125 contract). A tree that collapses, or any input the rule cannot draw, is an error, never a scored tree.**

- **R1a, architecture and cost without feedback.** One organogenesis rule driven by a continuous physiological age φ (Barthélémy & Caraglio's axis categories as one continuous variable; GreenLab-style organogenesis) generates every axis from trunk to twig, short shoots included, in one pass, replacing the scaffold and twig-layer seam. φ's curves set lean and straightening, the lateral position profile, apical persistence, rhythm, short or long fate and reiteration. Envelope attraction stays neutral (0). R1a passes when, at both seeds, the beech and oak show differentiated, reiterated limbs reaching the reference crown on Astra's visual gate, the spruce its leader and tiers, the supplementary bands pass, fine wood is at least today's, and time is within the bar. The report names, with measured milliseconds, every retained pass (traversals, state, any feedback) and confirms the deleted ones are gone. The bole is reported, not voted, in R1a. Whether continuous φ keeps GreenLab's factorisation is a hypothesis R1a measures (LITERATURE.md §4.2 marks the continuous reading as the researcher's, not a source's).
- **R1b, light and the carbon balance on top.** One Beer–Lambert light pass per cycle, as in round 4, modulates vigour, and shedding on the carbon balance (host decisions rounds 2 and 3) removes shaded wood. R1b passes when R1a's architecture and time still pass, and the bole band, seed-to-seed size within 1.5×, and the 9-step walks (beech to oak, oak to spruce, and the upkeep and shade-tolerance walks) pass the unchanged no-jump criterion. Round 4's results are hypotheses here, not inherited (its own walk table still flags jumps).
- **Bound.** Three recorded probe rounds for R1a and three for R1b, each round steered by the host. Either step unmet after its third round stops the run with `NEEDS_HUMAN`. This replaces the spent six-round bound; it is an explicit host decision under the owner's delegation, reported to the owner.

The host decisions after rounds 1 to 4 stand as history. Their light and carbon-balance mechanics return in R1b; markers, perception cones and per-bud allocation do not return.

## Host decisions after R1a round 1 (host, 2026-10-03)

R1a round 1 (`R1A-ROUND1.md`) met time (0.27–0.51 of the bar) and fine wood (1.07–1.34× today's) on all three species, the first round to do so; the beech and oak are bushes with no limbs (no lateral reaches 0.4 of the root radius; today's beech has 5).

1. **Limbs come from the spread of birth ages.** A limb is a lateral born young (low φ) that lives long and carries a large subtree; a bush is every lateral born alike. A lateral's birth φ is its parent's φ plus a jump that depends continuously on its position on the parent's annual shoot (acrotony) and on the parent's vigour: the few distal laterals of a vigorous shoot jump little and become limbs, the rest jump far and stay short. Each axis has a lifespan that falls with φ, so high-φ laterals on the trunk die young (natural pruning by age, not by light), and the trunk's apical persistence decays with age so limbs take over the crown (Troll's sympodial build). All continuous; no count of limbs is authored.
2. **Size stays R1b's.** No supply ÷ demand cap in R1a; keep partial reiteration so sizes agree across seeds. The carbon balance in R1b owns size.
3. **One straightening pass at the end is accepted** when it is the integral of a per-year reorientation rate over each axis's age, so it equals the yearly result in a run without feedback. It is not a free shape correction. R1b re-checks it once light feeds φ.
4. **Division, the developmental traces and the oak's "division − clear bole ≤ 0.10" clause are reported in R1a and vote in R1b.** R1a's vote stays: the visual gate, supplementary bands, fine wood and time.
5. **Wood below the ground is an error** in the probe driver, so the law is fixed (gravitropism near the base, branch angles), never clamped.

## Host decisions after R1a round 2 (host, 2026-10-03)

Round 2 (`R1A-ROUND2.md`) grew limbs (beech 6 majors) within time and fine wood, but every limb comes from the seedling's first years, low on the trunk, and each major carries 18–68 secondaries against a band of 4–10. Both have one cause: a shoot's vigour does not yet follow the age of the tree or of its own axis, so the seedling's laterals are born as vigorous as any later ones, and a major's late shoots branch as freely as its early ones.

1. **The tree has an establishment curve.** Every shoot's vigour is scaled by a smooth curve of the tree's age that rises from the seedling to the tree's prime (neutral: flat). Laterals born in the weak seedling years then jump far in φ, live short and are pruned by age, so the bole forms and limbs are born at the height the trunk had in its vigorous years. Division and limb height come from this, with no authored height.
2. **An axis's vigour falls with its own φ, which rises with its age.** A major's later annual shoots are weaker, so their laterals jump further and stay short; secondaries per major fall to what the axis's vigour affords. The same vigour(φ) curve applies to every axis; there is no per-order rule.
3. **φ approaches 1 smoothly** (each year φ moves a fraction of the way to 1), never clamped, so lifespans and fates spread instead of piling up at 1.
4. **The major-axis count stays a diagnostic** (host decisions round 1 item 5); no setting is tuned to `root04`, and the visual gate decides.
5. **R2's production design spells out the corrected two-ranked lateral rule** (laterals beside the shoot, not above and below it).

This is R1a's third and last round.

## Second rescope: reproduce each model before building the space (owner, 2026-10-03)

After R1a round 3 the owner judged the stills "clearly not even close... a huge regression". The host's diagnosis, accepted by the owner: the probe never built the beech's architectural model. LITERATURE.md ("Troll's model in *Fagus*", [M98]) describes the trunk as a sympodial stack of modules, each with an erect base and a plagiotropic tip, the next module arising from a bud in the curvature zone (the relay) and taking apical dominance, a fork near 20 m that stops height growth, and a crown of reiterates, the outermost shortest. The probe's beech had a persistent orthotropic leader (`organ.rs:183–218`: persistence by φ, relay only on abortion from the distal laterals, lean per axis by φ), which is Massart's trunk, so it drew a fir. Today's beech is not Troll either (owner: it "looks like ass", which is why this spec exists). Continuity between models is a property of working points; no point worked.

**R1 is replaced by staged reproduction, each stage judged by the owner on stills before the next starts** (memory: judge each generator round by its stills). The engine is R1a's single-pass organogenesis (`growth_law/organ.rs`), which met time and fine wood; its axis rule is replaced.

1. **Beech, Troll, per [M98] piece by piece:** seedling an orthotropic monopodium whose tip tilts, first laterals plagiotropic; modules of several growth units with an erect base and a plagiotropic tip; the relay from a bud in the curvature zone on the upper side (epitony), taking dominance; branch axes plagiotropic monopodia, distichous, in flat systems; the fork near maturity that stops height growth; the crown as a succession of reiterates, the outermost shortest. Stills at several ages beside the beech photographs.
2. **Oak, Rauh:** rhythmic growth, orthotropic branches like the trunk, acrotonic clusters at the top of each annual shoot, forks from the terminal bud cluster.
3. **Spruce, Massart:** orthotropic monopodial trunk, rhythmic tiers of plagiotropic, bilaterally organised branches.
4. **The space:** each model's features as continuous settings (the module angle profile from base to tip, relay readiness and position, rhythm strength, flowering position), walks beech to spruce to oak judged on stills.
5. **Light and the carbon balance** (round 4's bole by shading and self-limiting size) on top, judged the same way.

Time and fine wood are measured at every stage and reported; the bar applies from stage 4 on. No round bound; the owner's verdict on each stage's stills gates the next, and a stage the owner rejects twice stops for a decision.

## Boundaries

- Not the species runner or its score (owner, 2026-10-03: "let's not start writing specs on how to fix the runner with a proper score now"). [user]
- Not the cost of leaf placement in expansion: that is fn-125's (and fn-126's); this spec only agrees the leaf-station contract with it. [inferred]
- Not the species values beyond expressing the catalogue presets: fn-62 tunes the beech through the runner afterwards. [inferred]
- Not the hidden growth path (fn-181 removes it); a later rewrite of growth over time may build on this law. [inferred]

## Decision Context

- **Supersedes:** fn-182 (branching hierarchy; its shell dropped, its question answered here), fn-189 (deleting the tip shoot; done as part of R3 together with the gate it compensated for), and the growth-side speed specs fn-172 (shedding by outline distance) and fn-173 (the crown-shape table), whose targets the new law replaces. fn-188 closes into this spec with its evidence. [user, 2026-10-03]
- **Carries over:** fn-175 (the bias field's curl) as a term in the law's direction. [inferred]
- **fn-125 proceeds in parallel** on expansion, with the leaf-station contract from R2. [user, 2026-10-03]
- **fn-62 (the beech)** depends on this spec. [inferred]

## Strategy Alignment

- Changes STRATEGY.md's "Our approach" line "space colonization for the crown, botanical rules below the crossover" to one growth law from trunk to twig with species as coordinates in an architecture space; R7's merge carries that wording change with the owner's verdict. [strategy:Our approach]
