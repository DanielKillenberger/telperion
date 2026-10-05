# fn-197 design options: light and the carbon balance, 2026-10-05

This is a proposal for the host, not a decision (AGENTS.md, "Dispatch and escalation"). Every claim about the code is marked **checked** with file:line in `crates/telperion-space/src` on `9575c2bd`, or marked **unknown**. Every claim about a source is tagged:

- **[read]:** read in the local text under `.worktrees/fn-188-research/.firecrawl/lit/`.
- **[brief]:** taken from fn-188's LITERATURE.md.
- **[not read here]:** cited from memory, still to be checked against the source.

## 1. What the engine gives a light pass today

### Growth has no positions

- **Growth is topological; geometry comes after it (checked).**
  - The cycle loop is `grow.rs:70-72`.
  - After it run `assign` (presences, `grow.rs:80`), `shed` (`grow.rs:84`), `scale`, `thicken` and `place` (`grow.rs:89-93`), then sag's second `place` (`grow.rs:96-99`).
  - During growth, a phytomer is pushed with `tip`, `heading` and `side` at their defaults (`grow.rs:361-370`).
  - **So no bud knows where it is while the tree grows.** Every in-growth light model needs a provisional layout first (section 2).
- **Presences are causal, so they can be known as the tree grows (checked).**
  - A unit's survival and persistence presences are computed when the unit grows (`grow.rs:216-218`, `grow.rs:237-242`).
  - A lateral's birth presence is computed when it sprouts (`grow.rs:385-387`).
  - A continuation or relay takes `end[parent]` (`presence.rs:154-157`), which is fixed once the parent stops.
  - Dominance is keyed by lineage (`geometry.rs:303-309`).
  - **So a provisional scale equal to the final one can be carried during growth**, with two exceptions that are only known at the end:
    - the shedding fade, which reads the tree's age (`shed.rs:53`, `shed.rs:109`);
    - a woken bud's `kept` (`grow.rs:76-79`).

### What a provisional layout cannot match

A layout made as the tree grows differs from the final `place` in three places:

1. **Sag (checked):** it runs after growth, and "growth never reads it back" (`sag.rs:12-13`).
   - **Measured on the oak at 80 years, seed 1:** the box is 23.5 × 18.6 m with sag and 15.9 × 15.5 m with every `sag` at 0.
   - So the oak's drawn crown is about half again wider than any unsagged layout.
2. **Secondary erection (checked):** it reads the tree's final age (`geometry.rs:91-96`), so an old limb keeps rising after its wood is laid.
3. **The straightening fade (checked):** it reads the axis's whole scaled reach, younger wood included (`geometry.rs:131-144`, `geometry.rs:237-243`).

### Shedding today

- **It is post-growth and by idle time (checked).** A lateral or relay axis whose subtree has held no living apex for more than its PA's `shedding` is dropped (`shed.rs:24-31`).
- **The seed axis and continuations are never shed by this rule (checked, `shed.rs:25-26`).** The main line therefore cannot be shed by an idle rule, and needs no special case for that.
- **Shed wood leaves no girth (checked).**
  - `thicken` runs on the kept axes only (`grow.rs:84`, `grow.rs:92`; `girth.rs:11-57`).
  - Pałubicki 2009 keeps the pipes of shed wood: "branch width is not decreased when leaves and branches are shed" [read, palubicki.md:682-684].

### Measured grow cost (2026-10-05, release, this machine)

| Tree at 80 years | Grow time | Kept phytomers | Expected grown (closed form) | Grown in cycle 80 |
|---|--:|--:|--:|--:|
| oak s1 | 3.8 s (sag off: 2.9 s) | 1.95M | 14.3M | 659k |
| oak s7 | 4.6 s (sag off: 4.0 s) | 2.59M | 14.3M | 659k |
| beech s1 | 2.9 s | 2.04M | 9.6M | 517k |
| spruce s1 | 9.0 s | 4.43M | 18.9M | 601k |

- **About 80 to 86 percent of grown wood is shed at the end.**
  - The budget counts shed wood (`grow.rs:355-358`).
  - Every phytomer is grown and stored, then dropped.
  - That is phase F's lever: an apex that dies in shade stops growing at once, and the wood it would have grown is never paid for.
- **How the rest of the time splits between the cycle loop, `assign`, `thicken` and `place` is unknown.** The engine has no stage timer (FRICTION.md, 2026-10-05).

### Two more facts from the code

- **Node counts step without growing in (checked).**
  - All three passed species draw node counts from `NodeLaw::Uniform` (`oak.rs:37`, `beech.rs:30`, `spruce.rs:31`).
  - Under that law every node's lead is infinite (`lineage.rs:95-98`).
  - So a light term on node count would pop nodes. "Growth slows" has to act on scale or on survival, not on counts.
- **Free lineage keys exist (checked).**
  - Keys used under an axis's lineage: `u64::MAX`, `-2`, `-4`, `-5` and `-6` (`lineage.rs:15-23`).
  - So `u64::MAX - 1` and `u64::MAX - 3` are free for a shed draw.
  - A light term that only moves an existing bound, such as viability, needs no new draw at all.

## 2. Where growth could read light

| Option | How | Cost | Fidelity to the drawn tree |
|---|---|---|---|
| **P1. Provisional layout, carried as the tree grows** | Each new phytomer is laid from its parent's provisional frame, with the provisional scale: tropism, wander and the insertion turn, as `lay` does. No sag, no later erection, and the straightening fade from the reach so far. Laterals read their node's frame. Light is read once per apex per cycle at its provisional tip. The provisional values can sit in `Phytomer.tip/heading/side`, which `place` overwrites (`geometry.rs:274-279`; `frame` reads only parents already laid, `geometry.rs:380-451`), so it needs no new memory. | About one `lay` step per grown phytomer. That cost is unknown until stages are timed; light-driven apex death lowers the grown count it applies to. | Exact before sag and erection. Off by the sag spread on the oak (see the box above), and by erection on species that use it. |
| **P2. A full re-layout every k cycles** | `scale`, `thicken`, `place` and sag on the tree as grown so far, reused for the next k cycles. | One full placement per checkpoint, on a tree that today holds its not-yet-shed wood (up to 14M phytomers). | Includes sag, but up to k cycles stale. |
| **P3. A post-growth causal sweep** | Grow light-free, then lay and read light in cycle order, and turn light into a presence. | One extra pass. | Good. **But it cannot bound size:** the full potential structure is still grown, so R2's node bound and phase F's lever both fail. |

- **Double-buffer the field in every option.**
  - Every bud in cycle t reads the field as it stood at the end of cycle t − 1.
  - Then no read depends on the order of apexes within a cycle (`grow.rs:180-189`).
  - Pałubicki computes the environment once per iteration, then the bud fates [read, palubicki.md:251-258].
- **What presences mean for continuity:** every leaf deposit is weighted by the present scale of what bears it.
  - An element a setting is just making deposits nothing, and casts no shade.
  - So a branch that is born or crossed moves every other bud's light by degree.
  - This is the same contract `presence.rs` gives wood.

## 3. Light model candidates

### L1. Pałubicki's shadow propagation

**Mechanism [read, palubicki.md:335-388]:**

- Each bud adds shadow Δs = a·b^−q to the voxels in a pyramid q layers below it, for q up to q_max (typically 4 to 8).
- The exposure is Q = max(C − s + a, 0).
- The grid is 200³ cells, each edge about one internode [read, palubicki.md:776-781].

**How it reads the tree:**

- Under P1, each new unit casts its pyramid once.
- When its leaves expire or the unit is shed, the same pyramid is subtracted.

**Cost per cycle:**

- One pyramid is 1 + 9 + 25 + … + 169 = 455 voxel updates at q_max = 6.
- At 400k to 660k new phytomers in the late cycles, that is about 2–3 × 10⁸ updates per cycle.
- Casting per occupied cell instead is the same convolution and cheaper. Its cost is unknown.
- fn-190 round 3 measured 25.1M voxel updates for an 11k-node beech (R1-ROUND3.md).

**Continuity:**

- The bud's voxel is an integer index, so a bud crossing a cell face moves its whole pyramid at once.
- It needs splatting into neighbouring cells by trilinear weights.
- The max(…, 0) is a kink: continuous, not smooth.

**Oracle:** the formula itself, cell by cell. It is exact but tests only the bookkeeping.

**Known failure:** the shadow is local (q_max layers), so nothing high in the crown darkens seedling-era limbs, and no bole forms. fn-190's host wrote this after round 3 (SPEC-HISTORY.md, round 3 decisions).

### L2. Beer–Lambert on a coarse leaf-area grid, from a few sky directions

**Mechanism:**

1. Deposit each living unit's leaf area into a world-anchored lattice by trilinear weights.
2. Once per cycle, sweep each sky direction through the lattice layer by layer, bilinear along the ray, accumulating the leaf area met.
3. A cell's light is Σ_d w_d · exp(−k · A_d).
4. Use one zenith direction plus a ring or two. The weights w_d run from zenith-only to a standard overcast sky (Moon & Spencer 1942, radiance ∝ (1 + 2 cos θ)/3 [not read here]) along one continuous setting.

This is fn-190 round 4's field (zenith with a 45° cone) and round 5's (five directions), written again from the published law, not copied.

**How it reads the tree:**

- P1 deposits one unit at a time.
- A bud reads one trilinear lookup (8 cells) at its provisional tip.

**Cost per cycle, estimated:**

- At 0.5 m cells, the oak's 24 × 24 × 22 m box is about 100k cells.
- With 9 directions, a sweep is about 0.9M cell steps per cycle, or about 70M over 80 cycles.
- On top come 8 cell writes per deposited unit and 8 reads per live apex.
- fn-190 round 4 measured its zenith field at 24.8 ms for 0.98M cells over a whole run (R1-ROUND4.md).

**Continuity:**

- Trilinear deposit, trilinear read and exp() are all smooth in position and in presence.
- The lattice is anchored at the world origin and grows by whole cells. Added cells are empty, so growing the grid changes no value.
- A tree outside a stated maximum extent is an error, not a clamp.

**Oracles:**

- **(a) The homogeneous slab:** transmittance exp(−k·LAI) at every depth (Monsi & Saeki 1953 [not read here]). This is exact up to the cell size.
- **(b) A uniform sphere or ellipsoid of leaf density:** transmittance along any ray is exp(−k·ρ·chord), with the chord in closed form (the turbid-medium crown of Norman & Welles 1983 [not read here]).
- **(c) Whole-tree production:** the summed income against GreenLab's Q = PAR·RUE·Sp·(1 − e^(−k·S/Sp)) [read, letort.md:144-158] on a crown whose projected area is Sp.
- **(d) An executable oracle:** RATP (Sinoquet et al. 2001, a voxel radiative-transfer model in OpenAlea [not read here]) could be run unchanged. Whether it installs and runs here is **unknown**.

**Known failures (fn-190):**

- **Columnar crowns came with it (rounds 4 and 5),** but there light also steered direction (round 5's phototropism) and fed a resource that went to the top. Here directions stay the species' own. That this avoids columns is a hypothesis, unverified.
- **Low sky directions lit the lower crown's sides and lost the bole:** 0.20 H in round 4, 0.07 to 0.12 H in round 5 (R1-ROUND5.md). The zenith weight is the dial between the two.

### L3. Whole-tree Beer–Lambert with no spatial light (GreenLab)

**Mechanism:**

- One production number per cycle: Q(n) = PAR·RUE·Sp·(1 − e^(−k·S/Sp)), with Sp = Sp₀·(S/Sp₀)^α [read, letort.md:150, 188].
- It modulates counts or vigour through Q/D [read, letort.md:380, 422].

**How it reads the tree:** total leaf area and demand only. It needs no layout.

**Cost:** negligible.

**Continuity:** smooth in every input. GreenLab rounds its counts [read, letort.md:384], which this engine would replace with presences.

**Oracle:** the equations are closed form. The phase A oracle simulators are structure-only (checked, `scripts/greenlab-oracle.mjs:8-21`), so no executable GreenLab carbon model is in hand. Whether one can be run is **unknown**.

**Fails aim 1:** with no space, nothing shades one bough more than another. It forms no dome and no bole by shade. It can bound size only.

## 4. How light acts on a bud

The mechanisms below keep every existing draw and add no switch:

- **Stop (aim 1):**
  - An apex's survival bound becomes viability · I^φ, with φ ≥ 0 and neutral 0.
  - I^φ is Stava 2014's flush probability for apical buds, P = I^φ_LF_A [read, stava.md:273].
  - It moves the existing viability draw's bound (`grow.rs:205-217`), so it needs no new key.
  - A shaded apex dies by degree through the survival window, and the idle rule then sheds what it bore (`shed.rs:24-31`).
- **Slow (aim 1):**
  - A unit's phytomers are scaled by I^ψ, with ψ neutral 0. Fewer, shorter internodes in shade agree with DTT86's light effect on young beeches [brief].
  - It is a factor like a woken bud's `grown` (`presence.rs:139`), not a draw, so it carries no log-odds sensitivity.
  - Node counts cannot carry it (section 1).
- **Neutral is exact:** at k = 0, exp(−0·A) = 1 for finite A, and 1^φ = x^0 = 1 in IEEE arithmetic. Every bound and scale is therefore bitwise unchanged.
  - With every light setting neutral, the grid and the provisional layout can be skipped, as `sag::any` skips sag's pass (`sag.rs:35`, `grow.rs:96`).
  - The test is `Structure` equality (derives `PartialEq`, `structure.rs:162`) against today at every passed species and seed.

## 5. The carbon balance

### What each branch remembers

- **Per lateral axis, its subtree's balance** B = (I − C) / (I + C), in −1 to 1. This is Takenaka's light ÷ size [read, palubicki.md:651-658], made continuous and signed. Here:
  - I is the subtree's leaves × their light;
  - C is upkeep r × its wood (present internodes × scale; radius is unknown during growth, since `thicken` runs after it, `grow.rs:92`).
- **Memory** is an exponential average, M ← m·M + (1 − m)·B: one f64 per axis.
- **Aggregation:** one reverse pass over the axes each cycle, children before parents. This order holds because parents precede children (checked, `shed.rs:15-20`; `structure.rs` header).
  - The cost is O(axes) per cycle.
  - Axes grown are unknown: 221k are kept on the oak, and grown axes are a multiple of that.

### Shedding on the balance

- Each lateral subtree takes, each cycle, one draw under a new key (lineage · `u64::MAX − 1` · cycle).
- The draw is tested against a hazard h(M), which rises smoothly as M falls below the species' shade tolerance.
- Its lead past h gives a persistence presence that multiplies everything the subtree grows afterwards, as a woken bud's `kept` multiplies `birth[1]` (`grow.rs:76-79`).
- On the side where it fires, the subtree's apexes stop at once:
  - this is the cost lever;
  - the idle rule drops the subtree at the end.
- Neutral is r = 0 with the hazard scale at 0. The seed axis and continuations are never tested, as `shed.rs:25-26` already holds. This is the existing structure, not a special case.

### Vigour and size

- "Growth bounded by it" comes from self-shading. Under L2, income per leaf saturates as the crown fills, while upkeep grows with wood. A lateral's balance therefore falls with size, and its tips die or slow.
- GreenLab's Q/D ratio is the published form of the same coupling [read, letort.md:328, 380, 422].
- An optional unit factor (1 + M)/2 raised to a power, neutral 0, would let the balance slow growth as well as end it.

### Girth from leaves

- Each phytomer's own pipe (`girth.rs:27`) is weighted by its remembered light raised to χ, with χ neutral 0.
- Then limbs whose leaves catch the most light thicken most. GreenLab partitions ring growth by the leaf area above a metamer [read, letort.md, Fig. 1 caption].
- That needs light stored per growth unit (an f32 per unit).
- Whether shed wood should keep its pipes, as Pałubicki keeps them, is a separate decision (section 1). Its neutral would be off.

## 6. Risks

### Walks (bound 30, `tests/walks.rs:14`)

- **Log-odds gain near certain survival.**
  - With viability 0.999, a light change from 1.0 to 0.98 at φ = 1 moves the bound's log-odds from 6.9 to 3.9.
  - Light moved by a setting can therefore move draws' leads far faster than the setting itself moves the bound.
  - This is the largest continuity risk. Measure it with a walk on k and φ before anything else.
  - Acting through scale (ψ) has no such gain.
- **Feedback compounds over cycles.** A crossing near the base moves positions, hence light, hence presences, hence deposits, over many cycles. Whether the loop's gain stays below the bound is unknown.
- **Discrete grid effects:** an integer voxel index, a grid origin that moves, and order within a cycle. Each is removed by trilinear weights, a world anchor and double-buffering (sections 2 and 3).
- **Leaf expiry:** a whole number of foliage years as a setting would step. Weight the last year by a fraction, or decay leaves continuously.

### The oracle at neutral

- **Exact if the neutral path is skipped or bitwise identity-preserving.**
  - The closed-form and GreenLab tests stay at neutral settings.
  - With light on, the closed form no longer predicts counts. Shedding is already outside it (`closed_form.rs:12-13`).
- **Presence windows stay on the light-free expectation** (`presence.rs:22-41`, `expected_log_lengths`). That keeps them smooth in the settings, but overstates the wood a draw decides when light kills.
  - The effect is slower grow-in, not a jump.
  - Whether it matters is unknown.

### Cost

- **The provisional layout's per-phytomer cost** is unknown and is the largest new term. It falls as shade kills apexes, because fewer phytomers are grown.
- **Measure it first:** neutral-off against today, then light on.
- **The grid is cheap under L2 (estimated) and expensive under L1** (section 3).
- **The balance pass is O(axes) per cycle.** It is cheaper if shed subtrees leave the live list.

### Fidelity

- **The provisional crown misses the oak's sag spread** (measured, section 1). Light computed on the unsagged crown will over-shade it. That can misplace the dome the light is meant to make.
- **Collapse:** fn-190 rounds 2, 3 and 5 collapsed when the main line was shed. Here the main line is never balance-shed (`shed.rs:25-26`). Its apex could still die by light, but it is the most lit apex.

## 7. Recommendation

**L2 on P1**, with light acting through survival (φ) and scale (ψ), and the balance and shed draw of section 5.

### Reasons

1. **L2 is the only candidate that reaches the whole height and costs a sweep per cycle.**
   - It reaches the whole height, which the bole needs.
   - Its cost is a sweep per cycle, not a pyramid per bud.
   - Its laws have closed-form oracles: the slab, the turbid sphere, and GreenLab's Q.
   - fn-190 round 4 already showed it forming a 0.18 to 0.24 H bole with seeds within 1.2× in node count.
2. **P1 is the only reading that bounds growth and saves cost.** P3 grows the full potential structure. P2 re-lays a tree of up to 14M phytomers.
3. **Stava's I^φ and a scale factor I^ψ both reduce exactly to today at 0.** Both use draws the engine already has, so lineage keys and the walk machinery carry over.

### Order, each step with its own check

1. Build the grid and its oracle tests (slab, sphere) in isolation.
2. Add the P1 layout at neutral light.
   - Its provisional tips are compared against the final `place` with sag off, to measure the error.
   - Its cost is timed against today's 2.9 to 9.0 s.
3. Turn on φ alone on the oak, then run the walk on k and φ (the log-odds risk).
4. Add the balance and the shed draw (R2).
5. Add girth from leaves.

### Decisions for the host

- **Whether the provisional layout should approximate sag.** The oak's spread depends on it.
- **The sky weighting's range.** It sets the trade between bole and spread.
- **Whether shed wood keeps its pipes.**
- **Whether "slow" acts by scale alone or also by the balance factor.**

## 8. Host decisions (host, 2026-10-05)

The host accepts the recommendation: an L2 Beer–Lambert grid, read through P1, the rough layout built as the tree grows, in the five steps of section 7. The decisions below settle section 7's open questions and change section 4's survival form.

1. **No sag in the rough layout at first.**
   - Step 2 measures the gap between the rough and final layouts on the oak and the beech: crown extent and light per bud.
   - If the gap misplaces the dome (judged at step 3 on the oak's stills), a periodic full re-lay with sag every K cycles is added. K is an engine constant chosen for cost, and its cost is measured. It is not built now.
2. **Sky is one continuous `sky` setting.**
   - It runs from overhead-only (0) to a uniform overcast sky (1), with the standard overcast sky as the reference point.
   - It is a site setting, not a species one: it lives on the light model, and every species is judged under one value.
   - Step 3 walks it on the oak to find where the bole and the spread balance; the value chosen and the reason are recorded.
3. **Shed wood keeps its thickness, by share.**
   - `retained` (neutral 0, today) is the share of a shed branch's pipe that stays in its bearer's girth: Shinozaki's disused pipes.
   - Above 0 it changes every tree, so it is walked and judged per species.
   - It is built in step 5, beside the thickening from leaves.
4. **"Grows slower" acts through shoot size (ψ) only.** The balance acts only through shedding: one lever each.
5. **Light acts in hazard space, not by multiplying survival.**
   - ln(survival) = ln(viability) · I^(−φ), so survival near 0.999 moves gently as light changes, and φ = 0 is exactly neutral.
   - The balance's shed hazard likewise rises continuously as the remembered average falls.
   - Both are walk-tested, with a red-first test: the multiplicative form viability · I^φ against the walk bound, if it fails as section 6 predicts.
6. **Per-stage timings go into the engine's measures example** (growth, rough layout, light sweep, final lay) as part of step 2. This closes the 2026-10-05 friction entry, and cost is tracked from then on.
