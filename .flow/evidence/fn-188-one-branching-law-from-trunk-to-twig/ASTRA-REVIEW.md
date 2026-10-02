# Astra's challenge of fn-188's first plan, 2026-10-02

`gpt-6-astra`, high effort, read-only, on fn-182-girth, with the beech reference, the horn-fixed beech and a close-up.

**Revise fn-188 before building its probe.** The split causes real defects, especially the oversized terminal hand-off and disconnected radius calculations, but it does not explain the whole hierarchy problem. This beech is principally rule-generated, with attraction disabled; restrictive scaffold rules already produce long, poorly subdivided axes. Establish a trustworthy judging view, then separate radius allocation from topology with small ablations. A unified self-organizing model is a worthwhile subsequent experiment, but “one branching law” is currently too broad to yield an attributable result or a defensible rebuild decision.

1. **The seam diagnosis is only partly right.**

   - **Horns are substantially a hand-off defect.** A childless scaffold tip supplies its full radius to a new radius-derived shoot, whose continuation retains the prescribed length. See [seed.rs:158](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/local/seed.rs:158) and [advance.rs:108](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/local/advance.rs:108). R8’s shortening experiment reduced the local extension from 13.34 to 3.26 m, but left 3.88 m of undivided scaffold pole. Unifying the implementation would not repair that scaffold automatically.
   - **Bare thick wood reflects an explicit eligibility rule.** The radius gate blocks lateral births; terminal births bypass it. This is a removable modeling choice, not an inevitable consequence of having two stages. [seed.rs:132](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/local/seed.rs:132).
   - **Thickness has two separate causes.** The pipe solve excludes local descendants, genuinely disconnecting supporting wood from its fine branching. But equal downstream demands still produce near-equal daughters under any pipe solver; a single-child continuation still conserves radius. Whole-tree solving cannot guarantee progressively subordinate topology. [radius.rs:166](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/radius.rs:166), [radius.rs:194](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/radius.rs:194).
   - **The wall and clean-ups are not one diagnosis.** Local crown clipping is already absent. Scaffold clipping remains. Shedding currently uses geometric shell depth, rather than measured light competition. Apical twig exclusion now happens during seeding for rosette-bearing apices, so describing it as an after-the-fact beech repair is misleading. [planner.rs:51](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/local/planner.rs:51), [scaffold.rs:161](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/scaffold.rs:161), [branching.rs:295](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching.rs:295), [seed.rs:121](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/local/seed.rs:121).

   **The largest missed fact is that this beech does not use attractor competition.** Its weight is zero; production consequently creates no cloud. The standalone nearest-node colonizer is test-only. [beech.values:31](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/presets/european-beech.values:31), [specimen.rs:56](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/specimen.rs:56), [colonization.rs:168](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/colonization.rs:168). Its scaffold prescribes one lateral per deeper station, lengths from the whole parent axis, and forks at absolute heights. Those restrictions survive a shared branching function unless deliberately changed. [scaffold.rs:280](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/scaffold.rs:280), [scaffold.rs:318](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/scaffold.rs:318), [fork.rs:101](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/scaffold/fork.rs:101).

   Finally, R1 explicitly calls the photograph readings qualitative. Exact girth targets, metre-scale shell depths and the reference’s fine topology remain **unchecked**. R5’s tie-break also preferred fewer limb forks, working against progressive subdivision.

2. **Ask whether shared demand and bud competition produce the missing hierarchy.**

   Split that into two cheaper tests first:

   - **Freeze the existing topology and positions; solve radii from fine-branch demand across the whole tree.** Hold root diameter fixed and declare the terminal demand convention. This isolates the radius boundary. Do not feed the resulting radii back into generation initially: today radii determine shoot length and eligibility, which would confound the experiment. [specimen.rs:152](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/specimen.rs:152).
   - **Test deletion of the unconditional terminal extension**, building on R8’s existing shortening evidence. If thick scaffold poles remain, test axis-relative subdivision separately. Removing the entire crossover changes several contracts simultaneously and is a larger experiment.

   These tests can falsify the claim that replacing both generators is necessary. They cannot establish that competitive branching works.

   If needed afterward, use **Pałubicki et al. 2009’s extended Borchert–Honda family**: terminal and axillary buds, one coarse light field, bottom-up accumulated exposure, top-down resource allocation with apical preference, resource-dependent shoot extension, and a whole-tree pipe pass. Keep this an internal mature-tree construction experiment. The paper also retains historical thickness after shedding; “girth equals currently carried foliage” is therefore an assumption to test. [Published model, §§4.1–4.5](https://algorithmicbotany.org/papers/selforg.sig2009.small.pdf).

   Require shorter thick uninterrupted paths, subordinate intermediate branches and fine peripheral wood without order-specific repairs. Compare resource-biased allocation with neutral allocation. **Reject this candidate** if the same poles persist, improvements require reinstating the discarded rules, or acceptable detail exceeds the agreed cost. One failed implementation would not falsify every possible unified law.

3. **Calibrate the judging lens first; do not presume a renderer fix.**

   The 86% figure concerns **P1’s sampled twig segments associated with thick limbs**, not every twig in today’s beech. It measures projected width, not lost coverage.

   Today wood draws its full submitted index range, projects actual mesh positions without minimum-width expansion, and returns opaque fragment colour. There is no camera-based wood rejection in that draw path. The pixel-error selector is for leaves. [wood.rs:277](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-render/src/wood.rs:277), [wood.wgsl:25](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-render/src/shaders/wood.wgsl:25), [wood.wgsl:414](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-render/src/shaders/wood.wgsl:414), [select.wgsl:1](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-render/src/shaders/select.wgsl:1).

   Rasterization uses **4× MSAA when supported**, otherwise one sample; headless captures use that same count. Subpixel triangles can therefore contribute partial coverage, although sampling can miss them. The actual sample count of these captures is **unchecked**. [pass.rs:14](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-render/src/pass.rs:14), [headless.rs:54](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-render/src/headless.rs:54).

   Start with a fixed-camera full-tree render at higher resolution, downsampled in linear light, plus one resolved branch crop. Compare against native resolution with identical geometry and lighting. A closer crop diagnoses topology; it does not prove full-tree visibility. Keep framing fixed across candidates, unlike R8’s auto-framed comparison. Only demonstrated coverage loss justifies coverage-preserving thin-wood rendering. Opaque minimum-width inflation would exaggerate wood mass.

4. **Runtime is a serious risk, and 75 ms is a baseline rather than a verified ceiling.**

   Repeated environment queries and whole-tree resource passes cost roughly the sum of all intermediate population sizes. A naïve implementation can spend far more than a direct build even when final node counts match. The published model reported 21 seconds for 225,000 internodes on a Pentium 4, useful evidence of workload scale but no prediction for modern Rust. [Performance table](https://algorithmicbotany.org/papers/selforg.sig2009.small.pdf).

   Benchmark the same machine, compiler profile, height, terminal detail and requested outputs. Report cold and repeated latency, median/p95, peak memory, iterations, cumulative nodes visited, generated versus retained nodes, and time by stage. Include environment setup, shedding and budget retries. Today’s budget search regrows trees repeatedly, so output node count alone hides cost. [budget.rs:29](/home/daniel/Projects/telperion/.worktrees/fn-182-girth/crates/telperion-core/src/pipeline/branching/specimen/budget.rs:29).

   Compare both shipped beech and the relevant fn-182 candidate: R8 reports **22–26 ms** for the shortened candidate without shedding. Matching 75 ms by producing much less detail is not an equivalent result. Fresh timings and cap diagnostics are **unchecked** here.

5. **Delete assumptions before deleting the architecture; give the owner staged decisions.**

   First remove fn-188’s claims that most defects are proven seam effects, that subpixel width proves disappearance, and that success requires one law. Keep the shell dropped. Experimentally remove the unconditional extension and radius boundary before adding another control.

   Then ask the owner to decide, in order:

   - What visible structure is sufficient, using resolved branch evidence rather than precise targets inferred from the small photograph?
   - What latency and detail trade-off is acceptable?
   - Do the small ablations leave a topology defect worth the competitive-growth probe?
   - Does that probe justify targeted integration, further investigation, or a broader replacement?

   Replace R4’s binary “rebuild or two fixes” with those options. A successful beech probe would justify a production design, not establish support for the catalogue. Preserve rosette behavior and other consumers until replacements demonstrate their contracts. Keep correctness and repeatability checks; historical digest changes alone do not veto an intentional improvement.

No files changed; no tests, renders or benchmarks were run.