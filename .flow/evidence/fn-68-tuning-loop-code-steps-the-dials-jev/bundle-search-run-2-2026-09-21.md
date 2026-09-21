# Bundle-search run from bundle 1's tree, 2026-09-21

Production `tuning-loop` at `65e67062` (CI-profile gate EXIT=0, 804 tests): side-effect veto, contact sheet v2 (overall believability ranking and per-render "what looks wrong"), measured facts in Jev's state, named gate failures. Fresh bootstrap run whose baseline is bundle 1's tree (score 0.3045), owner's five priorities in the owner's words (approved in advance: "but yea list seems right other than that", "yes"). Raw state in ignored scratch `.flow/tmp/fn68-canonical/bundle2/`.

Routing, every round: crown shape, hanging foliage and branch structure to tuning; materials to appearance; **crown proportions to insufficient_evidence**, so the owner's first priority never reached the sheet.

Five bundle rounds, 33 to 52 dials each, nothing adopted. At half a step every variant ranked below the current tree overall and was graded worse on branch structure and hanging (one "slight" on crown shape twice); the reviewer's notes match the owner's complaints: "thick outer limbs and angular, crossing branches create a chaotic scaffold", "foliage is too sparse", "crown-wide skeletal lattice". At one and two steps the tree fails the species gate `height_m outside 25 to 40` (values 22.0 to 24.6). At four steps the crown is lost ("a bare pole with a tiny leafy tip"). The v2 sheet therefore refused exactly the kind of step the v1 sheet had adopted twice. No adoption, so the veto never ran.

The run then stopped on a Jev transport error: `HTTP 400 max_tokens_exceeded` on the first proposal batch of round 27, with the reservation left pending; the proposal state had grown with five rounds of bundle attempts and their reviewer notes.

Reading. Jev's bundles are broad and nearly the same each round despite five rounds of "worse" feedback, and halving only triggers on better-with-breaks, never on worse, so the search cannot find which part of a bad bundle is good. This is a stall: more rounds of the same shape would repeat it.

Reviewer spend: 7 calls (one all-view review, one initial review, five sheets). Visual passes 61 of 140, evaluations 84 of 140, images 414 of 700, rounds 27 of 34.

## Why nothing was adopted: the owner's first priority never reached the proposal question

Measured from `.flow/tmp/fn68-canonical/bundle2/run.json`, read-only:

- The beech's built height at the run's baseline is 29.89 m; the species numeric gate is `height_m` 25 to 40; the authored envelope height is 32.0 m.
- Across five rounds Jev proposed 53 distinct dials and **never once proposed `envelope_height`**, the dial that makes the tree taller. It did propose `spread` **up**, which makes the crown wider. The owner's first priority is the opposite of both: "the whole crown needs to be stretched vertically it's longer and taller in the reference", and the measured fact in the state says the crown is 8% too wide for its height.
- Every bundle at strength 1 or 2 then failed the gate at `height_m` 21.97 to 24.59: the moves it did choose (apical dominance down, lateral pitch up, rise primary down) spread the tree instead of raising it, so the built tree fell five to eight metres short of its envelope and the gate refused it. The gate was working; the direction was wrong.
- The cause is upstream of the bundle: `owner-proportions` routed `insufficient_evidence` in every round, and the proposal state lists only the priorities routed to tuning, so proportions was never among the objectives Jev was asked to move a dial for. The measured fact was in the router's state but the route question never told the router that a measured fact is evidence.

This is the defect the routing fix in `9334152f` addresses, and it is the reason to expect a different bundle from the next run rather than a sixth repetition. It also explains bundle 1, the one tree the owner called an improvement: it too raised `spread`, so even the accepted step moved away from the proportions the owner wants.
