# fn-23-fast-hero-the-oak-inside-the-frame.1 The probe: every leaf as a two-triangle quad, and the number that decides the spec

## Description
Answer the Early proof point before any level exists. Add a `--level quad` switch to the headless example that makes the renderer draw every foliage instance as a two-triangle quad spanning the element's blade extent, run the fn-22 timing protocol on the oak at the hero pose, and record the number. This is a measurement hook, not a feature; task 3 replaces the `quad` value with numeric levels.

**Size:** S
**Files:** `crates/telperion-render/examples/headless.rs`, `crates/telperion-render/src/foliage.rs`, `crates/telperion-render/src/lib.rs`, `.flow/evidence/fn23/probe-quad-timing.json`, `.flow/evidence/fn23/probe-full-timing.json`
**Touches:** [crates/telperion-render/examples/headless.rs, crates/telperion-render/src/foliage.rs, crates/telperion-render/src/lib.rs, crates/telperion-render/src/headless.rs, .flow/evidence/fn23/**]

### Approach
- Parse the new flag beside the existing ones at `crates/telperion-render/examples/headless.rs:27-78`; thread it to the renderer as an option on submission, not a global.
- In `Foliage::submit` (`crates/telperion-render/src/foliage.rs:140-176`) build the stand-in from the element's positions: the axis-aligned extent in the blade plane (x and y, blade face +Z per the element contract) as four vertices and six indices, with one normal along +Z; keep the instance buffer untouched so 555 thousand placements draw as before.
- Run the timing session twice at 1600 by 1000, seed 7, whole view, hero pose: once with `--level quad`, once without, both native, both saved. The comparison, not a single number, is the finding.
- Write a five-line `.flow/evidence/fn23/PROBE.md` stating both p50 values, the verdicts, and the conclusion: vertex-bound (quad well under 2 ms) or fill-bound (quad near the full number). If fill-bound, stop and report NEEDS_HUMAN with the numbers; the spec's mechanism is then wrong and the owner decides.
- The stand-in must not change `FrameStats.instances` semantics; triangles drawn will drop by construction.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/foliage.rs:74-216` — element and instance upload, the single draw at :180-205
- `crates/telperion-render/examples/headless.rs:27-78` — flag parsing and the `USAGE` text; :133-161 timing and JSON write
- `crates/telperion-render/src/lib.rs:101-114` — `Renderer::submit` and the fit check call
- `.flow/evidence/fn22/oak-native-timing.json` — the baseline record shape and number

**Optional** (reference as needed):
- `crates/telperion-core/src/foliage/element.rs:63-70` — the element's axis and blade-face convention
- `.flow/evidence/fn22/REPORT.md` — the resolution probe section, the model for how a probe is reported

### Key context
- fn-22 report claims the oak is raster-bound on large leaves; the triangle count says primitive-bound. This probe settles it. Do not optimise anything else in this task.
- Budget: two timing runs and one short markdown file. No stills.

## Acceptance
- [ ] `cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --out /tmp/oak.png --level quad --timing .flow/evidence/fn23/probe-quad-timing.json` exits zero and the record's verdict is valid
- [ ] The same command without `--level` writes `.flow/evidence/fn23/probe-full-timing.json` with a valid verdict and a p50 within 10 percent of fn-22's 17.87 ms
- [ ] `.flow/evidence/fn23/PROBE.md` states both p50 values and one conclusion: vertex-bound or fill-bound
- [ ] With `--level quad`, `stats().instances` equals the submitted instance count and `triangles` equals 2 times that count plus wood triangles
- [ ] An unknown `--level` value exits non-zero naming the value, and the flag is absent from the browser surface
- [ ] `cargo test --release -p telperion-render` passes or skips without an adapter
- [ ] If the conclusion is fill-bound: task stops with NEEDS_HUMAN and the two numbers in the task file; tasks 2 onward do not start

## Done summary
The probe is in and the answer is vertex-bound. `--level quad` makes the headless
example submit every foliage instance as a two-triangle quad spanning the
element's blade extent, threaded through a new `Renderer::submit_at` so the level
travels with the tree rather than sitting on the renderer; the plain `submit`
delegates at the full element, which is why the browser surface gains nothing.

The oak at seed 7, hero pose, 1600x1000 on the RTX 3080: 18.08 ms whole,
1.58 ms as quads, both verdicts valid, both records under `.flow/evidence/fn23/`.
Same 555,204 instances and the same coverage in both; only the triangles per leaf
change, 268 down to 2. The frame is spent on primitives, not on fill, so the
spec's levels are the right mechanism and tasks 2 onward proceed. fn-22's report
reads the oak as raster-bound on large leaves; that reading is wrong, and
`.flow/evidence/fn23/PROBE.md` says so with both numbers.

Two deviations to note. Both sessions ran with `--size 1600x1000` added to the
AC's command string, because fn-22's 17.87 ms was measured at that size and the
AC's own within-10-percent clause is only meaningful against it; the full run
came in at 18.085 ms, 1.2 percent above. And the unknown-`--level` error case is
covered by a unit test on `Level::from_id` plus a recorded CLI run (exit 1,
`unknown level "3"; one of quad`) rather than a test of the example binary, which
this crate has no rig for.

Triangle arithmetic checks out against fn-22 exactly: 6,391,128 drawn under quad
is 5,280,080 wood plus 2 x 555,204 leaves plus the room's 640, and the full run's
154,075,392 reproduces fn-22's number to the triangle.

Follow-up, not built: the stand-in ignores the blade outline and would thin the
crown visibly, so it is a measurement hook only. Task 3 replaces the `quad` value
with numeric levels, and `Level::NAMES` is the one place that has to change.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 4917a32032ea8339b047623c98acf33527805dbb
- Tests: cargo test --release --workspace (110 passed, 0 failed; GPU-gated suites ran on the RTX 3080), cargo fmt --all --check, cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --out /tmp/oak-quad.png --size 1600x1000 --level quad --timing .flow/evidence/fn23/probe-quad-timing.json (valid, p50 1.577 ms), cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --out /tmp/oak-full.png --size 1600x1000 --timing .flow/evidence/fn23/probe-full-timing.json (valid, p50 18.085 ms), cargo run ... --level 3 (exit 1: unknown level "3"; one of quad)
- PRs:
stage: plan-sync - skipped(config: planSync.enabled != true)
