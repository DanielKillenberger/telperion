# fn-209 step 1, decisions 15 to 17: the delta method, the extent bound, the split (worker, 2026-10-05)

A dispatched worker wrote this report. Decisions 15 to 17 are built. No coupling is built. Paths are relative to `crates/telperion-space/src/closed_form/kept/`.

## 1. Decision 15: sections and mass across exponents

**What a table carries now:** each subtree's base section in its own PA's exponent, as its mean and mean square (`SECTION`, `SECTION2`), and the chance that it is kept, `k[t]`, beside `a[t]`.

**How a bearer hands it over** (`section.rs`):
- A bearer of another exponent sums E[S^(e_p / e_c)] by the second-order delta method, taken among the subtrees kept, so the shed (section none) do not widen the spread.
- The result is held on the side of μ^r that Jensen's inequality puts it, so a wide spread never turns it over.
- A zone's section square comes from its node count's generating function and its first two derivatives (pairs of borne subtrees in two nodes); units, zones and ends add their cross terms.

**Mass at the true exponent** (`mass.rs`):
- A phytomer weighs l·r², where r = s·S^(1/e) + (1 − s)·established.
- S is the section beyond it: its own node's, the nodes beyond in its zone (on average (N + 1)/2, and by place for the moment), every zone after it, and the axis's end.
- r² is taken by the delta method.
- **A relay's section enters at its node,** the `relay_at` share along the last unit, as `geometry::relay_point` places it: phytomers before it carry it, those after do not. Letting it enter at the tip had cost 20 % of the trunk's mass on the relayed oracle tree.

**Oracles (tests in `tests.rs`):**

| Test | Result |
|---|---|
| `a_deterministic_tree_keeps_what_the_formula_keeps`: shedding twigs, trunk carried on whole, and trunk relayed half way along every unit | **Exact to 1e-9 in both,** for wood, along, square, mass, mass moment and section, ages 1 to 9 |
| `branches_keep_on_average_what_the_formula_keeps`: exponent 2 | within 4.5 standard errors, every birth group and P(T ≤ t) |
| `branches_of_another_exponent_keep_what_the_formula_keeps`: branch 2.4 over twig 2.0, and branch 2.0 over twig 2.6 | within 4.5 standard errors, every birth group and P(T ≤ t) |

**Pooled, realized over formula, on the oracle species:**

| Exponents | Section | Mass | Before the delta method |
|---|--:|--:|---|
| Branch 2.4, twig 2.0 | 0.894 | 0.944 | 1.013 and 0.654 |
| Exponent 2 (equal, and branch 2.0 over twig 2.6, pooled) | 0.913 | 0.942 | branch 2.0 over twig 2.6: 0.835 and 0.851 |

The equal-exponent shortfall is decision 10's partial presence: section 0.918, mass 0.941 (STEP1-B.md). The mixed cases now sit with it, where they had swung either way.

**Production** (beech and spruce at 25 years, 4 seeds, every lateral made, `tests/measure.rs`, ignored):

| Species, PA and age band | Wood | Section | Mass |
|---|--:|--:|--:|
| Beech, all bands but one | 0.99 to 1.04 | 0.96 to 1.01 | 0.99 to 1.01 |
| Beech, young PA 9 | 0.894 | 0.883 | 0.837 |
| Spruce, every band | 0.96 to 1.00 | 0.95 to 1.00 | 0.95 to 1.01 |

Tables cost 0.04 s (beech) and 0.10 s (spruce) on first use.

## 2. Decision 16: the extent bound (`bound.rs`, `Kept::reach_bound`)

The longest path from a bud's base to any wood its subtree can grow: every bud made, every unit the schedule allows, every relay and continuation taken, and each zone's node count at its 1 − 1e−9 quantile.

**Its tail, named in its doc:**
- Each zone's draw passes its quantile with chance at most `TAIL` = 1e−9.
- So over a subtree whose units draw Z zones, the bound holds with chance at least 1 − Z·1e−9.
- A uniform node law is bounded outright.
- A path is the farthest wood can stand from its base, whatever bends it.

**Test,** `no_wood_reaches_past_the_extent_bound`: no lateral's wood reaches past its bound. The species tested are the stochastic oracle species (30 seeds) and the beech and spruce at 20 years as they are, bent and sagged.

**How tight it is (measured):**
- Beech: realized reach over bound, median 0.80, max 1.00.
- Spruce: median 1.00, max 1.00. Its laterals' node laws are fixed, so the longest path is the one grown.

## 3. Decision 17: the split

| File | Lines |
|---|--:|
| `kept.rs` | 272 |
| `path.rs` | 334 |
| `mass.rs` | 184 |
| `ends.rs` | 125 |
| `bound.rs` | 108 |
| `section.rs` | 75 |
| `tests.rs` | 364 |
| `tests/bound.rs` | 62 |
| `tests/measure.rs` | 120 |

All are under 400.

## 4. Checks

- **The trees are unchanged to the bit:** every species at 80 years, seeds 1 and 7, neutral and lit, hashes as F2 recorded.
- **Tests:** the `telperion-space` closed-form, oracle and kept tests pass, and clippy is clean on the new code. The workspace gate was not run: it is the end-of-task gate.
- **Approximations, as `kept.rs` names them:**
  - presences whole (decision 10);
  - times whole cycles;
  - axes straight and azimuths even (envelope moments);
  - the delta method's second order;
  - within a zone, nodes carry equal sections on average, though a place-0 lateral's PA follows its rank;
  - sleeping buds independent of their bearer's axis.

F2 (#158) is still open, so master is not merged.
