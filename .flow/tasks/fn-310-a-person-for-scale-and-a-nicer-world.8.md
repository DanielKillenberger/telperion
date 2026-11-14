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


### Host rulings on task .8 (2026-10-23)
See the spec's "Host rulings on task .8". In the film: one side for the whole shot (the camera's left if out of the crown's actual shadow on every frame under the film's sun, else the right; if both fail, report the frames), and the distance from the smoothed reach A(f) = mean over [f-W, f+W] of M, where M(f) = max of r over [f-2W, f+2W] across the whole film and W = 15. The harness and `createRenderer` keep D7's per-tree sun side, with the `set_scene` update.
## Acceptance
- [ ] The foot is at or behind the trunk's depth along the shot's view direction, at the ellipse reach + 0.5 m, at least 90° from the camera's azimuth, on the sunlit side outside the crown's actual shadow; the choice is deterministic
- [ ] Harness and createRenderer use the hero azimuth and the row's sun; the film passes each frame camera's azimuth with the year-lerped placement
- [ ] Tests: depth (every preset at catalogue age and age 1, Telperion 80), out of shadow under 135/30 and 115/60, canopy, in-frame or exempt by name, film continuity without a side flip within a segment
- [ ] A consecutive-frame strip shows the person moving naturally with the orbit, or the task returned NEEDS_HUMAN with it
- [ ] Before/after sheet of film frames 105, 255, 720, 825 and 975, labelled, at the raw path with a committed compressed copy
- [ ] Static checks and focused render tests green

- [ ] Film: one side for the whole film (left preferred), tested constant; the shadow test covers every film frame on the chosen side; the ignored side-change test is replaced
- [ ] Film: reach A(f) per the ruling (W = 15, M over [f-2W, f+2W] across the whole film); tests show A >= r on every frame and the largest frame-to-frame foot move under 10 cm (or the measured bound stated)
- [ ] Strips 240-269 and 690-719 re-rendered and viewed; any frame where the person overlaps the map overlay is reported
## Done summary
The person now stands beside the subject and never in front of it, by D7 and the host's rulings on task 8.

Stills (harness and createRenderer): `beside::stand` puts the person 90 degrees round from the hero view on the sun's side, at the trunk's depth, at the crown ellipse's reach plus 0.5 m. A sun exactly on the view's axis takes view + 90. `set_scene` moves the person when the sun's azimuth changes. Whole in frame or exempt is unchanged: the seven presets at age 1 are exempt.

Film: the person stays 90 degrees to the camera's left of each frame's camera, at the trunk's depth. Its distance is the larger of the year-faded reach plus 0.5 m and the nearest distance out, in 0.02 m steps, at which the whole figure is out of every shown crown's shadow under 115/60. That distance is smoothed with W = 30 (the maximum over 2W, then the mean over W, floored at the frame's own need). Per-frame bounds come from one whole-film bounds pass, cached in raw/film-bounds.tsv. The film refuses the cache if the path's digest differs or if a drawn tree differs from its line.

Measured over all 1,050 frames from the cache: every frame is lit, out from under the crown, at the trunk's depth and on the left. The person needs to stand up to 3.52 m beyond the reach to leave the shadow. The largest frame-to-frame foot move is 0.183 m at frame 830, as Laurelin's crown grows; the test's bound is 0.19 m.

Choices delegated to the worker:
- 0.02 m search step: below a visible step at the film's distances, and cheap enough to search 1,050 frames in under a second.
- Search gives up 100 m past the reach and reports the frame: a sun above the horizon throws a finite shadow, so a frame with no answer is an error, not a fallback.
- Smoothed distance floored at the frame's own need: the mean of equal values can round below it.
- No cap short of behind the trunk: the angle is fixed at 90 degrees, so the person is never behind the trunk.
- Bounds file keyed by a digest of the path (review P2, fixed after SHIP).
- Growth years drawn as each is settled: the one-year lookahead is no longer needed, since placements come from the bounds pass.

Strips (raw/stills/left-strip-240 and 690, committed film-left-strip-*.jpg):
- 240-269: the person is steady at the left, not hidden and clear of the map overlay.
- 690-719: the person is out of frame throughout. The window spans segments, so the shrinking date palm's need holds it about 10 m out beside the seedlings.

Sheet: film-beside.jpg (raw/stills/film-beside.jpg).
- After: 105 and 255 whole at the left.
- 720: no person in frame, for the same reason as 690-719.
- 825 and 975: small and far, about 24 m and 32 m out (estimate).
- The 975 before frame was built from git archive 83ec74e0d under its own target (raw/base-83ec74e0d, 2.7 GB, safe to delete).

Follow-ups:
- The cross-segment window leaves the seedling walk and frame 720 without the person, and the growth lookahead keeps it well ahead of Laurelin's crown at 825. This is the owner's call by eye.
- film.rs is 468 lines.
- The side branch fn-310-t8-wip is merged and left for the host.

baseline: green via handoff (focused green at 3e63d7e32; only .flow/ changed since)
stage: impl-review - ran (codex, 3 draws: correctness, contracts, integration; all SHIP; one P2 kept and fixed in 7a375ceaf)
Tier: session at default effort (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 8378532344d6ef65afc70b161ae4ec173c7ff69d, f60c5ef798a2cca9082f6440efbf4ccb13ffd779, a521fd39ef2ed1bf94d8f760f37dc0288ed8e1b0, a9994c191b2c23f5efdadee93eca3373daa71014, 246caf091489af483c6a8046ebf2c4511eb343df, 5ee2ca00d6a099e599864f482769b160ce447cfc, 90e71bcaf64e6574889d9d836ea93ac1b517a3f3, 5fc6c5fc6f8537b0d091b3c7ade0fbd9103fed1a, 6c9906822409171483b607dc1aa459407c5013a3, f82a3ea75c2f9dd25024e7ed2f69e8da1f3daaec, 7a375ceafea412c6eae095e5ba441d46f32780dc, 0d105489e71caa78263f5deb51b7a347e05e2b61
- Tests: cargo test --profile ci -p telperion-render --test film_placing --test person_frame -- --include-ignored, cargo test --profile ci -p telperion-render --test sun_cache --test shadow --test conformance, cargo test --profile ci -p telperion-render --lib scene::, cargo test --profile ci -p telperion-render --lib tint_rule_tests, cargo test --profile ci -p telperion-render --lib compile_size, cargo fmt -p telperion-render --check; cargo clippy (no new warnings in changed files)
- PRs: