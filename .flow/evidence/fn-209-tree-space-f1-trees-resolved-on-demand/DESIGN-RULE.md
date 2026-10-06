# F1: what to resolve for a view, and how to draw the rest (host design, 2026-10-06)

Replaces the error models of probe P3 rounds 1 to 3. Owner goals: realtime, never baked; detail by one screen-error budget, no hardcoded counts (memory `trees-generate-in-realtime-with-screen-2026-10-05`); retuning near-instant, progressive accepted as the mechanism a game needs anyway (owner, 2026-10-06: "aight" to stand-ins first, full detail over a few frames, a GPU path measured).

## What three rounds showed

| Round | Rule | Resolved at 40 m, age 80 | Image against full detail |
|---|---|---|---|
| 1 | position error of the stand-in's extent bound over half a pixel | 84-98 % | beech bare 0.91 % (nothing drawn: 0.72 %); indistinguishable by eye |
| 2 | coverage variance of a density volume over the extent bound | 54-84 % | beech, oak at the noise floor in leaf; spruce 3.6 % against 0.6 % |
| 3 | coverage variance over the envelope, variance over shoots | beech, oak in leaf: nothing (a saturated volume predicts zero error) | 2.7-12 times the floor even fully refined |

Two conclusions:

1. **The cost of a first view at 40 m is mostly the content, not the rule.** At 960 x 720 a pixel is about 5 cm at 40 m. A beech leaf is 1-2 px, a two-year twig cluster 3-6 px: most of an old tree's structure is at or above pixel scale there, and rounds 1 and 2 both needed 74-91 % resolved to match the full tree. No rule makes that view cheap. The speed path is the cost per resolved element (cores, then the GPU), not a cleverer rule.
2. **Statistical error models of the stand-in are fragile.** Each one I specified failed in a new way (too strict, wrong for clumped spruce, blind to silhouette when saturated), and their proxies (tubes, sticks, scattered cards) never carried the look. A stand-in should be judged by a rule that needs no model of its own error.

## The rule

**A chain is resolved when its envelope projects to one pixel or more; every unresolved bud on a resolved chain is drawn as a sub-pixel coverage splat at its bud.**

- *Why it is sound.* An element under a pixel shows only its coverage: where in the pixel its parts lie is invisible. So an unresolved subtree whose whole envelope is under a pixel is drawn exactly, to the eye, by its expected coverage (wood area and leaf area from the committed rows) at its bud's position on its resolved bearer. The splat's position error is under one pixel by construction. This is F3's own principle for wood, which draws a ribbon at true width below one pixel; the rule extends it from a branch's width to a subtree's extent.
- *The envelope*, not the extent bound: the envelope is where the subtree's wood and leaves are expected to be (RMS reach about the centroid, committed rows); the extent bound stays the conservative test for culling and for prefetch only.
- *No new number.* One pixel is the display's own unit; the half-pixel budget of position error stays F3's, inside resolved chains.
- *Leaves.* A resolved chain's own leaves are drawn as the pipeline dresses them. A splat carries the expected leaf area of its subtree as fractional coverage in the leaf pass, and its expected wood area in the wood pass, both blended by coverage (alpha), never by stochastic placement. Its colour is the species' leaf and bark colour; lighting reads the bearer's normal at the bud.
- *Bare and in leaf* use the same rule; only what the splat carries changes.

## What is measured, and the bar

The probe runs the rule against the full tree at 40, 15 and 5 m, ages 40 and 80, four species, bare and in leaf, at 960 x 720 and at 1920 x 1080:

1. Resolved share and first-view cost (1 and 8 cores), and progressive frames to completion under an 8 ms budget per frame, paced by a measured per-chain cost model (P3 round 3's question 3).
2. Image difference against the full tree with the noise floor (full tree with leaves placed by another seed); every still the verdict rests on viewed.
3. The fly-in, frames over 8 ms after the first.

**Bar:** every still at or within 1.5 times its noise floor; resolved share reported, not barred (it is content); fly-in frames under 8 ms for beech and oak at age 80, and spruce's bursts reported with their cause.

**If splats miss the floor**, the cause is measured, not modelled: render the full tree with every sub-pixel subtree replaced by a splat of its *realized* coverage. If that matches the floor, the committed rows are wrong; if not, the splat drawing is.

## Outcome (2026-10-06): rejected by Astra (ASTRA-REVIEW-RULE.md); withdrawn

The review holds. The one-pixel claim is false (sub-pixel overlap, depth order, lighting and shadows stay visible; RMS reach bounds nothing; no stand-in for large pending subtrees), "content-bound" is unproved, and the alternate-seed noise floor measures specimen variation, not rendering error.

**Host decision.** F1 ships with round 1's conservative rule: a chain is resolved when its stand-in's extent-bound position error could exceed half a pixel (correct by construction; stills indistinguishable at 40 m; 84-98 % resolved). F1's speed comes from cost per resolved element (cores, the GPU probe) and progressive refinement on first appearance, with large pending subtrees drawn as round 1's stand-ins. Cheaper stand-ins are a separate later spec on Astra's measured-approximation contract: same-seed reference, adversarial sub-pixel checks (overlap, depth, shadow, threshold motion, cold appearance) before any matrix, and richer splats accepted only where they pass.
