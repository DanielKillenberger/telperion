# F4 merged into the birch (host merge, 2026-10-09)

The birch's branch at `713aa880` (round 14, on fn-223's clusters) took F4, `fn-198-tree-space-f-the-engine-in-production` at `6831fdb0`, so that the owner can check the birch in the harness. `scratch-fn-235-birch` was not merged. Raw output is in `raw/merge-f4/`.

## Conflicts

- **`suite/sweep.rs`:** git merged the `HELD` body three ways: F4's floret and `/supernatural` rows went in, and the birch's removal of fn-221's six old-bark rows, which the birch now states, was kept, along with its `/age` and supernatural switch. Only the header conflicted. It now carries both comments, and the count is 154 (F4's 158, less the six old-bark rows, plus the birch's two).
- **`tests/catalogue_identity.rs`:** the birch had re-pinned three digests without a header line, so the header is F4's, with one line added for the birch and this merge. The family, walk and override digests were read from the merged tree. The refusal digest is F4's and did not move.
- **`src/browser/presets.generated.ts`:** this was regenerated with `TELPERION_WRITE_REFERENCE=1 node scripts/build-wasm.mjs`, not merged by hand. Compared with F4's copy it carries the birch's round-14 rows, and `npm test` then reported the metadata current.

## Checks

- **`space_hash`** (`raw/merge-f4/hash-after.txt`): all 16 rows for beech, spruce, oak and palm at ages 40 and 80, seeds 1 and 7, equal MERGE-235.md's, node counts included.
- **The birch's DBH** (`birch_dbh.rs`, `raw/merge-f4/dbh-after.txt`) is 30.7 / 33.0 / 30.3 cm at 40 and 59.7 / 60.3 / 62.1 cm at 80 for seeds 1 / 7 / 42, which is SHEET-14's round 14. The node counts match too.

## Gate (`RAYON_NUM_THREADS=4 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4`)

- **Render half, under the GPU lock:** 214 passed, 1 failed and 39 ignored (`gate-render.log`). The one failure is the cold-build timing test, where `grow` was made cold in 22.7 s against 20 s. That run shared the CPU with the workspace half. Run alone under the lock (`gate-render-cold-rerun.log`), `grow` was inside 20 s, but the first tree took 5.04 s besides `grow` against its 5 s bound, while another process held a core. The merge leaves every shader and `generation/` file as F4 has it, and MERGE-235.md gave that bound 0.3 s of margin. The test is recorded as inconclusive because of machine load, not as a pass.
- **Workspace half** (`--exclude telperion-render`): 1,065 passed, 0 failed and 48 ignored (`gate-rest.log`).
- **`npm test`:** all 13 files and 143 tests pass, and the generated files were unchanged (`npm.log`).
