# fn-216 merged into F4 (worker, 2026-10-08)

fn-216's head `d4e3e16c` merged onto F4's head `b0a16c6a`. fn-216 brings flowers on the engine path and fn-220's node identity from lineage.

- **The first pass stopped at the device-path check.** On fn-212's device path, flowers are drawn only from the tree the CPU converts. A fine root dressed on the device never enters that tree, so most florets were missing (table below).
- **Host decision (2026-10-08): option (a), an interim named gap.**
  - `executor::apart` refuses a family any of whose ages flower, as it refuses light and kept trees. That family grows whole on the CPU.
  - This is interim: a flowering share crossing 0 switches executors, which the principles do not allow to stay.
  - The lasting fix is (c), fn-125's GPU bloom pass, to be written as its own spec. The supernatural trees depend on it.

## Flowers by path (proof that the gap is closed)

Every age flowered at 0.6 with the oak-catkin bloom rows (temporary probe, since removed):

| Tree | Path at the merge | Florets on the device path / grown whole | Leaves |
|---|---|--:|---|
| Beech 40 s7 | device | 125,776 / 3,369,006 (3.7 %) | equal |
| Beech 80 s1 | device | 1,253,355 / 28,771,908 (4.4 %) | equal |
| Spruce 40 s7 | device | 1,284,963 / 20,680,876 (6.2 %) | equal |
| Oak 40 s7 | CPU (reads light) | 5,532,316 / 5,532,316 | equal |
| Palm 40 s7 | CPU (no twig stations) | 9,184 / 9,184 | equal |

**Test:** `generation::fine::route_tests::a_flowering_tree_draws_every_floret_on_either_path` (beech at 12, every age at 0.6).
- It requires the refusal, and requires the resident, whole and CPU deliveries to draw the same florets.
- Red without the refusal: 1,900 against 2,643 florets. Green with it.

`docs/pipeline.md` now says what is true: flowers are one CPU step over the converted tree, and a flowering family is refused the device path until fn-125.

## How the conflicts were resolved (15 files)

- **Conversion.** `pipeline/space.rs` reads one `PaRow` per PA: internode, spread (fn-222) and flowering share (fn-216).
  - F4's `convert` (seams, attachments, frames, keys, grown wood) now also fills `blooms`.
  - fn-220's `NodeIdentity::lineage(p.key())` merged cleanly.
  - `Settling` carries the rows.
- **fn-220's identity tests** were ported to F4's API. F4 has no `grow_whole` or `grow_fine`, so they use `grow` and `grow_roots` at cuts 0, 16 and 1,024.
- **Lineage keys.** fn-216's raw draw keys moved into `telperion_space::lineage_keys`. Their values are unchanged, so no floret moves.
  - Under a node's key: `BLOOM`, `BLOOM_ROLL`, `FLORET` (0x0b10_0000 to 0x0b10_0002).
  - Under a new `Under::Floret`: `FLORET_SIZE` and `FLORET_OPENING` (0 and 1).
  - All five are mirrored in `fine/lineage.wgsl`.
  - fn-220 uses the phytomer's own key, not a raw value. fn-216 adds no probe selector.
- **Wire ranks.** Both branches took wire ranks 250 to 255 (F4's fn-221 old-bark rows and fn-216's bloom rows), and both took material check slots 95 to 100.
  - fn-216's moved: wire 256 to 277, material checks 101 to 106.
  - The catalogue now has 278 rows.
- **Device path.** fn-212's `resident.rs` built its `TreeMesh` without `organ`. It now takes `Expansion::organ` and its bounds, which are none while the refusal holds.
- **Renderer.**
  - `frame.rs` takes fn-216's `let uniforms` and the organ copy, plus F4's `mature` and `mature_radius`.
  - The organ uniforms are a `buffer::Owned`.
  - `lib.rs` keeps F4's `gpu.encoder` and submit-and-watch.
  - `shoot_colour.rs` keeps F4's shared ramp, and `common/ramp.rs` gets `organ: None`.
- **Sweep.** `HELD` has 100 rows: F4's 78 plus fn-216's 22.
- **Generated files.**
  - `docs/parameters.md` and `parameters.generated.ts` were rewritten with `TELPERION_WRITE_REFERENCE=1` (`parameter_reference`).
  - `presets.generated.ts` was rewritten by `scripts/build-wasm.mjs`.
  - The catalogue pages were rewritten by `scripts/catalogue-pages.mjs`.
- **Re-pins.**
  - **`catalogue_identity`:** all four digests. With the flower rows stripped from the text (`bloom`, `bloom*` colours, `flowering`, in both Debug and JSON form), the family, walk and override digests are F4's to the bit. The refusal digest pairs rows by index.
  - **Date palm pins:** placement and bounds only. The skeleton hash is F4's. fn-220 keys the fronds' draws by their apex's lineage, as on fn-216, so the frond jitter changes over F4's tree: height 21.451 to 21.678 m. Counts, element hash and leaf band hold.
- **Limits inventory:** one new site, the flowering refusal in `executor.rs`. Every other entry merged cleanly. `generation_limits_are_classified` passes (exact counts per site).
- **Evidence markdown:** no conflicts.

## Proof 1: the 16 clothed hashes (`space_hash`; before at `b0a16c6a`, then the merge)

```
beech 40 1   102846 9b1ffcb5797b3421 -> af1e34d92f5de418
beech 40 7   181014 f200fa233b551e9a -> 42bfbabd8debdc39
beech 80 1  1540442 422034adb7f91dd0 -> 582d621737d6ba3c
beech 80 7  2609576 5734063c680180e3 -> c0d3301359e16721
spruce 40 1 1147168 ac84ed3b66700cf3 -> b32c14b9a41e5405
spruce 40 7 1107132 10561fc4788207c3 -> 4d7fe97d251a607e
spruce 80 1 4635552 2b0c8d069fb283b0 -> 35e51826f380cf43
spruce 80 7 4608273 360ee96166ed9afd -> 38eb1907944fb770
oak 40 1     167162 bc2f6a9f05e66c19 -> 611cb08d00036381
oak 40 7     296157 0f568df5832dad0c -> 616d85aadf3289bd
oak 80 1     950009 984a867310877359 -> 7236dea7302e4782
oak 80 7    2332830 a294bf2394dbb495 -> 1992d9445d185fed
palm 40 1       993 6925eddec733ff1c -> da01c8cbf52dd468
palm 40 7       993 6fbb97f706637c3b -> de27cb076da06ca5
palm 80 1      1473 3cbb16e7dc559423 -> 7c90dd11f427e83a
palm 80 7      1473 f0c0f106297382f8 -> 84ca7b46a037c49d
```

- **Every hash moves, and every move is fn-220's identity alone.**
  - Node counts are unchanged.
  - `space/hash.rs` hashes each node's `identity`. Hashing the merged trees with every identity reset to the default gives all 16 of the before hashes exactly (temporary edit of `space_hash.rs`, reverted).
  - No shipped species flowers, so `blooms` is empty on all of them.

## Proof 2: hero stills, age 80, seed 1, 1024×1024 (`raw/merge-f216/stills/`, all viewed)

- **Pixels apart beyond a 2 % fuzz:**
  - beech, spruce, oak: 0 (identical);
  - palm: 1.65 %.
- **The palm** is the same tree, with the same height, crown, frond count, trunk lattice and shadow. Only the frond jitter differs, frond by frond, as fn-220's evidence found on its own base (viewed side by side).
- **Florets:** every still draws 0 florets.

## Proof 3: oak catkins, close (`raw/merge-f216/stills/oak-close-480ffb.png`, viewed)

- **Setup:** the oak at 40, seed 7, under fn-216's `overlays/oak-catkins.json`, with the camera at `{"distance":2.5,"targetHeight":0.6,"elevation":0,"fov":30}`.
- **Drawn:** 5,506,430 florets.
- **Viewed:** yellow-green catkins hang from the twigs beside the leaves, as on fn-216's sheet `sheet-oak-close-r5.png`.

## Proofs 4 to 7 (raw output in `raw/merge-f216/`)

- **Proof 4, the fine GPU tests under the GPU lock, four test threads (`fine.log`):** 33 passed, 0 failed, 22 ignored. These include:
  - the contract probe (`the_gpu_takes_every_decision_the_cpu_does`);
  - the feedback probes;
  - the mirror tests, which now cover the five bloom keys.
- **Proof 5, the walks (`walks.log`):**
  - `walks`: 10 of 10 pass.
  - `relays`: 4 of 4 pass, with the census ignored.
- **Proof 6, the gate:**
  - render half under the GPU lock: 192 passed, 0 failed, 29 ignored (`gate-render.log`);
  - the rest of the workspace: 1,040 passed, 0 failed, 46 ignored (`gate-rest.log`).
- **Proof 7, `npm test`:** 13 files and 143 tests pass (`npm.log`). The pre-test Wasm build left `presets.generated.ts` as committed.

## Open for the host

- **fn-125's GPU bloom pass (option c).** Until it lands, a flowering share crossing 0 switches executors. The refusal names fn-125, as do the doc comment on `executor::apart` and `docs/pipeline.md`.
