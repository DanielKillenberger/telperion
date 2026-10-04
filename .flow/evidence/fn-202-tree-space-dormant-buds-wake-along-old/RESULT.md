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

## The remaining jump: a design question for the host

Probed by bisection. A twig woken just before the end of cycle c, whose bearer then dies in cycle c + 1:

- Woken at c − ε, it wakes, grows a vanishing partial unit, then one full unit in c + 1.
- Woken at c + ε, it never wakes, because its bearer is dead by then.

The rule "wakes only while its bearer lives" is a hard gate in time. Nothing makes a shoot that woke just before its bearer's end small.

Candidate (not built): a woken shoot's size times share + (1 − share) × p. Here share is the part of its waking cycle it grew, and p is the presence of its bearer carrying on through the next cycle.

- A bearer that dies next cycle gives a shoot that vanishes as share goes to 0.
- A bearer that barely survives its next draw has p ≈ 0, which matches the side where it dies.
- The closed form is unaffected, because presences are not counts.

The alternative is to accept the remaining jump. `the_release_law_changes_the_tree_by_degree` stays ignored, with this reason, until the host decides.

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

## R5: the spruce walking a dormant probability (`raw/r5/strip.png`, viewed)

- **Setup:**
  - Seed 1 at 80 years, on `r5-spruce-tmp` (fn-194 rebased onto fn-202, plus a `--dormant` walk in the still runner).
  - Branchlets sleep on both main-branch stages (the young sprig and the branch).
  - Delay 3 and rate 0.15 are strip-only values.
  - Columns are dormant 0, 0.05, 0.1, 0.15, 0.2 and 0.3. Rows are in leaf, bare, the limb close-up, and the limb in leaf.
  - Each tree was rendered in its own process.
- **Rendering limits:** 0.4 and 0.45 ran the GPU out of memory (over 23 million needles). 0.8 passed the 20-million-phytomer budget.
- **Measures:** wood rose by degree, from 46.5 to 48.2, 49.8, 51.4, 53.1 and 56.3 km. Height stayed at 21.9 m and width at 17.1 to 17.2 m.

**Reading.**

- **By degree: yes.** The whole tree's outline and tiers do not move. Each step adds a little more fine wood along the boughs.
- **Curtains: no.**
  - At 0.3 the limb close-ups show boughs somewhat denser along their length, with short tufts standing up out of the bough's plane.
  - They do not show hanging draperies filling the curtain.
  - Two causes are visible or follow from the values:
    - The half-slot place puts a woken branchlet above or below the flat bough, and the ones above read as tufts.
    - Most buds wake after the branchlet stage's 10 years, so they wake as short spurs, not long comb branchlets.
- **Starting point for round 8:** this is not a confirmed starting point. A spruce round would want waking earlier (a shorter delay, a higher rate) so they wake as branchlets, and hang (fn-194's values).
