# fn-226 task 3, R4 before code: the inventory, the equations, and the stops (worker, 2026-10-08)

R4 asks for two things before any code: an inventory of what allotment and balance read, and the written equations with their inputs. This is both. No code was written. The base is `ae54e4ed`, which is task 2's receipt: the re-lay is deleted. Each claim about the code was checked against that commit.

**Result: stopped for the host.** One quantity the one-way design needs has no expectation in the closed form (S1). Seven further points are design choices the spec leaves open (S2 to S8). Each is listed below with its options.

## 1. What allotment and balance read today

### Balance, phase (i) (`grow/balance.rs`)

Each axis's subtree is summed child to parent in `totals`. The sums are exact in Q32.
- **Light Q:** for each living apex, `base_scale × running × light` (`balance.rs:76`).
- **Upkeep C:** for each laid axis, `upkeep(pa) × pencil.wood`. `pencil.wood` is the axis's own laid wood in metres (`draw.rs:414`).
- **Living:** whether any apex in the subtree lives.

D16's walk (`laterals`, then `fixed::feedback::shed::walk`) reads more for each lateral that decides:
- the summed Q and C;
- its remembered balance (`pencil.memory`);
- `balance_hazard` and `tolerance`;
- its draw under `SHED` and the cycle;
- the window at the cycle.

Parents go first. A shed bearer hands its children the shedding it inherited, and every living apex gets a kept share (`shed_walked`).

**Across the cut today** (`stepped/phases.rs::balance`):
- Each item reports `(light[1], cost[1], living[1])` at its root, and the coarse grower decides the root's placeholder from it.
- The coarse grower hands down `(shed, kept, memory)`.
- So each cycle needs a reduction up from every item and a broadcast back down.

### Allotment, phases (ii) and (iii) (`grow/sketch.rs`, `allocation.rs`)

**Per axis** (`inputs`):
- `presence = base_scale × running × kept`;
- `lit = presence × light`;
- `lambda = apical_control(pa)`;
- `psi = shade_size(pa)`. This is 0 where the axis has no living apex.

**Vigour** (`allocation::split`): Borchert–Honda over the whole tree.
- The base's vigour is the tree's summed `lit`.
- At each node, `v_main = v λ Q_m / (λ Q_m + (1 − λ) Q_l)`.
- At the tip, the own bud and what carries the axis on share the rest by their `lit`.
- The oak's λ is 0.45 (`oak.rs:95`), not the unbiased 0.5. So a bud's v/p is its light times the biases on its path, not its light alone.

**Size** (`allocation::sizes`, `size_of`):
- `size_i = (v_i/p_i)^ψ_g × Σ_g p / Σ_g p (v/p)^ψ`, where g is the bud's PA.
- `(v/p)^ψ` is `exp2(ψ (log2 v − log2 p))`, in Q32.
- The per-PA sums are exact W totals over the whole tree (`size_sums`).

**Across the cut today** (`stepped/phases.rs::allot`):
- Each item reports its subtree's `lit` at its root.
- The coarse grower splits with it and hands down the root's incoming vigour.
- Every grower's per-PA sums are merged (iii).
- Two more reductions and broadcasts a cycle.

### Other readers of a bud's light

- **Survival** (`grow/advance.rs:61`): `viability^(light^−shade_hazard)`.
- **The phytomer's recorded light:** `leaf_girth` reads it in `girth.rs:149`. The oak sets no `leaf_girth`.
- **Initialisation:** before any field, a bud reads light 1 (`grow/draw.rs:338`).

## 2. What the closed form supplies today

| Quantity | Where | Indexed by | Light | Shedding |
|---|---|---|---|---|
| Expected phytomers grown per PA and cycle | `expected_counts`, the counts walk | bud PA, cycles grown | neutral | none |
| Expected leaf area grown in a bud's m-th cycle | `last_leaves` (probe P2) | PA, m | neutral; probe-only light axis (`alone/field.rs::at_light`, `viability^(L^−φ)` at 5 levels) | none |
| Expected metres of wood grown in m cycles | `expected_log_lengths` | PA, m | neutral | none |
| Kept wood and its moments, sections, mass, kept phytomers, reach bound | `Kept` tables, `boundary::Rows` | PA, m (cycles before the tree's age), at the tree's age | neutral | yes, at the tree's age |
| Chance no apex lives past t | `Rows::dead_by` (extinction curve) | PA, birth, t | neutral | n/a |
| Stand-in wood moments (Σl, Σl x·h, Σl x²) | `closed_form/envelope.rs` | PA, m | neutral | none |
| Kept phytomers (the cut) | `predictor::Predictor` | PA, birth | neutral (`expected_kept` refuses lit species) | yes |

**What it does not supply:**
- **No expected living apexes or their presence at a cycle.** There is no expected `Σ base_scale × running` over the living apexes of a subtree at its a-th cycle.
- **No per-cycle quantity with shedding.** The `Kept` rows are at the tree's age. `couple/stand.rs::row_at` stands a row of a bud aged a in for cycle a, as fn-211.4 did.

## 3. The equations (R3)

Frame: z up, with the seed at the origin. A point's reach is `ρ = hypot(x, y)`, measured off the seed's vertical.

### The outline at the end of cycle c

It is folded by the structure above the cut, from the following points:
- **Its own leaves:** every leaf it grew in cycle c, at its rough-layout tip, with area `leaf_area(scale, size, area)`. These are the same leaves today's lattice deposits.
- **Its fine roots:** each living root's expected leaf area of cycle c, standing within its expected reach of its base (how a root's ball folds is S7).

The folds follow core's `Folds` (`pipeline/input/folds.rs`), in the engine's frame:
- `H = max z` over every point.
- `B = min z` over crown points, then `B = min(B, H)`. A crown point is off the stem: it is not on the seed's chain (`pencil.trunk`).
- `(R, z_R)` is the widest point's reach and its height.
- The fullness is `f = clamp((z_R − B)/(H − B), 0, 1)`.
- The shoulder s is the least in [1/16, 64] whose profile holds every crown point (`shoulder.rs::contained`). It folds in two passes over the coarse grower's own points, sequentially, with no added synchronisation.

**The profile:**
- For z in (B, H), let `t = (z − B)/(H − B)`.
- Then `p = 1 − t/f` for t < f, else `(t − f)/(1 − f)`.
- `r(z) = R · quadrant(p, s)`, where `quadrant(p, s) = (1 − p^s)^(1/s)`.

**The volume, by the 64-point midpoint rule, in fixed point, once a cycle:**
- `V = π (H − B)/64 · Σ_{j=0..63} r(B + (j + ½)(H − B)/64)²`.
- `quadrant` is built on contract A's `log2` and `exp2`.

**The leaf area density:** `D = A / V`, where A is the cycle's leaf area (the own leaves plus the roots' expected area). Where `V = 0`, `D = 0`.

**The 16 cone sections:**
- Sample at `z_k = B + k (H − B)/16`, k = 0 to 16, with `r_k = r(z_k)`.
- Section k spans `[z_k, z_{k+1}]`, with radius `a_k + b_k z`, where `b_k = (r_{k+1} − r_k)/h` and `h = (H − B)/16`.

### The light at a bud at P

The light a bud at P reads in cycle c+1 comes from the outline of cycle c. Along each of today's nine sky directions d (unit, `d_z > 0`, with shares `w_d` from `Light::directions`), the ray is `X(u) = P + u d`, u ≥ 0.

**The chord through section k:**
1. Let `e = a_k + b_k P_z` and `g = b_k d_z`.
2. The ray is inside where `q(u) = A u² + 2 B' u + C ≤ 0`, with:
   - `A = d_x² + d_y² − g²`;
   - `B' = P_x d_x + P_y d_y − e g`;
   - `C = P_x² + P_y² − e²`.
3. The section's own height bounds the ray to `[u_lo, u_hi]`, with `u_lo = max(0, (z_k − P_z)/d_z)` and `u_hi = (z_{k+1} − P_z)/d_z`. The section is empty where `u_hi ≤ u_lo`.
4. The chord `L_k` is the length of `{u : q(u) ≤ 0} ∩ [u_lo, u_hi]`. Within a section the radius is non-negative, so the mirror nappe never enters.

**The cases:**
- **`A ≠ 0`:** let `Δ = B'² − A C`.
  - `Δ < 0`, or `Δ = 0` (tangent): the chord is 0 for `A > 0`. For `A < 0` the inside set is the whole line, or the whole line minus a point.
  - `Δ > 0`, with `A > 0`: the interval is between the roots `(−B' ± √Δ)/A`.
  - `Δ > 0`, with `A < 0`: the inside set is the two rays outside the roots.
  - Each case is clipped to `[u_lo, u_hi]`. The `√` is an integer square root in fixed point, so there is no transcendental.
- **`A = 0, B' ≠ 0` (linear):** the ray runs along a cone's generator. The inside set is `u ≤ −C/(2B')` for `B' > 0`, else `u ≥ −C/(2B')`, clipped to the section's bounds. The chord can be positive.
- **`A = 0, B' = 0` (constant):** the ray is inside over the whole span where `C ≤ 0`, so `L_k = u_hi − u_lo`. Otherwise `L_k = 0`.
- **A ray that misses a section, or is tangent to it,** gives `L_k = 0` there.

**The transmission:**
- `T(P) = clamp(Σ_d w_d · exp(−k_ext · D · Σ_k L_k(d)), 0, 1)`.
- That is one `exp_neg` of contract A per direction: nine per bud, and no other transcendental.

**Before the first outline with `V > 0`,** every bud reads light 1, as `draw.rs` initialises it.

**Cost per bud:** 9 directions × 16 sections, each one small quadratic and one isqrt where `Δ > 0`, plus 9 exps. Every bud is independent, so this is evaluated inline where the bud grows (advance, stop and sizes read `light(axis)`), and there is no light pass and no stored field.

**The history:** one outline per cycle: `(B, H, R, f, s, V, D)` and the 17 radii, which are derived. The coarse grower records it, and every fine root reads the entry for its own cycle.

## 4. The equations (R4)

### Allotment's normalisation

- **Today:** `size_i = (v_i/p_i)^ψ_g · Σ_g p / Σ_g p (v/p)^ψ`.
- **R4:** `size_i = (v_i/p_i)^ψ_g / M_g(c)`, where `M_g(c)` is the per-PA mean the outline implies. Per the spec it is computed from the outline's mean transmission over its volume, `T̄(c)`. The reading `M_g = T̄^ψ_g` is S5.
- **In contract A:** `drawn(v, p, ψ) · ONE / M_g`: one division, one truncation. A bud with no presence is whole, as today.
- **What R4 drops:** the per-PA `Σ` reduction (iii) and its merge across growers.
- **A consequence to measure, not a stop:** at λ = 0.45 a bud's v/p is its light times its path's biases. Each PA's presence-weighted mean size is then no longer 1 to the last place. Growth can move between orders by degree, where today it cannot (host decision 23's "light moves growth within an order, never from one order to another").

### The vigour split

The split (ii) is kept inside the coarse grower and inside each fine root:
- the coarse grower, sequentially, over its own buds and each fine root's expected lit presence (S1, S3);
- each fine root over its own buds, from the incoming vigour the history hands it (S4).

### Balance

Balance is unchanged inside each fine root and inside the structure above:
- A fine root enters its bearer's balance as `(Q_r, C_r, living_r)` from expectation, not from its realized subtree.
- The coarse grower decides the root's `(shed, kept, memory)` and records them in the history.

### The per-root history

Each fine root reads its history per cycle: shed cycle, kept share, memory, incoming vigour, and the shared outline.

## 5. Each quantity the one-way design needs, against what exists

| Needed, per fine root r, per cycle c | Equation | Available? |
|---|---|---|
| Expected leaf area of cycle c | `A_r = s_r · last_leaves[pa][c − birth]` (per unit base scale, linear in it, as `alone/field.rs` and `stand.rs` scale it) | Yes, light-neutral. Reading it at light is S6. |
| Expected extent | `reach_r = s_r · Rows(pa, m = c − birth).reach`, as `stand.rs::row_at` does | Yes, light-neutral. A row is at the tree's windows. |
| Expected light Q, for its bearer's balance | `Σ_{living apexes} base_scale · running · light` | **No: S1** (living-apex presence); which light is S3 |
| Expected lit presence, for its bearer's split | `Σ presence · light`, the same sum times kept | **No: S1**, and S3 |
| Expected upkeep C | `upkeep(pa) · s_r · E[present wood]` | Approximations exist; which to take is S2 |
| Living | `1 − dead_by(pa, birth, c)` > 0, or the S1 quantity > 0 | Yes (extinction curve) |
| Incoming vigour, and its split inside the root | coarse grower's split | The rule is S4 |
| `T̄(c)` | mean of `T` over the outline's volume | The quadrature is S5 |

## 6. Stops for the host

### S1. No expectation of a fine root's living-apex presence

**Stop.** This is the quantity with no available expectation.
- The bearer's balance needs a root's `Q_r(c)`, a sum over its living apexes.
- The bearer's vigour split needs its lit presence.
- The closed form has counts, lengths, kept rows and extinction curves, but no expected count or presence of living apexes at a cycle (section 2).

**Options:**
- **(a) fn-211.4's `Aggregate`** (`couple.rs`). Living cohorts by PA are advanced each cycle under the root's history inputs (its light, its kept share, its sizes at 1).
  - It exists and has run in `grow_coupled`.
  - It is a mean-field history model, not the closed form.
  - It does not carry inner shedding or sleeping buds (its stated approximations).
- **(b) A new closed-form column:** expected growth units per cycle, carried through the counts recursion. It is new derivation, to be measured against realized trees.
- **(c) Derived from the counts:** `Σ_PA E[phytomers of PA in cycle a] / E[nodes per unit of PA]`. It is wrong where failed units and relays grow part units, and it carries no presence (running) term.

**Worker's leaning:** (a). It is the only one built, and fn-211.4's records already ride it. It does make the "expectation" a history model rather than the closed form, so it is the host's call.

### S2. Which expected wood is the upkeep's

**Options:**
- **(a) `expected_log_lengths`:** wood grown in a cycles, no shedding, light-neutral.
- **(b) The `Kept` row at m = a:** WOOD with shedding at the tree's windows (not in `Row` today; it would be added).
- **(c) The `Aggregate`'s `wood`,** with (a) of S1.

### S3. Which light a fine root's expectation reads

This light feeds `Q_r`, its lit presence, and survival in S1(a).

**Options:**
- **(a) R3's per-bud light at the root's base.**
- **(b) At its expected centroid,** as `stand.rs` read the lattice.
- **(c) `T̄`.**

(a) and (b) read the outline of c − 1, the same lag as a bud.

### S4. How a root takes its incoming vigour (open since probe P4, `couple.rs::Split`)

**Options:**
- **(a) `Recorded`:** the whole recorded share is the base of its own split.
- **(b) `PerLight`:** the recorded share per expected lit presence, times the lit presence its own buds collect.

R4's normalisation makes the scale of v matter, where today's per-PA total cancelled it, so the choice now changes sizes.

### S5. `M_g` and `T̄`'s quadrature

R4 names the mean transmission over the volume. It does not name:
- whether `M_g = T̄^ψ_g`, or the volume mean of `T^ψ_g` (they differ by Jensen's gap);
- the quadrature for `T̄`.

T varies with azimuth (the sky rings are at 4 azimuths), so the mean needs points in (z, ρ, θ).

**Options:**
- **(a) `T̄` by the 64 midpoint heights × a fixed set of radii (for example 4, at equal area) × the 4 ring azimuths, weighted by volume, and `M_g = T̄^ψ_g`.** This is 1,024 evaluations a cycle, on the coarse grower, once.
- **(b) The same points, and `M_g` the mean of `T^ψ_g`.**
- **(c) T on the axis only:** cheaper, and biased toward shade.

### S6. "Extended to read the expected light of R4" (R3's outline source)

There is no production extension. The only light axis is probe P2's: `last_leaves` of `at_light(species, L)` at 5 levels, interpolated in log L (`alone/field.rs`). It changes viability only, and the reach bound has no light axis at all.

**Options:**
- **(a) Probe P2's levels in production:** leaf area interpolated at the root's light (S3). The reach stays neutral.
- **(b) The `Aggregate`'s leaves** under the root's history light (with S1(a)). The reach stays neutral.
- **(c) Light-neutral expectations,** as `stand.rs` stands them.

### S7. How a fine root's expected ball folds into the outline

**Options:**
- **(a) The ball's top (`H`), bottom (crown base) and outermost point (`ρ_base + reach`, at the base's height) folded as points,** and the shoulder's front taking the outermost point.
- **(b) `stand.rs`'s 7-point stencil** (the centroid ± σ√(7/2) along the frame's axes) folded as points.
- **(c) Only the ball's centre**, with its leaf area.

### S8. Which GPU executor

R3 asks for the evaluation inline in the step that grows the bud, "on both executors". For a lit species, the GPU builds nothing today:
- `apart()` refuses a species that reads light, and `preparation.rs` routes it to the CPU.
- The only device path for lit roots is `fine/feedback_stepped.rs`, a test-only `Feedback` device. It walks D16, deposits, samples and lays, and the CPU grows the buds.
- The device's own growth step (the chain grower, `apex.wgsl`) grows unlit roots without positions, so it has nothing to read a bud's place in the crown from.

**Options:**
- **(a) R3's per-bud read as a `Feedback` call on the test device.** It replaces `deposit` and `sample`. Parity is held there, as task 10 held the lattice.
- **(b) Lit-root growth on the chain grower:** a rough layout in the growth step, D16 and the root's own split. This is a build of its own (`SCOPE_EXCEEDED` for this task), and a spec.
- **(c) The GPU half deferred to R6's verdict,** which already makes "inline on the GPU" conditional on R3+R4 being chosen.

## What was not done

- **No code.** The spec's order is the inventory first, and its stop rule applies (S1).
- **The `apical_control` light walk** carried from task 2 is not judged again. It needs R3's light built, so it waits on these decisions. Its bound is unchanged.
- **Cost:** no build or test was run, and no capture was taken. The inventory was read from the code at `ae54e4ed`.
