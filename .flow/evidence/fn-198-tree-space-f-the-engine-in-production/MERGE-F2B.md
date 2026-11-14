# fn-211 (F2b, contract A) merged into F4 (worker, 2026-10-08)

fn-211's head `bd291391` was merged onto F4's `18a6c9aa` as `8818b93b`. Follow-ups:
- `32291635`: the device's lay is tested with spreads;
- `9a8588c1`: the palm's catalogue re-pin;
- `39ced0c1`: the Wasm build fix;
- `67970709`: a byte-identical shortcut for continuations.

Contract A's rules (CONTRACT-A.md) were applied to every port.

## What was ported

- **fn-222 internode spread and fn-224 tropism spread.** Each is a Q32 factor, `ONE32 + spread (2u − 1)`, with `u = d >> 21`, and is exactly `ONE32` without spread.
  - A length is `prod_shr([internode, scale, factor, 1], 64)`, and a tropism `prod_shr([tropism, turning, length, 1], 64)`. Each is one floor, with the plain product's bits at spread 0.
  - `Layer` carries `turning`.
  - The device's rough layout (`feedback_lay.wgsl`) reads the internode spread in PA slot 7, which was a pad, and the axis's factor in job word 23, also a pad. `LayJob` and `LayPa` carry them.
- **fn-224 insertion spread.** An amount (Q32 turns): `amount(angle) + mul(amount(spread), 2u − 1)`, one floor, as the roll is. The closed form's `lineage::turn` stays f64, as the closed form does.
- **fn-199 rule 1.** One frame arm for relays and continuations: bend, reaches, handed pulls and the stepped executor's jobs all read `carried_on`.
- **fn-199 rule 2'.** `geometry/stand.rs` is now in Q32:
  - extents are summed in Q32 and capped at `ONE32`;
  - the units are blended by `mul`;
  - the blended side is made square again;
  - the azimuth is an amount.
- **The phase.** `Axis.phase` and `BearerFrame.phase` are amounts (i64). Each axis carried on takes `mul(ONE32 − b, phase + divergence × rank)`, one floor. The f32 GPU walk reads the phase in turns.
- **fn-199 R4.** Integer parts in girth and sag (`girth::relay_parts`): `b` of the section or load is floored, then `t` of that part, with each remainder exact, so the three parts sum to the whole. `relay_span` returns Q32. The bounded chain's f64 girth keeps `relay_shares`, as the bounded chain stays f64.
- **WANDER.** The key comes from the registry (`u64::MAX − 13`) on the CPU and in `feedback_lay.wgsl` (`KEY_WANDER`).
- **Grown extents in the rough layout.** These are `prod_shr([present, node, grown, 1], 64)`. On the device path the CPU computes them from the units' presences when it applies a laid job; this adds no device word.
- **Conflicts resolved:**
  - **`log2`:** F2b's polynomial is kept and made `const`, because F4's presence bounds build a `const` table from it. F4's squaring `log2` and `log2_pair` are deleted, and F4's `wide` helpers are kept.
  - **The stepped cut:** it reads F4's `Option` count, with an undrawn PA never fine.
  - **Timings:** they are on the engine's clock.
  - **Pinned math:** `log2_fixed` is added to the pinned trait for sag's log, and the sweep's `exp` is pinned.

No new draws were needed: the four keys the ports read were already in the registry.

## Hashes (`space_hash`, node count and clothed hash, before at `18a6c9aa` then after at `8818b93b`)

```
beech 40 1 102841 5811567f7f3670bf 102841 570a9fbd0b8dca69
beech 40 7 181015 11f2c0b5a799ac06 181015 b45a42dec68db1ff
beech 80 1 1540449 e12f497857fd4d8a 1540449 37f039098505418a
beech 80 7 2609652 cff3dcd09da8775a 2609652 b7c11683d4872d5a
spruce 40 1 1147615 3324e54ed90088b7 1147615 d5439c610751ea88
spruce 40 7 1107566 cda21916e9c0abf6 1107566 6d1222d78d7047f8
spruce 80 1 4637482 18d1e715d3968286 4637482 5f6628522e0f9f21
spruce 80 7 4610322 7ab24d5670ac9ed6 4610322 5a137cf98aa5d70f
oak 40 1 167177 1369b2a64d812568 167177 0925ab79a0368b0d
oak 40 7 296176 78cfd0ebb356b793 296176 5f3b746ce08e592d
oak 80 1 950317 04b0a548c5595683 950320 4feca510e443300f
oak 80 7 2333490 ecc67a4291310e96 2333492 d07d41b4adfde08e
palm 40 1 993 061987de19837971 993 6925eddec733ff1c
palm 40 7 993 8d2ce3067c21f86e 993 6fbb97f706637c3b
palm 80 1 1473 025e167fec51c301 1473 3cbb16e7dc559423
palm 80 7 1473 333ad5c2de1e903a 1473 f0c0f106297382f8
```

All 16 hashes move. `67970709` gives the same 16 hashes.

## What moves (`contract dump`, 28 trees by lineage, F4 head against the merge)

- **Topology:** unchanged in 27 of 28 trees. The lit oak at 80, seed 7, gains 4 axes (33 phytomers) and loses none.
- **Worst tip move on matching phytomers:**
  - beech: 0.4 µm at 40, 22 µm at 80;
  - spruce: at most 6 µm whole and 0.12 mm alone (80, seed 7);
  - oak at 40: 11 µm;
  - oak at 80: 0.12 mm (seed 1) and 3.5 mm (seed 7, beside its new axes; RMS 25 µm);
  - palm: 1.9 µm.
- **Radii:** the relative changes at or near 1 are radii below about 1e-9 m. Most sit at or under Q32's last place (2.3e-10 m) and floor to it or to none, as TASK9-CPU.md found.

| Tree | Axes | Only before / after | Phytomers | Only before / after | Tip max / RMS (m) |
|---|--:|--:|--:|--:|--:|
| beech 80 1 | 180955 | 0 / 0 | 1614523 | 0 / 0 | 2.97e-6 / 5.37e-7 |
| beech 80 7 | 316402 | 0 / 0 | 2799688 | 0 / 0 | 2.18e-5 / 9.54e-7 |
| spruce 80 1 | 955679 | 0 / 0 | 4780267 | 0 / 0 | 1.61e-5 / 2.16e-6 |
| spruce 80 7 | 948691 | 0 / 0 | 4722702 | 0 / 0 | 3.85e-5 / 1.89e-6 |
| oak 80 1 | 174809 | 0 / 0 | 1364357 | 0 / 0 | 1.17e-4 / 3.57e-6 |
| oak 80 7 | 426978 | 0 / 4 | 3121490 | 0 / 33 | 3.52e-3 / 2.50e-5 |
| palm 80 1 | 1 | 0 / 0 | 960 | 0 / 0 | 1.33e-6 / 6.99e-7 |

The full table, ages 40 and 80 with the alone trees, is in `raw/merge-f2b/` (gitignored).

## Hero stills, age 80, seed 1 (`raw/merge-f2b/stills/`, viewed)

- **Method:** before and after were rendered by each commit's `headless`, at 1024x1024.
- **The before renders** equal `raw/wander/after/` to the pixel, so the before binary draws F4's head.
- **Pixels differing beyond a 2 % fuzz:**
  - beech 1.7 %;
  - spruce 1.0 %;
  - oak 0.5 %;
  - palm 0.002 %.
- **Viewed:** no pair can be told apart by eye: the same crowns, limbs and shadows. No defect is seen.

## Walks, census, probe and gate

- **Walks (all settings):** `walks` passes 10 of 10; `relays` passes 4, with the census ignored.
- **fn-199's census** (`census_of_r1_and_r3_over_fifty_seeds`): R1 has 0 jumps and R3 has 0 jumps over 50 seeds.
- **Under the GPU lock, `generation::fine`:** 14 passed, 0 failed. They include:
  - the contract probe (`the_gpu_takes_every_decision_the_cpu_does`);
  - the feedback probe;
  - the mirror tests (keys and selectors);
  - the feedback slice;
  - the lit fine roots on the device;
  - every root grown and settled, and varied forms.
- **The device test now exercises the spreads.** The lit fine roots test's neighbour oak now varies its internodes, insertions and tropisms (`32291635`). With the shader's spreads disabled it fails (checked, then restored).
- **Gate, render half under the lock** (at `32291635`): 170 passed, 0 failed, 13 ignored.
- **Gate, the rest** (`--exclude telperion-render`): 1,022 passed and 1 failed:
  - The failure was `the_palm_s_pins_are_the_ones_its_catalogue_records`, because contract A moves the palm's stem by up to 1.9 µm.
  - The palm was re-pinned (`9a8588c1`), and `telperion-jev` then passed 389 of 389.
- **`npm test`:** it first failed to build the Wasm. F2b's `lattice::LEAVES` (`1 << 40` as a `usize`) overflows on wasm32 and was made a `u64` (`39ced0c1`). After that, 13 files and 143 tests pass, the native-against-Wasm parity suite among them.
- **Re-pins and tolerances:**
  - the closed form's rows were re-pinned for the beech, spruce and oak (the palm's hold);
  - `tests/form.rs`'s lateral angle moved from 1e-8 to 2e-8 (1.02e-8 measured);
  - `tests/internodes.rs`'s unvaried length moved from 1e-9 to 1e-6. Both are Q32's last places.

## Timing: one build per species at 80 (`space_hash`, pipeline build and clothing)

The two binaries alternated, with each run started at a one-minute load under 6. The table gives the best of three, in ms.

| Tree 80 | F4 head s1 / s7 | Merged s1 / s7 | Ratio s1 / s7 |
|---|--:|--:|--:|
| beech | 951 / 1,546 | 1,048 / 1,760 | 1.10 / 1.14 |
| spruce | 3,834 / 3,680 | 4,670 / 4,647 | 1.22 / 1.26 |
| oak (lit) | 5,732 / 12,557 | 7,349 / 14,633 | 1.28 / 1.17 |
| palm | 3 / 3 | 3 / 3 | 1.0 |

Contract A's fixed-point layout costs 10 to 28 % of a whole build here.
- TASK9-CPU.md measured +4 to +22 % against an f64 base, and the host accepted that residual for the device to take on.
- The oak's +17 to 28 % is above that range. Its cause is not profiled here.
- `67970709` skips the stand for continuations, with the same bits. Its timing gain is within noise.
