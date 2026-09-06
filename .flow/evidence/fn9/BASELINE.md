# FN9 baseline — 2026-09-06

Base commit: `7ce571c`. Research only: no Family calibration or generator edits preceded profile freezing. The existing `Ordinary` Family, seed 42, was measured and captured as a shared capability probe against both profiles. It is not presented as either species.

## Verification and CPU baseline

Host: Linux x86_64, AMD Ryzen 9 5950X (16 cores / 32 threads). Toolchain environment: `PATH=/tmp/telperion-cargo/bin:$PATH RUSTUP_HOME=/tmp/telperion-rustup CARGO_HOME=/tmp/telperion-cargo`.

Pre-edit commands exited 0: `cargo test --release -p telperion-core --test growth --test foliage` (14 tests), `npm run typecheck`, and `cargo run --release -p telperion-core --example measure`. The native runner performs one warm-up plus five measurements of the same seed, not six different specimens. No gate receipt was available. Logs are disposable `.flow/tmp/baseline-*.log`.

| Observation | Result |
|---|---:|
| Nodes / surface vertices | 13,264 / 375,478 |
| Retained individual leaves / instances | 63,029 / 63,029 |
| Native build, warm-up | 75.196 ms |
| Native build, five measured samples | 73.883, 71.636, 71.553, 72.095, 71.672 ms |
| Native build median | 71.672 ms |
| Peak RSS (runner VmHWM) | 31,528 kB |
| Browser pre-cull / retained instances | 64,454 / 63,029 |
| Browser unique marked twig runs | 4,958 |
| Browser complete / truncation flags | true / all false |
| Transferred surface buffers | 17,782,512 bytes |
| Transferred foliage buffers, including matrices | 4,034,216 bytes |
| Retained foliage AABB span X / Z | 12.8531 / 14.1841 m |
| Retained foliage bottom / top above root | 7.3532 / 23.3674 m |

Node count is not biological branch count. Existing runner calls instances “leaves”; that equivalence holds only for this one-blade baseline. Biological branch orders, DBH and actual leaf area are unmeasured here and belong to task 2. Field occupancy was not requested and is not an anatomical proxy. Giant supernatural presets were not benchmarked in this research task; later cross-template gates must report them honestly.

## Captures and inspection

`npm run wasm:build` exited 0 and reproduced generated metadata without tracked changes. A disposable headless Playwright script followed `tests/browser/integration.mjs`: Vite on port 5187, import `TreeEngine`, build `ORDINARY` requesting surface/foliage/structure, materialize through `src/browser/three.ts`, use the neutral `createStage`. Browser: HeadlessChrome 151.0.0.0; renderer: ANGLE Vulkan SwiftShader Device (Subzero), software. No visible windows or consent-bypass flags. Software timing is not hardware GPU performance.

Pinned whole/bare captures: 1200×1000, DPR 1; stage FOV 38°, direction normalized `(0.62,0.28,1)`, margin 1.3; `stage.frame(24)` fits the generated subject, grey clay, sky illumination only, optional key/fill off. The bare view toggles the canopy's visibility without rebuilding or reframing. The stage's small scale marker is not a second specimen.

Inspected files: `.refs/fn9/baseline-whole.png`, `baseline-bare.png`, `baseline-element.png`; raw metadata: `baseline-browser.json`. Capture script remains in `.flow/tmp/capture.mjs` for local reproduction. The element close-up uses the same generated element in a separate neutral scene: perspective FOV 38°, aspect 1.2, near/far .001/10, camera `(0.12,0.10,0.25)` looking at `(0,0.06,0)`, grey double-sided material, hemisphere light. An initial stage-framed element was too small to inspect and was replaced with this dedicated close-up; it is not counted as a successful detail observation.

Whole/bare inspection: sparse, upward-drawn crown, long sweeping scaffold axes, a conspicuously bent/flared lower trunk and large empty regions toward the upper crown. Element inspection: smooth polygonal pointed blade with no oak lobes, petiole segment or needle cross-section. Whole views establish architectural mismatches; they do not prove attachment continuity or close-up junction quality. An attached-twig detail remains unassessed in this baseline, explicitly required for final QA. No owner approval was requested or received.

## Capability matrix — smallest required changes

| Surface | Oregon white oak | Norway spruce | Implementation boundary |
|---|---|---|---|
| Crown and axes | Fail: too narrow/upward and sparse versus rounded spreading crown; insufficient crooked low scaffold character | Fail: no persistent dominant leader, primary tiers or hanging secondary curtains | Extend existing branch laws for oak; a narrowly scoped leader/tier habit for spruce if current colonization cannot preserve it. Envelope tuning alone cannot establish branch habit. |
| Natural orientation | Ordinary defaults contain writhe amplitude .07 and spiral rate 1.2 | Same problem; especially incompatible with clear spruce leader | Separate authored effects, ordinary off; retain natural lean and gravity. Preserve explicit supernatural template intent. |
| Foliage element | Fail: rounded lobes and petiole missing | Fail: no tapered four-sided needle | Extend procedural element generation with only these two shapes; no generic botanical grammar. |
| Attachment | Alternate individual blades needed; existing local twig frames are reusable, not a wholly missing system | Single peg-based needles around twig, upper side forward bias; no fascicles | Extend existing local placement frames and supply species-specific connection/orientation. Existing canopy-wide outward/upward offsets must not override local anatomy. |
| Crown gaps | Baseline is too sparse and string-like | Requires tier gaps and hanging curtains, not a spherical shell | Calibrate actual retained foliage with structure; shell occupancy is not anatomy. |
| Taper and junctions | Long smooth whip-like ends differ from repeated crooked subdivisions | Need narrow leader/branchlet taper without blunt cuts | Existing radius/surface machinery is reusable. No specific seam defect is confirmed at whole-tree resolution; inspect junction and terminal close-ups before claiming a fix is necessary or complete. |
| Measurements | Biological axes/DBH/actual foliage area missing | Same, plus needles must count as units | Extend native measurements, not runtime JSON rules. Preserve per-seed failures and unknown quantities. |

Similar-code search: reuse `Tree`, `NodeKind`, branch-run metadata and diagnostics; extend `foliage/element.rs` and local frames in `foliage/placement.rs`; follow the existing native measure example and neutral browser materialization. Relevant junction-containment memory was read: whole-ring containment must be judged against the actual drawn surface, not the centre alone. No geometry edits or speculative fixes are part of this research commit.

Profiles and fixed/fresh protocol are frozen in `profiles.json`. This baseline is an inspected discrepancy report, not the final 24-seed-per-species validation; missing attachment detail and unknown biological totals prevent an unqualified fidelity pass now.
