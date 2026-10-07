**Verdict: accept with changes.** Growing fine subtrees alongside the pass is a sound alternative to approximating their history. It can replace the history-dependent aggregate required by the earlier review. However, the proposed cycle exchange is incomplete, and neither “equals today’s oak” nor “well under 3.3 s” is established.

I reviewed the available evidence and code without modifying files or running benchmarks. `GPU-GEOMETRY.md` is absent from this checkout. I have not independently judged the reported stills.

**1. Is cycle-stepped GPU growth with per-root scalar exchange sound, and does it keep one pipeline?**

Yes, provided both executors implement the same phased algorithm. A CPU/GPU boundary does not require a second pipeline. The cut must remain a work-placement choice, and the CPU reference must support the same interleaved execution.

The draft’s single exchange per cycle is insufficient. [Balance]( /home/daniel/Projects/telperion/.worktrees/f2b/crates/telperion-space/src/grow/balance.rs:26) removes apexes and changes their inherited survival **before** [allocation](/home/daniel/Projects/telperion/.worktrees/f2b/crates/telperion-space/src/grow/sketch.rs:156). Allocation then computes the current incoming vigour and current PA normalizers. Last cycle’s completed moments cannot simply supply these.

The contract needs these dependencies:

1. Gather previous-state light, upkeep and living status; evaluate balance throughout the tree, including internal fine laterals and inherited ancestor survival.
2. Reduce the surviving population’s allocation demand and distribute incoming vigour to each root.
3. Split that vigour internally, gather **current-cycle, per-PA** moments, and distribute the global normalizers before any growth.
4. Grow and wake, sketch, perform any scheduled re-lay, then deposit all current-cycle leaves, sweep the combined field, and read light for the next cycle.

These are logical phases; an implementation may fuse exchanges after proving equivalence. For example, normalized internal allocation coefficients might reduce a round trip. That requires a specified derivation, not reuse of stale totals.

Consequently, “about eight scalars per root” is premature. A root can contain several PAs. Its interface also needs survival/termination, incoming vigour, mechanical contributions and attachment updates.

The existing GPU executor is also not merely missing a pause button. [Its chain kernel](/home/daniel/Projects/telperion/.worktrees/f2b/crates/telperion-render/src/generation/fine/chain.wgsl:283) runs chains through their remaining cycles; the host expands descendants in waves. Interleaving requires persistent live and sleeping state, internal balance/allocation, resumable growth and device-side work scheduling.

**2. Is a fixed-order lattice sum exact enough for the fixed-point contract?**

**No, by itself.** Fixed order addresses one source of variation. It does not make CPU/GPU inputs or arithmetic identical.

Today’s field uses floating-point positions, trilinear deposits, interpolated optical depths and exponentials. Separately summing CPU and GPU deposits also changes grouping when the cut moves. A reproducible result at one cut is not necessarily invariant across cuts.

More importantly, [FINE-CONTRACT.md explicitly excludes light-scaled viability](/home/daniel/Projects/telperion/.worktrees/f2b/.flow/evidence/fn-209-tree-space-f1-trees-resolved-on-demand/FINE-CONTRACT.md:224). [Growth](/home/daniel/Projects/telperion/.worktrees/f2b/crates/telperion-space/src/grow/advance.rs:50) evaluates `viability^(light^-phi)` in floating point before constructing its threshold. Balance shedding likewise computes a floating-point hazard. Continuous light therefore feeds discrete decisions.

Extend the numeric contract through the complete feedback path:

- decision-relevant positions and deposition weights;
- density accumulation, sweep and sampling;
- shade survival and balance hazard;
- allocation and presence values that affect subsequent geometry and light.

Specify rounding, reduction grouping, overflow and zero-light behavior. Integer accumulation can make deposit sums order-independent, but it does not settle the other stages. Quantizing the final result of differing floating-point calculations also cannot guarantee identical thresholds.

The CPU reference can adopt a new canonical procedure under the generator-evolution policy. Its difference from today’s tree then needs measurement separately from CPU/GPU agreement.

**3. What does it lose against today’s growth?**

A complete implementation need lose no physiological information. The actual fine subtree is itself a history-dependent state, so replacing the approximate aggregate is legitimate.

The draft nevertheless mixes two different references:

- Today’s whole-tree growth uses realized descendants for shedding, girth and mechanics.
- Fn-209’s fine executor uses committed descendant extinction, section, mass and moment, with geometry fitted to committed boundaries.

[The fine executor documents that distinction](/home/daniel/Projects/telperion/.worktrees/f2b/crates/telperion-space/src/fine/executor.rs:1). Stepping it annually does not remove it.

To claim equality with today’s oak, periodic and final layout must receive the appropriate realized fine-subtree contributions. The interface must cover live and retained pipes, the global leaf-girth normalization, loads and their moments, and attachment frames in the correct order. Today’s [re-lay](/home/daniel/Projects/telperion/.worktrees/f2b/crates/telperion-space/src/grow/relay.rs:25) includes settling, thickening, unbent placement, load gathering and sagged placement. “Lay from the bearer’s frame” describes only part of that dependency.

Alternatively, retain committed mechanics consistently on both sides of the cut and describe the result as that model. Then withdraw equality with today’s whole-tree oak and measure the botanical and geometric differences.

The architectural cost is **eager fine physiology and temporary layout for light-reading trees**, including off-screen branches and growth later shed. Final surfacing can remain demand-driven. This work has a consumer, the light field, but it sacrifices the earlier scheme’s ability to postpone fine growth.

The no-light shortcut is valid only where these dependencies disappear under the same rules. Test the approach to zero light response so skipping work does not introduce a change in shedding or mechanics.

Finally, the opening claim that one read is “botanically enough” is too broad. TASK4-LIGHT reports good growth totals, but also a 6.7% thinner trunk and a 27% median increase in order-1 base radius for one case. Those results support a tested approximation at the reported views, not general equivalence.

**4. Where will time go, and what should the probe measure first?**

The likely costs are synchronization, persistent-state traffic, internal balance/allocation reductions, rough layout, periodic whole-tree mechanics, and deterministic lattice construction. The beech’s 82–114 ms measures growth and shedding without this light-feedback workload. It cannot price the proposed oak executor.

Start with two small probes:

- **Correctness:** a partitioned CPU reference versus the whole-tree reference, compared after every phase. Include internal balance shedding, mixed PAs, fractional lifespans, waking/relays, retained pipes, shade/release histories and a loaded re-lay. Vary cut, ordering and chunking.
- **Synchronization:** persistent CPU/GPU buffers running the actual proposed exchange schedule across 80 cycles. Measure submission, waiting, mapping, transfers and lattice sizing separately from useful computation. Count real round trips rather than assuming 80 handovers.

Then measure representative oak runs at 40 and 80, both seeds, including historical work, peak resident memory, overflow recovery and re-lay spikes. Overflow retries must not duplicate deposits or contributions already committed to a cycle.

Keep the two timing boundaries explicit. F1’s device-resident botanical tree and geometry is an intermediate result; the product gate remains parameter edit to a usable rendered tree, including invalidated preparation and downstream conversion. Host decision 5 permits the intermediate boundary, not an end-to-end speed claim.

**Required changes for acceptance:**

1. Replace the three-step cycle sketch with a phased dependency contract using current post-balance demand and current per-PA allocation moments.
2. Extend canonical arithmetic to light and balance feedback; fixed-order deposits alone do not satisfy it.
3. Choose and specify realized versus committed shedding/mechanics, including retained pipes, global normalization and attachment exchange.
4. State that light-reading fine physiology becomes eager, while preserving cut independence and continuity at zero response.
5. Replace the equality and speed assertions with the correctness and timing probes above; correct the claim that GPU layout already exists in the reviewed evidence.

I would proceed with that revised probe. I would not continue refining the aggregate model merely to preserve the earlier architecture.