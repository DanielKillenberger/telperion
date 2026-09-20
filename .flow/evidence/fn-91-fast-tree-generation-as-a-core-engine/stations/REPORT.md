# Shared station preparation screen

The host provisionally retained shared station streaming and zero-contact GPU-admission overlap after one browser qualification. Warm completed-frame medians are158.1/180.8ms for oak and178.9/164.2ms for spruce. Oak seed1 remains1.1ms above its strict10× target; its9.93× result is not relabeled10×. The host chose to stop optimization and finish validation, without another timing sample or candidate.

**Memory tradeoff:** the explicit live CPU/GPU allocation envelope is unchanged on all four fixtures, including the newly overlapping station scratch. Oak seed7 Wasm linear-memory high-water nevertheless increases106,496,000→132,775,936bytes (+26,279,936bytes). Oak seed1 decreases94,240,768→69,599,232bytes, and spruce stays98,041,856bytes. Unused/allocator-retained linear-memory capacity is not part of the live envelope, and cannot simply be added without double-counting its live CPU allocations. This is not a whole-memory nonincrease qualification.

## Browser completed frame

| Fixture | Fresh baseline ms | Candidate ms | Original speedup | Strict10× margin |
|---|---:|---:|---:|---:|
| Oak1 |178.1|158.1|9.9304×|misses by1.1ms|
| Oak7 |198.2|180.8|10.3014×|inside by5.45ms|
| Spruce1 |194.2|178.9|42.6417×|inside by583.96ms|
| Spruce7 |186.5|164.2|44.9695×|inside by574.2ms|

One first-plus-five-warm matrix per package at1280×720 uses actual renderer queue completion. Candidate oak warm ranges are156.1–159.1/176.4–183.4ms; seed1's range crosses its threshold and its median misses. There is no robust both-oak strict10× pass. All24 candidate samples use admitted GPU positions without fallback. Oak remaining position wait falls12.4/13.6→0.2/0.3ms. Contact-bearing spruce retains its dependency order and9–10ms wait. Source-level synchronous queue submission and a native eager-map test support the mechanism; these browser measurements are the evidence of overlap effectiveness, not a promise of driver-independent behavior.

Warm explicit joint-envelope maxima remain511,842,716/609,104,276/736,449,204/702,912,352bytes. The candidate's conservative preparation overlap includes measured complete station peak with old output capacity during growth, pending position CPU/GPU, base CPU and previous tree. Final wood expansion remains the maximum. Native usize-sized diagnostic bookkeeping conservatively overestimates Wasm vector bookkeeping. Allocator overhead, upload staging, driver/compiler resources and deferred destruction remain unmeasured. Cold full startup and phone qualification remain absent; all warm cases remain above100ms.

## Shared preparation screen

The native screen misses the50% oak-stage aspiration. It saves4.66/6.22ms on oak and7.56/7.35ms on spruce; the host admitted the fixed overlap as a worthwhile combined candidate.

| Fixture | Baseline warm median ms | Candidate warm median ms | Saving ms | Speedup |
|---|---:|---:|---:|---:|
| Oak 1 | 23.160796 | 18.498085 | 4.662711 | 1.2521x |
| Oak 7 | 26.926198 | 20.709123 | 6.217075 | 1.3002x |
| Spruce 1 | 28.263238 | 20.698663 | 7.564575 | 1.3655x |
| Spruce 7 | 26.112751 | 18.758500 | 7.354251 | 1.3920x |

Each fixture ran baseline then candidate, first plus three warm samples; the median excludes the first. Raw JSONL includes every sample and parameters. Core release binaries use distinct archived source roots and target directories; binary-hashes.json and source-hashes.json verify provenance. The runner derives from .9 and stops after compact/station preparation; no full CPU foliage is timed. Both variants use the same baseline skeleton and compact preparation. Output serialization and comparisons run outside the station timer.

Every field of every station record matches exactly on the four mature fixtures: 55,005/66,870/93,051/88,111 records. This includes count, ordinals, tile phase, endpoints, radii, distances, frames and contact indices. Each variant's three warm dumps also matches its first dump exactly. Curved synthetic runs separately exercise the intended algebraic frame change against the canonical two-pass computation with 1e-12 component-distance bounds and independent orthonormal checks. Duplicate/zero spans, tiny and large finite coordinates, near-antiparallel and antiparallel transport are covered. Existing invalid-input/instance-limit station tests pass.

DESIGN.md proves no increased explicit per-run allocation envelope without relying on fixture run lengths: retained scratch does not cross runs, prepared frame storage is removed, child storage shrinks, and output push order/capacity stays fixed. This is not an allocator/driver RSS qualification.

Baseline passed42/42 selected core tests. Candidate final focused checks passed15/15; the initial inventory failure identified two removed loop sites, removed from the inventory without changing its scanner. Scoped format and diffcheck pass. Renderer generation checks pass20/20, including real postsubmission station fallback/error and reuse. Browser lifecycle smoke passes capability/error, busy overlap, stale replacement, disposal and zero live devices. Renderer Wasm build passes. The measured source is preserved in measured-candidate.patch; subsequent cleanup only renames an unused Wasm parameter gpu to _gpu, with no retiming.

The baseline core source is bf3e2fbb671f6832a8a5112a5b21691cd9d1e253. The existing position wait of12–13ms plus the native station savings is only a rough upper opportunity, not an additive completed-frame prediction. The historical oak1 gap to10x is22.1ms, larger than that rough opportunity; oak7's17.45ms gap is comparable. No threshold tuning, repeated qualification or second candidate screen has been performed.

## Native owned-output controls

One serial paired first-plus-three-warm control matrix uses the preserved, hash-verified .13 executable and a distinct candidate executable. CPU-owned output remains2.08/2.14/1.25/1.25× faster than the original baseline; GPU-assisted owned output is6.48/6.84/27.44/27.49×. These are separate representations from browser-resident delivery, and every native result remains above100ms.

| Mode/fixture | Baseline ms | Candidate ms | Gain | RSS delta KiB |
|---|---:|---:|---:|---:|
| cpu-output oregon-white-oak 1 | 477.635 | 453.891 | 4.97% | +288 |
| cpu-output oregon-white-oak 7 | 537.332 | 532.197 | 0.96% | -548 |
| cpu-output norway-spruce 1 | 4471.032 | 4472.435 | -0.03% | -5380 |
| cpu-output norway-spruce 7 | 4276.233 | 4256.987 | 0.45% | +2056 |
| gpu-output oregon-white-oak 1 | 149.448 | 145.592 | 2.58% | +40084 |
| gpu-output oregon-white-oak 7 | 184.728 | 166.327 | 9.96% | -100 |
| gpu-output norway-spruce 1 | 216.658 | 203.410 | 6.11% | +2592 |
| gpu-output norway-spruce 7 | 203.230 | 194.178 | 4.45% | -4044 |

GPU-assisted owned oak1 maximum RSS increases40,084KiB in this observation. That is a real unexplained process-memory increase, separate from oak7 browser Wasm capacity growth and from explicit live-buffer envelopes. It is not erased by another run. CPU-owned spruce1 is0.03% slower. No global performance or memory nonregression claim is made. The host retains the delivered latency gain with these qualification gaps explicit.

The reused timing runner omitted GENERATION_VERIFY, so its hash fields are null; counts/bounds repeat exactly but do not constitute full output equality. A separate one-sample untimed verification uses GENERATION_VERIFY=1 on all eight mode/fixture pairs for baseline and candidate. Its samples are excluded from every latency statistic. FRICTION.md records this missing-flag cost.

All eight untimed requested-output pairs have identical foliage hashes. Together with exact mature station records, unchanged wood kernels and existing GPU geometry/contact oracles, this carries forward the accepted mature visual evidence without new captures. No cross-backend equality is claimed; the comparison is baseline versus candidate within each output backend.

Final worker gates: focused core/inventory15/15, renderer generation20/20, both release Wasm targets, TypeScript and scoped format/diffcheck pass. Baseline core42/42 passed. The host owns the final aggregate workspace/JavaScript gates. `gpu`→`_gpu` removes the Wasm unused-argument warning after measurement; it changes the packaged Wasm SHA from01a03c43… to c9886b98… despite no behavioral source change. Both hashes and the source-only cleanup are recorded; no latency remeasurement was performed.

Tier: session (jev intelligent0.90; explicit IMPLEMENTER preserved).
Stage: host direct source, lifecycle and capacity review admitted the one fixed candidate; retention accepts useful latency improvement with strict oak1 target and whole-memory gaps explicit. No further optimization task follows this screen.

The host's final aggregate passes742 Rust tests (20skipped) and108 JavaScript tests after the test-reference correction. A final standards-review follow-up documents Rust delivery/backend/metric semantics and types the33 existing browser stage fields plus live-tree GPU byte snapshots. It changes no runtime behavior; direct TypeScript checking and scoped Rust format pass. The final correctness review reported no findings. The strict oak1 target and memory qualification gaps above remain unchanged by these gates.
