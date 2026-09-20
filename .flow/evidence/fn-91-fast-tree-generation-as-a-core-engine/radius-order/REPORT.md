# Radius-only ordering screen rejected

The single radius-only candidate missed the required 5% compact-preparation improvement on both oak seeds. The host reviewed the results and rejected retention. Production source has been restored exactly to `b822fcd10ab44e2d0f0b7688e9fa506b3b9f717f`; `candidate.patch` preserves the experiment. No browser matrix, new images or tuning run followed the rejection. The parent's 10x, 100 ms, cold-start, phone and complete peak-memory requirements remain open.

| Fixture | Compact baseline ms | Candidate ms | Change | CPU wood baseline ms | Candidate ms | Change |
|---|---:|---:|---:|---:|---:|---:|
| Oak 1 | 39.247 | 43.092 | +9.80% | 173.571 | 175.513 | +1.12% |
| Oak 7 | 51.608 | 50.748 | -1.67% | 198.085 | 201.254 | +1.60% |
| Spruce 1 | 25.448 | 25.479 | +0.12% | 139.012 | 138.461 | -0.40% |
| Spruce 7 | 22.276 | 22.222 | -0.24% | 129.782 | 131.603 | +1.40% |

Values are three-warm-sample medians from each fixture's first-plus-three run. `summary.json` also retains first-build values and warm ranges; `baseline.jsonl` and `candidate.jsonl` retain every sample and full parameters. Baseline and candidate ran in separate processes, paired in that order per fixture, serially with no concurrent compilation. The native release/LTO runner used the Ryzen 9 5950X Linux machine recorded in `hardware.txt` and the compiler recorded in `provenance.json`. Both modes used identical mature oak/spruce seeds 1/7, compact preparation with contacts and ordinary full CPU wood output.

Frequency, scheduling, allocator and cache state were uncontrolled. The compact warm ranges overlap on every fixture; three observations cannot isolate a causal slowdown or establish a latency distribution. The oak-1 observation still fails the screen, and oak-7's 1.67% improvement misses its target. Full wood observations remain visible rather than being rounded into a no-regression claim. The .9 measured 4.0-6.9 ms ordering-pass ceiling did not predict the achieved gain. This experiment does not identify why radius-only consumption failed to deliver the target.

## Exact output and scope

The candidate replaced the two ordering passes' full-sample buffers with an internal generic sample visitor. Full emission and radius reduction shared every existing formula. The radius consumer used the original zero-initialized, ordered `f64::max` reduction. Sorting, fork/socket/flare/burial math and output emission order were unchanged; the intended saving was compiler removal of unused positions and sample writes. The candidate added no allocation or live buffer.

Outside timing, the runner serialized every wood position, normal, coordinate, index, bound, run count, run-table field and dropped-triangle count, all compact rings, angular inputs, run records, run table and scalar metadata, and every optional node-to-contact range. Scalars use little-endian bit representations, including signed zero. A scratch-only accessor exposed private contact ranges; it never entered production. Four baseline/candidate binary comparisons passed, and all 24 warm repeats matched their own mode's first output byte-for-byte. `outputs.sha256` retains the output digests; the actual compared files remain at the scratch path. Every generated tree reported complete, with matching node/vertex/contact sizes and zero dropped triangles.

These comparisons are broader than .9's historical hash. They establish exactness for this mature matrix, not exhaustive parameter coverage. The candidate was rejected before the focused synthetic fork/flare/modulation checks, renderer checks or Wasm rebuild reserved for retained code. Existing .8 views remain applicable to unchanged production. Verification serialization and disk rereads allocate outside the measured intervals and affect later allocator/cache state identically by protocol; their process memory is not a production peak measurement.

## Reproduction and gates

`setup.py` archives the pinned base and builds both scratch binaries using `radius_order.rs` and the archived patch. `screen.sh` executes the fixed pair matrix and compares complete outputs; `analyze.py` derives the table. Reproduction overwrites evidence logs, so use a copied evidence directory if retaining the original observations. The originally preserved .9 diagnostic binary and the current .8 browser module are in the recorded scratch directory with SHA-256 provenance. Neither was rebuilt or used to claim a browser improvement. The runner's final formatting is whitespace-only after measurement.

Both scoped release diagnostic builds passed, the one screen exited zero, all complete comparisons passed, and the analysis assertions passed. Baseline is green via the task's .8 handoff (core 20/20, renderer 18/18, mature production 1/1, Wasm, TypeScript and browser lifecycle checks). Scoped Rust formatting passed before the candidate. The host explicitly waived additional builds/tests after exact restoration of all three production files. No production code or test remains changed.

The rollback safety hook caused about one minute of friction, recorded immediately in the spec's `FRICTION.md`. Archiving and inspecting the patch, then applying its reverse, completed a recoverable rollback. This is a local hook issue; no setup spec was created.

Host screen decision: reject, reviewed before completion. No implementation-review backend verdict is claimed.

Tier: session (jev intelligent 0.33; explicit IMPLEMENTER preserved).
stage: impl-review - skipped(config: REVIEW_MODE=none)
