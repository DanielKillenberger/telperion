# fn-229: florets placed on the device (worker, 2026-10-08)

The device now draws the florets of the fine roots it dresses, so a flowering family comes apart like any other. The interim refusal in `executor::apart` is deleted. The work follows the host's decisions D1 to D5 (HOST-DECISIONS.md), and raw output is in `raw/`.

## What was built

- **Keys and shares (D1).**
  - The settle lays each phytomer's 64-bit key in the laid record (`PL` 24 to 26).
  - The dress copies the key and its age's flowering share into the drawn record (`CV` 26 to 29).
  - The sweep copies all three into its node (`NW` 27 to 30).
  - The share is a new row in the per-PA geometry table (`GG` 37 to 38, `PaGeometry::flowering`).
  - No binding limit was raised, and nothing was re-keyed.
- **Counting (D3).** A `florets` kernel counts each fine node's kept florets. It writes the count, and whether the node keeps a bloom, beside the node's root-run count in the stations' existing per-node owners scan (`OW` 3). The totals come back in the stations' existing readback. A `blooms` kernel then writes one 32-word record for each bloom. The scan is deterministic and uses no atomics.
- **Placement (D2).**
  - `bloom.wgsl` mirrors `bloom.rs` draw for draw.
  - `place.wgsl` gains a second entry point, `bloom`, recorded in the encoder that places the device's leaves, so both go in one submission.
  - Each floret is packed as a leaf is.
- **Box and delivery (D4, D5).**
  - Both blocks are packed against the tree's combined leaf box, padded by `rachis + pedicel` (`Expansion::organ_in`). The CPU block's florets are repacked against it.
  - One organ buffer holds the CPU block first and the device's after it, with one mass grid over both. `submit_prepared` hands it to the organ's `submit_resident`.
- **Shadows (R3).** fn-216's organ draw is not in the shadow pass (`lib.rs`: the shadow pass draws wood and leaves only), so flowers still cast no shadow. This is recorded here, as R3 allows, and `docs/pipeline.md` says so.

**One reading of D3 for the host:** D3 asked for "one per-run count each for stations and florets". A bloom stands on any drawn node of a flowering age (`space.rs`, `convert`), not only on the bearing runs the stations number. So the florets are counted per node and folded into the stations' other existing scan, the per-node owners scan, rather than the per-run `bases` scan. There is still no scan of its own and no added readback.

## Parity (R2)

Each tree is grown with every age flowering at 0.6 under the oak's catkin rows (fn-216's overlay, with a 3 mm stalk so the pedicel is drawn too). It is built on the real path and compared against the CPU whole tree's florets packed against the same box (`florets_tests.rs`).

| Tree | CPU block | Device block | Whole tree | Exact | Within | Turned | Least (device, whole) | Unmatched |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| beech 30 s1 (gate) | 88,816 | 416,837 | 505,653 | 481,022 | 24,631 | 0 | 0, 0 | 0, 0 |
| spruce 30 s1 (gate) | 955,902 | 9,707,849 | 10,663,750 | 10,165,333 | 498,406 | 11 | 1, 0 | 0, 0 |
| beech 40 s7 | 125,776 | 3,243,230 | 3,369,006 | 3,183,258 | 185,544 | 204 | 0, 0 | 0, 0 |
| spruce 40 s7 | 1,284,963 | 19,395,913 | 20,680,876 | 19,675,631 | 1,005,206 | 39 | 0, 0 | 0, 0 |
| beech 80 s1 | 1,253,355 | 27,518,555 | 28,771,908 | 27,217,660 | 1,553,579 | 669 | 2, 0 | 0, 0 |
| spruce 80 s1 | 7,886,807 | 78,633,638 | 86,520,440 | 82,444,340 | 4,075,582 | 518 | 5, 0 | 0, 0 |

- **Florets per tree** equal the whole tree's, apart from the least class: +1 on spruce 30, +2 on beech 80 and +5 on spruce 80. The CPU block alone matches MERGE-F216.md's device-path counts, for example the beech at 40 s7's 125,776.
- **Within the screen.**
  - Positions agree to one 16-bit box code (the largest offset is 0.16 to 0.54 mm, which is one code of the box at each age).
  - Frames agree within 5.4 to 9.3 mrad, under the 10 mrad screen.
  - Scales agree within one half-float step.
  - The 50 µm screen lies under one code, so a position check finer than one code is not possible on packed words.
- **Named classes,** both f32-against-f64:
  - **Least.** A floret whose size lies at `bloom.rs`'s least size of 1e-3, within a half-float step, is kept on one side and dropped on the other. Every case was a floret the device kept.
  - **Turned.** The same floret, its size within the screen, stands 1 to 4 mm off, under twice the stalk's length. Measured on spruce 30 (`raw/turned-margins-spruce-30-1.txt`):
    - every turned floret's axis tangent lies within 1.4e-4 of `Vec3::perpendicular`'s switch, where two components are equal in magnitude, against 0.35 % of all florets lying under 1e-5;
    - the bloom inputs the device recorded (node, parent, bud side) agree with the CPU's within 2.4e-7.
    - At the switch, the two executors take different reference axes and the golden spiral starts from another normal. It is the leaves' "face turned" class of fn-212 in another form.

## Shipped species unchanged (R4)

- The device path's buffers for the beech and the spruce at 40 s7 were compared before (`7f781e19`) and after, with no flowering:
  - the sorted leaf words are equal;
  - the curve's points and clusters are equal;
  - the crown bounds are equal.
- **Found, not caused by this change:** the device path's leaf buffer *order* varies from run to run at the base. A root's runs are ranked by an `atomicAdd` in `stations.wgsl`'s `owned`. So the device stills vary by 0.01 to 0.3 % of pixels from run to run on either commit (`raw/inert/`).
- The florets are byte-identical across three runs (spruce 40 s7, 20,680,876 florets).

## Stills (`raw/stills/`, all four viewed)

- **Setup:** the flowering beech at 40 s7, at two device-drawn seams (`seam_stills::cameras`, pulled back 2.5×), on the real path and grown whole and delivered resident.
- **Florets:** both paths draw 3,369,006. The catkins hang from the twigs, the same on both.
- **Pixels apart:**
  - pair 0: 0.8 % exact, 0.3 % under a 2 % fuzz;
  - pair 1: 2.9 % exact, 2.5 % under fuzz. The difference is leaf shading, not florets.
- **Not this change:** the same cameras without flowers give the same pair 1 difference (2.5 % under fuzz; `raw/stills-noflower/`).

## Gate (`raw/gate-*.txt`, `raw/npm.txt`)

- **Render half,** under the GPU lock, first: 193 passed, 0 failed, 31 ignored.
  - Its first run failed `the_stations_keep_only_their_segments`: a 16-byte bloom-records buffer was kept live on every non-flowering tree. The records are now held only where a bloom keeps a floret, and the rerun is the count above.
- **The rest of the workspace:** 1,040 passed, 0 failed, 46 ignored.
- **`npm test`:** 13 files and 143 tests pass, and the build left the tree clean.
- **space_hash:** not taken. No CPU tree changes: every shipped age has flowering 0, and the CPU path's conversion and bloom draws are untouched.
