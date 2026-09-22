> Historical raw-output references: see the [archive and recovery instructions](../README.md).

# Native CPU-owned surface expansion

Native Linux x86_64 consumers now receive the same wood mesh through at most eight bounded workers. The single full-wood screen improved both oak medians by more than 2.57x and passed its 2x admission target. The 6x aspiration was missed. All four full-field baseline/candidate comparisons and 24 warm repeat checks matched bitwise, including positions, normals, coordinates, indices, bounds, dropped counts, sorted run tables and unaffected compact/contact records. Every candidate screen had four parallel attempts and zero fallbacks.

The host approved the capacity design before implementation, admitted this one screen to complete-output qualification, and approved retention contingent on the final capacity snapshot and gates. The measured shared CPU surface construction serves both ordinary native CPU output and GPU-assisted CPU-owned output. Wasm, other native platforms, small/single-core requests and PreparedSurface/contact preparation retain serial construction. Browser GPU-resident timing was not repeated for this native change.

## One full-wood screen

| Fixture | Serial warm ms | Parallel warm ms | Speedup |
| --- | ---: | ---: | ---: |
| oregon-white-oak 1 | 175.862 | 67.883 | 2.591x |
| oregon-white-oak 7 | 201.307 | 78.118 | 2.577x |
| norway-spruce 1 | 141.479 | 54.916 | 2.576x |
| norway-spruce 7 | 133.690 | 49.442 | 2.704x |

Each process built a first sample and three warm samples. Baseline and candidate alternate per fixture. Serialization and comparison occur outside the wood timer. The scratch-only candidate counter adds one relaxed atomic increment per successful build; it reports attempts/fallbacks outside the timed call. Raw timings, metadata, first observations, exact hashes and source/binary hashes are retained beside this report. Eight workers were selected once, with no tuning round.

## Complete requested CPU output

| Path | Fixture | Baseline warm ms | Candidate warm ms | Gain | Versus original CPU baseline | RSS delta KiB |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| cpu-output | oregon-white-oak 1 | 596.27 | 476.51 | 20.09% | 1.98x | +880 |
| cpu-output | oregon-white-oak 7 | 711.54 | 644.75 | 9.39% | 1.76x | +248 |
| cpu-output | norway-spruce 1 | 4635.14 | 4516.35 | 2.56% | 1.24x | +1,908 |
| cpu-output | norway-spruce 7 | 4431.16 | 4308.82 | 2.76% | 1.24x | +2,468 |
| gpu-output | oregon-white-oak 1 | 269.76 | 156.88 | 41.84% | 6.01x | -632 |
| gpu-output | oregon-white-oak 7 | 306.90 | 182.10 | 40.66% | 6.24x | +224 |
| gpu-output | norway-spruce 1 | 309.96 | 223.38 | 27.93% | 24.99x | +296 |
| gpu-output | norway-spruce 7 | 290.90 | 206.44 | 29.03% | 25.86x | +908 |

These are actual synchronous owned-geometry delivery times, including allocation and joins. CPU-output uses mesh::build; GPU-output uses Generator::for_cpu_output and still builds ordinary CPU wood. Both are compared with the original native CPU-owned baselines 942.79/1136.90/5581.80/5338.82 ms. The GPU-assisted ratios do not describe pure CPU speed or browser rendering. Every case remains above 100 ms. Pure CPU delivery on all four fixtures and GPU-assisted oak remain below 10x. GPU-assisted spruce exceeds 10x against the original CPU-owned baseline; this is a change of backend for the same requested representation, not a pure CPU speedup. Pure CPU spruce remains dominated by other work.

Oak seed7 pure CPU candidate warm observations span 583.71–647.95 ms, with median 644.75 ms. This three-warm sample does not establish tails or significance. Initialization is separate in delivery-summary.json, as are first samples, ranges, stage medians and process RSS. GPU-assisted initialization spans 192.20–401.87 ms across this pair and is not included in warm delivery. The example gained GENERATION_SEED solely to identify and run seeds1/7; its default remains1.

Pure CPU maximum RSS increased by 880/248/1908/2468 KiB. GPU-assisted RSS changed by -632/+224/+296/+908 KiB. These observations remain increases where positive. They are not explained away by the lower accounted capacity envelope, and they do not establish unchanged whole-process memory. Driver, allocator/TLS, retired stack cache and OS residency behavior remain outside the explicit allocation model.

## Capacity and fallback

DESIGN.md records the host-approved phase/lifetime design and conservative serial capacity floor. capacity.log measures actual baseline capacities, and candidate-capacity.log independently confirms actual final/scratch-input/run-descriptor capacities for both candidate phases. Their explicit phase-B envelopes are 210,772,016 / 240,860,592 / 173,014,688 / 163,915,136 bytes, including 16 KiB fixed control and both worker generations. They are below serial floors by 3,073,647 / 3,565,952 / 1,669,885 / 1,473,511 bytes. Phase A is lower still.

Each worker requests a 64 KiB nonrecursive stack. The measured host reports 4096-byte pages and 16384-byte pthread minimum stack; the envelope includes a 4 KiB guard plus 64 KiB opaque runtime allowance per worker, and charges both generations to conservatively cover retired reservations. These allowances bound the declared accounting model, not every implementation of pthread/TLS or allocator retention. Admission checks actual preparation capacities, descriptor sizes, both phase formulas and one captured worker count before execution.

Workers own disjoint final slices. Phase A shares canonical samples, frames and emission math. After it joins, path, distance, angular and per-worker scratch storage drop before normal/index allocation. Phase B retains canonical triangle order, float64 cross products and float32 writes. A creation failure, worker error, collapsed triangle or zero normal joins all started workers and discards candidate arrays before one threading-disabled serial retry. The candidate never overlaps a second complete mesh. Joined OS stack caches and allocator retention remain part of the named whole-process gap. Unsupported cases may retry once; the four qualification fixtures did not fall back in the screen.

## Verification and provenance

The baseline is e453ac51, with the .8 production core retained after rejected .10–.12 experiments. The runner reuses radius-order/radius_order.rs and its complete field serialization. prepare-screen.py adds scratch-only counters; measure.py adapts position-integration/measure.py for all four specimens and two CPU-owned delivery modes. Delivery source and executable hashes are recorded separately from the wood screen.

The first delivery candidate build reused the scratch executable from a shared Cargo target cache. Its 0.05s build and identical SHA256 were caught before timing and are an inconclusive observation. delivery-candidate-rebuild.log records the real root rebuild; final baseline/candidate hashes differ, and the final dependency file resolves the root parallel module. No delivery samples came from the false start. FRICTION.md records the cache issue and one resolved dynamic-path guard rejection.

The initial focused gate passed25/25; the actual admitted public-build fallback test then passed1/1. Its injected second-spawn failure proves that a started worker is joined and the production entry retries once, with complete mesh equality to serial. spawn-red.log records the intended failure before the shared spawn seam was wired. Modulation/caps, collapse handling, worker-error joins and capacity gating also have focused coverage. A tests-only split moved the unchanged test module into parallel/tests.rs after timing; production kernels did not change.

The generation-limit scanner remains unchanged. Its final integration inventory found stale .8 compact/prepared/contact traversal entries alongside the new worker scheduling sites; the host authorized truthful classifications for all twelve actual additions and removal of the replaced normal traversal. Worker counts and size gates select execution strategy and never truncate geometry. Final gate results are appended below.

Final root candidate verification passed28/28 focused core/inventory tests and18/18 renderer generation tests, with GPU tests serialized. Release Wasm builds for telperion-wasm and telperion-render passed. Scoped edition2021 rustfmt and git diff --check passed. The host's retention conditions are satisfied. No phone, full cold-start, browser CPU improvement or whole-process memory qualification is claimed.

baseline: green via .8 handoff (core20/20, renderer18/18, mature production1/1, Wasm, TypeScript and browser lifecycle); .9 evidence-only and .10–.12 production reverts. The stale limit inventory discovered during final integration is documented separately above.
Tier: session (jev intelligent 0.78; explicit IMPLEMENTER preserved).
stage: impl-review - skipped(config: REVIEW_MODE=none); host directly reviewed ownership, capacities, fallback, exactness and delivery before retention.
