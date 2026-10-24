I recommend **one cycle-based grower, executed entirely on the GPU or entirely on the CPU, with float arithmetic, explicit resource exchange, and a resolution-aware representation of fine growth**. Light should read a frozen description of the previous cycle’s foliage inside that grower.

Two parts remain research questions: whether a local analytic light description preserves fidelity cheaply enough, and whether aggregated fine growth can bring cold, single-thread Wasm below 0.1.4. Neither is established by the existing experiments.

I read the requested evidence and inspected the arithmetic experiment’s contact sheet. This is a read-only proposal; I changed no files and ran no new benchmarks.

**1. Make the cycle the shared algorithm.**

The current split is structural. [`Species::reads_light()`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/species.rs:423) selects the lit coarse grower at [`fine/coarse.rs:83`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/fine/coarse.rs:83), while [`grow::apart()`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/grow.rs:381) excludes light-reading trees from independent GPU roots. The GPU currently grows an entire chain alone in one invocation, which cannot support general inter-branch feedback without changing its scheduling contract. See [`chain.wgsl:1`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-render/src/generation/fine/chain.wgsl:1).

Replace that split with one state transition:

| Work | State and operation |
|---|---|
| Per living bud | Lineage, PA, integer cycle, fractional developmental age, presence, tip frame, incoming vigour, carbon memory and dormant/relay state. |
| Start of cycle | Read light at the bud’s tip from the previous cycle’s immutable foliage description. |
| Bottom-up | Sum light capture, upkeep and living presence through the botanical hierarchy; determine shedding inputs. |
| Top-down | Propagate shedding and split incoming vigour using the existing apical-control law. |
| Per PA | Reduce the actual presence-weighted size terms and compute normalization. |
| Per bud | Apply survival, abortion, continuation, dormancy and branching rules; emit this cycle’s growth unit and next-cycle buds. |
| End of cycle | Advance rough geometry; update foliage, mass and living summaries for the next cycle. |
| Final settle | Solve retained pipes, loads and bending; produce only the requested representation. |

This preserves the substance of [`grow/stepped.rs:15`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/grow/stepped.rs:15), while removing the CPU/coarse-versus-GPU/fine division.

Store buds and axes in compact arrays with stable parent/child ranges. Schedule GPU work in bounded blocks, with dispatch boundaries for dependencies between blocks. Keep queues, totals and indirect dispatch arguments resident. CPU execution traverses the same phase graph in ordinary loops.

Choose the executor explicitly when creating the build context. A site without a GPU requests the CPU executor. GPU capacity failure returns an error; it does not rebuild that species on the CPU.

Zero light-response coefficients produce neutral terms in these equations. They never select another grower. Dead arithmetic may be eliminated after equivalence is demonstrated, but execution semantics remain the same at zero and arbitrarily close to zero.

**2. Prototype local analytic foliage volumes, while preserving allocation.**

My preferred light candidate is a **hierarchy of local foliage volumes derived from grown structure**, updated each cycle. A single crown outline loses too much spatial information.

Each summary carries leaf area, centroid and spatial second moments. Begin with compact ellipsoidal density kernels around shoot groups. In normalized coordinates, a kernel proportional to `max(0, 1 − r²)` has a polynomial ray integral after finding its entry and exit points. This avoids lattice deposits, directional sweeps and numerical ray marching.

A bud evaluates the existing nine sky directions, accumulating optical depth through the hierarchy:

`light(p) = Σ direction_weight × exp(−extinction × optical_depth(p, direction))`

The direction set already exists at [`light.rs:23`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/light.rs:23). Keep it initially so the experiment changes the spatial approximation alone.

Concrete execution rules:

- Build leaf-area moments while emitting growth; combine them in structural summary reductions.
- Double-buffer summaries. Every bud in a cycle reads the same completed snapshot.
- Resolve nearby foliage in finer groups; use aggregate volumes farther away. Blend between parent and child approximations across a refinement interval to avoid parameter-walk jumps.
- Never count both a parent’s complete density and its children’s complete density.
- Preserve leaf-area weighting by presence. A newly appearing branch contributes from zero.
- Share the same light evaluator between GPU, CPU and coarse aggregate records.

This is an engineering hypothesis. A botanical hierarchy can be a poor spatial index, especially for interleaved branches. Measure visited summaries per bud, including the worst tail. If faithful reads approach all-pairs work, reject this candidate before building the full executor around it.

**“No extra passes” needs a precise boundary.** Inline light queries need no separate sampling pass. Moment updates may fuse with existing structural reductions, but that fusion must be demonstrated in the dispatch graph. If they require an additional reduction, that is an explicit miss against the strict requirement.

Whole-tree allocation still requires synchronization. WGSL barriers synchronize a workgroup, so an arbitrarily large coupled tree cannot safely complete all cycles in one dispatch. [WGSL synchronization scope](https://www.w3.org/TR/WGSL/#scoped-operations). I would target **no light-only deposit, sweep or sampling dispatches**, while retaining the grower’s necessary reductions.

The existing lattice remains a comparison oracle during development. Its production implementation explicitly deposits then sweeps at [`light/field.rs:35`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/light/field.rs:35); the partially GPU-backed feedback still builds and sweeps the shared lattice on the CPU at [`stepped/feedback.rs:142`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/grow/stepped/feedback.rs:142). That architecture should disappear once its replacement passes.

Crucially, **retain actual per-PA allocation normalization**:

`size_i = a_i × Σpresence / Σ(presence × a)`, where `a_i = (vigour_i / presence_i)^psi`.

[`allocation.rs:153`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/allocation.rs:153) documents the important consequence: a group containing one bud remains whole. fn-226 replaced this denominator with mean crown transmission raised to `psi`. Its report attributes the stem shrinkage to that substitution, although it was not traced bud by bud. See [`R3-R4.md:105`](/home/daniel/Projects/telperion/.worktrees/f4/.flow/evidence/fn-226-light-read-from-the-crowns-outline-per/R3-R4.md:105).

Therefore test light approximation and allocation changes independently. The failed experiment does not isolate analytic light as the cause.

**3. Keep identity exact; make numerical agreement tolerant.**

Use `f32` for GPU growth state and the production CPU reference’s corresponding operations. Retain ordinary `f64` where useful for CPU preparation and diagnostic calculations. Remove Q32/Q62 arithmetic and wide products from biological quantities.

The deletion targets are concrete: [`fixed.rs:472`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/fixed.rs:472), [`allocation.rs:40`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/allocation.rs:40), and the mirrored machinery beginning at [`contract.wgsl:1`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-render/src/generation/fine/contract.wgsl:1). Question the pinned cross-platform transcendental wrapper too; its stated purpose is preserving last-bit agreement, at [`transcendental.rs:1`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/transcendental.rs:1).

Retain:

- Integer lineage hashing and the key registry. GPU emulation needed for identity is much smaller than emulating every biological operation. See [`lineage/keys.rs:1`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/lineage/keys.rs:1).
- Integer cycle and event indices. Do not accumulate time by repeatedly adding small floats to a large age.
- Stable sibling order, deterministic prefix allocation and fixed reduction trees.
- Presence windows and fractional births. Removing exact arithmetic does not authorize hard topology thresholds.

For draws, derive a documented, exactly representable uniform value from the lineage hash, handle probability endpoints explicitly, and compare it with the float probability. Record this as a generator revision change.

“Same machine” should mean **same generator, executor, compiled configuration and driver/runtime**. Avoid floating-point atomic accumulation and allocation-order-dependent identities. WGSL permits reassociation and fusion, so hardware identity alone cannot promise unchanged results after compiler or driver updates. [WGSL floating-point evaluation](https://www.w3.org/TR/WGSL/#reassociation-and-fusion).

Replace universal CPU/GPU byte parity with four layers:

1. **Exact repeatability within an executor.** Repeated builds, fresh contexts, reused buffers and varied scheduling produce the same canonical tree.
2. **Exact discrete contracts.** Hashes, key derivation, parent links, event ordering and endpoint cases remain directly testable.
3. **Numerical invariants.** Conserved vigour, normalized PA means, bounded light, finite geometry, nonnegative radii, attachment/contact correctness and budget enforcement.
4. **Cross-executor tree agreement.** Compare by lineage, measuring position/radius error, unmatched wood weighted by length and radius, crown dimensions, leaf area and major-branch survival. Follow with whole/mid/close stills and motion inspection.

The [EXACT experiment](/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/exact-out/EXACT.md:1) supports this direction. Its sheet shows no obvious structural difference at the displayed scale. It does **not** bound accumulated float error, and it does not cover every shipped preset. Set numerical tolerances from actual float CPU/GPU runs before accepting the migration; do not turn its perturbation magnitudes into an assumed error bound.

**4. Delete the execution cut; retain representation approximation where a consumer needs it.**

The existing fine-root cut serves two different purposes. [`fine.rs:6`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/fine.rs:6) partitions execution, while the coarse field substitutes expectations for unresolved subtrees.

For the GPU, delete the biological CPU/GPU cut. Every axis belongs to the device grower. Work blocks are scheduling units, and changing their size must not change the tree. This also removes the need to predict expected kept phytomers merely to decide where computation runs.

That makes the standalone [`closed_form/predictor.rs:1`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/closed_form/predictor.rs:1) a deletion candidate. Replace its capacity role with checked counts and bounded allocation. An expectation must never be treated as a hard allocation bound.

For the site, retain a **coarse representation**, because materializing millions of fine organs to answer 46,656 cells defeats the consumer’s needs. However, replace the fixed “1,024 expected phytomers” criterion with a requested spatial error or cell footprint.

An unresolved record should carry:

- Its attachment and lineage.
- Living presence and developmental cohorts.
- Leaf area, wood mass and first/second spatial moments.
- Pipe contribution, load and carbon state.
- The information needed to split into finer records.

It participates in the same cycle’s light, balance and allocation phases. Coarse growth must not reintroduce `lit()` versus `settle_with()`.

I would **retain existing closed-form rows temporarily as the tested aggregate implementation**, then replace their full-history recursion with incremental cohort/moment updates derived from the same growth-unit rules. This replacement is the largest mathematical uncertainty: nonlinear survival and allocation do not generally commute with taking expectations. One average bud is insufficient.

Demand refinement convergence against fully expanded trees. If an aggregate cannot meet its error budget, refine it through the same grower. If the request exceeds resource limits, return a named error.

Also preserve the distinction between necessary unbent and bent geometry. [`fine/lay.rs:58`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/fine/lay.rs:58) uses unbent frames before loads and levers determine bent placement. Delete empty placeholder work; do not casually delete the dependency.

**5. The site target is plausible, but only with cold-work reduction.**

The supplied [profile](/home/daniel/Projects/telperion/.worktrees/f4/.flow/evidence/fn-245-a-breakthrough-in-generation-time/PROFILE.md:23) leaves approximately these grow budgets at 36³:

| Species | Approximate end-to-end target | Existing query, grid and voxelization | Remaining grow budget |
|---|---:|---:|---:|
| Oak | 300 ms | 90–115 ms | 185–210 ms |
| Birch | 350 ms | 75–140 ms | 210–275 ms |
| Spruce | 240 ms | 40–45 ms | 195–200 ms |

These are planning budgets derived from historical measurements, not forecasts.

The credible route is:

- Remove wide arithmetic from layout and allocation.
- Eliminate cold construction of the full kept-history tables.
- Stop making fine-root placeholders into ordinary layout axes.
- Aggregate growth below the requested field resolution while retaining its resource effects.
- Keep the field query initially; its profile is already competitive.
- Reuse a Wasm instance and bounded immutable preparation, but measure the first tree separately.

Caching alone cannot meet the cold target. This checkout already calls `Rows::shared_laterals()` at [`fine/coarse.rs:82`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-space/src/fine/coarse.rs:82); that is newer than the profile’s “recomputed every grow” finding. It also already uses bearer-only placement when bending. Remeasure before claiming those savings.

The strongest deletion evidence is [`PROFILE.md:93`](/home/daniel/Projects/telperion/.worktrees/f4/.flow/evidence/fn-245-a-breakthrough-in-generation-time/PROFILE.md:93): every measured internode is shorter than a cell, and 88–97% of converted nodes fall below the site’s wood cutoff. Their **geometry** is often unnecessary; their light, mass and allocation effects still matter.

Do not determine aggregation from bounds that are only available after full growth. Accept world-space cell size, or perform a cheap conservative planning step followed by bounded refinement. Benchmark that preparation as part of the field call.

My confidence is highest for spruce, then birch. Oak needs both cheaper feedback and fewer aggregate/root operations. Ordinary’s 11.2-second field call is a separate mandatory workload, not an outlier to exclude.

**6. Generation speed alone cannot satisfy the release bar.**

The [scorecard](/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/parity/SCORECARD.md:13) shows memory regression everywhere and frame-rate regression for Ordinary and both Two Trees. It also compares different default ages and treats up to 10% regression as “new ≥ old.” Neither convention should silently become the owner’s new acceptance rule.

Float conversion will reduce some intermediate state, but cannot by itself explain away final tree-buffer growth such as palm’s 1→106 MB or spruce’s roughly 400→1,450 MB.

Measure buffer lifetimes and consumers. Retire growth scratch after preparation, bound retained pools, remove duplicate laid/packed representations, and emit detail according to a measured geometric error. [`roots.rs:119`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-render/src/generation/fine/roots.rs:119) retains multiple per-run buffers; [`pool.rs:43`](/home/daniel/Projects/telperion/.worktrees/f4/crates/telperion-render/src/generation/fine/pool.rs:43) intentionally retains another build’s scratch. These are inspection targets, not proof that all those bytes are waste.

I would use six specs, in this dependency order. Numerical targets below are proposed gates, not existing measurements or owner approvals.

| Spec | Depends on | Measurable exit |
|---|---|---|
| **1. Reproducible growth with ordinary floats** | — | Refresh baselines; remove biological wide arithmetic on both executors; pass repeatability, invariants, actual cross-device comparison and visual checks. Demonstrate lower cold shader compilation and Wasm stage time. |
| **2. One light-responsive growth law** | 1 | CPU prototype runs every preset through one cycle graph. Compare local-volume light with the lattice while holding allocation fixed. Pass zero-crossing/inter-species walks and mature-tree checks. Report light error, visited volumes and dispatch-fusion feasibility; reject the candidate if fidelity or cost fails. |
| **3. The complete grower resides on the GPU** | 2 | No species-dependent executor selection or per-cycle CPU botanical work. CPU/GPU agreement passes. All-species build timings include preparation, transfers and device completion. Scheduling changes preserve the canonical tree. |
| **4. The field computes only resolvable detail** | 2; integrate after 3 | Cold single-thread 36³ calls meet roughly 300/350/240 ms for oak/birch/spruce, and every supported species meets its baseline. Verify 24³ and 48³, aggregate refinement convergence and field fidelity. Cold table construction is included. |
| **5. A prepared tree meets memory and frame budgets** | 3 | Every preset meets master’s resident-memory and frame-time bar at whole/mid/close views, with scratch and pool bytes reported separately. Geometry reduction passes still and motion checks. |
| **6. Release qualification across all consumers** | 4, 5 | Every species passes build, frame, memory and site comparisons; distinguish default-age product comparisons from matched-age diagnostics. Any slight regression names the measured loss and owner-accepted fidelity gain. Remove obsolete growers and arithmetic from shipped artifacts. |

The biggest risks are **light approximation bias amplified over cycles, inaccurate coarse feedback, GPU synchronization overhead, and detail growth overwhelming memory/frame gains**. Test those early. A successful float migration would be useful progress, but it would not establish the breakthrough or justify release on its own.