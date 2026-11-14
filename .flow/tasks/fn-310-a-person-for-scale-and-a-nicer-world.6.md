---
satisfies: [R4, R5]
---
# fn-310-a-person-for-scale-and-a-nicer-world.6 Explain the film's +51 ms a frame, and render the before frames 105, 255, 720 and 825

## Description
The spec's "Cut 7 findings": film draw time rose from 87 s (cut 6) to 141 s (cut 7) over 1,050 frames (about +51 ms a frame), while the harness measured +0.05 ms. Find out why before anything changes, and render the "before" stills of frames 105, 255, 720 and 825 through the film path, on the code as it stands (the same renderer code cut 7 used, c047afedb).

**Size:** M
**Files:** `crates/telperion-render/examples/film.rs`, `crates/telperion-render/examples/film/*.rs` (measurement and frame selection only), `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/`
**Touches:** [crates/telperion-render/examples/film.rs, crates/telperion-render/examples/film/**, .flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/**]

### Approach
- Read how the film draws a frame (`examples/film.rs`, `examples/film/*.rs`): how many renders per frame (a crossfade draws two years), whether the sun's map is redrawn each frame (sun-map cache key, `shadow_cached()`), and what "draw" in its per-segment table counts (`raw/cut7-render.log` and FILM.md in fn-272's evidence).
- Measure, never infer: time the candidates the host named one at a time on a short stretch of one segment that shows the rise (for example segment 7, Laurelin grown 1-80, 210 frames, build 11.8 s; or segment 0), with the shipped renderer at the base 201288747 (cut 6's renderer, before fn-310) against HEAD. Candidates: (a) the sun's map redrawn per frame, for example because the person or its casters change the cache key between the two crossfade renders, or `figure_casters` readback stalls; (b) the person drawn in both crossfade renders; (c) the ground noise and sky at 1280x720; (d) the film's own path (readback, encode, overlay). Toggle each where the code already allows it (`set_figure(false)`, the row's variation and haze at zero, `shadow_cached()` counts) rather than adding renderer switches.
- A frame-selection option on the film example (for example `--frames 105,255,720,825`) that builds only the segments those frames need and writes only those frames, through the same path, is in scope. It is example code, not shipped. Segment 8's build is 1,238 s, so avoid it unless a frame needs it.
- Report the measured cause with numbers in `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/FILM-DRAW.md`. If it is a real per-frame cost in the shipped renderer (not in the film's path), stop with the number and return NEEDS_HUMAN: the host decides before any fix.
- Render the before frames 105, 255, 720 and 825 (1280x720) to `raw/film-before/` and keep them for task .7's before/after sheet.

### Investigation targets
**Required:**
- `crates/telperion-render/examples/film.rs` and `examples/film/{camera,cost,map,path}.rs`
- `.flow/evidence/fn-272-the-release-film-through-species-space/FILM.md` (cut 6 and cut 7 sections)
- `crates/telperion-render/src/sun.rs`, `src/lib.rs` (`shadow_cached`, `figure_casters`)
- task .3's base build recipe in `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/COST.md` (git archive into scratch, own CARGO_TARGET_DIR)

## Acceptance
- [ ] The cause of the +51 ms a frame is measured and named, with numbers, base against HEAD, on the film path, in FILM-DRAW.md
- [ ] If it is a real per-frame cost in the shipped renderer, the task returns NEEDS_HUMAN with the number before any fix
- [ ] Before frames 105, 255, 720 and 825 rendered through the film path at 1280x720, kept under raw/film-before/
- [ ] Any frame-selection option added to the film example renders exactly the frames the full film renders (a frame from the selection matches the same frame of a full run, or the segment's own frame, byte for byte or within a stated tolerance)

## Done summary
The film's extra 51 ms a frame comes from writing the PNG, not from drawing. The renderer's per-frame cost is unchanged, so this is not a NEEDS_HUMAN case. The method and every number are in `.flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/FILM-DRAW.md`.

Measured on the film path, segment 0, frames 60-119 (60 frames, 75 renders). Base is 201288747, exported fresh. Shader cache off, `ci` profile, RTX 3080.
- **Render with read-back:** base 1.06-1.09 s, HEAD 0.99-1.05 s, within noise of each other.
- **The sun's map:** drawn 24 times on both sides.
- **Write:** base 1.44-1.49 s, HEAD 4.59-4.62 s. That is +52 ms a frame, or about 55 s over 1,050 frames; the film's draw rose by 54 s.
- **PNG encode alone:** base 20.8 ms, HEAD 74.1 ms, at 161 KB and 356 KB a frame.
- **The sky's dither:** with it off in a scratch copy, encode is 26.1 ms. So the dither accounts for 48 ms of the rise, and the ground mottle and haze for about 5 ms. The person costs under 1 ms.
- **For the host:** a fix belongs in the film's writer, for example a faster compression setting, encoding on another thread, or piping raw frames to ffmpeg. None was tried; the host decides.

Added `film --frames <list>` (single frames and ranges). It builds only the segments those frames fall in and assembles no video. The cost line now splits draw into render (with renders counted and sun maps drawn) and write.
- **The before frames:** 105, 255, 720 and 825, at 1280x720, are in `raw/film-before/`. All four are byte-identical (`cmp`) to cut 7's full-run frames, and so are the 60 timed frames.
- **Errors:** a frame out of range, a word, an empty entry and a backwards range are each refused by name.

Checks:
- baseline: green via handoff (gate 1493/1493 at f58a01cc8).
- Focused: `cargo test --profile ci -p telperion-render --test film_path --test film_overlay --test film_map` passed (5 tests). Clippy and fmt are clean on the example.

Also noted:
- film.rs is now 429 lines; it was 399 before this task.
- FRICTION.md has a new entry: the scratch "base" export is not 201288747.
- No feature-map route changed.

Tier: session at default effort (model: claude-opus-5-5)

stage: impl-review - ran (codex, one reviewer, SHIP, 0 findings)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 67c8aa5ff1e5623cad9fa59957098e1cae820783
- Tests: cargo test --profile ci -p telperion-render --test film_path --test film_overlay --test film_map, cargo clippy --profile ci -p telperion-render --example film, film --frames 105,255,720,825 (cmp against cut 7: 4/4 identical)
- PRs: