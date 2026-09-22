> Historical raw-output references: see the [archive and recovery instructions](../README.md).

# GPU wood-position feasibility

The four mature fixtures pass the fixed numeric screen. Combined compact preparation, upload and GPU position/validity/bounds completion is 2.70×/2.83× faster than canonical preparation for oak seeds 1/7, exceeding the preselected 2× bound. Counted simultaneous allocations are lower. The host approved retaining this isolated probe and advancing a separate, explicitly bounded integration task. This is not delivered rendering acceleration or completion of the parent 10×/100 ms goal.

The candidate is not numerically suitable for every valid engine input: a tiny tip 100 km from the origin gains six collapsed faces, and a large-coordinate test moves positions by 11.05 mm. Those failures remain evidence, with no compaction, fallback, relaxed threshold or unrestricted GPU-support claim. Integration must admit a narrower precision domain and otherwise preserve coherent canonical output.

## Final measured boundary

`mature-final.log` and `final-matrix.json` contain the final guarded implementation. Each fixture has one first observation and three warm observations, in an isolated native release test. Canonical preparation completes and drops before compact preparation. Skeleton generation, CPU reference meshes, bulk verification readback and repeatability checks are outside the measured interval. The timed completion includes the 32-byte status readback after position emission, procedural triangle validation, per-run bounds reduction and final reduction. No foliage or completed frame is measured.

| Fixture | Canonical preparation median ms | Compact CPU median ms | Upload/dispatch median ms | Completion median ms | Combined median ms | Speedup |
|---|---:|---:|---:|---:|---:|---:|
| Oak 1 | 120.431 | 42.296 | 1.219 | 1.043 | 44.567 | 2.70× |
| Oak 7 | 132.223 | 44.207 | 1.361 | 1.265 | 46.781 | 2.83× |
| Spruce 1 | 92.871 | 26.498 | 0.992 | 0.954 | 28.444 | 3.27× |
| Spruce 7 | 85.774 | 23.721 | 1.974 | 5.524 | 32.447 | 2.64× |

Stage medians do not necessarily sum to the combined median. This small sample does not establish tail latency. First observations and per-sample stages remain in the log. There was no overlapping build during measurement. The initial matrix predates only the arithmetic guard and diagnostic changes; it remains in `mature.log`/`initial-matrix.json`. One final matrix was warranted because the new guard changes timed validation work.

Hardware is Ryzen 9 5950X and NVIDIA RTX 3080, Vulkan, NVIDIA driver 610.57.04; the final diagnostic records the actual adapter. Rust 1.98.1, x86_64 Linux, wgpu 30, release profile with LTO and one codegen unit. Base revision is ed0f6819; the task commit supplies the candidate source. `provenance.json` records complete final source SHA-256 values and runtime/compiler details. No preset or generator input changes were made; shipped mature oak/spruce parameters with seeds 1/7 are used.

## Fidelity and structural checks

All four candidates have finite coordinates, exact vertex/run/cap counts and strip/cap index ordering, exactly matching cap centres, enclosing bounds and repeatable float32 output/status. Independent f64 admission checks of the readback candidate positions agree with GPU status: zero nonfinite triangles, zero zero-area or admission-range rejected faces, and zero newly collapsed faces. Ring/run counts also exercise the existing 2D dispatch beyond 65,535 workgroups. Canonical CPU surface hashes remain unchanged in the focused renderer gate.

| Fixture | Max / RMS position error µm | Minimum measured ring radius mm | Max radius error µm | Max face normal angle rad | Max area-weighted vertex normal angle rad |
|---|---:|---:|---:|---:|---:|
| Oak 1 | 2.336 / 0.581 | 0.625 | 0.728 | 0.32176 | 0.00347 |
| Oak 7 | 2.336 / 0.606 | 0.625 | 0.674 | 0.01134 | 0.00352 |
| Spruce 1 | 0.983 / 0.193 | 0.250 | 0.305 | 0.01242 | 0.00402 |
| Spruce 7 | 0.985 / 0.196 | 0.250 | 0.309 | 0.01239 | 0.00405 |

The fixed bounds are maximum 50 µm and RMS 10 µm, not changed after measurement. Small branches below 1 mm radius have the same listed maxima. Radii and normals are derived from actual candidate corners/triangles, not descriptor equality. Oak 1's face-normal outlier is important for subsequent fidelity checks; the much smaller vertex-normal change does not erase it or constitute visual acceptance. No images, browser integration or contact-placement qualification were performed in this probe.

`edge-fixed.log` covers minimal, curved/twisted, 0.1 mm-radius branches, far-origin geometry, existing degeneracy, empty/root-only trees, invalid height, float32 input overflow, explicit nonfinite positions and finite positions whose cross products would overflow. The 0.1 mm branch at a 25 m offset has 2.698 µm maximum / 1.134 µm RMS error, no new collapse and 0.03295 rad maximum vertex-normal change. The 100 km case has 11.05 mm maximum / 3.802 mm RMS error; the far-origin tip changes canonical six collapsed triangles to twelve, six newly collapsed. Both are unsupported fidelity outcomes, honestly reported while the diagnostic itself succeeds at detecting them.

The first overflow-arithmetic regression failed: post-cross exponent-bit inspection alone did not flag the extreme result. The host authorized a conservative pre-arithmetic guard. Every triangle point must have finite components bounded by 2^58, then finite edge components bounded by 2^59; each cross component is consequently at most 2^119, below f32::MAX/256 (approximately 2^120). Out-of-range arithmetic produces explicit invalid status before multiplication. This is arithmetic safety, not a sufficient spatial-fidelity admission rule. The regression now passes without weakening its assertion. `FRICTION.md` records discovery and the host decision.

## Explicit simultaneous allocation domains

The probe allocates compact CPU inputs and upload metadata, packed GPU positions, descriptor buffers, per-run reduction scratch, a uniform, status and 32-byte status staging. It allocates no complete index/normal/coordinate/radius arrays. Numbers below exclude the unchanged base tree and any previous live tree, which are common to both sides.

| Fixture | Candidate CPU upload bytes | Candidate GPU peak bytes | Combined bytes | Canonical corresponding upload boundary bytes |
|---|---:|---:|---:|---:|
| Oak 1 | 14,455,140 | 58,931,336 | 73,386,476 | 95,179,124 |
| Oak 7 | 16,709,740 | 67,586,536 | 84,296,276 | 109,095,660 |
| Spruce 1 | 12,131,184 | 48,531,120 | 60,662,304 | 78,274,112 |
| Spruce 7 | 11,479,608 | 45,941,184 | 57,420,792 | 74,098,856 |

Candidate CPU values use actual vector capacities; GPU values use actual buffer sizes plus status staging. Canonical boundary is the existing PreparedSurface capacities plus packed upload metadata plus the position GPU buffer. With V vertices, R rings, U runs and S=20 segments, that is `(12V+8R+36U+4S) + (16U+8R+4S) + 12V`. The formula reproduces .6's recorded spruce prepared CPU 39,656,536 bytes and metadata 2,009,944 bytes. Candidate arrays are `64R+36U+16S` before the extra `16U` packed upload metadata.

CPU preparation reuses the same paths, distance, sample/frame scratch and stable ordering allocation shapes. Its retained output arrays shrink by `184R+24U-12S` bytes on these fixtures. The transient angular helper is bounded to 20 entries and does not reverse that reduction. Thus this probe adds no higher counted preparation/upload peak. The .6 final wood GPU allocations alone are 223,158,952/255,226,624 bytes for oak and 182,554,664/172,834,808 for spruce, already larger than the candidate combined boundary. No downstream final attributes were moved into the foliage peak.

Position buffers retained after probe completion are 44,698,488/51,146,040 bytes for oak and 36,607,632/34,657,728 for spruce. Production integration must release descriptor/reduction scratch before foliage and final wood expansion, retain only needed compact metadata, and remeasure real joint lifetimes. Driver/compiler allocation, allocator overhead, queue upload staging and deferred destruction remain outside this explicit accounting; whole-process peak memory is not qualified.

## Gates and decision

Baseline was green via .6 handoff at ed0f6819 (core 19/19, renderer 14/14, Wasm/browser checks). Final release core surface/prepared/collapse/attachment gate passes 19/19; final renderer generation gate passes 15/15; isolated mature matrix passes 1/1 with all four numeric screens true. Wasm release compile check of the new core API and scoped rustfmt pass. Scratch Naga parsing/validation passed before linking. No debug all-preset or full-workspace suite was run. Logs retain the failed overflow regression followed by its passing fix, and the unchanged CPU hashes.

The host directly reviewed the design, implementation and evidence and approved retention, explicitly carrying the far-origin failure into the next task's admission design. Task .8 owns supported-domain integration, actual candidate radius/contact metadata, coherent canonical fallback, native/browser completion, fidelity and retained CPU-output checks. Parent task .1 remains in progress.

Tier: session (jev intelligent 0.87; explicit IMPLEMENTER preserved).
stage: impl-review - skipped(config: REVIEW_MODE=none)
