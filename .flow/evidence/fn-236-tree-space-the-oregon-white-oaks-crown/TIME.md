# fn-236: the oak's warm device build, against fn-237 R0a (2026-10-14)

The bar (fn-237 R0a): every shipped tree is ready in under 0.5 s warm on the device. The source is `oak_time_probe_tests.rs`, run by `time.sh`.

## Method

- The harness's resident build, `Generator::prepare` with `Delivery::Resident`, at seed 1, on the RTX 3080.
- One warm-up build, then five wall-timed builds. The table gives their median.
- Then five builds with every compute pass GPU-stamped. The table gives the median of the stamped passes' summed time and of their span from the first start to the last end. Stamping adds wall time, so the stamped runs are kept apart from the wall-timed ones.

## Value files

| Name | Commit | What it is |
|---|---|---|
| r3 | `5a7665aa5` | The release branch's oak, round 3. |
| r4-blade | `5ae542cd2` | Round 4 with the 72.5 mm blade. |
| r5 | `0fada95b6` | Round 5. |

## Results

| Oak | Age | Leaves | Warm wall, ms | GPU stamped sum, ms | GPU span, ms | Device peak, MB | Process peak RSS, MB | Under 0.5 s |
|---|---|---|---|---|---|---|---|---|
| r3 | 40 | 342,877 | 236 | 171 | 388 | 782 | 762 | yes |
| r4-blade | 40 | 47,248 | 247 | 173 | 347 | 782 | 608 | yes |
| r5 | 40 | 100,002 | 347 | 250 | 494 | 1,298 | 800 | yes |
| r3 | 80 | 1,754,337 | 1,130 | 735 | 1,137 | 4,209 | 696 | no |
| r4-blade | 80 | 254,766 | 1,119 | 732 | 1,118 | 4,209 | 700 | no |
| r5 | 80 | 415,305 | 2,044 | 1,367 | 2,126 | 4,191 | 965 | no |

The leaf count barely moves the build time; the structure sets it.
