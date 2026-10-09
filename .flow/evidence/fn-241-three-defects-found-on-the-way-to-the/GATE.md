# fn-241 R4: the gate, npm test, shipped species

Run at `661e72e4`, with `RAYON_NUM_THREADS=4 CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4`. Raw output is in `raw/`.

- **Render half,** `cargo test --profile ci -p telperion-render --no-fail-fast`, under the GPU lock and `timeout 3000`: 216 passed, 0 failed, 41 ignored (`gate-render.log`).
  - The first run failed on the R1 bounds test, which ran the spruce at 80 out of device memory beside the other tests (FRICTION.md).
  - Its trees at 80 are now a device-limits probe. That probe passes alone under the lock (`r1-at-80.log`), and the rerun is the result above.
- **Workspace half,** `cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast`: 1,065 passed, 0 failed, 50 ignored (`gate-rest.log`).
- **`npm test`:** 13 files and 150 tests passed (`npm.log`), and the generated files were unchanged.
- **Shipped species byte-identical.** `space_hash` (`raw/hash-after.txt`) gives all 16 rows: beech, spruce, oak and palm at 40 and 80, seeds 1 and 7.
  - The 12 beech, spruce and palm rows equal MERGE-235.md's, node counts included.
  - The 4 oak rows equal MERGE-SPECIES.md's round-3 hashes.
  - The device path's outputs are held by the chain, realized, mirror and florets parity tests in the render half.
