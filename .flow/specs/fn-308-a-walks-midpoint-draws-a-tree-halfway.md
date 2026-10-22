# A walk's midpoint draws a tree halfway between its ends

## Goal & Context

Owner (2026-10-22), on the fn-299 still of oak to palm at 40, seed 1, t = 0.5: "there's no branches still". The frame is a bare, palm-like stem with a thin crown and one leafless side shoot. Almost none of the oak shows halfway through the walk. The film walks through it.

What is measured (fn-299 `R1.md`, 2026-10-21): age 2's blended `lifespan` (a `Log { floor: 1 }` blend) is 43 years at t = 0.5, 117–150 years from t = 0.9 on, and 136.6 cycles at t = 0.962.

What is inferred, not traced: the tree is 40 years old, so its main axis never leaves age 2 at t = 0.5. The oak's branching ages are therefore never entered, and the drawn tree has almost no laterals. If that holds, the walk's rows change by degree but the drawn tree does not. It keeps the oak's branching only while age 2's blended lifespan is under the tree's age, and then drops it. That would be a cliff in t, which breaks the owner's "change by degree" rule (fn-277, host ruling 5a) for what is drawn.

## Owner verdicts

- 2026-10-22, on the C4 stills (rule B over every row, the lifespan clamped at the tree's age; oak to palm at 40, seed 1, t = 0.25, 0.5 and 0.75), as relayed by the host: "much better nice". The owner approved C4's direction. Evidence: `.flow/evidence/fn-308-a-walks-midpoint-draws-a-tree-halfway/R1.md`.

## Resolved by R1 (measured, `R1.md`)

- **Age 2's lifespan is not the whole cause.** The main cause is the placeholder rows at the ages the palm never reaches. `Species::canonical` writes lateral and dormant chances of 0 there, and the palm's readiness of 0. Mixed in, they damp every branching order by (1 − t)². The drawn oak is 93 % gone in wood by t = 0.25, long before the lifespan matters.
- **Age 2's lifespan is the second cause.** At t = 0.5 it keeps the stem in age 2 for all 40 years, so no crown age is entered.
- **The third cause is the palm's age-2 continuation of 0.** The palm never draws it within 40 years, yet it mixes in as 1 − t, which makes a per-seed switch once the lifespan is clamped.
- **This walk only.** The other five film walks do not fade. The defect belongs to the one walk with an end that has no branching. The curves are in `R1.md`.
- **The ends' rows.** The oak's age 2 is lifespan 12 and continuation 1, and the palm's is 150 and 0. `lifespan` mixes on `Log { floor: 1 }`, `continuation` and `relay_ended` as `Outcome`.

## Acceptance Criteria

- **R1:** (measure) the curves and the trace above, before any rule is proposed. Stills at t = 0.25, 0.5 and 0.75 for oak to palm, seed 1.
- **R2:** (rule, ruled) R1 measured C1 to C5 (`R1.md`). The host ruled C5 on 2026-10-22, after the owner's verdict on C4. Between the ends (0 < t < 1), `telperion_space::blend` changes the two ends' rows as follows before the mix, in this order:
  - **Rule B over every row (C4).** An age one end never reaches takes every row of the other end: timing, branching (zones, readiness, rhythm), girth, form and leaves. The placeholders `Species::canonical` writes are absence, not settings. What fades is the entry into the age, which already mixes through the ages that lead into it. This supersedes fn-299's narrowing to the timing rows.
  - **Lifespan clamped at the tree's age (C4).** Each end's `lifespan` becomes `min(lifespan, age)` and mixes on the existing `Log { floor: 1 }` scale. A lifespan past the tree's age is dormant (the settings table: "Dormant while the tree is younger than it"). The age reaches the blend through the one place `families` knows it, with no second path.
  - **An unreached end takes the exit rows (C5).** An age's end is reached by an end when its clamped lifespan is under the tree's age. Where one end reaches it and the other does not, the other end takes the reaching end's `continuation` and `relay_ended`. Where both reach it, or neither does, the rows mix as before.
  - No rule switches between the ends' rows by t, adds a species branch, or grows two trees and mixes them (`docs/principles.md`). `families` returns both ends unchanged.
  - **The walk laws weigh by reach (host ruling 2, 2026-10-22).** `walk_law` and `walk_leaves` read a row only where the frame enters its age, weighted by the frame's expected phytomers there (the closed form). A row at an age the frame does not enter is dormant (principle 3). Their tolerances are unchanged, and fn-277's ruling 5a stands: the law refuses a switch the tree draws.
  - **Host decision 1 (host ruling 3, 2026-10-22).** The decision itself stands: the size factor only shrinks, and the middle's count lies within its ends (fn-299). The clause asserting that the walked species equals the raw mix is removed. It followed from the old rows, and under this rule the palm midpoint branches, so the factor shrinks it.
  - **Re-pins (host ruling 4, 2026-10-22).** `catalogue_identity` is re-pinned with a comment citing fn-308. `the_grown_count_binds_where_a_blend_sheds_more_than_its_ends` moves its example point, as fn-299 did. `blend_reach::the_rest_of_an_unreached_age_still_mixes` is rewritten to assert that every row of an unreached age takes the reaching end's value.
- **R3:** (the ruled rule lands; "by degree" as the host ruled it on 2026-10-22) The lateral share is not the measure: over a crown of 20 to 60 m one limb moves it by 0.1 to 0.3, in both directions, which is the noise of a ratio over a small denominator. On oak to palm at 40, at seeds 1, 7 and 42:
  - (a) the largest step in log-wood between frames 0.025 apart is at most 1.1 times the base's largest at that seed (base: 0.756, 0.900 and 0.913);
  - (b) at 0.0025 spacing, over the steepest stretch, no lateral-share step exceeds 0.2. The steepest stretch is defined per seed: the 0.025-grid step with the largest lateral-share change, ± 0.05 around it;
  - (c) per seed, no more frames fall below both ends' wood than on the base (base: 11, 10 and 5 of 41).

  The probe that measures these stays as an ignored test. fn-299's guards hold: the units lie within the ends' max + 2, and the walks' laws as ruling 2 weighs them. The ends are byte-identical. Digests move only where the rule says, measured.
- **R4:** the gate (`scripts/gate.py` as declared) and `npm test` are green, with a codex review per task. The owner sees the before and after stills.

- **R5:** (one path) the species blend takes the frame's age as a required argument. `families` and `centroid` pass the age they draw at, the same whole age that the grower and the size solve read. No unaged blend remains beside them. A test holds every film walk's `families` species equal to the blend at that walk's age in every row but the lateral probabilities, which the size solve then scales by one factor of at most 1 (as `a_walk_between_branching_species_keeps_its_ends_size_at_40_and_80` compares them). Errors: none beyond the blend's existing refusals.
- **R6:** (the ends) a suite test holds that `families(a, b, 0)` and `families(a, b, 1)` are exactly `a` and `b`, for all six film walks at seeds 1, 7 and 42. Errors: no error surface beyond `families`' existing refusal of t outside 0 to 1.

## Boundaries

- Not fn-300 (CPU and device agreement) and not fn-305.
- Joins the release (fn-237), because the release film walks through this frame.
- **R3's step bounds cover oak to palm.** The other five walks are held only by the laws, the units bound and the size tests. Their curves stay as R1 measured them: beech to spruce, spruce to birch and birch to oak keep a share of at least 0.995.
- **Walks whose two ends differ in age are out of R3.** The film's walks share their ends' age. Elsewhere, a frame's age moves with t, so C5's exit rule can switch a side's exit rows at one t. That is recorded here, not solved.
- **A centroid of more than two points** clamps at each fold's own age, as `mixed` folds them. Its order dependence for lifespan rows past a fold's age is recorded, not solved.
- Not the device or GPU: no device path implements the blend (repo scout, 2026-10-22).

## Strategy Alignment

Active tracks served by this plan:
- **The tree space** — a point between two species now draws a tree between them. Placeholders are read as absence, and dormant rows stay dormant, as the space's continuity demands.
- **The catalogue** — the release film walks through the catalogue's species, and this midpoint is a frame of it.

## Decision Context

- **The candidates.** The host ruled C5 over C1 (blend the rate), C2 (share-of-age weighting, which moved the wrong way and switched at the passing end) and C3 (branching rows only, which left fat limbs from the palm's girth row). All were measured in `R1.md`.
- **The laws read the walk rows as `families` draws them, through the aged blend.** The reach weight is the frame's closed-form expected phytomers in an age, over the frame's total. That is phytomers, not short shoots, as a proxy: the closed form counts phytomers per age, and the law asks only whether the frame grows in that age.
- **Laws, cost (measured 2026-10-22, `walk_midpoint::the_reach_cost`).**
  - One closed form per frame costs 0.04 s at beech to spruce, 0.09 s at spruce to birch, 0.27 s at birch to oak, 0.03 s at oak to palm and 0.50 s at laurelin to telperion (age 80).
  - The law's 1,001 steps per walk cannot take it at every step.
  - So the law weighs a step only where the unweighted reading would refuse it: it reads the weight at that step's two frames. A step the rows pass is passed as before.

## Quick commands

```bash
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-space --test blend_reach
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-core --test walk_law --test walk_leaves --test space_walks --test catalogue_identity --test units_bound
MID_WALKS=3 ~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-core --test walk_midpoint -- --ignored --nocapture
```

## Early proof point

Task fn-308-a-walks-midpoint-draws-a-tree-halfway.1 validates the law's reach reading. On the base it passes. With C5's patch applied as a check, it clears the 15 refusals that land on near-unreached ages, and it still refuses a switch at an age the frame grows in. If it does not, re-evaluate the weight definition with the host before .2.


## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | (measure) the curves and the trace above, before any rule is proposed. Stills at t = 0.25, 0.5 and 0.75 for oak to palm, seed 1. | — | Done before the plan: measured in `R1.md` (commits 438f11203, 8d1685f33, ec0624624). |
| R2 | (rule, ruled) R1 measured C1 to C5 (`R1.md`). The host ruled C5 on 2026-10-22, after the owner's verdict on C4. Between the ends (0 < t < 1), `telperion_space::blend` changes the two ends' rows as follows before the mix, in this order: | fn-308-a-walks-midpoint-draws-a-tree-halfway.1, fn-308-a-walks-midpoint-draws-a-tree-halfway.2 | — |
| R3 | (the ruled rule lands; "by degree" as the host ruled it on 2026-10-22) The lateral share is not the measure: over a crown of 20 to 60 m one limb moves it by 0.1 to 0.3, in both directions, which is the noise of a ratio over a small denominator. On oak to palm at 40, at seeds 1, 7 and 42: | fn-308-a-walks-midpoint-draws-a-tree-halfway.2, fn-308-a-walks-midpoint-draws-a-tree-halfway.3 | — |
| R4 | the gate (`scripts/gate.py` as declared) and `npm test` are green, with a codex review per task. The owner sees the before and after stills. | fn-308-a-walks-midpoint-draws-a-tree-halfway.3 | — |
| R5 | (one path) the species blend takes the frame's age as a required argument. `families` and `centroid` pass the age they draw at, the same whole age that the grower and the size solve read. No unaged blend remains beside them. A test holds every film walk's `families` species equal to the blend at that walk's age in every row but the lateral probabilities, which the size solve then scales by one factor of at most 1 (as `a_walk_between_branching_species_keeps_its_ends_size_at_40_and_80` compares them). Errors: none beyond the blend's existing refusals. | fn-308-a-walks-midpoint-draws-a-tree-halfway.2 | — |
| R6 | (the ends) a suite test holds that `families(a, b, 0)` and `families(a, b, 1)` are exactly `a` and `b`, for all six film walks at seeds 1, 7 and 42. Errors: no error surface beyond `families`' existing refusal of t outside 0 to 1. | fn-308-a-walks-midpoint-draws-a-tree-halfway.3 | — |

