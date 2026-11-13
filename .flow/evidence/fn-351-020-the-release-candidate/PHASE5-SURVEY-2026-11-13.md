# Phase 5 survey, second read: counted foliage, 13 Nov 2026

By a Fable agent, read-only. Nothing was built, run or committed; no design is proposed beyond listing what the records and the code leave open.

**Why this file has a date in its name.** `PHASE5-SURVEY.md` already exists beside it, tracked (`b8c1eec08`, 12 Nov 07:04, read at fn-355's head), with `PHASE5-CENSUS.md` (`5d13c5a1e`, 12 Nov 07:23). The brief said to create `PHASE5-SURVEY.md` and to change no tracked file; the two conflict, so this is a new file and the earlier one is untouched. This read is taken on today's code and adds what the earlier one lacks: fn-357 to fn-360, `BEECH-VERTICES.md`, the drawn stage, the guarantees by test name.

**Tags.** [code] read in source at `.worktrees/fn-360` (`a37a9c4e1`), cited as file and function. [record] a filed number, cited by file. [derived] arithmetic on recorded numbers, done here. [unknown] not found. Paths starting `grow/`, `light/`, `closed_form/`, `presence/` are under `crates/telperion-space/src/`; `pipeline/` is under `crates/telperion-core/src/`. Evidence paths are under `.flow/evidence/`; fn-340 and fn-354 to fn-360 evidence is on the fn-360 branch, not in the integration worktree.

## 1. What the owner has ruled, and what the plan promises

### Rulings, in his words

| Date | Words | Where recorded |
|---|---|---|
| 13 Oct 2026 | "with this algo we want growth to be part of the product. It's great that we botanically grow trees. This should be locked in." | `RULINGS.md`, AGENTS.md |
| 13 Oct 2026 | "I just don't want prebaked 3d models." and "i have no problem baking things offline that ship as part of the engine" | `RULINGS.md` |
| 3 Oct 2026 | "growth decides where leaves are, so expansion places them without culling." | `RULINGS.md` (fn-125) |
| 18 Sep 2026 (paraphrase) | the far draw is "the near draw minus what the eye cannot resolve at that distance, and nothing more, so a walk from base to hero pose shows no step in fidelity" | `RULINGS.md` (fn-176) |
| 3 Nov 2026 | "remove the shoot limit everywhere.. We don't want artificial limits like that i've said that before" | AGENTS.md, "No artificial limits" |
| 3 Nov 2026 | "I think it's problematic that we're making core settings per preset. This should be a generalized solution that works across the tree space. This shouldn't be per species." | `RULINGS.md` |
| 9 Nov 2026 | "i don't think we want hand tuned rules. It's going against the engineering principles" | `RULINGS.md` |
| 9 Nov 2026 | "I want highest fidelity/performance ratio tradeoff. That's what we're maximizing. Realistic trees generated and rendered fast." | `RULINGS.md` |
| 10 Nov 2026 (paraphrase) | accepted that the leafy shoots stop being stored state and become a function of the structure, and that every preset is re-derived on sourced traits | `RULINGS.md` |
| 10 Nov 2026 | "3x cheaper is pretty good already" | `RULINGS.md` |
| 11 Nov 2026 | "speed i really think we should aim for better but in a way that doesn't get us stuck if it can't be reached with the approach we're doing now" | plan, section 4 item 1 |
| 11 Nov 2026 (plan approved, "ok prompt pls") | section 4 item 3: leafy shoots confined to a light cell are counted by the same law and built from their key when seen; this is the growth, not a shortcut | plan; AGENTS.md "Botanical growth is the product"; STRATEGY.md "Our approach" |

The rule that item 3 amends (13 Oct, AGENTS.md): "No speed-up may replace growth with a shortcut that changes the grown tree beyond its measured tolerance (fn-245 COUNCIL.md)." AGENTS.md now adds: "Counting is not such a shortcut (owner, 11 Nov 2026 ...): leafy shoots that cannot leave a cell of the tree's light are carried as counts moved by the species' same law, and a twig is built from its key when a view resolves it". STRATEGY.md adds "so a twig is the same twig whenever it is seen" and, further on, "a count steps only by one unit of the structure it counts".

Year-independence has no owner quote of its own; it is the plan's P4 ("a year is a state of the same tree", set by the 13 Oct ruling) and fn-351's P4 wording ("the tree at year y is the same whatever age was asked for and whether grown in one call or paused").

The closed form has no owner ruling [unknown: none found in `RULINGS.md`]. Its standing is the host's decisions (fn-192, fn-209) and the tests in section 4.

### What Phase 5 promises (plan section 5; fn-351 "Build order")

- Leafy shoots that cannot leave a light cell carried as counts.
- A twig built from its key when a view resolves it.
- A proof on the oak, spruce and beech before the product is touched.
- The per-species coarse rows deleted here (P6: "no preset carries a `/space/coarse` row").
- **Exit:** the scoreboard no worse and every tree cheaper than at the last landing; then the speed loop (profile, name the largest cost, remove it, measure, repeat while a round gains a tenth or more), which never blocks later phases.
- It serves P1 (browser: aim half of 0.1.4's time, host's number), P3 (native: oak at 40 in 100 ms, to 300 in 15 s, a thousand oaks in 10 s, host's numbers) and P6.
- Plan section 2: "The per-species coarse rows are what makes the website fast today (coarsest oak 164 ms in Wasm). They are ruled out (P6), so they go only when counted foliage replaces them."
- Plan section 7, not known: "Whether counted foliage stays close to the grown tree now that the dead-wood artefact is gone, and whether it reaches 0.1.4's browser times: the gap today is 5 to 56 times."

### What the log has since added to Phase 5's load

- 12 Nov 05:40 and 07:00: the old oak's cost "is Phase 5's" (fn-356 reverted whole).
- 12 Nov 07:00: the host recommended Phase 5's count before Phase 3 and put the order, and how Phase 2 lands, to the owner. **No answer is recorded** in the log, `RULINGS.md` or `OWNER.html` as of the last entry (13 Nov 03:07) [unknown].
- 12 Nov 07:23: "what can change is what a record costs, not how many there are" (host's reading of the census).
- 11 Nov 18:45: "the drawn leaf being Phase 5's to rebuild".
- 13 Nov 01:30: the beech at 80 draws 101.7 M wood vertices, "not accepted as it stands"; the gate's memory budgets stay, "so the gate cannot go green on this phase until the beech's wood is cheaper".

## 2. What earlier work tried or proved

### fn-339, the cohort law, the first proof (10 Nov; `fn-339-*/RESULT.md`, `R1-LAW.md`, `R1-ANSWERS.md`, `R3-RESULT.md`)

- **What was counted.** Whole lateral subtrees, by the root lateral's PA and birth year, on the bearing growth unit; a count is its whole part plus one member at its fraction's presence, "No keyed rounding" (`R1-ANSWERS.md` items 2 and 8).
- **Which PAs** (expected reach under the 0.5 m cell, `R1-LAW.md`): oak twig (0.313 m) and short (0.059 m); spruce shoot (0.068 m); beech short and shoot (0.450 m expected, 0.62 m potential, over the line).
- **Law.** The species' shedding rule at the bearing unit's size, in expectation; leaf deposited at the unit's place; light and upkeep into the bearer's balance; keeps the bearer open; enters the bearer's fade and girth (`RESULT.md`, R4).
- **Share of the cycle's units inside the cohorts at 40:** oak 0.925, spruce 0.820, beech 0.937 (`R3-RESULT.md`).
- **Replay on the cycle's own wood:** new shoots 1.006 to 1.012; living apexes 0.73 to 0.79, cause not isolated (`R3-RESULT.md`).
- **Coupled probe, time (cycle over probe):** oak 40 3.24, spruce 3.20, beech 1.95, oak 80 2.52, oak 120 1.79. Peak memory oak 120: 2.26 to 0.56 GiB.
- **Coupled probe, tree:** at 40 within 6% on height, spread, trunk (oak trunk 1.115). Oak drifts with age: trunk 1.359 at 80 and 1.906 at 120 (1.68 to 3.66 by seed), leaf 1.893 at 120.
- **Cost inside the probe:** the cohorts' yearly step took 68% of the oak's probe at 40; without it the cycle would be 11.8 times the probe (oak 40), 4.7 (spruce).
- **Not built:** cohort leaf was counted, never drawn ("Drawing it needs twig axes built from the cohorts, the third proof's work"). Sleeping buds of a cohort age were refused by name.
- **Verdict:** stopped by the goal's rule then (five times cheaper nowhere; no nearer real trees). **Kept as:** `probes/coupled.patch` and `replay.patch`, applying at `d2baa9508`; no crate change.

### fn-340, the cohort law carried forward (10 Nov; `fn-340-*/RESULT.md`)

- **Corrections to fn-339:** cohorts stand in the vigour split; a class that stops standing leaves frozen upkeep on its bearer; retirement reads the cohorts' living; state carried a year at a time (`law/carried.rs` in the patch, test `the_carried_state_is_the_recomputed_expectation`, within 1e-9 to year 30).
- **Isolated:** retirement not reading cohorts plus dropped frozen upkeep was most of fn-339's runaway (seed 1007's trunk at 120: 3.66 to 1.50).
- **Time (cycle over probe):** oak 3.88 / 2.49 / 2.63 at 40 / 80 / 120; spruce 2.99; beech 2.94. Peak memory oak 120: 2.18 to 0.61 GiB.
- **Tree (median of per-seed ratios):** oak trunk 1.135 / 1.361 / 1.258; spread at 120 0.857; current-year leaf at 120 0.812 (0.453 to 1.121). Spruce and beech at 40 within 2% but the beech's trunk at 1.055.
- **Cohort step's share of the probe:** oak 49% / 63% / 67% at 40 / 80 / 120; "Carrying the state did not make the step cheap."
- **Not isolated:** the trunk's 10 to 36% bias; the leaner oak at 120; "the law's open-loop bias": a fifth fewer living tips, 5% more leaf at 40 and 17% at 80, "no standing twig wood on units past ten years".
- **Verdict:** stopped by the reworded rule (not five times; trunk not within a tenth). The owner then said "3x cheaper is pretty good already". **Kept as:** `probes/coupled2.patch` at `577831cf0`.
- **Both proofs predate** fn-349 (dead wood charged nothing), fn-354 (girth by sapwood, one leaf account, `shade_short`), fn-355 (no horizon), fn-357 to fn-359. Whether either patch still applies is [unknown]; M5 of the first survey (re-run fn-340's patch) was not run (`PHASE5-CENSUS.md`, "Not run").

### fn-341, the earned structure, the second proof (10 Nov; `fn-341-*/RESULT.md`)

Not a counting proof. Girth by rings (leaf above over R x N) and posture by gravitropic performance. Oak trunk moved inside White's range from 80 to 300; beech trunk fell to 0.037 m at 40 because "its trunk is what its leaf earns". Upkeep on living wood (U) was switched off: it "held twice the cycle's structure at 120 and outgrew 24 GB at 300"; "the cycle's only bound on how much structure a tree holds is the charge for wood already dead". Cost: the ring step was 2.2 to 2.3 times the grow, because the probe settled a copy every year; "The rule itself needs one sum carried up the tree a year". Owner's verdict on its sheets (10 Nov): "every probe is worse than today". **Kept:** the rule's form, rebuilt as fn-354's `girth::lay`. **Bears on counting:** wood now reads leaf per phytomer, so a count must hand its leaf to the wood (section 4).

### fn-349, the corrected balance (11 Nov; `specs/fn-349-*.md`)

A living tip's charge is its size times the square of its path over the potential height; dead wood adds nothing; a lateral subtree is tested on its own tips and paying children. Built on a side branch, retired as superseded by fn-354, whose task 1 carried its four CPU commits onto one grower (log, 11 Nov 15:39 and 16:27). **Bears on counting:** the collapse with age that kept the base's shoot count down is gone, so the faint population shows (fn-356 `SURVEY.md`, "What the base hid").

### fn-350, the kept pipe (11 Nov; `fn-350-*/RESULT.md`, `VERDICT.md`)

A measurement of the old `retained` setting at 0, 0.5, 1. The old oak keeps its stem (300 on seed 1001: 7.7 m to 23 m); trunk 2.6 to 5.4 m across; cost unchanged (120.6, 122.0, 121.7 s; 1.99 GiB). **Dropped:** "Every tree from 80 on is worse to look at than today's". **Kept:** the finding that thinning wood lays the old oak down, which fn-354's "wood is never lost" answers. No bearing on counting beyond that.

### The five builds that tried to remove near-invisible shoots

The record is fn-356's, not fn-354's: `specs/fn-356-*.md` ("Result"), `fn-356-*/SURVEY.md`, `fn-356-*/TASK1-STOP.md`, on the fn-360 branch. All on the oak at 120, seed 1002 unless said.

| # | Design | Guarantee it broke | Measurement |
|---|---|---|---|
| 1 | A unit lays `min(1, size x presence)` of its nodes (`9bc970234`) | The leaf; the closed form's counts (nine oracles); independence of draws (three tests); one walk | Phytomers 9.18 M to 2.13 M; under 0.2 mm 3.89 M to 5 thousand; drawn leaf down 78% at 40 and 42% at 120; height at 120 30.0 to 25.6 m; 19 of 770 narrow tests failed; simulator comparison off by 47.6 standard errors |
| 2 | A tip whose unit is under one node ends: survival times `min(1, presence x N)` (`90e3b3dda`) | The closed form (nine oracles, the simulator comparison refused `Collapsed`); independence (three tests); two walks | 15 of 771 failed before any tree was measured |
| 3 | A tip under the grain ends: presence x share under `GRAIN` = 1e-7 (`551494642`) | The closed form on whole elements (`deterministic_trees_grow_the_closed_form_counts`: grew 0 against 2) | Would end 85% of living tips at 80 and 90% at 120; median presence x share at 120 is 3.9e-9; a full-presence tip reads 1e-7 to 1e-6 on the young oak |
| 4 | Faintness rounded without bias: under the floor tau an element is kept with probability f / tau at faintness tau, at two sites (`ec9efa1a7`) | The extent bound (one lateral 0.93% past); the shed-certain shortcut's two tests on the beech | 767 of 771; every oracle, independence test and walk held |
| 4b | The same at one site, a lateral's birth (`edd90ebba`) | The extent bound by more (one axis 44.6% past); a child stood above its bearer (vigour 4.68) | Two cases returned |
| 5 | The floor capped by the bearer's faintness (`9e41eb29e`) | None: every test passed | Held phytomers 9,175,834 to 8,985,206 (2.1%); living tips down 3.5%; 625,037 of 694,762 living apices still under the grain; leaf, height, trunk within 0.3% |

The log counts "Five builds, none kept"; the spec counts four designs with two narrowings. Reverted whole (`2b3af9e5c` to `7983897b7`).

**What they fixed as constraints** (spec, "Result"): a growth rule that treats a faint element differently from a whole one breaks the closed form (which counts an element at its share, linearly), or the independence of draws, or the leaf; unbiased rounding keeps those, but a child may not stand above its bearer, and 94% of faint shoots stand on a faint bearer, so rounding at birth never reaches the population. "It can be carried more cheaply."

### The Phase 5 census (12 Nov; `PHASE5-CENSUS.md`), and what it refuted

- Candidate C of the first survey (a faint subtree carried as the closed form's expected subtree scaled by its faintness) is **refuted before being built**: realised faint subtrees stand at 0.04 to 0.36 of base x expectation (scale, wood) and 0.08 to 0.12 (leaf), with a wide per-root spread (twig records / fN: p10 0, p50 22.4, p90 749). The closed form has no light and counts shed wood.
- Leaf deposits that round to nothing are 0.003% of the oak's leaf at 120: no saving there.
- 57.9% of held axes at 120 were retired roots still paying per-axis stores; acted on by fn-357.

### fn-357 and fn-358, cheaper records, the tree equal to the bit (12 Nov; `TASK1-DONE.md` of each, log)

- fn-357: a retired root keeps a 24-byte stand. Footprint, oak 120 seed 1002: 7.106 to 4.795 GiB.
- fn-358: a growing phytomer is 72 bytes, not 136. Footprint 4.795 to 3.845 GiB; process peak 7.47 to 4.34 GB. Seconds unchanged within load.
- **Kept, both.** Named and not built (log, 12 Nov 09:24): "nothing reads a closed axis's pencil and a closed axis's phytomers are read for 24 of their 72 bytes; by the host's arithmetic that is about a tenth more at 120 for a second record kind".

## 3. What the code has today that a count could stand on

### A short shoot and a leafy twig in telperion-space

- **One chain of 15 physiological ages** for every species [code: `chain.rs`, constants]: ... `BRANCH` 9, `BRANCHLET` 10, `SPUR` 11, `SHOOT` 12, `TWIG` 13, `SHORT` 14. An age a species does not use is passed through at lifespan 0 (`chain::passed`).
- **No short-shoot type in the grower.** A short shoot is an ordinary `Axis` of a late PA: `lineage`, `pa`, `birth`, `origin` (`Lateral { parent, node, slot, whorl, woken }`, `Continuation`, `Relay`), `vigour`, `apex_end`, its `phytomers`, its `units` [code: `structure.rs`, `Axis`]. A grower holds each phytomer as `Growing`, 72 bytes: cycle, key, tip, heading, side, scale, grown share, size, rank, section, sapwood [code: `structure/growing.rs`; docs/pipeline.md].
- **The short-shoot mark is a drawing row, not a growth one:** `PaState::cluster_leaves` ("Leaves at the tip of each axis of this PA, which then bears no leaf along its length ... The engine grows nothing of it; the pipeline's expansion seats the leaves") [code: `species.rs`, `PaState`]. The conversion marks such nodes in `Tree::clustered` and records one `Cluster { node, leaves, spread }` a tip [code: `pipeline/space.rs`, `grow`; `tree/cluster.rs`]. Only the beech's PA 14 states one (`clusterLeaves = 4.0`) among the three species [code: `presets/european-beech.values:657`].
- **Lifespans and fates of the fine PAs** [code: `crates/telperion-core/presets/*.values`]:

| Species | PA | lifespan | viability | shedding | internode m | nodes a unit | bears laterals of | leaf years |
|---|---|---|---|---|---|---|---|---|
| Oak | 12 shoot | 4 | 0.93 | 2 | 0.015 | 1 + 2..3 + 1 | 14 (0.3, 0.35) | 1 |
| Oak | 13 twig | 5 | 0.98 | 2 | 0.012 | 1 + 2..3 + 1 | 14 (0.7, 0.3) | 1 |
| Oak | 14 short | 3 | 0.9 | 2 | 0.006 | 3..5 | none | 1 |
| Beech | 12 shoot | 5 | 0.95 | 1 | 0.025 | 1 + 2..3 | 14 (0.6) | 1 |
| Beech | 14 short | 12 | 0.9 | 1 | 0.006 | 3..5 | none | 1; cluster of 4 |
| Spruce | 12 shoot | 2 | 0.9 | 3 | 0.02 | 2 | none | 5.5 |

  All of these have `continuation = 0`. The beech's short-shoot life of 12 is an estimate from Aas's "10 - 15 Jahre" (log, 12 Nov 04:02).
- **Which kinds close by table:** `shed::lasting` gives, per PA, the cycles past a bud's birth by which every apex its subtree can grow has stopped, "from the settings alone"; none where a lateral bears its own age, a sleeping bud wakes, an ended age relays or a failed unit relays [code: `shed.rs`, `lasting`]. The first survey read this as: oak twig, shoot, short; beech shoot, short; spruce shoot. Since fn-354 R7 a beech lateral's kind also reads light (`PaState::shade_short`, `lineage::turn`), which `lasting` does not read [code: `species.rs`; whether this opens any kind is unknown].
- **Leaf of a phytomer, one definition:** `girth::leaf` = `leaf_area x scale x size x clamp(leaf_years - age, 0, 1)`, borne only while the wood is open [code: `girth.rs`, `leaf`]. Read by the light's deposit and by the wood.

### Light cells

- The lattice's spacing is `CELL` = 0.5 m, a constant, anchored at the world's origin [code: `light.rs`, `CELL`; `light/field.rs`, `node`].
- A leaf is deposited at its phytomer's tip to eight corners by trilinear weights, as integer quanta of `QUANTUM` = 2^-20 m² in a u32 a node, so the sum is order-free [code: `light/field.rs`, `Field::quantised_with`].
- Light is read trilinearly at each living apex's tip, once a year [code: `grow/sketch.rs`, `Grower::shade`; `grow/cycle.rs`, `Grower::read_light`].
- The domain is refused past 2^12 m or 2^28 nodes [code: `light/field.rs`, module doc].
- fn-339's 0.25 and 1.0 m were thresholds beside the 0.5 m lattice, never lattice sizes (`fn-339-*/R1-LAW.md`, item 4).

### How the year's passes read an individual shoot

Order in a year [code: `grow.rs`, `Grower::year` and `Grower::step`]: compact, extend the windows, open the working set, retire, balance, allot, grow every living apex's unit, wake, rough layout, light, wood.

| Pass | What it reads of one shoot | Where |
|---|---|---|
| Open | Whether it is on the way from the root to a living apex or a sleeping bud; a closed subtree is skipped by every later pass | `grow/cycle/passes.rs`, `Grower::open`; `grow/cycle/working.rs`, `Working::open_from`, `Working::closed` |
| Retire | A closed lateral subtree whose fade is none: its phytomers and draws are let go, its root stands for it in its bearer's fade; later a 24-byte `Stand` | `grow/cycle/retire.rs`, `Grower::retire`; `grow/cycle/compact.rs`, `Grower::compact` |
| Balance | Per living tip: size = `base_scale x running`; income = size x light at the tip; charge = size x (path x inverse height)². Per living lateral subtree: its sums, its remembered balance, one keyed draw (`Key(lineage).child(SHED).child(cycle)`), its window | `grow/cycle/passes.rs`, `Grower::balance_one`, `charge`; `grow/sketch.rs`, `Pencil` |
| Shed (in growth) | The walk's verdict ends an apex (`apex_end = cycle - 1`) or leaves it a kept share | `grow/cycle/passes.rs`, `Grower::balance_one`; `contract/shed.rs`, `walk` |
| Allot | Per living tip: presence = `base_scale x running x kept`, lit presence, its PA's `shade_size`; the size is normalised over the growing buds of its PA, so it is non-local | `grow/cycle/passes.rs`, `Grower::allot_one`; `grow/cycle/working.rs`, `sizes`, `Working::split` |
| Grow | Each living apex grows one unit from its own lineage-keyed draws, touching only its axis and the axes it makes | `grow/shoot.rs`, module doc; `grow/unit.rs`, `grow_unit` |
| Wake | Per sleeping bud: its PA aged by the time slept, every axis that carries its bearer on | `grow/wake.rs`, `Grower::wake`; `dormant.rs`, `aged`, `carried` |
| Layout and light | Each new phytomer's tip, frame, scale, rank; then the year's leaf at each tip | `grow/sketch.rs`, module doc; `grow/deposit.rs`, `Grower::deposit` |
| Wood | From the tips down, each held open phytomer: leaf above = its own, its axis's beyond it, each child's at its node; `laid = max(0, need - kept)` | `girth.rs`, `lay`, `ring` |
| Shed (at the settle) | A lateral with no living apex past its PA's `shedding` delay is dropped; a kept one fades once, at the least of its own and its bearers' fade | `shed.rs`, module doc, `shed_past` |
| Settle | Marks living apexes, assigns presences, sheds, widens each phytomer, sizes, thickens (`radius = sqrt(T x share / pi)`), lays | `grow/settle.rs`, `Grower::settle_shed`, `settle_lay`; `girth.rs`, `thicken` |

Which of these reads is linear in presence is tabled in the first survey ("What a count must replace"); it was read at fn-355's head and its line numbers were not re-checked here. Checked today: income and charge are linear in size; the balance test carries a memory and a keyed draw per subtree; the wood's ring has a clamp.

### What the closed form already carries without growing

- `closed_form::expected_counts(species, age)`: expected phytomers per PA and cycle for the whole tree, by GreenLab's factorisation; "Each substructure depends only on its PA and age, so it is computed once" [code: `closed_form.rs`, module doc, `expected_counts`].
- Per bud: `Walk::counts(k, m, carried, first)` returns the count table of a bud of PA k grown m cycles; it is private to the module [code: `closed_form.rs`]. The census's public wrapper `expected_sub_counts` exists only uncommitted in `.worktrees/census-p5` (`PHASE5-CENSUS.md`, "What was built"); it is not on the fn-360 branch [code: grep].
- Lengths: `closed_form::log_lengths(species, [(pa, m)])` gives the log of the expected metres a bud of each PA grows in m cycles; each entry "is a function of its own bud alone" [code: `closed_form.rs`, `log_lengths`]. `presence::Windows` holds `look[k]`, the wood a bud of PA k grows over `LOOK` = 40 cycles, and `whole[c]` [code: `presence/windows.rs`, `Windows::over`; `presence.rs`, `LOOK`].
- Totals: `expected_grown`, `expected_length` [code: `closed_form.rs`].
- Kept wood: `expected_kept` reads the predictor (`closed_form/predictor.rs`, E[N; kept] with shedding by delay) and returns `None` "for a species whose buds read light: the predictor is light-neutral, and the lit oak keeps 27 to 64 % of its count at 80" [code: `closed_form.rs`, `expected_kept`].
- **What it lacks:** "Shedding is outside the formula"; no light, no balance, no allotment size, no `shade_short` turn [code: `closed_form.rs`, module doc]. The census measured the gap on faint subtrees (section 2). fn-339's law, which adds the shade hazard and shedding at a cell's light, lives only in the two patches; `crates/telperion-space/src/law*` does not exist [code: ls].
- Extent: `closed_form/kept/bound.rs` gives the farthest a bud's wood can reach, with every bud made and node counts at their 1 - 1e-9 quantile; the `kept` module is test-only [code: `closed_form.rs`, `#[cfg(test)] pub(crate) mod kept`]. fn-339 used an expected reach, not this bound, for its cell test (`R1-LAW.md`, open question 1).

### The coarse rows, and who reads them

- **Rows:** `/space/coarse/{first, factor, leaf, bud, hazard, shedding}` in three presets: the oak (first 9, factor 1.687, leaf 26.289, bud 0.82, hazard 2.385, shedding 0.243), the spruce (8, 2.693, 18.645, 0.982, 3.759, 0.051), the birch (8, 1.503, 10.045, 1.534, 2.366, 0.65) [code: `presets/oregon-white-oak.values:617-622`, `norway-spruce.values:493-498`, `silver-birch.values:792-797`]. Their comments name them as fitted (fn-325, fn-326, fn-328: medians within 5% on tuning seeds). The beech, palm, ordinary tree and the Two Trees carry none.
- **What they do:** `Species::coarsened(&Coarsening, fidelity)` multiplies internode, leaf area, hazards and shedding and divides node counts of every PA from `first` on, geometrically with `fidelity / COARSEST` (1.0 m) [code: `species/coarse.rs`, `Species::coarsened`].
- **Readers:** `GrowInput::of` applies it when `family.fidelity > 0` and the rows are not `Coarsening::NONE` [code: `pipeline/input.rs`]; `Family` validation checks the coarsest species [code: `family.rs`, about line 142]; a blend of two species blends the rows [code: `crates/telperion-core/src/blend.rs`, about line 171]; the preset parser and writer [code: `crates/telperion-core/src/presets/space.rs`, `coarse_row`]; `pipeline/fidelity_tests.rs`; the public `fidelity` parameter [code: `family.rs`; docs/parameters.md line 48]. A fifth row, the coarse pipe, was already deleted in Phase 2 (log, 11 Nov 18:53).
- Who passes a non-zero `fidelity` from the website or harness was not traced [unknown].

### How twigs and leaves are drawn

- **Conversion:** one node per phytomer at its tip. Wood thinner than `FINEST` = 0.2 mm "and all it bears" is not drawn; a phytomer is merged into the last node while the wood since is shorter than `SHORTEST` (1 mm). A node's identity is its phytomer's lineage key [code: `pipeline/space.rs`, module doc, `FINEST`, `SHORTEST`].
- **Wood, one path:** the sweep's rings become a `Curve` (points per ring; runs; clusters of up to 32 points with a bounding sphere and the error of each of `LEVELS` ring levels). No per-shoot mesh object and no instancing of twigs: every drawn phytomer contributes ring points to one curve [code: `pipeline/surface/curve.rs`, module doc; docs/pipeline.md "The wood's curve"].
- **Level of detail for wood exists, per view:** a cluster outside the frustum is not drawn; each cluster keeps the coarsest ring level within half the error; a ring under `RIBBON` = 1 pixel of radius is drawn as a ribbon facing the eye, not a tube; a view past its budget is drawn at a coarser error scale [code: `pipeline/surface/curve/tessellate/plan.rs`, module doc; `crates/telperion-render/src/curve.rs`, module doc].
- **The CPU mesh** (`SurfaceMesh`, chunks of 65,536 vertices) is the reference build "for tests and for consumers without a GPU"; it has no level of detail and uses the preset's `radialSegments` [docs/pipeline.md, "The renderer's one wood path"; `pipeline/surface/chunks.rs`]. `BEECH-VERTICES.md` counts this mesh (20 sides).
- **Leaves:** the leaf plan reads which wood bears leaves and how many stations each bearing segment carries, skipping short-shoot wood [code: `pipeline/foliage/plan.rs`, module doc and line 470]; a short shoot's cluster is seated at its tip from `cluster_key(tip)` [code: `pipeline/foliage/cluster.rs`; `tree/cluster.rs`]. Each leaf is one instance of one element in 12 bytes [code: `pipeline/foliage/packed.rs`]. The GPU picks each leaf's level a frame, "the coarsest level that stays under half a pixel", with no state between frames [code: `crates/telperion-render/src/select.rs`]. Leaf levels are nested simplifications of the one element [code: `pipeline/foliage/levels.rs`].
- **The field (the website's voxels)** reads the drawn tree: each solved node's sweep and each leaf-bearing record with its station count [code: `pipeline/field.rs`, module doc].
- **What a view "resolving" a twig could hook into** (what exists, not a proposal): the curve cluster's sphere and per-level error, tested a frame on the GPU (`measure`, `choose` in docs/pipeline.md) and on the CPU (`Curve::tessellate(&Viewer, error, budget)`); the 1-pixel ribbon line; the leaf selection's projected deviation. All of these sit after the whole tree's nodes and ring points exist; nothing today builds wood or leaves on demand from a key [code: as cited; no on-demand path found]. An older design for trees resolved on demand is fn-209 (`fn-209-tree-space-f1-trees-resolved-on-demand/`, FINE-CONTRACT.md; `fine/canonical.rs` keeps an identity by lineage); its state was not read [unknown].

## 4. The guarantees a counted shoot must keep

| Guarantee | Stated by |
|---|---|
| A tree at year y is the same tree whatever age was asked for, to the bit | `grow/cycle/paused/tests.rs`: `the_tree_at_a_year_is_the_horizons_prefix`, `a_year_is_a_state_not_a_horizon`, `every_year_is_the_horizons_prefix`; `pipeline/space/tests.rs`: `a_node_keeps_its_identity_across_ages`; docs/pipeline.md "A year is a state (fn-355)"; `presence/windows.rs` module doc; fn-351 P4 |
| A growth goes on past its first request | `paused/tests.rs`: `a_growth_goes_on_past_its_first_request` |
| Paused and grown on equals grown at once | `paused/tests.rs`: `a_tree_paused_and_grown_on_is_the_tree_grown_at_once` |
| Repeat, and any number of cores | `pipeline/space/repeat_tests.rs`: `every_preset_grows_the_same_tree_again_and_on_any_cores`; `grow/tests.rs`: `many_cores_grow_the_tree_one_does` |
| Letting go of retired records leaves the tree as it was | `grow/cycle/tests.rs`: `letting_go_of_retired_records_leaves_the_tree_as_it_was`, `the_working_set_and_retirement_leave_the_tree_as_it_was`; `paused/tests.rs`: `a_tips_path_survives_letting_go_and_a_pause` |
| A retired root is read as it was | `paused/tests.rs`: `a_retired_root_let_go_is_read_as_it_was`; `grow/cycle/tests.rs`: `woken_buds_that_stop_at_once_retire_as_the_reference_has_them`; `fn-357-*/READING.md` |
| A growing phytomer is read as it was | `grow/growing_tests.rs`: both tests; `fn-358-*/READING.md` |
| The closed form's counts | `crates/telperion-space/tests/closed_form.rs`: `deterministic_trees_grow_the_closed_form_counts`; `tests/settings.rs`, `death.rs`, `dormant.rs`, `relays.rs` (the oracles named in `fn-356-*/TASK1-STOP.md`); `tests/oracle.rs`: `stochastic_structures_match_the_simulators_in_distribution` |
| The closed form's lengths and bounds | `closed_form/tests.rs`: `the_expected_length_weighs_the_expected_counts`, `an_expected_length_does_not_depend_on_how_far_the_table_runs`; `closed_form/kept/tests/bound.rs`: `no_wood_reaches_past_the_extent_bound` (a child never past bound x its base) |
| What is certain to be shed may be left ungrown, the tree equal to the bit | `grow/tests.rs`: `a_lateral_certain_to_be_shed_is_shed`, `a_tree_grows_the_same_without_what_it_is_certain_to_shed`. `Shoot::shed_certain` reads `rules.age`, the age asked for [code: `grow/shoot.rs`]; how that sits with year-independence was not traced [unknown] |
| Determinism by key; adding a branch moves no other draw | `lineage.rs` module doc; `tests/lineage.rs`: `adding_a_branch_reshuffles_nothing_else`; `tests/dormant.rs`: `adding_a_sleeping_bud_moves_no_other_draw`; `tests/form.rs`: `dominance_leaves_a_few_laterals_whole`; `lineage/keys.rs` (one registry) |
| By degree: a walk never jumps; a branch enters under 1e-6 of the tree | `tests/walks.rs`: `the_release_law_changes_the_tree_by_degree`, `light_changes_the_tree_by_degree`; `tests/walk/degree.rs`: `VANISHING` = 1e-6; core `tests/space_walks.rs`, `walk_*.rs`; STRATEGY.md ("a count steps only by one unit of the structure it counts") |
| Wood follows the leaf above, per phytomer; wood is never lost | `girth/tests.rs`: `constant_leaf_holds_its_sapwood_and_adds_one_ring_of_it_a_year`, `wood_is_never_lost_when_the_leaf_above_goes`, `a_closed_subtree_is_skipped_and_every_section_stays_exact`, `a_phytomer_bears_its_leaf_for_its_leaf_years`; `girth.rs` module doc |
| One leaf read by light and wood | `paused/tests.rs`: `the_deposit_is_every_held_phytomers_leaf`; `fn-354-*/LEAF-ACCOUNT.md` |
| Shedding by light and path; dead wood charged nothing | `grow/cycle/balance_tests.rs`: `a_limb_is_charged_nothing_for_its_dead_twigs`, `a_tip_is_charged_the_square_of_its_path`, `a_part_earns_what_its_paying_children_leave_over`, `a_child_at_break_even_moves_its_bearer_by_degree`; fn-351 P8 |
| Fading once; a living subtree never drawn smaller than its living apex | `shed/tests.rs`: `a_line_idle_past_its_delay_fades_once_to_its_living_apex`; `shed.rs` module doc (fn-359) |
| The light's deposit is order-free | `grow/cycle/tests.rs`: `the_quantised_deposit_is_order_free_and_refuses_overflow` |
| Every pass is reported; the vigour is conserved | `grow/tests.rs`: `every_pass_is_reported`; `allocation/tests.rs`: `the_trees_vigour_is_conserved_at_every_lambda`, `sizes_move_by_degree_as_a_bud_vanishes` |
| Every preset's digest at both ends of the dial | `pipeline/cycle_cost_tests.rs`: `every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial` (a pin; it also reads the coarse rows' end) |
| A node's identity is its phytomer's key; a cluster's leaves draw from the tip's key | `pipeline/space.rs` module doc; `pipeline/space/cluster_tests.rs`; `tree/cluster.rs`, `cluster_key` |
| Native and Wasm grow the same tree | docs/pipeline.md (FINE-CONTRACT.md's representations); the test that holds it was not found by either survey [unknown] |
| No artificial limit; one pipeline; no fallback | AGENTS.md; core `tests/generation_limit_guard.rs`; docs/principles.md |

Not a test, but fixed by five refuted builds (section 2): a faint element may not be treated differently from a whole one by a growth rule, and a child may not stand larger than its bearer.

## 5. The measured size of the prize, from records only

### The beech at 80, seed 1 (`BEECH-VERTICES.md`; base `5ebab2de0` / head `1958c708c`; one measurement each; the CPU mesh)

| | Base | Head |
|---|---|---|
| Wood vertices | 35,182,102 | 101,808,366 |
| ... on short-shoot nodes (PA 14) | 20,395,648 (58.0%) | 74,793,368 (73.5%) |
| ... on PA 12 | 7,336,382 | 18,527,972 |
| ... in rings of 1 to 2 mm radius | 7,876,142 (22.4%) | 73,529,562 (72.2%) |
| ... in rings under 1 mm | 9,709,680 | 18,311,932 |
| Engine axes / PA 14 axes | 181,120 / 137,537 | 321,607 / 241,179 |
| Phytomers kept / nodes | 1.78 M / 1.63 M | 5.04 M / 4.82 M |
| Short-shoot nodes | 923,343 | 3,546,261 |
| Wood length / in the 1 to 2 mm class, m | 20,809 / 3,289 | 48,233 / 32,382 |
| Leaves / leaf area m² | 449,923 / 1,548 | 871,760 / 3,000 |
| Peak memory after the build, MiB | 2,146 | 5,875 |

- Vertices per ring are 20.1 in every class; rings per metre at 1 to 2 mm are 113 (head).
- By axis length (head): 5 to 20 cm hold 46.3% of vertices and 49.1% of leaf area; 20 cm to 1 m hold 35.5% and 16.0%.
- [derived] Per short-shoot axis at the head: 14.7 drawn nodes and about 310 wood vertices (3,546,261 and 74,793,368 over 241,179). A 12-year short shoot at 3 to 5 nodes a year can hold up to 36 to 60 phytomers; the preset rows are in section 3.
- At 40 the same pattern: 9.41 M against 2.50 M; 6.93 M of the head's vertices stand on short-shoot nodes (73.7%).
- This is the drawn stage's cost. The record does not split the grower's time or memory for this tree [unknown for the beech at 80]. The gate's failing items it explains: `smooth_bark`, `form the_beech` (log, 13 Nov 01:30).

### The oak

- "On the oak at 120, 47% of held phytomers stand under a presence of 0.05 and carry 0.8% of the leaf; half the living tips are under 8% present" (log, 12 Nov 04:34; `fn-356-*/SURVEY.md`: 47.2% and 0.8% of 1,228 m², seed 1002, fn-354's head; under presence 0.2, 56.4% and 2.4%).
- The census at `48edda0a7`, seed 1002 (`PHASE5-CENSUS.md`, M1): by axis faintness under 0.05, 38.5% of 11,558,758 held phytomers at 120 carrying 0.83% of the grower's leaf (21.0% and 0.32% at 80). The two figures differ by the measure (unit presence against axis faintness) and the build.
- By PA among those faint phytomers at 120 (open + closed): branch 1.44 M + 0.30 M, twig 0.89 M + 0.59 M, short 0.52 M + 0.63 M. So the faint population is not mainly short shoots; 8,711 faint laterals stand on bearers that are not faint, and two faint limbs hold 3.01 M of the 3.35 M phytomers under them.
- Drawn under the 0.2 mm floor at 120: 3,888,756 of 9,175,834 settled phytomers (42.4%); these are grown and held but not drawn.
- The old oak: 200 on seed 1002 at fn-354's head, 29.58 GiB and 1,091 s, 29.4 M phytomers on 6.3 M axes, 21.0 M under 0.2 mm (log, 12 Nov 04:20); 22.43 GiB and 539 s on fn-355's top. With limb dominance 0.5: 5.5 M drawn phytomers and 5.6 GiB at 200 on seed 1001, and seed 1007 at 300 peaked at 16.4 GB (log, 13 Nov 01:28 and 03:07).
- How much of the oak's count is leafy kinds at full presence (the cohort class): fn-339's share of units inside the cohorts at 40 is 0.925 (`R3-RESULT.md`), measured before fn-349. On today's build it is [unknown].

### The spruce

- Census at 40, seed 1002: 1,167,938 held phytomers on 705,882 axes; faint under 0.05 is 0.5%, so the spruce has no faint population to speak of. 55.8% of its held axes were retired roots (before fn-357). Under the 0.2 mm floor: 2.5%.
- fn-339: 0.820 of the cycle's units sit in the shoot cohort at 40; the probe was 2.99 to 3.20 times cheaper and within 2% of the cycle's tree.
- Its pipeline seconds at 40: grow 3.9, mesh 4.7; the mesh sets its peak, 1.58 GiB (census M2). Wood vertices by PA for the spruce: [unknown, no record].
- Shoots a year against real spruces: the "27 times" is mostly a mismatch of tree size, the real factor nearer 3 on one disputed source (log, 11 Nov 13:45).

### The grower's time and memory, such shoots against the rest

- **Memory by store, oak 120** (census M2, before fn-357 and fn-358): footprint 7.106 GiB; 660 bytes a held phytomer, 1,502 an axis; phytomer stores 199 bytes a phytomer; every other store 1,050 bytes an axis. After both cuts 3.845 GiB, the phytomers' store 1.069 GiB (log, 12 Nov 09:24). The split by PA or by faintness is [unknown]; by [derived] proportion, 38.5% of phytomers under 0.05 would be about 0.41 GiB of the 1.069 GiB phytomer store, if records are uniform.
- **Where the peak is:** at 120 the grower's steady state at the end of growth; at oak 80, beech 40 and spruce 40 the mesh sets it (census M2).
- **Time:** pipeline seconds grow / mesh: oak 80 26.8 / 2.0; oak 120 97.2 / 2.8; beech 40 0.7 / 0.2; spruce 40 3.9 / 4.7 (census M2). The grower's time split between leafy or faint shoots and the rest on today's build is [unknown]. The nearest records are fn-340's phases, on the older grower: the probe without its cohort step would be 9.5 (oak 40), 8.8 (oak 120), 4.2 (spruce), 4.9 (beech) times cheaper than the cycle, and the cohort step itself took 26 to 67% of the probe.
- **The wood pass:** 8, 13 and 13% of the grow at 40, 120 and 300, visiting every held open phytomer (log, 11 Nov 22:00).
- **Presets on one thread, seed 1001** (log, 12 Nov 14:24, load 3 to 40): oak 40 1.58 s; oak 120 42.8 s; spruce 11.6 s; a thousand oaks 496 s. Wasm at the base: oak 40 2,073 ms, spruce 7,548 ms, against 0.1.4's 124 and 122 ms (log, 11 Nov 16:24). The coarsest oak with its coarse rows: 164 ms in Wasm (plan, section 2).

## 6. The options left open

Listed, not ranked. "Has it" answers whether the closed form already carries the law.

### O1. Cohorts by table, on the bearing growth unit (fn-339 and fn-340's form)

- **Counted, grain:** whole lateral subtrees of the kinds `shed::lasting` closes whose reach stays inside a cell; a count per bearing unit x kind x birth year.
- **Yearly law:** the species' tables in expectation at the unit's light, with the shade hazard and shedding. The closed form has the unlit counts and lengths per bud (`Walk::counts`, `log_lengths`); it has no light, no shedding inside the formula, no balance. The lit law exists only in `coupled2.patch`.
- **From a key:** each member's axis, phytomers and leaves when drawn; never built in either proof. `R1-ANSWERS.md` item 8 chose "No keyed rounding", so which whole twigs stand for a fractional count is undecided.
- **At risk:** the closed form's oracles (a lit expectation is a new term); independence of draws (fn-340's class in the vigour split); wood follows leaf (measured trunk bias 10 to 36% under the old girth); year-independence and paused equality (carried state must be a function of the year alone); by degree at the cell line (the beech's shoot at 0.450 m expected, 0.62 m potential).
- **Measure first:** whether `coupled2.patch` applies today (M5, not run); the trunk and leaf bias under `girth::lay`; the share of today's phytomers and drawn vertices in those kinds, by species; the faint population it does not reach (oak 120: branch-kind faint phytomers 1.74 M of 4.45 M).

### O2. The same class, counted per light cell (the plan's and AGENTS.md's literal wording)

- **Counted, grain:** leafy shoots by kind and birth year per 0.5 m lattice cell, or per bearing axis within a cell.
- **Yearly law:** as O1; the cell's light is one read for all its members, where the cycle reads each tip trilinearly (`R1-LAW.md`, item 10: a shift of 0.06 to 0.45 m).
- **From a key:** each shoot's bearer and place must be recoverable, since the wood pass enters leaf at a bearer's node (`girth::lay`) and the drawing seats a twig on a node; a cell alone names no bearer.
- **At risk:** wood follows the leaf above (which phytomer receives a cell's leaf); the balance's per-subtree sums; retirement and letting go (a bearer leaves, its cell's count stays); determinism by key (a cell is a place, not a lineage; the lattice is anchored at the world's origin).
- **Measure first:** how many bearing units share a cell on each species; whether `R1-LAW.md` item 5's question (phytomer, stretch or cell) was ever answered beyond "on the growth unit" [unknown].

### O3. Exact draws, compact state (the first survey's B)

- **Counted, grain:** nothing is counted; the same leafy laterals keep their keyed draws but drop axis, pencil and phytomer records, their place regenerated from the key.
- **Yearly law:** today's draws, so the closed form's oracles hold as they are.
- **From a key:** position, frame and leaf at every pass that reads them (deposit, light read, wood) or only at the settle and the drawing; which reads can do without geometry is the question.
- **At risk:** bit-equality of the light read if the layout is not re-run; time (regenerating each year may cost what holding did).
- **Measure first:** bytes and seconds a leafy axis costs today by store; the first survey's bar was a gain under 1.5 times refutes it.

### O4. Cheaper records, the tree equal to the bit (fn-357 and fn-358's line)

- **Counted, grain:** nothing; a second record kind for closed axes (pencil unread; 24 of 72 phytomer bytes read).
- **Yearly law:** unchanged.
- **From a key:** nothing.
- **At risk:** none of section 4 if byte-equality is the proof, as before.
- **Measure first:** the host's estimate is "about a tenth more at 120"; the closed share of phytomers at 120 was 1.55 M of 4.45 M among the faint (census), the whole-tree closed share is in the census tables.

### O5. The drawn stage only: fine wood not built until a view resolves it

- **Counted, grain:** the grower is untouched; the conversion, the curve and the CPU mesh carry short-shoot and sub-pixel wood as a record per axis or per curve cluster, built from the phytomer keys on demand.
- **Yearly law:** none; the grown tree is the same tree.
- **From a key:** rings and leaves of a twig when a view's error admits them. The hooks that exist are in section 3; none builds on demand today.
- **At risk:** the 18 Sep ruling (far draw is the near draw minus what the eye cannot resolve); the 3 Oct ruling (no cull after growth); the field and the CPU mesh, which have no view; "a twig is the same twig whenever it is seen"; the gate's fixtures that count wood vertices.
- **Measure first:** the beech at 80's 74.8 M short-shoot vertices are CPU-mesh vertices at 20 sides; what the GPU path uploads and draws for the same tree (curve points, ribbon share) is [unknown]. Whether P1 and P3's aims are bound by growth or by the mesh, per species (the spruce at 40: grow 3.9 s, mesh 4.7 s).

### O6. Faint subtrees as scaled templates (the first survey's C)

Listed because it was on the table; **refuted by the census in the form proposed** (realised 0.04 to 0.36 of expectation). Open only with a law that carries light and shedding, which is O1's law applied to whole faint limbs, where kinds are not closed by table.

### What no option above answers

- The coarse rows' replacement for the website. The plan ties their deletion to counted foliage; none of the proofs measured a coarse tree, a Wasm time or the field against the counted tree [unknown].
- The oak's faint branches and boughs (not leafy, not confined to a cell, not closed by table).

## 7. Open questions for the owner or the host

**Owner.**
1. Does counting go before Phase 3, and how does Phase 2 land with old oaks that cost more than the base's (asked 12 Nov 07:00; no answer recorded)?
2. Is counting that changes the grown tree in expectation (O1, O2: fn-340 measured the oak's trunk 14 to 36% thick and its leaf at 120 at 0.81) inside "that is the growth", or must a counted tree equal the grown one to a stated tolerance? The 13 Oct rule says "beyond its measured tolerance"; no tolerance is recorded.
3. May the coarse rows be deleted before a replacement reaches their Wasm time (164 ms for the coarsest oak), or does the website keep a fidelity dial of some general form?
4. Is "3x cheaper is pretty good already" (10 Nov) still the bar, given the plan's aims of 100 ms and 15 s (host's numbers) against 1.58 s and 118 to 171 s measured?
5. Does a faint limb's miniature crown count as "leafy shoots that cannot leave a cell", or is carrying it outside what was approved on 11 Nov?
6. The beech at 80 at 101.8 M wood vertices: is a fuller beech at three times the wood acceptable until Phase 5 or 6, or does it block Phase 2's landing?

**Host.**
7. Which is "the proof on the oak, spruce and beech before the product is touched": a patch like fn-340's, or a product-shaped build behind no switch? fn-339 and fn-340 used an environment switch in a patch.
8. What is "a view resolves it" for the CPU entry, the field and the scoreboard's stills, which have no screen error unless given one?
9. Is the cell test the expected reach (fn-339) or the extent bound (`closed_form/kept/bound.rs`, test-only today)? The beech's shoot sits between them.
10. Does `shade_short` (a beech lateral's kind reads light) change which kinds close, and does `shed::lasting` need to know?
11. `Shoot::shed_certain` reads the age asked for. Is it live in the one cycle, and how does a count interact with wood left ungrown because it is certain to be shed?
12. Which leaf does the drawing show for a counted twig: the grower's (`girth::leaf`) or the drawn blade (beech 2.1 times its measured leaf, oak 1.4; `LEAF-ACCOUNT.md`)? The log gives the drawn leaf to Phase 5 "to rebuild".
13. Does Phase 5's exit ("every tree cheaper than at the last landing") read the grower, the mesh, or both, and on which measure (seconds, peak, vertices)?
14. Should the census readers in `.worktrees/census-p5` (uncommitted: `phase5.rs`, `expected_sub_counts`, `p5census.rs`) be kept as a probe for the measurements section 6 names?

## What was not read

`PHASE5-CENSUS.md`'s full per-tree tables (lines 149 on); `fn-356-*/TASK1-STOP.md` beyond its test lists; fn-209's FINE-CONTRACT.md; `P4-SURVEY.md`, `LEAF-SURVEY.md`, `fn-354-*/LEAF-ACCOUNT.md` (cited from the log); the render crate's shaders; `src/` and `harness/`; `docs/principles.md`, `docs/tree-space.md`. The first survey's line numbers were not re-checked against today's code.
