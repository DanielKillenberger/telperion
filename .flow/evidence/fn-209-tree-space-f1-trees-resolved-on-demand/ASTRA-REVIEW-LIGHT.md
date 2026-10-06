**Verdict: accept with changes.** The coupling tree is a principled architecture for camera-independent physiology and lazy geometry. The constant-light stand-in, allocation contract, continuity rule, and timing gate need revision before implementation can claim to meet the owner’s requirements.

I reviewed the supplied design, proposal, earlier review, principles, and relevant code. Performance figures below are reported measurements or estimates, not benchmarks I reran. `PROPOSAL.md` ends at decision 14 in this checkout; I read decisions 15–17 through `STEP1-C.md` and their implementation. No files were modified.

**1. Is the coupling tree sound, or a patch?**

Sound in principle. Resolving the large structure that organizes the light field, while representing smaller subtrees by permanent physiological summaries, is a coherent approximation. Camera refinement must never replace those summaries in physiology. Both streamed and exhaustive builds must consume the same state and growth rules.

However, the opening justification overstates the evidence. The proposal explicitly labels **expected versus realized leaf area as unknown**. Its 5–9% p95 result tests reading one light value per fine subtree using realized foliage. It does not establish that an expectation-driven coupling tree achieves that error. P2’s poorer result and the earlier result measure different approximations.

Restoring the coupling tree is therefore a reasonable hypothesis, not a demonstrated recovery of fidelity.

Also, spatial extent alone does not establish physiological insignificance. A small subtree can cross a sharp shadow boundary or carry a consequential starvation hazard. Extent is a useful first approximation criterion; the probe must establish where it works.

**2. Does it answer the five dependency questions?**

| Question | Assessment |
|---|---|
| Time axis | Substantially answered |
| Circularity | Answered for explicit time stepping, with an unresolved classification dependency |
| Allocation | Partly answered; “exact” is too strong |
| Balance hazard | Not answered by an age/light table |
| Leaf girth and retained pipes | Leaf-girth input identified; retained-pipe state still unspecified |

The details matter:

- **Time axis.** A camera-independent forward pass supplies the history lazy chains need. Preserve the actual ordering. [`Grower::step`](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-space/src/grow.rs:366) performs balance, allocation, growth, and waking; sketching, periodic re-layout, and shading follow. The design’s parenthetical ordering differs. Boundary records also need inherited survival/presence, including the ancestor balance factor `K(c)`, and when growth terminates. Light and vigour alone are insufficient.

- **Circularity.** Reading the previous cycle’s light removes an instantaneous light-growth fixed point. But classification must also be computable without the history being constructed. If “expected extent at the tree’s age” depends on light-driven size or survival, the design needs an independently defined classification predictor. Otherwise the circularity moves into deciding which chains enter the pass. Using final-age classification also makes earlier coarse histories potentially depend on the requested final age; that consequence should be explicit.

- **Allocation.** Borchert–Honda can conserve the incoming budget exactly on the represented coupling tree. That does not make its allocation equal to the detailed tree’s allocation. The PA normalizer in [`sizes`](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-space/src/allocation.rs:99) needs both  
  \[
  \sum p_i,\qquad \sum p_i(v_i/p_i)^{\psi_i}.
  \]
  A subtree’s total vigour and expected bud count do not generally determine the second quantity. Internal branching and apical control matter. The stand-in must supply an approximation to this moment, by PA, and the design must specify how lazy chains subdivide their incoming vigour without resolving descendants. Retain decision 4’s **under-2% per-PA growth-drift check**; the proposed P4 bars omit it.

- **Balance hazard.** Constant light does not make a realized subtree’s balance a function of age and light alone. [`balance.rs`](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-space/src/grow/balance.rs:25) uses living-apex light, accumulated wood upkeep, remembered balance, and ancestor shedding. Wood also depends on allocation history. Even under constant light, different specimens and incoming allocation histories produce different balances. Applying the nonlinear hazard to mean quantities creates a new approximation; it does not compute the expected hazard automatically.

- **Leaf girth and retained pipes.** A global scalar is appropriate for leaf normalization, provided its numerator and denominator reproduce the intended population and weights. [`attachments`](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-space/src/girth/disused.rs:53) computes that scalar over the grown tree before shedding. Each phytomer needs its light when it grew. Retained pipes additionally require the section associated with lost wood and its fade, transferred without double-counting live section. Section 7 currently supplies no such rule.

The earlier review’s immutable-junction requirement also remains. A coupling pass does not itself guarantee agreement between a committed attachment and subsequently resolved geometry. If “option 4 geometry” supplies that contract, this design should explicitly depend on it.

**3. Is constant-light closed form defensible?**

**As an oracle and a restricted approximation, yes. As the general state of a fine subtree, no.**

Even the simpler shade-survival term exposes the problem. For fixed viability \(v\), shade exponent \(\phi\), and whole cycles:

\[
\log P_{\rm survive}=\log(v)\sum_c I_c^{-\phi}.
\]

Substituting arithmetic mean light generally changes that sum. An effective light could preserve this particular cumulative hazard, but it would not also preserve branching times, allocation, wood accumulation, balance memory, or girth. Early shade followed by release can produce a different tree from early light followed by shade despite identical mean light.

Comparing a history-grown subtree with a mean-light table is useful, but one realized subtree is not an oracle for an expectation. Test multiple lineages under controlled histories, including both orders of a shade/release sequence.

**I would use a compact state that advances each cycle under the actual coarse inputs.** It should track the quantities its consumers need: living cohorts by PA/development state, wood upkeep, allocation moments, balance memory or an explicitly approximated distribution, and live/disused pipe contributions. Birth cohorts preserve when light and size affected wood.

Derive its transitions from the shared growth rules. Evaluate them online from current parameters; do not bake species responses offline. Constant-light tables remain useful for validating that recurrence. They should not repeatedly replace its accumulated state.

This still requires approximation and measurement, especially for correlated subtree death. It makes the approximation explicit and preserves the causal information the current design discards.

**4. Can it reach about 100 ms per oak retune?**

**The coupling pass might. The proposed complete retune has no credible 100 ms case yet.**

Two obstacles dominate:

1. The estimated one-second pass becomes **125 ms under ideal eight-way scaling**, before serial work, synchronization, or preparation. Coarsening must reduce the work as well. Moreover, [`Field::sweep`](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-space/src/light.rs:237) is currently serial. Directions and work within a layer offer parallelism, but layers and growth cycles retain dependencies.

2. **P1’s 0.7–12 seconds of table preparation remains part of a retune whenever changed parameters invalidate those tables.** Sharing tables across trees does not amortize their rebuild across successive parameter edits. A coupling-only timing gate can pass while the harness still stalls for seconds.

The gate must measure parameter change to a usable updated representation, including invalidated preparation, coupling, necessary geometry, and transfer. Separate cold initialization and repeated edits, but repeated edits must include growth parameters that invalidate the expensive tables.

The strongest alternative is **the same coarse physiological architecture with compact online cohort updates instead of rebuilding complete subtree tables**. Keep explicit chains where spatial arrangement matters; advance aggregate state elsewhere; resolve visible geometry lazily. First determine which table outputs the first usable view actually consumes, then avoid computing the rest up front.

If that still misses, investigate fewer coarse physiological time steps with error-controlled handling of sensitive events, and parallel CPU/GPU execution of the same equations. Those are further measured approximations, not guaranteed speedups. There is currently no evidence supporting an unconditional 100 ms promise.

**5. Is the continuity band sound?**

**A complementary blend can be sound. “Weight its couplings” is not a sufficient specification.**

For each aggregate contribution, the intended rule should resemble

\[
A=(1-w)A_{\rm standin}+wA_{\rm expanded}.
\]

That requires:

- Both representations to use consistent historical inputs throughout the band.
- Complementary accounting for foliage, demand, upkeep, section, and load, so nothing disappears or enters twice.
- Smooth transition of the inputs the branch itself reads, including light and allocation, as well as what it contributes to its parent.
- Hierarchical accounting that does not retain a whole-subtree proxy while independently adding its descendants.

Blend extensive inputs before nonlinear allocation and hazard evaluation. Blending only outgoing foliage while switching the branch’s own light history can still change its survival abruptly.

The extent predictor must itself be continuous. The existing [`reach_bound`](/home/daniel/Projects/telperion/.worktrees/ondemand/crates/telperion-space/src/closed_form/kept/bound.rs:26) uses discrete quantiles and possible-path inclusion; it is not automatically suitable as a smooth classification coordinate. Keep the conservative resolution bound distinct from the smooth coupling predictor.

Walk tests should cross both band endpoints, table interpolation knots, and the zero-light-response boundary. A smooth weight also says nothing about computational cost: every positive-weight expanded branch may still need its history computed.

**Required changes for acceptance:**

1. Replace the general constant-light stand-in with a specified history-dependent aggregate update.
2. Define the boundary records, allocation moments, inherited survival, retained-pipe accounting, and immutable geometry interface.
3. Specify complementary continuity blending for both contributions and consumed inputs.
4. Correct the fidelity claim and restore the per-PA allocation-drift check.
5. Gate **end-to-end retuning**, including invalidated tables, rather than the coupling pass alone.

I would proceed with a bounded probe of that revised architecture. I would not build the current constant-light table design further on the assumption that P4’s coupling-only timing and light-error bars establish the product requirement.