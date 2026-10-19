# fn-23-fast-hero-the-oak-inside-the-frame.1 The probe: every leaf as a two-triangle quad, and the number that decides the spec

## Description
The probe: draw every leaf as a two-triangle quad to decide whether the oak frame is primitive- or pixel-bound.

## Acceptance
- Same instances and coverage, whole vs quad timing recorded with valid verdicts.

## Done summary
Vertex-bound: oak at seed 7 is 18.08 ms whole and 1.58 ms as quads with the same 555,204 instances and coverage; `--level quad` threads the level through `Renderer::submit_at`. Records in `.flow/evidence/fn23/PROBE.md`.

## Evidence
- Commits: 4917a32032ea8339b047623c98acf33527805dbb
- Tests: cargo test --release --workspace
