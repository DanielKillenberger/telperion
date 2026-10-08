# fn-212 merged into F4 (worker, 2026-10-08): halted at the device parity probes

fn-212's head `34e52ec2` was merged onto F4's `8f451083`. The merge compiles across the workspace, and the CPU proofs hold. The proofs **stopped at item 3**: on the spruce at 80, seed 1, fn-212's dress, sweep and real-path probes now part 1,376 runs that fn-212 matched. The cause is already in F4's device path, and it is not one of fn-212's named classes (below). The gate, `npm test` and the timings were not run.

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

- **fn-222's internode spread against fn-212's grown wood.** fn-212 measures a node's grown wood as `internode[pa] × scale`, in the conversion and in `dress.wgsl` alike. F4's internode spread scales each internode by its own factor, so with a spread above 0 that measure is not the wood grown. Both executors would still agree. Every shipped preset has a spread of 0, so no tree here changes.
- **Not run:** the contract probe and the fine GPU tests under the lock, the full gate, `npm test`, and the native timings (proofs 5 to 8).
