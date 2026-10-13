# fn-255: COUNCIL.md's early tests 1 and 2 (worker, 2026-10-11)

These are the council's first two tests and nothing beyond them. The branch has F4 merged (`ba86e1da`).

- **Tools:** `crates/telperion-core/examples/early.rs` (modes `three` and `deposit`), with `grow_build`, `grow_census` and `Field::quantised` in `telperion-space`.
- **How it ran:** `--profile ci`, pinned to cores 0 to 3 (`RAYON_NUM_THREADS=4`), one process per build so each peak memory is its own, and one process at a time.
- **Raw output:** `raw/early/` (ignored).

## 1. Three-way CPU experiment

Each preset runs at its default age, seed 42. The three builds:

- **F4 Skip:** F4's build as it ships (`Certain::Skip`).
- **F4 Grow:** the same build with laterals certain to be shed grown (`Certain::Grow`).
- **Cycle:** the one cycle.

### Grown, time and peak memory

| Preset (age) | Grown: Skip | Grow | Cycle | Seconds: Skip | Grow | Cycle | Peak MB: Skip | Grow | Cycle |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| norway-spruce (40) | 1,258,554 | 2,872,640 | 2,872,640 | 0.50 | 0.58 | 2.86 | 523 | 621 | 1,302 |
| telperion (82) | 4,505,157 | 18,239,310 | 18,239,310 | 0.67 | 1.23 | 11.95 | 1,043 | 1,240 | 5,058 |
| laurelin (80) | 4,192,109 | 16,564,124 | 16,564,124 | 0.62 | 1.16 | 10.72 | 978 | 1,154 | 4,604 |
| european-beech (40) | 355,209 | 657,516 | 657,516 | 0.07 | 0.09 | 0.48 | 100 | 103 | 210 |
| silver-birch (40) | 692,565 | 1,161,350 | 1,161,350 | 0.17 | 0.21 | 0.76 | 182 | 194 | 362 |
| oregon-white-oak (40) | 1,213,974 | 1,213,974 | 1,213,974 | 1.09 | 1.15 | 0.84 | 545 | 545 | 419 |
| ordinary (40) | 389,743 | 389,743 | 389,743 | 0.48 | 0.46 | 0.39 | 197 | 197 | 154 |
| date-palm (40) | 480 | 480 | 480 | 0.00 | 0.00 | 0.00 | 11 | 12 | 11 |

- **All three builds keep the same tree** on every preset: the same kept phytomers and axes, kept ancestors and attachments.
- **Telperion under Grow is 18.24M, under the 20M budget.** The stop condition is not met; the margin is 9%.
- **The oak and the ordinary tree already grow every lateral,** because they read light. All three builds agree in what they grow, and there the cycle is the fastest.
- **Grow costs F4 little:** Telperion's growth goes up 4.0× and its time 1.8×. F4 grows its fine roots alone, with no layout and no light.
- **The cycle costs 4 to 10× F4 Grow for the same growth.** Its peak memory is 2 to 4×, because the whole grower holds every grown phytomer to the end: Telperion's 18.2M against F4's per-root drop.

**F4's phases** (from `Stepped`): the structure above the cut is 0.00 to 0.02 s for the unlit species. Nearly all their time is the fine roots grown alone and the tree settled: 0.42 to 1.12 s. For the oak the stepped structure's cycles take 0.82 to 0.88 s and its roots 0.25 s.

**The cycle's slowest phases (s)**, as measured under the census:

| Preset | Units | Sketch (layout) | ShadeField | Balance | Allot | Shed | Merge |
|---|--:|--:|--:|--:|--:|--:|--:|
| telperion | 3.01 | 2.03 | 1.76 | 1.67 | 1.07 | 0.56 | 0.33 |
| laurelin | 2.71 | 1.84 | 1.60 | 1.47 | 0.93 | 0.51 | 0.31 |
| norway-spruce | 0.34 | 0.53 | 0.31 | 0.42 | 0.28 | 0.17 | 0.13 |
| european-beech | 0.14 | 0.09 | 0.07 | 0.03 | 0.02 | 0.03 | 0.02 |
| silver-birch | 0.20 | 0.12 | 0.11 | 0.06 | 0.05 | 0.03 | 0.02 |
| oregon-white-oak | 0.19 | 0.11 | 0.12 | 0.08 | 0.07 | 0.04 | 0.03 |
| ordinary | 0.07 | 0.04 | 0.05 | 0.03 | 0.03 | 0.01 | 0.01 |

ShadeField under the census also includes building the quantised lattice beside the f32 one, about twice its cost alone. The census's seconds overstate it by that much.

### Records, hierarchy and lattice (cycle)

| Preset | Active apexes (busiest cycle) | Axes held | Phytomers held | Ancestors, all grown axes p50/p99/max | Ancestors, busiest cycle's apexes | Attachments per bearing axis p50/p99/max | Largest lattice | Nodes | Occupied |
|---|--:|--:|--:|---|---|---|---|--:|--:|
| telperion | 240,331 (82) | 1,532,978 | 18,239,310 | 25 / 46 / 63 | 30 / 51 / 63 | 7 / 33 / 60 | 50×44×28 | 61,600 | 0 |
| laurelin | 225,593 (80) | 1,396,481 | 16,564,124 | 24 / 44 / 61 | 29 / 49 / 61 | 7 / 33 / 59 | 49×43×28 | 58,996 | 0 |
| norway-spruce | 121,225 (40) | 797,858 | 2,872,640 | 5 / 7 / 7 | 6 / 7 / 7 | 22 / 54 / 234 | 13×15×24 | 4,680 | 0 |
| european-beech | 26,738 (40) | 66,923 | 657,516 | 16 / 22 / 25 | 17 / 22 / 25 | 6 / 31 / 51 | 21×16×14 | 4,704 | 0 |
| silver-birch | 54,825 (40) | 130,297 | 1,161,350 | 6 / 10 / 12 | 6 / 10 / 12 | 14 / 48 / 109 | 16×18×34 | 9,792 | 0 |
| oregon-white-oak | 27,249 (40) | 164,355 | 1,213,974 | 6 / 10 / 12 | 7 / 11 / 12 | 11 / 35 / 65 | 21×20×21 | 8,820 | 2,048 (23.2%; 27.9% over the run) |
| ordinary | 11,428 (40) | 64,183 | 389,743 | 8 / 13 / 16 | 9 / 14 / 16 | 1 / 29 / 98 | 19×20×26 | 9,880 | 2,820 (28.5%; 30.1% over the run) |

- **Active against history:** the busiest cycle is the last, and it holds 13 to 40% of the axes as active apexes. Every grown phytomer stays held to the end.
- **Ancestor depth:** Telperion and Laurelin reach 63 and 61 ancestors, with a p99 of 46 to 51. Every other preset is at most 25.
- **Attachments per bearing axis:** at most 234 (spruce's leader), with a p99 of 29 to 54.
- **Zero occupancy is real, not a defect of the census.** Only the oak and the ordinary tree carry any `leafArea` among the shipped presets. Spruce, birch, beech, palm, Laurelin and Telperion carry none, so they deposit nothing to shade. "Light rows sourced for every species" includes their leaf area. Until those rows exist, the lattice cost for those species is its box and reads alone.

### What it sets (the council's question)

**The cycle's extra cost is processing, not growth.** F4 Grow grows the same phytomers as the cycle in a tenth of the time, and the cycle's time sits in units, layout, light, balance and allocation over the whole held tree. Its extra memory is the transient representation: the full history held. Both of the council's levers apply, and processing is the larger.

## 2. Quantised deposit

The deposit is integer weighted area at q = 2^-20 m², u32 per node, overflow refused by name (`light.deposit`).

**Tests** (`grow::cycle::tests`, green):

- the quantised lattice reads the same to the bit in either order of its leaves;
- a node past its u32 is refused;
- zero and 1e-30 grow the same tree on both deposits.

### Frozen oak

f32 drives growth, and the quantised lattice is read beside it on the same leaves and buds every cycle:

| Oak | Buds | Mean abs difference | Largest | Buds differing at all |
|---|--:|--:|--:|--:|
| 40 s1 | 322,202 | 4.4e-6 | 3.0e-5 | 99.6% |
| 40 s7 | 446,369 | 3.6e-6 | 3.3e-5 | 99.7% |
| 40 s42 | 368,882 | 4.1e-6 | 7.4e-5 | 99.6% |
| 80 s1 | 3,352,778 | 6.1e-6 | 1.2e-4 | 99.7% |

### Through feedback

The quantised deposit drives growth. It is compared with the f32 cycle, and beside it is F4's build against the f32 cycle: two f32 builds that differ only in their sums' order.

| Oak | Quantised against f32: unmatched / tip / scale / radius | F4 against f32 cycle: unmatched / tip / scale / radius |
|---|---|---|
| 40 s1 | 16 / 9.4e-4 / 0.045 / 3.7e-4 | 4 / 5.6e-5 / 0.0080 / 1.2e-4 |
| 40 s7 | 8 / 2.3e-4 / 0.0048 / 1.9e-5 | 0 / 2.9e-5 / 0.0024 / 1.4e-4 |
| 40 s42 | 0 / 6.8e-5 / 0.0030 / 2.0e-5 | 0 / 7.4e-5 / 0.0007 / 4.2e-4 |
| 80 s1 | 73 / 2.7e-3 / 0.18 / 7.1e-4 | 388 / 9.4e-3 / 0.62 / 6.4e-3 |

- **Neither is within `agree::PATHS`** (0 unmatched, tip 2e-5, scale 6e-4, radius 1e-4), whose bounds were set at age 24.
- **The quantised deposit moves the tree by the same order as reordering the f32 sums.** It is larger at 40 on two seeds and smaller at 80.
- **Kept phytomers:** 357,617 against 357,601 (s1), 488,475 against 488,483 (s7), equal on s42, and 1,878,014 against 1,877,981 at 80.

### Viewed renders

The oak at 40, seed 42, whole, mid and close: F4's build, the cycle with f32 and the cycle with the quantised deposit. The sheets are `raw/early/stills/oregon-white-oak-*-sheet.png`, and every one was viewed. The render came from a throwaway route patch (`raw/early/stills-deposit.patch`, not committed).

**Pixels differing from F4's** (difference image; any channel / more than 16 levels):

| View | Cycle, f32 | Cycle, quantised |
|---|---|---|
| whole | 15.2% / 2.4% | 15.0% / 2.5% |
| mid | 40.3% / 4.7% | 40.2% / 5.9% |
| close | 53.2% / 6.4% | 52.9% / 7.0% |

The three cannot be told apart by eye at any view: the same crown, limbs and leaf masses.

### Stop conditions

- **Overflow:** none on the oak. A node holds up to 4,096 m² of weighted area, and the overflow refusal is tested.
- **Fidelity:** the quantised deposit is within f32's own order noise, frozen and through feedback.
- **Continuity was not walked.** A leaf fading below half a quantum (5e-7 m² of weighted area) a corner deposits nothing, which is a step of at most that much area. The walks were outside tests 1 and 2.
