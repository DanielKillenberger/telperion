# fn-199: relays change the tree by degree

Base: `e9bb5b08` (fn-198's tip). Walk tests: `crates/telperion-space/tests/relays.rs`, each held to the walk tests' verdict (`tests/walk/degree.rs`, moved out of `walks.rs` unchanged): a slope bound of 30 per unit, and the steepest steps split eighth by eighth six times must shrink twentyfold.

## Status

| R | Status | Walk test | On the base | Now |
|---|---|---|---|---|
| R1, survival with every stop relayed | **Stopped: design question 1** | `survival_with_every_stop_relayed_changes_the_tree_by_degree` (ignored) | red: jumps of 0.23, 0.17, 0.14 of the tree that do not shrink | red |
| R2, relay position in whole nodes | Resolved by `b172c2d4` (fn-206) | `a_node_growing_in_moves_no_relay` (moved from `walks.rs`) | green | green |
| R3, a relay barely off a bent limb | **Stopped: design question 2** | `a_relay_barely_off_a_bent_limb_changes_the_tree_by_degree` (ignored) | red: jumps of 0.051, 0.036, 0.034 | red |
| R4, relay girth on a discrete node | **Built** (`56e431d8`) | `a_relay_moving_along_its_parent_thickens_it_by_degree` | red: 0.20, 0.35, 0.057 of the thickest radius, unchanged under refinement | green |
| R5 | Partial: R2 and R4 green, R1 and R3 ignored; the crate suite otherwise green (176 passed) | | | |
| R6 | Not run: the beech is re-rendered once R1 and R3 land. `raw/final21/five-seeds.png` is on no disk any more, so the comparison needs a base render too | | | |

Red logs: `raw/red-base.log`. Probe that found each crossing: `raw/probe199.rs` (needs `blend`, `units`, `alive` made `pub` to build).

## R4, built

A relay standing `t` of the way along the span of its parent's node `k` (`geometry::relay_span`, the same `t` `relay_point` places it by) hands its section to node `k` by `b t`, to node `k - 1` (or the parent's base, for `k = 0`) by `b (1 - t)`, and to the tip by `1 - b` (`girth::relay_shares`). The same rule is applied in four places that held the old one:

- `girth::thicken_given`;
- the bounded chain's girth (`bounded/girth.rs`, `hand_on`);
- sag's load (`sag::gather`): the same defect, a relay's load stepping a node, which moves a sagging tree's shape; folded in because it is the same rule, not a new one;
- the GPU's `girths` and `levers` (`settle.wgsl`), with `relay_span` factored out of `walk.wgsl`'s `relay_point`, and four scratch words per axis for what a relay hands its parent's base (`SA` 66 to 70).

**Trees moved** (`raw/digests-*.txt`, `raw/hash-*.txt`): topology unchanged everywhere (the digests' second column). The beech's and the oak's fields moved (both relay at `relay_at` below 1); the spruce and the palm are unchanged to the bit. `measures hash 80`: beech `7d1d7dba`/`b03594ae` (was `c88dd57b`/`1fcaf39f`), oak neutral `34ed1f16`/`c0301514`, oak lit `afc37044`/`4e37a735`; spruce and palm as before. The lit oak through the stepped executor still equals `grow`.

**Executors agree:** `generation::fine` under the GPU lock, 6 passed: the contract probe (`the_gpu_takes_every_decision_the_cpu_does`), units, chains, and every root grown and settled on the GPU as on the CPU (`raw/gpu-fine-r4.log`).

## Design question 1 (R1): what a failed unit's relay grows in the year the unit failed

**What happens.** At the walk's crossing (`raw/probe199.rs r1 1 1.04 1.0653`), limb `8aaa595c`'s first unit (cycle 5) fails or survives:
- **Fails:** its relay is born in cycle 5 at blend 0 (laid as the continuation) and grows its first unit in cycle 6.
- **Survives:** the limb grows units in cycles 5 and 6.

So the failing side has lost cycle 5's unit, and everything above it is a year behind. With `relay_failed` 1 the stop's stake is 0 (`stop_stake`), so neither the surviving unit nor the relay has a window to fade over: the year swaps at once. Decision 11's answer for abortion (the apex all but stopped carries on as itself and stands in for its relay) does not carry over, because the failed unit's relay is a year late and is not the apex carried on.

**Two candidates, neither built:**
- **(a) Decision 11 extended to failure.** The survival draw's window takes the unit's whole stake (the relay does not replace that year). An apex that all but failed also grows the relay it would have had, at the share its own unit lacks, as a separate axis born that cycle. At the bound the surviving unit fades to none and its relay is whole, which is the failing side. Cost: one more axis near every such bound, a new topology rule on both executors (`advance.rs`, `apex.wgsl`), and the abortion test's no-dip property to be shown for failure too.
- **(b) fn-202's continuous time** (fn-202 RESULT.md, "fn-199's first finding"). A failing unit grows the share of its year the apex lived, `t = ln u / ln viability`, and the relay's first unit grows the rest of that cycle (`carry_on(cycle, used)`, as an ended age's relay does now). At the bound `t` tends to 1, the failed unit grows whole and its relay's first share is none, which is the surviving side. Cost: a per-bud logarithm decides a unit's share, so FINE-CONTRACT D5 and D7 gain a fixed-point `ln` both executors must compute bit for bit, and the failed unit, which grows nothing today, grows.

The host's call: (a), (b) or another. Both change the trees wherever a unit fails with a relay share above 0.

## Design question 2 (R3): where the relay of a relay that never grew stands

**What happens.** Every R3 crossing (seed 1 at -3.61, seed 3 at -0.71 and -1.30) is the same:
- An axis `q` all but stops, so its relay `p` is laid at blend 0 (as `q` carried on) with no phytomers: its first unit failed, or grew no node.
- `p`'s own relay `r` then stands on `p`. `p` has no last growth unit, so `relay_point` puts `r` at `p`'s base, which is `q`'s tip, framed by `p`'s heading.
- On the other side of the crossing `p` does not exist, and the same bud (same lineage and draws) is `q`'s relay, standing `relay_at` along `q`'s last unit.

So `r` jumps from `q`'s curvature zone to `q`'s tip: 0.05 of the tree on seed 1 (`raw/probe199.rs r3 1 -3.63 -3.5994`). fn-206's handed straightening (R3 as the spec words it) is in place; this crossing is what is left.

**Candidate, not built:** an axis with no phytomers that carries `q` on (a relay or a continuation) has `q`'s last unit as its own: `r` stands, and takes its node, on `q`'s last unit (recursively), whatever `p`'s blend. At blend 1 that is `p`'s base already, but framed by `q`'s direction there rather than `p`'s bud heading. Botanically it reads as a dead relay bud's place taken by another bud of the same curvature zone.

**Why it is the host's call:**
- It moves blend-1 relays of empty relays too (their heading).
- It leaves a crossing where `p`'s first node grows in from nothing: `r` then switches from `q`'s frame to `p`'s vanishing unit. A complete answer needs `p`'s last unit blended with `q`'s by how much of `p` has grown, a rule that does not exist yet.
- Girth and load follow the node (R4's rule), so `r`'s pipe would enter `q` at its own place rather than through `p`'s tip.

## Gate

Not run: the task stops at the two design questions. The crate suite (`raw/space-tests-r4.log`; R1 and R3 then red, since ignored) and the GPU fine tests are green on `56e431d8`'s code.

## Host decisions (2026-10-07)

- **R1: option (b), continuous time.** A failing unit grows the share of its year the apex lived (`ln u / ln viability`), and its relay grows the rest of that cycle. The `ln` is a specified fixed-point routine built on the contract's fixed-point `log2` (FINE-CONTRACT D5/D7 extended), mirrored bit for bit in WGSL and covered by the contract probe. Trees change only where a failing unit with a relay exists; show which and how much.
- **R3: not the worker's candidate**, which still jumps when the first node grows in. A relay on a node-less continuation is placed by a blend from the grandparent's curvature zone to its own, weighted by the continuation's grown fraction of its first node, so the placement moves by degree. The heading change of existing relays is accepted, with stills.
- **R4's matching sag-load fix stays:** one rule for girth and load.
- **Then R6:** the beech at its five seeds on the base (fn-198's tip) and after, viewed side by side; the gate; `flowctl done` if the acceptance holds.

## R1 under decision (b): built in the grower and both executors, stopped at the closed form

Kept on the local branch `fn-199-r1-wip` (one commit on `cfa2a735`), not on this branch, because it leaves the crate suite red (below).

**What is built:**
- **Contract:** `fixed::decide::lived_q(l, share, d)`, the ticks `-log2(d / 2^53) · 2^20 / l`, floored, at most the unit's share, with `l = -log2 viability` (Q32, the row D5 already holds). The WGSL `lived_q` is word for word, using `log2q` and `fraction`; the contract probe's selector 6 now also checks it, edges included (`l` 0 and `INF`, shares to a tick past whole).
- **Grower** (`grow/advance.rs`, `grow/stop.rs::fail`):
  - Where a failure's relay draw makes one (D9's integer test), a failing unit grows its lived share at the relay's presence `ρ`.
  - Its relay is made whole (presences 1, 1, so its vigour is the parent's end, which holds `ρ` once), laid by the stop's lead over `SPAN` as before. It grows the rest of the cycle via `carry_on`, its first unit keyed apart (`lineage::FAILED`): nodes, laterals and decisions.
  - The surviving side carries `ρ + (1 - ρ) · faded`, decision 11's form, so the two sides meet at the bound.
  - A failed unit's relay carries the apex's time on: `Carried.relay` is false for it, so the rest is bounded by the lifespan and a failure in an age's last unit moves on as the surviving apex does (this removed lifespan-walk jumps the first build had).
  - With no failure relay (`relay_failed` 0, or the draw over it) nothing changes, to the bit.
- **GPU** (`apex.wgsl` `fail`, `assign.wgsl`): the same law. A unit records whether its failure is relayed (`U_FAILS`), its cycle's used ticks, and a fourth ask for `ρ` (`UNIT_ASKS` 4, `UW` 24 to 28). A failed unit's relay asks nothing, and its made presences are 1. `IS_REST` keys the relay's first unit apart.

**Walk tests:** R1's walk (`survival_with_every_stop_relayed…`) is green. Still red:
- **The closed form.** It models the old law: a failed unit grows nothing and its relay starts next cycle with a whole first unit (`closed_form/steps.rs`, `failed`). Five statistical tests compare it with the grower and fail:
  - `closed_form::kept` (two tests): born 2, phytomers 17.0 ± 0.4 grown against 15.2 in the formula;
  - `the_settings_grow_the_expected_counts`;
  - `sleeping_buds_grow_the_expected_counts`;
  - `a_bud_never_wakes_in_a_stage_it_outlived`.

  The closed form also feeds the windows and the realized cut's predictor and kept tables, so those see the old law too. Bringing it to (b) means modelling a random lived share (its mean and square for lengths, and a relay whose first unit starts at a random point in the cycle) through `steps.rs`, `events.rs`, `kept/` and `predictor/`. The choice of how is the host's: the mean lived share (cheap, biased by Jensen's inequality), or a few quadrature points over the lived-time distribution (exact to the test's tolerance, at a cost per failed step).
- **Two walk jumps the new law exposes on the walk tree** (its limbs relay 0.3 of failures):
  - The release law: `sleeping: states[1].zones[0].rate` and `.delay` at seed 3, 0.019 of the tree, not shrinking. At the crossing (`raw/probe199.rs rate 3 0.4692 0.4790`) a sleeping twig on a relay whose age ended wakes either in the last tick of cycle 8 (vigour 1e-6) or at the start of cycle 9 (vigour 1). The base's walk did not meet this configuration; not yet traced to its cause.
  - `branches_are_made_and_unmade_at_vanishing_size`: limb viability, seed 3, a branch made at 1.3e-6 of the tree against the 1e-6 bound.
- `a_relay_stands_in_the_curvature_zone_on_the_upper_side`: epitony's rise is 0.045 against the 0.05 the test asks. The pool of relays now includes failed units' relays, which stand in a partial unit.

**GPU parity: not yet run.** The GPU lock stayed held by other sessions for the 20 minutes `flock` waited (`raw/gpu-fine-r1-wip.log` holds only the timeout). The WGSL compiles only when the GPU tests run, so the shader edits are unchecked.

## R3 under the host's design: not started

Time ran out after R1. R6 and the gate wait for R1 and R3.

## Host decisions, second round (2026-10-07)

1. The closed form models the lived share by quadrature over its distribution, with the fewest points that meet the comparison tests' tolerance, through `steps.rs`, `events.rs`, `kept/` and `predictor/`. Measure what it costs the count-only predictor on one core (`taskset -c 0`) at 40 and 80 for the beech, spruce and oak, against before.
2. The release-law jump (seed 3): waking time must change the tree by degree. Trace the cause and fix it at its source in the same spirit as R1: a bud waking at the end of a cycle and one waking at the start of the next must meet.
3. The 1.3e-6 vanishing-size breach and the epitony test's 0.045: find the cause. A test's threshold changes only if the botany is shown right, and the reason is stated.
4. Then R3 (the blended placement), GPU parity for R1 under the lock, R6 (the beech, base against after, five seeds, viewed), and the gate (in two parts: the workspace without `telperion-render` unlocked, then `telperion-render` under the lock).

## Resumed after the crash (2026-10-07, evening)

The crashed worker's tree was committed as it stood (`2b8769fd`). It held the closed form by quadrature (`schedule::lived`, `LIVED` 2 points), `lived_q` rounded up (a draw at the bound lives the whole unit), the GPU's unit keys, and a fix for the release-law jump (`dormant.rs`, `carry.wgsl`). The crate suite then aborted on a stack overflow.

### Fixed: a failed whole unit's relay (`795a3555`)

**Cause.** The relay of an ended age grows one whole unit past its lifespan. Where that unit failed at `t`, its relay took the lifespan's left as its share, which is `-t`. Its unit ended at once, and its own ended relay began the cycle again from `gone` 0.
- In the closed form the recursion never ended (the overflow, beech PA 7 at `m` 1).
- In the grower the cycle was grown twice.

**Fix.** The schedule's `Unit` records whether it is a relay's whole first unit (`whole`). A failed unit's relay keeps that, so it grows the rest `1 - t` of the unit and ends as the apex would have. Built in the grower (`relay_bud` takes `whole`), the closed form and `Living` (`Carried::failed`), and the WGSL (`relay_bud`'s `whole`, `IS_ENDED`).

**Reach bound.** A lived part's relay is bounded from the part's tip, not the unit's (`kept/bound.rs`). The parts a relay chains through one cycle then add to no more than the unit, and the rows' reach is the old reading's to 1e-14 again. With the unit's tip, the bound had grown fourfold at the 95th percentile.

**Length test.** `the_expected_length_weighs_the_expected_counts` holds only where every unit is whole. A failed unit's lived part grows its nodes whole at a share of their length, so the test's species relays no failure. It already excluded the two other partial-length cases.

### Suite state on `795a3555`

- **Crate tests:** green except two pins and one walk.
- **Comparison tests:** the five statistical tests RESULT.md listed as red are green (`closed_form::kept`, `settings`, `dormant`).
- **Walks:** R1's walk is green; still `#[ignore]`d, to be lifted with the rest. The release-law walk (`the_release_law_changes_the_tree_by_degree`) is green on the worker's `dormant.rs` fix.
- **Rows' pins:** `the_rows_hold_their_values` and `the_rows_stay_near_the_old_reading` fail for the beech and the oak only, the species that relay failed units. Spruce and palm hold to the bit. To be re-pinned once the point count is decided.
- **`branches_are_made_and_unmade_at_vanishing_size`:** red, 1.18e-6 against the 1e-6 bound. Decision B below.

### Decision A: the quadrature's points, and its cost

**Comparison tests.** One point (the median of the lived-time law) and two points both meet the tolerance of every comparison test against the grower.

**One point against two:**
- beech at 80: section moments (`s2`) differ by 0.33%, mass by 0.16%, chance kept by 0;
- oak at 80: `s2` by 3.9%, mass by 2.9%, kept by 1.7e-6.

**Predictor cost.** The count-only predictor (`Predictor::new`) on one core (`taskset -c 0`), best of 5, in ms. The spruce relays no failure; its spread (10 to 13, 29 to 51) is noise from the shared core. Tool: `raw/timing-test.rs`.

| | before (`e9bb5b08`) | 1 point | 2 points |
|---|---|---|---|
| beech 40 | 2.4 | 17 | 153 |
| beech 80 | 10.9 | 168 | 1253 |
| oak 40 | 5.2 | 21 | 187 |
| oak 80 | 19.3 | 117 | 1318 |
| spruce 40 | 10.2 | 10.8 | 11.3 |
| spruce 80 | 29.0 | 31.9 | 31.4 |

- **Rows (all 40 pins):** 2.5 s at one point, 22.5 s at two, more than 15 minutes at four (stopped).
- **Floor:** raising `LIVED_FLOOR` from 1/256 to 1/16 roughly halves two points (beech 80 378 ms, oak 504) and leaves one point about the same.
- **Why it costs:** each failed unit's relay is a new table at the same age, keyed by its own time spent and part of a cycle gone. Its first unit can fail again, so the relays nest through the cycle and multiply with the points.

**Options:**
- **(A1)** One point, as the rule "fewest points that meet the tests" gives. It is close to the mean lived share the host turned down; the tests cannot tell the two apart.
- **(A2)** Two points, at 7 to 115 times the predictor's cost.
- **(A3)** One or two points with the nesting cut: a relay's own first unit (the rest of the cycle) counts its failure as carrying on, as `LIVED_FLOOR` does for slivers. Not built or measured.

The tree is at `LIVED` 2 (the worker's) until decided.

### Decision B: the vanishing-size breach is the tick grid

**Where.** `states[1].viability` at seed 3, crossing at viability 0.698642337848412 (`raw/probe199.rs made 3 "states[1].viability"`, with `LIN=9b185353e12a9f5d` for the axis below).

**What happens.**
- Lateral 17's first unit fails. It lives 0.9504280 of the cycle on one side and 0.9504290 on the other: one tick (2^-20) apart.
- Its relay 73 grows the sliver that is left (share 0.04957) and survives it on one side only. Its survival threshold `v^share` moved by one tick's worth of share, about 2e-5 log-odds, past the draw.
- So the surviving side starts at presence 7.29e-5, not 0. That is the lead one tick buys over the window, not a fade from none. The sliver's subtree (12 phytomers, about 1.6% of the tree at full presence) comes in at 1.18e-6 of the tree.

**Cause.** Under (b) the lived share depends on the walked viability, and it is quantized to ticks (FINE-CONTRACT D5, host decision 1). The next unit's thresholds then step by one tick's worth wherever the lived share crosses a tick. Under the old law no walked setting moved a share through a draw (a lifespan is not walked). The release law has the same quantization in wake times but met the bound on this run.

**Options:**
- **(B1)** A finer grid for the lived share only, e.g. 2^-32 of a cycle in D5's `lived_q`, carried in 64-bit ticks to the relay's first unit. A contract change on both executors.
- **(B2)** The relay's first-unit thresholds read the lived share before it is put on the grid, as a Q32 real both executors compute. Also a contract change.
- **(B3)** Accept a step of one tick's worth, and restate the walk's bound as wood per tick of the crossing. A test change the host must accept on the botany.

### The epitony test (unchanged from the worker's reading)

`a_relay_stands_in_the_curvature_zone_on_the_upper_side` asks 0.04, not 0.05; the test passes. The worker's cause is in the test's comment:
- The pooled relays now include failed units' relays.
- These stand on the lived part at the axis's tip, already pointing up (heading z 0.76 against 0.66) and nearer their bound (blend 0.25 against 0.53).
- Epitony turns a relay as far as it has left the line, so it raises them only 0.009.
- The relays of stops after a whole unit still rise 0.050 (0.053 on the base).

For the host to accept or reject; the alternative is to pool only the relays of whole-unit stops and keep 0.05.

### Still to do, after A and B

R3's blended placement, GPU parity under the lock (the WGSL edits are still unchecked, now including `whole`), the pins, R6 and the gate.
