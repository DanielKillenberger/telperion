# The harness holds 60 fps on every shipped tree

## Goal

The owner (2026-10-09), after the release branch's harness: build time is acceptable for now, "It's the framerate after." Measured natively on the RTX 3080 at 1920×1080, 4x MSAA (fn-198's performance record): Telperion 8 fps whole and 13 close, Laurelin 11 and 16, the oak 28 close, the spruce 48-52; every slow case GPU-bound. Three costs: the leaf and flower draw of dense crowns (Telperion 9.05M leaves and 10.4M flowers, ~9 instances a pixel), the wood's shaded pass at close views (~30 ms), and sun-side work recomputed every frame; the harness also redrew every frame when nothing changed. Host (2026-10-09): release blocker for fn-237. Drawing only; generation unchanged.

## Decisions

- Host design (2026-10-09): leaves and flowers thinned by screen size with area kept (deterministic share keyed by each organ's stored position words, continuous, inside the existing selection pass); sun-side work only when the tree, light, caster membership or stride changes; no redraw when nothing changed (a pending change survives skipped frames); cached passes timed as cached (zero, flagged); the wood's shaded pass measured, then fixed by its cause.
- Host decision (2026-10-09): prefilter the bark relief to the pixel footprint (fade sub-2 px octaves, slope variance into roughness, one cell per pixel) instead of supersampling 9-25×; judge against a 4× ground truth (mean ≤ 3.0). The prefilter missed it: its error is per-pixel noise, not bias, which no expected-value term can remove.
- Owner decision (2026-10-09): on the wood, "b and c sound sgood": (b) the prefiltered bark ships, its measured distance to the truth the accepted picture, bark tests re-pinned to it plus 0.5 mean; (c) texture bark computed at build, mipmapped, after the release (fn-247). On the Two Trees' leaves, "2a is gud": larger leaves in their own specs.
- Leaf thinning threshold 2 px, fitted on `examples/crown_truth`.
- Owner decision (2026-10-09) on the floret card below 4 px, after the labelled comparison (https://claude.ai/artifact/8WwaynDsif2JekPvHDDtDi): "I see almost no difference from reference even vs the card implementation." Ships past the crown bound (+4.43 against 3.0); `tests/thinning.rs` pins it.
- The Two Trees stay below 60 fps for now (owner).

## Acceptance Criteria

- **R1:** per pass, per species, whole and close, before; the wood's shaded pass with bark relief off and on — met.
- **R2:** the four design points built; one pipeline; the picture changes by degree — met.
- **R3:** every shipped species at 60 fps or more whole and close natively (best of 3), stills before and after viewed, idle frames free, CPU and device builds unchanged — met except the Two Trees (owner accepted).
- **R4:** gate and npm test green — met (the load-sensitive cold-compile test passed alone).

## Result

Final, 1920×1080 4x MSAA, best of 3, fps whole → close (R1 before in brackets): ordinary 1437 / 1001 (842 / 656); oak 234 / 72 (71 / 28); spruce 122 / 130 (47 / 52); beech 589 / 330 (218 / 106); birch 486 / 230 (302 / 188); date palm 1529 / 1260 (713 / 361); Telperion 50 / 34 (8 / 12); Laurelin 56 / 52 (11 / 16). Telperion close: vegetation 25.0 ms (wood ~9, leaves and florets ~16). At 2× the prefiltered oak trunk loses most of its furrows (accepted; restored by texture bark). Tests: `tests/sun_cache.rs`, `tests/timing.rs`, `redraw.rs`, `src/browser/render.test.ts`, `tests/thinning.rs`, `card.rs`, re-pinned `bark_resolution.rs` and `bark_distance.rs`; probes `examples/frames.rs`, `bark_truth.rs`, `crown_truth.rs`. npm test 154. Impl-review SHIP after four fixes.

## Open

- Cluster cards for the Two Trees' remaining floret cost (host).
- Texture bark (owner's (c)): fn-247.

## Friction

- The Two Trees' baseline took ten minutes -> view filter added to `examples/frames.rs`.
- Command guard refuses redirects into a variable path -> local setup, reported only.
- The wood's same-picture claim could only be checked after building -> a host-stated picture tolerance before dispatch (proposed).
- The bark truth had to be built before judging the prefilter -> a shared bark-view fixture list (proposed).
- Other sessions loaded the machine during timing -> fn-185's "machine busy" marker; local, reported only.
- The crown truth's absolute bound could not be met -> measure the unthinned baseline first; the fit now has a T=0 column.
- No numpy; ~5 GB raw stills only removable by hand -> local setup, reported only.
- The crown bound was spent when the card arrived -> fit mechanisms against one shared budget (proposed).
- Cold-compile timing test fails under load -> run serially or measure CPU time (proposed).

## Evidence

- `.flow/evidence/fn-246-the-harness-holds-60-fps-on-every/FPS.md` (measurements, decisions and final tables; cited by the bark and thinning tests and `card.rs`)
- `.flow/evidence/fn-246-the-harness-holds-60-fps-on-every/FRICTION.md` (cited by `hang_tests.rs` and `docs/gpu-tests.md`)
