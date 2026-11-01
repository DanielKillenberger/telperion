# fn-320 result

A tree grown twice on this device now grows the same field and the same digest, in every run measured. The cause and its cost are in `FUSED.md`. All figures are from 1 Nov 2026, RTX 3080, driver 615.78.08, `--profile ci`. Raw logs are in `raw/`.

## R1: the cause

A fused multiply-add taken on some runs of a program and not others. One invocation on restored words gave two answers, and keeping the marked products apart removed the second. The fusing is inferred; why the driver takes two paths is unknown.

## R2: the field

`every_preset_grows_the_same_field_on_thirty_repeats`, every shipped preset at seeds 1 and 5678, grid of 24:

| Run | Result |
| --- | --- |
| Process 1 (`r2-a.log`) | 0 of 29 parted on all 16 |
| Process 2, held to process 1's hashes (`r2-b.log`) | 0 of 29 parted on all 16, hashes equal |

The ordinary test is `the_worst_cases_grow_the_same_field_each_time`: birch and oak at 5678, 8 grows each.

**How much the zeros say.** The second answer came in bursts before the fix, and 2 of 8 oak runs showed none, so these are measured across processes, not proven:

| Oak 5678, or the old flaky test | Before | After |
| --- | --- | --- |
| Oak repeats that differ, 8 runs | 58 of 604; 2 runs had none | 0 of 150, one run |
| The catalogue, 8 rounds | parted on the first 4-round run tried | 0 parted (`kept2.log`) |
| `every_preset_grows_its_field_from_its_parameters_as_from_its_id`, alone | failed 3 of 6 | passed 8 of 8 |

## Mutation proof

With the marks plain again:

- `the_worst_cases_grow_the_same_field_each_time` failed 1 of 10 runs (`mutation.txt`), on oak. So it catches the defect only sometimes.
- `every_float_module_keeps_its_marks_apart` fails whenever a float module goes back to plain marks. It reads the sources, so it holds the fix in place but cannot see the defect itself.

## R3: the renderer's device build

The cause reaches it, since the build grows through the same cycle. `every_preset_digests_the_same_on_thirty_repeats`, every preset the device draws at seeds 1 and 5678: 0 of 29 parted on all 16, in each of two processes, with the second process's digests held to the first's (`r3-digest-a.log`, `r3-digest-b.log`).

## Errors: the answers changed

The fix changes every field's bytes, as the kept products round otherwise. `field_answers` at seed 1, against fn-319's `FLORETS.md`:

| Preset | 24 | 36 | 48 |
| --- | --- | --- | --- |
| ordinary | db3f0e62a4b6a4ac to c11ffe96d2166316 | f0c34cf0d59263e0 to 99ec77fba3144361 | 9ba1b4673fc60872 to df8cfdd32134aa66 |
| oregon-white-oak | c3ee908903462a91 to 65869c3af10c0f56 | 44656f99daa32e2c to d33d0ce8fc7e7687 | 6255c08839f39709 to a252c19b50ee328f |
| norway-spruce | ea2009f036bbb452 to ed24adc0b276ef9d | fe52f0fa635b435d to 3c0a3933ef181f28 | 6d2c215f340bda71 to 32d09a785b91e208 |
| european-beech | c06122507ce3426d to 190360bf83faef89 | b96369cd4aac4051 to 2e8f28158ee18216 | 4e6c41c19f059165 to 3b7dc862bf9a52e6 |
| silver-birch | dc8586c5474bfe55 to 633bf34cfb4eb92e | 9ddc6db2e5c068f6 to 6d4e95f1bd473d84 | bc0188925dbd67eb to 00b36c0cc8badaab |
| date-palm | fdb5d32ae41100d5 to 26dd1e3d57ea2d5f | 31ea35331505df4e to dce21db0974bda23 | ba01aab2419293f0 to bd78c8e71f34e2f1 |
| telperion | 032639b46ecfb344 to e3aac008e0312d60 | 5926bc7243afea18 to 118df8d477a8c98a | 3c5dac609bc6b30f to 8d3952b877119f69 |
| laurelin | 6a465eab35fbe9c0 to 57dff4cf26e8fb9e | 41655b700e5b1a8b to 13008a72bd03c219 | 0237684ca76ce53d to 47fd994192cb0ce4 |

**The look.** Stills of the oak, birch and spruce at seed 1, plain against kept, viewed in pairs: the same silhouettes and branches, with only leaf-level speckle apart (`raw/stills-*`). The three are a sample, not every preset.

## Limits

- One GPU and driver. Other hardware is unmeasured, and cross-GPU agreement stays fn-318 R4's.
- Unmarked products remain in `light_sweep.wgsl`, the dress and the stations, and none parted here.
- The cause's mechanism in the driver is unknown, so a driver update could change this. The ordinary tests are what would show it.
