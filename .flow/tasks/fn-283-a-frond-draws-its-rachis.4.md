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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
