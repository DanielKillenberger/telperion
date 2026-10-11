# fn-260 checks, measured (task 1, 2026-10-09 to 10)

Run under `cargo test --profile ci` on the owner's machine, `--test-threads 4`, compilation excluded (R3: under 60 s each).

| Test | Run time |
|---|---|
| `telperion-render shader_tests::every_shader_parses_and_validates` with `every_shader_file_is_checked` and `a_faulty_fixture_fails_the_check` | 0.11 s together |
| `telperion-core pipeline::space::ceiling_tests::every_shipped_preset_keeps_under_the_wasm_ceiling` | 5.16 s |

## R2: each preset at its own seed and age against `WASM_KEPT` (4,000,000)

| Preset | Seed | Age | Kept | Margin |
|---|---|---|---|---|
| ordinary | 42 | 40 | 163,214 | 95.9 % |
| oregon-white-oak | 42 | 40 | 354,122 | 91.1 % |
| norway-spruce | 42 | 40 | 1,212,283 | 69.7 % |
| european-beech | 42 | 40 | 332,405 | 91.7 % |
| silver-birch | 42 | 40 | 518,131 | 87.0 % |
| date-palm | 42 | 40 | 480 | 100.0 % |
| telperion | 42 | 82 | 3,773,761 | 5.7 % |
| laurelin | 42 | 80 | 3,534,309 | 11.6 % |

## Red proofs

These were temporary edits, each reverted before the commit.

- Telperion's `/age` set to 88, the fn-248 case: the test failed in 5.44 s. Telperion kept 4,540,863, a margin of -13.5 %, and the test reported `over the browser's ceiling: ["telperion"]`.
- `let floor = thick;` added to `dress.wgsl`, the fn-248 case: `every_shader_parses_and_validates` failed with `fine dress: error: local declaration cannot be called` at the `floor(...)` call. The four committed fixtures each fail, and each sound twin passes: a reserved word, a missing return, and locals shadowing `floor` and `length`.

## Gate

| Part | Result |
|---|---|
| Render, under the GPU lock | 138 lib tests and every integration test passed, in 8.5 min. One test failed: `every_pipeline_within_2_5_s_but_grow_within_20_s_cold`, where `grow` took 20.96 s against its 20 s bound (fn-237 R9). Run alone, it passed: the first tree built cold in 20.54 s, 5.23 s of it outside `grow`. |
| telperion-space | Green in 7.3 min. |
| telperion-field | Green in 0.9 min. |
| telperion-wasm | Green in 0.8 min. |
| telperion-core | Green in 5.5 min. |
| telperion-jev | Green on its rerun after core. Its replay test needs core's `species_measure` example (see FRICTION.md). |
| `npm test` | 14 files and 155 tests green, in 4.3 min. |
