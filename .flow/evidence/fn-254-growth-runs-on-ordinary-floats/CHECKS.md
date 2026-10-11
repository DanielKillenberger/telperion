# R4: four checks replace byte parity

All figures below were measured on this branch (f32), with `cargo test --profile ci`. Device runs used the RTX 3080 under the GPU lock.

## 1. Repeatability within an executor
- **CPU:** `telperion-core` `repeat_tests::every_preset_grows_the_same_tree_again_and_on_any_cores` covers all 8 shipped presets. Each is built three times: on 4 cores, again on 4 cores, and on 1 core. The three `Structure`s are equal to the bit.
- **Closed-form rows:** the boundary rows are pinned on 1 and on 4 threads.
- **Stepped executor and build:** its items' partials are merged, and their leaves deposited, in the items' own order, whatever order and chunks the plan ran them in. Before, the review and Astra found that a shuffled or chunked plan regrouped the f32 sums. The build is now the same tree to the bit forward in 1s, reversed in 64s and shuffled in 7s (`the_build_is_the_same_in_any_order_and_chunks`). It is also the same on one core and four, every phase digest included (`the_stepped_tree_is_the_whole_tree`). The partition probe (`grow_partitioned`) keeps its deliberate regroupings, because they are its subject.
- **Device:** `every_root_settles_on_the_gpu_as_on_the_cpu` settles a tree four times: a fresh `Device` each time, sown or ordered, forward and reverse, in batches of 4,096, 777 and 64. All four give the same geometry digest. `every_root_grows_whole_on_the_gpu_as_on_the_cpu` grows three orders on reused tables.

**Kept in f64 (Astra):** the light sweep's optical depth and transmission (`light.rs`, `swept_all`). This is CPU preparation, R1 inventories it, and its result is rounded to f32 where growth reads it.

## 2. Exact discrete contracts
- **Draw thresholds:** `contract::tests` checks that a draw falls under an f32 chance exactly as its real uniform does, over 10^6 cases with endpoints named. `floor53` is exact.
- **Device decisions:** the device probe (`the_gpu_takes_every_decision_the_cpu_does`) holds every discrete selector exact: keys, draws, ticks, counts.
- **Topology:** the root chains (`every_chain_grows_on_the_gpu_as_on_the_cpu`) and the realized roots keep their axes by lineage, exactly, as do the kept axes after shedding.

## 3. Invariants
`telperion_space::invariants` holds on every shipped preset at its own age (the same test as check 1). It checks:
- links point back;
- geometry is finite;
- headings are unit to 1e-5;
- radius, scale and size are never negative;
- light stays within [0, 1];
- no node is more than whole;
- no wood stands more than 0.1 mm under the ground.

Other tests cover conserved vigour (`the_trees_vigour_is_conserved_at_every_lambda`) and budgets (`a_tree_past_its_budget_is_refused*`, `node_budget`).

## 4. CPU and GPU agreement by lineage
The agreement test is `preset_agree_tests::every_gpu_preset_grows_on_the_device_as_on_the_cpu`. Each preset grows at its own age and seed 42. The kept axes match exactly, so no wood goes unmatched and every major branch survives on both sides.

| Preset | Phytomers | Tip most (m) | Tip RMS (m) | Radius (m) | Crown apart (m) |
|---|---|---|---|---|---|
| norway-spruce 40 | 1,141,207 | 3.2e-5 | 9.8e-7 | 2.0e-7 | 3.9e-7 |
| european-beech 40 | 261,194 | 3.9e-5 | 2.9e-6 | 1.9e-7 | 5.4e-7 |
| telperion 92 | 3,749,026 | 3.5e-4 | 1.2e-5 | 2.7e-7 | 7.3e-6 |
| laurelin 80 | 2,683,717 | 3.9e-4 | 9.9e-6 | 5.3e-7 | 3.5e-6 |
| silver-birch 40 | 474,706 | 0.219 | 1.0e-2 | 3.9e-8 | 6.1e-4 |

**Bounds (about twice the measured):** tip most 8e-4 m, RMS 2.5e-5 m, radius 1e-6 m, crown 1.5e-5 m.

Relative radius gaps reach 3.9e-2. All of them sit on roots thinner than a micrometre, where the scale presence is near zero, so radii are bounded in metres.

**Silver birch:** contract A measured the same 0.219 m tip and 1.0e-2 m RMS at the base `59e627e7`, so this gap is not caused by the floats. It is held to its own measured bound, and it is an open item for the birch's own spec.

**Root presences and livings (25 years, seeds 1 and 7; beech, spruce, oak unlit, fractional buds):** at most 1.5e-4, against a bound of 3e-4. Base radii at 30 differ by at most 8.8e-5 relative, against a bound of 2e-4.

**Leaf area.** `resident_tests::every_gpu_preset_seats_its_leaves_as_the_cpu_path_does` covers the five GPU presets at 30, seed 42. The device's leaves are compared with the CPU path's by key and station. Every leaf matches, the counts are equal, and no leaf's scale is apart. On the silver birch, 42,548 leaves stand apart in place and 5,673 in direction; they follow the birch's pre-existing wood gap.

**Review fixes:**
- **Allocation:** the main axis's light is summed from the tip down, so a faint continuation is not rounded away under full apical control.
- **Survival:** a survival chance under a half keeps its own threshold on both executors.
- **Light:** the light sample is summed corner by corner and held to [0, 1].

Each fix has a regression test.

**On the CPU, deferred to fn-256 (R4a):**
- ordinary and oregon-white-oak, because they read light;
- date-palm, because its leaves stand on no regular twig stations.

## Cross-path agreement on one executor
The CPU's coupled, partition and stepped paths are compared against the whole tree by lineage (`agree::PATHS`). Over the oak, beech and spruce at 24, seeds 1 to 6:
- no node is unmatched;
- tips are at most 1.03e-5 of the height apart;
- scales differ by at most 3.6e-4;
- radii are at most 5.1e-5 of the thickest apart.

The beech and the spruce match to the bit.

## Re-pins
- **Boundary rows:** against contract A, 7,734 of 125,640 fields move, by at most 2.9e-7 relative.
- **Coarse trees:** every stand and node is kept. Tips move at most 8.9e-6 of the height, scales 1.5e-5.
- **Extent bound:** reach exceeds the bound by at most 2.2e-6 relative (1.35 m bound) and 9.8e-7 m (5 mm bound), over 101,271 laterals.

## After the merge into F4 (`3cb7793a`) and its review

**Telperion and Laurelin are now measured as production grows them, with their supernatural overrides applied.** Before, the agreement test and the repeatability test read the unapplied species (merge review).

| Preset | Phytomers | Tip most (m) | Tip RMS (m) | Radius (m) | Crown apart (m) |
|---|---|---|---|---|---|
| telperion 82 | 3,689,822 | 8.75e-4 | 1.73e-5 | 5.7e-7 | 8.4e-7 |
| laurelin 80 | 3,456,235 | 8.31e-4 | 1.76e-5 | 8.5e-7 | 2.6e-6 |
| european-beech 40 | 318,766 | 3.9e-5 | 2.9e-6 | 2.4e-7 | 5.4e-7 |

The other presets are unchanged.

**New bounds:**
- tip most 1.75e-3 m;
- tip RMS 3.5e-5 m;
- radius 2e-6 m;
- crown 1.5e-5 m.

**Birch leaves:** 42,546 of 60,084 leaves stand apart in place and 5,668 in direction, against bounds of 47,000 and 6,300. The birch's other leaf checks stay exact.

**Correction to check 2:** the device probe holds the discrete selectors exact (keys, draws, counts). Its time selectors are held to measured tolerances instead, not exactly:
- lived time within one tick;
- fine time within 4,096 fine units;
- waking within 256 ticks.

They sit at f32 resolution of the lived and waking times (`fine/tests.rs`).

**Fixed by the merge review:**
- A lever a hair off upright wrapped to minus a half turn. The fall is now taken by its sine and cosine, on the CPU and the device.
- A draw near one lost its lived time.
- Two near-certain chances conditioned on each other never shed.
- A small uniform's wait to wake rounded to none.
- The walk's noise floor measured across the jump it was judging. It is now taken outside the last sub-step, and the switch is still caught at full depth.
