# fn-198 design options: the engine in production (worker, 2026-10-05)

This is a proposal for the host. It changes no code. Every code claim is marked **checked** (file:line, read on this branch at `986a16a1`), **measured** (run on this branch), or **unknown**.

## 0. The short of it

- **The speed bar is not reachable as written.**
  - R2 asks for engine growth within 1.5x of today's skeleton. Engine growth is **57x to 409x** today's skeleton time.
  - The cause is size, not per-node cost:
    - The engine keeps 1.3M to 4.6M nodes at about 1 cm each. Today's skeleton has 91k to 215k nodes.
    - At today's own cost per node (about 0.27 µs), the engine's kept nodes alone would take 0.4 to 1.2 s, against a bar of 45 to 87 ms.
  - R2's condition "at equal or greater fine wood" holds only for the spruce. The engine's beech and oak carry 38 to 51% of today's fine wood.
  - The host has to restate the bar, or the wrong-path stop fires (section 1).
- **Shedding can be decided before growth.**
  - 75 to 79% of grown phytomers are shed.
  - 92 to 97% of those sit in subtrees whose shedding is certain at the bud's birth, from its PA's lifespans and delay alone.
  - Skipping those subtrees removes 72 to 75% of grown phytomers. For the beech, spruce and palm it is exact. For the oak it is not, because shade, vigour, balance and retained girth read the subtrees while they live (section 2a).
- **One exact light saving is already proven.**
  - The last cycle's rough layout, full re-lay and sweep are read by nothing.
  - Skipping them leaves the oak's hash identical at seeds 1 and 7.
  - It saves the largest re-lay, 1.2 to 1.6 s an oak (section 2c).
- **Drawing is the wall for the oak's web and the spruce's curtains.**
  - The spruce draws 198M to 207M wood triangles, about 5.6 GB of GPU wood buffers, and 16M needles.
  - Every ring has 12 sides whatever its radius, and fine wood is 92 to 98% of wood length (section 2d).
- **Order:**
  1. Drawing levers in core now; they do not depend on fn-206.
  2. Exact engine levers once fn-206 merges.
  3. fn-199 on fn-206's model.
  4. Integration into stage 2, with the station contract.
  5. Re-measure (section 5).

## 1. The cost table

**How measured.**
- Branch `fn-198-…` at `986a16a1` (origin/master), release profile, this worktree's own `target/`.
- One process per tree, at 80 years, seeds 1 and 7, in each species' production light: the oak at extinction 0.5, sky 0.5 (fn-195 RESULT.md, round 5); the beech, spruce and palm at neutral light, since their light-reading values are all 0 (`beech.rs:58`, `spruce.rs:70`, `palm.rs:57`, checked).
- Dressed as the stills runner dresses (`examples/space/still.rs`, each species' `ROWS`).
- **The probe** is `cost-probe.rs`, a copy of the scratch example `f_cost`. It times `grow_staged`'s stages, the conversion (`examples/space/tree.rs::convert`), `executor::expand`, the CPU wood alone and the CPU mesh (`Expansion::mesh`, wood and leaves concurrently, as the baseline's full build).
- **The grown count** comes from `cost-probe.patch`, a temporary `eprintln!` of `Grower::grown` (reverted, not committed).
- **The timed run** is `raw/cpu3.log`, at a 1-minute load of 3.0 to 4.2 on 32 threads (the baseline's was 2 to 3).
- **Two earlier passes** ran at loads of 4 to 25 (`raw/cpu1.log`, `raw/cpu2.log`, `raw/probe.log`). They give the same counts. Their times run up to 2.6x slower (FRICTION.md).

**Hashes.**
- At neutral light, the beech's and spruce's structure hashes equal fn-197's recorded ones (`9b15095b…`, `c98a7af3…`, `b51d107c…`, `204b4664…`; fn-197 STEPS-1-2.md:51-58).
- So the build measured is master's engine.

### Stage times (ms) and counts

| | palm 1 | palm 7 | beech 1 | beech 7 | oak 1 | oak 7 | spruce 1 | spruce 7 |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| Growth (cycles) | 0.3 | 0.4 | 2,669 | 1,949 | 3,429 | 4,078 | 5,139 | 5,407 |
| Rough layout | – | – | – | – | 1,352 | 1,622 | – | – |
| Full re-lays (8) | – | – | – | – | 3,263 | 3,896 | – | – |
| Light sweep | – | – | – | – | 523 | 496 | – | – |
| Settle (sizes, shed, girth) | 0.0 | 0.0 | 959 | 730 | 1,333 | 1,655 | 2,581 | 2,721 |
| Final lay | 0.2 | 0.3 | 324 | 268 | 675 | 786 | 3,083 | 3,039 |
| **Engine total** | **0.6** | **0.7** | **4,134** | **3,077** | **10,770** | **12,788** | **11,410** | **11,798** |
| Conversion to a pipeline tree | 0.0 | 0.0 | 251 | 191 | 143 | 168 | 623 | 591 |
| Expansion's preparation (clothe, element, reference) | 1.8 | 2.0 | 10 | 9 | 6 | 7 | 28 | 23 |
| CPU mesh (wood ∥ placement, cull) | 2.4 | 2.5 | 1,731 | 1,467 | 990 | 1,035 | 13,202 | 13,935 |
| CPU wood alone (inside the mesh) | 2.4 | 2.4 | 587 | 504 | 398 | 411 | 1,408 | 1,409 |
| **Full build** | **4.8** | **5.2** | **6,125** | **4,744** | **11,909** | **13,997** | **25,263** | **26,346** |
| Phytomers grown | 960 | 960 | 9.55M | 7.04M | 7.04M | 8.37M | 17.95M | 18.47M |
| Phytomers kept | 960 | 960 | 2.04M | 1.69M | 1.61M | 1.80M | 4.43M | 4.62M |
| Shed | 0% | 0% | 78.6% | 76.0% | 77.1% | 78.5% | 75.3% | 75.0% |
| Pipeline nodes | 961 | 961 | 1.94M | 1.62M | 1.34M | 1.44M | 4.32M | 4.51M |
| Wood / fine wood (km) | 0.03 / 0 | 0.03 / 0 | 25.2 / 23.8 | 22.6 / 21.3 | 13.9 / 12.9 | 14.7 / 13.6 | 48.9 / 48.0 | 52.0 / 51.0 |
| Wood triangles | 69,160 | 69,160 | 83.7M | 70.0M | 58.0M | 62.5M | 198.2M | 206.8M |
| Leaves (needles, leaflets) | 6,554 | 6,554 | 3.20M | 2.83M | 1.74M | 1.84M | 15.51M | 16.48M |
| Peak RSS, whole CPU build (GB) | 0.007 | 0.007 | 4.43 | 3.49 | 3.41 | 3.96 | 9.38 | 9.74 |

**More from the same runs:**
- **The oak's eight full re-lays** (cycles 10 to 80) took 3, 18, 55, 145, 344, 612, 844 and 1,243 ms at seed 1, and 5, 20, 58, 176, 414, 715, 1,073 and 1,435 ms at seed 7. The last is 38% of the stage.
- **Engine-only peak memory** (`measures stages`, `raw/measures.log`, cumulative in one process): the beech 2.3 GB at neutral light, the spruce 5.4 GB. fn-197 recorded the beech 2.09 GB, the oak 2.01 GB in light, the spruce 4.87 GB.
- **The GPU stills runner** (`raw/gpu.log`, under the GPU lock, `--shots whole`, at loads of 10 to 49, so the times are noisy):

  | Species | Wall per tree | Peak RSS, process with the renderer |
  |---|--:|--:|
  | Palm | 0.3 to 0.9 s | 0.2 GB |
  | Beech | 8 to 11 s | 5.5 to 7.0 GB |
  | Oak | 18 to 31 s | 5.2 to 5.8 GB |
  | Spruce | 46 to 62 s | 15.1 to 15.6 GB |

  - No run failed. fn-196 saw out-of-memory on three of six spruce launches.
- **The GPU wood buffers are estimated, not measured.** 32 B a vertex and 12 B a triangle (`WoodExtent::bytes`, `crates/telperion-core/src/pipeline/surface.rs:76-90`, checked):

  | Species | Wood vertices | Wood buffers |
  |---|--:|--:|
  | Spruce | 100M to 105M | about 5.6 GB |
  | Beech | 35M to 42M | 2.3 GB |
  | Oak | 29M to 31M | 1.6 GB |

  - Leaves are about 67 B each on the CPU (`foliage.rs:251`, "seven million leaves is 470 MB"), so the spruce's needles are about 1.1 GB.
  - GPU memory itself is unknown: no tool reports it.
- **Triangles per pipeline node:**
  - 43 to 46 on every species.
  - Every ring is cut into `max(radialSegments, 4·lobes)` sides (`surface/build.rs:14`, checked), 12 by default (`surface.rs:206`, checked), whatever its radius.

### R2 against the pinned baseline (BASELINE.md, `8e0141dc`)

R2's growth is read as the engine plus its conversion: everything stage 2 would do. Today's skeleton is the stage's warm median.

| Preset, seed | Today's skeleton | 1.5x bar | Engine growth + conversion | Ratio | Fine wood: engine / today | Full build: engine / today |
|---|--:|--:|--:|--:|--:|--:|
| beech 1 | 57.3 ms | 86 ms | 4,385 ms | **77x** | 23.8 / 46.7 km (0.51) | 6.1 / 3.5 s (**1.8x**) |
| beech 7 | 57.8 ms | 87 ms | 3,268 ms | **57x** | 21.3 / 47.2 km (0.45) | 4.7 / 3.5 s (**1.4x**) |
| oak 1 | 29.8 ms | 45 ms | 10,914 ms | **366x** | 12.9 / 31.2 km (0.41) | 11.9 / 0.36 s (**33x**) |
| oak 7 | 31.7 ms | 48 ms | 12,955 ms | **409x** | 13.6 / 35.6 km (0.38) | 14.0 / 0.42 s (**33x**) |
| spruce 1 | 33.9 ms | 51 ms | 12,033 ms | **355x** | 48.0 / 18.8 km (2.55) | 25.3 / 4.8 s (**5.2x**) |
| spruce 7 | 32.1 ms | 48 ms | 12,388 ms | **386x** | 51.0 / 17.9 km (2.85) | 26.3 / 4.6 s (**5.7x**) |
| palm 1 | 1.6 ms | 2.4 ms | 0.6 ms | **0.4x, met** | 0 / 0 | 4.8 / 4 ms (1.2x) |
| palm 7 | 1.7 ms | 2.6 ms | 0.7 ms | **0.4x, met** | 0 / 0 | 5.2 / 4 ms (1.3x) |

**Reading it:**
- **The palm meets R2 and roughly meets "full build no slower than today".** Its 4 to 5 ms is within the timer's noise of today's 4 ms.
- **The beech's full build is 1.4 to 1.8x today's.** It is the only tree near R2's second clause.
- **The spruce is 5x.** Its CPU leaf placement (16M needles, about 12 s) is most of it. The GPU executor places leaves in production, but its time on an engine tree is **unknown**: not measured, since `executor::expand` feeds the CPU mesh in the runner.
- **The oak is 33x.** Today's oak is small (134k nodes, 0.8M leaves), and the engine's oak pays light.
- **Fine wood:** today's beech and oak carry twice the engine's, because today's twig layer is counted as fine wood. The owner's gap for the oak (a denser web) points the same way.
- **Per kept node, the engine's growth is 2.0 µs (beech), 6.7 to 7.1 µs (oak, with light) and 2.6 µs (spruce).** Today's skeleton is 0.27 µs a node. Even a perfect lever set that matched today's cost per node leaves the bar 5 to 25x away, because the engine keeps 10 to 45x the nodes.

**Decision needed (host):** what R2 compares. Candidates, each measurable today:
- (i) Growth time per kept node or per km of fine wood against today's.
- (ii) Full build no slower than today's at equal or greater fine wood. The beech is near it and the palm meets it.
- (iii) An absolute budget per mature tree on the named machine, from the frame and build metrics in STRATEGY.md.

As written, R2 trips the wrong-path stop for the beech, oak and spruce whatever is optimised.

## 2. The levers

Gains are estimates from the table above unless marked measured. "Exact" means the kept tree is byte-identical, held by the structure hash. That proof is all a lever needs for continuity: a tree that does not change cannot pop.

### (a) Shedding decided before growth

**The rule today** (`shed.rs:12-36`, checked):
- A lateral or relay goes, with all it bears, when its subtree has held no living apex for more than its PA's `shedding` delay at the tree's age.
- An apex lives at most its PA's `lifespan` units, and then its `next` PA's (`grow.rs:338-365`, checked).
- A relay carries the units already spent (`grow.rs:373-376`, checked), so it adds no lifetime.

**So a subtree's last living cycle has an upper bound at its bud's birth, from the PA table alone:**
- The bound is `maxlife(p) = lifespan(p) + max(maxlife(next), maxlife(q) for every lateral PA q)`.
- It is infinite where:
  - a PA bears its own PA (a self-loop);
  - a zone holds sleeping buds, which wake by a yearly hazard up to the tree's age (`dormant.rs`).
- Viability and abortion only shorten life, so the bound holds under every draw.
- A bud of PA p born at cycle b is certain to be shed when `age - (b + maxlife(p)) > shedding(p)`.

**Measured (`raw/probe.log`, the probe in `cost-probe.patch`):** the rule never marked a kept axis (asserted in the probe).

| Tree | Shed phytomers | Certain at birth | Share of shed | Share of grown | Grown after the skip |
|---|--:|--:|--:|--:|--:|
| beech 1 | 7.51M | 7.13M | 94.9% | 74.7% | 2.42M |
| beech 7 | 5.35M | 5.09M | 95.1% | 72.3% | 1.95M |
| oak 1 | 5.43M | 5.06M | 93.1% | 71.8% | 1.99M |
| oak 7 | 6.57M | 6.02M | 91.6% | 71.9% | 2.35M |
| spruce 1 | 13.52M | 13.05M | 96.6% | 72.7% | 4.89M |
| spruce 7 | 13.85M | 13.38M | 96.6% | 72.4% | 5.10M |

**Where the certain subtrees are.**
- Almost all are in the leaf-bearing shoots of the last order, those with no laterals and lifespans of 2 to 3 years:
  - the beech's PA 8, 6.19M of its 7.13M;
  - the oak's PA 9, 4.06M of its 5.06M;
  - the spruce's PA 8, 12.64M of its 13.05M.
- The beech's PA 6 bears itself, so its subtrees have no finite bound. A bound that counts a self-loop's expected depth would find more, but it is no longer certain.

**Estimated gain:**
- Growth falls with grown phytomers, to 25 to 28% of today's. Settle walks every grown axis (`assign`, `fades`, `standing`), so it falls about by half.
- Final lay and conversion are unchanged.

| Species | Engine total today | With the skip |
|---|--:|--:|
| Beech | 3.1 to 4.1 s | about 1.2 to 1.6 s |
| Spruce | 11.4 to 11.8 s | about 5.5 to 6 s (the final lay, 3 s, now dominates) |

- Memory falls about in proportion to grown axes.
- **The phytomer budget** (20M, counting every grown phytomer including the shed ones; `grow/unit.rs:47-50`, checked) would then count living wood. That gives about 3.5x headroom.
  - That budget is what stopped the oak's finer web: twigs bearing twigs refused 20M at seed 1 (fn-195 RESULT.md:587).
  - It also stopped the spruce's longer-lived spurs (fn-194 RESULT.md:282).

**Exactness and the interactions the host asked about:**
- **Beech, spruce, palm: exact, expected; to be held by hash.**
  - Their values read no light, balance or retained girth: `leaf_area`, `upkeep` and `retained` are 0 (checked, lines above).
  - Draws are keyed to the lineage, not a stream (`lineage.rs:1-5`, `grow/unit.rs:78`, checked), so a skipped subtree moves no other draw.
  - Presence windows come from the closed form, never from grown wood (`presence.rs:1-8`, checked).
  - One place could differ: `fades` (`shed.rs:76-137`, checked) carries a shed descendant's presence-weighted living time up to its kept ancestors. Whether a certain-shed descendant can ever set a kept ancestor's fade is **unknown**. The hash test settles it, and if it can, the skip records that one number per subtree.
- **Oak: not exact.** A certain-shed subtree is alive for its 2 to 7 years, and in those years:
  - its leaves shade the lattice (`grow/relay.rs:77-84`, checked: a cycle's leaves are its units' phytomers);
  - it takes a share of its bearer's vigour under apical control (`allocation.rs`);
  - it enters its bearer's remembered carbon balance (`grow/balance.rs`);
  - at retained 0.5, its pipe stays in the bearer's girth (`girth::attachments`, `grow.rs:154-160`, checked).

  Skipping it changes the light every later bud reads. Step 4's balance shedding is decided on that light, so it cannot be predicted, but it only shortens life, so it never breaks the bound. Two options:
  - **(a1) Ephemeral subtrees, exact in intent.** Grow certain-shed subtrees as today, for light, vigour and balance. Release them from the axis arrays once they are dead and past their delay, recording only their retained pipe at the bearing node.
    - It saves memory, the settle pass and every later walk over all axes (`standing` in each re-lay).
    - It saves no growth, rough layout or sweep: an estimated 1 to 1.5 s an oak.
    - Whether the retained pipe can be recorded exactly at release is **unknown**: `attachments` reads final presences that `assign` computes after growth.
  - **(a2) A closed-form stand-in, not exact.** A skipped subtree deposits its expected leaf area, takes its expected vigour and leaves its expected pipe, all from the closed form (`closed_form.rs`), which already gives each PA's expected growth.
    - It changes the oak by degree in every setting, since the stand-in is a smooth function of the settings, as presence is.
    - It needs a look check against round 10, and it is a model change: a design call for the host.

**Interaction with fn-206.**
- fn-206 replaces `next` with a continuation share and makes lifespans fractional.
- It also sheds "once its time-weighted fade reaches nothing, not on whole cycles" (fn-206 RESULT.md, palm worktree, read-only).
- The bound then runs over the shared chain with fractional lifespans, and certainty means a fade of exactly 0.
- The probe measured master's model, so its shares must be re-measured on fn-206's.

**Verified by:**
- The structure hash at 80 years, seeds 1 and 7, before and after, for the beech, spruce and palm.
- A test that grows with the skip and without it and compares every kept field.
- The walk tests unchanged.
- For (a2) only, the oak's sheet beside round 10.

### (b) Substructure instancing (GreenLab's factorisation)

- **What it needs.** GreenLab grows a substructure once per (PA, age) and reuses it. That works when every substructure of the same PA and age is the same, as in deterministic mode, or the same in distribution, with stochastic factorisation over a few drawn variants.
- **What breaks it here:**
  - **Lineage-keyed draws.** Every lateral's key is its parent's zone key, node and slot (`grow/unit.rs:78`, checked), so no two subtrees draw alike. Reuse needs a finite pool: key a subtree's draws by (PA, birth cycle, a pool index hashed from its lineage). Adding a branch still moves no other draw, and a walk of any setting still moves every instance alike, by degree. But the tree changes from today's individuals, and the owner's eye would judge visible repetition. fn-206 keys growth units by cycle from the lineage (`Key::onto`), which makes the pool key simpler.
  - **Light.** Under shade every subtree's vigour, survival and size differ. Instances are not identical for the oak, and for any species that reads light.
  - **Positions.** Tropism pulls towards a world elevation, sag bends under load, and erection and straightening act in world frame (`geometry.rs`, `sag.rs`). So an instance's geometry is not a rigid copy. The final lay and the conversion still walk every phytomer.
- **Estimated gain.** After (a), growth is about a quarter of today's and the final lay and settle dominate. Instancing the growth of light-free species would save a further 10 to 20% of the engine's time, nothing for the oak, and nothing downstream.
- **Recommendation:** do not instance growth. The repetition it exploits is cheaper to take in drawing (d3), where an instanced spray is invisible as a copy.

### (c) Light's cost

Light is paid only by the oak in production.

**(c1) Skip the last cycle's rough layout, re-lay and sweep. Exact, measured.**
- The cycle loop (`grow.rs:127-142`, checked) lays, re-lays every tenth cycle and sweeps light at every cycle, the last included. Nothing reads them:
  - a bud's light is read in the next cycle's `advance` (`grow.rs:298`);
  - the final lay rewrites every position (`grow/sketch.rs:10-11`).
- A probe that skips them at `cycle == age` gives the oak the same structure hash at seeds 1 and 7 (`7f879919…`, `b00fc549…`; `raw/skiplast.log`).
- It saves the cycle-80 re-lay (1,243 / 1,435 ms) plus a rough layout and a sweep: about **11% of the oak's engine time**.

**(c2) Gate the rough layout on a species that reads light. Exact, checked.**
- The layout runs whenever the site's light shades (`grow.rs:123`, checked), whatever the species' values.
- A beech or spruce under a shading site light pays 1.3 to 6 s for light no bud reads (fn-197 STEPS-1-2.md:60-84; `raw/measures.log` gives the spruce 39 to 75 s under load).
- fn-197's `tests/light.rs` proves those trees bit-identical to neutral light (STEPS-1-2.md:47-58).
- This matters once the pipeline takes the light from a scene, which every species in it shares.

**(c3) A sparser re-lay schedule late in life. Changes the tree.**
- The re-lays grow as the tree does: the cycle-70 re-lay alone is 0.8 to 1.1 s.
- `RELAY_EVERY` (`grow/relay.rs:20`, checked) is an engine constant, not a setting, so no walk crosses a schedule change.
- fn-197 measured that a re-lay every 5 cycles is no closer than every 10 (STEP3B.md:66).
- A schedule of 10, 20, 30, 45, 60 and 80, with the 80 dropped by (c1), would save an estimated 30 to 40% of the re-lays.
- Verified by the oak's sheet beside round 10, and by fn-197's gap measure (`measures gap`).

**(c4) The sweep and the lattice: no lever worth taking.** The sweep is 0.5 s; the lattice's 0.5 m cell is a site constant (`light.rs:19`).

**(c5) The rough layout itself: no lever in (a).**
- It costs about 0.28 µs a grown phytomer (fn-197) and reads every unit grown that cycle.
- Under (a1) it stays. Under (a2) it falls with the skipped subtrees, about 70%, roughly 1 s an oak.

**Estimated gain:**

| Levers | Oak engine time (10.8 to 12.8 s today) |
|---|--:|
| c1 alone | about 9.5 to 11.3 s |
| c1 + c3 + a1 | about 7.5 to 9 s |
| c1 + c3 + a2 | about 4.5 to 5.5 s |

### (d) Cheap fine wood in the pipeline: the lever for the oak's web and the spruce's curtains

**Where the cost is.**
- Fine wood is 92% (oak) to 98% (spruce) of wood length, and so nearly all triangles.
- The spruce's 198M to 207M triangles are about 5.6 GB of GPU wood buffers on a 10 GB card.
- Its 15.5M to 16.5M needles are about 1.1 GB more.
- More curtain volume or more web adds to both, and fn-194 hit GPU out-of-memory at sleeping probability 0.5 (RESULT.md:471).

**(d1) Sides that fall with the radius.**
- A ring's side count falls with its radius, from `radialSegments` at the trunk to 3 or 4 on the finest wood.
- It is a count that steps by one side at a time as a radius passes a threshold that is a smooth function of the rows.
- **Estimated gain:** fine-wood triangles to a third or a quarter; the spruce to about 55 to 70M triangles, and the CPU wood time in proportion.
- **Continuity:** the side count is a drawing resolution, not a tree parameter. A walk changes it only where a radius crosses a threshold, by one side on wood a millimetre across. The step is sub-pixel at the supported views; that is a claim for the host's stills.
- **Verified by:** triangle counts, the CPU wood time, and stills of the trunk base, limb and spray close-ups beside today's.
- **Cost:** the GPU executor's ring kernels assume one segment count per tree (`compact.rs:95`, `prepared.rs:74`, checked: `segments` is one field), so they change too.

**(d2) Ring spacing on fine runs.**
- The engine lays a node every 1.1 cm (48.9 km over 4.43M kept phytomers on the spruce), and the sweep puts one ring on each (`surface/build.rs:16-20`, checked).
- A straight fine run needs a ring only where it bends by more than a tolerance.
- **Estimated gain:** fine rings, and so triangles, to a half or a third again.
- **Continuity:** the tolerance is a drawing value. Dropping a ring that a bend at the tolerance would keep is a step, so the error must be held below a pixel at the supported views.
- **Verified by:** as for (d1), and by the rings' measured deviation from the swept path.

**(d3) Instanced shoot elements (sprays or cards).**
- The last leaf-bearing order is drawn as one instance of a shoot element, its wood and leaves, at its base and sized by its presence:
  - the oak's PA 9;
  - the beech's PA 8;
  - the spruce's PA 8, the needle shoots on the comb branchlets.
- Today this order is about half the kept tree, as swept wood plus leaves:

  | Species | Kept phytomers of this order |
  |---|--:|
  | Spruce | 2.0M of 4.4M |
  | Beech | 1.07M of 2.04M |
  | Oak | 0.81M of 1.61M |

- The leaf element machinery already instances leaves this way (`Instances`, `foliage.rs:110`, checked).
- **Estimated gain:**
  - kept nodes, conversion, wood triangles and placements all fall by about half;
  - a spray costs one instance instead of 40 to 100 triangles a node plus its leaves.
- **What it gives the owner's gaps:**
  - The oak's web can carry a further order of division as element detail at no growth cost.
  - The spruce's curtains can hang denser comb sprays.
- **The design question for the host.** The element must be the engine's own shoot, its node count, length and presence drawn by the engine, or it is a second grower and the twig layer again, which R1 removes.
- **Verified by:**
  - kept nodes, triangles and instance counts;
  - the spray and limb close-ups beside the photographs;
  - a walk over the shoot PA's settings, which changes the elements by degree.

**(d4) Cards at a distance.**
- A level of detail is the rendering track's work ("Surface and rendering at scale"), with its own error measure.
- It is not needed for F's gate, but it is the frame metric's lever.

### (e) Other levers found

- **(e1) A dormant cull costs nothing. Exact.**
  - At `shellDepth` 1 the cull keeps every leaf: `radius_at(y) - r <= shell` holds for every r ≥ 0 when the shell is the envelope's widest radius (`foliage.rs:265, 297-300`, checked).
  - It still transforms every leaf. Today it costs 464 to 821 ms on today's beech and spruce (BASELINE.md), and it runs on 16M needles on the engine's spruce.
  - Skipping the walk when the shell is at least the widest radius returns the same leaves.
- **(e2) Parallel growth and lay.**
  - Draws are order-independent (lineage keys), so the living apexes of a cycle can grow on threads and be merged in a fixed order. Subtrees can be laid in parallel.
  - The engine is single-threaded today (`telperion-space/src` has no threads, checked by search). The pipeline already runs wood and leaves concurrently (`pipeline/drawn.rs`).
  - **Estimated gain:** 3 to 8x on growth and lay on the 16-core machine. Nothing in wasm, which has no threads.
  - **Exactness:** byte-identical if the merge order is the sequential one; unproven.
- **(e3) A compact phytomer.**
  - A `Phytomer` is 10 fields, three `Vec3` of f64 and six f64 beside a u32: about 128 B (`structure.rs:131-160`, checked).
  - Heading and side are derivable at lay time, and f32 serves positions inside a tree.
  - **Estimated gain:** 40 to 60% of the engine's memory. Not exact; the hash changes by rounding.
- **(e4) One record a leaf-only shoot.**
  - An axis of a PA with no laterals and no sleeping buds grows nothing but its own nodes. It is the same shoot that (d3) draws.
  - Stored as one record (lineage, birth, PA, node count, presence) and expanded only when laid, it halves the kept phytomers in the engine too, as listed under (d3).
  - (d3) and (e4) are one design.
- **(e5) Ages the oak does not use are not grown at all.** No gain here: the passed species' unused ages are pass-through after fn-206 (`Species::canonical`).

### What the levers reach

All estimates, single thread:

| Species | Today's engine + conversion | (a) or (a1), (c1), (c3), (e3) | Plus (e2) at 8 threads | Plus (e4) |
|---|--:|--:|--:|--:|
| Beech | 3.3 to 4.4 s | 1.4 to 1.8 s | 0.2 to 0.3 s | about 0.1 to 0.2 s |
| Oak (a1) | 10.9 to 13.0 s | 7.5 to 9 s | 1 to 1.3 s | about 0.6 to 0.8 s |
| Oak (a2) | 10.9 to 13.0 s | 4.5 to 5.5 s | 0.6 to 0.8 s | about 0.4 to 0.5 s |
| Spruce | 12.0 to 12.4 s | 6 to 6.5 s | 0.8 to 1 s | about 0.5 s |

Against bars of 45 to 87 ms, the best column is still 2 to 10x over. Hence the decision in section 1.

## 3. Integration into stage 2

### Stage 2 today

- `pipeline::skeleton` (`pipeline.rs:163-171`, checked) calls `branching::crowned` (`branching.rs:360-365`), which grows `Specimen`: the scaffold, the twig layer (`branching/local/*`), then `finish` (`branching.rs:371`, checked).
  - `finish` sheds by the shell (`:378-379`), solves radii, holds girth (`radius/hold.rs:13`), and caps the radius of childless structural tips (`:403`).
- Expansion opens with `clothe` (`pipeline.rs:176`) and is otherwise the same for any tree (`executor::expand`, `executor.rs:43`, checked).
- **The engine's path is stage 2 replaced:**
  - the engine grows the structure;
  - the conversion (today `examples/space/tree.rs::convert`, an example, checked) makes the pipeline `Tree`;
  - plan, outputs and the GPU executor follow unchanged (`generation/preparation.rs:14-23`, checked: it calls `executor::grow` and then `expansion`).

### What replaces each removed piece for the passed presets (R1)

| Today | File (checked) | For the engine's presets |
|---|---|---|
| The twig layer | `branching/local/{seed,advance,…}.rs`, rows `/skeleton/twigs/*` | Not run. The engine grows to the leaf-bearing shoot. Two of its rows stay read by the leaf stations, `twig.internodeLength` and `stationsPerInternode` (`input.rs:120-125`, `foliage/canopy.rs:17`). The station contract replaces them (below). |
| Its thickness gate | `local/seed.rs:133` and `local/advance.rs:52` (`limbRadius` × root radius); `advance.rs:48` (`bearingDiameter`) | Gone with the layer. Which wood branches is the engine's zones, by PA. |
| The tip shoot | `local/seed.rs:132` (a childless structural node always gets a terminal bud, past the gate) | Gone. An apex grows or stops by the engine's lifespan, abortion and relay. |
| Girth patches | `radius.rs:65,77` (`lateralShare`, `forkBalance`, read in the solve), `radius.rs:91,112` and `radius/hold.rs:13` (`girthHold`, `girthFall`), the tip cap `branching.rs:403` | Not run. The engine's girth is the pipe model per PA (`form.pipe`, `exponent`, `ripening`, `secondary`, `retained`, `leaf_girth`; `girth.rs`). `Tree::pipe` stays empty, and leaf decisions read the radius. Whether every reader of `pipe` takes an empty one is **unknown**. |
| The foliage cull | `foliage.rs:255` (`shellDepth`, the preset's envelope) | `shellDepth` 1, where the cull is dormant and, with (e1), free. The beech, spruce and oak presets are at 1 already; the palm is at 0.85 and would move to 1. The envelope is a preset shape the engine's tree never reads. Whether leaf placement also reads the envelope (it travels in `LeafInput`) is **unknown**. |
| `sheddingThreshold` | `branching.rs:280,378` | Not run; the engine sheds by its own rule. |

**The conversion's own constants are unnamed requirements** (`docs/principles.md`, step 1). In `tree.rs:13-17` (checked):
- `STRUCTURAL` 0.05 of the root radius sets node kinds.
- `FINEST` 0.2 mm drops thinner phytomers. It is a hard cut, a switch on a growing-in phytomer, though sub-millimetre.
- `SHORTEST` 1 mm merges internodes.
- `FORK_FROM` 0.4 and `FORK_AT` 0.7 set codominance.

Each needs a name or deletion when the conversion moves into core. `FINEST` should become a fade by scale.

### The station contract: leaves and organs from the engine's phytomers

**Today:**
- Leaves bear on `Twig`-kind nodes, and on any wood thinner than `shootRadius` × `stem_radius` (`foliage/plan.rs:270-281`, checked).
- `stem_radius` is measured where the stem first parts (`tree.rs:151`, checked).
- **The shootRadius issue** (fn-195 FRICTION.md):
  - An engine value that moved the oak's first stem fork up a thin central limb cut its leaves from 2.4M to 0.5M at seed 1, and differently per seed.
  - The rows were patched to `shootRadius` 0.06 (beech 0.018, spruce the preset's 0.025).
- Kinds come from the conversion's `STRUCTURAL` rule, not from botany.

**Proposed:**
- **The engine says which nodes bear leaves, how many and for how long.**
  - A per-PA leaf row: the years a node keeps its leaves. 1 for a deciduous tree's current shoots; about 5 for spruce needles (fn-194 RESULT.md:360-462); a frond's life for the palm.
  - Each node's leaf presence: its phytomers' scale times the fade of its leaf years.
- **The pipeline `Tree` carries it per node,** beside `pipe` and `sections`: a station weight and the node's cycle.
- **Placement reads it.** `shootRadius`, the `Twig`-kind rule and `stem_radius` go dormant for the engine's presets.
- **Continuity:** a station's leaves grow in by the phytomer's presence, so a setting that crosses a draw grows its leaves in from nothing.
- **The palm, fn-196's gap (host decision 1):**
  - Today `rosettes` hangs `rosetteFronds` (42) at every stem apex (`rosette.rs:108-143`, checked), and `leaf_bases` spreads `leafBases` (256) evenly from the crown to the foot (`branching/leaf_bases.rs:87`, checked). Both are preset counts at every age.
  - Under the contract, each stem phytomer is one leaf (12 a year). Its fronds are the phytomers within the frond's life below the apex, and its leaf bases are the ones below them within the bases' retention.
  - Count follows the stem.
  - Size: a frond follows its phytomer's scale times an establishment law of frond length on stem age or girth. That law needs a source: **unknown** which; fn-196's SOURCES.md has none for it.
- **Cost:**
  - The GPU executor packs stations per run from `TwigPlacement`'s spacing (`generation/preparation.rs:23-127`, `foliage/prepared.rs:41-129`, checked).
  - Per-node stations change that packing once. After that no species needs renderer code (STRATEGY.md, the renderer contract).
  - How large the GPU change is: **unknown**.

### How presets choose the engine

**The principle at stake.**
- A family that grows by the engine and one that grows by `Specimen` are two ways of building.
- `docs/principles.md` allows one sanctioned exception, the GPU executor, and "no parameter is a switch between ways of building".
- The spec says "today's generator keeps the rest", so during the migration the switch exists whatever form it takes.

**Options:**
- **(i) A named, temporary sanctioned exception.**
  - The preset declares its grower by an explicit field outside the tree space: not a row, not walkable, not blendable.
  - docs/principles.md lists it beside the GPU executor, with its removal condition: the last preset onboarded to the engine.
  - **Recommended.** It is the honest form of what the spec asks.
- **(ii) The engine's table present or absent decides.** This is a switch disguised as data, refused by the principle.
- **(iii) Onboard every preset first.** That needs four more species specs (the birch, Telperion, Laurelin, ordinary) before F, which the spec's boundaries exclude.

This amends an owner principle, so it is an owner decision, through the host.

**The values.**
- The engine's species becomes value rows on fn-206's shared chain: one fixed table of N ages (15 in fn-206) that every engine preset carries, with unused ages pass-through.
- The species' Rust constructors (`beech.rs`, `oak.rs`, ...) become `.values` files, as presets are value tables with no species branch (AGENTS.md).
- The site's light comes from the scene or request, never the preset (fn-195 RESULT.md:337).
- **Open (unknown):**
  - Whether the CI size budget absorbs 15 ages × about 40 settings × zones per preset in the wasm presets table.
  - Whether the engine's float maths may stay on `std`. Core pins libm for its own (`Cargo.toml`, `libm = "=0.2.16"`), and telperion-space calls `.exp()`, `.ln()` and `.powf()` directly.

**Where the code lives.**
- `docs/pipeline.md` makes stages private to the pipeline so a second chain does not compile.
- `telperion_space::grow` is public today and called by examples outside it.
- F should move the engine into a private stage module of `telperion_core::pipeline`, or make it a dependency called only by the pipeline, with a compile-fail boundary test like `crates/telperion-wasm/tests/boundary.rs`.
- telperion-core does not depend on telperion-space today (`telperion-core/Cargo.toml:18-22`, checked). telperion-space has no dependencies and no threads, so it should build for wasm32 (unverified).
- **The beech is `IN_WORK`, not `CATALOGUE`** (`presets.rs:24,42-43`, checked). `every_shipped_preset_builds_every_artifact_through_the_pipeline` (`pipeline/tests.rs:189`) skips it. Whether F ships it is an owner call.

## 4. Dependencies

### fn-206 (one reference axis; running in `.worktrees/palm`)

Read-only, from its RESULT.md. fn-206:
- puts all four species on a 15-age chain (`chain.rs`);
- replaces `next` with a continuation share and keys continuations by age (`Key::onto`);
- keys growth units by the cycle they grow in, not by their count;
- makes lifespans fractional;
- sheds by a time-weighted fade reaching 0;
- adds `Species::canonical` and `blend`.

"Unchanged in look" there means the same species law, not the same individual (its host decision 6).

**What in F depends on it:**
- **The preset rows (section 3).** The chain is the table every engine preset carries, and designing rows before it lands is work redone.
- **Lever (a)'s bound.** `maxlife` over the chain with a continuation share and fractional lifespans, and certainty as a fade of exactly 0, are fn-206's model. The 72 to 75% shares above must be re-measured on it.
- **Every exactness baseline.** fn-206 changes each individual, so F's before-and-after hashes are taken on fn-206's head.
- **Lever (b)'s keys** (cycle-keyed units).
- **Everything that edits `grow.rs`, `shed.rs` or `geometry.rs`:** (a), (c1) to (c3), (e2) to (e4). fn-206 rewrites these files, so building them first means conflicts.

**What does not depend on it** (core only):
- (d1), (d2) and (e1);
- the measurement tooling (FRICTION.md's `cost` mode).
- (d3)'s element is core-side, but its shoot is the engine's PA, so its data path waits for the station contract.

### fn-199 (relays change the tree by degree)

- F's spec depends on it (`depends_on_epics`).
- fn-206 checked fn-199's four findings against its code (RESULT.md, "fn-199's four relay-continuity findings"):
  - **R2 is resolved** by `b172c2d4` (`a_node_growing_in_moves_no_relay`).
  - **R1** (relay 1 crossing survival), **R3** (a relay barely off a bent lateral) and **R4** (relay girth on a discrete node) **stay open**.
- fn-206 does not cover fn-199. fn-199 should be built on fn-206's model, since it edits the same files.
- Its R6, the beech re-rendered, must come before F takes its exactness baselines.

## 5. Recommendation and order

1. **Host decisions, before any build:**
   - **R2's bar** (section 1): (i), (ii) or (iii).
   - **The grower switch:** the temporary sanctioned exception, option (i). An owner principle change.
   - **The oak's (a):** (a1) exact, or (a2) the closed-form stand-in.
   - **Whether (d3)/(e4), the last order as a shoot element, is the design** for the oak's web and the spruce's curtains.
2. **Now, independent of fn-206** (core only):
   - (e1) the free dormant cull;
   - (d1) sides by radius and (d2) fine ring spacing, with the GPU executor's ring kernels;
   - a `cost` mode on `measures` (FRICTION.md).
   - **Verified by:** triangle counts, GPU wood buffer sizes, CPU wood time, and close-up stills beside today's.
   - **First red test:** the spruce at 80 years under 8 GB of wood buffers, failing today at about 5.6 GB with the more-volume values of fn-194 round 9.
3. **After fn-206 merges:**
   - the exact engine levers: (c1) and (c2), byte-identical, which (c1) already showed on the oak;
   - (a) for light-free species, byte-identical, held by hash;
   - the phytomer budget counting living wood;
   - then (a1) or (a2) for the oak, and (c3), each beside round 10's sheet.
4. **After fn-199 lands on fn-206's model:**
   - the engine and the conversion into a private stage-2 module;
   - the preset rows;
   - the removals of section 3 for the four presets;
   - the station contract (leaves and palm organs from phytomers);
   - the boundary test and the every-preset test with the engine's presets.
5. **Then the owner's two gaps,** inside the freed budgets: the oak's finer web and the spruce's curtain volume, by values and (d3). Each judged as C.
6. **Re-measure R2** as the host restates it. If the bar is still missed, (e2) parallel growth and lay comes next, before the wrong-path stop is declared.
7. **The gate:** workspace gate, `npm test`, Codex review, size budgets, and the PR with every species' sheet (R3, R4).

## Files

- This file; `FRICTION.md` (three entries, written as they happened).
- `cost-probe.rs`: the scratch cost example (runs as `crates/telperion-render/examples/f_cost.rs`, `f_cost <species> 80 <seed>`).
- `cost-probe.patch`: the temporary grown-count, certain-shed and skip-last instrumentation of `grow.rs`. `F_GROWN=1` prints counts, `F_SKIPLAST=1` skips the last cycle's light.
- `raw/` (ignored): every log named above, and the run scripts.
