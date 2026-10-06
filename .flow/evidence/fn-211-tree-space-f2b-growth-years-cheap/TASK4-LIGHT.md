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
