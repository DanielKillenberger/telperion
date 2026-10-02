# A twig's length comes from its room in the crown

## Conversation Evidence

> user (2026-10-02): "one question i have is why do we even have to check this? couldn't we embed the growth rule differently without capping an exact crown width?"
> user (2026-10-02, on twigs planned from their room instead of clipped at the crown's outline): "that sounds like it'd fit a tree crown better anyway?"
> user (2026-10-02): "so this is what i'm talking about when i asked about a fundamentally more elegant solution. How could we have landed there quicker?", then "yes add it and then spec the new spec" (the two rules now under "Before work is made faster" in `docs/principles.md`).

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 50% [checked], 20% [inferred] -->

A real crown has no surface its twigs stop at. Its outline is where growth runs out of room and light: shoots near the outside are short and weak, and the edge is a band where twigs thin out. The generator's twig layer does the opposite. It plans each axis at full length, walks it step by step asking the crown's outline whether each step is still inside, and when a step crosses the outline it halves the step 40 times to cut the axis exactly there. [user, paraphrase]

That wall is most of the crown queries growth makes. Counted on master e12a9a28 at seed 1 (scratch instrumentation, 2026-10-02): [checked]

| Preset | Radius queries | Per-step checks | 40-step edge search | Local axes | Axes that reach the wall |
|---|---:|---:|---:|---:|---:|
| Oregon white oak | 183,563 | 67,546 | 34,719 | 11,738 | 868 (7%) |
| European beech | 293,726 | 104,081 | 71,999 | 16,116 | 1,800 (11%) |
| Silver birch | 1,267,571 | 320,952 | 616,959 | 12,655 | 1,190 (9%) |

The per-step checks and the edge search are 56% of the oak's queries, 60% of the beech's and 74% of the birch's. On the birch each check also runs the curtain's lower-surface search. [checked]

This spec plans each twig-layer axis from its room in the crown, so the outline comes from growth weakening toward the edge, not from a wall that cuts it. It removes the per-step check and the edge search, and gives every crown an edge that thins out the way the owner's plane and beech references show. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-10-02 on master e12a9a28.** [checked]
- `local::planner::Planner::run` (`pipeline/branching/local/planner.rs`) walks an axis in `count` strides. Each stride end is put to `admitted` (`planner.rs:3-24`): `rejected` against the crown's outline for a shortened limb system, `Curtain::admits` otherwise. A refused stride starts a 40-iteration bisection (`planner.rs:134-146`); the axis is then cut to the last admitted point less one twig length. The comment at `planner.rs:107` says "most stop early at the shell"; the count above says 7 to 11% do.
- The scaffold's first-order limbs already plan from their room: `scaffold::reach` (`pipeline/branching/scaffold.rs:252-270`) marches a straight probe of `height / 64` up to `reachProbeSteps` times (305 to 426 queries per build), and `raggedReach` (fn-61) keeps a drawn share of that room. A limb system kept short carries a `Bound` (`limbs.rs:22-30`) its shoots are mapped through.
- The curtain (`local/pendant.rs`) hangs pendant twigs from a lower surface and admits each stride against it; its search is fn-174's.

**Unknown until measured (R1).** [unknown]
- How room is best measured for a twig axis: the radial depth of its start inside the outline at its height (one query, direction-blind), the straight-line room along its first heading (the scaffold's probe, a handful of coarse queries), or depth from the crown's outer surface as fn-182's twig shell needs (one measure for both specs).
- How far an axis planned from its room overshoots the outline once crookedness, the bias field and sag bend it, and whether that needs a single trim at the tip or nothing.
- What a pendant (curtain) axis's room is, and whether the curtain's per-stride admission is still needed.

**Shape (host, 2026-10-02; settled by R2 after R1).** An axis's planned length is its authored length scaled by a falloff of its room: full length where the room exceeds it, shorter toward the edge. One new row, the falloff's sharpness, runs from a soft fade to nearly a hard stop, so a species with a crisp outline (a dense conifer, an open-grown young tree) keeps one. No per-stride outline check and no edge search remain on the twig layer. The wall is replaced for every preset, not kept as the default: a default that keeps the wall keeps its cost. [inferred]

**Profiling by purpose (R1, `docs/principles.md` rule 1).** `growth_profile` reports crown queries per caller and purpose (scaffold room, scaffold containment, twig planning, curtain, scheduling, shedding), so the next speed spec starts from who asks. [inferred]

## Edge Cases & Constraints

- **A crisp outline stays reachable.** The falloff's hard end draws an outline within a stated distance of today's for the oak, measured on stills, not pinned bytes. [inferred]
- **No twig ends outside the crown by more than a stated tolerance.** R1 measures the overshoot; R2 states the tolerance and whether a single tip trim is needed. [inferred]
- **The growth path stays buildable.** It is hidden and fn-181 removes it; it follows the direct build's rule and is not tuned. [AGENTS.md]
- **Identity is not required.** Every preset's tree changes, under AGENTS.md "Generator evolution", with the owner's visual verdict. [user]
- No full-forest capture. [AGENTS.md]

## Acceptance Criteria

- **R1:** `growth_profile` reports crown queries per caller and purpose. With it, the report for the oak, beech and birch at seeds 1 and 7 states: today's counts and growth time; for each candidate measure of room, its queries per axis, the share of axes it shortens, and the overshoot distribution of axes planned from it; and stills of the oak and birch at seed 1, today against the leading candidate, with the twig tips nearest the outline marked. [inferred]
- **R2:** The host's decision is recorded in this spec before code: the measure of room, the falloff and its row (bounds, neutral value, dial window), the overshoot tolerance and any trim, and the curtain's rule. [inferred]
- **R3:** The twig layer makes no per-stride outline check and no bisection. A test shows an axis's length falls continuously with its room over the falloff, and the tree moves continuously as the row moves. Every axis ends inside the outline within the stated tolerance on every catalogue preset at seeds 1 and 7. The row is declared once in the catalogue, blended, on the dials, and refused by name off its rails. [inferred]
- **R4:** Paired `growth_profile` medians against one named base: crown queries and growth time for every catalogue preset at seeds 1 and 7, with the oak's and beech's queries at least 40% lower and the birch's at least 60% lower, and no preset's growth slower. [inferred]
- **R5:** The owner's visual verdict on the oak, beech and birch at seeds 1 and 7, at most four stills per preset, with each preset's falloff set by the host and stated. [user]
- **R6:** The workspace gate and `npm test` are green; build time, peak memory and every shipped artifact's size are reported. [inferred]

## Boundaries

- The scaffold's limbs and `raggedReach` keep their room probe; this spec may reuse it but does not change how limbs are planned. [inferred]
- The twig shell (where twigs are borne) is fn-182's. This spec decides how far a twig grows, not where it is borne; R1 checks whether both can read one measure of depth. [inferred]
- Leaf culling against the outline and shedding are unchanged. [inferred]

## Decision Context

- **Why this spec exists (owner, 2026-10-02).** fn-173 spent two sessions making each crown query cheaper. A count by caller showed most queries come from enforcing the crown as a wall on twig growth. `docs/principles.md`, "Before work is made faster", records the lesson. [checked]
- **fn-173 waits on this spec's measurement.** Its outline table speeds the queries that remain (scaffold, scheduling, the GPU cull). Its rework is re-scoped once R4's counts are known. [inferred]
- **fn-174 is re-evaluated after R2.** If the curtain's per-stride admission goes, its search may no longer be hot. [inferred]
- **The 40-to-10 bisection fix is not taken.** It would keep the wall and is removed by R3. [inferred]
- Evidence: the instrumentation diff and its output are in this spec's evidence directory. [checked]

## Strategy Alignment

- Serves STRATEGY.md "Our approach": botanical rules below the crossover, and measured cost and look. [strategy:Our approach]
