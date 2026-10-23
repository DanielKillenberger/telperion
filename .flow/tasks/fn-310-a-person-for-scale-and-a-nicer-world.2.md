---
satisfies: [R2, R5]
---
# fn-310-a-person-for-scale-and-a-nicer-world.2 The person: a procedural 1.75 m scale figure placed outside the crown and its shadow

## Description
The person half (D2, D3): a new `scene/person.rs` replaces the capsule in `room.rs`, with the host's better proportions, a cool dark matte colour, a stated vertex budget, and the placement rule reading the tree's bounds. Ends with the owner's still, `person-v2.jpg`.

**Size:** M
**Files:** `crates/telperion-render/src/scene/person.rs` (new), `crates/telperion-render/src/scene/room.rs`, `crates/telperion-render/src/scene.rs`, `crates/telperion-render/src/lib.rs`, `crates/telperion-render/src/generation/submit.rs`, `crates/telperion-render/tests/shot.rs`, `crates/telperion-render/src/camera.rs` (tests only)
**Touches:** [crates/telperion-render/src/scene/person.rs, crates/telperion-render/src/scene/room.rs, crates/telperion-render/src/scene.rs, crates/telperion-render/src/lib.rs, crates/telperion-render/src/generation/submit.rs, crates/telperion-render/src/camera.rs, crates/telperion-render/tests/shot.rs, .flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/**]

### Approach
- Rewrite the concept's ring-lofted body (`scene-concept: crates/telperion-render/src/scene/person.rs`) to D3. Fix what the host named: thin legs, a dark band where torso meets legs, the warm grey. Legs of believable thickness and length, joined to the torso with no step in width at the hips; shoulders and a slight taper; arms at the sides; no face. Smooth normals as the concept does.
- `FIGURE_HEIGHT` (`room.rs:18-21`) becomes 1.75 and is derived from the person, so the camera's floor (`camera.rs:80`) follows it. Remove the capsule and its test.
- Replace `FIGURE` with a cool dark slate in the room palette (`room.rs:10-16`).
- `place_figure` (`scene.rs:422-431`) takes the tree's bounds (callers `lib.rs:243`, `generation/submit.rs:50`) and applies D2: azimuth = default shot azimuth less 68°, distance = the inscribed crown ellipse's reach in that direction (`camera.rs` `reach_of`) plus 0.5 m, facing the camera (host ruling 2026-10-23, spec D2). The index count and buffer offsets (`scene.rs:165-179`) follow the person's mesh.
- `tests/shot.rs:10-11` derives the figure's triangle count from the person, not a literal.
- Then render the owner's still, before this task's review: oak at 40 and Telperion at 80, seed 7, 1280x720, through `headless` under the new default row, set beside the concept's `c-oak.png` and `c-telperion.png` (branch `scene-concept`, `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/concept/`) in one sheet at `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/raw/stills/person-v2.jpg`. The host asked for that path. A compressed copy of the sheet, which carries an owner verdict, is retained outside `raw/` at `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/person-v2.jpg` (docs/evidence-retention.md); the single stills stay under `raw/`. Age goes through `--family` with a partial wire object; check the route works before relying on it.

### Investigation targets
**Required:**
- `crates/telperion-render/src/scene/room.rs` - the capsule, palette, `fixture()`
- `crates/telperion-render/src/scene.rs:160-180,420-470` - buffer layout, `place_figure`, `draw`
- `crates/telperion-render/src/shot.rs:20-32`, `camera.rs:60-90` - the default azimuth and the azimuth convention (clockwise from +Z, `sin` on x)
- `crates/telperion-render/src/shadow/fit.rs:39-46` - the sun direction convention
- `crates/telperion-render/examples/headless/walk.rs:13-17,205-208` - headless flags
**Optional:**
- `crates/telperion-render/tests/shot.rs` - the triangle count use

### Key context
- The person is a fixture (colour alpha 0): never repainted or hazed by the row.
- Heavy commands only through `~/.local/bin/telperion-heavy 5 --gpu 3 --threads 2`.
## Acceptance
- [ ] The person stands on y = 0 and is 1.75 m tall; `FIGURE_HEIGHT` equals its measured height
- [ ] Vertex and triangle counts are within the stated budget (800 / 1,200), tested
- [ ] Placement test: for a shrub's bounds and for Telperion-wide bounds, the foot lies outside the inscribed crown ellipse (spec D2, host ruling 2026-10-23) and on the side the default sun (135°) and the film's sun (115°) light, so outside the crown's shadow
- [ ] Canopy test (host, 2026-10-23): no wood or foliage vertex of any shipped preset at its catalogue age, nor of Telperion at 80, lies above the person's footprint (its horizontal extent at the placed foot), measured on the real generated geometry; within the gate's per-test budget or `#[ignore]`d beside a fast bounds-table check, as the in-frame test does
- [ ] Placement test (host, 2026-10-23): at the hero shot the whole person projects inside the frame for every shipped preset at its catalogue age and for Telperion at 80, using the trees' real bounds; the test stays inside the gate's 120 s and 4 GiB per-test budget (reuse an existing every-preset build or a measured bounds table checked against the generator)
- [ ] A colour test: the person's lit colour under the default sun and in shade keeps blue at or above red (reads cool, not skin)
- [ ] `tests/shot.rs` derives the figure's triangles from the person; capsule code and test removed
- [ ] `person-v2.jpg` exists at the stated `raw/stills/` path, beside the concept's C row, and was sent to the host; a compressed copy is committed at the evidence folder's top level
- [ ] Focused render tests green through `telperion-heavy 5 --gpu 3 --threads 2`
## Done summary
A procedural 1.75 m person replaces the capsule (`scene/person.rs`): lofted rings, legs rising into the torso with no step at the hips, shoulders, a slight taper, arms at the sides, a faceless head, smooth normals, in a dark cool slate (`FIGURE` 0x353d4a). `FIGURE_HEIGHT` is the person's `HEIGHT`. `place_figure` takes the tree's bounds and stands the person 68 degrees to the camera's left of the default shot, 0.5 m past the inscribed crown ellipse's horizontal reach, facing the camera (spec D2, host ruling 2026-10-23).

- Budget: 627 vertices and 1,020 triangles, against 800 / 1,200 (tested).
- Tests (person.rs): on y = 0 at 1.750 m, `FIGURE_HEIGHT` its measured height; outside the crown ellipse and its shadow for a shrub and for Telperion at 80 under the 135/30 and 115/60 suns; blue at or above red when lit by the default sun and in shade.
- Tests (camera.rs): in the 16:9 hero frame for every shipped preset at its catalogue age, Telperion at 80, and the owner's two seed-7 stills (worst y = -0.97). The test reads a measured bounds table. The ignored test `the_bounds_table_is_the_generators_and_no_canopy_covers_the_person` (84 s, run with --ignored) holds that table to `mesh::build` and checks the real geometry: no wood vertex, and no leaf or floret out to its element's reach, stands over the person's footprint. It is red with the foot at the trunk (oak trunk wood at 1.4 m) and green at the placed foot.
- `tests/shot.rs` takes the figure's triangles from `figure_counts()`; the capsule and its test are gone.
- Owner's still: `raw/stills/person-v2.jpg` (oak 40 and Telperion 80, seed 7, 1280x720, `--scene '{"sunAzimuth":115,"sunElevation":60}'` to match concept C), sent to the host; compressed copy at `.flow/evidence/fn-310-.../person-v2.jpg`. After viewing the first sheet I removed a hip crease and darkened the slate.
- Scale check (`scale-check.md`): Telperion at 80 is 46.46 m; the person projects to 20.5 px against the trunk's 548 px, a ratio of 26.7 against the true 26.5. The scale is honest.
- Feature-map route: unchanged (no `.flow/features` route moves).
- Baseline: green via handoff (91eddc36; only .flow paths changed since).
- Focused tests green through telperion-heavy 5 --gpu 3 --threads 2: `--lib scene`, `--lib camera`, the `shot`, `look`, `neutral_light`, `shadow` and `environment` test files, the ignored generator and canopy test, and clippy on the changed files.

Tier: session (jev long_running 0.29) at default effort (model: claude-opus-5-5)

stage: impl-review - ran (codex: round 1, 3 draws, NEEDS_WORK on the ellipse versus bounds-box footprint; answered by the host's D2 ruling plus the canopy test; re-review SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: d5dc6ef61b2c5cc721d4efc4f79d9e6fd813fe9d, 6ac1a9072ce4cd7b7d26acfeb1d4dcb9d1fa61d0, e985e397203ee2fd2a30a5aadcebd942f9a72aca, 13328e77c1320c0a17a5572f06654e08a618f4c4
- Tests: ~/.local/bin/telperion-heavy 5 --gpu 3 --threads 2 -- cargo test --profile ci -p telperion-render --lib scene, ~/.local/bin/telperion-heavy 5 --gpu 3 --threads 2 -- cargo test --profile ci -p telperion-render --lib camera, ~/.local/bin/telperion-heavy 5 --gpu 3 --threads 2 -- cargo test --profile ci -p telperion-render --test shot --test look --test neutral_light --test shadow --test environment, ~/.local/bin/telperion-heavy 5 --gpu 3 --threads 2 -- cargo test --profile ci -p telperion-render --lib -- no_canopy_covers --ignored
- PRs: