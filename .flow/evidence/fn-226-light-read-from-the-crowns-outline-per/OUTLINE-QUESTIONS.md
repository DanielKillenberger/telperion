# fn-226 task 1: design questions before R2 and R3 (worker, 2026-10-08)

R2 and R3 are not built. The questions below are beyond the spec and the owner's inline rule (2026-10-08, recorded in the spec under R2 and R5), so they go to the host. R1's findings are in R1.md. Each claim below about the code was checked against it at `e193ea55`.

## Q1. Where A's fold lives: no existing reduction carries it in time

- **What A needs each cycle:**
  - the outline's folds (height, crown base, widest reach and its height, shoulder);
  - the cycle's total leaf area.
- **What the existing per-cycle reductions carry:**
  - **Balance (i):** light, upkeep and living status (`balance.rs` `Totals`), reported up at the start of the cycle.
  - **Allotment (ii, iii):** the lit presence and the per-PA sums (`stepped/phases.rs`).
  - Neither carries a position.
- **Why riding them does not work in time:** both already read each bud's light (Q = scale × running × light; lit presence = presence × light). The light must therefore exist before they run. An outline that rides them arrives too late for the cycle that needs it.
- **Options:**
  - **(a) Ride phase (i)'s up and down exchange with a one-cycle lag.** Each root folds its last-laid leaves into its (i) report, and the root of the tree hands the outline down with the shed flag. Buds then read the outline of leaves two cycles old, where today's lattice reads leaves one cycle old.
    - No added synchronisation.
    - The light changes by a cycle's growth.
  - **(b) One added synchronisation per cycle, after layout.**
    - This is today's lattice point, carrying about 6 scalars per root in place of every leaf.
    - It is the extra pass the owner's rule counts against A.
- **The shoulder is a two-pass fold.** Its points (p, q) need the base, height, radius and fullness first (`shoulder.rs` `crown_point`). The least shoulder of the set is then the largest of each point's own least shoulder, since `p^s + q^s` falls as s rises.
  - So even (b) needs a second exchange, or the shoulder taken with the previous cycle's shape (a lag).
- **Recommendation:** (a), with the shoulder's (p, q) read against the previous cycle's shape. It adds no pass, and the lag is a cycle's growth. Report A as "1 fold riding (i), lagged one cycle" in the comparison.
  - If the host wants no lag, A costs (b), the extra synchronisation, and that is A's mark.

## Q2. What is folded, and the outline's frame

- **Today's folds are on the pipeline's dressed nodes (y up).** They are `telperion-core` `Folds`, `grown_envelope` and `shoulder.rs`, and they run once, after the build.
- **The engine has no fold of its own.** Its frame is z up (`light.rs` directions).
- **Recommendation:** fold over the cycle's leaves (the same population the lattice deposits, 158k a cycle on average at 80 s7). Use the same crown rule (off the stem), and reach off the seed's vertical through the origin.
  - The cost is per leaf, so it rides the gather that already happens.
  - Folding every kept phytomer would cost more than today's deposit, which is 6 to 7 % of the build.
- **Unknown:** whether the leaves' outline is close enough to the dressed tree's outline. It need not be: it is the light's outline, not the pipeline's envelope.

## Q3. The outline's volume: the leaf area density's denominator

The outline is a solid of revolution, `r(z) = R · quadrant(p, s)`. Its volume is exactly `π R² (height − base) · Γ(1+1/s) Γ(1+2/s) / Γ(1+3/s)`. Contract A has no Γ, and `quadrant` is f64 in core.
- **Options:**
  - **(a) Quadrature in Q32:** a fixed n-point midpoint rule over p, with `quadrant` built on contract A's `log2` and `exp2_neg`. Deterministic, once a cycle. Its cost is n × 2 transcendentals.
  - **(b) A fixed table of the Γ factor over log s** (the shoulder ranges from 1/16 to 64), interpolated in Q32.
  - **(c) Ignore the shoulder for the volume:** two half-ellipsoids, `(2/3) π R² (height − base)`.
- **Recommendation:** (a) with n = 64. It is exact enough, keeps every operation within the contract, and runs once a cycle.
  - Under B the volume is a per-age constant, so its cost does not matter.

## Q4. The path length per bud per direction

A ray from the bud along a sky direction meets the outline. The path is the chord inside the outline: from the bud, or from where the ray enters the outline when the bud lies outside it (below the crown base, or beyond it).
- **The surface is convex for s ≥ 1.** The measured shoulders are 2.8 to 26.5 (`shoulder.rs`, fn-198 task 6), so the ray leaves it once.
- **Options:**
  - **(a) The exact super-ellipse.** Bisection on the ray parameter, a fixed number of steps, each a `quadrant`. That is about 9 directions × 2 roots × 30 steps × 2 transcendentals per bud: about 1,000 transcendentals per bud.
    - At about 8 ns each, that is about 8 µs per bud. Over 4.0M bud reads at 80 s7 it is about 32 s on one core, or about 4 s on 8 cores.
    - That is above the lattice's whole 1.6 s.
  - **(b) The outline replaced by a stack of K frustums.** The profile is sampled at K heights once a cycle. The chord of each frustum is a quadratic with closed-form roots (an `isqrt`), with no transcendentals per bud. K = 16 gives at most 16 quadratics per direction.
  - **(c) An ellipsoid-pair outline (s = 2):** one quadratic per direction. It drops the shoulder.
- **Recommendation:** (b) with K = 16.
  - It keeps the shoulder and is integer-exact per bud.
  - It costs about 9 × 16 small quadratics, about 1 µs or less per bud.
  - It needs the host's word that the K-frustum profile is "the outline".

## Q5. The sky directions

- **Recommendation:** today's nine (`Light::directions`: overhead, four at 60°, four at 30°). Their shares come from the sky row, so the existing rows mean what they mean today.
  - The cost is 9 chords per bud.
- **Cheaper alternatives:**
  - 5 (overhead and the 60° ring, the 30° ring's share folded into it);
  - 1 (overhead, with the sky's shares folded in).
  - Either changes what the sky row does, so it is the host's call.

## Q6. B's outline does not exist in the closed form; it must be derived, and it is light-neutral

- **What the closed form provides:**
  - expected counts per PA and cycle (`expected_counts`);
  - kept tables with sections, masses, moments and reach bounds (`closed_form/kept.rs`);
  - the stand-ins' wood moments, Σl, Σl(x·h) and Σl|x|² about a bud's base (`closed_form/envelope.rs`).
- **What it lacks:** height, crown base, widest reach, fullness or shoulder.
- **Its stated assumptions:** straight axes and even azimuths. It is light-neutral (`expected_kept` returns none for a species that reads light).
- **Options:**
  - **(a) Derive the outline from the closed form's moments.** Extend the stand-in recursion with vertical and horizontal moments of the whole tree's leaves at each age, then fit the outline by moments (for example, height and widest reach from the mean and the second moment, with a fixed shoulder).
    - It is new derivation work.
    - It gives the unlit tree's outline: the lit oak is 25 to 45 % wider at 80 (R1: lit 24.0 / 21.3 m against unlit 18.0 / 17.9 m at 80 s1). The light would read a column the tree does not have.
  - **(b) A per-species, per-age outline table measured from grown trees.** Constants per species and age fit the owner's inline rule exactly.
    - It is circular if measured from the lit build it replaces.
    - It is a fitted table, not the closed form.
  - **(c) B's outline from the species' own envelope rows** (`Envelope`: height, spread, crown base, fullness, shoulder) scaled by age.
    - Today these are the pipeline's dressing rows. The engine does not read them, and they describe the mature crown.
- **B's leaf area density needs the cycle's leaf area as a constant too:** the expected leaf area at that age, from expected counts × leaf area × expected scale. It is also light-neutral: the lit oak grows 0.62 to 0.87 times the unlit's phytomers.
- **Recommendation:** none of these yet. The host should first decide whether a light-neutral expected outline is acceptable for a species whose light makes its width.
  - If it is, take (a) and measure it.
  - If not, B is not the closed form but a table, which is (b) or (c).

## Q7. Synchronisation points: what changes under each model, from R1

| Per cycle, lit oak | Today (lattice) | A, option (a) | A, option (b) | B |
|---|---|---|---|---|
| (i) balance up and down | yes | yes, carrying the fold | yes | yes |
| (ii) demand up and down, (iii) normalisers merge | yes | yes | yes | yes |
| Light exchange after layout | every leaf up, the field down | none | 6 scalars a root up, the outline down | none |
| Light passes | gather, box, deposit, sweep, sample, write back | inline in the bud's step | 1 fold and exchange, then inline | inline in the bud's step |
| Re-lay every 10 cycles | yes | open (Q8) | open | open |

**Under every model the lit oak still synchronises twice a cycle,** for (i) and for (ii)/(iii). "Nothing crosses between roots and the structure above per cycle" (R3) holds for light only, not for the tree.

## Q8. Delete before optimise (docs/principles.md): the re-lay

- **What R1 found:** the re-lay is 16 to 27 % of the lit build, and the largest light pass at 80. Turned off, the measures move by 0 to 5 %, and the tree builds 18 to 29 % faster at 80.
- **Why it matters here:** an outline read is coarser than the lattice, so it may need no sagged positions at all.
- **Question for the host and owner:** is the re-lay's deletion in this spec's scope? R2 and R3 would then be measured without it. It is not in the spec's requirements, so the worker has not acted on it.
