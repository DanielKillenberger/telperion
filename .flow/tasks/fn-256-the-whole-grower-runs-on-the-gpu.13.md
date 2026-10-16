# fn-256-the-whole-grower-runs-on-the-gpu.13 The light kernels at native speed

## Description
COUNCIL round 6 E3: the device light code emulates f64 in software and sweeps each direction in one workgroup; native f32, a parallel schedule of the same bilinear recurrence, the lattice over the crown's occupied box, the combine fused into the read; CPU reference matched within the lineage tolerance. Built on fn-264's lit presets (merge branch fn-264-every-species-shades-and-answers-light first).

## Acceptance
Lit Telperion's light phase under 100 ms GPU (755 ms today); trees within lineage tolerance and stills not told apart; refusals unchanged; gate and review.

## Done summary
The light runs in native f32: box, corners, quanta and densities exact to the CPU's by construction; the depth recurrence in skewed 16x16 tiles of 16 layers a dispatch from the highest node a leaf with area reaches; the combine fused into the read; the software f64 light, column records, `lights` limit, `LightRanges` and the VM's `Floor` op deleted. Lit Telperion's light phase 746.7 -> 26.0 ms GPU (Laurelin 671.7 -> 23.5; the 40-year presets 13-26 -> 2-6 ms); warm builds Telperion about 2,022 -> 1,252 ms, Laurelin 1,840 -> 1,156 ms. Captures within 2.4e-7 of the CPU; young trees match by lineage; stills of every preset viewed, not told apart; refusals' names unchanged (PROOF.md, DESIGN.md section 19).

Merged fn-264 (clean) at 7a16fc809 first. The occupied box saved no layer on the busy oak and Telperion captures: their boxes are already occupied.

Follow-ups: `the_roofline_is_measured` and `the_executor_is_measured` no longer run on lit presets; `build-wasm.mjs` ignores `CARGO_TARGET_DIR` (FRICTION.md).

## Evidence
- Commits: 78cb1605dd262ab3a3cb3ecdb00ca518dbb57cb1, d2b2182d166122b7f7095f7c8324f1694b869b73, 77e3880d02ef9a4434bb592fa1b264c7ac9ee402, a05da09a8572f2fca013f34129657d1e5ae3c5d8
- Tests: cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, cargo test --profile ci -p telperion-render --no-fail-fast, npm test, cargo test --profile ci -p telperion-render the_light_phase_is_timed -- --ignored
- PRs:
