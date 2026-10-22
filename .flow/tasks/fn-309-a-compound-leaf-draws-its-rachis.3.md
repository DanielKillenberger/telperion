---
satisfies: [R3, R4]
---
# fn-309-a-compound-leaf-draws-its-rachis.3 Proof: cost, stills of frame 526, and the owner's packet

## Description
The measurements the owner and the host read: `build_time` base vs change, frame 526 base and change side by side, the class table across every R3 frame, and `npm test`.

**Size:** S
**Files:** `.flow/evidence/fn-309-a-compound-leaf-draws-its-rachis/{RESULT.md,stills-526.jpg}` (raw output under `raw/`, gitignored)
**Touches:** [.flow/evidence/fn-309-a-compound-leaf-draws-its-rachis/**]

### Approach
- Cost: `BUILD_FRAME=oregon-white-oak,date-palm,0.34 BUILD_AGE=40 BUILD_SEED=7 cargo test --profile ci -p telperion-render build_time -- --ignored --nocapture` and `BUILD_PRESETS=date-palm BUILD_AGE=40` on the base (a separate checkout of `e2c035e51` with its own `target/`, per AGENTS.md, or the base binary built before task 1's edits) and on the change, each through `telperion-heavy 8 --threads 2 --gpu <n>`; report warm medians. A rise over 5 % is reported to the host.
- Stills: frame 526 (oak to palm at 40, seed 7, t = 0.34) at the hero pose (`telperion_render::hero_pose`, as `examples/space_walk.rs` frames a walk, but dressed: leaves on), base and change, composed side by side into one JPEG. Prefer an existing stills example that draws a blended family with foliage; if none, a small ignored test or example that builds `blend::families` and renders through `Renderer`. View at most four images. Say plainly in RESULT.md whether the tufts read well or wrong.
- Run `npm ci` then `npm test` once, inside `telperion-heavy`, after the gate.

### Investigation targets
**Required**:
- `crates/telperion-render/src/generation/build_time_tests.rs:8,100-145` — `BUILD_FRAME`
- `crates/telperion-render/examples/space_walk.rs` — hero pose framing for walks

### Key context
- AGENTS.md token budget: read summaries, at most four images per capture; raw output stays in gitignored `raw/`.

## Acceptance
- [ ] `build_time` on the t = 0.34 frame and the palm at 40 measured base vs change, in RESULT.md.
- [ ] Frame 526 base and change side by side committed as evidence, with a plain-language read of the tufts.
- [ ] RESULT.md carries the class table at every R3 frame (base and change).
- [ ] `npm test` green after `npm ci`, inside `telperion-heavy`.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
