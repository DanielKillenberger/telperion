# A year is a state: the grower reads no horizon

Child of fn-351 (criterion P4). Host-written, 12 Nov 2026. Builds on fn-354's branch and lands with it.

## Problem

The grower reads the requested age as a horizon: every presence window is normalised by the tree expected at the requested age, a sleeping bud that would wake after it is never recorded, and a growth cannot go on past it (`.flow/evidence/fn-351-020-the-release-candidate/P4-SURVEY.md`, sites G1 to G9). So the tree at year 40 on the way to 120 is not the tree asked for at 40, and the owner asked on 13 Oct 2026: "is there a way to not have to fix maximum age?" Since fn-354 a lateral's kind follows its light, which makes the dependence visible in the structure: `a_node_keeps_its_identity_across_ages` is red on the beech between requested ages 14 and 15.

## The design (host)

A draw's window is a function of the species, the draw's physiological age and the cycle it is made in. Nothing a draw reads depends on the age asked for.

- **The look.** Where a window today takes "the wood a bud of PA k is expected to grow over the cycles left to the horizon", it takes the wood over the cycles left of that PA's own lifespan (its `lifespan` row; for a draw made part-way through an axis's life, what is left of it). Where it divides by "the whole tree's expected wood at the horizon", it divides by the whole tree's expected wood at the draw's cycle plus that look.
- **The table** of expected lengths is filled as far as the growth needs (the year grown plus the longest lifespan) and extended when a growth goes on; an entry does not depend on how far the table runs. If today's does (to the bit), that is fixed first and reported.
- **Sleeping buds.** A sleeper's expected wood is summed over the years its release law lets it wake in, not to the horizon. Every sleeper drawn is recorded with the cycle it wakes in; `asleep` grows as needed.
- **The last year** is laid out like any other (fn-354 built this); the expected-kept ceiling is judged at the year grown; `grow_to` past the first request goes on.
- **RATE, SPAN and FLOOR stay** as they are.
- Estimate, named: "the PA's own lifespan" as the look is the host's choice among the survey's option A's forms; fn-192 rejected a one-cycle look because narrow windows pop.

### Corrected by the host, 12 Nov 2026, on the builder's first stop

The look by the whole lifespan needs the expected-length table 500 rows past the year on the oak (1.4 s on a 2.6 to 3.9 s tree). The table itself does not depend on how far it runs (tested to the bit on five species).

- **The look is bounded:** the cycles left of the PA's lifespan, or `LOOK` = 40 cycles, whichever is less. `LOOK` is a contract constant beside RATE, SPAN and FLOOR. Estimate, named: 40 is the host's, the age most presets are asked for and their walk bounds were measured at, so a young draw's window stays near what it has been. A fractional lifespan reads between two rows.
- **The table** runs to the year grown plus `LOOK`, for the grower and for every other reader of the windows (`expected_kept`, `blend::size`); its cost is measured at that extent.
- **A sleeper is recorded when it can wake:** when its drawn wake time falls before the bud ages past its last stage, which is a bound of the species' own tables (16 years on the oak, 50 on the spruce, 12 on the birch as the builder read them). A sleeper that can never wake is not recorded and holds nothing open.

## Acceptance Criteria

- **R1.** No site in `crates/telperion-space` reads the requested age to decide a draw, a window, a presence, a sleeper or a closure. A structural test holds it: the tree at year y of a growth to y + n is the tree of a growth to y, by the digest the paused-prefix test uses, for the oak, beech and spruce at two pairs of ages each.
- **R2.** A growth goes on past its first request, and the result is the tree of a growth asked for that far.
- **R3.** `a_node_keeps_its_identity_across_ages` passes unchanged on every species it covers.
- **R4.** The walks hold their bounds (re-measured, the steepest step of each reported before and after).
- **R5.** Measured on seeds 1001, 1007, 1012 beside fn-354's head: the oak at 40 and 120, the beech, birch and spruce at 40, Telperion at 82: height, spread, trunk, leaf, new shoots, drawn nodes, the summed length and the length-weighted mean presence; seconds and peak memory on one thread; the count of sleepers held at the end. Sheets (whole, bare) beside fn-354's.
- **R6.** Docs (`docs/pipeline.md`, `docs/tree-space.md`) say a year is a state; every repinned digest is listed before and after.

## Stops (return to the host with the numbers)

- Any PA whose lifespan is unbounded or whose look makes the table's cost material.
- Summed length at the asked age falling by more than a tenth on any tree, or a walk failing its bound.
- Sleepers held or peak memory rising by more than a fifth on the oak at 120.
- Any question of how closure, retirement or waking should work once every sleeper is recorded.

## Boundaries

- No change to which buds wake or when (Phase 3 owns the crown's renewal).
- No per-species value; no new constant beyond what the design names.
- The fractional-age blend between two consecutive years of one growth stays.

## Rigor

Product code, Opus to this design; narrow tests while building; the full gate and one codex review at the landing with fn-354.
