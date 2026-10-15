# fn-257 task 3: sized ahead (worker, 2026-10-16)

**Cause.** fn-252 added `ground_hazard` to `Species::reads_light`; `blend/size.rs` zeroed it when predicting, `build_arena::hint` did not. The birch sets a ground hazard, so `expected_kept` returned none, the hint's kept phytomers were 0, and its device build began from the least tables (16,384 phytomers) and moved 23 times (`raw/r5/birch-trace.txt`).

**Fix.** One `Species::unlit()` beside `reads_light` zeroes every row it reads; `hint` and the blend's size prediction both call it. Test: `every_preset_is_sized_from_its_expectations` (every shipped preset's hint carries a kept prediction; red on the base for the birch).

**Measured** (`growth_profile`, warm, RTX 3080; `raw/r5/`):

| Tree | moves before → after | `tree::drawn` before | after | grow peak MB after |
|---|--:|--:|--:|--:|
| birch 40 s1 | 23 → 0 | 383 ms | 210 ms | 573 |
| birch 40 s1234 | 23 → 0 | 415 ms | 237 ms | 573 |
| spruce 40 s1 | 1 → 1 | 414 ms | 417 ms | 1,595 |
| oak 40 s1 | 0 → 0 | 227 ms | 250 ms | 838 |

The spruce still moves once after cycle 38 (units and woken tables, which its own ARENA ratios set at margin 1.0) and compacts twice; that is about 50-70 ms, left as measured (the owner accepted the spruce at about 0.35 s).
