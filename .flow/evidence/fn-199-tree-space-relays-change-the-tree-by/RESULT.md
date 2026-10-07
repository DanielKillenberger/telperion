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

## Host decisions, third round (2026-10-07)

- **A.** The predictor has to stay in milliseconds, since it runs on every parameter change in the harness; two points at 1.3 s are out. A1 (one point) now. Then A3, the nesting cut, built identically in the grower and the closed form, and adopted only if every comparison test passes and it brings the predictor at 80 down toward its old cost; numbers reported either way.
- **B.** B2: the relay's first-unit thresholds read the unrounded lived share, a Q32 value both executors compute, written into the fixed-point contract. A one-tick step breaks "by degree", so B3 is out; B1 changes more than needed.
- **Epitony.** Neither the threshold lowered for the whole pool nor the pool filtered to pass. Two checks: the relays of stops after a whole unit rise at least 0.05 (the original law), and failed units' relays do not fall (they start near their bound).
- **Then** R3's blended placement, GPU parity under the lock (every WGSL edit compiled and run), the rows re-pinned once, R6's beech stills viewed, `pub(crate)` restored on `Axis.blend`, `units` and `alive`, and the gate, render half first.

## A1 and A3 built: adopted

**A1:** `LIVED` 1 (`eacb5181`).

**A3, the nesting cut**, built alike in every executor:
- **Grower:** `advance.rs`. A failed unit's relay's first unit (`draws.rest`) whose failure would relay carries on instead, at that relay's presence. That is what an apex that all but failed carries, so the two sides still meet.
- **GPU:** `apex.wgsl`, the same test on `IS_REST`.
- **Closed form and `Living`:** `Carried.rest`, set by `Carried::failed` and part of every table's key, counts that failure as carrying on, as `LIVED_FLOOR` does for slivers.

**Comparison tests:** every one passes (`closed_form::kept`, `settings`, `dormant`, `closed_form`). So do the walks but the tick-grid breach (now 1.183e-6; B2 next), among them R1's walk and the release law. The rows' pins still differ for the beech and the oak alone.

**Predictor**, one core, best of 5, ms (the second of two runs; the first ran beside a build):

| | before (`e9bb5b08`) | A1 | A1 + A3 |
|---|---|---|---|
| beech 40 | 2.4 | 17 | 3.4 |
| beech 80 | 10.9 | 168 | 18.0 |
| oak 40 | 5.2 | 21 | 5.6 |
| oak 80 | 19.3 | 117 | 20.7 |
| spruce 40 | 10.2 | 10.8 | 10.2 |
| spruce 80 | 29.0 | 31.9 | 27.6 |

## B2 and epitony built; R3 built; stopped at a design question in R1

### B2 (`704ad2d3`)

**Contract.** D5 gains `lived_fine_q`: the lived time floored in 2^-32 of a cycle. What `lived_q` rounds it up by, under a tick, goes to the failed unit's relay:
- CPU: `Draws::below`;
- GPU: axis word 25, `BELOW`.

The relay's first unit reads its share in 2^-32 of a cycle for its survival (`survival_fine_q`) and abortion (`abortion_fine_q`, where the exposure is that unit's own share). A tick's share `s << 12` gives the tick forms' bits; `fixed::tests::a_share_finer_than_a_tick_meets_the_tick` holds that, and the contract probe has two new selectors (16, 17). Written into FINE-CONTRACT.md's table.

**Walk.** `branches_are_made_and_unmade_at_vanishing_size` is green.

### Epitony (`704ad2d3`)

`a_relay_stands_in_the_curvature_zone_on_the_upper_side` now holds two claims, a relay classed by whether it grows from the cycle after its parent's stop (`birth == apex_end`):
- the relays of stops after a whole unit rise more than 0.05;
- failed units' relays do not fall.

Green.

### A failed unit's relay that grows from the next cycle is not a rest

**Change.** Where a failed unit lived its whole unit (lived time at the bound), its relay grows from the next cycle. It is now the apex carried on: not flagged `rest`, so not keyed apart, with no nesting cut and no `below`. CPU `relay_bud`, `Carried::failed` (`rest: same`), and WGSL `relay_bud`.

**Found by.** The probe on R1's walk: the A3 cut was applying to whole next-cycle units.

### R3, the blended placement (uncommitted at `704ad2d3`; this commit)

**Built (CPU only so far).** `Axis.first` replaces the crashed worker's unused `base_scale`. It is how far an axis's first node has grown in: its unit's presence times its node's, 0 with no phytomers. It is set in `presence::assign` and in the grower's two pencil paths.

`geometry::carried_point`: a relay of an axis that carries another on (a relay or a continuation), whose `first` is under 1, stands blended between two points by `first`:
- where it would stand on the axis carried on, at that axis's `relay_at`, the node found by scaled length (`zone_node`), taken from the farthest such axis down;
- its own point.

**The weight.** The weight is the first node's presence, not its length. A first try weighted by length (presence × share). It failed the abortion walk at seed 1 (-3.61): there the carried-on relay's first unit is a sliver of the cycle, grown whole in presence. Standing it on the axis carried on moved it off that sliver, which the other side of the crossing stands on. With the presence weight, two of R3's three crossings meet.

**R3's walk.** One crossing is left: seed 2 at -0.7675, 0.028 of the tree.
- A lateral whose first unit grew no node all but aborts. Its relay, at blend 0, grows the next unit's two nodes.
- On the other side the lateral grows them itself.
- The relay of either stands on the same unit's node 1, but those nodes are laid apart (their bases 0.2 m apart in height). The relay at blend 0 is laid from the node-less lateral's frame with its handed pulls, the lateral from its own straightening.
- That is R3 as the spec first worded it (the straightening a relay barely off a bent limb keeps). fn-206's handed pulls do not cover a parent of no phytomers. Not traced further; the GPU mirror waits on it.

### Design question C: the abortion a failure at its bound skips

R1's walk, green on every commit up to `eacb5181`, is red from `3afbe0fd` (A3) on. Three crossings: seed 3 at 1.2364, seed 1 at 1.8963 and 3.6070, 0.05 of the tree each. A3 moves which draws these seeds meet; the gap it shows is decision (b)'s, not A3's. Every one is the same (`raw/probe199.rs r1 <seed> <from> <to>`):
- **Failing side.** A limb's unit fails at the survival bound, its lived time the whole unit. The failure's relay is laid as the continuation (blend 0) and grows on from the next cycle.
- **Surviving side.** The apex survives the unit and then takes the abortion draw after it. It aborts, or its age ends, and its relay is laid by that stop's lead (blend 0.48 to 1).

The failing side never takes that abortion draw, so at the bound the relay's blend jumps from 0 to the abortion's. Decision (b) makes the two sides meet in what grows, not in the decisions that follow the unit.

**Options:**
- **(C1)** The failed unit's lived part takes the unit's abortion draw (the same key) over the exposure `min(lived, next unit's share)`, which vanishes as the lived time does. Where it aborts, the failure's relay is itself an apex that aborted at once: its relay stands on it, so by R3's placement on the failed axis's zone, blended by the abortion's presence. Both executors and the closed form (the abortion's chance after a lived part).
- **(C2)** The failure's relay, where it grows from the next cycle, takes the abortion draw the apex would have taken after the unit (the apex's key, not its own), before its first unit. Simpler, but only for a lived time at the bound. A lived time just under it grows in the same cycle and takes no such draw, so the step moves to `WHOLE` rather than going.
- **(C3)** Fade the abortion's relay in by the survival's presence: the abortion draw after a unit decides at the survival lead's presence. It changes every abortion near a survival bound, not only relayed ones.

### Not yet done

GPU parity under the lock (none of the WGSL edits has compiled yet), the R3 GPU mirror, the pins, R6 and the gate. The crate suite is red on R1's and R3's walks and the two pins.

## C1 built; R1's walk red with a new kind of crossing (host stop rule)

**C1, as decided.** Built on:
- the grower: `fail` and `persists` in `stop.rs`; `stop` and `relay_bud` take `whole` and `below`;
- the GPU: `fail`, `make_relay`, `abortion_after`;
- the closed form and `Living`: `Carried::lived_on` in `schedule.rs`, folded into the lived part's relay weight; the predictor's lived end through `maybe`;
- FINE-CONTRACT.md's table.

A relay that grows in the cycle its axis stopped in, other than an ended age's, is a rest relay. That now covers the aborted lived part's relay too.

**Comparison tests:** all green (`closed_form::kept`, `settings`, `dormant`, `closed_form`). The WGSL has still not been compiled.

**R1's walk.** The three C-class crossings (seed 3 at 1.2364, seed 1 at 1.8963 and 3.6070) are gone. Two new ones at seed 2, 0.6743 and 0.6987 (0.054 and 0.099 of the tree), are of a new kind:
- **Failing side.** A failure lives one tick short of the whole unit. Its relay grows a one-tick rest in the cycle it failed: born the cycle before, sleep near 1.
- **Other side.** The unit lives whole and its relay grows from the next cycle: born a cycle later, sleep 0.
- **Effect.** That relay's own relay is laid with a different heading (z 0.54 against 0.95).

**Suspected cause, not yet checked by a fix.** In the grower's sketch (`grow/draw.rs::pencil`), a relay's handed pulls count its years as `axis.birth - p.birth - draws[parent].sleep`. That leaves out the relay's own sleep. So its birth stepping by a cycle at the tick, with its sleep stepping from about 1 to 0, moves its age by a whole year. Elsewhere an axis's years are `age - birth - sleep`, which is continuous there.

### Every crossing class found so far, and what each fix changed

| # | Class | Where found | Fix | What it changed |
|---|---|---|---|---|
| 1 | A failed unit lost its year: its relay grew next cycle, whole (R1 as specced) | R1 walk on the base | Decision (b), the crashed worker's `3e34dc6b`: lived share plus the relay's rest | Grower, GPU, schedule. R1's walk green on the walk tree |
| 2 | A failed whole unit of an ended age's relay took a negative share; the closed form recursed forever, and the grower regrew the cycle | Stack overflow in the crate suite | `795a3555`: `Unit.whole` carried to the failure's relay | Schedule, grower, GPU flag. Rows' reach back to the old reading |
| 3 | Closed form on the old law; then its cost (7 to 115 times at two points) | Comparison tests; timing | A1 (one point) and A3 (no nesting within a cycle), `eacb5181`, `3afbe0fd` | Predictor at 80 back near its old cost. Every comparison test green |
| 4 | The tick grid: a lived time crossing a tick stepped the relay's thresholds | Vanishing-size walk, 1.18e-6 | B2, `704ad2d3`: a share below the tick | Contract D5/D6, both executors. Walk green |
| 5 | A3's cut applied to a whole-lived failure's next-cycle relay | Probe on R1's walk after A3 | `e21d3d7a`: such a relay is not a rest | Grower, schedule, GPU |
| 6 | A relay on a node-less carried-on axis stood at its base, not in the curvature zone (R3) | R3 walk, three crossings | R3 blend by the first node's presence, `e21d3d7a` | Geometry (CPU). Two of three crossings met |
| 7 | A failure at its bound skipped the unit's abortion draw | R1 walk, three crossings | C1 (this commit) | Grower, GPU, closed form, `Living`. All three met |
| 8 | The straightening handed by a parent of no phytomers | R3 walk, seed 2 at -0.7675 | Not traced yet | |
| 9 | A one-tick rest relay's birth and its age | R1 walk, seed 2, two crossings | Not built (suspected cause above) | |

**Convergence.** Each fix removed the crossings it aimed at, and none came back. What is left are geometry and age-accounting cases at a stop's bound (8, 9), not growth-law ones. Classes 6, 8 and 9 sit in how an axis that carries another on is laid. Each fix so far moved which seed shows the next class rather than shrinking a common measure. A walk over more seeds would say whether there are more.

## Class 9 fixed; the WGSL compiled; the census (host, 2026-10-07)

### Class 9 (`0e7ddd65`)

**The defect.** The relay's handed years in `grow/draw.rs::pencil` now use `age - birth - sleep` on both axes, as the host asked. That alone did not move the two crossings. Their cause was R3's weight: it read the first phytomer alone, a one-tick unit's node at partial presence (0.43), and pulled a relay of a whole-grown relay back toward the axis carried on.

**The fix.** `Axis.first` is now the presence grown, unit times node, summed over the axis's nodes to one. With it, R1's and R3's walks on seeds 1 to 3 are green, R3's seed-2 crossing (class 8) included.

### WGSL

**Status.** The WGSL is compiled and run under the lock: `generation::fine`, 6 passed in 252 s (`raw/gpu-fine-c1.log`).
- The contract probe passes with selectors 16 and 17.
- Chains, units, and every realized root grown and settled on the GPU as on the CPU.

**Caveat.** R3's placement is not mirrored in `walk.wgsl` yet, and the settle test did not catch that. Its trees likely hold no relay on an axis of partial `first`, or the geometry screen absorbs it.

### Census: R1 and R3 over seeds 1 to 50 (`raw/census.log`, probes in `raw/census/`)

**Method.** Each seed's three steepest steps were refined six levels of eight (the walk tests' rule). Every flagged step was then bisected 60 times by `probe199` on the largest change. Six of the 13 flags bisect to 4e-11 to 3e-10 of the tree: steep slopes the 4096th refinement does not resolve, not jumps. They are R3 seeds 5, 8, 12, 19, 23 and 32.

**Jumps:**

| Class | Walk, seed | Size (share of tree) | What happens |
|---|---|---|---|
| 10 | R1 30 | 2.3e-2 | A relay grows a one-tick rest of one node on one side, none on the other. R3's weight counts that node's presence (1) though its length is a tick, so the relay's relay stands on it, not on the carried-on axis's zone |
| 10 | R1 42 | 1.3e-2 | The same, one step further down a chain of relays |
| 11 | R3 27 | 1.1e-2 | An abortion at its bound: one side's axis carries on itself, the other's relay at blend 0 carries it on. The laterals it bears are framed differently on the two |
| 11 | R3 4 | 3.6e-3 | The same |
| 11 | R3 48 | 2.4e-3 | The same |
| 11 | R3 25 | 2.8e-4 | The same |
| 11 | R3 50 | 2.7e-4 | The same |

**Reading.** Both live classes sit where the host placed classes 6, 8 and 9: how an axis that carries another on is placed and laid. Two of seven are 1e-2 and over.
- **Class 10** is the presence-against-length tension in R3's weight. Weighting by presence breaks for a node of a tick's length. Weighting by length broke the sliver unit at seed 1, class 6's first try.
- **Class 11:** a relay at blend 0 is laid, and frames what it bears, from its own phytomers, not as the axis it carries on would be. Which part differs (handed pulls, years, or the phyllotaxis side) is not traced.

Nothing was fixed after the census.

## Host decision: one rule for carried-on axes (2026-10-07)

- **Rule 1, one object.** An axis carrying itself on is a relay at blend 0. There is no second way to lay, age, frame or straighten a continuation: the apex's own continuation and a relay go through the same function, and a relay at blend `b` differs from it only through `b`. Class 11 vanishes by construction, with no special case. Where the code has two paths, one is deleted.
- **Rule 2, one measure for R3's weight.** The weight is the carried-on axis's grown extent: the sum over its nodes of unit presence × node presence × the node's lived share of its unit, capped at 1. It is continuous in presence and in lived time, so a one-tick node adds about 0 (class 10). Class 6's sliver case must still meet under Rule 1; if it does not, it is reported with the probe, not patched.
- Both rules are built on the CPU, then the census over seeds 1 to 50 is rerun as the judge. If it is clean apart from refinement-limited slopes: the R3 GPU mirror, full GPU parity, re-pinning, R6, `pub(crate)` and the gate. If new classes appear: stop and report.

## Rules 1 and 2 built (CPU, `803a8063`); the census found a new class; stopped

### What was built

**Rule 1.** `Axis::carried_on` gives `(parent, blend)` for a relay and `(parent, 0)` for a continuation. Through it, one path:
- **Frames:** one arm in `geometry::frame` for both.
- **Ages and straightens:** `bend`, and `reaches`, where a continuation's reach now joins its parent's.
- **Pulls handed down:** `place::start`, `single::lay_one`, `draw::pencil`; a continuation now carries its parent's straightening as a relay at blend 0 does.
- **Phyllotaxis:** an axis carried on no longer turns its side. Its laterals' azimuth adds a phase: each axis carried on adds `divergence × rank` (`geometry::phase`; `bud_frame` too). R3's blend mixes the absolute azimuths.

**Rule 2.** `Axis.first` is now Σ unit presence × node presence × lived share, capped at 1.

### Census, seeds 1 to 50

- **Rules 1 and 2:** R1 40 jumps, R3 80 jumps.
- **Rule 1 with the earlier presence-only weight** (a measurement, not committed): R1 2, R3 1.

### Under rules 1 and 2: a new class 12, class 6's sliver case not meeting

Every crossing probed is this class: R3 seeds 1, 23, 37; R1 seeds 1 and 40; sizes 0.054 to 0.10.
- **Side a.** An axis `q` ends in a partial unit, the lived part of a failure (share 0.02 to 0.37). A relay of `q` stands on that partial unit, as `relay_point` takes `q`'s last unit whatever its extent.
- **Side b.** `q` stops a unit earlier. Its relay `p`, at blend 0, grows that same partial unit. The next relay stands on `p`, but Rule 2's weight is `p`'s extent, the partial unit's share (0.05 to 0.37). So it is pulled mostly to `q`'s previous unit.

The host's check, that class 6's sliver case still meets under Rule 1, fails. Probe: `raw/probe199.rs r3 1 -3.6259 -3.5994`, axes 81, 275 and 556 against 81 and 275.

### Under Rule 1 with the presence-only weight (3 jumps)

| Class | Walk, seed | Size | What happens |
|---|---|---|---|
| 10 | R1 31 | 2.8e-2 | One-tick node, as before |
| 10 | R1 42 | 1.8e-2 | One-tick node, as before |
| 13 | R3 40 | 7.5e-3 | A relay at blend 0 grows one node of partial node presence. The next relay is blended back toward the axis carried on by that presence. On the other side the axis grows the same node itself, and its relay stands on it whole |

**Class 11 (five crossings) is gone under Rule 1.** Rule 1 holds as designed.

### Why classes 10, 12 and 13 are one question

The weight pulls a relay standing on a carried-on axis back toward the axis carried on, by how much of that axis has grown. `relay_point` on the axis carried on stands on its last unit however little of it has grown. The two meet only when the same growth counts the same on both sides, whichever axis holds it. Any weight measured on the carried-on axis alone breaks where the same partial unit sits on the other axis on the other side:
- with share counted, class 12;
- with presence only, class 13 (and class 10 for length).

**Candidate for the host, not built.** `relay_point` itself stands on the last unit blended back toward the unit before by the last unit's grown extent (Σ presence × node presence × share, capped at 1), along the whole line of axes carried on, as if one axis. The weight then belongs to the unit, not to which axis holds it. R3's `carried_point` becomes this rule's case where the last unit is on the axis carrying on. It covers classes 6, 10, 12 and 13 with one measure. It changes every relay that stands on a partial last unit (failed units' lived parts, partial lifespans).

## Host decision: Rule 2 withdrawn, Rule 2' adopted (2026-10-07); built; the census is clean

**Decision.** Rule 2 (R3's weight on the carried-on axis) is withdrawn. Rule 2': `relay_point` blends from the last unit back towards the unit before, by the last unit's grown extent (unit presence × node presence × lived share, capped at 1), along the whole line of carried-on axes read as one axis. R3's blend is that rule's special case, and the separate R3 weight is deleted. Moving relays that stand on partial last units is accepted: it is the "by degree" requirement.

**Built (CPU):**
- **`geometry/stand.rs`.**
  - `stand(axes, species, p, node)` reads the line's units newest first. The newest takes the contract's node (D18); the others take the node by scaled length.
  - Each unit's extent is `Phytomer.grown`, set in `presence::assign` and the grower's pencils. It replaces `Axis.first`, which is deleted, along with `carried_point`.
  - Points blend down to a whole unit or the line's base. The ranks are absolute azimuths, phase included.
  - `relay_span` (girth and sag, R4) stays on the last unit.
- **The base phase.** `Axis.phase` is the phase from outside the structure. A fine root's stub (`stub((bearer, phase), node)`) holds its bearer's phase, so a root's laterals keep the phyllotaxis of the line they stand on (`phyllotaxis_runs_on_across_a_continuation` caught its absence).
- **Closed form.** A lived part's relay stands blended back by the lived part's extent, its nodes at their mean (`lived_extent`, in `events.rs` and `kept/path.rs`), an explicit approximation.
- **`Living`.** Not affected: it reads chances, not places.

**Tests.** Every comparison test passes. `relays`, `form` and `geometry` are green. The two pin tests wait for the re-pin.

**Census, seeds 1 to 50 (rules 1 and 2'):** R1 0 jumps; R3 1 flag (seed 50, 4.8e-4 after refinement). It bisects to 8.1e-11 of the tree: a refinement-limited slope. **Clean.**
