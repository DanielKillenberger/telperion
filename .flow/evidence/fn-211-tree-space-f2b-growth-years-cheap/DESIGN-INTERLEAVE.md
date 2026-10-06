# Light-reading species: fine roots grown in step with the pass (host design, 2026-10-06)

## Why

TASK4-LIGHT.md: with today's exact field, fine buds (below fn-209's cut, N = 1,024) reading one value per root (oracle C) keep the oak within 1.5 % per order and 1 % per PA, and its stills are indistinguishable from today's (host viewed both seeds at 80, bare and in leaf). So one read per fine root is botanically enough. What fails is producing the pass's couplings without growing the fine subtrees: the aggregate state's light read (p50 12-17 %, p95 40-57 % off at fine roots) and its vigour split (whole share: trunk +213 to +507 %; share per light: fine PAs 25-52 % short). Each fix needs a further model (within-subtree vigour moments, outer-point reads), and each is an approximation of something that could be computed.

## The design

**Question, delete, then optimise.** The aggregate state exists only because fine roots were to be grown after the pass from a record. Delete it: fine roots are grown by fn-209's GPU executor **in step with the pass, cycle by cycle.**

Each cycle *c*:
1. CPU: the pass's balance and allot over the coupling structure, reading each fine root's per-cycle totals from *c - 1* (vigour demand Q, leaf light collected, wood upkeep, the PA sums Σp and Σp(v/p)^ψ of decision 23).
2. GPU: each fine root grows its cycle *c* units (fn-209's chain grower, stepped one cycle), lays them roughly (today's sketch rule), deposits its leaves into the light lattice, and reads each of its buds' light at the bud's own position (today's rule, exact); then reduces its per-root totals for *c*.
3. Exchange per cycle: the lattice (CPU's coupling leaves plus GPU's fine leaves, summed in a fixed order), and per root a handful of scalars (about 4.4k roots × ~8 values for the oak at 80). No per-node data crosses (Astra change 3 holds).

The re-lay every 10 cycles lays the coupling structure on the CPU and the fine roots on the GPU from their bearers' frames, as fn-209's executor already lays them.

**What it gives.** The oak equals today's oak up to fn-209's fixed-point contract: every coupling is the real one, not an aggregate. DESIGN-LIGHT.md's aggregate state (Astra change 1), the vigour-split question and the light-read question are all removed. Where a species reads no light, the per-cycle exchange is empty and the executor runs its cycles without stopping: the same steps, nothing to exchange (no second way of building).

**What it costs (to measure).** 80 cycle handovers with a lattice sweep each; the pass itself measured 0.24 s for oak 80 seed 1 (load 6-8); the GPU grew a whole beech's fine roots over all cycles in 82-114 ms warm. The sweep with fine leaves is today's (oak 80 seed 1: 287 ms serial before F2b). Estimate, not measured: well under today's 3.3 s.

## Questions for review

1. Is stepping the GPU executor per cycle, with a per-root scalar exchange, sound, and does it keep one pipeline?
2. Is the fixed-order lattice sum across CPU and GPU deposits exact enough for the fixed-point contract (light is continuous; decisions read it through fn-209's fixed-point thresholds)?
3. What does it lose against today's growth, if anything?
4. Where will the time go, and what should the probe measure first?

## After Astra's review (2026-10-06, ASTRA-REVIEW-INTERLEAVE.md): accept with changes, all taken

1. **A phased dependency contract per cycle** replaces the three-step sketch: (i) gather previous-state light, upkeep and living status, and evaluate balance throughout the tree, fine laterals and inherited ancestor survival included; (ii) reduce the surviving demand and distribute incoming vigour to each root; (iii) split it inside each root, gather current-cycle per-PA moments, distribute the global normalizers before any growth; (iv) grow and wake, sketch, any scheduled re-lay, deposit every current leaf, sweep, read light for the next cycle. Fusing phases only after equivalence is shown. The per-root interface is specified by phase, not as "about eight scalars".
2. **Canonical arithmetic extends through the light and balance feedback:** decision-relevant positions and deposit weights, lattice accumulation (integer), sweep and sampling, shade survival and the balance hazard, allocation and presences that feed geometry and light; rounding, grouping, overflow and zero light specified. The CPU reference adopts it (AGENTS.md "Generator evolution"), its difference from today's trees measured apart from CPU/GPU agreement.
3. **Host choice: realized mechanics, for every species.** Each fine root reports its realized totals to its bearer (section and retained pipes, mass and moment, survival and termination, leaf-girth contributions); the CPU settles exactly as today's engine; fine roots are laid from their bearers' frames. Option 4's committed boundaries leave production (its trunk and droop errors with them); every species equals today's engine up to the canonical arithmetic, by one model with no species switch.
4. **Fine physiology becomes eager for light-reading species** (grown each cycle, off-screen and later-shed growth included); final surfacing stays demand-driven. The cut stays a work placement; continuity is tested approaching zero light response.
5. **No equality or speed claim before the probes:** (a) correctness, a partitioned CPU reference against the whole-tree reference compared after every phase, varied over cut, order and chunking (bit-identical expected, the same arithmetic partitioned); (b) synchronization, persistent buffers on the real exchange schedule over 80 cycles, round trips counted and timed apart from useful work; then the oak at 40 and 80, both seeds, memory, overflow recovery (no duplicated deposits) and re-lay spikes. GPU-GEOMETRY.md lives on fn-209's branch (fine worktree); the reviewer's checkout lacked it.
