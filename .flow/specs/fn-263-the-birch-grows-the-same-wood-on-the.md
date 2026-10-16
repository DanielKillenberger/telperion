# The birch grows the same wood on the device as on the CPU

## Goal

fn-254's checks (2026-10-11): the silver birch's device-grown wood stood up to 0.219 m off the CPU's, while the beech, spruce, Telperion and Laurelin agreed within their measured bounds; the birch was held to its own wider bound and exempted from the leaf-area test. fn-256 puts every axis on the device, so an executor gap left here would become the shipped birch. Find the first stage that differs, fix the wrong executor, and remove the birch's exemptions. Release item (fn-237).

## Decisions

- The wrong executor was the CPU: its sag turned a phytomer over a length other than the one it is laid at; the fix belongs in the shared geometry (`geometry/sagged.rs`). Nothing changed on the device.
- The CPU birch changes; under the generator-evolution rule this is an intended correctness change.

## Acceptance Criteria

- **R1:** standalone test growing the birch at 30, seed 42 on both executors, reporting the first stage and lineage keys apart; cause named against the code — met (`birch_tests::the_birch_grows_the_same_wood_on_the_device_as_on_the_cpu`; first stage apart: sag, root 333's first axis, lineage 17581127258315254970, PA 9).
- **R2:** cause fixed; both executors within the other presets' bounds; the birch's own bound and leaf exemption removed — met.
- **R3:** birch stills whole/mid/close beside the current ones, viewed; gate and npm test green — met.

## Result

Cause: `geometry/sagged.rs` took `length = state.internode * phytomer.scale` (before the node's own internode draw), while `Layer::step` and the device's `lay.wgsl` use the varied length (fn-222). The birch is the only preset whose internodes vary (`internode_spread` 0.2-0.7), so only it parted. `sag_angle` now takes the laid length. Preset agreement, birch 40/42: tips 0.219 m → 1.26e-4 m most, 1.0e-2 → 3.55e-6 m RMS; leaves apart 42,546 → 0 of 60,084. Other presets unchanged. Stills (before/after, viewed): RMSE whole 3.9 %, mid 10.3 %, close 19.1 %, strands sitting in new places with the same character; no perceptible regression. Gate green per package; `npm test` 155.

## Open

None.

## Friction

- No readback of the device's per-phytomer sag inputs; temporary shader patches needed -> test-only device probe of a named phytomer's step inputs suggested.
- Doctests hit the tmpfs `/tmp` quota -> local setup, not specced.
