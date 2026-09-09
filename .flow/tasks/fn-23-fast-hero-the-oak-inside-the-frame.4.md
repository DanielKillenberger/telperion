---
satisfies: [R1]
---
# fn-23-fast-hero-the-oak-inside-the-frame.4 The selection pass timed, the orbit session, and the native gate

## Description
Make the cost attributable and run the R1 gate. Bracket the compute pass with its own timestamp pair, read back per-level counts inside a timing session, add the orbit session to the protocol and the headless command, split the timing module so it stays under the line rule, then measure the oak on the RTX 3080 and record the verdict.

**Size:** M
**Files:** `crates/telperion-render/src/timing.rs`, `crates/telperion-render/src/timing/report.rs` (new, from the split), `crates/telperion-render/src/select.rs`, `crates/telperion-render/src/lib.rs`, `crates/telperion-render/src/camera.rs`, `crates/telperion-render/src/headless.rs`, `crates/telperion-render/examples/headless.rs`, `crates/telperion-render/tests/timing.rs`, `.flow/evidence/fn23/oak-native-timing.json`, `.flow/evidence/fn23/oak-native-orbit.json`, `.flow/evidence/fn23/spruce-native-timing.json`
**Touches:** [crates/telperion-render/src/timing.rs, crates/telperion-render/src/timing/**, crates/telperion-render/src/select.rs, crates/telperion-render/src/lib.rs, crates/telperion-render/src/camera.rs, crates/telperion-render/src/headless.rs, crates/telperion-render/examples/headless.rs, crates/telperion-render/tests/timing.rs, .flow/evidence/fn23/**]

### Approach
- Split `timing.rs` (413 lines) into the session protocol and the report before adding anything; keep public names stable so `web.rs` and the headless example compile unchanged.
- Second timestamp pair around the compute pass (`ComputePassTimestampWrites`, same feature as today's render-pass pair at `timing.rs:263,293`); the session resolves both pairs per frame and reports `selection_p50_ms` and `selection_p95_ms` beside the existing fields, with the same verdict rules. The vegetation number in R1 is the vegetation pass plus selection; report both and the sum.
- Per-level counts: inside a timing session only, read the counters back once per measured frame with the same mapping path the timestamps use, and record the per-level median counts and the none bucket as `levels: [{deviation_m, instances_p50}]`. Not in the plain frame path.
- Orbit session: `camera::orbit_pose(bounds, t)` for t in 0..1 rotates the hero pose one full turn at the hero elevation and distance; the session runs conditioning and warmup at t=0, then the measured frames advance t uniformly, recording GPU numbers as before plus frame-to-frame wall time p50, p95 and maximum. Headless gains `--orbit`, writing the orbit record to the `--timing` path.
- Run the gate: oak seed 7, 1600 by 1000, whole, hero pose, native, idle GPU: a plain session into `oak-native-timing.json` and an orbit session into `oak-native-orbit.json`; spruce plain into `spruce-native-timing.json`, recorded and not gated. If oak's p50 is above 2 ms with every level and the none bucket in place, stop with NEEDS_HUMAN and the number in the task file per R1's stop rule; do not add levers in this task.
- Write `.flow/evidence/fn23/REPORT.md` in the shape of the fn-22 report: protocol, measured table with selection and vegetation columns, per-level counts, orbit wall times, verdicts, and a slot for the owner's verdict that task 6 fills.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/timing.rs` — the whole module; the Report and `to_json` at :119-224, the query set and resolve at :263-293
- `crates/telperion-render/src/select.rs` — the pass to bracket (task 3)
- `crates/telperion-render/src/camera.rs:61-111` — `hero_pose`, the pose the orbit turns around
- `crates/telperion-render/examples/headless.rs:133-161` — `time()` and the JSON write
- `.flow/evidence/fn22/REPORT.md` — the report shape to follow

**Optional** (reference as needed):
- `crates/telperion-render/tests/timing.rs` — existing session tests to extend
- `crates/telperion-render/src/web.rs:296-320` — the browser measure loop that must keep compiling

### Key context
- An invalid session carries no number that reads as a pass; the new fields follow the same rule and are absent, not zero, on an invalid verdict.
- Budget: three plain sessions and one orbit session on an idle desktop; no stills in this task.

## Acceptance
- [ ] The timing record adds `selection_p50_ms`, `selection_p95_ms`, `levels` with per-level deviation and median instance count including the none bucket, and for an orbit session `wall_p50_ms`, `wall_p95_ms`, `wall_max_ms`; all absent on a non-valid verdict
- [ ] `--orbit` runs one full turn around the hero pose over the measured frames and writes the orbit record; a plain session is unchanged in protocol and its existing fields are byte-compatible with fn-22's records
- [ ] No file under `crates/telperion-render/src` exceeds 400 lines after the split, and `web.rs` and the headless example compile without changes to their public calls
- [ ] `.flow/evidence/fn23/oak-native-timing.json` is valid with vegetation plus selection p50 at or under 2 ms on the RTX 3080 at 1600 by 1000, or the task stops with NEEDS_HUMAN and the measured number
- [ ] `.flow/evidence/fn23/spruce-native-timing.json` is recorded with its verdict and not gated
- [ ] `.flow/evidence/fn23/REPORT.md` follows the fn-22 report shape with the new columns and an empty owner verdict slot
- [ ] `cargo test --release -p telperion-render` passes or skips without an adapter

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
