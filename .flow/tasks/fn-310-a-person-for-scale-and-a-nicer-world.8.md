---
satisfies: [R2, R5]
---
# fn-310-a-person-for-scale-and-a-nicer-world.8 The person beside the subject, never in front: D7

## Description
Owner verdict (2026-10-23): the person should "stand beside, not in front". The spec's D7 replaces D2's direction: the person's foot is at the trunk's depth or behind it along the shot's view direction, to one side at the crown ellipse plus 0.5 m, on the sunlit side at least 90° from the camera's azimuth, outside the crown's actual shadow. The harness and `createRenderer` use the hero camera's azimuth; the film passes each frame camera's azimuth with its lerped placement (D5 as ruled). D6 (whole in frame or exempt) is unchanged.

**Size:** M
**Files:** `crates/telperion-render/src/scene/person.rs`, `crates/telperion-render/src/scene.rs`, `crates/telperion-render/src/lib.rs`, `crates/telperion-render/examples/film.rs`, `crates/telperion-render/examples/film/*.rs`, `crates/telperion-render/tests/person_frame.rs`, `crates/telperion-render/src/shadow/fit.rs` (only if the shadow test needs its projection), other render tests, `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/`
**Touches:** [crates/telperion-render/src/**, crates/telperion-render/examples/**, crates/telperion-render/tests/**, .flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/**]

### Approach
- `person::stand` takes the shot's view azimuth and the sun (azimuth, elevation) as inputs, with the bounds. Candidate directions are at least 90° from the camera's azimuth on either side; the foot is at the ellipse reach + 0.5 m. Pick the side and angle whose foot is outside the crown's actual shadow (project the crown ellipse along the sun onto the ground, as the shadow fit does) and on the sunlit side. Then apply D6 (whole in the hero frame where a camera-side fit exists, otherwise exempt by name). State the angle search and its tie-break so the choice is deterministic and continuous in the view azimuth.
- Depth: the foot's depth along the view direction is at or behind the trunk's.
- The harness and `createRenderer` pass the hero camera's azimuth (default shot) and the row's sun. The film passes each frame camera's azimuth and its sun with the lerped placement; the lerp between years stays.
- Continuity: in the film the person moves with the orbit. Render a short consecutive stretch via `film --frames` (for example 30 consecutive frames of one orbiting segment) and view them as a strip or contact sheet. If the person visibly slides against the ground, or jumps sides, stop and return NEEDS_HUMAN with the frames' path before choosing anything else.
- Tests:
  - depth at or behind the trunk, for every shipped preset at its catalogue age and at age 1, and Telperion at 80;
  - outside the actual crown shadow under the harness sun (135/30) and the film's sun (115/60);
  - not above any canopy (the canopy test, as now);
  - in frame or exempt by name (as now);
  - the film's placement continuous across a segment's frames, with no side flip within a segment.
- Before/after sheet of film frames 105, 255, 720, 825 and 975 via `film --frames` (before = the frames at 83ec74e0d, which task .7 already rendered for four of them; render 975's before from 83ec74e0d's code if needed, or say how it was obtained), at `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/raw/stills/film-beside.jpg` (atomic write) with a compressed committed copy at the evidence folder's top level. Label rows before and after. View it. Frame 975 needs segment 8, whose build is about 21 minutes; budget for it once.
- Run `--lib tint_rule_tests`, `--lib compile_size` and `--test conformance` before review.

### Investigation targets
**Required:**
- `crates/telperion-render/src/scene/person.rs` - `stand`
- `crates/telperion-render/src/shadow/fit.rs` - `towards`, the shadow projection
- `crates/telperion-render/examples/film/placing.rs` - the lerped placement
- `crates/telperion-render/examples/film/camera.rs` - the frame camera's azimuth
- `crates/telperion-render/tests/person_frame.rs`

## Acceptance
- [ ] The foot is at or behind the trunk's depth along the shot's view direction, at the ellipse reach + 0.5 m, at least 90° from the camera's azimuth, on the sunlit side outside the crown's actual shadow; the choice is deterministic
- [ ] Harness and createRenderer use the hero azimuth and the row's sun; the film passes each frame camera's azimuth with the year-lerped placement
- [ ] Tests: depth (every preset at catalogue age and age 1, Telperion 80), out of shadow under 135/30 and 115/60, canopy, in-frame or exempt by name, film continuity without a side flip within a segment
- [ ] A consecutive-frame strip shows the person moving naturally with the orbit, or the task returned NEEDS_HUMAN with it
- [ ] Before/after sheet of film frames 105, 255, 720, 825 and 975, labelled, at the raw path with a committed compressed copy
- [ ] Static checks and focused render tests green

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
