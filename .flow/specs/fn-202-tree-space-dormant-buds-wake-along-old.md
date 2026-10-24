# Tree space: dormant buds wake along old branches

## Goal

The spruce (fn-194) had the frame of a Norway spruce, but seven rounds and three Astra gates ended on the same fault: "the boughs read as thin, flat shelves rather than substantial, drooping sprays with hanging comb branchlets". Values were exhausted; the missing piece was a mechanism. On an old spruce bough, buds that slept for years wake and release the branchlets that fill the curtain (MODEL-SPRUCE.md F4, M10); the same mechanism gives broadleaves their epicormic shoots, so the oak (fn-195) can use it too (host, 2026-10-04). Engine only; the spruce's values stay fn-194's work; no light-driven release.

## Decisions

- Design (host, 2026-10-04): each zone gains a `dormant` lateral table plus a release law (`delay`, continuous yearly hazard `rate`); neutral at 0 and byte-identical. Draws keyed by lineage (`DORMANT`), waking time = delay + inverse-CDF exponential. A bud wakes by degree (share of its cycle left scales its first growth), only on living wood, and the closed form adds expected waking.
- Host decision 1 (2026-10-04): the partial first unit runs its share of the year's risks (survives with viability^share, aborts with 1 − (1 − abortion)^share), and a sleeping bud ages through its stages. Scaling the first unit alone (the Design as first written) still jumped (worst slope 6.3).
- Host decision 2: a shoot woken near its bearer's end keeps share + (1 − share) × p of its size, p its bearer's presence through the next cycle (`Woken::kept`, with a `failed` flag).
- Host decision 3: a woken bud takes its slot's azimuth in the parent's plane and stands half an internode below its node; girth and sag take its load at the node.
- For fn-199: the continuous-time mirror of decision 1 (failing unit grows t = ln u / ln viability of its year) answers the relay's survival jump; relays not changed here.

## Acceptance Criteria

- **R1:** `dormant`, `delay`, `rate`, neutral at 0, every existing tree byte-identical — met (beech stills seeds 1 and 7 byte-identical, oracle exact).
- **R2:** Lineage-keyed draws: adding a sleeping bud moves no other draw — met.
- **R3:** Walks on `rate`, `delay` and a `dormant` probability within bound 30 and the jump check; the release-law walk red first on partial growth alone (`dcf6add1`), green on host decision 1 — met (six release-law walks, worst slopes 1.2 to 2.9).
- **R4:** Closed form matches simulation in distribution, seeds 0 to 3,999 — met (every PA and cycle within 4.5 SE).
- **R5:** Spruce strip walking a dormant probability from 0, viewed by the host — met.
- **R6:** Crate tests, workspace gate, Codex review — met (1,034 passed; Codex SHIP round 2).

## Result

- R5 (seed 1, 80 years, delay 1, rate 0.3, dormant 0 to 0.2): wood rose by degree 46.5 to 56.1 km, height 21.9 m, width 17.1 to 17.3 m. By degree; woken branchlets lie in the spray plane; lower boughs' combs visibly denser. "Not yet heavy draperies": moderate at what the GPU can render (0.3 ran it out of memory). Delay 1, rate 0.3 proposed for round 8. Strips were in ignored `raw/`.
- Codex round 1 fixed: a woken axis's abortion hazard counts units slept (`Draws::aged`); secondary erection reads `Axis::sleep`. Cost: beech 3.5 s before, 3.6 s after (one sample).

## Open

Draperies remain moderate; the spruce's values are fn-194's.

## Friction

- The Design's continuity claim for the release law was never run before ready; cost ~30 min and four walk runs -> run a naive standalone walk before marking ready (AGENTS.md "Gates and checked claims").
- R5 render killed for low memory -> local setup, reported only.
- Gate ran before Codex's fixes, so twice -> gate once after the review's last fix.
