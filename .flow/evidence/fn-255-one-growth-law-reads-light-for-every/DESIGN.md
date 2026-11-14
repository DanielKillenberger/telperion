# fn-255 design: the one cycle and the light candidate (worker, 2026-10-11)

Written before code, as the dispatch asks. Scope: R1 and R2 on the CPU, R3's candidate built and measured, then a stop at R3 for the owner. Nothing below deletes the lattice, the stepped executor or `reads_light()`. Those deletions are R3b and R4, after approval.

## 1. The cycle graph (R1)

### What exists

The whole-tree engine already runs Astra's cycle (`grow.rs`, `Grower::year`) in one grower:

| Astra's phase | Where it runs today |
|---|---|
| Light read at each bud from the previous cycle's frozen foliage | `shade` at the end of cycle c-1. It reads every living apex's tip into `Pencil::light`, which cycle c uses. Every bud reads the same completed field, so it is double-buffered in effect. |
| Bottom-up sums of capture, upkeep and presence | `balance` → `totals`: each axis's Q, C and living, summed child to parent in reverse index order |
| Top-down shedding | `shed_by` → `contract::shed::walk`, parents first |
| Vigour split by apical control | `allot` → `allocation::vigours_from` |
| Per-PA normalisation | `allocation::sizes`, the actual `a_i Σp / Σ(p a)` |
| Per-bud survival, abortion, continuation, dormancy and branching | `grow_live`, `wake` |
| End-of-cycle geometry and summary update | `sketch`: the rough layout, and the cycle's leaves with their areas |
| Final settle | `settle` |

Three gates make this a choice between ways of building rather than one law:

- `start` grows the rough layout only where `species.reads_light()` holds and `light.shades()`. Without a layout, nothing reads light.
- `balances()` runs the balance only where some `balance_hazard != 0`.
- `allots()` runs the allotment only where some `shade_size != 0`.

### The one cycle

`grow_cycle(species, request, model)` runs this grower with all three gates open, for every species and every light:

- The rough layout and the light read run every cycle but the last. The last cycle is skipped for every species alike, because no bud reads it.
- Balance and allotment run every cycle.
- The light evaluator is the `model` the build context is created with: `Lattice`, today's `light::Field` (the oracle), or `Volumes`, the R3 candidate. The model is chosen once per build, never by species or setting.

Each zero in the light settings gives a neutral term, not a different path:

- `extinction` 0 makes every transmittance exp(0) = 1.
- `shade_size` (ψ) 0 makes `drawn` return 1, so a bud's size is Σp/Σp, exactly 1 in f32.
- `balance_hazard` 0 makes the walk's hazard 0, so nothing is shed and the kept share is the bearer's.
- `shade_hazard` 0 makes survival ignore the light.

At zero and arbitrarily near zero the same code runs.

**Consequence: every lateral is grown.** Today's `lasting` skip of laterals certain to be shed applies only where no layout is grown. Under the one cycle a layout is always grown, so such laterals are always grown: they cast shade while they live. This adds grown phytomers, and so budget pressure, to the species that read no light today. It is measured per preset (section 4), not assumed.

**Production is untouched until approval.** `grow`, `grow_marked`, `grow_coarse` and the GPU roots stay on F4's executors. Each `reads_light()` branch selects between the stepped executor and another grower. Deleting the branches means moving the consumers onto the cycle, which is R3b, and the dispatch forbids deleting the stepped executor before approval. Until then the cycle is an entry point beside them, as fn-211's `grow_stepped` and `grow_oracle` were.

The gates stay in the code until R3b for the same reason. The cycle opens them with one grower flag, `one`.

**R1's checks before the stop:**

1. A species that reads no light, at extinction 0, grows through the cycle to the same tree as today's whole-tree engine. The kept tree is unchanged because a skipped lateral is shed anyway; the grown count may rise.
2. The oak through the cycle with `Lattice` agrees with today's build (`grow`, the stepped executor) within `agree::PATHS`. That is the tolerance today's stepped-against-whole test already holds since fn-254's floats.
3. The tree at extinction 0 equals the tree at 1e-30, and at ψ 0 the tree at 1e-30. Same code, no jump.

## 2. Allocation held (R2)

`allocation::sizes` is called unchanged, as `sizes(vigour, presence, psi, pa)`, i.e. `size_i = a_i × Σp / Σ(p a)` with `a_i = (v_i/p_i)^ψ`. The candidate replaces only the source of `Pencil::light`: the lattice's `Field::at` or the volumes' read. Every comparison in section 4 holds the allocation code fixed and changes the light evaluator alone. fn-226's `T̄^ψ` denominator is not used anywhere.

## 3. The light candidate (R3)

### Foliage

A cycle's foliage is the leaves the rough layout laid that cycle, as today's lattice receives them (`draw.rs`, `sketch_end`). Each leaf is a phytomer's tip with its area `scale × size × leaf_area`. Presence weighting is already in that area: a phytomer fading in or out carries its presence in `scale`.

Each leaf is now also tagged with the axis that laid it. The leaves an axis laid in one cycle are that axis's **group**.

### Moments

Each leaf is a small isotropic kernel with variance `s²` on each axis, where `s` derives from its PA's full leaf size `ℓ = sqrt(leaf_area)`.

A summary holds raw moments:

- `m0 = Σ A`
- `m1 = Σ A x`
- `m2 = Σ A (x xᵀ + s² I)`, six numbers

These are plain sums, so:

- merging two summaries is adding them;
- leaf area is conserved through every merge;
- a summary goes to zero continuously with its areas.

The summary's kernel is centred at `μ = m1 / m0`, with `S = m2 / m0 − μ μᵀ`.

The floor `s²` stays in `S` for any shape, so S is positive definite and every kernel is finite:

- one leaf: `S = s² I`;
- a line: `S` is `s²` across the line;
- a plane: `S` is `s²` across the plane.

The floor uses the PA's unscaled `ℓ`, never the scaled leaf. A fading leaf therefore keeps its kernel's size and only its density falls, rather than its density rising as it shrinks.

One leaf's kernel reaches `c ℓ` (`s² = (c ℓ)² / 7`). The multiplier `c` is measured, not fixed: see section 4.

### Kernel

The density is `ρ(x) = m0 × (15 / 8π) / sqrt(det 7S) × max(0, 1 − (x−μ)ᵀ (7S)⁻¹ (x−μ))`. Its second moment is `S` exactly, because the profile `(1 − r²)` on the unit ball has variance 1/7 per axis.

Along a ray `x = p + t d` (t ≥ 0, d toward the sky), write `u = p − μ` and `M = (7S)⁻¹`. Then:

- `q(t) = a t² + 2 b t + c0`, with `a = dᵀMd`, `b = uᵀMd`, `c0 = uᵀMu`.
- The density is positive between the roots of `q = 1`.
- Clipped to `t ≥ 0`, the integral is the closed form `[t (1 − c0) − b t² − a t³ / 3]` between those limits.

There is no marching and no lattice.

### Hierarchy

The hierarchy is the botanical one, so that its moment sums can fuse into the structural reductions:

- A node is an axis whose subtree holds any leaf area this cycle.
- The node's **subtree summary** is its group plus its children's subtree summaries, summed child to parent in reverse index order. That is the order `balance` and `allocation::subtotals` sum in.
- Each node also carries a **bound**: the radius about the subtree centroid that holds every kernel in the subtree, its own group's, its children's and every aggregate's. It is a max reduction, conservative, using `sqrt(7 tr S)` for each kernel's reach.

### Read

For each bud, the evaluator walks from the root and carries one optical depth per sky direction. Each node it reaches is handled as follows:

1. **Cull.** If none of the nine rays meets the node's bound sphere, the node adds nothing. Its every kernel, at every level, lies off every ray, so the cull is exact.
2. **Measure distance.** Let `D` be the bud's distance to the subtree centroid and `R = sqrt(7 tr S)` the aggregate's reach.
   - With `D ≥ far × R`, the aggregate kernel stands for the subtree.
   - With `D ≤ near × R`, the node is opened: its own group's kernel plus each child, each child judged the same way.
   - In between, the depth is `w × aggregate + (1 − w) × opened`, with `w` a smoothstep of `D / R` across `[near, far]`. This is the refinement interval's blend.
3. **No double counting.** The aggregate and the opened children are two complete descriptions of the same leaves, and the blend is convex.
4. **Continuity.** `w` reads only moments, which are continuous in the areas, so a walk moves the light by degree.
5. **Light.** The bud's light is `Σ_d w_d exp(−k depth_d)` over `Light::directions()`, today's nine directions and shares, held to [0, 1].

The evaluator counts the nodes it visits for each bud.

### Double buffering

The summaries are built from cycle c's leaves at the end of cycle c. Every bud reads them, and only then does cycle c+1 begin. No read sees a partial snapshot.

### Fusion with existing reductions

This is reported from the dispatch graph, not assumed:

- **Per group.** Each group's own moments come from the leaves the layout pass emits. That fuses into `sketch_end`, which already walks every laid phytomer.
- **Per subtree.** The subtree sums are a bottom-up reduction over the same links as `balance`'s `totals`. But the light read must finish before cycle c+1's balance, because balance sums the light read. The moment reduction therefore cannot ride inside that balance pass without lagging light by one more cycle, which is a model change.
- On the phase graph as it stands, it is one extra O(axes) additive reduction per cycle. On Astra's strict reading that is a miss. The final report states it either way.

## 4. Measurements for the stop

Every measurement uses its own target directory, at most 4 threads, one heavy process at a time.

### (a) Frozen

The oak (shipped preset, its own light) grows through the cycle under `Lattice`. At every cycle, on the same leaves and the same bud tips, both evaluators are read beside each other. The lattice's reads feed growth, so foliage, buds and allocation are identical for both.

Reported per age (40 and 80) and seed:

- light error per bud: signed mean, mean absolute, and p50, p90, p99 and max of the absolute error;
- the error's dependence on depth in the crown;
- summaries visited per bud: mean, p99 and max;
- build seconds and read seconds for each evaluator.

The leaf-size multiplier `c` is reported at 1 and 4. The lattice's own smoothing scale (0.5 m) is reported beside them. A brute-force sum of every group's kernel on a sample of buds separates the hierarchy's approximation error from the kernel model's difference to the lattice.

### (b) Accumulated

The oak grows through the cycle under `Volumes`, then under `Lattice`, with the same seeds and ages. Reported:

- kept and grown phytomers, shed share, height, crown extent and crown base;
- build time for each;
- stills (whole, mid and close) of the oak under `Volumes` beside F4's.

The stills are rendered from a throwaway patch that routes the renderer's build to the cycle. The patch is kept under `raw/` and never committed to the branch.

### (c) The figures fn-254 and fn-251 ask to be re-measured, on the cycle grower

- **Light to subtree totals.** The relative change of the subtree totals Q against a few f32 places of `sky`, on fn-254's walk tree (seed 3).
- **The shed window.** The memory change that tips it.
- **The fn-251 repro.** The oak → birch frame at t = 0.07504803866484144, age 80, seed 7: grown and kept on the cycle grower.

### (d) The one cycle's cost

Build time and grown count per shipped preset through the cycle with `Lattice`, against F4's `grow`.

## 5. What comes back as NEEDS_HUMAN

The stop at R3 carries:

- the numbers in section 4;
- the leaf-size multiplier, with what each value does;
- the fusion finding;
- the stills.

Any design question beyond this file goes back to the host as well. The candidate's parameters, `c`, `near` and `far`, are proposals with their measured effect. They are not settled values.
