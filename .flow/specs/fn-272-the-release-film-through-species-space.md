# The release film: through species space, trees growing and shrinking

## Goal & Context

The owner (2026-10-13): "i'd want to do the video we conceptualized at some point going through space and growing/shrinking trees. This should be much more efficient to do now right?"; "we can do it with release and embed it into the repos readme while we're at it". The teaser session (2026-10-09) drove `headless --walk` through species and ages and found fn-250 and fn-251. With fn-270 a tree grows once and each frame settles it at a fractional age (leaves and shoots unfolding, R5/R5a), so the age axis no longer regrows per frame; a step through species space is still one device build per frame (0.07-0.5 s warm, about 1.2 s for the Two Trees, fn-237 R0a). Growth is the product (AGENTS.md).

## Requirements

- **R1 (the path):** one declared path of segments, each either a growth (one species, age a0 -> a1, settled at fractional ages through one GrowthState) or a walk (species A -> B at one age, fn-250/251's walk); shrinking is a growth segment played in reverse; the shipped species and the Two Trees appear; the camera path is part of the declaration. The path is a value file read by the tool, not code.
- **R2 (the tool):** a headless film command renders the path on the device at a stated resolution and frame rate, through the one pipeline and the shipped renderer (no film-only drawing), and assembles the video; per-segment cost (frames, build and settle time, peak memory) reported. One heavy run at a time.
- **R3 (fidelity in motion):** no pop between frames in growth segments (R5a's blend) and no frame over its ends' size in walks (fn-251's oracle); a defect found here is fixed by its own spec, never hidden by the camera or the cut.
- **R4 (the README):** the film is embedded in the repository README, the form that renders inline on GitHub found and checked (unknown today: a committed file against an uploaded asset), within the repository's size budget, with a still as fallback; the README's text around it says what the viewer is seeing in a sentence or two.
- **R5:** the owner's verdict on the film.

## Boundaries

Release item (fn-237). After fn-270 task 4 (unfolding and blending) and fn-251 (walk size). No prebaked trees: every frame is generated.
