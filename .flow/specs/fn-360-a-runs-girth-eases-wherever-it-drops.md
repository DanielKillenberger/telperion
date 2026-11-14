# A run's girth eases wherever it drops

Child of fn-351 (P1's old trees; Phase 3, "a flat rim where two stems fork"). Host-written, 13 Nov 2026. Builds on the phase's top branch (`fn-359-a-subtree-fades-once`).

## Problem

Measured (`.flow/evidence/fn-351-020-the-release-candidate/FORK-PROBE.md`, the Oregon white oak on seed 1001 at 120 and 200; sheets beside it, viewed by the host):

- **The ledge.** Past a codominant fork the surface eases the run's girth from the fork node's radius (`surface/samples.rs`, `girths`). A second fork inside the first fork's ease window restarts the ease from its own smaller radius: the trunk run at 200 falls from 0.6375 to 0.4577 m over 0.20 m, at the 1-in-1 floor. 6 of 80 forks sit at that floor. It draws as a flat-topped ledge all round the trunk with the stems standing on it. 0.1.4's successor on master draws the same ledge.
- **The bands.** Every stem carries regular dark rings. Projected ring by ring into the still, they sit where the node radius the grown tree hands the surface drops 3 to 6.5% in one ring (constant over 10 to 13 rings, then a step at a growth-unit boundary, about every two units) and at relay joins (9.3% over one 0.40 m segment). The steps are the grown tree's: the node radius equals the phytomer's radius times its scale in every row. They are right as wood (the section follows the leaf above, which steps at each lateral); drawn ring to ring they are a small downward-facing shelf that the light draws black.

A trial (reverted, not product) started the ease past a fork from the run's drawn girth at the fork and ran it over twice that girth: the trunk run at 200 eases 0.6375 to 0.325 m over about 1.25 m, forks at slope 1.00 go from 6 of 80 to 0, the largest ring-to-ring fall on the trunk run at 120 is 3.7%, and all 56 surface tests pass. The host viewed its sheet: the ledge is gone and the trunk tapers into the stems; one dark band remains round the trunk where the radius steps (node 112), which is the second defect above.

## The design (host)

One rule for every drop, fork or not, with no state (host, 13 Nov 2026, final form; it replaces a first rule whose ease depended on its own history, and a max form whose term r(s) stepped down at a second drop and left a band). Along a run, every ring j where the grown radius falls from the ring before (δ_j = r(j−1) − r(j) > 0, a_j = r(j−1)) lays a hold that fades over twice the radius it fell from, and the holds add:

  g(s) = r(s) + Σ over drops j with s_j ≤ s of  δ_j × w((s − s_(j−1)) / ℓ_j),   ℓ_j = min(2 a_j, S_end − s_(j−1)),   w(t) = (1 − t)²(1 + 2t) on [0, 1], 0 beyond

with S_end the distance of the run's last ring: a hold fades over twice the radius it fell from or over what is left of the run, whichever is shorter, so a run ends at its own radius (host, 13 Nov 2026, after a 3 mm shoot carrying on from a 0.227 m limb was drawn ending 0.116 m wide).

The radius falls across the segment from ring j−1 to ring j, so a hold's clock starts at s_(j−1), where the fall starts (host, 13 Nov 2026).

(w is the ease curve the fork already used). Rises lay nothing, so g ≥ r always, and g = r wherever no drop lies within twice its own upstream radius. g is a pure function of the run's radii and distances: no state, no order.

- One step from a to b draws b + (a − b) w(Δ/2a), Δ measured from the ring the wood fell from. Across the dropping segment of length L, g falls by δ(1 − w(L/2a)): a fall over a segment at least twice its radius long is drawn at the node radii (tips taper and collapse as before), and on closely ringed wood it is a sliver of the drop (0.4% of it at L = 0.033 m on a 0.47 m radius).
- A stem thinning steadily is drawn wider than its radius by at most the thinning within each ring's upstream reach (each hold is at most its fall), and, where a full reach of wood lies upstream, at most half that thinning plus one ring's drop.
- The 1-in-1 floor stays as a separate step after the rule: without it a twig of the oak at 200 falls 1.57 in 1 (node 27778). The floor outranks the run's end (host, 13 Nov 2026): a run whose fall is steeper than 1 in 1 near its end ends where the floor draws it, as the base did, with no hold.
- The scan reaches back only as far as a hold can reach (s − s_j < 2 a_j; bounded by twice the largest radius seen so far on the run).
- The run's base, the collar swell and the sockets are untouched; the rule reads the radii the old ease read.
- The fork's own constants (`FORK_FROM`, `FORK_AT`) decide what is a codominant fork and stay as they are; the ease past a fork becomes this rule and its separate window goes.

No new value: "twice the radius" is the fork ease's existing length (`2*at`). The rule is drawing only: the grown tree, its radii, its wood and its digests upstream of the surface stage do not change.

Not checked by the host, to be read by the builder before changing anything: where a run starts on its parent (a lateral's first ring against the bearer), whether the collar there has its own rule that this must not fight; how the bark coordinate along the stem and the curve clusters read the girth; and whether any consumer needs the drawn girth to equal the node radius exactly at a node (attachment of children, leaf placement, the wire and the distant levels). A child attaches to the bearer's drawn surface or to its node radius: find which, and if a child would now start inside the bearer's drawn wood or float off it, stop and return with the site.

## Acceptance Criteria

- **R1.** The reading above, in the evidence, before any product change.
- **R2.** Tests, red before and green after: a hand-built run with two forks 0.3 m apart on a 0.64 m stem, whose largest ring-to-ring slope is under the floor and whose girth falls monotonically over at least the first ease length; a hand-built run with a 5% step every 0.8 m, whose drawn girth has no ring-to-ring fall above 1%. A third: a run whose radius never drops is drawn at its radii to the bit. A fourth: a run thinning 0.05% per ring over 200 rings is drawn over its radii by at most the thinning within each ring's upstream window, and where a full window lies upstream by at most half that thinning plus one ring's drop, plus 1e-6. A fifth: a ring's girth reads only the wood up to it and where the run ends: extending a run moves no ring lying at least twice the widest upstream radius before the old end, to the bit, and a run cut at a ring draws that ring with no hold. A sixth: a 0.2 m run carrying on into a 3 mm shoot ends with no hold (at the floor below a 0.2 m shoot, under the old rule's 0.1015 m; at 3 mm below a 0.6 m shoot) and narrows all along the shoot.
- **R3.** The existing surface, fork and bark tests pass, or a test that changes is named with what it checked.
- **R4.** Measured before and after on the oak, seed 1001, at 120 and 200: forks at slope 1.00 of all forks; the largest ring-to-ring fall on the trunk run and over all runs; count of ring pairs falling more than 2%; wood vertices; one-thread seconds of the surface stage. The same for the spruce and the beech at 40 on one seed.
- **R5.** Stills on one sheet, before beside after: the oak's first fork at 200 from two azimuths as the probe drew it, one stem of the oak at 200 where the bands were matched, the spruce's and the beech's trunk at 40.
- **R6.** Every digest pin that moves is listed with the tree it pins. Pins upstream of the surface stage do not move, and that is checked.
- **R7.** `docs/pipeline.md` or the module doc states the rule.

## Stops

- A consumer that needs the drawn girth equal to the node radius at a node; a child that starts inside or off the bearer's drawn wood: return with the site and numbers.
- Any tree on the narrow suites whose wood vertices rise by more than 2%.
- Any question about what the grown tree's radii should be: that is the girth rule's, not this spec's.

## Boundaries

Drawing only, in the surface stage. No change to `telperion-space`, to the girth rule, to what is a fork, or to any preset. No new value.

## Rigor

Product code, Opus; narrow tests while building; the gate and review at the phase's landing. Heavy runs through `telperion-heavy` with `TELPERION_HEAVY_RESERVE=8`, one GPU job at a time; a watcher may freeze a job for a long time, so jobs run in the background and are waited for, and wall-clock seconds count only with no PAUSE line of `yield.log` inside the run.
