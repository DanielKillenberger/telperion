# fn-202 result: dormant buds wake along old branches

## What was built

- **Settings (R1).** Each zone has a `dormant` table shaped like `lateral`, and a release law: `delay` (years) and `rate` (a yearly waking hazard after the delay). At 0 nothing changes.
- **Draws (R2).** A bud place's sleeping bud is keyed `place.child(DORMANT)`. Its waking time is keyed one step below that. The time is the delay plus an exponential wait at `rate`, by the inverse of its distribution (`dormant.rs`).
- **Waking.** The bud wakes in the cycle its time falls in. Its first growth unit lacks the share of that cycle it still slept, in length and in leaf-arrangement count. It stands half a slot past the bud that grew at once.
- **Only on living wood.** A bud wakes only if its bearer is carried on, by its apex or a continuation or relay of it, through the waking cycle. Its size carries the presence of every draw that kept the bearer alive from its node to then.
- **Closed form.** The closed form adds each woken bud as a lateral, weighted by:
  - the chance it wakes in each year;
  - the chance its bearer is carried on through that year (`Living`, the closed form of `carried`);
  - the stage it has aged into;
  - the law of its partial first unit.

## Host decision 1 (2026-10-04): the partial unit runs its share of the risks, and a bud ages while it sleeps

**Why scaling the first growth unit was not enough.** That was the Design as first written. On `dcf6add1` the release-law walks fail the jump check at every cycle boundary.

| Variant | Worst slope | Jump check |
|---|---|---|
| Naive integer-cycle waking (first unit whole) | 10.4 | fails |
| First unit scaled by its share (the Design as written) | 6.3 | fails, steps of 1 to 9 percent of the tree |
| Same, units keyed by years since the node, leaf-arrangement count by share | 6.3 | fails |
| Same, plus no survival or abortion test in woken stages and lifespans past the tree's age (experiment only) | 4.3 | passes |
| Share-scaled risk only (lifespans as they are) | 5.3 | fails |

The partial unit still made whole decisions:

- its survival and abortion tests, so a partial unit that dies stops everything after it;
- the whole unit it adds to the lifespan count, so the branch changes stage or stops a cycle early.

**Built:**

- The partial first unit survives with viability^share and aborts with 1 − (1 − abortion)^share.
- A sleeping bud ages through its stages as its axis does. A dormant bud is not timeless: it is carried in the bark as the axis ages.
  - One that slept past its last stage never wakes.
  - One that slept past a stage that leads on wakes into the later stage, through axes of no length that stand as its continuations, so its units are keyed and counted from the node's year.
- The closed form follows:
  - the aged stage and the units it carries;
  - the partial unit's survival and persistence, averaged over where in its year it woke (`First`);
  - the bearer carried on through the waking cycle.

Waking is now gated on the bearer living **through** the waking cycle, not just at its start. That fixes one more jump: a bud that woke late in a cycle in which its bearer died had been allowed to wake.

**After A:** worst slopes 1.2, 0.5, 2.5 and 0.9 on the four release-law walks. Three pass the jump check. One step still fails: limb rate at seed 2, about 1 percent of the tree.

## The remaining jump, and host decision 2 (2026-10-04): a shoot woken near its bearer's end is small

**The jump, found by bisection.** A twig wakes just before the end of cycle c, and its bearer then stops in cycle c + 1.

- Woken at c − ε, the twig wakes, grows a vanishing partial unit, then a full unit in c + 1.
- Woken at c + ε, it never wakes, because its bearer is gone.

**Built (decision 2).** A woken shoot keeps share + (1 − share) × p of its size (`Woken::kept`).

- share is the part of its waking cycle it grew.
- p is its bearer's presence carrying on through the next cycle:
  - the next unit's survival times its persistence;
  - times the presence of the continuation or relay that took over, where the bearer stopped after that unit (or died before it);
  - 0 where nothing carried it on.
- A bearer that died in the cycle after its next unit is told apart from one that stopped right after that unit by a `failed` flag. Without the flag a jump stayed: a probe found one at delay 0.45 on seed 1.
- Expected counts are unchanged, because presences are not counts. R4 is unchanged and green.

**Result.** `the_release_law_changes_the_tree_by_degree` is un-ignored and passes all four release-law walks. Worst slopes are 1.2, 0.5, 2.5 and 0.9, and every step passes the jump check. Red first: on `dcf6add1` (first growth unit scaled by its share alone) the same walks fail the jump check, with worst slope 6.3.

## Host decision 3 (2026-10-04): where a woken bud stands

- A woken bud takes its slot's azimuth in the parent's plane, as ordinary laterals do.
- It stands half an internode below its node, so it does not stack on the bud that grew at once.
- Girth and sag still take its load at the node.
- Re-checked on the spruce's limb close-up: see R5 below.

## fn-199's first finding

Does the same treatment answer the relay at probability 1 crossing survival? In kind, yes; the same formula, no.

- fn-199's fault is that an apex failing survival loses its whole failed unit, and the relay starts a cycle later.
- The continuous-time answer is the mirror of decision 1: the failing unit grows the share of its year the apex lived. That share comes from where the survival draw lies in the year's hazard: viability^t = u gives t = ln u / ln viability. The relay's first unit takes the rest.
- Relays are not changed here.

## Requirements

- **R1:** neutral, byte-identical.
  - Every earlier crate test passes, with the oracle signatures exact.
  - The beech stills at 80 years, seeds 1 and 7, are byte-identical to before (`raw/beech-before`, `raw/beech-after`, and `raw/beech-a` after decision 1).
  - `sleeping_buds_are_dormant_where_none_wakes` covers three cases: a release law with no sleeping bud, rate 0, and a delay past the tree's age.
- **R2:** `adding_a_sleeping_bud_moves_no_other_draw` passes.
- **R3:** passes:
  - the sleeping-probability walks (limbs and trunk);
  - the limb survival and abortion walks with buds asleep;
  - three of the four release-law walks.
  - One open (above).
- **R4:** `sleeping_buds_grow_the_expected_counts` passes: 4,000 seeds, every PA and cycle within 4.5 standard errors.
  - The species has limbs that abort, relay and die, and twigs that bear sleeping twigs.
  - Breaking the bearer's living term makes it fail (checked).
- **Cost:** the beech grew in 3.5 s before and 3.6 s after at seed 1. That is one sample each, and the neutral path adds only empty loops.

## R5: the spruce walking a dormant probability (`raw/r5b/strip.png`, viewed)

**Setup.**

- Seed 1 at 80 years, on `r5-spruce-tmp` (fn-194 rebased onto fn-202, plus a `--dormant` walk in the still runner).
- Branchlets sleep on both main-branch stages (the young sprig and the branch).
- Release law: delay 1, rate 0.3.
- One tree per process.
- Columns are dormant 0, 0.05, 0.1, 0.15 and 0.2. Rows are in leaf, bare, the limb close-up, and the limb in leaf.

**Release law picked from one limb close-up** (`raw/r5b/pick.png`: dormant 0, then rate 0.3 and rate 0.5 at dormant 0.1). The two rates read alike. Rate 0.3 grows fewer needles, so it renders one step further up the walk.

**Limits.** At delay 1 the woken buds wake as branchlets with spurs, so needles climb faster than in the first strip. Dormant 0.3 at rate 0.3, and 0.2 at rate 0.5, ran the GPU out of memory. The strip stops at 0.2.

**Measures.** Wood rose by degree: 46.5, 48.9, 51.3, 53.7 and 56.1 km. Height stayed at 21.9 m and width at 17.1 to 17.3 m.

**Reading** (`raw/r5b/limb-0-vs-0.2.png`, with the first strip's dormant-0 limb).

- **By degree: yes.** The outline and tiers hold, and each step adds a little more to the boughs.
- **The tufts are gone.** Woken branchlets lie in the bough's spray plane, as decision 3 meant.
- **The combs fill in.** At 0.2 the lower boughs' combs are visibly denser, with more hanging branchlet threads under the inner bough.
- **Not yet heavy draperies.** The effect is moderate at what the GPU can render.
- **Values for round 8:** delay 1 and rate 0.3 are a reasonable starting point.

**The first strip** (`raw/r5/strip.png`, delay 3, rate 0.15, half-slot angular place) is kept for comparison. It showed upward tufts and short spurs.
