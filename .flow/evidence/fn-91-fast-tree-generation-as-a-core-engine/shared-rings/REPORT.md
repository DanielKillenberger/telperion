# Shared canonical ring preparation

The candidate removes repeated contact-ring emission on resident requests with positive surface contact. Spruce's native completed-frame median improves 16.71%, clearing this task's 15% aim. Oak is the zero-contact control: its native median increases 2.00% with overlapping sample ranges, while browser delivery improves slightly. No parent target is waived: oak remains below 10×, every fixture remains above 100 ms, and whole-memory, cold-start and phone qualification remain open.

## Native delivery and CPU control

The saved task .5 executable is the fresh baseline, SHA-256 `6148fccdad4c223db2b6e8d69e637c88a0b0acbec27f3154a005e9e14384b3b2`, from admission revision `ef52e093`. Candidate SHA-256 is `90bae8a8a1471b2b0e3155d57a66d2c5e4772c646560247438bd1269f489a195`. Both use the existing release/LTO profile on the Ryzen 9 5950X, Linux and NVIDIA RTX 3080. `measure.py` runs each in a separate process: first plus three warm mature seed-1 specimens, GPU-resident output, hero camera, 1280×720 and actual device completion. Builds and measurements occupy separate windows. Full fixture parameters, initialization, stages, samples and previous live-tree bytes are in the JSONL files; RSS records retain binary checksums. Background OS activity is uncontrolled; this sample count does not establish tail latency or statistical significance.

| Fixture | First completed frame, baseline → candidate ms | Warm median ms | Change |
|---|---:|---:|---:|
| Oak 1 | 244.377 → 226.512 | 214.439 → 218.737 | 2.00% slower |
| Spruce 1 | 277.478 → 229.996 | 257.917 → 214.812 | 16.71% faster |

All three spruce candidate warm samples beat every baseline warm sample. Oak's warm ranges overlap (214.276–224.221 versus 211.006–220.258 ms). The first candidate oak process initializes in 2577.300 ms versus baseline 288.187 ms; subsequent spruce initialization is 222.085 versus 222.217 ms. Shader/driver cache state is uncontrolled. These observations do not qualify cold startup.

Owned CPU-output medians are 586.162 → 591.471 ms for oak (+0.91%) and 4744.904 → 4705.691 ms for spruce (−0.83%). This comparison detects no substantial change at its sample size. It does not resolve the older pre-.4 oak CPU attribution question. `cpu-summary.json` and `cpu-output-*.jsonl` preserve the observations.

The measured native executable predates only the final `canopy.size != 0` shared-path admission guard. All measured specimens have positive size, so their path is unchanged. The final guard is compiled in the Wasm build and final native checks; no repeated mature measurement is claimed for it.

## Browser delivery

`browser-completed.json` uses the existing six-sample protocol (first plus five warm), four mature fixtures, the same 1280×720 hero view and renderer queue completion. Chromium version, flags, adapter and final module SHA are recorded there. `browser-smoke.json` passes the existing validation, fallback, overlapping request, stale replacement, disposal and post-disposal checks.

The saved browser baseline is task .4's module, not .5: these gains include both admission traversal (.5) and shared contacts (.6). Native results above isolate .6.

| Fixture | .4 → candidate warm completed-frame median ms | Cumulative improvement | Original baseline speedup | Above 100 ms |
|---|---:|---:|---:|---:|
| Oak 1 | 325.5 → 320.1 | 1.66% | 4.90× | 220.1 ms |
| Oak 7 | 383.7 → 378.8 | 1.28% | 4.92× | 278.8 ms |
| Spruce 1 | 384.0 → 313.9 | 18.26% | 24.30× | 213.9 ms |
| Spruce 7 | 366.5 → 293.6 | 19.89% | 25.15× | 193.6 ms |

Wasm module high-water capacity is unchanged for oak 1 (101,187,584 bytes) and falls for oak 7 (189,267,968 → 162,988,032), spruce 1 (216,006,656 → 134,676,480) and spruce 7 (209,715,200 → 132,710,400). This is linear-memory capacity, not simultaneous live CPU allocation or physical residency.

## Output and ownership

`DESIGN.md` records the host-approved boundary. The optional private node map comes from the canonical sorted run, including burial offsets and excluding caps. Read-only access and consuming extraction prevent callers from changing positions beneath that map. Default `prepare` and CPU builders request no map. Default owned foliage stations remain available; the shared station API borrows real float32 positions and skips ring emission and unused segment AABBs. Capability checks avoid the shared work for known unsupported, zero-contact and zero-size requests. Collapsed geometry and compute limits use explicit fallback; device errors remain scoped, and renderer identity/replacement rules are unchanged.

One packed xyz GPU buffer is uploaded before foliage, retained through mass construction, and adopted as wood positions. CPU positions and the map are released before foliage GPU work; only compact metadata survives. Owned standalone contacts are explicitly owned and drop before mass construction. Contact WGSL changes only its load layout, from padded vec4 to three f32 values; projection operations and ordering are unchanged.

All four fixtures' prepared-field Debug-text FNV-1a hashes equal task .5 and repeat across four runs: oak 1 `5f2459a6f88d0fa8`, oak 7 `e6e8e17976391a22`, spruce 1 `1aa9eb9b861ccd2c`, spruce 7 `7b94d8082b55e601`. `preparation.rs` also compares every station field after remapping indices and every rounded contact corner. Its microtimings are diagnostic only: old inputs remain live while shared preparation is timed, so no performance claim relies on them. GPU leaf readback runs separately from timing: seed-1 hashes match baseline exactly (`79cca656bd60f673` oak, `7ee559575850f9b2` spruce). Bounds and counts match. Existing four-fixture wood tests preserve CPU surface hashes, positions, indices, coordinates, bark radii and the normal-angle contract. Exact-output checks reuse task .4's accepted visual evidence; no new images or owner confirmation were requested.

## Simultaneous allocation domains

For native spruce 1, descriptor CPU capacity falls from 155,994,528 to 38,124,928 bytes. The new shared-preparation CPU snapshot is 69,686,496 bytes, including canonical wood, station descriptors and map/upload-metadata overlap; retained metadata during foliage is 2,841,112 bytes. Map capacity is 3,815,560 bytes. Base tree/element CPU capacity is 26,217,504 bytes in both runs.

The foliage GPU peak falls from 265,081,248 to 254,541,040 bytes. The shared position buffer is already inside that peak and is counted once; its lifetime through mass work is included. Wood GPU peak remains 182,554,664 bytes, again counting positions once. New foliage remains 88,624,472 bytes and the previous live tree remains 416,821,772 bytes. Thus wood expansion's simultaneous GPU total remains 688,000,908 bytes. Its counted CPU overlap changes from base + prepared wood + packed metadata (67,883,984 bytes) to base + retained compact metadata (29,058,616 bytes). The corresponding explicit combined snapshot decreases from 755,884,892 to 717,059,524 bytes. The earlier foliage GPU snapshot plus base/retained metadata is lower than that; preparation/upload snapshots are also lower. Oak has no shared map or retained shared metadata, and its GPU counts remain unchanged. These explicit snapshots support no increase in the counted domains, not whole-peak qualification.

Process peak RSS changes 308,452 → 354,588 KiB for oak and 387,212 → 309,312 KiB for spruce. The oak increase remains an unresolved process-domain observation; its candidate initialization was also much slower. Separate single-build verification processes report 284,528 → 284,592 KiB for oak, but that different protocol cannot cancel the measured replacement-run increase. Scratch allocation peaks, allocator overhead, wgpu upload staging, shader/compiler/driver memory and deferred destruction are not fully accounted. Whole-memory and phone requirements remain gaps.

## Gates and decision

Baseline: green release core 17/17 and renderer 14/14, plus scoped formatting. The accidentally selected debug core baseline was interrupted and is inconclusive; the debug renderer suite completed 14/14. `../FRICTION.md` records the selection error when it occurred.

New core coverage first failed because the shared APIs did not exist, then passed exact station/contact checks across reordered runs and burial offsets. Empty/root-only coverage first failed on premature solved-radius validation and then passed. The existing 65,536-run GPU test caught a dispatch-count regression in the ownership split; dispatch now uses the compute run count, independent of the render run table. No test or gate was weakened. The extended GPU test checks exact owned/shared leaf bytes and identity of the adopted position buffer. Focused final logs, Wasm build, TypeScript, formatting and browser smoke/matrix are the verification evidence; no full-workspace LTO gate is claimed.

Host retention decision is pending final review of these results. This task evaluates a bounded improvement; it does not complete task .1 or the parent spec.

Tier: session (jev intelligent0.72; explicit IMPLEMENTER preserved)
stage: impl-review - skipped(config: REVIEW_MODE=none)
