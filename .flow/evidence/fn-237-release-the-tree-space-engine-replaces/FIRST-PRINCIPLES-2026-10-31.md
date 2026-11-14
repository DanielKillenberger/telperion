# Rethinking generation from first principles, 31 Oct 2026

Daniel asked what botanically accurate, fast-to-generate, fast-to-render trees would look like with no limit on engineering. This is the host's proposal. No part of it is owner-approved, and no experiment below has run.

## Baseline (measured 31 Oct, RTX 3080, native, warm, fn-319 head)

Grid size does not change cost: oak 236-239 ms, spruce 441-468 ms and birch 215-220 ms at 12, 24, 36 and 48 cells. Oak at 36 cells, seeds 1, 7 and 42: age 40 is 247-277 ms, age 60 is 598-799 ms, age 80 is 1,119-2,043 ms. Ages 100 and 120 are refused because the cycle's axes buffer passes the 2 GiB binding limit. Run with the GPU also in other use (45-57 %), so treat these as a diagnostic sample.

## Proposal

Treat growth as a multi-scale simulation, on the model of large-eddy simulation. Resolve the scaffold year by year and coupled, represent shorter-lived wood with a closure trained on the full simulator, and realise twigs and leaves on demand from lineage keys and the scaffold's recorded conditions. Run the scaffold on the CPU and the flat detail on whichever device is available. Render through one screen-error cut over generated pages that can be evicted and regenerated. The full cycle stays as the reference, and as the N = 0 end of one resolution parameter.

The acceptance contract moves from "this seed matches the reference's tree" to "the same distribution as the reference, and indistinguishable to the owner beside it".

## Scope (Daniel, 31 Oct 2026)

"i want realistic trees in 3d engines like unreal also. We don't just want my website to function again. It needs to work on any scale/resolution." The website is one request on the resolution parameter. The hero tree in Unreal is the harder acceptance case, and both are judged on the same tree. Unknown and untested: whether realised twigs can sum to the closure's predicted totals so a zoom shows no pop.

Daniel, 1 Nov 2026, narrowing this for the 0.2.0 goal (fn-321): "we don't want to integrate anything in unreal. this goal should only work within the harness and being imported into a website. Any imports to game engines are out of scope for this goal. But what we're builing must have a modular path to build a plugin or smth to be imported by game engines to generate trees at scale." How generated pages reach an engine's renderer stays unknown and is for a later goal.

## Training volume (host, 31 Oct 2026, arithmetic from the baseline)

Daniel proposed training a network on high volumes of generated trees. Today's warm times imply about 15,000 age-40 oaks or 2,700 age-80 oaks an hour on the RTX 3080, excluding storage and readback, so generator speed does not block a first dataset. Its range does: nothing past the refusals near age 90-100 can be learned. A regression network would repeat form (a)'s failure by predicting the mean, so the model must sample a distribution. Inference cost is not automatically lower: 46,656 field queries or about 24,000 closure calls at about 1 µs each is 24-47 ms. fn-268 holds the whole-field version of this idea; a closure uses the same data pipeline and carries less risk.

## Evidence for it (fn-245 COUNCIL.md)

- Silhouette, limbs and shed interior need the coupled years; mid-view architecture needs the last few years; close twig and leaf geometry needs no history.
- Form (d) was exact on the CPU but slower on the device, because the serial dependency down the tree stays. Log-depth sums reached 1.3-1.8x and fired their stop condition.
- A CPU scaffold axis costs 900-2,270 ns a cycle.

## Evidence against it

`couple::Aggregate` reached fidelity only with nearly full expansion. Form (a), the closed-form mean, ran +14-17 % on the oak with half the sheds. Form (c) failed the oak by +2.5-4.2 %. All three were hand-written closures judged against the same seed. A trained closure carrying a spread is untested.

## Owner decisions it would reopen

- 11 Oct: the coarse-record route closed.
- 13 Oct: no shortcut that changes the grown tree beyond its measured tolerance (AGENTS.md, "Botanical growth is the product").
- Runtime twig motifs refused, if a closure's table were to touch geometry. The proposal keeps tables to statistics.

## Experiments, in order

1. **E0, what the view needs.** From existing full trees at 40 and 80, across seeds: live axes by order and year, and the 36-cell field drawn from orders up to k with the rest as density. No new simulation. Stop if the field needs the fine orders' own positions.
2. **E1, the closure.** Record subtree transitions from the reference across the oak's seeds, train the smallest closure, then run scaffold plus closure. Compare distributions over 50 seeds (height, spread, limb counts and angles, shed share) and cost. Blind stills beside the reference at the grove's view, the whole tree, mid and close, plus one continuous walk from far to a twig at 5 cm. Stop if Daniel can tell them apart at any view, if detail pops or disagrees with the closure's totals on the walk, or the closure cannot hold sheds and low limbs.
3. **E2, old trees.** Extend the reference past today's limits offline, and check that cost per year flattens once the crown fills.
4. **E3, the web floor.** Scaffold plus closure in single-thread Wasm, cold, with its size.
