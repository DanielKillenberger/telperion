# fn-211 task 5: the oak stepped on the GPU (worker, 2026-10-07)

The task's description has three parts. The merge is done. Probe (b), synchronisation, is measured. The device executor is not built: one design question stops it, below with options.

## Answer in brief

- **Merged** (`f929dbeb`): fn-209 F1 through task 8 is in this branch. F1's driver, its canonical lineage order, the Q32 presences, the count-only predictor and the GPU's realized executor sit beside F2b's stepped executor, order-free sums and probes.
  - **The beech, spruce, palm and unlit oak** have every one of the 32 lineage digests equal to F1's tip. That covers four species at ages 40 and 80, seeds 1 and 7, neutral and lit.
  - **The lit oak through the stepped executor** equals `grow`'s digest at 40 and 80, seeds 1 and 7.
  - **The lit oak's topology by lineage** equals both tips' in all four trees. The topology is the sorted identities with each axis's phytomer count.
  - **Its fields move, because each tip lacks the other's arithmetic.**
    - Against F2b's tip, which lacks F1's Q32 presences: tips move at most 0.30 µm and radii at most 2.8e-4 relative. That is F1's measured move for the oak (FINE-CONTRACT.md, "Host decision 8 built").
    - Against F1's tip, which lacks F2b's order-free sums: tips move at most 1.6 nm and radii at most 3.8e-6.
  - The `telperion-space` tests and the GPU's fine tests are green.
- **Probe (b), synchronisation alone, oak at 80:** 330 round trips cost 113 ms (seed 1) and 179 to 282 ms (seed 7). This is the floor a stepped device executor pays before any growth.
- **Not built:** the stepped device executor, the canonical light arithmetic, lifting the oak's refusal and the oak's timing. One design question stops them.
- **The design question.** Every continuous value of the rough layout reaches a topology decision through light. That covers positions, wood, sizes, the balance's sums and the re-lay's girth and sag. The task's list of canonical arithmetic omits positions. So identity by lineage on the GPU needs either:
  - the whole rough layout in a specified arithmetic on both executors;
  - or a measured flip rate in place of a guarantee.
- **Measured:** perturbing the light field's inputs at 10 to 100 times the GPU's expected error flips nothing in the four lit oaks. Flips begin at 1e-4 relative light and 1 mm of position.

## 1. The merge

Every conflict was a place where both branches had changed the grower.
- `grow.rs`:
  - F1 factored the seed into `Grower::seeded` and the whole-tree test into `keeps`;
  - F2b factored `start` (the grower before its first cycle) and `year` (one cycle), and added the probes' hooks.
  - The merge keeps both: `start` builds through `seeded` and returns the grower; the probes' fields join F1's `handed`.
- **Two methods named `mark`.**
  - F1's `mark()` marks every living apex before the settle.
  - F2b's probe hash `mark(cycle, k)` is renamed `hash_phase`.
  - The stepped executor's own `Executor::mark` is unchanged.
- **The re-lay's lever gather** takes F1's new signature, borne loads and given loads, with none given.
- **`lib.rs` and `fine.rs`:** F1's exports, plus F2b's `survey_laid`.
- `examples/digests.rs` now also prints the lit oak through the stepped executor.
  - Command: `cargo run --profile ci -p telperion-space --example digests`.
  - Raw output, gitignored: `raw/task5/` and the scratch copies. The tips were run in two temporary detached worktrees with their own `target/`.

**The stepped executor's open question 1 is settled by F1's merge.** That was the axis order of an assembled tree, which made renders differ from today's. Every `grow` now returns its tree in canonical lineage order, so the order no longer depends on how the tree was built.

## 2. What a stepped device executor would be paying: probe (b)

`crates/telperion-render/src/generation/fine/step_probe.rs` (`stepping_costs`, ignored; `raw/task5/stepping-costs.txt`) runs PHASES.md's exchange schedule with persistent device buffers. Each trip has three parts:
- the records going down are written;
- a kernel runs a thread per record and only touches them;
- the records coming up are read back.

No growth, lay or sweep runs, so what is timed is the cost of synchronisation alone.

**Per cycle, four trips:**
- (i) balance: 3 words a root each way;
- (ii) demand and vigour: 2 each way;
- (iii) the normalisers: each PA's two Q64.64 sums, each way;
- (iv) the lattice's Q64.64 deposits up (4 words a node) and its swept light down (2 words a node).

**A re-lay** adds two trips: each root's section, pipes, mass and moment up and its bearer's frame down, then the bent pass.

The probe runs 79 cycles (the last cycle lays and sheds no light) with 7 re-lays. The fine roots are at cut 1,024. The lattice spans the lit oak at 0.5 m.

RTX 3080. Load 2.4 at the start and 4.3 at the end.

| Oak | Roots | Lattice nodes | Trips | Total ms, cold / warm | Per trip, median / max ms | Submit / wait / read ms (warm) |
|---|--:|--:|--:|--:|--:|--:|
| 40 s1 | 1,821 | 14,256 | 162 | 12 / 11 | 0.06 / 0.24 | 4 / 7 / 1 |
| 40 s7 | 2,937 | 15,456 | 162 | 13 / 17 | 0.09 / 1.96 | 6 / 10 / 1 |
| 80 s1 | 7,179 | 78,660 | 330 | 114 / 113 | 0.21 / 2.15 | 34 / 74 / 5 |
| 80 s7 | 17,370 | 168,740 | 330 | 282 / 179 | 0.24 / 1.86 | 54 / 113 / 13 |

What the table shows:
- **The floor is 3 to 8 % of today's oak at 80 seed 1** (3.65 s). It does not decide the design, but it is not free.
- **The lattice is most of it at 80.** Its nodes outnumber the roots ten to one, and it crosses both ways every cycle.
- **Sweeping on the device would halve the lattice's traffic.** The CPU's coarse leaves would go up instead, at 2 to 4 % of the phytomers. This is an option for the executor, not a design question.

## 3. The design question: the positions light reads are not on the task's list

### What the code shows

The task's second bullet asks for canonical arithmetic for five things:
- the lattice deposits (already canonical, `fixed/sum.rs`);
- the sweep;
- sampling;
- shade survival;
- the balance hazard.

Every one of these reads its inputs from the rough layout, and each read reaches a topology decision:

| Continuous value (site) | Decision it reaches |
|---|---|
| Leaf and bud positions: the sketch's pencil walk (`geometry::Layer`, trig and tropisms; `grow/draw.rs`) and the re-lay's thicken, place and sag levers, written back every 10 cycles (`grow/relay.rs`) | The trilinear deposit and read weights (`light.rs`, `corners`), and from those every light below |
| A bud's light | Shade survival `viability^(light^-φ)` (`advance.rs:50`), a D5 threshold; the balance's Q; the allotment's lit presence |
| Wood, `internode × scale` (`draw.rs:292`) | The balance's upkeep C |
| The balance's Q and C summed child to parent in f64; the remembered balance; the hazard `-expm1(-h)` (`balance.rs`) | Shedding (`d.under(p)`); the kept share `windows.decided`, which enters the Q32 presences and so the fade, D17 |
| The vigour split by λ and the size `(v/p)^ψ` (`allocation.rs`, `sketch.rs`) | The unit's length, then positions and wood, then all of the above in the next cycle |

FINE-CONTRACT.md's rule ("continuous values that set geometry and never topology stay within the screen") does not hold under light. Every value of the rough layout is on the path to a decision.

### Measured: how far the oak is from a flip

The probe patch is `task5-sensitivity-probe.patch` (not committed); raw output is `raw/task5/sensitivity*.txt`.
- **The setup:** the lit oak (`grow`) was grown with its light field's inputs moved:
  - every deposit and read position rounded to f32, or moved by deterministic noise of a given size in metres;
  - or every read light scaled by (1 + ε·h), with h in [-1, 1).
- **The measure:** the moved tree's axes by identity against the unmoved tree's. A control with ε = 0 is identical.
- **The perturbation reaches the trees:** every run's digest moves. So zero flips is not a dead probe.

| Perturbation | 40 s1 | 40 s7 | 80 s1 | 80 s7 |
|---|--:|--:|--:|--:|
| positions to f32 | 0 | 0 | 0 | 0 |
| positions ± 1 µm | 0 | 0 | 0 | 0 |
| positions ± 25 µm (the GPU's worst tip, F1) | 0 | 0 | 0 | 0 |
| positions ± 1 mm | 0 | 0 | 0 | 2 |
| positions ± 1 cm | 1 | 3 | 32 | 403 |
| light × (1 ± 1.2e-7), f32's epsilon | 0 | 0 | 0 | 0 |
| light × (1 ± 1e-6) | 0 | 0 | 0 | 0 |
| light × (1 ± 1e-4) | 0 | 0 | 1 | 64 |
| light × (1 ± 1e-2) | 235 | 231 | 1,587 | 5,569 |

Each cell counts the axes present in only one of the two trees.

What the table shows:
- **Positions and light do reach topology.** At 1 mm and 1e-4 axes change, and the count grows with the perturbation.
- **At the GPU's expected error, nothing flips in these four trees.**
- **Extrapolated linearly from 1e-4 (an estimate, not a measurement):** at f32's epsilon, an oak at 80 differs by about 0.001 to 0.08 axes a tree.
- **The probe covers less than a device executor would.** It moves only the light field's inputs. A device executor in f32 would also round the balance's sums, the sizes and the wood.

### Options

**A. One specified arithmetic for the whole rough layout and its feedback, on both executors.** This is fixed point, as host decisions 1 and 8 chose for thresholds and presences. It covers:
- the pencil walk and its trig;
- the re-lay's thicken, place and sag levers;
- wood, the balance's sums, memory and hazard;
- the vigour split and sizes, the deposit weights, the sweep, sampling and shade survival.

The final lay after growth reads no decision except D19, which stays above the cut, so it can stay as it is.

What it gives and costs:
- **Exact by construction:** identity by lineage holds for any tree, seed or retune.
- **It moves today's oak at rounding level.** It is measured as before, on the 28-tree dump and the stills.
- **It is the largest option.** About 1,400 lines of layout, girth and sag on the CPU are on that path, plus their device mirrors.
- **The worker's sizing:** three to four sessions for the arithmetic, plus two to three for the stepped device executor itself (balance, allotment, waking, the sketch and the re-lay's exchanges on the device).
- **It is two arithmetics for one stage,** the rough layout's and the final lay's, unless the final lay adopts it too.

**B. IEEE binary64 as written, emulated on the device for the whole feedback loop.**
- It is exact if every operation and transcendental is reproduced bit for bit.
- Host decision 1 rejected this shape for the thresholds ("a large port whose one wrong bit flips topology"). Here the port is larger, because the trig, `powf` and `exp` sit in the layout as well.
- Listed for completeness.

**C. Light-reading roots stay on the CPU.**
- The oak's fine roots grow on the stepped CPU executor. The GPU serves the species whose fine roots read no light, as F1 built it.
- Nothing is ported; identity is the CPU executor's, to the bit.
- The oak's speed then comes from the CPU alone. The stepped executor is 1.1 to 1.4 times today's per year (PHASES.md, "Cost").
- The task's GPU goal for the oak is withdrawn.

**D. The device in f32, with a measured flip rate in place of a guarantee.**
- The device steps the roots in f32 geometry, as F1's lay does. The light arithmetic of the task's list is optional.
- The CPU stepped executor stays the reference. Identity by lineage becomes a measured property, held by tests on chosen trees, such as the four oaks above, rather than a construction.
- Measured above: no flip in four trees under perturbations of 10 to 100 times the expected error.
- Not measured: the full device's own error sources (the balance's sums, sizes, wood).
- A flip would be a botanically valid tree that differs from the reference by an axis or a subtree.
- This is the shape the host declined for D17 (REALIZED.md, option D). It is listed with this case's numbers because here the alternative, A, is far larger than D17's was.

**Why quantising alone does not help:** rounding positions or light to a coarser grid before a decision still flips wherever the true value straddles a grid line. The flips are rarer per decision, but there is no guarantee. Only identical inputs, as in options A or B, give exactness.

**Worker's reading, not a recommendation of record:**
- If identity by lineage must hold by construction, A is the only way, and it is large.
- If the oak's GPU path is wanted soon, D is the measured route, with the CPU executor as its reference and a lineage test as its guard.
- C costs nothing now and keeps the oak exact. It stays slow.

## Acceptance against the task

| Criterion | State |
|---|---|
| The oak identical by lineage to the stepped CPU executor on the GPU, at 40 and 80, seeds 1 and 7, geometry within the screen | **not met.** The device executor is not built; the design question above stops it |
| The canonical light arithmetic specified, mirrored and measured on today's trees | **not met.** Its scope is the design question: the task's list omits the positions light reads |
| Round trips counted; the oak timed warm and cold against today | Round trips: **counted and timed** for the synchronisation floor (probe (b)). The oak's timing waits on the executor |
| Workspace gate green | not run: no product change since the merge's crate and GPU tests |

The merge (step 1 of the brief) is met.
