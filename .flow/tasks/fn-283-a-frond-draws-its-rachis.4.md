---
satisfies: [R5, R7]
---
# fn-283-a-frond-draws-its-rachis.4 The walk, the timing and the stills

## Description
The measured close: R7's walk measure as a test, R5's warm timing on base and branch, the stills the verdict rests on, the docs, and the evidence for the host's owner verdict.

**Size:** M
**Files:** a test in `crates/telperion-core/tests/` (the walk measure; rewrite of the film probe `git show 6429cf4a5:crates/telperion-render/examples/frond_probe.rs`, nothing copied without a rewrite), `docs/pipeline.md`, `.flow/evidence/fn-283-a-frond-draws-its-rachis/RESULT.md`
**Touches:** [crates/telperion-core/tests/**, docs/pipeline.md, .flow/evidence/fn-283-a-frond-draws-its-rachis/**]

### Approach
- Walk test: `blend::families(oak@40 seed 7, palm@40 seed 7, t)` and `mesh::build` over a t grid that includes 0, the halo (0.067 to 0.562, at least every 0.05) and 1; for each frond leaflet base the distance to the drawn curve as the spec defines it. Separate frond leaflets from station-borne ones (a build at `cauline` 0 keeps the rosette's bytes, so its leaf positions identify them; see `raw/walk-probe2.tsv`'s method). Assert every frond leaflet base under 1 mm; print the station-borne ones' numbers for the Open question, never assert them. Keep the test's runtime modest (it runs in the gate).
- Timing: `BUILD_PRESETS=date-palm BUILD_AGE=40 cargo test --profile ci -p telperion-render build_time -- --ignored --nocapture` on the base (a worktree of the release branch at the fork point, its own target dir) and on the branch; record warm ms.
- Stills with the headless example: the palm at 40, the young palm (the age .1 tested), the walk at t = 0.5 and 1 (`--family`/`--to-family` or `--to`); view each (four images at most); keep the sheet the verdict rests on as one compressed JPEG beside the evidence, raw PNGs under `raw/`.
- Docs: `docs/pipeline.md` lines 7-11, 36, 56, 65 (the expansion's first step and the device's room now hold the frond axes).
- `space_hash` lines for the branch in RESULT.md (`docs/evidence-retention.md`).

### Investigation targets
**Required:**
- `crates/telperion-core/tests/walk_leaves.rs` - walk test pattern
- `crates/telperion-render/examples/headless.rs` and `headless/walk.rs`
- `crates/telperion-render/src/generation/build_time_tests.rs`
**Optional:**
- `crates/telperion-render/examples/space_hash.rs`

### Key context
- The capture budget in AGENTS.md: stills only, at most four images viewed, no full-forest capture.
- The owner's verdict is the host's to collect: finish with the stills' paths.

## Acceptance
- [ ] A test holds every frond leaflet base within 1 mm of the drawn wood across the oak-to-palm walk at 40, seed 7, t = 0 to 1; the station-borne compound leaves' numbers are reported in RESULT.md.
- [ ] The palm at 40 warm device build measured on base and branch; under 0.5 s on the branch, or the gap reported.
- [ ] Stills of the palm at 40, the young palm and the walk at t = 0.5 and 1 viewed; the sheet committed as one JPEG; paths in RESULT.md.
- [ ] `docs/pipeline.md` names the frond axes where it names the leaf bases.

## Done summary
The new test walk_rachis holds every frond leaflet base within 1 mm of the drawn wood. It covers the oak-to-palm walk at 40, seed 7, at 20 shares from t = 0 to 1 (worst 0.25 mm, at t = 1). Station-borne leaves are printed, never held. RESULT.md records the walk table, the R5 timing, the stills, the space_hash lines and the decisions from tasks .1 to .3. docs/pipeline.md names the frond axes beside the leaf bases.

Measured:
- R7: frond leaflets are 0 to 2.5e-4 m off the wood at every share. The test takes 18 s (raw/walk-4.log).
- Station-borne leaves: 1,445, 352, 87, 168, 56, 5 and 9 at t = 0.1, 0.15, 0.2, 0.25, 0.3, 0.35 and 0.45, with medians of 0.12 to 1.18 m. At t = 0.45, 9 remain, against the base note "none from t = 0.4". This is the open question for the owner.
- R5: the device's warm build of the palm at 40 is 85 to 94 ms at width 0.25 and 82 to 94 ms at width 0 (seeds 1, 7 and 42, twice each). The base's recorded figure is 78 to 79 ms (fn-259 RELEASE.md). The branch is under 0.5 s. The width-0 run used a temporary env read in build_time_tests.rs, removed uncommitted (FRICTION.md).
- Stills, all four viewed: raw/stills/palm-40.png, raw/stills/palm-12.png, raw/stills/walk/frame-0002.png (t = 0.5) and raw/stills/walk/frame-0003.png (t = 1). The sheet is stills-4.jpg. At t = 0.5 the sparse fronds read as bark-coloured spokes carrying leaflets. The owner's verdict is the host's to collect.

Observed, not acted on: at t = 0.4, `executor::expand` on the grown, clothed tree with cauline 0 drew a different curve height from a full build (15.74 m against 14.69 m). The cause was not isolated. The test grows both builds instead.

Gate: `telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` (raw/gate-4.log, admitted 15:40): 1,412 passed, 0 failed, 112 ignored, gate_rc 0.

baseline: green (cargo test --profile ci -p telperion-core rachis, raw/baseline-4.log)
stage: impl-review - ran (codex, one reviewer: correctness, SHIP, no findings)
Tier: not given by the conductor (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 0058be591632637e2508054a26aa608bfbb86c51, f992e406c0a81a820e92c4207b7902e5d6c1e03f
- Tests: cargo test --profile ci -p telperion-core --test walk_rachis -- --nocapture, BUILD_PRESETS=date-palm BUILD_AGE=40 cargo test --profile ci -p telperion-render build_time -- --ignored --nocapture, telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast
- PRs: