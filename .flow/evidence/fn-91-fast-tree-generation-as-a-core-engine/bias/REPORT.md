# Inactive growth-bias screen rejected

The single no-active-effect specialization missed the required 10% skeleton gain on both oak seeds. Production `bias.rs` is restored exactly to the pre-task commit; the candidate and its oracle test remain in `candidate.patch`. No tuning, CPU foliage expansion, browser rebuild or new visual captures followed the failed screen. This rejected experiment does not deliver the parent's fast-generation requirements.

| Fixture | Baseline warm ms | Candidate warm ms | Change |
|---|---:|---:|---:|
| Oak 1 | 62.632 | 60.738 | -3.02% |
| Oak 7 | 72.266 | 72.608 | +0.47% |
| Spruce 1 | 44.847 | 44.031 | -1.82% |
| Spruce 7 | 41.332 | 41.505 | +0.42% |

Each value is the median of three warm skeleton requests following one first request. Raw samples and complete parameters are in `baseline.jsonl` and `candidate.jsonl`; `summary.json` includes first values and warm ranges. Modes ran in separate processes, baseline then candidate for each fixture, with builds and timings serialized. The native release/LTO run used the Ryzen 9 5950X Linux host in `hardware.txt`; `provenance.json` records the compiler, pinned source, scratch location, build command and preserved .8 browser module SHA-256. Frequency, scheduling and cache state were uncontrolled. The small sample does not establish a latency distribution or prove causation for the observed sub-percent regressions. It does unambiguously miss the declared screen target.

## Change and correctness

The generic shortcut required zero lean and gravity plus either disabled supernatural effects or zero writhe amplitude. It skipped stray-distance and height calculations without adding allocation or persistent state. The remaining zero additions retained the original lean and gravity signs, and the original tiny-vector fallback and normalization remained. Enabled nonzero amplitude, even with zero maximum magnitude, kept the general path; disabled dormant fields were ignored as before. No traversal, RNG, trig, clipping or surface code changed.

A frozen copy of the original general implementation served as the test oracle. The single table test covered active and inactive effects, enabled zero amplitude, dormant authored fields, zero maximum magnitude, signed-zero parameters, zero/tiny/nonunit directions, two seeds and three positions. The deliberately naïve normalization shortcut failed on signed-zero components (`oracle-red.log`); preserving the additions and fallback passed. Eight focused release tests passed (`focused.log`), including existing bias contracts, disabled-effect independence, preset bias propagation and whole-build/local/scaffold retained replay. This supports the tested cases, not exhaustive finite-input equivalence.

Outside timing, bincode 1.3.3 serialized the entire derived `Tree` and `GrowthReport.shed`: all nodes in order, stable identities including slot keys, shoot state and private width/vigour data, floating-point bits including signed zero, parent/run/kind/stem fields, radii, crossover and diagnostics. All four baseline/candidate files matched byte-for-byte, and all 24 warm repeats matched their own first output. Every tree was complete and passed solved-tree validation. `outputs.sha256` retains digests; compared binaries remain in the recorded scratch directory. Verification allocations and file reads are outside timing but influence subsequent process state; this is not a peak-memory measurement.

## Reproduction and gates

Run `python3 setup.py` from the repository root using the script's full evidence path, then `bash screen.sh` and `python3 analyze.py` likewise. Setup pins the pre-task revision recorded as `base` in `provenance.json`. Reproduction overwrites logs, so preserve the evidence directory before running it. Setup archives source into scratch, preserves the current browser package, builds baseline, applies the archived candidate patch and builds candidate. The runner makes only skeleton requests and serializes results outside their measured intervals. All builds, the screen, output comparisons and analysis passed. Scoped Rust formatting, shell syntax and diff checks passed.

Baseline: green via .8 handoff (core20/20, renderer18/18, mature production1/1, Wasm, TypeScript and browser lifecycle), with .9 evidence-only and .10 restored. Final production restoration is exact. The Flow classifier reports docs-only, so no repeat test suite is needed after rollback.

GATE_SKIPPED:unittest:docs-only - cumulative diff classified tier-B (no executable paths touched)

Parent delivery remains at .8 browser medians: oak179.1/203.7ms (8.77/9.14x original; 22.1/17.5ms above its10x targets), spruce194.6/178.9ms (39.20/41.27x). All exceed100ms. No new memory claim is made; allocator/driver/staging/deferred-destruction peaks, cold startup and phone qualification remain open. Existing .8 visual evidence applies to unchanged production.

Tier: session (jev moderate 0.79; explicit IMPLEMENTER preserved).
stage: impl-review - skipped(config: REVIEW_MODE=none)

The host reviewed the native screen and approved rejection before Flow completion. No implementation-review backend verdict is claimed.
