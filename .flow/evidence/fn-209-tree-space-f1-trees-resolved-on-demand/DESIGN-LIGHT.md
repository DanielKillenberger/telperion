# F1: species whose growth reads light (host design, 2026-10-06)

Answers P2's five open questions (probe P2 report; branch fn-209-probe-alone, d47e0eb7). Owner goals that bind it: realtime, never baked offline (memory `trees-generate-in-realtime-with-screen-2026-10-05`); retuning in the harness near-instant (owner, 2026-10-06; STRATEGY.md, Build track); one pipeline, every parameter by degree (`docs/principles.md`).

## The finding behind this design

P2 built my brief's light field from committed data only: every unresolved bud's expected leaf area spread over its extent bound. Bud light came out 10-25 % off at the median and 30-60 % at p95. PROPOSAL.md's coupling tree (section 1, decision 7) measured one light read per fine subtree at 5-9 % p95. My P2 brief dropped the coupling tree; that was a regression, and this design returns to it.

## The design

**1. Two scales, one rule.** A bud whose expected subtree extent at the tree's age exceeds the light lattice's cell is a *coupling bud*; every other bud roots a *fine subtree*. The cell is the existing engine constant (`light.rs:296`, 0.5 m), named in the request, not the species. Where a species reads no light, nothing couples: every chain is grown alone (P2's GO for beech, spruce and palm), and the coupling pass has no per-cycle work.

**2. The coupling pass is the time axis (P2 Q1).** It is today's per-cycle loop (grow, sketch, shade, allot, balance, re-lay every 10 cycles) run over the coupling chains only, with each fine subtree present as its stand-in: expected leaf area, upkeep, vigour demand and section under the light it reads. The grid is rebuilt each cycle from coupling chains and stand-ins, as today. It never reads a resolved fine chain, so it is identical whatever the camera resolved.

**3. No circularity (P2 Q2).** Cycle *c* reads the light the state at *c - 1* casts, as the engine does now. A forward time step, never a fixed point.

**4. Allocation (P2 Q3).** Borchert-Honda and the per-PA size normalisation (decision 23) are exact among coupling chains. A fine root's share uses its stand-in's expected Q under its light; the PA means are taken in the coupling pass, so a fine subtree reads them as recorded scalars per cycle.

**5. Records.** For each fine root, per living cycle: its light, its vigour share, the PA means it reads (about 12 B per root per cycle; PROPOSAL.md measured under 35 MB for the oak). A fine subtree is then grown alone (P2's grower) reading its root's records as its history. Inside a fine subtree, light is its root's recorded value (decision 7's accepted approximation): it does not shade itself.

**6. The closed form under light (P2 Q4, decision 5).** A fine root's stand-in is the closed form under a constant light I, the light axis P2 built (five levels, log-interpolated) carrying the shade hazard. The balance hazard is the deep part: under a constant I, a subtree's light-to-upkeep balance is a function of its age and I alone, so it is a table axis like the shade hazard; the hard question is whether "constant I" is close enough for a fine root whose light changes as the crown grows. Measured, not assumed: step 2 below compares a fine root grown alone under its recorded history against its stand-in under its mean light.

**7. Leaf girth and retained pipes (P2 Q5).** The tree-wide leaf mean is a scalar of the coupling pass; a fine chain's phytomers read their root's recorded light.

## Cost against the near-instant goal

PROPOSAL.md estimated the coupling pass at about 1 s on one core for the oak (grid sweep 0.33 s, layouts 0.47 s, chain growth 0.1-0.25 s), with the coupling tree at 15-19 % of the oak's kept phytomers. That is not near-instant. Three levers, to be measured, not assumed:

- **Cores:** the sweep and each depth's lay parallelise today; 8 cores.
- **The cell:** P2 measured the field at 0.8-1.1 ms at 2 m, 3-5 ms at 1 m, 17-36 ms at 0.5 m. A coarser cell also shrinks the coupling tree, since fewer subtrees exceed it. The cell trades light fidelity for cost; the measurement decides its default.
- **Incremental retune:** the coupling pass's cycles before a parameter takes effect are unchanged, but almost every parameter acts from cycle 0, so this lever is weak. Not counted on.

If the oak's coupling pass cannot reach about 100 ms on 8 cores at a cell whose light error and stills hold, light-driven species are not near-instant under F1, and the host brings that to the owner before building further.

## Continuity (PROPOSAL.md Q3, open)

A bud whose expected extent crosses the cell flips between coupling chain and fine root: a step in a walk. Proposed: blend over a band in log extent, a bud in the band grown in the pass and its couplings weighted by its distance from the cell. The band width would be a new constant; it is measured on the walk test before it is accepted.

## Probe P4 (after Astra's review)

1. The coupling pass alone, oak seeds 1 and 7, ages 40 and 80, cells 0.5, 1 and 2 m: time on 1 and 8 cores, coupling share, memory.
2. Fidelity against today's oak: bud light per fine root (p50, p95), kept phytomers per order, trunk base and order-1 junction radius (option 4 geometry), stills bare and in leaf; and a fine root grown alone under its recorded history against its stand-in (the constant-I question).
3. Bar: coupling pass under 100 ms on 8 cores at some cell whose stills the host stands behind; kept phytomers within 10 % per order; bud light p95 under 10 %.

## After Astra's review (2026-10-06, ASTRA-REVIEW-LIGHT.md): accept with changes, all taken

The host accepts all five required changes. What they change:

1. **Fine subtrees carry a state, not a table lookup.** Section 6's constant-light stand-in is withdrawn for the general case. Each fine root carries a compact aggregate state advanced every cycle by the coupling pass under its actual inputs (light, vigour share, ancestor survival K): living cohorts by PA and development state, wood upkeep, the allocation moments Σp and Σp(v/p)^ψ by PA, balance memory (an explicitly approximated distribution), live and disused pipe contributions. Its transitions are derived from the engine's own growth rules and evaluated online from the current parameters. Constant-light tables stay only as an oracle for that recurrence, tested over many lineages and controlled light histories, including shade-then-release against release-then-shade.
2. **Boundary records are specified before code:** per fine root per cycle, its light, vigour share, inherited survival K(c) and growth termination; allocation moments; the retained-pipe transfer (lost section and its fade, never double-counted with live section); and the immutable geometry contract of option 4, on which this design depends explicitly. The step order is the engine's (`grow.rs:366`: balance, allocation, growth, wake; then sketch, re-lay, shade), not section 2's parenthesis.
3. **Continuity blends complementarily:** A = (1 - w)·A_stand-in + w·A_expanded for every contribution (foliage, demand, upkeep, section, load) and for every input the branch reads (light, allocation), blended before the nonlinear hazards; a smooth classification predictor distinct from the conservative extent bound; walk tests across both band ends, interpolation knots and the zero-light-response boundary.
4. **Claims corrected:** returning to the coupling tree is a hypothesis, not a recovered 5-9 %; that figure read realized foliage. Decision 4's per-PA growth drift under 2 % is restored as a bar.
5. **The gate is end to end:** parameter change to an updated usable tree in the harness, including every invalidated preparation (tables, coupling pass, records, the geometry of the view, transfer), for growth parameters that invalidate the expensive state, cold and repeated measured apart. No coupling-only timing claims the goal.

**Classification** is computed without the history: from a light-neutral predictor of extent, so which chains couple never depends on the pass it feeds; the final-age dependence of early history this implies is stated.

**What it means for P1.** The aggregate state is evaluated online per fine root, only where a tree needs it, so it may replace much of the per-species tables for light-reading species. P1's reformulation continues for the table work every species still needs (committed rows and extent for non-light species). The end-to-end gate counts both.
