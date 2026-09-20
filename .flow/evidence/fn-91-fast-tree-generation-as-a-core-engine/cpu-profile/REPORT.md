> Historical raw-output references: see the [archive and recovery instructions](../README.md).

# Shared CPU preparation attribution

The next bounded experiment is a radius-only compact ordering pass. The current pass costs 5.61/6.85 ms on native oak seeds 1/7 and 4.19/4.04 ms on spruce. It constructs full samples merely to find their maximum radius, then constructs them again for emission. Share the radius calculation with emission and avoid the first pass's centre/attachment-direction work and sample writes. Those measured times are ceilings; computing radii still costs work, so achievable savings are smaller and unmeasured. A cheap exact-output screen should reject the experiment if the gain is negligible.

This task changes no production code. It narrows the next decision without completing the parent's 10×, 100 ms, cold-start, phone or full-memory requirements. Existing browser oak delivery remains 179.1/203.7 ms, 22.1/17.5 ms over its 10× bounds. Native component times do not predict browser savings.

## Measurement and invariants

`provenance.json` records revision, compiler and machine. An archived checkout of `0ade56601bb4d7dfc55952a73be4d7f36d3f2eeb` ran a release/LTO core example on Ryzen 9 5950X Linux. No renderer or GPU device participated. Each unchanged mature oak/spruce seed 1/7 fixture ran in its own process with first plus three warm builds. Control then instrumented modes ran serially, with no concurrent builds. `control.jsonl` and `instrumented.jsonl` contain every parameter, sample, output count and hash; `summary.json` contains derived results. OS scheduling, frequency and cache state were uncontrolled. Three warm observations are descriptive, not tail or significance evidence.

The diagnostic times skeleton generation, compact preparation with contact ranges, and compact foliage station preparation. It then generates the canonical CPU wood/foliage arrays outside those intervals to verify the historical `generation_stages` hash. This extra leaf expansion is correctness work, not attributed shared preparation or GPU delivery. All 32 primary CPU hashes match their four historical `cpu-candidate-<fixture>.jsonl` hashes. All parameters, node/leaf/wood counts, zero dropped triangles and completion flags match between modes; compact ring hashes match every repeat and follow-up. The historical hash covers positions, indices, packed leaves, element positions/indices and foliage bounds, not every tree identity or surface field. No semantic edit is claimed or qualified here.

| Fixture | Control skeleton ms | Compact/contact ms | Stations ms | Shared total ms | Instrumented shared delta |
|---|---:|---:|---:|---:|---:|
| Oak 1 | 61.808 | 38.362 | 22.256 | 123.313 | -0.75% |
| Oak 7 | 69.758 | 55.720 | 25.090 | 150.552 | +0.62% |
| Spruce 1 | 39.424 | 25.393 | 27.754 | 93.214 | -0.11% |
| Spruce 7 | 37.709 | 22.849 | 26.039 | 88.386 | +3.62% |

These are independent warm medians. The total is the median of each sample's stage sum, so it need not equal the sum of medians. The observed delta combines instrumentation and run-order noise; negative values do not mean profiling speeds generation. Spruce 7 has the largest perturbation, including +7.47% compact and +5.91% skeleton, retained in the raw results. The largest positive shared delta is below the prior fine-timer experiment's 6.5–21.3%, but this small fixed-order sample cannot isolate pure timer cost.

## Coarse attribution

Instrumented warm medians in milliseconds follow. Raw event order preserves the two growth-step calls; the summary sums repeated labels per sample. Radius and local validation are nested in their parent intervals. Compact rows and station rows partition their respective outer requests, apart from small timer/call/drop gaps. Independent medians need not sum to the parent median.

| Phase | Oak 1 | Oak 7 | Spruce 1 | Spruce 7 |
|---|---:|---:|---:|---:|
| Scaffold advance | 3.762 | 4.688 | 9.351 | 8.868 |
| Local seeding | 0.919 | 0.841 | 6.367 | 6.160 |
| Local advance including validation | 50.563 | 56.165 | 18.947 | 19.097 |
| Local validation, nested | 1.234 | 1.554 | 0.740 | 0.841 |
| Two early radius solves | 0.191 | 0.248 | 1.111 | 1.080 |
| Final radius solve | 2.003 | 3.004 | 1.605 | 1.655 |
| Final remap/identify | 1.685 | 1.609 | 1.054 | 1.056 |
| Compact paths | 2.489 | 6.714 | 1.449 | 1.214 |
| Compact order-key sampling | 5.614 | 6.852 | 4.186 | 4.040 |
| Compact sort | 1.748 | 2.149 | 1.285 | 1.235 |
| Compact second sampling/frames/packing/contact ranges | 26.634 | 32.988 | 16.365 | 14.847 |
| Station validation/contact setup | 2.174 | 2.334 | 1.217 | 1.058 |
| Station runs/frames/descriptors | 19.239 | 23.189 | 27.001 | 25.701 |

`summary.json` also retains creation, step remapping/identity, allocation, tips, and separate radius validation/body costs. The radius body exclusive of validation costs only 0.582/0.850/1.365/1.302 ms across all three solves. Do not add nested validation or the raw parent step/finish intervals to their children. Splitting frames/contact work further would require timers per path, so those costs remain combined.

Both fixtures have zero shedding threshold. Final shedding is about a microsecond; final identity refresh still visits the completed tree. No separate change-record construction occurs on mature `Specimen::grow`; birth/link bookkeeping is included in identify. The first two radius calls solve a small structural tree; the final call validates the whole tree and fills local start radii. Their repetition does not establish redundancy, and identity maps/incremental builders have real contracts.

The .8 browser skeleton medians are 73.3/81.7/57.9/53.4 ms, position preparation 36.3/43.6/31.0/25.7 ms and descriptors 20.3/25.3/31.0/28.4 ms, in table order. Native oak-7 compact time is higher; these runtime differences prohibit transferring native percentages to browser delivery.

## One sampled follow-up

The primary pass identifies local advance as oak's largest measured CPU phase. The single follow-up samples whole `Planner::run` calls by a mixed axis-key predicate at approximately 1/64 probability. Selected calls time setup, the full tracing loop, its nested clipping block, and tail/resampling. No timer runs per internode. Raw `followup-{coarse,sampled}.jsonl` retain first plus three warm observations; `followup-summary.json` gives exact selected counts and estimates. Both modes use the same shared preparation, with previously completed CPU-output verification omitted.

| Fixture | Planner calls | Selected calls | Selected clips | Estimated planner total ms | Nested clipping ms | Sampled/coarse skeleton delta |
|---|---:|---:|---:|---:|---:|---:|
| Oak 1 | 11,738 | 153 | 12 | 23.140 | 4.754 | -0.45% |
| Oak 7 | 13,693 | 202 | 13 | 22.604 | 4.524 | -7.75% |
| Spruce 1 | 16,292 | 245 | 4 | 5.595 | 0.386 | -1.05% |
| Spruce 7 | 15,664 | 231 | 3 | 4.894 | 0.299 | -2.01% |

Each estimate scales sampled duration by total calls / selected calls. The deterministic key subset repeats per fixture; warm repetitions do not add independent axis samples. In particular, 3–13 selected clips leave substantial sampling uncertainty and no confidence interval is claimed. Sampling adds conditional branches, one per-call counter and sparse timer/event work. Fixed mode order and the oak-7 -7.75% delta expose noise; these are observed overhead comparisons, not optimization gains. No further run was used to improve them.

Oak's sampled trace is about 21.3–21.9 ms including clipping. The source performs heading/turn limiting, envelope admission and run storage there; their individual costs are unresolved. The clipping loop's 40 containment tests account for only an estimated 4.5–4.8 ms of local growth. A spatial stopping tolerance would change output and needs botanical/geometry/visual qualification, while this measured ceiling cannot close the oak gap. Keep that semantic change behind an exact-output experiment.

## Ranked next decision and limits

1. **Radius-only compact order key.** Reuse the same radius law and floating-point evaluation order as `sample_path`; do not create a second independent formula. Compute the same maximum without materializing position/distance samples for the sorting pass. Preserve trunk burial, flare, stem-fork girth easing, branch socket containment, swell, maximum reduction and stable ordering ties. Keep one longest-path scratch vector for emission. No all-path sample cache or additional tree/mesh representation is justified. Task .10 owns the screen: all four native fixtures, exact compact rings, run tables, contacts and canonical outputs, and at least 5% compact improvement on each oak before a browser matrix. The whole first-pass ceiling is 4.0–6.9 ms native; no guaranteed or browser saving is claimed. Repeating the radius math may leave most of that cost intact, which is a valid rejection result.
2. **Local planned-run kernel.** The larger opportunity is approximately 23 ms sampled oak planner work inside 51–56 ms total local advance. A subsequent bounded experiment could share invariant envelope/heading calculations without changing traversal or random draws. Which calculation dominates is still unknown; these data cannot rank heading against admission. Preserve annual/incremental equivalence, stable identities, local taper, branch reach and finite geometry. Reusing scalar constants adds little ownership cost; materializing all planned runs would extend memory lifetimes and needs separate accounting.
3. **Repeated radius/identity elimination or clipping tolerance.** Early radius solves have too little oak potential to lead. Identity/remap costs are only a few milliseconds and serve retained-builder contracts. Clipping offers an estimated 4.5–4.8 ms oak ceiling with changed-output risk. Neither is the next implementation decision.

No GPU readback proposal follows from this profile. Reading a full GPU mesh into CPU-owned arrays adds transfer and synchronization and may increase peak allocations. It requires explicit lifetime design and measurement; this profile does not measure or reject such a candidate. The diagnostic records contact capacity and output sizes but does not establish peak process/device memory. Since production is unchanged, it makes no new claim of memory improvement or full-memory qualification.

## Reproduction and checks

Run `python3 <evidence>/setup.py`, `bash <evidence>/run.sh`, then `python3 <evidence>/analyze.py` from the repository root. The one optional follow-up is `bash <evidence>/followup.sh` followed by `python3 <evidence>/analyze-followup.py`. The scripts archive the pinned base, build only the core diagnostic, and keep all instrumentation in scratch. `diagnostic.patch` preserves the final sampled scratch variant; `instrument.py` preserves the coarse variant. Binary SHA-256 files and build logs accompany each mode. Rust formatting changes in the retained example are whitespace only after measurement.

Baseline is green via .8 handoff. Four release diagnostic builds and both analysis assertions pass. All 32 canonical checks, 64 primary/follow-up compact repeats and matching counts pass; scoped retained-Rust formatting passes. No broad suite, image capture, phone gate or GPU matrix was run. The host reviewed the perf-tooling friction as a local setup issue and approved the bounded sampler; no repository setup spec is warranted.

Tier: session (jev intelligent 0.82; explicit IMPLEMENTER preserved).
stage: impl-review - skipped(config: REVIEW_MODE=none)

Host direct retention review: YES after checking primary/follow-up measurements, evidence-only scope and the corrections above. No implementation-review backend verdict is claimed.
