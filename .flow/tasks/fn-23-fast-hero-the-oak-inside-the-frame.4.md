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
The oak hero renders its vegetation pass including selection at a p50 of
**1.750 ms** at 1600 by 1000 on the RTX 3080 through the native headless
target, valid verdict, against R1's budget of 2 ms. R1 is met. The number is
now attributable: the selection compute pass carries its own timestamp pair and
costs 0.057 ms of it, the vegetation pass the other 1.693 ms.

Three sessions, all valid, all first-run - none came back contended, so none
was repeated. Oak plain 1.750 ms together; oak orbit 1.912 ms with a
frame-to-frame wall p50 of 2.17 ms and a worst of 2.25 ms; spruce plain
32.619 ms, recorded and not gated. Against fn-22 on the same machine: the oak's
vegetation pass went 17.87 ms to 1.69 ms, the spruce's 94.18 ms to 31.88 ms.
`.flow/evidence/fn23/REPORT.md` carries the protocol, the tables and the
verdicts in the fn-22 report's shape, with the owner's slot left empty for
task 6.

Two commits.

The first splits `timing.rs` before growing it: the protocol stays, the record
moves to `timing/report.rs`, and every renderer source file is now under the
400-line rule (the largest is `select.rs` at 397). Then the record gains what
this task measures. A second timestamp pair brackets the selection compute
pass - a pass-boundary write like the render one, base timestamp feature only,
so a browser can run the same session. `selection_p50_ms` and
`selection_p95_ms` sit beside the existing pair, and `total_p50_ms` /
`total_p95_ms` add the two passes **frame by frame** before ranking them, so
the total is a frame's own cost rather than a sum of two separately ranked
tails. `levels` records each level's tolerance in metres and its median
instance count over the measured frames, the unseen bucket last with a `null`
deviation because it approximated nothing. `camera::orbit_pose` turns the hero
pose about the vertical axis through its subject, keeping elevation and
distance exactly; `measure_orbit` runs the protocol around one full turn and
adds `wall_p50_ms`, `wall_p95_ms`, `wall_max_ms`. `headless` gains `--orbit`.

The second commit is the evidence.

Four decisions worth naming.

**Every new field is absent on anything but a valid verdict, and one of them is
absent more often than that.** A selection pair that no pass wrote resolves to
a pair of zeroes, and a zero is not a duration - so `with_selection` refuses it
on its own account even when the frame around it was measured cleanly. That
rule has teeth because of the next one.

**A timed frame opens the selection pass even when it selects nothing.** The
bare and leaf views do not select, and neither does a crown of no leaves; a
resolve that waited on a query no pass ever wrote would wait on the device for
good. So `Select::dispatch` opens an empty compute pass when timestamps are
given, and the pair is always written.

**`Renderer::draw` did not change shape.** The second pair needed a
`ComputePassTimestampWrites` down the same call, which would have changed
`draw`'s parameter list and forced an edit to `web.rs`, which this task does not
own. Instead `draw` delegates to a private `draw_with` and a new
`draw_timed(.., vegetation, selection)` carries both pairs. `web.rs` compiles
untouched - checked against `wasm32-unknown-unknown` - and task 5 has the entry
point it needs for the browser orbit.

**The per-level counts are read back inside a timing session and nowhere else.**
`Renderer::level_counts()` keeps its type and its callers; `level_deviations()`
is new and comes off the mesh at submit rather than off the GPU, which kept
`select.rs` inside the line rule.

One deviation from the task's declared Touches, named rather than buried:
`crates/telperion-render/src/foliage.rs` is edited. It is the only path from
`Renderer::draw` to `Select::dispatch`, so the compute pair cannot reach the
pass without it. The change is a pure forward - `dispatch` gains the
`timestamps` parameter and passes `view == View::Whole` through as a bool - with
no behaviour of its own. The task's Files and Touches lists appear simply to
have missed the plumbing file.

Tests: `orbit_pose` is pinned over 24 steps of a turn to keep the eye's height
and its distance to within 1e-9, to come back to its start after a full
revolution, and to move more than its stand-off distance in a quarter turn; a
full record is asserted to carry every new key including the bucket's `null`
deviation, and the total is checked to be the frame's own sum; every invalid
verdict is offered selection, levels and wall numbers and shown to take none of
them; a selection series of zeroes and a level readback that disagrees with the
ladder are both refused on a valid frame; and a device test times both passes on
a real adapter, checks the plain session's per-level medians sum to the whole
crown with the bucket's deviation absent, and checks an orbit session's worst
wall frame is at or above its median.

Not done here, by scope: no browser orbit (task 5), no stills and no owner
verdict (task 6), no levers of any kind - the gate passed, so none were needed.

Follow-ups worth a later spec, not built here: the native orbit's wall time
includes the per-frame timestamp readback, so it is the loop's pace and not a
frame-rate claim - the browser's own loop in task 5 is where R2 is judged. And
fn-22's parked spruce browser-native gap is not explained by selection: 0.744 ms
of a 32.6 ms frame is two per cent, nowhere near thirty milliseconds.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 1e42b0969fb8344b067b47a04c76cfb410af095f, c9918333d03f782ca354b4323506de9b0abc3dbd
- Tests: baseline: green via handoff (verified at 407fc92 by fn-23-fast-hero-the-oak-inside-the-frame.3), cargo fmt --all, cargo clippy --release --workspace --all-targets (clean), cargo check --release -p telperion-render --target wasm32-unknown-unknown (web.rs unchanged, compiles), cargo test --release --workspace (suite_rc=0, 29 test-result groups ok, 0 failed), flowctl gate classify --base $BASE_COMMIT -> FULL (code .rs touched); full gates run, receipt 1e42b096-unittest.json, cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --size 1600x1000 --timing .flow/evidence/fn23/oak-native-timing.json (valid, together p50 1.7495 ms), cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --size 1600x1000 --orbit --timing .flow/evidence/fn23/oak-native-orbit.json (valid, wall p50 2.17 ms), cargo run --release -p telperion-render --example headless -- --preset norway-spruce --seed 7 --size 1600x1000 --timing .flow/evidence/fn23/spruce-native-timing.json (valid, together p50 32.6188 ms)
- PRs: