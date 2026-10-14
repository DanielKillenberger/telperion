# fn-276 R1 and R2: the refusals, their limits and the CPU's counts (worker, 2026-10-14)

Raw output is in `raw/` (ignored): `r1-red.log`, `r2-limited.log`, `r2-full.log`, `r2-full2.log`, `r2-all.log`, `r2-trace.log`, `r2-cpu-rounds.log`, and the throwaway diagnostic patch `r2-diag.patch` (reverted, not committed).

## R1

`generation::cpu_builds_tests` (110d35e97) builds each family on the CPU (`mesh::build`) and then on the device (`Generator::prepare`, resident). The CPU builds every family; the device refuses all seven:

| Family | Device refusal |
|---|---|
| beech to spruce at 40, seed 7, t 0.00037 | cycle 9: UnitRounds |
| seed 7, t 0.01 | cycle 6: UnitRounds |
| seed 7, t 0.1 | cycle 6: UnitRounds |
| seed 1, t 0.00037 | cycle 15: UnitRounds |
| seed 1, t 0.01 | cycle 8: UnitRounds |
| seed 1, t 0.1 | cycle 4: UnitRounds |
| Laurelin at 20, seed 7 | cycle 18: Given |

## R2, UnitRounds: three stepped limits, one cause

`build_size::limits` gives each cycle the units rounds the census read two cycles earlier used, plus `ROUND_SLACK` (2). It gives the layout's generations the deepest read plus 2, and the hierarchy's generations the generations read plus 2 (`STEP`). The slacks were measured on shipped presets at their default ages, where rounds rose by 2 at most over two cycles (fn-256 PROOF.md).

Per cycle on the CPU, the most units one living apex grew. This was a throwaway depth counter on `Shoot::advance`, which recurses through `carry_on` for each unit grown in the same cycle:

- **Beech at 40 (t = 0):** 1 unit, and 2 at a few late cycles.
- **Walk frame at t = 0.00037, seed 7:** 1 for cycles 2 to 8. Then cycle 9: 3, 10: 2, 11: 1, 12: 4, 13: 4, and 4 in most cycles after.
- **t = 0.1:** 3 every cycle from cycle 6.

On the device with every stepped limit at its plan bound (`Limits::full`), cycle by cycle:

- **Rounds:** a cycle used 2u - 1 rounds, where u is the CPU's units on one apex. The rounds rose from 1 to 5 or 7 in a single cycle.
- **Layout generations:** the deepest rose from 1 to 4.
- **Hierarchy generations:** rose from 1 to 4, a cycle later.

With those limits lifted, all six walk frames build on the device. The instance counts equal the CPU's: 69,435, 75,182, 83,416, 33,476, 33,111 and 56,838.

Each of the three limits refused in turn as the one before was lifted:

- **UnitRounds:** a limit of 3 against 5 to 7 needed.
- **LayGenerations:** a limit of 3 against 4 needed.
- **HierGens:** a limit of 3 against 4 needed.

**Why the counts jump:** six of the walk's 15 ages have a lifespan of 0 at the beech, which passes an apex through them without growing a unit. A walk mixes the lifespan by value. At t = 0.00037 those ages last 0.0004 to 0.0014 cycles (spruce: 2 to 40), so an apex grows a sliver unit in each age it enters. Up to 4 units are chained in one cycle, each a new axis generation. Such a chain appears when an apex first enters those ages, so the count steps rather than trends. No projection from earlier counts foresees it.

The family is valid, and the CPU builds it. The cap is at fault, not the tables and not the parameters. The plan bounds held: 8 rounds against 7, with layout and hierarchy generations at 4. A chain of 5 units would need 9 rounds and pass the bound of 8.

## R2, Given (the Laurelin at 20): the lattice's node table

The dispatch over the lattice's nodes (`Div(At(GRID+6), 64)`) was given 64 workgroups and needed 68. Its span is capped at the node table, 4,096 rows. The CPU's lattice at cycle 18 holds 4,290 nodes, with box [13, 11, 30]. The table moved to 6,144 rows only at cycle 19.

| Cycle | CPU lattice nodes | Box |
|--:|--:|---|
| 15 | 2,340 | [10, 9, 26] |
| 16 | 2,673 | [11, 9, 27] |
| 17 | 3,360 | [12, 10, 28] |
| 18 | 4,290 | [13, 11, 30] |

`build_size::grown` sizes the table for cycle 18 from the census of cycle 16. Its rate is cycle 16 over cycle 15, applied twice and divided by `ROOM`:

2,673 x (2,673 / 2,340)^2 / (7/8) = 3,986 rows

That fits inside 4,096 rows, so there was no move. But the box's sides stepped together, and the node count grew 1.6 times in two cycles.

The dispatch span (`given.rs`) already projects the lattice by its box: each side grows by three of its last steps (at least two cells), and the box is doubled. `grown` does not use that rule for the node table, and the span is capped at the table. With all limits lifted, the Laurelin is still refused here, so this is a table and not a stepped limit.

## Proposed fixes (not built; for the host's ruling)

1. **Units rounds and both generation limits get a floor from the species.** The most units one apex can grow in a cycle, u, is a closed form over the age table on the tick grid: the longest run of ages along a succession whose lifespans sum under one cycle, plus one. Each cycle then takes the larger of the observed projection and a floor from u: 2u - 1 rounds, and u layout and hierarchy generations. Where that floor passes a plan bound, the plan bound is raised to it. A species past a hard cap is refused by name before submit.

   This is by degree: a sliver age adds to the floor exactly as it adds units, and no branch is needed for walk frames. It needs checking:
   - the closed form against the CPU's per-apex count on every preset and on the R4 frames;
   - relays of failed units, whose chains may add to u;
   - its cost on the shipped presets, where u is expected to be 1 or 2.

2. **The alternative to fix 1:** record those three limits at their bounds every cycle. That gives back what fn-256 task 12 saved, 2 to 3.5% of GPU time on Telperion, the oak and the beech. It still refuses a chain past the bound.

3. **The node table grows by the box rule the dispatch span already uses** (`given.rs`): each side grows by three of its last steps, at least two cells, and the result is doubled. One rule then serves the table and its dispatch. Cost: a larger node table earlier on young trees, to be measured by the census.

## The other worker's branch (fn-270-grow-budget, `.worktrees/growbudget`)

Its commits past this branch touch `hier_blocks.wgsl` (one parenthesis) and the harness. Its uncommitted work touches the ledger's `Owned::destroy` and the grow-in's peak. Neither branch touches `build_size.rs`, `given.rs`, `gate.rs` or `plan_units.rs`, so it does not change these limits or the node table's growth.
