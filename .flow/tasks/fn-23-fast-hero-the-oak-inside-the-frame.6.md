---
satisfies: [R4, R5]
---
# fn-23-fast-hero-the-oak-inside-the-frame.6 Every preset through one path, the stills, the owner's verdict, and the docs that say one draw

## Description
Close the spec: the preset and random-set sweep on the final path (R5), the species stills through the headless target, the oak still placed beside fn-22's for the owner's verdict (R4), the report completed, and the documentation that still describes one instanced draw brought to the truth.

**Size:** S
**Files:** `crates/telperion-render/tests/conformance.rs`, `tests/species.mjs`, `.flow/evidence/fn23/oak-hero.png`, `.flow/evidence/fn23/spruce-hero.png`, `.flow/evidence/fn23/REPORT.md`, `.flow/specs/fn-23-fast-hero-the-oak-inside-the-frame.md`, `README.md`, `package.json`, `crates/telperion-render/src/foliage.rs`
**Touches:** [crates/telperion-render/tests/conformance.rs, tests/species.mjs, .flow/evidence/fn23/**, .flow/specs/fn-23-fast-hero-the-oak-inside-the-frame.md, README.md, package.json, crates/telperion-render/src/foliage.rs]

### Approach
- Confirm the conformance sweep from task 3 covers every shipped preset and at least twenty seeded random parameter sets on the final path, with one exercised generator rejection surfacing the generator's message; add the seed list to the test if it is not already pinned.
- `npm run species:qa` renders every profile's still through the headless target; commit the oak and spruce hero stills at seed 7 to `.flow/evidence/fn23/`.
- Complete `REPORT.md`: the measured tables from tasks 4 and 5, the spruce numbers marked as recorded and not gated, and the owner verdict slot. The owner compares `oak-hero.png` with `.flow/evidence/fn22/oak-hero.png` and writes the verdict into the spec under a `## Owner verdict` heading; the task is done only on an accepting verdict.
- Documentation: `package.json` description and the module doc at `crates/telperion-render/src/foliage.rs:1` say one instanced draw; `README.md:58` describes the draw path, `:92-93` the headless example, `:100` the timing record. Update each in one sentence to the per-level indirect draws, the `--level` and `--orbit` flags, and the selection and orbit fields.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/tests/conformance.rs` — the sweep to confirm or extend
- `tests/species.mjs` — the stills rig
- `README.md:55-105` — the three statements to update

**Optional** (reference as needed):
- `.flow/evidence/fn22/REPORT.md` — the owner verdict section shape

### Key context
- The verdict is the owner's, recorded in the spec in their words; the task prepares the comparison and waits.
- Budget: two stills and the sweep; no further timing runs unless task 4 or 5 evidence was invalidated by a change here, in which case rerun that one session.

## Acceptance
- [ ] Every shipped preset and at least twenty pinned random parameter sets render a non-background still through the final path, and one generator rejection surfaces the generator's message, in `cargo test --release -p telperion-render`
- [ ] `npm run species:qa` passes and `.flow/evidence/fn23/oak-hero.png` and `spruce-hero.png` are committed at seed 7 from the headless target
- [ ] `.flow/evidence/fn23/REPORT.md` carries the native and browser tables, the orbit numbers, the spruce rows marked recorded and not gated, and the owner's verdict
- [ ] The spec carries the owner's verdict on the oak still in a `## Owner verdict` section; the task closes only on an accepting verdict
- [ ] `package.json`, `README.md` and the foliage module doc no longer say one instanced draw; the README names `--level` and `--orbit` and the selection and orbit record fields
- [ ] `npm run typecheck` and `cargo test --release --workspace` pass

## Done summary
Everything R4 and R5 ask for is on disk except the one line only the owner can
write. The oak and spruce stills are committed at seed 7, whole view, hero pose,
1600 by 1000 from the native headless target, the same command, pose and size as
fn-22's stills, so the two oaks lay over each other exactly. `REPORT.md` now
carries the native tables from task 4, the browser still and orbit tables from
task 5, the preset and random-set sweep, the stills, and verdicts for R1, R2, R4
and R5. The spec carries a `## Owner verdict` section with an empty slot.

**R5 is met.** `cargo test --release -p telperion-render` renders all five
shipped families and 20 parameter sets nobody wrote by hand through one submit
and one frame from a pinned generator seed: 20 rendered, 5 refused, 25 tried.
The refusals carry the generator's own message, asserted on a negative envelope
height and a negative leaf length. Two tests hold the no-branch rule by reading
the source: no renderer file may contain the family table, and `select.rs`,
`foliage.rs`, `select.wgsl` and `foliage.wgsl` may name neither a shipped family
nor any of six anatomy words. Task 3 had already pinned the sweep's seed and its
twenty-set floor, so no test changed here. `npm run species:qa` ran both profiles
over the protocol's 24 seeds: 48 numeric cases pass, 48 stills captured, none
missing and none failed. The runner exits 1 while the visual inspection is
unassessed, which is its standing contract and not a failure of this run.

**R4 waits on the owner.** The comparison is
`.flow/evidence/fn23/oak-hero.png` against `.flow/evidence/fn22/oak-hero.png`.
As an aid, not a verdict: 4.6 per cent of the frame differs at all, 1.0 per cent
by more than 32 of 255 in some channel, the mean absolute difference over the
whole frame is 0.83 of 255, and the mean luminance moves from 170.83 to 170.88.
The differences sit inside the crown, where a leaf a few pixels tall is drawn
from 4 or 12 triangles instead of 268. The report and the spec both hold an
empty slot for the owner's words.

**Documentation.** `package.json` said foliage was culled to a shell "for one
instanced draw" and now says one indirect draw per level, each leaf at the
coarsest level a pixel cannot tell from the finest. The README's draw-path
paragraph gains the compute pass, the half-pixel rule, the unseen bucket and the
subset property that keeps a coarse leaf's vertices a fine leaf's; the headless
example gains `--level` and `--orbit` with a sentence on each; the timing
paragraph names `selection_p50_ms`, `selection_p95_ms`, `total_p50_ms`,
`total_p95_ms`, `levels`, `wall_p50_ms`, `wall_p95_ms`, `wall_max_ms` and
`wall_frames`, says a non-valid verdict carries none of them, and links the FN23
report beside FN22's. The foliage module doc needed nothing: task 3 already
rewrote it to the level selection, so the task's premise about that one file was
stale.

One thing worth knowing that the report now states: the per-frame statistics
line the headless target prints reports the crown as if every leaf were at the
finest level, because no count comes back from the device outside a timing
session. A reader running the oak sees 154 million triangles on that line where
the measured truth is 5.6 million. That is the documented statistics contract
from the spec, kept so the panel text and the rigs' parsers are unchanged.

`npm run test:render` was not rerun. This task changed no renderer, page or
browser code, and a second session would overwrite the browser records task 5
committed at 1dc7829, which are the numbers this report reads from.

stage: impl-review - skipped(policy: host-deferred - conductor owns the gate)

### Owner verdict (R4)

"side by side they look exactly the same so we achieved the goal" (owner, 2026-09-09). Recorded in the spec and in .flow/evidence/fn23/REPORT.md by the conductor.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 59b6f4fe6dcb33fcc2b7cf886f4bd945225a20e6
- Tests: cargo test --release --workspace (29 suites, 0 failed), cargo test --release -p telperion-render (5 conformance tests; sweep printed 20 sets rendered, 5 refused, 25 tried), npm run species:qa (48 numeric cases pass, 48 stills captured; runner exits 1 by design while visual inspection is unassessed), npm run typecheck, npm test (64 vitest cases), headless oak-hero.png and spruce-hero.png at seed 7, 1600x1000, whole view, baseline: green via handoff (verified at 1dc7829 by fn-23-fast-hero-the-oak-inside-the-frame.5), SKIPPED: npm run test:render - no renderer, page or browser code changed in this task; task 5 recorded the committed browser sessions at 1dc7829 and rerunning would overwrite them with a second session
- PRs: