**I would reject decision 12 as written.** Removing mandatory whole-tree growth is the right direction, and expectation-based physiology can be a defensible approximation. But the stated rule does not establish junction consistency. It also understates the changes to shedding, topology and botanical behaviour.

I read the six requested documents and the relevant growth, geometry, girth, shedding, light and closed-form code. This is a design review. Performance figures below come from the supplied reports; I did not rerun their captures or benchmarks.

**1. The architectural direction fits the principles, but its central invariant is missing.** One lazy resolver, with the one-pass build exhausting that same resolver, fits the one-pipeline rule. Making the approximation permanent also avoids a fallback from one botanical model to another when the camera approaches.

However, [decision 12](/home/daniel/Projects/telperion/.worktrees/ondemand/.flow/evidence/fn-209-tree-space-f1-trees-resolved-on-demand/PROPOSAL.md:374) repeats the dependency problem identified in [STEP1](/home/daniel/Projects/telperion/.worktrees/ondemand/.flow/evidence/fn-209-tree-space-f1-trees-resolved-on-demand/STEP1.md:50).

Let a chain’s proposed section be

\[
F(C)=O(C)+\sum_{G\text{ borne by }C}\mu(G),
\]

where \(O(C)\) is its realized own-chain contribution and \(\mu(G)\) is a closed-form subtree expectation.

An unresolved child contributes \(\mu(C)\). Once its chain is generated, it computes \(F(C)\). **Generally, \(\mu(C)\ne F(C)\).** Applying the same algorithm does not help when its inputs contain different realized information.

Having the parent generate C’s chain first moves the discrepancy to C’s children. For example, take \(O(C)=1\), \(\mu(G)=4\), and \(F(G)=1\):

- P records C’s section as \(1+4=5\).
- When G appears, its section is 1, although C allocated 4 to it.
- Correcting C changes its section to 2 and invalidates P’s recorded contribution.

For exponent 2, that last correction changes C’s radius from \(\sqrt5\) to \(\sqrt2\), about 37%. I checked this small algebraic counterexample separately; it is illustrative, not a measurement of your trees.

There are three distinct requirements here:

- Parent and child meshes meet.
- Parent pipe allocation agrees with the child’s section.
- Previously visible geometry remains unchanged during refinement.

A shared attachment surface can ensure the first while violating the second. Immutable geometry can ensure the third while violating the second. Streamed-equals-one-pass can pass despite either violation. Those tests are not interchangeable.

The principles this decision strains are:

- **“Stand on botany, invent the space.”** [Tree-space explicitly places the pipe model among reproduced science](/home/daniel/Projects/telperion/.worktrees/ondemand/docs/tree-space.md:18). Replacing specimen-level allocation with population expectations changes that model. It needs to be named and validated as an approximation.
- **Measured cost and look.** “Girth changes slightly” is unsupported. The supplied subtree-radius spreads are large, and no completed decision-12 comparison establishes the resulting error.
- **Question, delete, then optimise.** Questioning full descendant evaluation is appropriate. Deleting its dependence also deletes some botanical feedback; that consequence needs measurement.

Expectation coupling does not inherently violate parameter continuity. Hard table bins, discontinuous shedding substitutions or a switch between exact and expected feedback could violate it.

**2. The botanical case supports statistical approximation, with substantial qualifications.** The premise that a branch should become thinner after losing descendants needs correction. Existing wood does not disappear when leaves or branches die. Shinozaki’s original account includes disused pipes retained in trunks and boughs. Pałubicki explicitly retains branch width after shedding and pruning. A thick branch carrying few current twigs can therefore be realistic. [Shinozaki et al., 1964, original abstract](https://cir.nii.ac.jp/crid/1390001204292708864?lang=en); [Pałubicki et al., 2009, §4.5](https://algorithmicbotany.org/papers/selforg.sig2009.small.pdf).

The important distinction is **a branch that once supported substantial foliage versus one that never did**. Expectation-only allocation can make both equally thick. It loses the relationship between a specimen’s diameter and its realized productive history.

The models provide different levels of support:

| Model | What it supports | What it does not establish |
|---|---|---|
| Shinozaki pipe model | Current supporting pipes plus retained historical pipes | Assigning every stochastic branch its type’s expected demand regardless of its history |
| GreenLab | Compact organ cohorts, source–sink allocation and analytical population moments | That independently realized topology should receive mean-field girth without checking their joint distribution |
| Pałubicki | Approximate environmental inputs, realized basipetal diameter accumulation, retained width | Replacing all descendant contributions with expectations while preserving the same specimen-level mechanics |

GreenLab is the strongest precedent for your computational direction. Kang et al. derive organ-count means and variances and approximate biomass moments, including covariance. That supports analytical summaries, but also demonstrates why a mean alone is insufficient through nonlinear growth equations. [Kang et al., 2008](https://www.sciencedirect.com/science/article/abs/pii/S0378475407002029).

GreenLab also allows radial allocation to mix pipe-based and common-pool mechanisms. The teak study fitted predominantly common-pool ring allocation for its young trees. Thus exact local pipe allocation is not the only botanically motivated choice, but that study does not validate decision 12’s particular approximation. [Tondjo et al., 2018](https://academic.oup.com/aob/article/121/7/1397/4955412).

**The amount of realism lost is currently unknown.** The reported oak ratios of approximately 0.09–2.07 and beech ratios of approximately 0.33–1.34 are comparisons with kind means, not measured errors of the completed proposed estimator. They nevertheless rule out assuming a uniformly small effect.

Large branches may average over many descendants. That argument weakens when descendants share early mortality, shade, vigour and shedding events. Those correlations prevent millions of phytomers from behaving like millions of independent samples.

**3. A better design starts with an immutable boundary contract.** Every unresolved bud should expose one canonical summary whose interpretation remains unchanged after refinement. Parent allocation, child attachment, coarse light and coarse loading must explicitly agree on which parts of that summary they use.

There are two coherent ways forward.

The smaller engineering change is a **permanent coarse physiological model with boundary-constrained chain geometry**:

- Commit the interface radius, load, retained-pipe history and coarse physiological inputs independently of camera resolution.
- Generate the chain’s topology lazily.
- Construct its radius profile subject to the committed attachment radius. Do not recompute that boundary from “own realized chain plus fresh expectations.”
- Keep any local adjustment inside that chain. Do not multiply it into descendant radii.
- Use exactly this definition in the exhaustive build.

This guarantees stable interfaces if implemented correctly. It deliberately gives up exact specimen-level pipe accounting. A fixed smooth transition from the committed base to a locally informed profile is one possible construction, but its taper and thickness correlations require visual validation. It is a proposed replacement, not an already measured solution.

The stronger botanical design is **sampled aggregate state followed by conditional refinement**. Sample a subtree’s coarse state first, then generate its detailed structure conditional on that state. For a simple branching process this could mean sampling total descendant counts and subsequently sampling offspring partitions conditioned on those counts.

That can preserve both variation and aggregate consistency. But the later topology must actually be conditioned on the sampled aggregate. Giving a stand-in a random radius and subsequently running the existing independent lineage draws does not achieve this. It merely replaces expectation error with disagreement between two realizations.

For your full model, joint summaries of survival, pipe history, mass and light may be expensive to sample and condition on. Means alone do not supply such a sampler. This is a promising design investigation, not a proven inexpensive substitution.

The other proposed approaches have narrower uses:

| Approach | Assessment |
|---|---|
| Propagate realized corrections through a fixed number of ancestors | Cheap, but leaves an unresolved discrepancy at the boundary. There is no general fixed-depth error guarantee. |
| Propagate until the projected discrepancy is small | Potentially useful, but work is data-dependent and may reach the root. It needs conservative residual bounds. |
| Post-pass over resolved chains | Produces consistency for a partial tree. As more chains resolve, upstream girth and possibly downstream positions still change. |
| Temporal interpolation of corrections | Bounds change per frame if controlled, but can leave visible error for many frames. It does not guarantee correct detail on arrival. |
| Resolve complete small subtrees | Useful where their worst-case work fits the budget. A measured average of 70 phytomers is not a bound. |

For fixed exponent \(e\), correcting section \(S\) by \(\Delta S\) changes radius by

\[
\Delta r=(S+\Delta S)^{1/e}-S^{1/e}.
\]

A screen-space stopping rule needs a bound on the **remaining aggregate** \(\Delta S\), not merely the most recent correction. Sag additionally moves descendants, so a radius-only bound is insufficient.

At the proposal’s 720-pixel height and 45° vertical field of view, half a pixel at 5 cm is approximately **0.029 mm** of transverse displacement. That makes “small enough locally” a demanding condition.

Also, no finite computation budget guarantees fully correct detail after an arbitrary unanticipated camera move. Continuous motion alone does not provide sufficient prefetch time. The realtime claim needs a supported camera-motion/workload envelope and measured scheduling latency. Decision 13 guarantees neither merely by removing a blocking whole-tree call.

**4. Several hidden risks need explicit treatment before implementation proceeds.**

**Topology preservation is overstated.** In [balance.rs](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-space/src/grow/balance.rs:44), descendant light and upkeep determine a hazard that removes living apexes. Changing those inputs changes outcomes despite identical random draws. Light also changes viability in `grow/advance.rs`. Replacing descendant-dependent final shedding can change the kept topology even for neutral-light species. Distinguish unchanged random draws, unchanged pre-shedding organogenesis and unchanged final topology.

**Shedding requires joint history.** Generally,

\[
E[W\,1_{\mathrm{kept}}]\ne E[W]P(\mathrm{kept}).
\]

Large and long-lived subtrees are more likely to remain. STEP1 already observes survivorship bias. Presence-weighted last-living times, maxima and dormant-bud dependencies make an extinction probability alone insufficient. Retained pipe also needs historical allocation, with live and disused contributions partitioned without double counting.

**Nonlinear quantities need different summaries.** [girth.rs](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-space/src/girth.rs:82) converts a child’s section through the parent’s exponent. When exponents differ, the parent needs a moment such as \(E[S_c^{e_p/e_c}]\), which generally differs from \(E[S_c]^{e_p/e_c}\). Expected radius, expected section and expected wood mass are not interchangeable. Secondary-growth blending introduces another distinction between carried section and drawn radius.

**Sag is especially sensitive.** The code uses radius to the fourth power in bending. Averaging mass and radius separately does not reproduce expected bending. Its existing envelope is length-weighted; a mass centroid requires mass-weighted moments. Ground support further couples load to geometry.

**Coarse light has systematic bias.** The current field applies exponential attenuation. For random optical depth \(D\),

\[
e^{-E[D]}\le E[e^{-D}].
\]

Consequently, an expected-density field generally transmits less light than the mean of varying fields. The proposal’s small measured error from gathering *realized* leaves at centroids does not establish the error from replacing their areas with expectations. Once rendered leaves replace proxies, they must not also enter physiology and double-count the subtree.

**Carbon balance is more than conservation of a total.** Ratios, thresholded hazards and survival probabilities do not commute with expectation. A small whole-tree growth drift can conceal large errors in which branches survive. Check those outcomes separately.

**Determinism requires fixed conditioning state.** Lineage keys reproduce draws; they do not freeze light histories, normalisation, floating-point reduction order or cache-dependent approximations. Each summary needs a complete, versioned input definition. Refinement order, eviction and worker count must never alter it.

**Closed-form cost is already material.** The report gives 0.22–1.54 seconds for existing species/age computations. That is runtime initialization even if shared across trees. The current [memo key](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-space/src/closed_form.rs:50) shares type/state calculations; adding lineage-specific conditioning or continuous light histories can destroy that reuse. Measure cold preparation, cache misses, memory and worst chain-resolution latency. Observed chain maxima are not supported-domain bounds.

**The current envelope cannot certify half-pixel error.** [`Envelope::reach`](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-space/src/closed_form/envelope.rs:53) returns a root-mean-square distance. It is neither a maximum extent nor a bound on deviation from the sampled tree. A strict refinement guarantee requires conservative bounds, or an explicitly probabilistic error contract.

The first decisive check should therefore be a tiny three-generation example that separately verifies immutable interfaces, pipe accounting and refinement-order independence. Larger renders cannot repair an undefined boundary rule.

**VERDICT=REJECT — replace decision 12 with immutable, camera-independent boundary summaries and boundary-constrained chain geometry, explicitly accepting approximate pipe accounting. Investigate conditional sampled aggregates if specimen-level accounting must also hold. Neither option has yet demonstrated the required realtime bound.**