# fn-211 task 4, step 1: P4's light error, and one light value per fine root at N = 1,024 (worker, 2026-10-06)

Step 1 is done. Steps 2 and 3 have not started: step 1 turned up a design question that comes before them, set out at the end with options for the host.

## The tool

`p4-light-probe.patch` applies to P4's commit (`6e79fbf2`, branch `fn-209-probe-light`). It ran in a scratch worktree, so the probe branch itself was not touched.
- **What it keeps:** the pass's last field before the tree's last cycle. That is its leaves, the coupling chains' own and the fine stand-ins', marked by kind, and each read: the axis, the point and the light.
- **What it compares (`examples/probe.rs`):** that field against the laid tree's field built from the same cycle's leaves, read at the same points. The total error splits into two parts:
  - **position:** the pass field at the pass's points against the same field at the laid points;
  - **field:** the pass field against the laid field, both at the laid points.
- **The field part splits again** by swapping one population of leaves at a time:
  - laid coupling leaves plus the pass's stand-ins;
  - the pass's coupling leaves plus the laid fine leaves.
- **Environment switches for experiments:** `PROBE_NO_SAG`, `PROBE_NO_BALANCE`, `COUPLE_LEAF_SCALE`, `COUPLE_STAND`, `COUPLE_FINE_GIRTH`, `PROBE_DEBUG`.
- **Output:** `raw/task4/probe-*.txt`.

## The cause of P4's error: three faults, measured on the oak at 80 (seed 1, cell 0.5)

| Read (p50 / p95 absolute) | Today's oak | P4 | P4, no sag in either tree | P4, no sag, stand-in leaf area scaled to the realized |
|---|--:|--:|--:|--:|
| coupling buds, total | 0.026 / 0.095 | 0.201 / 0.354 | 0.156 / 0.364 | 0.092 / 0.175 |
| fine roots, total | – | 0.185 / 0.349 | 0.169 / 0.352 | 0.081 / 0.178 |
| distance from pass point to laid point (m) | 0.42 / 1.61 | 2.22 / 6.42 | 0.002 / 0.08 | 0.002 / 0.08 |

Without sag, today's rough layout and re-lays match the final lay almost exactly: 0.0016 p50 and 0.009 p95 in light, at 80 and at 40.

1. **The stand-ins cast about twice the realized leaf area.**
   - The measurements: at cycle 79, 450 m² against 211 m² as the subtrees grew it, before shedding (seed 1). Seed 7 at 80 gives 1,210 against 623; seed 1 at 40 gives 86 against 46.
   - **Not the scale.** A realized phytomer's scale is 0.96 to 0.99 of its root's base scale, which is what the aggregate assumes.
   - **It is the population.** Over the cycles, the aggregates' expected phytomers run 1.45 to 1.76 times what the subtrees grow under the same records, from cycle 15 on (`PROBE_DEBUG` per-cycle tally).
   - **Not the balance shedding inside a fine subtree**, which is the aggregate's stated approximation. With the balance switched off the ratio widens to 2.5.
   - Which transition overcounts was not isolated.
   - Totals over the life agree to 0.2 % (4.32M against 4.31M). The metric P4 reported therefore hid this.
2. **The pass lays the coupling tree without its fine roots' wood.**
   - A fine root has no living apex in the grower, so the re-lay's `assign` and `shed` drop it as dead wood. Its loads are dropped with it: of 112k fine roots at cycle 70, `fine_loads` reached about 900. Its pipes are never counted.
   - The pass's trunk base radius at cycle 70 is therefore 0.28 m, against 0.47 m in today's pass.
   - The thin coupling wood sags too far, and coupling tips stand 2.2 m (p50) from where the final lay puts them, against 0.42 m today.
   - Re-lays without sag remove this part: position error p50 falls from 0.20 to 0.0001.
   - P4 found that switching its loads off changed nothing. That is because the loads almost never reached the tree.
3. **Each stand-in's leaves are put at one point**, its expected centroid along its first heading. With the area corrected and sag removed, this leaves 0.08 to 0.09 p50 and 0.18 p95.

The same holds for seed 7 at 80 and seed 1 at 40 (`raw/task4/probe-more.txt`).

## The finding that blocks steps 2 and 3: at N = 1,024, one light value per fine root cannot meet the bar

`examples/cutlight.rs` (committed) grows today's oak whole and lays it in full. It classes every bud by the cut's light-neutral predictor (`survey_laid`) and compares each living bud's light in the laid tree with its own root's mean. No record of one value per root can do better than that spread. Source: `raw/task4/cutlight2.txt`.

| Oak | Cut | Fine roots | Living buds that are fine | Phytomers above the cut | \|bud − its root's mean\| p50 / p95, % of mean bud light | Residual of a fitted gradient on height / on position, p95 % |
|---|--:|--:|--:|--:|--:|--:|
| 80 s1 | 1,024 | 7,284 | 99.9 % | 18,491 | 2.0 / **17.2** | 10.7 / 4.6 |
| 80 s7 | 1,024 | 17,643 | 99.8 % | 45,655 | 2.1 / **17.8** | 11.6 / 5.1 |
| 40 s1 | 1,024 | 1,508 | 99.8 % | 3,737 | 2.0 / **14.5** | 10.6 / 3.5 |
| 40 s7 | 1,024 | 2,698 | 99.8 % | 6,648 | 2.5 / **18.7** | 11.3 / 4.6 |
| 80 s1 | 512 | 45,214 | 98.5 % | 119,204 | 1.0 / 14.3 | 8.6 / 3.5 |
| 80 s1 | 256 | 102,556 | 96.0 % | 274,013 | 0.2 / 4.3 | 2.6 / 0.4 |
| 80 s7 | 256 | 246,280 | 95.7 % | 655,806 | 0.2 / 5.7 | 3.4 / 0.6 |

The gradient columns are least-squares fits on the laid tree's own light (an oracle, in sample). Roots with few buds fit exactly, so these are floors.

## Design question for the host: how a fine root's buds read light at N = 1,024

Decision 7 (PROPOSAL.md) has a fine subtree's buds read their root's one recorded value. At N = 1,024 that misses the 10 % bar by itself, before any error in the pass. The options:

- **A. A lower cut for light-reading species, about N = 256.**
  - The one-value floor at p95 is 4 to 6 %.
  - What stays on the CPU is 274k to 656k phytomers at 80, about a quarter of the tree, against about 2 % at 1,024. There would also be 100k to 250k aggregates.
  - The year passes that follow the whole tree (PROFILE.md: re-lay, balance, allot) would run over about a quarter of it.
  - The cut becomes a per-species value, with the non-light species staying at 1,024.
- **B. Keep N = 1,024 and record a light gradient per root per cycle**, a value and three spatial slopes taken from the pass's field over the root's expected extent (about 16 B per root per cycle).
  - The oracle floor is a p95 of 3.5 to 5.1 %.
  - A fine bud needs a position while it grows: a rough layout inside the executor, or a position predicted from the record's frame and the bud's chain.
  - The pass's own field error comes on top of the floor.
- **C. Keep N = 1,024 with one value, and change the bar.**
  - The floor is a p50 of 2.0 to 2.5 % and a p95 of 14.5 to 18.7 %.
  - The bar would be read per root (the root's mean against its record) rather than per bud, and the stills would judge whether this is acceptable.
- **D. The executor reads the pass's coarse field itself**, one shared field per cycle rather than one per root, at the bud's position.
  - This is the most faithful option.
  - It needs fine bud positions during growth, as in B, and the field's record for the cycles (about 50k cells × 80 cycles at 0.5 m on the oak at 80; not measured).

Whichever is chosen, the coupling pass also needs three repairs, all inside Astra change 1:
- an aggregate whose living population matches the subtrees grown under its records, held by the lineage oracle;
- fine roots that stay in the re-lays while their aggregate lives, carrying their live and disused pipes and their loads;
- stand-in leaves spread over the expected extent rather than put at one point.

Steps 2 and 3 (the oak above the cut grown year by year, the comparison against today's oak, the shade-then-release lineages and the stills) wait on this answer. They also need P4's coupling pass brought onto this branch: it lives only on `fn-209-probe-light`.

## How measured

- **Builds:** the `ci` profile. The P4 probe used its own scratch target and this task's worktree its own `target/`.
- **Load:** one-minute averages of 2 to 7 on a Ryzen 9 5950X with 32 threads. No timing is claimed in this step.
- **Commands:**
  - `probe 80 1 0.5,2`, `probe 80 7 0.5`, `probe 40 1 0.5`, with and without the switches;
  - `cutlight 40,80 1,7 64,256,512,1024`.

## The oracle, after the host's decision (2026-10-06): C and B both hold on today's oak

The host's question: does it matter for growth and look what a fine root's buds read? The oak was grown exactly as today, under today's exact light field. Only the light read by buds below the cut changed: N = 1,024, the predictor's classification, made per axis as the tree grows.
- **C:** each bud reads its fine root's mean light that cycle.
- **B:** each bud reads that mean plus the root's three least-squares slopes in its buds' tip positions, fitted that cycle.

How it was run:
- `grow_oracle` (`src/grow/oracle.rs`) is its own entry point. `grow` never builds an oracle, and the oracle's exact read is today's tree to the bit (a digest check on all four trees).
- The measures come from `examples/oracle.rs` (`raw/task4/oracle.txt`).
- The stills come from `space_oak --oracle exact|mean|slopes --light 0.5,0.5 --shots bare,whole`.
- Side by side (today | C | B; bare above, in leaf below): `raw/task4/stills/pair-oak-{40,80}-{1,7}.png`. Each was viewed.

| Oak | Read | Kept, all orders | Kept by order, worst | Per-PA growth, whole life (worst PA) | Worst cycle (PA) | Trunk base | Order-1 base radius over today's, p5 / p50 / p95 | Height | Leaf reach p95 |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|
| 40 s1 | C | −1.2 % | −1.5 % (4+) | −0.80 % | 2.1 % | −2.6 % | 0.95 / 1.01 / 1.08 | +0.0 % | −2.3 % |
| 40 s1 | B | −0.1 % | +0.6 % (2) | −0.11 % | 0.6 % | −1.0 % | 0.99 / 1.00 / 1.01 | +0.0 % | −0.4 % |
| 40 s7 | C | −0.3 % | −0.4 % | −0.67 % | 2.4 % | −2.5 % | 0.96 / 1.01 / 1.05 | +0.1 % | +0.9 % |
| 40 s7 | B | +0.0 % | +0.3 % | −0.06 % | 0.6 % | −0.2 % | 0.99 / 1.00 / 1.01 | +0.0 % | +0.1 % |
| 80 s1 | C | −0.5 % | −0.6 % | −0.90 % | 2.7 % | −4.7 % | 0.96 / 1.01 / 1.10 | −0.2 % | +0.7 % |
| 80 s1 | B | +0.0 % | −0.8 % | −0.46 % | 3.5 % | −1.2 % | 0.99 / 1.01 / 1.02 | +0.0 % | +0.3 % |
| 80 s7 | C | −0.6 % | −1.3 % | −0.95 % | 2.3 % | −6.7 % | 0.93 / **1.27** / 1.41 | −0.1 % | −0.1 % |
| 80 s7 | B | −0.5 % | −0.9 % | −0.33 % | 1.6 % | −1.1 % | 0.99 / 1.06 / 1.10 | −0.1 % | −0.0 % |

The worst-cycle column counts only cycles that hold at least 1 % of that PA's growth.

What it shows:
- **Neither read fails the growth bars.**
  - Kept phytomers stay within 1.5 % per order (bar 10 %).
  - Whole-life growth per PA stays within 1 % (bar 2 %).
  - Single cycles move by up to 2.7 % under C and up to 3.5 % under B.
- **The girth moves most, and more under C.** The trunk base is 2.5 to 6.7 % thinner under C and 0.2 to 1.2 % under B. At 80 s7, the order-1 base radius has a median of 1.27 times today's under C, against 1.06 under B.
  - This is likely the oak's `leaf_girth` (χ = 1). It thickens wood by the light it grew in, set against the tree's mean, so a root that reads its mean redistributes pipe. Not isolated.
- **The stills:** bare and in leaf, C and B cannot be told from today at the hero view.
  - Branch for branch they are the same tree, with small differences in the outer twigs.
  - The camera is fitted to each mesh's bounds, so the frame shifts slightly.
  - Pixels differing from today's in-leaf still: 1.7 to 1.9 % under C and 1.1 to 1.2 % under B (40 s1, 80 s1).
- Neither clearly fails, so the task carries on into the three pass repairs.

## Step 2: P4's coupling pass on this branch, the three repairs, and where it stops (2026-10-06)

### What was built

The coupling pass is rewritten on this branch rather than copied. It sits behind its own entry point, `grow_coupled(species, request, expected, (cut, split))`, and `grow` never builds it.
- **Classification:** each lateral is classed at the cut by the predictor (`count <= N`).
- **The aggregate state:** `src/couple.rs`.
- **The pass's hooks:** `src/grow/couple.rs` for classification, balance, allocation moments, records and advance; `couple/stand.rs` for the stand-ins, the re-lay loads and pipes, and the kept field.
- **The fine subtrees:** grown alone from their records by `src/grow/subtree.rs`, then joined and laid whole.
- **The tool:** `examples/couple.rs` with `examples/light/{field,fidelity}.rs`.
- **Proof it leaves today's trees alone:**
  - `measures hash 80 1,7 all both` matches the base `2e135350` for the beech, spruce, oak and palm, unlit and lit (`raw/task4/hash-{base,head}.txt`).
  - The test `a_pass_with_no_fine_root_is_todays_tree` holds a pass with no bud below the cut to today's tree, digest for digest.

### The three repairs

1. **The population: which step overcounts.** The step is P4's extinct aggregates, which kept casting their last unit's leaves every cycle after they died out. The unit transitions are not at fault: under controlled histories they track subtrees grown under the same records to within 0 to 6 %, on all five of P4's oracle histories.
   - Fixed in P4's code, its population matches: 0.99 to 1.02 of the realized subtrees per cycle at cell 0.5. The stand-in leaf area falls from 2.1× to 1.08× the realized.
   - In bud light without sag, fine roots go from p50 0.169 / p95 0.352 to p50 0.040 / p95 0.081 (`raw/task4/probe-stalefix.txt`).
   - Here, `Aggregate::advance` zeroes an extinct aggregate's leaves; the test is `an_extinct_aggregate_casts_no_leaves`.
2. **The re-lays carry the fine roots.** A fine root stands in the re-lays while its aggregate lives. Each hands its bearer the expected section and load of a subtree of its age then: the closed form's row for that age, the retained share included.
3. **Stand-in leaves are spread** over the subtree's expected extent: its centroid plus six points that give the row's spread.

The pass's light against the laid tree, at N = 1,024 with the per-light split, from the light tool. Each row turns one more repair on (`raw/task4/couple-repairs.txt`; one-minute load 15 to 87).

| Oak 80 | Repairs on | Coupling buds, total p50 / p95 | Fine roots, total p50 / p95 | Fine roots, position p50 | Stand-in leaf area against realized |
|---|---|--:|--:|--:|--:|
| s1 | none | 0.066 / 0.238 | 0.095 / 0.263 | 0.320 | 291 / 255 |
| s1 | population | 0.064 / 0.242 | 0.094 / 0.273 | 0.314 | 287 / 256 |
| s1 | + stands | 0.052 / 0.176 | 0.059 / 0.204 | 0.133 | 266 / 242 |
| s1 | + spread | 0.053 / 0.174 | 0.069 / 0.194 | 0.110 | 275 / 250 |
| s7 | none | 0.052 / 0.302 | 0.080 / 0.282 | 0.232 | 844 / 745 |
| s7 | + stands | 0.044 / 0.215 | 0.064 / 0.211 | 0.118 | 787 / 713 |
| s7 | + spread | 0.044 / 0.235 | 0.063 / 0.209 | 0.105 | 799 / 723 |

What it shows:
- **The stands help most.** They remove most of the sag error.
- **The population repair matters little at this cut.** Large roots seldom die out.
- **The spread does not reliably help.**
- **What remains is the stand-ins' field.** Fine roots stay at p50 12 to 17 % and p95 40 to 57 % of the mean light at 80.
  - Each root reads its light at its centroid, inside its own stand-in leaves.
  - The realized buds stand on the subtree's lit outside.

### Where it stops: two design questions beyond DESIGN-LIGHT.md

**1. How a fine subtree splits its root's recorded vigour (P4's question 1).** At N = 1,024 a realized subtree differs from its aggregate's expectation lineage by lineage. Neither split holds, and both are measured (`raw/task4/couple-{recorded,light}.txt`).
- **`Split::Recorded`, the subtree taking its whole recorded share (P4's rule):** a subtree that realized less presence than its aggregate gets far too much vigour per presence.
  - The tree comes out wrong: phytomer scales up to 249 against today's 8.9.
  - The trunk base is +213 to +507 % and order-1 junctions are 1.3 to 2.5 times today's.
- **`Split::PerLight`, the share per light the aggregate collected, times the light the subtree collects:** the scales stay sane (at most 6) and the trunk is −1 to −13 %.
  - But the growth of the fine PAs is −25 to −52 % (PAs 8, 9, 12 and 13).
  - The likely cause is the pass's per-PA means. These take each aggregate's (v/p)^ψ as its root's ratio^ψ, while a subtree's buds spread v/p by apical control. That is Astra's allocation moment Σp(v/p)^ψ, which the aggregate does not carry.
- Kept phytomers per order stay within 6 %.
- Options:
  - (a) Carry the moment in the aggregate. This needs a model of Borchert–Honda inside a subtree: per cohort, the share of vigour a bud of that PA and rank takes under λ.
  - (b) Take the subtree's buds out of the pass's per-PA means and normalise each subtree on its own sums (each fine root's buds average a size of 1 within it). Simple, but it drops decision 23's tree-wide normalisation below the cut.
  - (c) Let each subtree's growth feed back to the pass each cycle by growing subtrees alongside the pass. That is no longer a record, and against the cut.
- This report has not run the per-light split's stills or its growth bars, beyond these numbers.

**2. Where a fine root reads its light.** At N = 1,024 its buds span metres.
- **Today:** one read at the centroid of its own stand-in deposit, which self-shades. Bud light per fine root against its record is p50 24 to 26 % and p95 71 to 104 % at 80 (7 to 11 % and 30 to 35 % at 40).
- **The oracle:** one value per root (option C) costs the tree almost nothing, if it is the right value: the mean of the root's buds' own light.
- Options:
  - (a) Read at the subtree's expected bud positions: the stencil's outer points rather than the centroid, and without the root's own deposit.
  - (b) Record the field's slopes at the stencil (option B's data).
  - (c) Accept the read and let the stills judge.

### Cost (one-minute load 6 to 8, every core)

| Oak | Today | Coupled | Pass | Fine subtrees | Lay |
|---|--:|--:|--:|--:|--:|
| 40 s1 | 0.33 s | 0.16 s | 0.04 s | 0.03 s | 0.08 s |
| 80 s1 | 3.32 s | 1.23 s | 0.24 s | 0.29 s | 0.69 s |
| 80 s7 | 7.52 s | 2.99 s | 0.58 s | 0.70 s | 1.70 s |

- **Records:** 0.5 to 5.1 MB.
- **Not measured:** one core, and per year.

### Not done, waiting on the two questions

- the acceptance bars on the coupled tree;
- the stills of the coupled tree;
- the lineage oracle in shade-then-release against release-then-shade, at this cut and on this branch;
- the refusal in fn-209's executors, which stays.
