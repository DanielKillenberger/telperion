# The twig layer grows without the crown as a wall

## Conversation Evidence

> user (2026-10-02): "one question i have is why do we even have to check this? couldn't we embed the growth rule differently without capping an exact crown width?"
> user (2026-10-02, on twigs planned from their room instead of clipped at the crown's outline): "that sounds like it'd fit a tree crown better anyway?"
> user (2026-10-02): "so this is what i'm talking about when i asked about a fundamentally more elegant solution. How could we have landed there quicker?", then "yes add it and then spec the new spec" (the two rules now under "Before work is made faster" in `docs/principles.md`).
> user (2026-10-02, on the beech now waiting behind fn-182, fn-173 and this spec): "that's fine let's do this proper"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 25% [user], 55% [checked], 20% [inferred] -->

The twig layer treats the crown's outline as a wall. A planned axis is walked stride by stride, each stride asks the outline whether it is still inside, and a refused stride starts a 40-step bisection to cut the axis exactly at the outline. Terminal and leaf-bearing twigs get a second check of their own. That wall is most of the crown queries growth makes: 56% on the oak, 60% on the beech, 74% on the birch at seed 1 (evidence below), from the 7 to 11% of axes that reach it. [checked]

The generator may not need the wall at all. The scaffold is planned inside an inner crown that leaves an outer skin of depth `twigReach` for twigs alone (`inner_envelope`, `scaffold/frontier.rs:146`, `specimen.rs:111`), and twigs already take their length from their own rules: a length from the shoot's radius (`local/seed.rs:176`), inherited lateral length ratios (`local/advance.rs:147`), the twig's own length for terminals. A crown's edge then comes from the architecture: how far the last limbs reach and how long their twigs are by order and vigour. The literature supports architecture over a light-starved edge: peripheral shoots on open-grown trees extend more with more light, and branching order and age drive peripheral shoot length (Sterck and Bongers 2001, J. Ecol.; Buck-Sorlin and Bell 2000, Forestry). [code checked; the two sources were cited by Astra's review and not read here]

This spec makes the twig layer grow from its own rules and ask the crown's outline only where a measurement shows a question earns its place. The work is removed, not made cheaper (`docs/principles.md`, "Before work is made faster"). [inferred]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-10-02 on master a192a407.** [checked]
- `local::planner::Planner::run` (`pipeline/branching/local/planner.rs`) walks an axis in `count` strides; each stride end goes to `admitted` (`planner.rs:3-24`), `rejected` against the outline for a shortened limb system and `Curtain::admits` otherwise. A refused stride starts a 40-iteration bisection (`planner.rs:134-146`). The comment at `planner.rs:107` says "most stop early at the shell"; the counts say 7 to 11% do.
- Terminal and leaf-bearing lateral twigs bypass `run`: they take `twig.length`, cleared by the curtain (`advance.rs:192`), and are admitted separately (`advance.rs:193`). The check near `advance.rs:262` serves the growth path's expanding envelope.
- `Curtain::admits` (`local/pendant.rs:111`) short-circuits for a point inside the outline; only a point outside runs the band search for a dropping curtain. A curtain already has a floor from its descendants' tips, a pendulous length, sag and clearance (`seed.rs:39`, `pendant.rs:88`); its band below the crown (`pendant.rs:226`) is part of the birch's look.
- An axis bends while it grows (crookedness, the position-dependent bias field and sag, `planner.rs:120-133`), so it can leave and re-enter the outline along its run.
- `Envelope::distance_to_profile` is the smooth, axisymmetric outline's distance, not distance to a lobed surface (`envelope.rs:242`).

**The ladder R1 measures, in order; production code follows only R2's decision.** [inferred]
1. **No outline question.** The stride check, the bisection and the terminal admission are removed from the mature build; the existing length rules stand. The curtain keeps its floor, pendulous length, sag and clearance and loses its per-stride admission.
2. **An inherited allowance.** An axis's extent is bounded by an allowance it inherits from its limb system (the system's planned reach, `scaffold::reach` and `raggedReach`, carried down the orders). Distance to the parent's tip alone would starve terminal shoots and is not a candidate.
3. **Room from the outline, asked once per axis.** Two measures, which answer different questions: the radial depth of the axis's start (one query, blind to heading and slope) and a directional probe along the first heading, capped at the axis's authored length. Either sets an axis's length through a falloff law; that law and any row for it exist only if this rung is chosen.

The first rung that reads as the references do and keeps excursions within a stated tolerance is taken. [inferred]

**Decision (host, 2026-10-02, from `R1-LADDER.md` in this spec's evidence).** [checked]
- **Rung 3b, the directional probe, is taken.** Every twig-layer axis and terminal twig is planned once against the outline: the end of its authored length is tested first, and only if it lies outside does a probe march from the start in quarters of that length; its length is `min(authored, room)`. That is at most four outline tests per axis, no per-stride check, no bisection and no separate terminal admission. A point below `trunkHeight` counts as outside, as `rejected` counts it today. A shortened limb system is probed through its mapped `Bound`.
- **Why not the cheaper rungs.** Rung 1 (no question) cuts 83 to 98% of queries but 4 to 14% of oak and beech axes leave the outline, p95 0.28 to 0.49 of the widest radius, many below the crown base. Rung 2 (inherited allowance) empties crowns: beech nodes −16%, Telperion −65% and −92%. Rung 3a (radial depth) is blind to heading: up to 2.3% of axes out, max 0.40. Rung 3b keeps today's look on the oak at seed 1 (host's look at the whole still) at 40 to 51% fewer queries on the oak and beech, 11 to 18% less growth time, and 96% fewer queries on the birch.
- **The curtain.** A dropping curtain is probed the same way, against its own admission (the outline, or its band below the crown), so its strands stop where the band ends and the band stays. Exempting it leaves 2.6 to 3.1% of the birch's axes up to 0.75 out; capping it at the outline removes 88% of the band. Whether the band keeps its nodes is checked in R3.
- **No new row.** The falloff row was conditional on rung 3 needing one; `min(authored, room)` holds the look. A softer edge is a separate request if the owner asks for one after R5.
- **Tolerance.** On every catalogue preset at seeds 1 and 7, at most 0.2% of axes outside a curtain band leave the outline, none by more than 0.1 of the widest radius. A dropping curtain's band holds at least 90% of today's band nodes on the birch.
- **R4 targets,** from rung 3b's measurement: crown queries at least 35% lower on the oak, 45% on the beech, 80% on the birch; growth time at least 8% lower on the oak, 12% on the beech, 70% on the birch; the spruce and Telperion no slower. "Slower" is a warm median outside the base's warm range in interleaved rounds, as R1 measured.

**Room is not fn-182's depth.** fn-182's twig shell asks where on the scaffold twigs are borne (depth of the birth point below the outer surface); this spec asks how far a twig grows. They may share geometry, never one scalar. [Astra review]

**Profiling by purpose (R1, `docs/principles.md` rule 1).** `growth_profile` reports crown queries by exclusive caller and purpose (scaffold room, scaffold containment and sampling, twig strides, twig bisection, terminal admission, curtain band, shedding), with node and axis counts, so the next speed spec starts from who asks. Timings are taken with the counters compiled out. [inferred]

## Edge Cases & Constraints

- **Excursion is measured along the whole run,** terminal wood included, against the lobed outline and through a shortened system's mapped bound; an endpoint check cannot validate a bent axis. A dropping curtain's band below the crown is permitted and measured as its own region. [Astra review]
- **No emptied crowns.** Query savings never come from fewer nodes or leaves: node and leaf density per crown volume are reported beside every count. [Astra review]
- **A crisp outline stays reachable** where a species needs one, with "crisp" defined as a measured distance from the outline (R2). [inferred]
- **Any new row** is one finite continuous law over its whole range, neutral value included; no value restores the stride-and-bisect algorithm; an input off its rails is refused by name. [principles]
- **Identity is not required.** Every preset changes, under AGENTS.md "Generator evolution", with the owner's visual verdict. The hidden growth path stays buildable, follows the direct build's rule and is not tuned; fn-181 removes it. [AGENTS.md]
- No full-forest capture. [AGENTS.md]

## Acceptance Criteria

- **R1:** `growth_profile` reports crown queries by exclusive caller and purpose, with node and axis counts. With it, exploratory code (not merged) measures each rung of the ladder on the oak, beech, birch, spruce and Telperion at seeds 1 and 7: queries, growth time with counters out, node and leaf density, and the distribution of whole-run excursions beyond the outline (curtain band separate). Stills of the oak, beech and birch at seed 1, bare and leafy, today against each rung that keeps excursions bounded. [inferred]
- **R2:** The host's decision is recorded in this spec before production code: the rung taken; how planned axes, terminal twigs, curtains and shortened limb systems each get their extent; the excursion tolerance; and, only if rung 3 is taken, the measure of room and the falloff row with its bounds, neutral value and dial window. The query and time targets for R4 are set here from R1's numbers. [inferred]
- **R3:** The mature build's twig layer makes no per-stride outline check, no bisection and no separate terminal admission. Tests show: every axis's whole run stays within the tolerance on every catalogue preset at seeds 1 and 7, curtain bands as their own region; the tree moves continuously with any new row, station and terminal transitions included; repeated builds at one revision are identical. [inferred]
- **R4:** Paired `growth_profile` runs against one named base, under a stated noise policy: queries, growth time, nodes and leaves for every catalogue preset at seeds 1 and 7, meeting R2's targets with no preset slower beyond the noise. [inferred]
- **R5:** The owner's visual verdict on the oak, beech and birch at seeds 1 and 7, plus the spruce (a crisp conifer) and Telperion (a strong bias field), at most four stills per preset, bare and leafy. [user]
- **R6:** `cargo test --profile ci --workspace --no-fail-fast` and `npm test` are green; build time, peak memory and every shipped artifact's size are reported. [AGENTS.md]

## Boundaries

- The scaffold is unchanged: its limbs, `reach` and `raggedReach` stay as they are, and the ladder may read them. [inferred]
- Where twigs are borne is fn-182's. Leaf culling and shedding are unchanged. [inferred]
- The curtain's search is fn-174's only for what survives R2. [Astra review]

## Decision Context

- **Why this spec exists (owner, 2026-10-02).** fn-173 spent two sessions making each crown query cheaper; a count by caller showed most queries come from enforcing the crown as a wall on twig growth. `docs/principles.md`, "Before work is made faster", records the lesson. [checked]
- **Astra review (2026-10-02, `ASTRA-REVIEW.md`).** It changed the framing from "plan each twig from its room" to "twig extent follows branch architecture; outline queries must earn their place", added the zero-query and inherited-allowance rungs, the terminal admission the first draft missed, whole-run excursion, the curtain's existing controls, density checks, and the budget per axis a replacement may spend (oak 2.5, beech 3.6, birch 14 queries per planned axis at the first draft's targets). The first draft's botanical claim, that a crown's edge is where shoots run out of light, is withdrawn. [checked]
- **fn-173 waits on this spec,** re-scoped from the residual time R4 measures, including its preparation and the GPU cull's, not from counts alone. Scheduling is not a reason for it: the direct build does not schedule, and fn-173 keeps the tangent exact. [Astra review]
- **fn-182 follows this spec, not fn-173.** Its twig-shell depth needs the outline's geometry, not fn-173's table, and both specs change the twig layer, so fn-182 builds on this spec's twig layer. [host, 2026-10-02]
- **The 40-to-10 bisection fix is not taken.** It keeps the wall. [inferred]
- Evidence: `QUERY-COUNTS.md` (with corrections) and `query-count.diff` in this spec's evidence directory. [checked]

## Strategy Alignment

- Serves STRATEGY.md "Our approach": botanical rules below the crossover; measured cost and look. [strategy:Our approach]
