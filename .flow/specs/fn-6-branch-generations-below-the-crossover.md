# Branch generations below the crossover, and one fixed twig

## Goal

fn-5 shipped both presets at eight "orders" of local recursion, and in clay the fine wood bushelled up at the ends of massive limbs instead of filling the crown: each order was one short segment, so 79 cm handoff wood carried 8 m of branching where a real limb carries ~20 m with laterals along its length. Following the owner's botany ("always 1 order of twigs, we increase the order of branches"), this spec replaces the local rules with branch generations that have a branch's anatomy plus one fixed twig, the generation count derived from the wood. Rules are natural at rest, supernatural by dial.

## Decisions

Owner (verbatim, 2026-09-05 unless noted):
> "can you come up with a new preset for them where they ship with appropriate orders and i'll judge"
> "now see that doesn't look right to me.. Those branches/twigs have to fill out the volume not just bushel up at the massive trunks"
> "i mean i'm still confused by having 8 orders of twigs to be honest.."
> "isn't biologically correct always 1 order of twigs we increase the order of branches?"
> "well i see a problem with usability though when we make big trees all branch down to realistic twig size it'll impact performance. But I guess we can think about that later? i'd like to really push the envelope here maybe make an opensource alternative to speedtree."
> "I feel we have a seed of smth a bit special because I'd like to see trees with branching like this in games and i've never seen it.."
> "can we have thousands of realtime generated trees in a scene with this approach?"
> "so that it would be usable in a new version of valheim for example"
> "do we need to be more ambitious with frame targets? if it's to be used in games there's other stuff that takes rendering time"
> "we also probably need to account for procedural leaves? and have them be generated from seeds? that'd be awesome if it's achievable"
> (edit cycle 1) "we should review the biological accuracy of this. It should be structurally close to natural in the way we generate but we should allow for supernatural parameters (not limited by size as we are in this reality, or have twisting trees etc.)"
> (amendment, 2026-09-05) "we need to be able to simulate a tree growing procedurally with a timestep function" / "from seed to actual tree"
> (amendment, 2026-09-05) "we'll be working on fn-6 now have to check how that interacts?" / "can you make amendments to fn-6 to ensure compatibility"

- Branch law (reviewed against literature, recorded in task 1): first branch length `L = C r^(2/3)` (McMahon elastic similarity, calibrated on Telperion), lateral radius `r_parent * lengthRatio^ratioPower` (0.4, 1.3; Weber and Penn), one fixed twig (5 mm diameter), recursion ends on radius, not a counter. The `levels` dial is retired for a derived read-out.
- Laterals sprout along colonization limbs under a stated radius, not only at tips; the shell rule then sheds the interior.
- For growth over time (fn-7, not built here) the pass is a per-node pure function of the skeleton, its radius field and parameters (R8); a developmental model (Palubicki 2009) was rejected and recorded.
- The frame budget is the strategy's game budget: a hero tree inside 2 ms GPU.
- Rejected: a finer colonization step, fewer orders, colonization run length as parent length, sharing parent radius among laterals, a separate rule set for magical trees.
- Out of scope: rendering at scale, procedural seeded leaves, interpenetration (fn-4), performance beyond measurement ("we can think about that later").

## Acceptance Criteria

- **R1:** Every generation below the crossover is a branch (several internodes, laterals along it, bias field and turn limit at every internode), with sourced resting values reviewed against literature — met (~21 m of branching on 79 cm wood).
- **R2:** One fixed twig, identical on both presets at rest, never scaled by tree size — met (5 mm, 25 cm twig edge, leaf every 20 mm).
- **R3:** Generation count derived from handoff radius, twig radius and radius law; `levels` retired; count shown as a read-out — met.
- **R4:** Fine wood fills the crown's shell rather than clustering at colonization tips, both metrics reported and held — met (Telperion 62-68% shell occupancy on the final build vs 20% earlier).
- **R5:** fn-5's continuity and bias-field criteria hold at every handoff — met (127 Telperion, 362 Laurelin handoffs; seam taper tolerance 8 degrees).
- **R6:** Cost measured on both presets on the RTX 3080; hero tree inside 2 ms GPU — measured, budget missed (see Result).
- **R8:** The pass is a pure per-node function of skeleton, field and params; invalid input refused by name — met (prefix test).
- **R7:** Owner confirms in clay the branches fill the crown and the trees read as themselves — met on the fifth pass.

## Result

- Owner R7 passes on 2026-09-05: first "not yet" ("for telperion at original size the resolution of the growth doesn't seem fine enough. Laurelin seems okish. In general there's not even close to enough leaves on any of them."); third "it's not branching right inward. It's too parallel, that's not how a tree would behave. It needs to fill out the canopy, it looks like a cactus." (fix: turn budget scaled by internode length); fourth "very thick branches ending at the border of the canopy, and those reaching out" (fix: `reach` term shrinks colonization's envelope, the pass clips to the silhouette). Fifth accepted: "i think it's fine for now we'll do another spec where we try and build all kinds of real trees as templates with visual QA which will definitely find structural issues."
- Fifth pass changes: branch-local taper `r(s) = r_twig + (r_base - r_twig) * sqrt(1 - s/L)` recorded as `endRadius`; surviving runs keep complete leaders; crown candidates rejected when they would leave the shell.
- Final cost (task 7, RTX 3080, native DPR 1, timer queries): Telperion 175,035 nodes, 1.35 M leaves, 12.9 M wood / 34.5 M scene triangles, CPU build 4.92 s, GPU 5.85 ms; Laurelin 104,207 nodes, 0.80 M leaves, 2.42 s, 3.43 ms. Both miss the 2 ms hero budget and Telperion misses the ~3 s build target; reported, not relaxed. GPU cost barely moves with resolution (resolution-independent work dominates).
- fn-5 R8 annotated as superseded by fn-6 R3/R7.

## Open

- The owner found the tapered branches "very very pokey"; left to the later species-template visual QA work.
- Rendering at scale (geometry, foliage representation, LOD) needed to meet 2 ms; CPU build left to the Rust migration.
