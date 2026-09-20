# Qualified resident GPU positions

The candidate meets the task’s incremental browser target: oak seeds 1/7 improve 44.47%/45.05% in completed-frame delivery, with no increase in the explicit joint-capacity envelopes. All four mature fixtures use GPU positions without fallback. This does not complete the parent goal: oak remains below 10×, every fixture remains above 100 ms, and phone, cold-start and complete peak-memory qualification remain open.

## Browser delivery

Fresh .6 baseline and candidate use Chromium 152, NVIDIA RTX 3080, 1280×720 hero framing and renderer queue completion. Each fixture has a first observation plus five warm observations. `browser-baseline.json` and `browser-candidate.json` retain hardware, flags, module hashes, initialization, each sample and stage timing. No builds or other timed runs overlap these samples. Cache/compiler state and background OS activity are uncontrolled; five warm observations do not establish tails or significance.

| Fixture | .6 warm ms | Candidate warm ms | Improvement | Original speedup | Above 100 ms |
|---|---:|---:|---:|---:|---:|
| oregon-white-oak 1 | 322.5 | 179.1 | 44.47% | 8.77× | 79.1 ms |
| oregon-white-oak 7 | 370.7 | 203.7 | 45.05% | 9.14× | 103.7 ms |
| norway-spruce 1 | 303.2 | 194.6 | 35.82% | 39.20× | 94.6 ms |
| norway-spruce 7 | 295.6 | 178.9 | 39.48% | 41.27× | 78.9 ms |

Original comparable completed-frame targets for 10× are 157.0/186.2 ms for oak and 762.9/738.4 ms for spruce. Spruce clears those targets; oak misses by 22.1/17.5 ms. These original speedups compare browser resident rendering with the original browser completed-frame baseline, not native CPU-owned output. All 24 candidate browser observations and eight primary native resident observations report `gpuPositions=true`, `positionFallback=null`. Rejected precision, geometric and profile cases are tested separately; their timings are never counted as GPU position success.

## Native delivery and retained CPU controls

The saved .6 native executable SHA-256 is `90bae8a8a1471b2b0e3155d57a66d2c5e4772c646560247438bd1269f489a195`; the candidate is `5d3c9d83397c4185b3dfd78555d087e850b89ae4c0f7aa2bf08a44d09352d417`. Both run the shipped mature seed-1 parameters in separate processes on Ryzen 9 5950X / RTX 3080 Vulkan, release/LTO. `measure.py` preserves first plus three warm observations, initialization and wait4 RSS. Native rendering uses the same 1280×720 hero and actual completion.

| Requested output | Fixture | Baseline median ms | Candidate median ms | Change |
|---|---|---:|---:|---:|
| gpu-render | oregon-white-oak 1 | 213.334 | 133.332 | -37.50% |
| gpu-render | norway-spruce 1 | 204.706 | 142.152 | -30.56% |
| cpu-output | oregon-white-oak 1 | 565.345 | 562.728 | -0.46% |
| cpu-output | norway-spruce 1 | 4567.812 | 4651.211 | +1.83% |
| gpu-output | oregon-white-oak 1 | 270.589 | 288.407 | +6.59% |
| gpu-output | norway-spruce 1 | 331.645 | 305.658 | -7.84% |

GPU-to-CPU oak’s observed 6.59% regression remains a result, not erased by unchanged owned-path source. Warm ranges overlap: 270.238–275.366 ms baseline and 265.601–302.551 ms candidate. Its stage medians also vary across skeleton, descriptors, wood and readback; this sample does not attribute causality. No repeated control loop was used to improve that number. Default CPU hashes and canonical exact-position/radius/index contracts remain unchanged.

## Geometry, admission and ownership

The initial capability domain is unmodulated profiles (`lobes==0` or `lobe_depth==0`), ring centres within 64 m per component, radius ≤32 m and radius ≥`max(1 m,max(abs(centre)))/131072`. This is a conservative fast-path domain, not an engine parameter restriction or a universal error bound. Other valid inputs use the complete canonical resident path. Compact float32 packing rejection also falls back; genuine invalid engine inputs and device failures retain explicit errors.

Station capability is checked before position emission. A typed compact contact map preserves canonical run ordering, burial offsets and node ranges. GPU position emission precedes triangle and normal admission plus enclosing-bounds reduction; only a successful 32-byte status readback exposes the candidate to foliage. Point components ≤2^58 imply edges ≤2^59 and cross components ≤2^119, below the existing admission maximum. Guards precede multiplication; shared reduction flags are snapshotted before unconditional barriers. Geometry rejection releases buffers and candidate metadata before canonical preparation.

One immutable packed xyz buffer is borrowed by contacts and adopted by final wood. Actual ring radii are reduced from emitted corners using coordinates relative to the first corner; descriptor radius is never substituted for the bark metric. GPU descriptors and reduction scratch drop before foliage. GPU ring/run/angle metadata remains through foliage; full normals, coordinates, radii and index output arrays are allocated afterward. The preflight and final expansion share the same ordered area-weighted normal helper. An unexpected later normal failure is an explicit invariant/device error, not mixed canonical wood with candidate contacts. Standalone CPU-output generators create no position pipeline.

`numeric.json` / `mature-production.log` show exact counts, caps and topology; finite positions, enclosing bounds, repeatable position bytes and zero new collapsed faces for all four fixtures. Maximum/RMS position differences remain 2.336/0.606 µm for oak and below 0.985/0.196 µm for spruce, within the unchanged 50/10 µm screen. The skinny oak face-normal outlier remains 0.32176 rad; maximum area-weighted vertex-normal difference versus canonical is 0.00405 rad. These are distinct from arithmetic accuracy on the candidate mesh: GPU radius versus independent f64 candidate-corner radius differs by at most 0.119 µm absolute and 3.494e-7 relative; normals versus f64 sums over actual candidate triangles differ by at most 2.20e-7 rad.

The contact test measures decoded leaf origins against the actual emitted triangles under the existing two-quantization-step plus float32 arithmetic tolerance. It also verifies that contacts and final wood hold the same position buffer. Core map checks cover reordered paths/burial and preserve every station field. Numeric edge tests cover minimal/curved geometry, domain boundaries, root-only/empty output, rejected tiny/far inputs, collapsed geometry and device failure. Existing limits, normal failure and lifecycle tests remain.

An isolated preflight-normal-only rejection fixture remains unproven: two proposed folded-ring fixtures had nonzero ordered normal sums, so their false assertions were removed after a bounded host decision. No production guard or threshold changed. The canonical unusable-normal test, shared-helper equivalence, code review and independent actual-position normal checks are the evidence for this branch; this is an explicit test limitation.

## Explicit memory accounting

`summary.json` and `summarize.py` retain each sample’s phase capacities. Final expansion follows actual overlapping ownership: previous live tree + new foliage + wood outputs/metadata/status/staging + base CPU + retained run table + CPU metadata on the canonical path. Canonical CPU positions have already dropped and are not counted again there. Preparation and foliage envelopes conservatively combine each phase’s measured CPU capacity maximum with GPU buffers; they may overcount allocations that drop earlier, and are not whole-process measurements. Shared positions count once, including zero-contact and zero-leaf cases; GPU metadata remains counted through foliage. Rejected candidates use maximum phase snapshots, not a sum of nonoverlapping fallback lifetimes.

| Browser fixture | Baseline joint envelope bytes | Candidate bytes | Wasm high-water bytes baseline → candidate |
|---|---:|---:|---:|
| oregon-white-oak 1 | 514,177,820 | 511,842,716 | 101,187,584 → 94,240,768 |
| oregon-white-oak 7 | 611,832,716 | 609,104,276 | 162,988,032 → 106,496,000 |
| norway-spruce 1 | 736,449,204 | 736,449,204 | 134,676,480 → 98,041,856 |
| norway-spruce 7 | 702,912,352 | 702,912,352 | 132,710,400 → 98,041,856 |

The primary native oak RSS increases 302,344→323,460 KiB while candidate initialization is unusually slow; spruce decreases 310,052→283,740 KiB. One host-requested same-protocol oak follow-up records 302,260→271,140 KiB with initialization 270.719→212.806 ms and warm medians 218.504→133.948 ms. This is consistent with cache/compiler-state sensitivity, not proof of its cause; primary latency and RSS observations remain unchanged. No more repetitions were used. Allocator overhead, compiler/driver resources, upload staging and deferred destruction remain outside explicit capacities. Wasm capacity and process RSS are separate observations, not complete memory qualification.

## Visual scope and gates

The four PNGs compare the prior GPU path against the candidate for oak/spruce seed 1, in the previously accepted lit exterior **bare-wood** view. Candidate captures load the exact baseline pose; material, light and view are unchanged. They assert actual GPU position delivery. They do not establish canopy appearance, close/motion coverage or device-independent fidelity. The host inspected candidate oak/spruce images against the prior GPU view, found no perceptible blocker in this fixed bare-wood comparison, and opened [the HTTP gallery](http://127.0.0.1:8768/positions.html) via `xdg-open`. On 2026-09-20 the host approved retention subject to final green gates, preserving all named limitations; the terminal gate is now green.

Baseline: green via .7 handoff (core19/19, renderer15/15, mature probe1/1, Wasm core check). Core final20/20, production mature1/1 and terminal release renderer18/18 pass in their logs; Wasm build and TypeScript pass. Browser smoke checks candidate and canonical profile accounting plus errors, capability fallback, overlap, stale replacement and disposal. False fixture observations and the old smoke accounting assertion failure remain in logs/FRICTION.md; no failed observation is reported as green. The host directly reviewed code, numerical results, delivery, memory and visuals and approved retention; gate receipts and handover preserve that bounded decision.

Tier: session (jev intelligent 0.41; explicit IMPLEMENTER preserved).
stage: impl-review - skipped(config: REVIEW_MODE=none)
