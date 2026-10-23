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
- `place_figure` (`scene.rs:422-431`) takes the tree's bounds (callers `lib.rs:243`, `generation/submit.rs:50`) and applies D2: azimuth = default shot azimuth less a stated offset, distance = the bounds' horizontal support in that direction plus a stated margin, facing the camera. The index count and buffer offsets (`scene.rs:165-179`) follow the person's mesh.
- `tests/shot.rs:10-11` derives the figure's triangle count from the person, not a literal.
- Then render the owner's still, before this task's review: oak at 40 and Telperion at 80, seed 7, 1280x720, through `headless` under the new default row, set beside the concept's `c-oak.png` and `c-telperion.png` (branch `scene-concept`, `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/concept/`) in one sheet at `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/raw/stills/person-v2.jpg`. Age goes through `--family` with a partial wire object; check the route works before relying on it.

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
- [ ] Placement test: for a shrub's bounds and for Telperion-wide bounds, the foot lies outside the crown's horizontal support and on the side the default sun (135°) and the film's sun (115°) light, so outside the crown's shadow
- [ ] Placement test (host, 2026-10-23): at the hero shot the whole person projects inside the frame for every shipped preset at its catalogue age and for Telperion at 80, using the trees' real bounds; the test stays inside the gate's 120 s and 4 GiB per-test budget (reuse an existing every-preset build or a measured bounds table checked against the generator)
- [ ] A colour test: the person's lit colour under the default sun and in shade keeps blue at or above red (reads cool, not skin)
- [ ] `tests/shot.rs` derives the figure's triangles from the person; capsule code and test removed
- [ ] `person-v2.jpg` exists at the stated path, beside the concept's C row, and was sent to the host
- [ ] Focused render tests green through `telperion-heavy 5 --gpu 3 --threads 2`
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
