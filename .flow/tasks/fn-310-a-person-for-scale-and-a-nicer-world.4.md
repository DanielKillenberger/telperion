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
The +1.7 s start cost is not real: it was the state of the NVIDIA driver's shader disk cache. Base and branch alternated under equal cache state give `fresh_ms` within noise, cold (cache off, 7 runs each: 8,109 against 8,245 ms, ranges overlapping) and warm (own primed cache, 3 runs each: 377 against 374 ms). The default cache, `~/.cache/nvidia/GLCache`, is at its 1 GiB ceiling and takes no new entries, so the base's older entries hit and the branch's new modules miss on every run (re-measured: base 5.8-5.9 s, branch 7.7-7.9 s, no speed-up on repeat). Following the task's order, the remaining steps reduce to stating this: no move out of `common.wgsl`, no per-module timing, no code change, so drawing is unchanged and there is no cause for a guard. COST.md carries the diagnosis and the final numbers (about 8.1 s cold, 0.38 s warm, same on both; frame time is task 3's table).

- Follow-ups (not this spec's): any `fresh_ms` on this machine with the default cache measures the cache, so the release bar's start number (fn-237 R0a) needs a stated cache state, for example `__GL_SHADER_DISK_CACHE=0`. A cold start is 8 s on the base too, which a first visit pays. The full cache is a local setup problem on the owner's machine (clear it or raise `__GL_SHADER_DISK_CACHE_SIZE`); it was neither moved nor cleared.
- Friction: one entry in FRICTION.md (the full cache; dcg refusing a heredoc script to a variable path).
- Raw rows and the script: `raw/frames/ab.sh`, `cold1.jsonl`, `cold2.jsonl`, `warm1.jsonl` (ignored).
- Baseline: green via the handoff from 13328e77; no code changed. Feature map: no route changed.

Tier: session at default effort (model: claude-opus-5-5)

stage: impl-review - skipped(policy: risk - the diff is two evidence Markdown files, no code)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: ae335683c1453421ee040b6e8fe6d8c7316cd809
- Tests: frames oregon-white-oak full whole, base/branch alternated, cold and warm cache (raw/frames/ab.sh), GATE_SKIPPED:render-scene:docs-only - cumulative diff classified tier-B (no executable paths touched), GATE_SKIPPED:render-tests:docs-only - cumulative diff classified tier-B (no executable paths touched)
- PRs: