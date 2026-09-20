# Preliminary native process memory

Linux wait4 reports each fresh child process peak resident set. Each process makes two sequential native CPU builds. There is no renderer or GPU device in these measurements. Binary SHA-256 identifiers and raw values are in native-peak-rss.json; rss-*.jsonl retain the specimen records.

| Fixture | Original peak (KiB) | CPU candidate peak (KiB) | Difference |
|---|---:|---:|---:|
| oregon-white-oak / 1 | 253932 | 256644 | +1.07% |
| oregon-white-oak / 7 | 295864 | 295800 | -0.02% |
| norway-spruce / 1 | 402068 | 402960 | +0.22% |
| norway-spruce / 7 | 370148 | 371280 | +0.31% |

These observations do not establish the no-increase bound. The original stage example predates the candidate fingerprint scan, so process-level instrumentation is not fully matched, and each fixture has only one process observation. The small rises are not established generator regressions either. The production edits add stack values and reuse the existing frame vector; unchanged heap allocation shape alone cannot qualify whole-process peak memory.

A qualifying CPU/GPU comparison must use matched instrumentation, include required readback and device/driver costs in relevant process measurements, and separately report explicit GPU buffer peaks. Browser CPU Wasm capacity remained identical in the earlier matched comparison, but that capacity is not a measurement of whole browser or GPU peak memory.
