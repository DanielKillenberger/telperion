# Native GPU foliage experiment — 2026-09-20

This experiment improves native seed-1 latency, but does not complete fn-91. The matched CPU reference is the current CPU candidate at f2c7e81c, not the original pre-fn91 implementation. Spruce resident completed rendering is 10.13× faster in this small native sample; oak is 1.57×. CPU-output and oak resident process-RSS results fail to establish the no-memory-increase requirement. Other seeds, browser GPU adoption, phone hardware, motion/close-up quality remain open; the owner accepted the four shown hero views only.

## Protocol and provenance

`cargo build --release -p telperion-render --example generation_gpu`, rustc 1.98.1 (48a229cea), Linux 7.2.3-arch1-3 x86_64, NVIDIA GeForce RTX 3080. Each species/mode has one fresh process, one first build and three subsequent builds. Every build regenerates the skeleton and specimen. Only device/pipeline initialization persists; no generated-output cache exists. Runs were sequential after the host released its browser/RSS measurement window. CPU and GPU modes use the same executable and authored family/seed; full parameters and adapter identity are in each JSONL header.

Modes are explicit: `cpu-output`, `gpu-output`, `cpu-render`, `gpu-render`. Output means owned wood and packed foliage, with the GPU mode's full readback inside the timer. Rendering means preparation, submission, hero camera at 1280×720 and device/queue completion. Frame pixel readback and verification leaf readback are outside render timing. The retained previous tree remains live during later preparations. `gpu-*-*.jsonl` holds every sample; `GPU-CANDIDATE-summary.json` holds the independent stage medians. With only three warm observations, report p50/max, not p95.

Initialization is separate from an initialized-engine build. Process initialization ranged from 331–429 ms for renderer modes, except the first GPU-output oak process at 2,976 ms. Shader/driver disk caches were not cleared, so these are process-initialization observations, not a controlled cold-cache qualification. CPU-output constructs no renderer; GPU-output currently constructs the full renderer before its generator. Avoiding that renderer is a plausible memory improvement, not a measured adjustment to the reported result.

`gpu-measured-binary.sha256` records the executable used for timing. A later capture executable adds capacity reporting, the extreme-phase capability fallback and verification/capture support; it retains the timed normal-fixture algorithm. It is preserved as `/tmp/telperion-fn91-tools/generation-gpu-first-candidate` (SHA256 93705480039024b1a4f9862ba2b43cb649fc65b07d9d51a12e815448c2239ca2). The existing CPU stage binary was also copied to `generation-stages-first-gpu-checkpoint`. Both timings and four captures precede the final signed-zero extrema-key fix described below; no normal-fixture speed claim depends on that edge case.

## End-to-end measurements

Milliseconds; first build excludes the separate initialization above.

| Fixture / delivery | CPU first | GPU first | CPU warm p50 / max | GPU warm p50 / max | Warm speedup |
|---|---:|---:|---:|---:|---:|
| Oak1 owned CPU output | 720.87 | 395.97 | 699.50 / 711.09 | 390.83 / 408.89 | 1.79× |
| Oak1 completed frame | 817.49 | 514.79 | 806.90 / 819.11 | 514.34 / 516.09 | 1.57× |
| Spruce1 owned CPU output | 4,953.64 | 570.01 | 4,880.27 / 4,945.17 | 546.59 / 550.67 | 8.93× |
| Spruce1 completed frame | 5,194.69 | 531.03 | 5,242.21 / 5,246.66 | 517.60 / 558.08 | 10.13× |

Counts remain 715,065 oak leaves and 7,353,754 spruce needles. These fixtures retain all initial stations; the small correctness fixture separately exercises actual culling and stable survivor order.

GPU-output warm stage medians: oak skeleton 70.65 ms, descriptor/contact preparation 26.67 ms, upload/dispatch 1.64 ms, placement/cull/bounds completion wait 1.30 ms, compaction 0.15 ms, mass 0.13 ms, readback 2.40 ms, wood 287.32 ms. Spruce: 47.02, 141.59, 25.09, 13.10, 0.56, 0.37, 89.73 and 227.10 ms respectively. Upload timing may overlap GPU execution; the subsequent wait exposes completion, so do not interpret it as isolated shader duration. Prefix work is included in placement completion. Stage medians need not add to the total median.

Resident preparation p50 is 373.75 ms oak and 427.28 ms spruce. Remaining completed delivery is 138.14 and 90.32 ms. Wood now dominates oak and is the largest spruce resident preparation stage; contact/descriptor construction follows for spruce. Full CPU readback is material for spruce output mode. No wood GPU expansion or further fixture expansion was attempted.

## Memory: different observations, not one interchangeable peak

Linux child-process `wait4.ru_maxrss`, KiB, entire four-build process including initialization:

| Fixture / delivery | CPU peak RSS | GPU peak RSS |
|---|---:|---:|
| Oak1 owned CPU output | 256,764 | 501,808 |
| Oak1 completed rendering | 585,200 | 621,992 |
| Spruce1 owned CPU output | 406,020 | 500,816 |
| Spruce1 completed rendering | 620,256 | 484,784 |

These are matched-instrumentation raw observations, not repeated statistical memory qualification. The GPU runtime and unnecessary full-renderer initialization in GPU-output remain included. No RSS rise is explained away or subtracted. Resident spruce improves this observation, while resident oak and both output modes do not.

Explicit allocated buffer/capacity snapshots, bytes:

| Allocation domain | Oak1 | Spruce1 |
|---|---:|---:|
| Prepared descriptors/rings plus converted upload vectors, CPU capacity | 20,147,840 | 155,994,528 |
| GPU generation phase peak, including raw/rank/final/config/summary/contact/element buffers | 27,078,068 | 265,081,248 |
| Retained GPU leaf + mass buffers after generation | 8,850,092 | 88,624,472 |
| Skeleton + principal element CPU capacities | 26,230,976 | 26,217,504 |
| Completed CPU wood capacities | 206,813,840 | 169,173,280 |
| CPU-path submitted tree GPU buffers | 312,228,772 | 484,114,056 |
| GPU-path submitted tree GPU buffers | 310,016,248 | 461,957,936 |

Upload vectors and compact descriptors are dropped before raw/final generation allocation. Contact/raw/rank buffers and bind groups are released after GPU completion and before CPU wood construction. Resident delivery retains only packed leaves and mass while wood builds. CPU-output copies packed leaves through mapped staging and a byte vector into owned leaves, then drops GPU foliage before wood. That implementation has an additional staging/byte-vector/owned-vector overlap during readback; it is not hidden from the timer or RSS. Counted upper overlap for these three readback allocations is another 36 bytes per surviving leaf beside resident buffers and CPU base. Queue staging, contact-builder scratch, growth/wood scratch, allocator retention, driver/pipeline allocations, textures and complete process/device peaks are not captured by the explicit capacity table. RSS captures process-resident memory, not device-local VRAM.

Warm old-tree buffers remain live: add the previous GPU-path tree allocation to generation phase buffers when reasoning about that phase. For example spruce has roughly 462 MB of old tree buffers beside the 265 MB generation phase, plus CPU data. These lifetime sums are accounted buffer observations, not a no-total-memory-rise proof.

## Implementation and verification scope

The pure core provides regular twig station tiles and swept contact rings without a GPU dependency or CPU per-leaf transforms. Capability checks explicitly fall back for short shoots, limb clumping, non-twig placement and extreme phase uncertainty. Invalid parameters and authored population budgets still reject. Tile phase bases use the original f64 angle chain; the shader evaluates only reduced local deltas. Eight f64 rounding units above 0.00025 rad trigger fallback. This is a conservative experiment capability policy, not a rigorous bound on total GPU error. An illustrative host-arithmetic diagnostic at divergence 1e9 degrees and internode ordinal 1e7 found about 0.043 rad discrepancy between original full-angle arithmetic and periodic reconstruction, justifying explicit fallback rather than assuming visual equivalence.

GPU passes use indexed SplitMix32 draws, actual swept-facet ray contact and closest-facet fallback, current shell semantics, stable rank/prefix compaction, full and origin bounds, and canopy occupancy/depth. Dense checked 2D dispatch indexes descriptor-local tiles by binary search. The resident buffer enters existing Select/Foliage draw setup. Owned prepared results are renderer-bound; wrong-renderer submission fails before replacing an existing tree. The native API is experimental; browser production setTree is not switched.

Focused tests cover repeated packed bytes, actual survivor rejection with CPU positions/orientations/scales in survivor order, bent/rotated contact within packed-position/f32 tolerance, bounds and mass, invalid/budget/empty/fallback, large-ordinal orientation and wrong-renderer preservation. Host review found that the initial float-to-ordered-key function mishandled negative zero; the final implementation uses the sign bit and adds a GPU probe over -0, +0 and mixed signed finite extrema. This fix is included in the final gate, not silently attributed to the earlier captures.

Four hero images only: `gpu-hero-oregon-white-oak-{cpu,gpu}.png` and `gpu-hero-norway-spruce-{cpu,gpu}.png`. Their JSONL files retain packed-leaf FNV1a fingerprints. Bytes intentionally differ: oak CPU ae785c3b8b7e72e7 / GPU 79cca656bd60f673; spruce CPU 5f60764ddbf12010 / GPU 7ee559575850f9b2. Host inspected all four and saw no obvious canopy silhouette/density or gross geometry loss at this scale; spruce sparsity occurs in both. The owner viewed the gallery and said “they look the same to me”; OWNER-VISUAL.md records this scoped acceptance. These views do not qualify close attachment detail, motion or all supported views.

Final gate results are recorded below when observed. The previous canonical Cargo workspace SIGSEGV remains failed evidence; its later isolated renderer/Wasm pass is not relabeled as a canonical pass. Task remains in_progress. Next changes require host dispatch; `NEXT-EXPERIMENT.md` is design evidence only.

## Final verification

The isolated nextest run selected 215 tests across 48 binaries: all renderer tests, plus core foliage/packed-leaf/mesh/surface/generation-limit suites and relevant core unit modules. It completed in 79.701 s with 214 passes and one family-table boundary failure. The host resolved that boundary by moving the generic `Family` struct and Default into core `family.rs`, exporting it at the core root, and retaining the old preset-module export for compatibility. The renderer imports the neutral type; the conformance guard is unchanged. A targeted replay of that conformance test and all five GPU tests passed 6/6 in 3.852 s. The initial failed run remains `gpu-nextest.log`; the correction is `gpu-targeted-nextest.log`. No other passed suite was repeated.

The package-wide build preceding that run timed out at 300 s because test filtering did not narrow Cargo binary linking. The host-authorized explicit-target recovery completed in 43.58 s; its exact target list and artifact timestamps are retained. Test execution used saved binaries, `-j 4`, `--no-fail-fast`, and a separate 180 s limit. No canonical workspace pass is claimed.

`cargo check --release --target wasm32-unknown-unknown -p telperion-wasm --lib` passed in 3.09 s. The renderer's separate Wasm target check and final formatting result are retained in their adjacent logs. The final targeted native build carried two unused-import warnings from moving Family; removing those unused imports was the only subsequent native-source cleanup. It does not change executable behavior.

This is worker invocation 3 and one substantive experiment checkpoint. Task status remains in_progress; the next experiment is not implemented here.

The separate `telperion-render` Wasm-target check passed in 2.76 s, with four dead-code warnings for the native experiment's renderer identity/resident helper methods. They are recorded, not treated as runtime qualification. Scoped rustfmt check passed. No further implementation was made after the host's checkpoint instruction.
