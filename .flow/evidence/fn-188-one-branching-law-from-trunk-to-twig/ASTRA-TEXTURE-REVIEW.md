# Astra's diagnosis of the combed in-leaf texture, 2026-10-03

`gpt-6-astra`, high effort, read-only, with B-BARE, B-WHOLE and the probe's rounds 5 and 7, bare and whole, and the probe code from history.

The direction remains promising, but rounds 4–7 do **not yet isolate a failure of competitive branching**. The foliage adapter introduces repeated placement patterns, and the inherited clumping rule can turn leader systems into separated masses. Underneath those confounders, the probe also lacks differentiated short and long shoots. I would retain round 5 as the structural baseline, correct the adapter and isolate foliage effects before another growth-parameter sweep. This was read-only; visual causality and proposed speed gains remain unchecked by a new run.

**1. What produces the streaks?**

- **A concrete repetition defect appeared with round 4’s foliage adapter.** `to_tree()` constructs every node with `Node::root()` and never assigns its identity. Every node therefore retains the same default birth identity. Short-shoot placement keys its phase, radial bearing and random stream by that identity and station index. Consequently, each segment repeats the same cluster pattern in its local frame. This is a strong candidate for the combing, although its visual contribution is **unchecked**.  
  `fff4da72:crates/telperion-render/examples/selforg_probe/render4.rs:34`; `2ec2c4b2:crates/telperion-core/src/tree.rs:55`; `2ec2c4b2:crates/telperion-core/src/tree/identity.rs:12`; `2ec2c4b2:crates/telperion-core/src/pipeline/foliage/short_shoots.rs:180`.

- **The foliage strongly emphasizes branch trajectories.** Terminal chains become `Twig`; other grown wood becomes `Branch`. Eligible branch wood receives eight leaves every 3 cm, held 20 cm off the wood: approximately **267 leaf candidates/m**. Terminal twig wood receives approximately **20/m** at 5 cm spacing. Thus “eight leaves every 3 cm on long terminal shoots” is incorrect: the dense sleeves principally clothe *branched* wood. The 13.3× density contrast makes branching status an abrupt foliage change, while repeated clusters trace the supporting axis.  
  `2ec2c4b2:crates/telperion-render/examples/selforg_probe/render4.rs:42`; `2ec2c4b2:crates/telperion-core/presets/european-beech.values:109`; `2ec2c4b2:crates/telperion-core/src/pipeline/foliage/short_shoots.rs:163`; `2ec2c4b2:crates/telperion-core/src/pipeline/foliage/station.rs:193`.

- **Growth is unusually uniform at the chosen allocation.** With light disabled, winning buds receive binary \(Q=1\). At λ = 0.5, the allocation equations cancel to \(v_\text{bud}=\alpha Q_\text{bud}\). With α = 2, every winning bud requests **two 0.25 m metamers**, before occupancy stops it. Repeated extensions can make long chains, but the reports provide no terminal-run length distribution or successful-lateral frequency. Those remain **unchecked**. Retirement distinguishes “sees nothing” from “lost ownership”; merely losing allocation does **not** retire a bud.  
  `2ec2c4b2:crates/telperion-render/examples/selforg_probe/grow.rs:343`, `:386`, `:494`, `:524`.

- **Geometry plausibly suppresses fine subdivision, but does not establish the cleft’s cause.** Perception reaches 1.25 m; occupancy removes markers within 0.5 m of every scaffold and grown node. That is a coarse exclusion scale relative to leaf-bearing sprays. Retirement then makes exhausted locations permanent. However, **another mechanism explicitly cuts inter-leader gaps**: inherited `limbClumping=0.25` thins foliage near boundaries between systems, and only `Structural` nodes can open systems. Round 6 makes all new side branches nonstructural, so their foliage inherits the few leader systems. The bare image already has a central opening; foliage clumping can amplify it. Relative contributions are **unchecked**.  
  `2ec2c4b2:crates/telperion-render/examples/selforg_probe/main.rs:65`; `2ec2c4b2:crates/telperion-render/examples/selforg_probe/grow.rs:174`; `2ec2c4b2:crates/telperion-core/src/pipeline/foliage/clumping.rs:25`, `:174`.

- **Plagiotropy changed the wrong observable for this defect.** R7 moves order-3 inclination from 71.2° to 88.3°, while nodes remain approximately 49.5k, coverage approximately 0.85, and tortuosity near 1.005. It rotates almost-straight axes without changing their foliage sleeves or creating branch-associated sprays. Also, the implemented direction is **previous + ξ·markers + η·tropism**; ξ weights markers, not the previous direction.  
  `2ec2c4b2:crates/telperion-render/examples/selforg_probe/grow.rs:531`.

**2. What fine structure is missing?**

Beech needs a distribution of shoot forms and coherent sprays, not merely more horizontal branches.

- Dupré et al. describe short shoots with **3–5 short internodes and undeveloped lateral buds**, versus long shoots with **6–10 variable-length internodes and functional lateral buds**. Their first growth flush is plagiotropic and two-ranked; later flushes can differ. They describe both monopodial and sympodial development, so obligatory terminal death would be an overcorrection. [Original paper, pp. 88–89](https://scispace.com/pdf/morphologie-et-architecture-des-jeunes-hetres-fagus-v4vyonrp5k.pdf).
- Branching should have positional structure within a shoot. Nicolini identifies preferential development of upper laterals, rather than equivalent opportunities at every station. [Nicolini, 1998](https://publications.cirad.fr/une_notice.php?dk=390685).
- Mature upper-crown measurements also show shoot-dependent foliage counts and annual variation: approximately 3.2 versus 4.7 leaves per nonfruiting versus fruiting shoot in the ambient treatment. These are contextual observations, not universal preset values. [Han et al., 2011](https://pmc.ncbi.nlm.nih.gov/articles/PMC3101148/).

**Existing parameters** can change departure angle, inclination, curvature, competition spacing and leaf density. They cannot independently specify compact shoots, within-shoot bud development, or a persistent branching plane. The probe hardcodes bud bearings from the **global node index × 137.5°**, rather than shoot-local two-ranked organization.  
`2ec2c4b2:crates/telperion-render/examples/selforg_probe/grow.rs:138`.

**Mechanisms needed** are variable extension and internode compression, shoot-local orientation, and differentiated lateral development within the same branching process. A small zigzag requires alternating local geometry; smooth tropism alone does not express it. Exact mature-beech shoot-length percentiles and laterals-per-shoot counts are **unchecked** here. Comparing a 25 cm *metamer* directly with an annual *shoot* also mixes different units.

**3. Keep the direction? Smallest next experiment?**

**Yes as a hypothesis, not yet as an integration decision.** Competition removed the thickness gate and excessive discarded growth; the whole-tree pipe improved taper. R5 remains the strongest supplied bare result. R6’s leader count does not establish better overall architecture.

The adapter and foliage contract need attention first. In particular, “leaf pipes” currently estimate demand from length and spacing; they do not reproduce radius eligibility, station rounding or subsequent clumping/culling. They are a demand proxy, not the actual retained-leaf count.  
`2ec2c4b2:crates/telperion-render/examples/selforg_probe/grow.rs:418`.

**Smallest next experiment:** freeze R7’s seed-1 positions, topology and radii, then make a four-image comparison:

1. Current adapter, current clumping.
2. Unique stable node identities, current clumping.
3. Current adapter, clumping zero.
4. Unique identities, clumping zero.

Use the same resolved branch crop and whole-tree framing. This separates repeated short-shoot patterns from the inter-leader gap without changing branching. **Identity repair is the smallest concrete correction most likely to reduce artificial repetition; it is not a demonstrated complete texture fix.**

If sleeves persist, test differentiated short/long shoot development on R5’s scaffold. Measure terminal-run length, successful laterals per metre and foliage contribution by wood class. Avoid another global angle sweep.

**4. Structural speed paths the reports miss**

- **Neutral allocation can delete work.** At λ = 0.5, compute \(v=\alpha Q\) directly instead of traversing the hierarchy basipetally and acropetally. R3-SPEED attributes about **5.9 ms on beech, 17 ms on oak** to those passes. This is an algebraic consequence of the existing equations; implementation equivalence and savings are **unchecked**. It does not apply unchanged to nonneutral λ.  
  `2ec2c4b2:crates/telperion-render/examples/selforg_probe/grow.rs:386`, `:494`.

- **Existence queries do not exit early.** `sees()` continues scanning every nearby marker after finding one. A genuinely short-circuiting query could reduce extension work without changing its answer. Savings are **unchecked**.  
  `2ec2c4b2:crates/telperion-render/examples/selforg_probe/space.rs:190`, `:223`.

- **Foliage representation is the larger opportunity.** Short shoots and limb clumping explicitly exclude this family from the compact station plan. Supporting these placements as compact descriptors would target the dominant expansion cost while retaining the pipeline. This requires implementation and measurement, not another growth optimization.  
  `2ec2c4b2:crates/telperion-core/src/pipeline/foliage/plan.rs:17`; `2ec2c4b2:crates/telperion-core/src/pipeline/foliage/prepared.rs:30`.

R7’s meshing is **91% of probe-plus-mesh time**; eliminating growth entirely improves that total only about **1.10×**. R5’s faster beech also has 1.17M leaves versus the comparator’s 2.52M, so it is not an equal-detail speed proof. Laurelin’s 7.12 s versus 0.367 s makes excess generated foliage especially urgent. Frozen parallel extension already failed its measured speed test in R4; the report’s earlier estimated gain should not guide the next experiment.