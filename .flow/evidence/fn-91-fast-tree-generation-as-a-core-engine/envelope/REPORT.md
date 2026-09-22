> Historical raw-output references: see the [archive and recovery instructions](../README.md).

# Prepared envelope admission candidate rejected

The single native screen missed its required 20% gain on both oak seeds. Oak seeds 1/7 improved 24.22%/14.39%, while spruce regressed 4.03%/1.74%. The host explicitly authorized one delivery qualification because oak saved 16.222/10.669 ms. The threshold remains missed. The guarded candidate then improved browser completed-frame oak delivery 1.31%/3.92%, with spruce regressions of 2.98%/3.93%. The host rejected retention. The delivered gains do not justify the numerical filter complexity, observed spruce regressions and unresolved memory increase. Production source, tests and the generated browser module are restored exactly to .8 production; both candidate phases and all evidence remain archived.

## Native screen and exact output

| Fixture | Baseline warm ms | Candidate warm ms | Change |
|---|---:|---:|---:|
| Oak 1 | 66.968 | 50.746 | -24.22% |
| Oak 7 | 74.143 | 63.475 | -14.39% |
| Spruce 1 | 46.158 | 48.020 | +4.03% |
| Spruce 7 | 43.893 | 44.657 | +1.74% |

These are medians of three warm skeleton requests following one first request, with table construction included. Baseline then candidate ran serially for each fixture on the Ryzen 9 5950X Linux host described in `hardware.txt`. `baseline.jsonl`, `candidate.jsonl` and `summary.json` retain all samples, parameters, first values and ranges. Frequency, scheduling and cache state were uncontrolled. One paired screen ran, with no table-size tuning or timing repeats.

The bincode oracle serialized every field of `Tree` and `GrowthReport.shed` outside timing. All four outputs matched byte-for-byte, including floating bits, private state, stable identities and node order. All 24 warm repeats matched their respective first output; every tree was complete and passed solved-tree validation. `outputs.sha256` preserves the digests and `scratch-path.txt` locates the binaries. No CPU foliage expansion was repeated for this comparison.

The initial candidate is archived in `candidate.patch`. Host review then required two safeguards before delivery qualification. Only a mature full-drain advance (`!growing_envelope && budget == usize::MAX`) prepares tables; incremental advances keep the canonical predicate. Unsupported/tableless contexts immediately call that predicate, avoiding repeated rejection checks. `delivery.patch` identifies this guarded version. The native screen above belongs to the initial candidate, and is not relabelled as a measurement of the guarded version.

## Capability and numerical qualification

The prepared domain requires a valid axisymmetric envelope, height in [0.001, 1000], spread in [0.001, 10], crown base in [0, 0.99], fullness in [0.01, 0.99], and shoulder in [1.25, 8]. All other valid inputs retain the original predicate. Each table holds 257 radius endpoints over 256 uniform bins in the canonical normalized coordinate `p`. Query `t` and `p` use the exact original arithmetic. Multiplication by 256 is exact in this normal range, so endpoint bin selection introduces no approximation to the coordinate.

For the mathematical normalized shape `f(p) = (1-p^s)^(1/s)`, monotonic decrease bounds each bin by its endpoints. The code expands those radii outward by `max_radius * 1e-6`. It falls back at crown endpoints, the peak, nonfinite/nonnormal queries, uncertain radial comparisons and the entire final bin, p >= 255/256. It computes the original horizontal hypot rather than replacing the comparison with squared distance. The canonical predicate remains the fallback and the test oracle. Pendant-band admission still follows shell rejection; its search and the 40 clipping halvings remain unchanged.

This is empirical qualification of pinned libm 0.2.16, not a universal error proof. Excluding the final bin keeps `1-p^s >= 1/256` mathematically for s >= 1, and the outer root's derivative with respect to the inner power is at most 128 over the stated shoulder range. Thus an assumed inner absolute error of 1e-12 would amplify to at most roughly 1.28e-10 before ordinary subtraction/multiplication rounding, far below the 1e-6 padding. The assumption is deliberately loose relative to the observed errors; libm's source does not supply a formal bound establishing it for every admitted input. The guard and fallback therefore do not establish exhaustive binary64 equivalence.

`numeric_oracle.rs` checks the pinned pow directly at 8,192 endpoint-adjacent, extreme-shoulder, tiny-p and random cases. The independent Python Decimal oracle uses 60 digits, exact conversion of binary64 inputs, and a mathematical reciprocal. Maximum observed inner-power absolute error is 7.98e-17 and normalized-shape error 3.68e-16 (`numeric.csv`, `numeric-summary.json`). The focused admission tests compare 111,024 endpoint/peak/crown/radial-neighbor cases and 320,000 random queries over varied height, spread, fullness and shoulder, plus unsupported/extreme and exceptional inputs. These samples support the bounded implementation qualification and do not prove a universal bound.

## Context lifetime and query fractions

Actual births and planned runs receive separate copied configs with their own tables, prepared once per advance. The planned config preserves the authored shell override; the current config retains the live boundary. There is no serialized state, heap cache, species branch, RNG change or traversal change.

The separate scratch-only diagnostic reports two contexts per mature fixture. Each native context occupies 2,184 bytes, including its copied config and optional 257-entry array. The pair adds 4,368 stack bytes during local advance and no heap allocation. Oak has two prepared tables; spruce's shoulder-one contexts are unprepared. Untimed useful/fallback fractions are 78.85%/21.14% for oak 1 and 83.95%/16.05% for oak 7. The remainder is direct vertical rejection. Spruce uses the canonical path. Diagnostic output also matches the four original outputs. `diagnostic.py` injects counters only into scratch and never into timed executables or production.

The guarded version still reserves fixed context storage for incremental calls, but avoids table evaluation there. It makes no incremental speed claim. The existing irregular-budget and whole-build replay tests pass.

## Delivery qualification

| Browser fixture | Fresh .8 baseline ms | Guarded candidate ms | Change | Original speedup |
|---|---:|---:|---:|---:|
| Oak 1 | 175.4 | 173.1 | -1.31% | 9.0699x |
| Oak 7 | 193.8 | 186.2 | -3.92% | 10.0027x |
| Spruce 1 | 184.6 | 190.1 | +2.98% | 40.1294x |
| Spruce 7 | 173.0 | 179.8 | +3.93% | 41.0679x |

Each row uses one first request plus five warm completed-frame requests. The fresh baseline module hash is the accepted .8 hash, verified before rebuilding. Both modules used GPU positions with no position or wood fallback in all 24 samples each. `browser-baseline.json` and `browser-candidate.json` preserve runtime flags, viewport, adapter, initialization, first requests, samples and Wasm capacity. First-request and initialization observations remain separate from warm results and do not qualify full cold startup or phone delivery.

Oak skeleton stage medians fell 73.4 to 61.7 ms and 77.4 to 60.6 ms. Its position preparation rose 33.5 to 41.4 ms and 42.2 to 45.9 ms; wood stage medians rose 53.8 to 61.5 ms and 64.4 to 69.0 ms. Spruce skeleton medians changed 55.1 to 56.2 ms and 51.9 to 51.4 ms, while position preparation rose 27.3 to 29.4 ms and 26.7 to 28.0 ms. These stage medians are not additive and do not establish a cause for the differences in unchanged stages. All regressions remain observations; no retiming was used to replace them.

| Native owned-output control, seed 1 | Baseline ms | Candidate ms | Change |
|---|---:|---:|---:|
| CPU oak | 589.447 | 568.572 | -3.54% |
| CPU spruce | 4611.853 | 4573.256 | -0.84% |
| GPU-assisted CPU oak | 277.717 | 259.741 | -6.47% |
| GPU-assisted CPU spruce | 306.731 | 314.049 | +2.39% |

Controls used first plus three warm requests, serial baseline/candidate processes and wait4 RSS observations. The GPU-assisted spruce regression remains unresolved. Native GPU initialization also varied, including oak 253.6 to 342.9 ms. Warm gains cannot be used as cold-start qualification. `delivery-summary.json`, `*-rss.json` and raw control logs retain initialization, ranges and process RSS.

## Memory and remaining bounds

| Fixture | Baseline/candidate renderer joint capacity envelope bytes | Baseline/candidate Wasm high-water bytes |
|---|---:|---:|
| Oak 1 | 511,842,716 | 94,240,768 |
| Oak 7 | 609,104,276 | 106,496,000 |
| Spruce 1 | 736,449,204 | 98,041,856 |
| Spruce 7 | 702,912,352 | 98,041,856 |

Each listed renderer envelope and Wasm high-water value is unchanged. `summarize-delivery.py` applies the accepted .8 GPU-position ownership formulas to both revisions. Preparation, foliage and final expansion include the previous live tree and their relevant CPU/GPU capacity snapshots. The new pair of admission contexts is a separate 4,368-byte native stack addition during skeleton growth and drops before those renderer phases. Native CPU-output RSS changed 256,660 to 254,508 KiB for oak and 399,924 to 401,540 KiB for spruce. GPU-assisted owned-output RSS rose 444,936 to 486,760 KiB for oak (+41,824 KiB, 9.4%) and fell 496,392 to 493,484 KiB for spruce. The oak increase remains unexplained and was not remeasured; the 4,368-byte context pair cannot explain its magnitude. Fixed context storage, growth frontier overlap, allocation overhead, driver resources, upload staging and deferred destruction prevent a complete peak-memory claim from these counters. Unchanged later-phase envelopes do not prove a whole-process peak bound.

The candidate's oak 7 median is only 0.05 ms below its 186.25 ms ten-times target; the small sample does not establish a robust 10x result. Oak 1 remains about 16.1 ms above its target. Every candidate fixture exceeds 100 ms. Parent CPU-output, memory, cold-start and phone requirements remain open, after rejection. Full output equality allows reuse of accepted .8 visual evidence without new captures.

## Verification and reproduction

Baseline was green via the .8 handoff, with .9 evidence-only and .10/.11 reverted. The initial preparation test failed before implementation at the missing-table assertion. Sixteen focused release tests then passed. The batch-guard test subsequently failed before the guard at its no-table assertion, and all 17 focused release tests passed after the guard. Required local, pendant, actual/planned, unsupported-input and incremental behaviors are included. The guarded Wasm build passed. The guarded final version also matched all four original complete Tree/shed files in a separate untimed check (`final-output-check.log`). Scoped Rust formatting, diff whitespace, shell syntax and Python syntax checks passed.

Run `setup.py`, `screen.sh`, `analyze.py`, `numeric_oracle.py` and `diagnostic.py` from this evidence directory by full path with the repository as the working directory to reproduce the original screen. Scripts overwrite logs. `candidate.patch` and `delivery.patch` deliberately represent different phases. `final-output-check.py` applies the guarded patch to the recorded scratch baseline and checks the four original full outputs without collecting timing. `measure.py` names preserved native executables; `delivery-provenance.json` records module/executable hashes and browser commands. The numeric example's private-trait compile failure and correction are recorded in FRICTION.md. A scoped restore was blocked by the command guard; a checked inverse of the archived delivery patch restored the exact candidate instead. No candidate code or counters ship. The host reviewed delivery, correctness and memory evidence and authorized rejection before Flow completion. The retained-only renderer gate was therefore skipped; production restoration is exact.

GATE_SKIPPED:unittest:docs-only - cumulative diff classified tier-B (no executable paths touched)

Tier: session (jev intelligent 0.78; explicit IMPLEMENTER preserved).
stage: impl-review - skipped(config: REVIEW_MODE=none)
