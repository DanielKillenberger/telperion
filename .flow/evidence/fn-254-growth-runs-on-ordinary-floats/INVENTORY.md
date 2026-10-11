# fn-254 R1: what contract A carries today

Read against the code at 59e627e7 (`fn-254-growth-runs-on-ordinary-floats`, cut from F4), before any change. Paths in `src/` are `crates/telperion-space/src/`; paths in `fine/` are `crates/telperion-render/src/generation/fine/`. Line numbers are those of 59e627e7.

## 1. Who grows where (R4a)

The executor is chosen in `crates/telperion-core/src/pipeline/executor.rs:73-82` (`executor::apart`), which calls `telperion_space::apart` (`src/grow.rs:381-390`).

| Preset | Light rows (`shadeHazard`, `shadeSize`, `balanceHazard`, `leafGirth`, `retained`) | Fine roots grow on |
|---|---|---|
| european-beech | none | device |
| laurelin | none | device (the beech's structure, scaled) |
| telperion | none | device |
| norway-spruce | none | device |
| silver-birch | none | device |
| oregon-white-oak | 40 rows | CPU: `reads_light()` (`src/species.rs:423-430`) |
| ordinary | 75 rows | CPU: reads light and `keeps()` |
| date-palm | none | CPU: no regular twig stations (`executor.rs:77-79`) |

`fine/route_tests.rs:22-34` pins the spruce and the beech to the device and the oak and the palm to the CPU. The other three device rows are read from the values files and the routing code; R5 records the route each build took.

The lit stepped executor's device version (`fine/feedback_stepped.rs`, `feedback_lay.wgsl`, `feedback_stepped.wgsl`, `feedback_slice.*`, `feedback_probe.wgsl`, `feedback_quotient.rs`) is test-only (`fine.rs:201-212`). No production build runs it.

## 2. The formats

From `src/fixed.rs:1-21` and `.flow/evidence/fn-211-tree-space-f2b-growth-years-cheap/CONTRACT-A.md` §2.2:

| Format | Storage | Used for |
|---|---|---|
| ticks | u64 count of 2^-20 cycle, or an f64 on that grid | every time |
| Q62 | u64, `n / 2^62` | probabilities: survival, abortion, go, the D16 hazard |
| Q32 | i64 or u64, `n / 2^32` | logs (saturated at `INF` = 2^62), presences, light, vigour, wood, positions, lengths, unit vectors, scales |
| W | i128, `n / 2^64` | sections, masses, moments, lattice deposits, tree-wide sums (`Total`) |
| T / A | u32 wrapped turns / i64 Q32 turns | orientations / angular amounts |
| I256 | `fixed/feedback/wide.rs` | exact intermediates of W products and quotients |

Every product is built from 32-bit halves on wasm32 (`src/fixed.rs:55-93`), and from u32 pairs on the device (`fine/wide.wgsl`, `fine/feedback_wide.wgsl`). The f64 fields of the tree hold exact images of these integers (`src/geometry/q.rs:1-6`, `src/presence.rs:306-309`).

## 3. Biological quantities on contract A

### 3.1 Decisions (the chain grower; CPU `src/fixed/decide.rs`, `src/fixed/presence.rs`; device `fine/contract.wgsl` + `apex`, `assign`, `carry`, `sleep`, `zones`, `chain`, `shed.wgsl`)

| Quantity | Today | CPU | Device mirror | Feeds |
|---|---|---|---|---|
| Survival over a share of a cycle (D5) | Q62 `p`, `-log2` Q32, share in 2^-32 cycle, `threshold` | `decide.rs:20-36`, from `grow/advance.rs:83-98` | `survival_fine_q`, `contract.wgsl:360`, `apex.wgsl:362` | a unit fails where `d >= t` |
| Lived time of a failed unit (D5) | ticks rounded up, plus a 2^-32 rest | `decide.rs:42-90`, `advance.rs:100-101` | `lived_q`, `lived_fine_q`, `contract.wgsl:450,466` | `Unit.share`, `apex.spent`, the relay's `below` |
| Abortion (D6) | Q62 `1 - a`, Q32 rise, ticks, `exp2`, `mul` | `decide.rs:96-128`, `advance.rs:111,182` | `abortion_fine_q`, `contract.wgsl:370` | aborts where `d < t` (`stop.rs:155`) |
| Move-on presence `made` (D10) | Q62 go, `logit`, Q32 woods, `exp`, `presence` | `decide.rs:134-172`, `stop.rs:66` | `moved_q`, `contract.wgsl:384`, `apex.wgsl:249` | the continuation's presence, and whether its relay grows (`stop.rs:77`) |
| Relay, ended and failed relay presences | f64 `lineage::threshold`; presence Q32 | `stop.rs:230-240,269-276`, `advance.rs:91` | `apex.wgsl` | a relay axis and its birth presence |
| Window and expected wood share | f64 closed form, then `q32`, `exp`, a Q32 table interpolated by ticks | `src/presence.rs:44-59,138-147`, `fixed/presence.rs:164-185`, `decide.rs:155-178` | `window_q`, `wood_at_q` (`assign.wgsl:356`); tables `fine/tables.rs:80-88`, `fine/surface.rs:97-124` | every presence |
| Node count and node presences | f64 bounds, Poisson cumulative by `mass *=`, `cumulative +=` (`lineage.rs:307-314`); presences `ln_ratio`, `logit` Q32 | `lineage.rs:274-317`, `decide.rs:176-233`, `unit.rs:35-37` | `node_lead` (`zones.wgsl:69`), `apex.wgsl:40-65` | node count; D18; the laid quota |
| Laid nodes of a cut unit | ticks times a u128 sum of Q32 presences; `t = ceil(left 2^53 / (p of))` | `lineage.rs:191-212`, `schedule.rs:221-235`, `unit.rs:38-40` | `apex.wgsl` | how many nodes are laid |
| Lateral bud PA | running f64 sums compared by `lineage::threshold` | `lineage.rs:217-251`; tables `fine/tables.rs:73-78`, `fine/geometry.rs:69-80` | `drawn_pa`, `bud` (`zones.wgsl`) | whether a bud exists and its PA |
| Place share | f64 through `lineage::threshold`; presence Q32 | `lineage.rs:166-180` | `zones.wgsl` | whether a bud place exists |
| Dormant wake time (D11) | `grid`, `ticks`; Q32 years a nat; `mul(ln_ratio(..))`, ticks | `decide.rs:239-252`, `dormant.rs:76-97` | `wake_q`, `contract.wgsl:542`, `sleep.wgsl:24` | whether the sleeper exists; its wake cycle |
| Lifespan | f64 on the tick grid (`species.rs:456-457`), `WHOLE = 1e-12` (`schedule.rs:43`) | `schedule.rs:68-87` | tables `fine/surface.rs:88` | when `move_on` runs |
| Time state (spent, aged, gone, used, sleep, below) | f64 on the tick grid; `below` in 2^-32 cycle | `advance.rs`, `stop.rs`, `schedule.rs`, `wake.rs:40`, `dormant.rs:162` | ticks in `apex.wgsl` | shares, exposure, the cut |
| Survive and persist presences | Q32 (`of_under`, `of_over`, `times`), stored as f64 | `advance.rs:152-153`, `stop.rs:168`, `presence.rs:316-344` | `lead_q`, `times_q` (`assign.wgsl`); `carry.wgsl` | `assign`, `dormant::carried`, `Woken::kept`, the shed sums |
| Shed fade (D17) | Q32 presences, lived time Q32.32 | `shed.rs:131-226`, `fixed/presence.rs:187-203` | `carry_q`, `fade_q` (`contract.wgsl:516,521`), `fades.wgsl` | an axis is dropped where its fade is none |
| Chain kept (D15) | Q62 `neg_log`, saturating sum, `prob`, `mul62` | `decide.rs:259-282`, `alone/shed.rs:37-127` | none (CPU only) | which axes a chain keeps |
| Relay node (D18) | Q32 `relay_at`, a prefix scan | `decide.rs:293-312`, `presence.rs:255-260` | `relay_node` (`apex.wgsl`) | the relay's node |
| Internode, insertion, tropism spread | `q32(spread)` times a signed Q32 draw, `mul` | `lineage.rs:100-161` | `lay.wgsl`, `walk.wgsl` (f32 already) | geometry |

### 3.2 Feedback and geometry (CPU only, except girth and stand)

| Quantity | Today | Where | Read by a decision? |
|---|---|---|---|
| Phytomer scale (rough layout) | Q32, four-factor `prod_shr` | `grow/draw.rs:391-394`, `grow/stepped/lay.rs:152-166` | yes: wood is D16's upkeep (`grow/balance.rs:71-73`), length moves the leaves that make light |
| Scale, base scale, dominance share | Q32, `mul` | `geometry.rs:94-174`, `girth/disused.rs:43-63` | base scale times light is D16's Q (`grow/balance.rs:76-77`) |
| Positions, tips, headings, sides | Q32 vectors (`geometry/q.rs`) | `geometry/lay.rs:217-362`, `geometry/place.rs` | yes, through light; refusals `Stop::Beyond` (`lay.rs:342-344`) and the trunk below ground (`:345-347`) |
| Angles: azimuth, insertion, elevation, wander, sag | T / A, polynomial `sincos`, table `atan2` | `fixed/feedback/angle.rs:33-211`, `geometry/q.rs:59-73` | through light |
| Bend, erection, falloff, reach | Q32 through Q62 `exp2_neg` | `geometry.rs:129-162`, `geometry/lay.rs:372-379` | through light |
| Relay attach and stand | Q32 | `geometry.rs:264-277`, `geometry/stand.rs:57-198`; device `stand.wgsl:254-325` | no (geometry, girth join, sag load) |
| Wood of a pencil | Q32 | `grow/draw.rs:414` | yes, D16 |
| Leaf area and lattice deposits | Q32 area; W u128 wrapping sums | `light.rs:137-141`, `fixed/feedback/lattice.rs:57-158` | yes, through light |
| Node light | f64 sweep, rounded once to Q32 | `light.rs:178-236`, `fixed/feedback/lattice.rs:37-42,166-176` | yes |
| Bud light | Q32 from a Q62 trilinear sample | `fixed/feedback/lattice.rs:180-191`, `light/field.rs:88-90` | yes: D16 (`grow/balance.rs:76`), D5 shade survival (`grow/advance.rs:69-74`), sizes |
| Subtree light and cost totals | Q32 i64, `checked_add` | `grow/balance.rs:65-99`, `allocation.rs:136-144` | yes, D16 (`fixed/feedback/shed.rs:58-96`) |
| D16 memory, hazard, threshold, kept share | Q32, Q62, u64 | `fixed/feedback/shed.rs:58-133` | it is the decision |
| Vigour and its split | Q32; I256 `quotient` | `allocation.rs:40-133` | through size, scale, wood |
| Size `(v/p)^psi` and group sums | Q32; W `Total` | `allocation.rs:153-215` | through scale, wood |
| Fine-root aggregates (cohorts, living, wood, leaves, ratio) | f64 already | `couple.rs:97-170`, `grow/couple.rs:286-326`, `grow/stepped/stands.rs:157-169` | yes, D16 for fine roots |
| Pipe, own, established, section, radius | Q32 `prod_shr`; W through `exp2_w`, `log2_w` | `girth.rs:52-84,116-154,227-262`; device `girth.wgsl` (`feedback_wide.wgsl`, `feedback.wgsl`, `feedback_sites.wgsl`) | no (output, sag) |
| Leaf mean | Q32 `ratio` of `Total`s | `girth.rs:116-154`, `girth/disused.rs:84-115` | no |
| Sag mass, moment, lever, turn | W, I256; `unit_w`, `log2_w` | `sag.rs:249-281`, `geometry/sagged.rs:18-116`, `sag/turn.rs:57-103` | no (`sag.rs:12-13`); the refusals after it |
| Landing support and ease | Q32 `isqrt` | `geometry/lay.rs:418-424` | no |
| Bounded chain girth, fine coarse radius, landing margin | f64 already | `bounded/girth.rs`, `fine/coarse/expected.rs:150`, `fine/likely.rs:100-116` | no |

The device's lay, walk, settle, dress, sweep, stations, rings and handoff passes are f32 already (`settle.wgsl:1-11`). On the device only the chain grower, `fades`, `girths` and `stand` compute in emulated fixed point.

`src/fixed/feedback.rs:6` says "No tree reads it yet". That is stale: `angle`, `vector`, `wide`, `total`, `power`, `edge`, `lattice` and `shed::walk` are all read by production code.

## 4. Where identity or ordering depends on these values

Identity is the lineage key, a 64-bit hash (`lineage.rs:22-50`, keys in `lineage/keys.rs`; device `contract.wgsl:144-159`, `lineage.wgsl`). No key takes a fractional value. Keys are built from integers: the cycle (`advance.rs:23`, `balance.rs:146`), `ended_relays` (`advance.rs:40,207`), the node index (`unit.rs:43,54,79,102,112,125`), the laid index (`lineage.rs:203`), the PA (`onto`, `lineage.rs:44`), the slot. Arithmetic reaches identity only by deciding which of those integers exist:

1. **The move-on presence decides an axis.** `stop.rs:77` grows a relay only where `made < ONE32`; the relay bumps `ended_relays` (`stop.rs:202`), which keys later draws.
2. **Node counts and the laid quota decide node indices**, and node indices are keys: `lineage.rs:191-212` (a u128 sum of Q32 presences against ticks) and `lineage.rs:292-317` (the Poisson cumulative, an f64 running product and sum).
3. **The wake bucket is an event index:** `dormant.rs:97` files a sleeper under cycle `cycle + 1 + floor(years)`; `grow/wake.rs:20` reads it.
4. **The shed decisions renumber axes:** D17 (`shed.rs:99-117`), D15 (`alone/shed.rs:96`), D16 (`fixed/feedback/shed.rs:68-69`).
5. **Every draw test** (`d < t`, survival `advance.rs:98`, abortion `stop.rs:155`, laid `lineage.rs:206`, D16 `feedback/shed.rs:69`, D15 inverted at `decide.rs:271`) ends an apex or a relay, and so the units, phytomers and keys after it.
6. **Running f64 sums decide a PA:** the lateral bounds (`lineage.rs:219-224`), in their written order.

Ordering:
- No sort is keyed on a fixed-point value. Sorts on hashes or integer indices: `unit.rs:60` (stable, `Reverse(Draw)`), `fine/canonical.rs:55,200,206`, `fine.rs:244,351`, `grow/partition.rs:111`, `allocation.rs:86` (parent, node, index).
- One sort on an arithmetic f64: `fine/likely.rs:115` (`total_cmp` on a margin). It orders work on spare cores, not the tree (`fine/landing.rs:1-5`).
- Two prefix selections over Q32 values act as orderings: `relay_node_q` (`decide.rs:293-312`) and `laid` (`lineage.rs:193-210`).
- `couple.rs:161-168` merges cohorts after `sort_unstable` on (pa, spent); equal keys merge in an unstable order.

Hashes over float bits (digests, not identity): `fine/canonical.rs:113-153` hashes whorl, vigour, blend, alive, rank, sleep, units, radius, scale, size, light, tip, heading and side, so every value moved by this spec changes the digest. Memo keys on f64 bits (`schedule.rs:122-130`, `dormant/living.rs:117,203`) and the log cache on Q62 bits (`fixed/presence.rs:131-160`) are caches whose result does not depend on them.

## 5. What stays exact (R3)

- **Lineage hashing and the key registry:** `lineage.rs:22-50`, `lineage/keys.rs`; on the device `mix` and `child` use `mul64` (`contract.wgsl:144-159`). This is the 64-bit emulation that survives the change; it is called once or twice per draw, not in every product.
- **Draws:** 53 bits of `mix(..) >> 11` (`lineage.rs:48-50`). The comparison is the part that changes (§7).
- **Time:** ticks of 2^-20 cycle as an integer count, or an f64 holding one; integer cycle indices; the wake bucket. Time is accumulated in ticks (`advance.rs:106,127,168,174`; `stop.rs:122,126,201`; `schedule.rs:73-197`; `wake.rs:40`; `dormant.rs:162`), never by adding small floats.
- **Sibling order and indices:** the sorts in §4; the device's u32 prefix scan (`scan.wgsl:1-5`) and queue index (`record.wgsl:27,73`).
- **Atomics on the device are integer only** in every production pipeline: u32 `atomicAdd` counters and `atomicMin`/`atomicMax` on ordered u32 keys (`fold.wgsl:81-186`, `sweep.wgsl:176-253`, `stations.wgsl:142`, `handoff.wgsl:168-171`). The only wide atomics are the test-only lattice's four u32 adds with carries (`feedback_stepped.wgsl:42-55`, `feedback_slice.wgsl:37-43`).
- **Presence windows and fractional births** (`presence.rs:44-59,138-147,283-344`; `unit.rs:117-121`; `stop.rs:71-134,277`; `dormant.rs:36-146,329-373`; `lineage.rs:202-207`; `fine/expansion.rs:541-550`; `shoot.rs:351-360`; `wake.rs:103-176`): their formulas stay; only their arithmetic changes.

## 6. The pinned transcendental wrapper

`src/transcendental.rs` routes every `*_fixed` call to `libm`, with its own `hypot`, "so a tree grown natively and in Wasm" does not "differ in its last bits" (fn-11; fn-198 host decision 10).

- **A consumer needs it:** `harness/parity.test.ts:40-52` requires the native and Wasm node buffers of all eight engine presets to be byte-identical. Without a pinned libm, std's platform transcendentals would break it, contract A or not.
- **It is not contract A's:** about 35 files of `telperion-core` call it (envelope, radius, twigs, branching, colonization, foliage, surface), as do light (`light.rs:91-93,162,169,231`), shade survival (`grow/advance.rs:73`), fine sizes (`grow/couple.rs:326`, `grow/stepped/stands.rs:169`), the closed form and the dormant law.

So R2's condition holds: R1 names a consumer, and the wrapper stays. The float arithmetic adds the f32 functions it needs (`libm`'s `sinf`, `cosf`, `expf`, `exp2f`, `logf`, `log2f`, `powf`, `atan2f`, ...) to the same wrapper, so native and Wasm keep growing the same tree. Basic IEEE operations (+, −, ×, ÷, sqrt) round identically on both; Rust does not fuse them.

## 7. What changes, by area

| Area | Becomes |
|---|---|
| `fixed.rs` decision core (`q62`, `q32`, `log2`, `exp2_neg`, `exp2`, `neg_log2`, `prob`, `ln_ratio`, `exp`, `mul`, `scaled*`, `presence`, `logit*`, the tables, `wide*`, `by_halves`) | f32 functions over the pinned wrapper; the tables and the wide products deleted |
| Draw test | the uniform `u = d · 2^-53` of the 53-bit draw, exact, against the f32 probability `p`: `u < p`, taken exactly as the integer test `d < ceil(p · 2^53)` (`p · 2^53` is exact for an f32 and its ceiling fits 54 bits; on the device two u32 words), with `p ≤ 0` or NaN never and `p ≥ 1` always. A 24-bit f32 uniform would turn every probability under 2^-24 into 2^-24 or none, so the draw keeps its 53 bits. Documented beside `Draw` |
| `fixed/decide.rs`, `fixed/presence.rs` | the same formulas in f32 (logs, presences, windows, leads) |
| Ticks, cycles, wake buckets, keys, scans | unchanged (§5) |
| `fixed/feedback/{wide,total,power,edge,angle,vector,lattice,shed,slice}.rs`, `geometry/q.rs` | f32 vectors and angles in radians; I256, `Total` and the T/A formats deleted; sums in a fixed index order |
| `allocation.rs` | f32 vigour and sizes; `thrice_over` and the I256 quotient deleted |
| `girth.rs`, `sag.rs`, `geometry/sagged.rs`, `sag/turn.rs` | f32 |
| Device: `contract.wgsl`, `wide.wgsl` products, `feedback_wide.wgsl`, `feedback.wgsl`, `feedback_sites.wgsl`; `apex`, `assign`, `carry`, `sleep`, `zones`, `fades`, `girth`, `stand` | f32 mirrors; `mul64` kept for the lineage hash only |
| Test-only device feedback (`feedback_stepped*`, `feedback_lay.wgsl`, `feedback_slice*`, `feedback_probe.wgsl`, `feedback_quotient.rs`, `feedback_tests.rs`) | removed with the integer lattice they prove: their 128-bit atomic deposits cannot become float atomics (R3), and the lit grower's device form is fn-256's |

## 8. Tests that hold contract A today, and what replaces them (R4)

Word-for-word CPU-device tests: `the_gpu_takes_every_decision_the_cpu_does` (`fine/tests.rs:406`), `the_gpu_computes_every_feedback_word_the_cpu_does` (`fine/feedback_tests.rs:227`), `the_gpus_quotient_is_the_cpus_over_four_million_cases` (`fine/feedback_quotient.rs:273`), `every_chain_grows_on_the_gpu_as_on_the_cpu` (`fine/chain_tests.rs:151`), `every_unit_of_the_young_trees_draws_the_cpu_s_nodes` (`fine/units_tests.rs:271`), `every_root_grows_whole_on_the_gpu_as_on_the_cpu` (`fine/realized_tests.rs:166`), `the_device_s_base_radii_are_the_cpu_s_to_the_bit` (`fine/realized_tests.rs:444`), `the_lit_fine_roots_on_the_device_are_the_cpus` (`fine/feedback_stepped_tests.rs:65`), `the_feedback_slice_is_the_cpus_in_any_order_chunking_and_retry` (`fine/feedback_slice.rs:162`).

Pins: the closed form's row hashes (`src/closed_form/boundary/tests.rs:17`), the species suite's digests (`crates/telperion-core/src/suite/species/digests.json`), the oracle signatures (`crates/telperion-space/tests/oracle.rs:19`).

Under R4 they become:
1. **Repeatability within an executor:** repeated builds, fresh contexts, reused buffers, any order and batch size give the same canonical tree (the order and batch checks of `chain_tests.rs:170` and `realized_tests.rs:365` stay exact).
2. **Discrete contracts:** keys, `mix`, draws and the uniform, threshold endpoints, parent links and event order, compared CPU to device exactly where they are integers (`mirror_tests.rs`, the `child`/`draw` selectors).
3. **Invariants:** conserved vigour, normalised PA means, light in [0, 1], finite geometry, non-negative radii, contacts, budgets.
4. **CPU-device agreement by lineage** for the five device presets: position and radius error, unmatched wood weighted by length and radius, crown dimensions, leaf area, major-branch survival, with tolerances from measured float runs. The oak, ordinary and the palm grow on the CPU on either delivery; their agreement and cold `grow` timing are fn-256's.

The pins are re-pinned once R5's stills and timings stand (AGENTS.md "Generator evolution").
