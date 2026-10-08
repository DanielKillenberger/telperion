# fn-209 F1: the fine-root record and its decision contract (task 1, worker, 2026-10-06)

Task 1 builds Astra's changes 1 and 2 (ASTRA-REVIEW-FINE.md, sections 2 to 4) on the CPU reference executor. This file has four parts. The first lists every continuous-to-discrete decision a fine root's growth, shedding, lay and conversion takes, with its site, its representation and whether it moves trees. The second is the canonical record. The third covers the work partition. The fourth is the predictor against realized sizes. Line numbers are as of this branch's task-1 commit.

## Answer in brief

- **Fine roots grow from their records alone, on the CPU reference executor:** `crates/telperion-space/src/fine/` (`grow_fine`, `fine/executor.rs`).
  - The tree is the same to the bit at every cut (N in {0, 256, 1,024, 4,096, ∞}).
  - It is the same in every item order (forward, reverse, shuffled), in every chunking (1, 7 or 64 items a piece) and on one core or four.
  - It equals the tree P2's chain grower grows whole (`fine/tests.rs`, digest over every field).
- **The cut is a work partition and nothing else (Astra's change 2).** A chain does the same work wherever it runs:
  - it is grown from its bud alone (P2);
  - it is shed by its own axes and its borne buds' committed extinction;
  - it is resolved by option 4 against its bearer's laid frame and its own committed boundary.
  - Moving the cut therefore cannot change the tree, and the tests hold this.
- **Draws now decide as integers (D1 to D4):** each draw is 53 bits, compared with a probability's canonical threshold `ceil(p · 2^53)`.
  - The integer decision is the real one to the bit, so **no tree moves**.
  - Evidence: all 28 dumped trees match the base exactly, with tips and radii identical (four species, ages 40 and 80, seeds 1 and 7, whole and P2).
- **Open, for the host: the arithmetic a threshold is computed in.** It concerns D5, D6, D7, D10, D11, D12, D15 and D18. The executors take the same decision only when they compute the same threshold. Two candidates were measured on the four species. Neither changes topology:
  - **A. IEEE binary64 with pinned `libm` (0.2.16, which `telperion-core` already pins).** The GPU emulates f64 for the decision chain. Tips move at most 9.7e-13 m and radii at most 2.6e-9 relative.
  - **B. Integer time ticks of 2^-20 cycle plus fixed-point thresholds.** Tips move at most 2.5e-5 m and radii at most 1.6e-2 relative, on the spruce's and oak's woken buds. The fixed-point part was not built.
  - A is recommended (see "The open question"). Nothing past D1 to D4 is coded until the host decides.
- **The predictor** (the closed form's expected kept phytomers, a new `count` column of the kept tables, held against grown trees by `kept/tests.rs`). Its median realized/predicted ratio among non-empty roots is 1.06 to 1.14 for the beech and spruce, and 0.71 to 0.98 for the oak.
  - **Empty roots:** 9 to 21% for the spruce, 34 to 56% for the beech, and 25 to 58% for the oak.
  - **Roots that realize more than N:** the spruce has at most 3, up to 1.2N. The beech has up to 122 at 80, up to 2.2N. The lit oak has up to 347, up to 4.7N.
  - An overflow rule is needed (task 2), as Astra predicted.

## Part 1: the decisions

The representation is the one specified for both executors. "Status" says what this task built.

| # | Decision | Site | Inputs | Representation | Moves trees | Status |
|---|---|---|---|---|---|---|
| D1 | Every draw | `lineage.rs:66` (`Key::draw`), `:39` (`mix`) | the lineage key (u64) | 53-bit integer `mix(key ^ c) >> 11`; `mix` is SplitMix64's finalizer (wrapping u64 multiply, xor-shift), exact on a GPU as u32 pairs. A decision is `draw < threshold(p)`, `threshold(p) = ceil(p · 2^53)` (0 for p ≤ 0 or NaN, 2^53 for p ≥ 1), which equals `draw / 2^53 < p` exactly (`lineage.rs`, `threshold`). | no (measured: 0 of 28 trees) | **built** |
| D2 | A zone's node count | `lineage.rs:163` (`nodes`): uniform `:172`, Poisson `:183` | draw; the zone's law | Integer draw against the law's cumulative thresholds, a table per PA and zone computed once per species point on the CPU (uniform `(j - min + 1)/span`, Poisson's running `mass *= mean/k` in that order), and supplied. | no | **built** (the CPU evaluates the same sequence per call) |
| D3 | A bud's PA (lateral and dormant) | `lineage.rs:147` (`bud`, the test at `:152`); `grow/unit.rs:42`; `dormant.rs:74` | draw; the zone's lateral or dormant probabilities | Integer draw against the running sums of the probabilities in index order, a table per PA and zone, supplied. | no | **built** |
| D4 | Node order along a unit (acrotony) | `grow/unit.rs:47`, `:54` | the first place's draw | Sort by the integer draw, descending; equal draws keep the node's index order (stable). | no | **built** |
| D5 | A unit's survival | `grow/advance.rs:53`, `:56`; `grow.rs:368` (`shared`) | viability (species), the unit's share of its cycle (D7) | Integer draw against `threshold(viability^share)`. At share 1 (whole lifespans, no woken first unit) the threshold is a per-PA table. Otherwise `pow` runs per bud. **The open question.** | A: no; B: by D7's share | open |
| D6 | Abortion after a unit | `grow/advance.rs:97` to `:102`; `species.rs:421` (`abortion_at`) | abortion, its rise, time grown and exposure (D7) | Integer draw against `threshold(1 - (1 - abortion_at(grown))^exposure)`. A per-(PA, whole cycles) table where `abortion_rise = 0` and exposure is whole. Otherwise `pow` runs per bud. **Open.** | A: no; B: by D7 | open |
| D7 | An age's end and what carries on within the cycle | `schedule.rs:60` (`ends = left <= room + WHOLE`), `:116` (`same_cycle`), `:23` (`WHOLE = 1e-12`); `grow/shoot.rs:316` (`used >= 1.0`) | lifespans (species, real), sleep fractions (D11), times spent | A: IEEE f64, evaluated in the reference's source order, no fused multiply-add; `WHOLE` stays. B: integer ticks of 2^-20 cycle, so `WHOLE` becomes exact comparison; lifespans in ticks (every shipped lifespan is whole, so none moves). **Open.** | A: no; B: via woken buds (measured) | open |
| D8 | An age moves on | `grow/stop.rs:57` | continuation (species) | Integer draw against a per-PA table. | no | **built** |
| D9 | A stop's relay (aborted, ended, failed) | `grow/stop.rs:119` to `:123`, `:154` to `:158` | relay shares (species) | Integer draw against a per-PA table. | no | **built** |
| D10 | An age all but stopped also grows its relay (host decision 11) | `grow/stop.rs:61`, `:62`, `:71`, `:72` (`made < 1.0 && relayed > 0.0`) | the move draw's lead in log-odds; the window from `Windows::wood_at` (`presence.rs:57`): the closed form's log lengths, interpolated by the share of the cycle used | **A presence decides topology here.** `made < 1` ⇔ `logit(go) - logit(u) < window(wood_at(...))`, which has `ln` of the draw and `exp` of the interpolated log length. A: as written, in f64. B: a second integer threshold `σ(logit(go) - window)`, computed in fixed point. `relayed > 0` is `u < relay` (D9), but through `below()`; a draw one ulp under the bound can give a lead of 0 in f64, so the specified form is D9's integer test. **Open.** | A: no | open |
| D11 | A sleeping bud's waking time; whether it wakes within the age | `dormant.rs:75` (`wake`, `:25` `ln_1p`), `:76` (`years >= age - cycle`), `:79`, `:91` (whole years and the sleep fraction) | draw; the zone's delay and rate | Waking within the age: an integer threshold `ceil(F(age - cycle) · 2^53)` per zone and whole remaining years, supplied (exact under either option). The fraction is time state: A computes `log1p` in pinned `libm`; B truncates to ticks. **Open.** | A: no; B: measured below | open |
| D12 | The stage a woken bud wakes in | `dormant.rs:152` (`slept < lifespan`) | the sleep time (D11), lifespans | As D7. | as D7 | open |
| D13 | A woken bud's bearer: which axes carry it | `dormant.rs:125` | integer `apex_end`, cycle | integer | no | as is |
| D14 | A lateral certain to be shed is not grown (fn-210 lever 1) | `grow/shoot.rs:279` | horizon (a maximum of `delay + 1` along the bud's line), `lasting` (species), cycle, age | Equivalent integer form: grown unless `cycle <= age - last - ceil(horizon)`. The record carries the horizon. Every shipped delay is whole, so this is exact either way. | no | as is (exact under A; integer under B) |
| D15 | P2's shedding of a chain's laterals and relays | `alone/shed.rs:57` (threshold cycle), `:86` (`until > t`), `:76` (product of borne buds' `dead_by`), `:92` to `:97` (conditioned on the bearer, decision) | each borne bud's extinction curve (closed-form table, supplied), the record's context | The threshold cycle and `until` are integers. The product is taken in `chain.buds` order (the chain's kept axes in index order, then the buds that woke on it). A: f64 in that order. B: fixed-point sums of `-ln dead_by` (exact and in any order), with the conditioning in fixed point. The decision is D1's against `threshold(1 - lives)`. **Open** on the product's arithmetic. | A: no | open |
| D16 | The carbon balance's shedding | `grow/balance.rs:82` | light | Light-reading species only. | – | **F2b** (refused here) |
| D17 | Whole-tree shedding by fade (`fade <= 0`), and `standing` | `shed.rs` (`fades`), `:36` | presences, living times | On the fine path under realized mechanics (task 6). Host decision 8: every presence the fade reads is Q32 and the fade is an integer test (`fixed::presence`; "Host decision 8 built"). | no (measured: 0 of 28 trees) | **built** (CPU) |
| D18 | A relay's node on its parent | `geometry.rs:55` (in `scale`), `:250` | the parent's last unit's phytomer scales (products of presences) against `relay_at` | **A presence decides an index:** the node is where the cumulative scale first reaches `relay_at` of the unit's. A: f64 as written. B would have to put every presence on the path in fixed point, the logit leads included. **Open.** | A: no | open |
| D19 | An axis whose base stands below the ground is refused | `geometry/single.rs:56`; `geometry/lay.rs:285` (trunk only) | laid positions | A geometry test. Within a fine root it reads float32 positions on the GPU. **Design question 3.** | – | open (host) |
| D20 | A vector of no length | `structure.rs:26` (`length > 1e-12`) | geometry | Degenerate guard. Geometry within fn-125's screen; it matters only at exact degeneracy. | no | as is |
| D21 | The girth fit's branches | `bounded/girth.rs` (`fit`: `r0 <= 0`, `base <= 0`, `|log| > 1`) | profile radii, committed base | The zero tests read exact zeros from supplied rows. The `|log| > 1` branch is continuous across its switch. | no | as is |
| D22 | The converter's merges and culls | `crates/telperion-render/examples/space/tree.rs:171` (`radius >= FINEST` drops the rest of the axis and all it bears), `:177` (a relay's internodes behind its node), `:178` (`length < SHORTEST` merges), `:212` (only a whole axis is carried on), `:226` (`STRUCTURAL` twig or branch), `:64` (codominance weight) | laid geometry, radii | Geometry tests that set output counts and identities. **Design question 2.** | – | open (host, F4's converter) |
| D23 | The cut | `fine.rs` (`grow_fine`) | the bud's PA and birth; its expected kept phytomers from `closed_form/predictor.rs` (task 7), the closed form's `count` to the bit | CPU only, before growth. No executor reads it. | no (the partition only) | **built** |
| D24 | A tree's budget | `grow/unit.rs:58` | integer counts | P2 clones the rules per chain (`grow/alone.rs`), so its budget is per chain, not per tree. A tree-level budget across items is part of task 2's bounded execution. | no | noted |

Continuous values that set geometry and never topology stay within fn-125's screen on the GPU (50 µm max, 10 µm RMS). They are the presences (`presence.rs`; `lineage.rs`, `below` and `above`), scale and fades, girth, lay, sag, dominance and roll. Two exceptions sit in the table: D10 and D18 read presences to take a discrete decision.

### The open question (host): the arithmetic of a threshold

D5, D6, D7, D10, D11, D12, D15 and D18 compute a threshold, or an index, from per-bud state with `pow`, `exp`, `ln` and `log1p`. Two executors take the same decision only if they compute the same threshold. Astra (section 2) asked for one specified representation, with its rounding, overflow and evaluation order. The closed form's tables (rows, extinction curves, windows' log lengths) are computed once per species point on the CPU and uploaded, so they need no shared arithmetic. Only per-bud arithmetic does.

| | A: IEEE binary64, pinned `libm` | B: integer ticks and fixed point |
|---|---|---|
| Specification | IEEE 754 binary64, round to nearest, the reference's evaluation order, no FMA contraction (Rust contracts none). Transcendentals are `libm` 0.2.16's pure-Rust algorithms, which `telperion-core` already pins. | Time in u32 ticks of 2^-20 cycle (enough for an age below 4,096). Probabilities as Q53 thresholds. Log-probabilities in fixed point. `exp2` and `log2` by a specified integer algorithm. |
| Covers | Every decision as written: D10's and D18's presences, and any future one. | Each decision re-specified by hand. D10 and D18 need the presences, the logit leads included, in fixed point. |
| GPU cost | Soft f64 for the decision chain only, which is a few transcendental calls a unit. Geometry stays in f32. Not measured (task 2). | Native 32-bit integer operations, with u64 as pairs. Cheaper per operation. |
| Trees on the four species (measured, `raw/contract/*-vs-base.md`) | Topology identical in all 28 trees. Tips move at most **9.7e-13 m**. Radii move at most **2.6e-9** relative (oak 80). | Ticks only (waking times truncated to 2^-20): topology identical. Tips move at most **2.5e-5 m** and RMS at most 1.8e-7 m (spruce 80, seed 7). Radii move at most **1.6e-2** relative, on a woken bud's first unit. The beech and palm are unchanged (no sleeping buds). Fixed-point thresholds were not built; they would flip a decision only within about 1e-12 of its bound. |

**Recommendation: A.**
- It moves no tree by more than a picometre.
- It covers the two decisions that read presences (D10, D18), which B would have to rewrite in fixed point along with the presences that feed them.
- Every future decision is covered without a new specification.
- Its cost is soft f64 on the GPU for the decision chain. Task 2 measures that against B's integer arithmetic before the choice binds, and B remains the fallback if A's cost fails the R2 budget.
- What A needs in code: per-bud `powf`, `exp`, `ln`, `ln_1p` and `exp_m1` calls in the decision chain (`grow.rs:370`, `species.rs:425`, `dormant.rs:25`, `:364`, `lineage.rs:113`, `:141`, `presence.rs:50`, `:62`, `:84`, `:85`) become `libm`'s. That is `raw/contract/libm.patch`, measured above and not committed.

### Further design questions (host)

1. **The arithmetic** (above): A or B.
2. **The converter's merges (D22).** `FINEST`, `SHORTEST`, "behind its node", `STRUCTURAL` and codominance decide output counts and identities from laid geometry, so two executors within 50 µm can disagree at a bound. The options:
   - (a) decide them on the reference's arithmetic, which needs geometry emulated as well as decisions;
   - (b) make them draw-time choices that never change identities (every phytomer a node; merging is a level of detail);
   - (c) decide them from record and integer data, for example the committed radius rather than the realized one.
   The converter is F4's (`tree::convert` is example code), so this may belong to fn-198.
3. **The ground refusal inside a fine root (D19).** Today a laterally borne axis whose base is below the ground refuses the tree. Within a fine root that test reads GPU float32 positions. The options:
   - (a) the refusal belongs to the structure above the cut only, and a fine root clamps or carries no refusal;
   - (b) it is checked on the CPU reference after the fact.
4. **Where the executor lives.** The spec puts fine growth "inside fn-125's expansion". `telperion-core`'s pipeline does not depend on the engine (integration is F4, fn-198), so the reference executor lives in `telperion-space` (`fine/`). `docs/pipeline.md` says so and names F4 as the move into `pipeline::executor`. Confirm this, or move integration ahead of task 2.

## Part 2: the record (`crates/telperion-space/src/fine/record.rs`)

`FineRoot` is written once by the structure above the cut, from the bud and its laid bearer. The executor reads nothing else; no field is read back.

| Field | Holds | Requirement (Astra section 3) |
|---|---|---|
| `bud` (P2's `Bud`, `grow/alone.rs:34`) | The first axis as made: lineage, PA, birth, origin (node, slot, whorl, woken), vigour. Its birth draws' presences. `root`: the lineage its chain of ages began from. `horizon`: what it and the laterals or relays it stands on keep it for past its last growth. `wake`: a woken bud's sleeper, the time it aged asleep, the presence that carried its bearer to the waking, its cycle, and the share of its size its bearer kept. | everything P2's `Bud` carries; dormant and relay history; ancestor termination (the horizon) |
| `context` | Where the bearer was kept only on its borne buds' chances: the chance the bearer's other buds had all died. The root's own keeping is conditioned on it (D15). | ancestor survival |
| `record` | The bud as its bearer's girth and sag read it: origin, PA, birth, lineage, vigour. | |
| `attachment` | The bearer's index, PA, lineage and birth; the node; the bearer's laid frame there before bending and as laid (the node's phytomer: tip, heading, side, rank, scale, size; and the point below it). The executor holds it as a one-phytomer stub. | finalized attachment frame |
| `committed` | The base section handed to the bearer at the bearer's exponent, with the bearer's retained share of the root's shed pipes (oak `retained`); the mass and its first moment along the heading; the base radius the root's girth is fitted to. All are rows times powers of the base scale (`bounded/girth.rs`). | committed section, mass and first moment; retained-pipe transfer; immutable attachment radius |
| `predicted` | The expected kept phytomers that put it below the cut. | the cut |
| light | **Named, not carried.** A species any bud of which reads light needs DESIGN-LIGHT.md's history-dependent aggregate state, which is F2b's (fn-211). `grow_fine` refuses it by name (`grow/alone.rs`, `rules`), and a test holds the refusal. | aggregate state (F2b) |

The record carries its own attachment and committed boundary. A root therefore expands the same in any order, on any core, and without the tree above the cut in memory. The bearer reads only `committed`, never what the root realizes. The tests show the root's base standing exactly on its bearer's laid node, before and after sag (`a_loaded_junction`).

## Part 3: the partition (`crates/telperion-space/src/fine.rs`)

1. **Above the cut, on the CPU.** P2's chain loop (`alone.rs`, `chains`, shared with `grow_alone`) grows chains breadth first. A borne bud whose `predicted <= cut` is recorded and handed on instead of queued. The chains above the cut are scaled and resolved by option 4 on every core.
2. **Records.** Each handed bud's `FineRoot` is read off the laid tree above the cut (`fine.rs`, `record`).
3. **Expansion** (`fine/executor.rs`, `expand`). Each root is one item: its own tree, the bearer's stub as axis 0, then its chains grown by the same loop, scaled, settled and resolved by option 4 with the stub laid. The items run in the plan's order, `chunk` to a piece, on `threads` cores. Each result is stored by the root's index, so the assembly order is the record order whatever ran first.
4. **Assembly.** The tree above the cut, then each root's axes in record order, the root's first axis on its bearer's node.

Tests (`fine/tests.rs`, `fine/tests/*.rs`; 12 tests, about 15 s):

- `a_tree_is_the_same_at_every_cut`: beech and spruce, age 30, seeds 1 and 7, N in {0, 256, 1,024, 4,096, ∞}. Each equals the whole P2 tree's digest, and the cuts move work (the axes above the cut differ).
- `a_tree_is_the_same_in_every_order_and_chunking`: forward, reverse and shuffled; (1 a piece, 1 core), (7, 4) and (64, 4); more than 100 roots.
- `a_species_that_reads_light_is_refused` (the oak).
- `draws.rs`, threshold-adjacent draws:
  - `Draw::under` at d, d + 1 and one ulp, the integer form against the real one;
  - a bud and a node count exactly at their bands' bounds;
  - a fine root's first unit with its PA's viability set exactly at its survival draw (it fails) and one bit above (it grows), each tree the same at every cut.
- `adversarial.rs`, each the same at every cut as grown whole and each case shown present among the roots:
  - fractional lifespans (the oak, light off, lifespans × 1.37 plus a fraction; woken roots present);
  - relay and dormant buds (woken roots and relays inside roots);
  - ancestors' shedding (roots whose keeping read their bearer's context; the oak, light off, and the spruce);
  - retained pipes (the oak's `retained` 0.5, committed sections handed);
  - a loaded junction (the spruce's sagging limbs carry committed masses, and every root's base stands exactly on its bearer's laid node).

"Changing light" (R1's list) cannot be exercised until F2b carries light into the record.

## Part 4: the predictor against realized sizes, empty roots and item work (N = 1,024)

`cargo run --profile ci -p telperion-space --example cut -- <ages> 1,7 1024`. Raw output is in `raw/cut-40.md` and `raw/cut-80.md` (gitignored).
- The beech and spruce are grown by `grow_fine`.
- The oak reads light. Its sizes are surveyed on the lit tree grown whole (`survey_lit`: light 0.5, 0.5; buds classified by the light-neutral predictor; realized kept phytomers read off the tree), and its item work is read off the oak with its light switched off, expanded.
- "Grown" counts the phytomers an item grew, the shed ones included. "Chain-cycles" sums each chain's cycles of growth. "Axes made" is the most axes an item made, the shed ones included.
- Timings are this machine under load 4 to 6, from another worker's fn-211.1 profiling, on all cores. They are indicative only.

| Tree | Roots | Empty | Above the cut (phytomers) | Realized / predicted p5 / p50 / p95 (non-empty) | Realized over N: roots, most | Σ realized / Σ predicted | Grown p50 / p95 / most | Chains p50 / p95 / most | Chain-cycles p50 / p95 / most | Axes made, most | µs p50 / p99 / most | Wall s |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| beech 40 s1 | 553 | 190 | 2,976 | 0.51 / 1.11 / 1.96 | 2, 1,160 | 0.76 | 45 / 739 / 1,172 | 4 / 95 / 205 | 9 / 189 / 306 | 409 | 43 / 1,747 / 3,168 | 0.02 |
| beech 40 s7 | 975 | 328 | 5,193 | 0.59 / 1.09 / 1.86 | 12, 2,216 | 0.77 | 34 / 629 / 2,381 | 4 / 88 / 309 | 7 / 163 / 618 | 631 | 44 / 2,626 / 5,483 | 0.03 |
| beech 80 s1 | 7,865 | 4,384 | 43,404 | 0.51 / 1.07 / 1.96 | 79, 2,098 | 0.53 | 67 / 576 / 2,213 | 1 / 77 / 348 | 13 / 151 / 581 | 699 | 48 / 3,184 / 7,944 | 0.14 |
| beech 80 s7 | 12,829 | 7,082 | 71,478 | 0.53 / 1.07 / 1.96 | 122, 2,222 | 0.57 | 69 / 605 / 2,348 | 1 / 80 / 362 | 13 / 158 / 620 | 723 | 50 / 3,194 / 15,083 | 0.22 |
| spruce 40 s1 | 21,307 | 2,447 | 12,275 | 0.65 / 1.06 / 1.34 | 0 | 0.95 | 59 / 105 / 965 | 11 / 30 / 404 | 31 / 52 / 456 | 805 | 241 / 609 / 5,468 | 0.26 |
| spruce 40 s7 | 20,444 | 2,263 | 11,684 | 0.66 / 1.06 / 1.34 | 3, 1,197 | 0.95 | 59 / 104 / 1,197 | 11 / 30 / 520 | 31 / 52 / 577 | 1,033 | 228 / 548 / 6,536 | 0.23 |
| spruce 80 s1 | 92,703 | 19,765 | 53,165 | 0.71 / 1.14 / 1.55 | 1, 1,153 | 0.93 | 61 / 99 / 1,153 | 8 / 28 / 476 | 34 / 55 / 547 | 942 | 260 / 721 / 19,723 | 1.02 |
| spruce 80 s7 | 91,262 | 19,229 | 52,348 | 0.71 / 1.14 / 1.55 | 0 | 0.94 | 61 / 100 / 796 | 8 / 29 / 378 | 34 / 55 / 383 | 748 | 251 / 669 / 32,077 | 0.98 |
| oak 40 s1, lit (sizes) | 1,508 | 872 | 3,737 | 0.28 / 0.87 / 1.75 | 30, 4,251 | 0.85 | – | – | – | – | – | 0.39 |
| oak 40 s1, light off | 796 | 196 | 2,973 | 0.36 / 0.99 / 1.71 | 26, 2,715 | 0.86 | 75 / 1,008 / 3,286 | 13 / 119 / 386 | 16 / 257 / 849 | 817 | 188 / 3,575 / 6,731 | 0.03 |
| oak 40 s7, lit (sizes) | 2,698 | 1,541 | 6,648 | 0.24 / 0.82 / 1.61 | 32, 2,782 | 0.76 | – | – | – | – | – | 0.55 |
| oak 40 s7, light off | 1,430 | 303 | 5,657 | 0.32 / 0.98 / 1.89 | 52, 3,092 | 0.93 | 94 / 1,127 / 3,770 | 17 / 135 / 462 | 22 / 285 / 970 | 969 | 271 / 4,588 / 8,864 | 0.05 |
| oak 80 s1, lit (sizes) | 7,284 | 3,933 | 18,491 | 0.20 / 0.78 / 2.26 | 159, 3,579 | 0.60 | – | – | – | – | – | 5.02 |
| oak 80 s1, light off | 5,726 | 2,592 | 18,905 | 0.27 / 0.95 / 1.93 | 201, 3,126 | 0.63 | 78 / 1,083 / 3,794 | 1 / 129 / 453 | 16 / 281 / 981 | 984 | 58 / 4,803 / 9,309 | 0.17 |
| oak 80 s7, lit (sizes) | 17,643 | 8,655 | 45,655 | 0.15 / 0.71 / 1.90 | 347, 4,858 | 0.57 | – | – | – | – | – | 10.85 |
| oak 80 s7, light off | 14,290 | 5,736 | 46,771 | 0.29 / 0.94 / 1.91 | 504, 4,488 | 0.67 | 79 / 1,083 / 5,627 | 5 / 130 / 673 | 16 / 279 / 1,457 | 1,466 | 85 / 5,241 / 14,241 | 0.40 |

What the table says. These are the worker's readings, not decisions.

- **The spruce:**
  - The predictor is tight: p5 to p95 runs 0.65 to 1.55, and almost no root exceeds N.
  - 1.1% of the phytomers stay above the cut, as SPLIT.md found (53k of about 4.8M at 80).
- **The beech:**
  - It is spread (0.5 to 2), and half its roots at 80 come out empty.
  - Σ realized is about 0.55 of Σ predicted at 80. The closed form's shedding is the whole-tree fade, while P2 sheds by committed extinction; P2 measured the beech's order 4 and up about 10% short at 80.
  - 1 to 1.6% of roots realize past N, up to 2.2N.
- **The lit oak** is the widest, at 0.15 to 2.3, and runs up to 4.7N, because the predictor is light-neutral. Up to 2% of its roots exceed N.
- **Item work is heavy-tailed, as Astra warned.** The median item grows 34 to 94 phytomers. The worst grows 2,200 to 5,600 (beech and oak) or 800 to 1,200 (spruce), across up to 670 chains and 1,457 chain-cycles.
- **Bounded execution needs an overflow rule (task 2).** An item that outgrows its budget is split along its chains, each from its bud's record, which the record already allows. That preserves the result, since every chain grows from its bud alone.
- **Woken roots:** the predictor reads a fresh bud's row at the woken bud's PA and birth. It does not condition on the time the bud slept.

## How measured

- **Build:** this worktree, the `ci` profile, its own `target/`. Ryzen 9 5950X, 32 threads.
- **Load:** 1.4 to 7.9 (one-minute), shared with another worker's fn-211.1 profiling. The predictor table's timings are indicative only; every other figure is a count or a bit comparison.
- **Tree moves:** `cargo run --profile ci -p telperion-space --example contract -- dump <file> 40,80 1,7`, then `compare <a> <b>`.
  - Every tree of the four species at ages 40 and 80, seeds 1 and 7, is dumped by lineage: the whole engine (`grow`) for each species, and P2 (`grow_alone`, resolved) for the beech, spruce and palm. The oak grows under light 0.5, 0.5.
  - The base was dumped in a temporary worktree at `f9ac2d6d`, since removed.
  - Candidates A and B were applied there as patches: `raw/contract/libm.patch` and `raw/contract/ticks.patch`.
  - Tables: `raw/contract/libm-vs-base.md` and `raw/contract/ticks-vs-base.md`. The integer draws (D1 to D4) against the base: every tree identical.

## Host decisions on the four open questions (2026-10-06)

1. **Threshold arithmetic: B, integer fixed point.** Time in integer ticks of 2^-20 cycle; every threshold (survival, abortion, time, the borderline relay, waking time, P2's shedding product, the relay node) computed by one specified fixed-point procedure, identical on both executors, compared against the 53-bit integer draw. Not A: A needs the pinned libm's f64 pow, exp and log reproduced bit for bit in software on the GPU, a large port whose one wrong bit flips topology (fn-12's failure mode). B's measured cost (topology identical in 28 trees; tips under 25 µm; radii up to 1.6 % on woken buds) is accepted under AGENTS.md "Generator evolution", shown on stills of the four species.
2. **The converter's merges and culls** never decide identities from laid geometry: identities are botanical (one per kept phytomer); merging or omitting thin wood becomes a draw-time level-of-detail choice (F4 with fn-176). Task 2 compares executors on the botanical tree before conversion.
3. **The below-ground refusal** stays above the cut. A fine root whose conservative extent bound from its attachment can reach the ground is grown on the CPU reference executor: a work-placement rule, botany unchanged, so exact.
4. **The executor** stays in `telperion-space` until F4 moves it under `pipeline::executor`; integration is not brought forward.

**Overflow (task 2):** an item that grows past its bound is split along its chains, each from its own bud's record, preserving the result.

## Host decision 1 built (task 2, worker, 2026-10-06)

`crates/telperion-space/src/fixed.rs` is the contract's arithmetic and `fixed/decide.rs` its per-bud thresholds. The GPU executor mirrors these two files in 32-bit words; nothing else on the decision chain computes a threshold.

**The procedure.**
- **Time** is whole ticks of 2^-20 cycle. A lifespan is read on the grid (`PaState::life`, round to nearest), a waking time is floored to it (D11), and every other time is a sum or difference of those. The CPU holds ticks in an f64, where they and their sums are exact, so its comparisons are the integer ones. A positive lifespan under half a tick is refused by name.
- **A probability** is Q62 (`q62`, floored from the species' f64). Its threshold against the 53-bit draw is `ceil(n / 2^9)`, which is the real comparison on Q62.
- **A log** is Q32 in an i64, saturated at ±2^62. `log2` of an integer takes its mantissa (Q62) and `exp2_neg` its fraction through a table-reduced Q62 polynomial (host decision, 2026-10-07, fn-211 task 9, in place of 32 truncating squarings and 32 roots; the decision contract's job is determinism across executors, which a fixed-order integer polynomial gives as surely, and generator evolution lets the bits move with evidence):
  - **log2's fraction:** `j` is the mantissa's top six fraction bits. `t = floor(m r_j / 2^62) − 2^62` with `r_j = round(2^62 / c_j)`, `c_j = 1 + (j + ½)/64`. Then `log2 m = L_j + floor(t P(t) / 2^62)`, where `L_j = round(−log2(r_j / 2^62) · 2^62)` and `P` is Horner over `(−1)^(k+1)/(k ln 2)`, k = 1..6, from the sixth coefficient down, one floor a product. The sum is floored to Q32 (`>> 30`) and clamped to [0, 2^32).
  - **exp2_neg:** the whole part `k` shifts at the end. Of the fraction, `j` is its top six bits and `g` the rest. `y = floor(g ln 2 / 2^32)` (Q62); `q` is Horner over `(−1)^k/k!`, k = 0..6, from the seventh coefficient down. The result is `floor(E_j q / 2^62) >> k` with `E_j = round(2^(−j/64) · 2^62)`.
  - **The constants** are literal in `fixed.rs` and `contract.wgsl` and generated by fn-211's `log-exp-constants.py` at 120 digits.
  - **Every product** is 64 × 64 bits, its high part kept by a flooring shift; `ln` multiplies a log2 by `ln 2` in Q62.
  - **Accuracy against the reals, no looser than the replaced procedures** (`the_polynomials_are_no_looser_than_the_procedures`): `log2` within 1.000 units of Q32's last place (the squarings: 1.000); `2^-f` within 2.22e-16 relative (the roots: 2.22e-16, both at f64's own limit). `log2(2^n)` is exact.
  - **Cost:** 8 ns where the procedures took 62 to 69.
- A setting that decides alone keeps D1's exact threshold: a whole unit's survival, and an abortion hazard that neither rises nor runs over part of a cycle. So a tree of whole lifespans and no sleeping buds moves only by D10 and D18's presences.

| # | Built as |
|---|---|
| D5 | `survival`: `p^share` as `exp2_neg(-log2 p · share_ticks / 2^20)`. |
| D5, lived time (fn-199 R1) | `lived_q`: the share of its cycle an apex lived whose unit failed with a relay, `-log2(d / 2^53) · 2^20 / l` ticks, `l = -log2 p` (Q32), rounded up, at most the unit's share; a draw at the bound lives the whole unit. |
| D5, D6 below a tick (fn-199 B2) | `lived_fine_q`: the same time floored in 2^-32 of a cycle. What `lived_q` rounds it up by (at most 2^12) is held by the failed unit's relay (`Draws::below`, the GPU's axis word 25), and its first unit's survival and abortion read its share in 2^-32 of a cycle, `share_ticks · 2^12 + below` (`survival_fine_q`, `abortion_fine_q`, scaling by `l s / 2^32`), so a lived time crossing a tick moves no threshold by a step. A tick's share `s · 2^12` gives the tick forms' bits. |
| Carried-on axes (fn-199, host 2026-10-07; geometry, both executors) | A continuation is a relay at blend 0: one function frames, ages, straightens and lays both, a relay differing only through its blend. Phyllotaxis runs on as a phase: an axis carried on adds its parent's `divergence × rank` to its laterals' azimuths, its side not turned. A relay stands at `relay_at` along the last growth unit of the line of axes carried on, read as one axis, blended back towards the unit before by the last unit's extent (Σ over its nodes of unit presence × node presence × lived share, capped at 1), that one towards the one before by its own, down to a whole unit or the line's base (rule 2', `geometry::stand`). A fine root's stub holds its bearer's phase. |
| D6 after a lived part (fn-199 C1) | A failed unit whose failure relays takes the unit's abortion draw (its decision key's `ABORTION` child) after its lived part, over `min(lived, next unit's share)` in 2^-32 of a cycle (`abortion_fine_q`), unless its age ends with the unit. Where it aborts, the failure's relay is made, stopped at once (`APEX_END` its cycle) and the abortion's relay stops it as an aborted apex, growing the rest of the cycle where any is left; else the lived unit's persistence is the one an apex that all but aborted takes. |
| D6 | `abortion`: `1 - (1 - a)^(t^rise · exposure)`; `t^rise` as `exp2(rise · log2 t)`, Q32. |
| D7, D12 | ticks (above); `WHOLE` stays and is now exact, since two grid times differ by at least 2^-20. |
| D9 inside D10 | the relay of an apex that all but stopped is grown on D9's integer test, never on a presence above 0. |
| D10 | `moved`: the move's presence `(logit go - logit u) / window`, Q32, its window from the log wood interpolated by the used ticks. The continuation's and relay's presences are that Q32 value, so the decision and the geometry agree. |
| D11 | `wake`: `delay` on the grid plus `-ln(1 - u) / rate`, floored to a tick. |
| D15 | `kept`: each borne bud's `dead_by` as `-log2`, Q32, summed (exact, order-free), the conditioning on the bearer's context in Q62. The record's `context` is now that Q32 sum. |
| D18 | `relay_node`: a phytomer's scale within one growth unit is its node presence times factors the unit shares, so the relay's node is decided on the unit's node presences alone, Q32 integers (`lineage::nodes` now gives Q32 leads, `decide::node` the presence). The node is set in `presence::assign` (and by the rough layout when it frames a relay); `geometry::relay_point` reads it and keeps only the continuous point within the span. |

**What it moves** (`contract dump` on the four species, ages 40 and 80, seeds 1 and 7, whole and P2, against task 1's head `f3633bd7`; `raw/fixed/fixed-vs-task1.md`):
- **Topology:** identical in all 28 trees (no axis and no phytomer only on either side).
- **Tips:** at most 24.5 µm (spruce 80, seed 7, whole), RMS at most 0.25 µm. The beech moves at most 1.5 µm, the oak 11 µm, the palm not at all.
- **Radii:** at most 1.56 % relative (spruce 80, seed 7), on woken buds' first units, as the tick probe measured; the beech at most 1.1e-5, the oak 0.93 %.
- These are task 1's candidate B figures (2.5e-5 m, 1.6e-2), now with the fixed-point thresholds built: the thresholds add no topology change and no visible movement.

**Stills** (age 80, seed 1, bare and in leaf, before and after; `raw/fixed/stills/`, side by side in `pair-<species>.png`): indistinguishable. Pixels that differ out of 691,200: beech 7 bare and 26 in leaf, spruce 1 and 3, oak 15 and 23, palm none. Viewed by the worker; the host's and owner's judgment stands.

**Tests:** `fixed/tests.rs` (7: accuracy, monotonicity, the threshold's semantics, presences, the grid) and `fine/tests/ticks.rs` (3: a lifespan read on the grid, waking on it, a lifespan under half a tick refused). Every existing `telperion-space` test passes unchanged, the cut, order, chunking and adversarial tests included.

**Not covered:** the lit oak's shade-scaled viability (`viability^(light^-φ)`) is computed in f64 before the fixed-point share, since light is F2b's and refused on the fine path.

## Host decision 5 (2026-10-06, after task 2): where fine geometry lives on the device

(b): the F1 executor ends at the botanical tree plus its laid geometry (scale, girth, lay), held in device buffers. F4 (fn-198) with fn-171 defines the device-side readers (rings, stations, curves). Not (a), which would pull integration into F1 against decision 4; not (c), which reads fine geometry back and breaks Astra change 3 (about 3.9 s on the spruce at 80).

## Host decision 6 (2026-10-06, task 5): the wave loop on the device, totals read back

Option (a) of task 5's question: the GPU executor's wave loop runs on the device, and only a few totals per run and per wave cross to the host, to size the buffers the next steps fill. Astra change 3 forbids per-node data crossing, not small totals. Nothing per chain or per node is read back except by the tests (`fine/readback.rs`).

- **Per run** (one batch of chains at one region size), one round trip of four counters, the totals of the run's scan (`record.wgsl`, `scan.wgsl`):
  1. the packed axes;
  2. the packed phytomers;
  3. the geometry scratch words;
  4. the chains queued for a region eight times larger (the overflow rule; `gather.wgsl` gathers them on the device).
- **Per wave**, one round trip of three counters, the totals of the wave's scan:
  1. the kept axes, which size the wave's laid buffer;
  2. their phytomers, which size the same buffer;
  3. the buds they bear, which size the next wave's buds.
- Everything else stays on the device:
  - a run's records and places;
  - packing;
  - the geometry items (`emit.wgsl`, `describe`);
  - laying;
  - the next wave's buds with their roots and their bearers' laid indices (`emit`), in the CPU reference's order.
- The round trips per tree are the runs plus the waves. `roots::Cost::trips` counts them where they are read, and the tests hold that count (`laid_tests.rs`, `trips`).

## Host decisions 7 and 8 (2026-10-07, after task 6's CPU executor; REALIZED.md)

7. **`grow` routes through the partition.** `telperion_space::grow` builds every species that reads no light through the realized CPU executor at one cut, N = 1,024. Note, not a per-species value tonight: the measured optimum is the spruce at ∞ (no closed-form table) and the beech at 1,024 to 4,096 (REALIZED.md, "Timing"). Light-reading species and those with `retained` or `leaf_girth` stay on today's path in this branch until fn-211's stepped executor is merged (the same partition stepped by cycle). Proof: `measures hash 80`, seeds 1 and 7, neutral and lit, every tree unchanged; `grow` timed before and after on 1 and all cores, ages 40 and 80.
8. **D17: option A.** Every presence that feeds the shedding sums becomes a Q32 integer on both executors, extending decision 1, specified in the fixed module the GPU mirrors. What it moves on today's trees is measured (the 28-tree dump; stills of the four species at 80 if anything moves, viewed). Then the GPU mirror: the three bottom-up passes across waves and the conservative ground bound, topology identical by lineage to the CPU executor, geometry within the screen.

**Open on decision 7 (worker, 2026-10-07): the axes' order.** The partition assembles the same tree by lineage, but not in today's index order: each root's first axis takes its stand-in's index, and its other axes follow the structure above the cut. `measures hash` folds the axes in index order, with each origin's parent index included, so it differs although every axis, phytomer and float is the same (`digest`, which folds in canonical order, is equal). See the task's report for the options.

**Host decision on the order (2026-10-07): option C.** Every `grow` returns its tree in one canonical lineage order, whichever path built it: parents first, siblings by lineage. fn-198 adopts the same order before conversion, so one rule serves both. As built (`fine/canonical.rs`, `lineage_order`): breadth first from the seed; each placed axis's children follow, in the order their parents were placed, each parent's children sorted by lineage and, where two share a lineage (a sleeping bud woken on two carrying axes), by identity (`identities`). The proof of decision 7 is the lineage digest of every tree equal to today's (all four species, seeds 1 and 7, ages 40 and 80, neutral and lit); `measures hash` is re-pinned once to the canonical order. `grow_whole` is today's whole-tree path, in the same order, kept as the tests' reference.

## Host decision 8 built on the CPU (task 6, worker, 2026-10-07)

`crates/telperion-space/src/fixed/presence.rs` is the contract's presence arithmetic; the GPU executor mirrors it. Every presence the shedding sums read is a Q32 integer, held exactly in its real, and every sum and product over them is taken in integers.

- **Leads** come from the decision's own threshold: `under(t, d) = logit(t) - logit(d)` and `over(t, d)`, Q32. A presence and its decision therefore agree. This covers survival (D5's threshold), abortion (D6), the move's stop, the relays (D9's threshold), and a bud's PA (D3's running-sum thresholds, `lineage::bud`).
- **Windows** are the expected wood's share from the Q32 log-wood table (`exp`), or D10's interpolation by used ticks (`wood_at`), times the rate and clamped, all Q32 (`Windows::window_q`). A sleeping bud's window is `decide::node`'s, from its supplied wood.
- **Products** are floored in the grower's order (`times`); a species' real (a place's share, a reach, a continuation, the relay share) enters floored (`q`). The products covered:
  - a unit's survival times the balance's keeping;
  - a persistence, `relayed + (1 - relayed) faded`;
  - the carrying of a woken bud (`dormant::carried`) and what its bearer keeps of it (`Woken::kept`);
  - the living share of an apex (`Grower::living`);
  - the running presence, unit presences and phytomer presences (`presence::assign_one`);
  - each axis's vigour.
- **Fades** (`shed.rs`) are integers throughout:
  - living times are Q32.32 cycles, presences Q32;
  - the carry `before + weight (gap + lived)` is floored;
  - the shedding test is `delay + 1 - (age - until)` clamped to the living apex, an exact integer.
  - The real fade that sizes a kept axis is that integer's real.
- The ranks stay real: geometry alone reads them, and D18 reads the node presences, which were already Q32.

**What it moves** (`contract dump`, the 28 trees at 40 and 80, seeds 1 and 7, whole and P2, against `2bda5531`; `raw/d17/q32-vs-base.md`):
- **Topology:** identical in all 28 trees.
- **Tips:** at most 0.45 µm (spruce 80, seed 7); RMS at most 19 nm.
- **Radii:** at most 2.8e-4 relative (oak 80, seed 1), and 5.5e-5 on the beech and spruce.
- **The palm:** does not move.

**Stills** (age 80, seed 1, bare and in leaf, six views each, before and after; `raw/d17/stills/`, the in-leaf pairs side by side in `pair-<species>.png`, all four viewed by the worker): indistinguishable.
- Out of 691,200 pixels, the most that differ in one view: the beech 89 (crown); the oak 22; the spruce under 1; the palm none.

**On the GPU** (task 6, 2026-10-07): the chain grower takes every one of these presences as the same Q32 integer, held to the bit against the CPU executor per axis (REALIZED.md, "The GPU mirror built").

**Tests:** `fixed/tests.rs`, `presences_lead_from_their_thresholds` (the lead against the real log-odds within 1e-7; certain elements unbounded; products floored). Every existing `telperion-space` test, the realized partition's bit identity with `grow_whole` included, passes on the integer path.

## Host decision (2026-10-08, the fn-212 merge into F4): phase 2's girth in contract A's fixed point

- **Why.** In fn-212's parity probes at spruce 80, seed 1, the device-settled structure above the cut lay 70 µm from the CPU's (fn-198 MERGE-F212.md). The cause was phase 2: `settle.wgsl` computed each root's base radius in f32, about 1.5e-6 relative from the CPU's fixed-point girth, and the sag amplifies that.
- **The decision.** The device's girth pass mirrors `girth::thicken_given` exactly, using contract operations that already exist. Any new operation goes into the contract probe.
- **Built** (`ee13130f`, `fine/girth.wgsl`):
  - sections in W and pipes and radii in Q32, through the feedback contract's `log2_w`, `exp2_w`, `prod_shr`, `mul_shr` and `ripe` and the decision contract's `log2q`, `exp2q` and `mulq`;
  - relays split by `relay_parts` at a Q32 `relay_span`;
  - the PA's girth values carried as Q32 in the geometry table;
  - each chain's base radius carried as Q32 in its record and read back exact.
  - One new operation, `q32_f` (the CPU's `q32` of a float the device holds), is in `contract.wgsl` and the contract probe (selector 20, `P_Q32_FLOAT`).
- **Result** (spruce 80, seed 1):
  - 71,459 of 74,358 base radii equal the CPU's to the bit. Every root that differs also gets different scales (in Q32) from the f32 scale pass; none with equal scales differs.
  - The structure above the cut lies 5.8 µm from the CPU's, against 70 µm before.
- **Still f32:** the scale, lay, lever and bend passes. Their share, from replaying the device's outputs into the CPU settle: the radii they still move account for 6.4 µm above the cut, and the lay's loads for 0.74 µm. Porting them is a separate step (host).
