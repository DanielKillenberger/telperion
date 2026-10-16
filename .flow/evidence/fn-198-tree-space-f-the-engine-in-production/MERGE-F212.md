# fn-212 merged into F4 (worker, 2026-10-08)

fn-212's head `34e52ec2` was merged onto F4's `8f451083` as `d52e1a76`.
- **The first pass stopped at proof 3:** on the spruce at 80, seed 1, fn-212's parity probes parted 1,376 runs. The cause predated the merge, in F4's device girth.
- **After the host's decisions:**
  - grown wood counts each internode at its spread (`1b74db15`);
  - the device's girth takes contract A's fixed point (`ee13130f`).
- **Every proof then holds, except the artifact ceilings** (proofs 5 to 8 below; the ceilings are an owner decision).

## How the conflicts were resolved (15 files)

- **One driver.**
  - `Roots` is fn-212's async trait (`Step`). F4's as-grown exchange methods (`grown_partials`, `grown_radii`) stay synchronous on it. They refuse by default, so the device still refuses by name a species that keeps its tree as grown.
  - F4's `settle_grown` is now `async`. Its phases 2 to 4 run through fn-212's `fine/lay.rs`, which now also takes the exchange's `Girth` (before, fn-212 passed `Girth::default()`).
  - The stepped executor drives `settle_grown` with `step::now`, because the CPU's steps are ready at once.
  - fn-212's `settle_with` (sowing, spare cores, landing) feeds `settle_grown`. The reach is a `JoinHandle` natively and `Rows` on wasm (`Reach`).
- **`grow_marked`.** It is built on the stepped executor (`grow_roots`), since `grow_fine` is gone. Marks and bases come from its `Fine`, as fn-212 took them from `grow_fine`'s. A species `apart` refuses marks none.
  - `apart` keeps fn-212's two refusals (light, kept tree). Its doc now names why: the realized settle's roots grow alone, and the device carries no exchange.
  - `Fine::bases` reads `Settled::radii`, which now comes from `lay`.
- **Stubs and phase.** fn-212's `Stub` carries F4's phyllotaxis `phase`, and every `Stub::of` reads `geometry::phase`.
- **The grower's bud lists.** fn-212's `laterals` is renamed `handing`, because F4's balance pass owns `Grower::laterals`. F4's `handed()` stays for the stepped executor. It is now the merge of `handing(0)` and `woken(0, true)`, which is the same list in the same order.
- **Laid layout in `dress.wgsl`.** F4 widened the laid buffers, and `dress.wgsl` hard-codes them, so they were updated: scratch `SA` 66 to 70, phytomer `PL` 22 to 23, table `GG` 24 to 25. F4's new fields are appended (`S_BASE` at 66, phase at 22, spreads at 22 to 24), so every offset `dress.wgsl` reads is unchanged.
- **Device.** fn-212's `device.rs` was taken whole, with F4's divergence passed into `bearer_frame` (lay and bend). `unlaid` (now in `group.rs`) carries `phase: 0`. `Tables` keeps F4's `divergence`. `stubs` writes F4's phase at `p[22]` and fn-212's made words to `origins`.
- **Unit test kernel.** fn-212 moved it to `units_tests.rs`, and F4's `unit_key` (fn-199 R1) moved with it.
- **F4's feedback test kernels** take fn-212's deferred-destroy `Owned` buffers. The feedback contract's WGSL is `cfg(test)`, because only test kernels read it, and the fine module is now production code.
- **Registry.** fn-212 adds no lineage key and no probe selector. `dress.wgsl`'s `node_key` is a hash of the lineage, not a draw step. The contract probe keeps F4's `Selector` enum.
- **Docs.** `pipeline.md` keeps fn-212's executor text, saying that a tree that does not come apart is built by the stepped executor, and names the exchange refusal.
- **Limits inventory.** fn-212 never changed it, so it is F4's. The duplicate (path, site) pairs are sites that occur twice, as the guard counts them. `generation_limits_are_classified` passes.
- **The Wasm ceiling** is fn-212's 2,800,000.

## Hashes (`space_hash`: node count and clothed hash, before at `8f451083`, then after)

```
beech 40 1   102841 570a9fbd0b8dca69 -> 102846 9b1ffcb5797b3421
beech 40 7   181015 b45a42dec68db1ff -> 181014 f200fa233b551e9a
beech 80 1  1540449 37f039098505418a -> 1540442 422034adb7f91dd0
beech 80 7  2609652 b7c11683d4872d5a -> 2609576 5734063c680180e3
spruce 40 1 1147615 d5439c610751ea88 -> 1147168 ac84ed3b66700cf3
spruce 40 7 1107566 6d1222d78d7047f8 -> 1107132 10561fc4788207c3
spruce 80 1 4637482 5f6628522e0f9f21 -> 4635552 2b0c8d069fb283b0
spruce 80 7 4610322 5a137cf98aa5d70f -> 4608273 360ee96166ed9afd
oak 40 1     167177 0925ab79a0368b0d -> 167162 bc2f6a9f05e66c19
oak 40 7     296176 5f3b746ce08e592d -> 296157 0f568df5832dad0c
oak 80 1     950320 4feca510e443300f -> 950009 984a867310877359
oak 80 7    2333492 d07d41b4adfde08e -> 2332830 a294bf2394dbb495
palm (all four)                         unchanged
```

- **The engine's trees are unchanged.** `examples/digests` gives identical lineage digests and topologies on all 36 lines (four species, 40 and 80, two seeds, two lights, and the stepped lit oak). Every move is fn-212's conversion: `SHORTEST` in grown micrometres, the fork rule at a scaled lateral, and the seams of the beech and spruce, which are held apart.
- **The spruce's node changes equal fn-212's own TASK8 changes:** −447, −434, −1,930 and −2,049.
- **The beech loses 1.5 % of its leaves.** The hero still draws 2,532,164 instances before and 2,494,405 after. fn-212's grown-wood station count (decision D2) replaces the chord count. fn-212's `grown_stations_at_80` on the merged tree:
  - beech 80 s1: 2,531,511 by the chord, 2,494,405 by the grown wood (−1.47 %), with 36,805 runs counted lower;
  - beech 80 s7: −1.20 %;
  - spruce: −0.22 %, as fn-212 found (−0.236 %).
  - On fn-212's own base the beech moved −0.02 %. F4's laid geometry since then (the wander drawn apart, the relays' framing) makes the beech's chords longer than its grown wood. **For the host:** this follows D2 as decided; the size of the drop is new.

## Hero stills, age 80, seed 1, 1024×1024 (`raw/merge-f212/stills/`, all four viewed)

Pixels differing beyond a 2 % fuzz: beech 1.83 %, spruce 1.07 %, oak 1.05 %, palm 0 (identical). Viewed side by side, each pair is the same tree and cannot be told apart by eye. No defect is seen.

## Walks and census (`raw/merge-f212/walks-census.log`)

- `walks`: 10 of 10 pass.
- `relays`: 4 of 4 pass.
- fn-199's census: 0 R1 jumps and 0 R3 jumps over 50 seeds.

## fn-212's probes, one tree a process under the GPU lock (`raw/merge-f212/probes.txt`)

| Probe | Beech 40/80, both seeds | Spruce 40, both seeds | Spruce 80 s7 | Spruce 80 s1 |
|---|---|---|---|---|
| Dress | nodes equal, 0 strays | nodes equal, 0 strays | nodes equal, 0 strays | nodes equal (4,213,263); **6,587 strays** |
| Sweep | runs equal, 0 unmatched | runs equal, 0 unmatched | 0 unmatched; turn 8.9e-3 | **1,376 unmatched; turn 1.28e-2 over 1e-2: fails** |
| Stations | counts equal | counts equal | counts equal | half-µm class 111 keys; tied 2 / 332; 222 strays |
| Placement | 0 over, 0 left, 0 flags | 0 over, 0 left, 0 flags | out of device memory | out of device memory |

- **Real path, spruce 80 s1:**
  - leaves: 16,870,702 / 16,870,700, with 16,870,370 compared;
  - left out 0 / 332 / 0 (the named half-micrometre class), 0 over the bound, 0 flags;
  - **runs unmatched 1,376 and missing 1,376 (fn-212: 0);**
  - above-cut nodes at most 70 µm apart.
- **Out of device memory:** another process (`bun`, 1.15 GB) held the card during the placement probe at spruce 80. Not re-run.

### What parts the spruce at 80, seed 1 (traced)

- **The strays are close and grouped.** All 6,587 lie within 0.1 mm, just past the probe's 50 µm screen. They fall in 113 roots, all on one far limb (about x 5, z −5 m).
- **The CPU side is consistent.** On the CPU, the realized `settle` and the stepped build choose the same 74,358 roots (seeds 1 and 7) and build the same tree to the digest (a temporary test, removed).
- **The device lays its roots alike on F4's head and on the merge:** tips at most 7.2 / 0.44 µm RMS against `grow_roots`, with the same worst root (`realized_at_40_and_80`).
- **The structure above the cut is where they part.** Settled with the device's roots, it lies at most 70.0 µm (seed 1) and 26.4 µm (seed 7) from the CPU's, with headings at most 9.7e-5 rad.
  - F4's head gives **the same 70.0 and 26.4 µm** (the same measurement built at `8f451083`).
  - Cause: the device's f32 loads feed the CPU's sag. Roots on that limb inherit their bearers' bent frames, and so pass the screen.
- **Conclusion:** the merge did not cause this. F4's device path already bends the structure above the cut this far. fn-212's probes had not run against F4's settle before, and fn-212's named classes do not cover it.

**Options for the host:**
- (a) Name it as a class, "device loads bend the structure above the cut, up to 70 µm at spruce 80". Widen the run screens of the dress, sweep and real-path probes by it, then finish the proofs.
- (b) Trace why F4's device loads move the sag more than fn-212's base did, for example contract A's Q32 loads or fn-199 R4's integer parts. This takes the same measurement at `e7816c69`.
- (c) Hold the merge until the device's loads match the CPU's closely enough to stay inside the 50 µm screen.

## Open for the host

- **Not run:** the contract probe, the full gate, `npm test`, and the native timings (proofs 5 to 8). The fine GPU tests ran once after `1b74db15`: 31 passed.

## After the host's decisions (2026-10-08)

The measurements were made on archived copies of `e7816c69` (fn-212's base), `8f451083` (F4's head) and `34e52ec2` (fn-212's tip), each built with its own `target/`. They used temporary recorders and tests, all removed. The raw output is in this session's scratch directory.

### 1. The 70 µm sag on the spruce at 80: phase 2's radii (cause only, no fix)

Each phase's outputs were recorded on both executors at spruce 80, seeds 1 and 7. Then the device's outputs were replayed into the CPU's settle.

| Seed | Snapshot | Device-settled, CPU-settled above the cut | CPU with the device's phase 2 radii only | CPU with the device's phase 3 loads only |
|---|---|--:|--:|--:|
| 1 | fn-212's base | 25.5 µm | 29.1 µm | 3.7 µm |
| 1 | F4's head | 70.0 µm | 80.8 µm | 10.8 µm |
| 7 | fn-212's base | 28.1 µm | 32.3 µm | 4.3 µm |
| 7 | F4's head | 26.4 µm | 31.8 µm | 5.4 µm |

- **The input to phase 3 is identical on both executors:** 0.0 apart above the cut. The gap opens in phase 4.
- **The cause is phase 2's radii:** each root's base radius from `settle.wgsl`'s girth pass, in f32.
  - The device's radii alone reproduce the gap on the CPU. The loads alone give 4 to 11 µm.
  - The device's radii lie about 1.5 to 2.6e-6 apart (relative) from the CPU's on the heavy roots. That is about 30 times f32 rounding: the CPU's own radii rounded to f32 move the tips 0.05 µm. So it is the device's arithmetic, not its storage.
  - These radii thicken the structure above the cut, and its sag amplifies them.
- **The gap was there before contract A:** 25 to 28 µm at fn-212's base, where the CPU still lays in f64.
- **What changed at seed 1 is the CPU, not the device:**
  - The device's radii are the same to the last digit at both snapshots: 8.727258e-3 m on the heaviest root.
  - The CPU's moved down by 4 to 6e-9 m on every heavy root at F4's head (about −6e-7 relative), with F4's fixed-point girth.
  - The two now lie about 2.1e-6 apart, against 1.4e-6, and spruce 80 s1's far limb amplifies that to 70 to 81 µm.
  - Seed 7 is unchanged.
  - Which F4 commit moved the CPU's radii (contract A's girth, or fn-199 R4's integer parts) was not bisected.
- **The phase 3 outliers do not matter.** The roots whose mass is 100 % apart are sub-micron roots (radius about 1e-7 m, mass about 1e-20 kg) at Q32's resolution, and their loads cannot bend a limb.
- **For the host:** whether `settle.wgsl`'s girth pass takes contract A's fixed point (a design step), or which F4 commit to bisect first.

### 2. Grown wood at a spread: fixed (`1b74db15`)

- **The CPU:** the conversion's grown wood multiplies each internode by `lineage::internode`'s factor, now public as `telperion_space::internode_factor`.
- **The device:** the walk writes each laid phytomer's factor to slot 23 (`PL` 23 to 24), and `dress.wgsl` multiplies by it. `lay.wgsl`'s `internode_factor` is shared by the lay and the walk.
- **Tests at a spread of 0.7, both red without the factor and green with it:**
  - `pipeline::space::tests::grown_wood_is_the_internodes_as_laid_at_a_spread`: every node drawn after another on its axis records the laid lengths of its phytomers, within one Q32 last place each. Without the factor, node 31 records 0.07 m against 0.1028 m laid.
  - `generation::fine::dress_tests::a_beech_of_varied_internodes_is_drawn_as_the_pipeline_draws_it`: 20,250 nodes on both sides, 0 strays. Without the factor the device drew 20,933 nodes.
- **No shipped tree changes:** without spread the factor is exactly 1, and the 16 clothed hashes are unchanged.
- **Test detail:** an axis carried on into the next age keeps its chain's lineage, so `tree::node_key` names two phytomers. The test skips such keys.

### 3. The beech's −1.5 % leaves: chord ties, not a layout defect

- **Chord over arc, node to its parent, beech at 80** (the conversion's tree, classed by node):

  | Node | Seed 1 | Seed 7 |
  |---|--:|--:|
  | Along its axis | 0 of 1,391,550 | 0 of 2,355,633 |
  | A relay's first | 0 of 73 | 0 of 119 |
  | A continuation's first | 0 of 2,570 | 1 of 4,175 |
  | A lateral's first | 30 of 131,687 | 163 of 223,322 |
  | Unclassified (shared lineage key) | 415 of 14,561 | 698 of 26,326 |

  - **fn-212's tip** has the same classes in the same numbers, for example 17 and 145 laterals' first nodes, and 442 and 737 unclassified.
  - **Without wander** the counts hardly change.
  - **Not the relay stand blend:** no relay's first node has a chord over its arc.
- **In bearing runs:** 71 (seed 1) and 249 (seed 7) pairs have a chord over the arc. All but three are a run's first pair, and every one's second node is its first node's child.
- **The 1.5 % is the old chord count's ceiling at exact ties.**
  - The beech's 30 mm internode is exactly five twig internodes of 6 mm, so a run's grown wood is a whole number of them: 1.29 m, exactly 215.
  - Laid in fixed point, the same run's chord comes out 1.290000002 m. `ceil(length / unit − 1e-9)` rounds 215.0000003 up to 216.
  - Of the 36,983 runs counted apart at seed 1, 36,768 lie within 1 µm of their grown wood and account for +36,762 stations; the other 215 account for +344. At seed 7: 53,761 runs and +53,749, against 357 runs and −102.
- **The grown-wood count is the right one.** The F4 change that makes the chords land just over the boundary is inferred to be contract A's Q32 lengths, not traced. On fn-212's own base the same measure moved −0.02 %.

## The girth in contract A's fixed point (host decision, 2026-10-08; `ee13130f`)

- **The decision:** the girth pass in `settle.wgsl` mirrors the CPU's girth exactly, using existing contract operations, with no bisect. The record is in FINE-CONTRACT.md, under "Host decision (2026-10-08, the fn-212 merge into F4)".
- **Built:** `fine/girth.wgsl`.
  - **Sections and radii:** sections in W, pipes and radii in Q32, through `log2_w`, `exp2_w`, `prod_shr`, `mul_shr` and `ripe` (feedback contract) and `log2q`, `exp2q` and `mulq` (decision contract).
  - **Relays:** split by `relay_parts` at a Q32 `relay_span`.
  - **Table and records:** the PA's girth values travel as Q32 in the geometry table (GG 25 to 37), and each chain's base radius as Q32 in its record, read back exact.
  - **One new operation:** `q32_f`, the CPU's `q32` of a float scale. It is in `contract.wgsl` and the contract probe as selector 20. The probe first caught its missing saturation at 2^62; that is fixed.
  - **Names the shader now shares:** the feedback contract's `atan2` is `atan2_t` (it shadowed the builtin), `lay.wgsl`'s `unit_of` is `key_unit`, and the float `ripe` is `ripe_f`. `over` moved from `feedback_lay.wgsl` to `feedback.wgsl`.
- **Phase 2's radii, the device's against the CPU build's:**

  | Tree | Equal to the bit | Apart: median / worst (relative) |
  |---|--:|--:|
  | Beech 30 s1 | 149 of 150 | 1.5e-7 |
  | Spruce 30 s1 | 9,579 of 9,764 | 2.5e-7 / 5.2e-6 |
  | Beech 80 s1 | 4,819 of 5,040 | 9.3e-8 / 1.5e-6 |
  | Spruce 80 s1 | 71,459 of 74,358 | 3.6e-7 / 2.9e-6 |

  - **The residual is the f32 scale pass.** At spruce 80 s1, no root whose scales all equal the CPU's in Q32 has a base radius apart: 0 of 1,692. All 2,864 roots apart sit under scales the f32 pass parts. 64,235 other roots have such scales, yet their radii still round to the same bits.
  - **Gate test** `the_device_s_base_radii_are_the_cpu_s_to_the_bit`: at least 90 % of the roots at 30 equal to the bit.
- **The structure above the cut, device-settled against the CPU's** (replay into the CPU settle; spruce at 80):

  | Seed | Before | Now | The device's radii alone (scale pass) | The device's loads alone (lay) |
  |---|--:|--:|--:|--:|
  | 1 | 70.0 µm | 5.8 µm | 6.4 µm | 0.74 µm |
  | 7 | 26.4 µm | 5.2 µm | 5.8 µm | 0.79 µm |

  The scale, lay, lever and bend passes stay f32. They were not ported in this merge.

## fn-212's probes after it (`raw/merge-f212/probes-girth.txt`, one tree a process, under the lock)

- **Dress:** nodes equal and 0 strays on all eight trees. Spruce 80 s1 had 6,587 strays before.
- **Sweep:** runs equal one for one, 0 unmatched, on all eight. Spruce 80 s1's turn is 2.1e-3 against its 1e-2 bound (it was 1.28e-2).
- **Stations:** counts equal on seven trees. Spruce 80 s1 keeps fn-212's named half-micrometre class (111 keys, 2 / 332 tied).
- **Placement:** all eight compared, with 0 over the bound and 0 flags. Spruce 80 s1 has the rounding class (332 leaves) and its one grazing seat.
- **Real path, spruce 80 s1:**
  - runs: 0 unmatched, 0 missing, 0 parted, 0 sockets; above-cut nodes at most 7.1 µm apart;
  - leaves: 16,870,702 / 16,870,700, 332 in the named class, 0 over the bound, 0 flags.

## Proofs 5 to 8

- **Proof 5, the contract probe and every fine GPU test under the lock:** 32 passed, with the contract probe at 21 selectors and `q32_f` among them. With eight test threads, four tests failed for device memory while another process (`bun`, 1.15 GB) held the card; at four threads every test passes.
- **Proof 6, the gate** (at `ee13130f`; `raw/merge-f212/gate-*.log`):
  - render half under the lock: 189 passed, 0 failed, 29 ignored;
  - the rest: 1,026 passed, 0 failed, 46 ignored.
- **Proof 7, `npm test`:** 13 files and 143 tests pass.
- **The artifact ceilings (`npm run build`, then `scripts/artifact-budgets.mjs`):** four modules are over their ceiling. F4's head was already over on three of them. **For the owner:** crossing a ceiling is the owner's decision.

  | Artifact | Ceiling | F4 head `8f451083` | Merged |
  |---|--:|--:|--:|
  | `telperion-render.wasm` | 2,800,000 | 2,430,697 | **3,137,657** |
  | `telperion.wasm` | 1,600,000 | 1,700,018 | 1,758,162 |
  | `telperion-field.wasm` | 632,580 | 1,124,458 | 1,171,132 |
  | `telperion.js` | 153,565 | 154,861 | 154,994 |

  fn-212 raised the render ceiling to 2,800,000 over a base without F4's engine. F4's engine and fn-212's device executor together come to 3,137,657.
- **Proof 8, native timing** (`drawn_tree`: a parameter change to the tree drawn, one side a process, under the lock, load 3.4 to 4.8; `raw/merge-f212/timing.txt`):

  | Tree | Side | Prepare ms | Drawn ms | Peak RSS MB | Vegetation p50 ms | Frame p50 ms | fn-212 TASK10 (prepare / drawn / frame) |
  |---|---|--:|--:|--:|--:|--:|--|
  | Spruce 80 s1 | device | 1,547 / 1,592 | 1,652 / 1,693 | 820 / 880 | 49.6 / 50.1 | 70.7 / 71.4 | 1,453 to 2,349 / 1,592 to 2,570 / 69.2 to 70.7 |
  | | whole | 8,504 to 8,922 | 9,016 to 9,457 | 4,889 to 4,901 | 39.0 to 39.4 | 57.5 to 58.1 | 8,396 to 10,437 / 8,963 to 11,193 / 56.3 to 57.2 |
  | Beech 80 s1 | device | 539 to 557 | 576 to 597 | 304 to 307 | 10.2 to 10.4 | 14.6 to 14.7 | 373 / 550 / 13.59 |
  | | whole | 1,748 to 1,897 | 1,889 to 2,038 | 1,312 to 1,319 | 9.7 to 10.0 | 14.4 to 14.8 | 1,731 / 2,003 / 14.00 |

  - The spruce matches fn-212's figures.
  - The beech's device build is 551 ms against fn-212's 373 ms (drawn 588 against 550). F4's tree at the beech carries the stepped executor and contract A's lay; the extra time was not profiled.
  - The third spruce device run was refused its device ("Out of Memory") while `bun` held 1.15 GB of the card.
- **Hashes:** the 16 clothed hashes are unchanged from the merge by `1b74db15`, checked by `space_hash`. `ee13130f` changes only the device's code.
