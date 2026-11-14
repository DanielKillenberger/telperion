# R1: the field makes only the programs it dispatches (task .2)

28 Oct 2026, native, RTX 3080, `--profile ci`. Base `ecd5902a4` (task 1's
integrated head) plus the measurement test alone (`015a818e3`), so the base
binary prepares the field the way `web/field.rs` prepared it before.

## What a field no longer makes

- `build::Tools` is split. A field's tools are the hanger, dresser,
  stationer and fielder. A build's `Drawing` is the sweeper (11
  programs) and the ringer (1). `Kit` holds the drawing in a cell that only
  a build, or the renderer's `prepare_programs`, fills.
- `Generator::prepare_field_programs` makes the kit without the drawing
  and without the wood's 11 surfacing passes (`curve::passes`).
  `web/field.rs` calls it. The renderer's `prepare_programs` and
  `preparePrograms` still make all of them.
- Test: `field_tests::a_field_generator_holds_no_curve_pass_sweeper_or_ringer`
  prepares Telperion, grows its field, then asserts that `gpu.curve` and
  the kit's drawing are both empty.
- Compile guarantee: `field::grow` takes `&Tools`. `readied` and `placed`
  take `(&Tools, &Drawing)`. A scratch call `readied(gpu, tools, ...)` from
  `field::grow` fails with E0308 `expected (&Tools, &Drawing), found
  &Tools`, and was reverted.

The field still makes three things it does not dispatch. These are
follow-ups, because the fix lies outside this task's Touches:

- `Generator::create` still makes `mass` and `positions`. The field calls
  neither `record_mass` nor `ring_vertices`. Dropping them changes
  `generation/compute.rs` and `positions.rs`.
- `tree::Kernels.present` is still made. A one-shot `tree::drawn` never
  reads it; only `cycle/growth.rs` does. Dropping it changes
  `cycle/tree.rs`.

## Answers

`field_answers` was run on all eight presets at 24, 36 and 48. All 24
hashes equal FLORETS.md's.

One open item: in 3 of 14 all-preset runs of the final binary,
`silver-birch 36` hashed `1a62b934cb578ef1` instead of `9ddc6db2e5c068f6`.
One of those 3 runs also differed at 48. The base binary ran 12 times with
no mismatch, and the measured binary 5 times with none. Birch alone ran 9
times, 3 on each binary, all equal. All three mismatches came between
12:47 and 12:52. The 15 runs after that, interleaved with the base, were
all equal. The field code path is the same in the measured and final
binaries; only call-site syntax differs. The likely cause is latent
run-order or timing nondeterminism in the device field, exposed by a
different device allocation pattern. This is inferred, not proven. Logs:
`raw/` was not kept; the counts are from `/tmp/fn319/t2-allpresets*.log`.

## Paired cold measurement (`field_time_tests::field_cold`, ms)

Each run is one process. The driver's shader disk cache was off
(`__GL_SHADER_DISK_CACHE=0`). Runs alternated base and new, three runs each.
"Prepare" is `for_fields_async` plus the field's preparation; the device
request is not included. "First" is the first `grow_field` at 36 cells,
seed 1.

| Preset | Stage | base runs | new runs | median base | median new |
|---|---|---|---|--:|--:|
| oregon-white-oak | prepare | 5754, 5765, 5772 | 5656, 5880, 6244 | 5765 | 5880 |
| oregon-white-oak | first | 563, 536, 511 | 465, 426, 462 | 536 | 462 |
| telperion | prepare | 6076, 5660, 10176 | 5994, 5726, 8683 | 6076 | 5994 |
| telperion | first | 1202, 1207, 1630 | 1101, 1172, 1222 | 1207 | 1172 |

- Native preparation is unchanged within its spread. Natively `io::pair`
  makes the modules on parallel threads, so the time is the slowest
  module's, most likely the cycle's kernels (inferred). The 23 programs
  that are no longer made were off the critical path.
- The first field is about 35 to 75 ms faster. This was measured, but the
  cause is not known.
- In a browser every program is compiled by the GPU process. The 23 fewer
  programs are work it no longer does. That gain was not measured here,
  because WebGPU in a browser is out of bounds for workers.

## The birch hash, re-run by the host (28 Oct 2026)

The two differing `silver-birch 36` hashes (`1a62b934cb578ef1`, `274204c16d2a62bd`) both came from the loop that ran `target/ci/deps/telperion_render-e638275db5c659d6` at 12:50, while the worker was still rebuilding that crate. Afterwards, with nothing building: the base binary 15 runs over every preset and the final binary 15 runs, every hash equal to task 1's (`/tmp/fn319/t2-base15.log`, `/tmp/fn319/t2-final15.log`), and the worker's copied final binary 22 more birch rows equal. Not reproduced in 30 clean runs on either binary; the cause of the two is unknown. fn-318's byte equality (R3) is the contract that would catch it again.
