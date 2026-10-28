# R5: florets boxed and counted on the device (task .1)

28 Oct 2026, native, RTX 3080 desk card, `--profile ci`. Base `eb6e7ccce`.

## Answers (FNV-1a with the flowers word), seed 1, the device's own box

`field_answers` was extended to hash each cell's `flowers` before any
change, then run on every shipped preset before and after
(`FIELD_PRESETS=<all eight>`). All 24 lines are equal before and after,
including the two flowering trees: the seats' box falls inside the
wood and leaves box, and every seat binned on the device's own frame
lands in the cell the CPU's binning gave.

| Preset | 24 | 36 | 48 |
|---|---|---|---|
| ordinary | db3f0e62a4b6a4ac | f0c34cf0d59263e0 | 9ba1b4673fc60872 |
| oregon-white-oak | c3ee908903462a91 | 44656f99daa32e2c | 6255c08839f39709 |
| norway-spruce | ea2009f036bbb452 | fe52f0fa635b435d | 6d2c215f340bda71 |
| european-beech | c06122507ce3426d | b96369cd4aac4051 | 4e6c41c19f059165 |
| silver-birch | dc8586c5474bfe55 | 9ddc6db2e5c068f6 | bc0188925dbd67eb |
| date-palm | fdb5d32ae41100d5 | 31ea35331505df4e | ba01aab2419293f0 |
| telperion | 032639b46ecfb344 | 5926bc7243afea18 | 3c5dac609bc6b30f |
| laurelin | 6a465eab35fbe9c0 | 41655b700e5b1a8b | 0237684ca76ce53d |

## Field time (`field_time`, warm, 36 cells, ms)

The baseline run on all presets, then the base and new test binaries
alternated three times on the trees that flower and the oak:

| Preset | before (all-preset run) | after (all-preset run) |
|---|--:|--:|
| ordinary | 409.21 | 384.76 |
| oregon-white-oak | 250.73 | 234.41 |
| norway-spruce | 505.43 | 439.90 |
| european-beech | 170.44 | 167.36 |
| silver-birch | 208.05 | 208.43 |
| date-palm | 88.96 | 94.99 |
| telperion | 902.09 | 892.99 |
| laurelin | 892.12 | 858.22 |

| Preset | base runs | after runs | median base | median after |
|---|---|---|--:|--:|
| telperion | 882.89, 929.39, 919.65 | 889.92, 914.76, 905.45 | 919.65 | 905.45 |
| laurelin | 860.78, 892.79, 891.74 | 850.54, 870.49, 856.73 | 891.74 | 856.73 |
| oregon-white-oak | 241.74, 254.03, 252.08 | 247.99, 252.13, 265.11 | 252.08 | 252.13 |

A flowering tree's field saves the floret copy and the CPU binning,
about 15 to 35 ms of some 900 (2 to 4 %), near the run-to-run spread;
a tree without flowers is unchanged.

## Flower agreement over the CPU's grid (`flowers_agree`)

`the_device_field_counts_the_flowers_on_every_grid`, after the change:
the most moved share is 4.01e-4 (Laurelin at 5678 on 48), under
`FLOWERS_MOVED` 1e-3; the totals equal the CPU's but Telperion at 5678
(10 of 157,417 more, 6.35e-5, under `FLOWERS_TOTAL` 2e-4), the same as
fn-316 measured with the CPU's binning.

| Tree | Seed | 24 | 36 | 48 |
|---|--:|--:|--:|--:|
| telperion | 1 | 7.10e-5 | 1.52e-4 | 1.67e-4 |
| telperion | 5678 | 2.29e-4 | 2.99e-4 | 3.68e-4 |
| laurelin | 1 | 9.26e-5 | 1.44e-4 | 2.26e-4 |
| laurelin | 5678 | 1.54e-4 | 3.01e-4 | 4.01e-4 |

## Limits at the largest side

At 256 cells a side the grid is `2 + 8 * 2^24` words (536,870,920
bytes), the answers `5 * 2^24 + 4` (335,544,336 bytes). The grid is
always the larger, so its limit is the one that binds: a device whose
buffers stop at 536,870,916 bytes refuses `field grid` by name; one at
536,870,920 grows the palm's field
(`the_largest_side_grows_or_refuses_as_the_device_s_limits_say`).
