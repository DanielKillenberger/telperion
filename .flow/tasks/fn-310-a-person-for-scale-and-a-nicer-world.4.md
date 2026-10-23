---
satisfies: [R4]
---
# fn-310-a-person-for-scale-and-a-nicer-world.4 Find and remove the 1.7 s renderer start-up cost

## Description
Task .3 measured the renderer's start (`fresh_ms`, frames probe) at +1.66 to +1.68 s (+24 to 28 %) against the base 201288747 on both trees, outside noise; `warm_ms` and the vegetation pass did not move (`.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/COST.md`). Host ruling (2026-10-23): diagnose before the gate, because the release bar (fn-237 R0a) times what the site and the harness pay at start.

**Size:** M
**Files:** `crates/telperion-render/src/shaders/common.wgsl`, `scene.wgsl`, `sky.wgsl`, `crates/telperion-render/src/shaders.rs`, a test under `crates/telperion-render/`, `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/COST.md`
**Touches:** [crates/telperion-render/src/shaders/**, crates/telperion-render/src/shaders.rs, crates/telperion-render/tests/**, crates/telperion-render/src/scene.rs, crates/telperion-render/src/scene/**, .flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/**]

### Approach (host's order)
1. Cold-cache control: base and branch alternated under the same cache state (both cold, or both warm; say which and how the driver's and any wgpu pipeline cache were cleared or kept), to confirm the +1.7 s is real. If it vanishes under equal cache state, record that, and the remaining steps reduce to stating it.
2. Move the new sky and ground functions (`sky_radiance`, the value noise and `ground_field`, and anything only they use) out of `common.wgsl` into the two shaders that use them (`sky.wgsl`, `scene.wgsl`), then re-measure `fresh_ms`.
3. If the time remains, measure each shader module's compile (device logs, `wgpu` timestamps or timing around each `create_shader_module` / pipeline creation), base against branch, and name the module that grew.
- Keep whatever removes the cost, and guard it with a test that catches this cause: a start-time guard, or a check of which modules include the sky and ground code (for example, that only the scene and sky modules contain `ground_field`).
- If the cost is inherent to the new look, stop: record the measured number and the module, and return NEEDS_HUMAN.

### Investigation targets
**Required:**
- `crates/telperion-render/src/shaders.rs` - how the prelude is composed into each module
- `crates/telperion-render/examples/frames.rs` and `docs/gpu-tests.md:44-54` - the probe and `fresh_ms`
- `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/COST.md` and `raw/frames/frames.sh` - task .3's method and base build (scratch `base/` export of 201288747 with its own target dir)

### Key context
- Drawing must not change: the same frames before and after the move (the environment tests from task .1 hold it).

## Acceptance
- [ ] Base and branch `fresh_ms` measured alternately under the same, stated cache state; whether the +1.7 s is real is recorded with numbers
- [ ] The new sky and ground functions live only in the shaders that use them, and `fresh_ms` is re-measured after the move
- [ ] If the cost remains, the per-module compile time, base against branch, names the module that grew
- [ ] Whatever removes the cost is kept and guarded by a test that fails if the cause returns
- [ ] Task .1's environment tests and the focused render tests stay green; drawing is unchanged
- [ ] COST.md updated with the diagnosis and the final start and frame numbers

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
