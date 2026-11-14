# fn-91 retained evidence

The decision record is the spec, `.flow/specs/fn-91-fast-tree-generation-as-a-core-engine.md`. Benchmark reproduction is documented in [generation.md](../../../scripts/benchmarks/generation.md).

Kept here are the five reports the benchmark doc and the README cite: [READBACK.md](READBACK.md), [resident-wood/REPORT.md](resident-wood/REPORT.md), [shared-rings/REPORT.md](shared-rings/REPORT.md), [position-integration/REPORT.md](position-integration/REPORT.md) and [stations/REPORT.md](stations/REPORT.md). The raw measurements, replay scripts, patches, screenshots and the other task reports they name were archived in two passes:

- The first pass (raw logs, measurement lines and screenshots) is recoverable at Git revision `c4871093bd7ef208c2c3f65fb46a223c4e397a0b`, e.g. `git worktree add --detach /tmp/telperion-fn91-replay c4871093bd7ef208c2c3f65fb46a223c4e397a0b`.
- The second pass (the remaining summaries, scripts, patches and reports) is listed with checksums and the source revision in `.flow/evidence/ARCHIVE-MANIFEST.md`; recover any file with `git show <rev>:<path>`.

A file named in a kept report but absent here is in one of those archives. Hardware timings need comparable hardware to reproduce.
