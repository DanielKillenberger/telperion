# fn-316: the field answers a tree's flowers

## Device against the CPU (2026-10-25, `--profile ci`, the desk's RTX GPU)

`the_device_field_counts_the_flowers_on_every_grid`: per tree, the CPU's floret total, the device's, and the florets the device counts in another cell than the CPU's.

| Tree | Seed | Side | CPU | Device | Moved | Share |
|---|--:|--:|--:|--:|--:|--:|
| telperion | 1 | 24 | 197,183 | 197,183 | 14 | 7.1e-5 |
| telperion | 1 | 36 | 197,183 | 197,183 | 30 | 1.5e-4 |
| telperion | 1 | 48 | 197,183 | 197,183 | 33 | 1.7e-4 |
| telperion | 5678 | 24 | 157,417 | 157,427 | 36 | 2.3e-4 |
| telperion | 5678 | 36 | 157,417 | 157,427 | 47 | 3.0e-4 |
| telperion | 5678 | 48 | 157,417 | 157,427 | 59 | 3.8e-4 |
| laurelin | 1 | 24 | 658,740 | 658,740 | 50 | 7.6e-5 |
| laurelin | 1 | 36 | 658,740 | 658,740 | 95 | 1.4e-4 |
| laurelin | 1 | 48 | 658,740 | 658,740 | 149 | 2.3e-4 |
| laurelin | 5678 | 24 | 519,030 | 519,030 | 80 | 1.5e-4 |
| laurelin | 5678 | 36 | 519,030 | 519,030 | 155 | 3.0e-4 |
| laurelin | 5678 | 48 | 519,030 | 519,030 | 208 | 4.0e-4 |

The oak answers zero on both. The spec's "counts are exact" was written unchecked: the device tree is f32, so a seat near a cell face can fall either side, and Telperion at 5678 hangs 10 more florets on the device. Tolerances: moved share ≤ 1e-3, total ≤ 2e-4 (about twice and three times the measured most).

## The frame

At seeds 1 and 5678, Telperion's and Laurelin's seats lie inside the wood and leaf box, so the field's frame is the one it was and the wood, radius, leaves and limbs answers are unchanged. R2 only widens the box for a tree whose florets hang past it.

## The other answers and the time

`field_time_tests::field_answers` (FNV-1a over every preset's frame and its wood, radius, leaves and limbs words at 24, 36 and 48 cells, seed 1) prints the same 24 lines on the base (`f4`, 30394598c) and on this branch: those answers are byte-identical.

`field_time_tests::field_time` (warm, the second of two read-outs at 36 cells, seed 1), the two binaries run alternately three times on the desk's GPU:

| Tree | Base ms (3 runs) | fn-316 ms (3 runs) | Median change |
|---|---|---|--:|
| telperion | 877.3, 888.8, 890.7 | 890.5, 887.8, 899.4 | +0.2 % |
| laurelin | 841.9, 838.2, 842.5 | 846.0, 850.0, 853.1 | +1.0 % |
| oregon-white-oak | 244.7, 242.6, 242.0 | 240.3, 236.3, 239.3 | -1.3 % |

No flowering preset is over the spec's 5 %.

## Found, not fixed

`telperion-space/src/shed.rs:211` subtracts with overflow in the dev profile (overflow checks on) while building a CPU reference in `field_tests`; the CI profile (release, no checks) runs it. It needs checking on the base before anyone fixes it.
