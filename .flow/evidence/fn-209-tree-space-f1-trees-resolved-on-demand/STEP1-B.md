# fn-209 step 1: shedding, section, mass and the extinction curve in the closed form (worker, 2026-10-05)

A dispatched worker wrote this report. Step 1 of the current order is built and validated, apart from the two items in section 4. The code is now past the 300 to 500 line estimate (section 3), so the worker stops here as decision 9 asks.

## 1. What is built

All in `crates/telperion-space/src/closed_form/`:

- **`steps.rs`** (216 lines).
  - A bud's own axis is now written as growth units: for each one, the chance the axis reaches it, grows it and carries on past it, and the relays and continuation each end leaves.
  - The flat events of the counts, lengths and envelope (`events.rs`) are now read from these steps, so the schedule's chances are worked out in one place.
  - The counts are unchanged to the bit: every species at 80 years, seeds 1 and 7, neutral and lit, hashes as F2 recorded.
- **`kept.rs`** (255 lines) and **`kept/path.rs`** (435 lines): what a bud's subtree keeps at the tree's age, per unit base scale.
  - **The quantities:**
    - kept wood and its moments (first along the heading, second about the base, second along the heading);
    - the pipe model's base section at each pipe exponent the species uses;
    - the mass sag hangs on it and that mass's moment along the heading (pipe mass and leaf mass);
    - **the extinction curve**, P(the subtree's last living cycle ≤ t), for every t.
  - **Shedding with its joint history (Astra's note: E[W·1_kept] is not E[W]·P(kept)).** A bud's table holds E[Y; T ≤ t] jointly with T, the subtree's last living cycle.
    - A lateral or relay is faded by its own T against its PA's delay, as `shed.rs` does.
    - A borne subtree enters its bearer's table as the factor P(T_c ≤ t), and once as its quantity.
    - Over a zone's random node count, that becomes the count's generating function and its derivative. Each zone's node count and each unit's length are tilted by what they bear, so the place of later wood is taken under the same weighting.
    - Each way the axis can end weighs in the units grown before it: failing, aborting, ending its age, or living on.
- **`along.rs`:** the node-count probabilities (uniform and Poisson, as `lineage::nodes` draws them) and dominance moments at any power.

## 2. Oracles

| Test | Result |
|---|---|
| `a_deterministic_tree_keeps_what_the_formula_keeps`: opposite pairs, a relayed trunk, twigs that die and are shed after a delay; ages 1 to 9 | **Exact to 1e-9** for wood, both moments, mass, mass moment and section |
| `branches_keep_on_average_what_the_formula_keeps`: Poisson and uniform node laws, 1.5 and 2.5 bud places, failed units and abortions with relays, twigs shed after a delay; 60 seeds; every branch made is counted, a shed one as 0 | **Within 4.5 standard errors** in every birth group of 300 or more branches, for all six quantities and for P(T ≤ t) at every t. T is read off a twin of the species that never sheds, with the same draws. |

**Measured, not asserted** (ignored tests `measure_bias_and_exponents` and `measure_production`):

| Case | Realized / formula, pooled |
|---|---|
| Oracle species, equal exponents | wood 0.965, section 0.918, mass 0.941 (the partial-presence shortfall of decision 10; a square makes it larger) |
| Oracle species, branch 2.4 and twig 2.0 | section 1.013, **mass 0.654** |
| Oracle species, branch 2.0 and twig 2.6 | **section 0.835**, mass 0.851 |
| Beech, 25 years, 4 seeds, by PA and five-year age band | kept wood 0.89 to 1.04 and section 0.88 to 1.01. Every band but one is 0.96 to 1.04; the exception is young PA 9 (369 made) at 0.89 and 0.88. Cohorts that are all shed are 0 on both sides. |
| Spruce, 25 years, 4 seeds | kept wood 0.96 to 1.00, section 0.95 to 1.00, branchlets included: before shedding was in the formula, they read 0.22 to 0.86 (STEP1.md) |
| Table cost on first use | beech 0.02 s, spruce 0.11 s |

## 3. Size

| What | Lines |
|---|--:|
| `kept.rs` + `kept/path.rs` + `along.rs` additions | about 750 |
| `steps.rs` (a restructure: `events.rs` lost about 220) | 216 |
| `kept/tests.rs` | 497 |

The production code is about 1.5 times the upper estimate. `kept/path.rs` is 435 lines, over the 400-line rule. It splits naturally at the end weighing (failing, aborting, ending its age) against the unit and zone work.

## 4. Not built: the decisions needed

1. **Sections across exponents, and mass at the true exponent (Astra's note).**
   - The table sums every section at one exponent, and the mass at the pipe model's square.
   - The production species match within a few percent, but the oracle species shows the error once exponents differ: section 0.835 and mass 0.654.
   - **Doing it properly:** carry E[S] and E[S²] of each subtree's section in its own exponent, and convert to the bearer's exponent by the second-order delta method, E[S^r] ≈ μ^r + r(r−1)/2 · μ^(r−2) · Var. Mass then follows from the radius at its own exponent.
   - **Estimate:** 150 to 250 more lines, since the second moments need each zone's pair term (the generating function's second derivative).
2. **A conservative extent bound beside the RMS reach (Astra's note).**
   - Poisson node counts are unbounded, so a hard bound would be infinite.
   - **The proposal:** the longest path any wood could take, with every bud made, every unit grown, and each zone's node count at its 1 − 1e−9 quantile. That is a probabilistic contract with a stated tail, not a certainty.
   - **Estimate:** about 60 lines on `steps.rs`.

Not touched: decision 12 (on hold), the oak's light input to the closed form (decision 5), the leaves of the last cycle for drawing stand-ins.

F2 (#158) is still open, so master is not merged into this branch.
