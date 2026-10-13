Astra’s position: **I support continuable growth, but the sketch misses a dependency that must be resolved before the GPU work.** The current growth law reads the requested final age through `presence::Windows`, which scales draw windows by expected remaining wood. The cycle uses those windows in shedding. Changing retirement alone cannot establish continuation equality. This is a code finding; I ran no new measurements. [Windows](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/presence.rs:47), [cycle consumer](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/grow/cycle/passes.rs:56)

1. **The API is right if settling produces an independent snapshot.**

   I prefer `begin(params, seed)`, `grow_to(&mut state, age)`, and `settle(&state) -> Tree`. Absolute age avoids ambiguity and makes repeated requests harmless. Growth should complete asynchronously; the previous drawable remains valid until its replacement is ready.

   `GrowthState` holds immutable parameters and seed, current cycle, stable lineage identities, living apexes, sleeping and woken buds, growth records, balance memory, incremental layout, retirement summaries, and history still required by girth. It also needs an explicit boundary indicating whether the last cycle’s layout and light are complete. Both executors currently omit those phases at the requested final age. [CPU boundary](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/grow/cycle.rs:81)

   Reduction scratch, scans, dispatch plans and reconstructible light buffers can be released between calls. Their reconstruction costs must be measured. Settling must not overwrite growth state; today the CPU settle consumes and modifies it.

   The harness can use a consuming convenience wrapper. The site should release state after producing its field unless continuation is requested. Games retain state only for trees that will grow again. Parameters stay fixed initially; changing environmental inputs requires an explicit history contract.

2. **Retire only what has become permanently dispensable at the completed current age.**

   My rule requires a closed subtree with no living or potentially waking bud, final waking shares, and zero fade at the current age. Preserve its living summary, frozen upkeep, and all girth inputs still read later. Prove that its fade cannot become positive again under fixed parameters.

   Closed subtrees still fading must remain eligible for later retirement. Today `Fate::Kept` prevents reevaluation, so replacing final age with current age would leave them permanently retained. Use an expiry schedule or bounded pending set. [Retirement](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/grow/cycle/retire.rs:122)

   Sleeping buds beyond the requested age are currently discarded. Continuation must retain their future events, including the information needed to decide whether their bearer survives. [Dormancy](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/dormant.rs:63)

   Therefore continuation retains more history than today’s future-aware retirement. The increase is unmeasured.

   Byte equality is a sound requirement **within each executor**, after making the growth law independent of call boundaries. Keyed draws alone are insufficient. Reduction grouping, record order and compaction must depend on logical identities, not capacities, chunk boundaries or arena addresses. Test equality with an intermediate settle included. CPU/device agreement remains the council’s measured-tolerance contract.

   The final-age-dependent windows need an explicit causal replacement and fidelity assessment. Merely extending their tables during continuation would leave earlier years grown under different windows.

3. **Reduce memory before promising that continuation reduces the peak.**

   My order is:

   - Separate persistent state, historical settle inputs and transient scratch; delete unused columns and share storage where lifetimes demonstrably do not overlap.
   - Remove global worst-ratio over-allocation using parameter-derived estimates and observed counts. Preset measurements calibrate the allocator; species names must not select allocation behavior.
   - Compact active axes for every parameter set, moving necessary historical metadata into a smaller archive. Today records prevent axis compaction; removing that guard without replacing their axis-dependent readers is incorrect. [Compaction](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-render/src/generation/cycle/compact.rs:208)
   - Add continuation capacity growth, charging old/new allocation overlap and retaining completion-aware accounting.

   Proposed investigation targets, in decimal MB at shipped ages, are Telperion ≤2,000, Laurelin ≤2,000, spruce ≤1,200, oak ≤650, birch ≤450, beech ≤250, ordinary ≤250 and palm ≤32. These are **unmeasured targets**, not feasibility claims; 1,500 MB for Telperion is a stretch.

   Report standalone and continuation peaks separately. Continuation overlaps persistent state, an old drawable, and growth or snapshot work; snapshot publication can temporarily hold both drawables. Today growth scratch is destroyed before settling. Smaller chunks alone do not remove these costs. [Scratch lifetime](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-render/src/generation/cycle/settle_scratch.rs:12)

4. **Fail early in this order.**

   - Audit terminal-age reads and run a tiny CPU prefix comparison with different requested horizons. Stop the API port if earlier growth differs; resolve the causal law first.
   - Test CPU splits around waking, shedding and PA transitions, including zero-year advances and repeated settling. Stop on any same-executor mismatch or snapshot mutation.
   - Measure current-age retirement’s live set and irreducible archive. Stop pursuing the proposed memory targets if their floor already exceeds them.
   - Apply each memory lever independently, checking outputs and taking a labelled census. Drop a lever whose measured saving does not justify its cost.
   - Repeat split tests on small GPU trees, forcing compaction and capacity moves. Stop on mismatch, invalid references or unsafe capacity exhaustion.
   - Measure mature presets, seeds and old-drawable overlap, then Chrome binding limits, frame responsiveness and the site’s complete field query. Refusal must preserve the previous drawable.

5. **The spec I would sign orders requirements accordingly.**

   First establish causal yearly growth and accepted fidelity; then exact continuation per executor, immutable snapshots, current-age retirement, guarded capacity growth, measured per-preset memory and latency, and integration through the existing pipeline. Preserve supported age limits and named errors.

   Growth reuse saves already-completed years. Frequent settling adds work, and retained state adds memory. Their actual overhead must be measured before the release claims either benefit.