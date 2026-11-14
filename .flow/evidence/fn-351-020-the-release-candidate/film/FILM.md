# fn-351 L5: the README's film re-rendered on the candidate (Opus, 14 Nov 2026)

Branch `fn-351-film` from `fn-351-candidate` at a7ae69477, worktree `.worktrees/film`, its own `target/`.

## Script

The README's film is cut 8 of fn-272: `crates/telperion-render/examples/film.rs` reading `crates/telperion-render/examples/film/release.json` (nine segments, 35 s at 30 fps, 1280x720, seed 7), as recorded in `.flow/evidence/fn-272-the-release-film-through-species-space/FILM.md`, "Cut 8". `scripts/scenic-cut.mjs` is the older fn-25 cut, not this film. The script runs on the candidate unchanged; no entry, preset or path was edited.

## Commands (times by the clock, Zurich)

Probe, which also writes the bounds pass to a file: 09:03:29 to 09:59:11, 55.7 min, admitted at once (room 16 GB), build 43 s.

    TELPERION_HEAVY_RESERVE=8 telperion-heavy 8 --gpu 2 --threads 4 -- cargo run --profile ci -p telperion-render --example film -- crates/telperion-render/examples/film/release.json .flow/evidence/fn-351-020-the-release-candidate/raw/film/probe --frames 0,52,105,157,209 --bounds .flow/evidence/fn-351-020-the-release-candidate/raw/film/film-bounds.tsv

Film, reading that bounds file ("bounds: read ..."; nothing refused as stale): 09:59:20 to 10:59:41, 60.4 min, admitted at once (room 18 GB).

    TELPERION_HEAVY_RESERVE=8 telperion-heavy 8 --gpu 2 --threads 4 -- cargo run --profile ci -p telperion-render --example film -- crates/telperion-render/examples/film/release.json .flow/evidence/fn-351-020-the-release-candidate/raw/film/cut --bounds .flow/evidence/fn-351-020-the-release-candidate/raw/film/film-bounds.tsv

Total 116 min of queue time, nearly all of it CPU tree builds, about 5 min of it GPU render and PNG writing. The estimate before the full run was about 1 h, from the probe's bounds pass (3,289 s).

## Result

- `/home/daniel/Projects/telperion/.worktrees/film/.flow/evidence/fn-351-020-the-release-candidate/raw/film/cut/film.mp4`: 34,900,930 bytes, 35.000 s, 1,050 frames, 1280x720 at 30 fps (ffprobe). Kept in ignored `raw/` with its 1,050 PNGs and `frames.tsv` (`docs/evidence-retention.md`: videos go under `raw/`). Not uploaded.
- Contact sheet: `film/contact.jpg`, 12 frames evenly spaced (frame i = k * 1049 / 11), 4 by 3, each labelled with its time, 326,636 bytes.
- No frame refused; exit 0 for both runs.
- Map: 64.4 % of the variance on the first two components. The README says 62.9 %.
- Person: out of the shadow by up to 3.50 m, largest move 0.200 m a frame.

| segment | frames | trees | build | draw | peak resident | device | cut 8 build |
|---|--:|--:|--:|--:|--:|--:|--:|
| 0 beech grown 1-40 | 210 | 40 | 12.6 s | 33.4 s | 1,265 MB | 1,003 MB | 8.3 s |
| 1 beech to spruce | 90 | 90 | 348.3 s | 15.1 s | 1,787 MB | 1,259 MB | 260.8 s |
| 2 spruce to birch | 90 | 90 | 489.2 s | 18.1 s | 1,798 MB | 1,259 MB | 346.4 s |
| 3 birch to oak | 90 | 90 | 233.1 s | 13.8 s | 1,570 MB | 1,003 MB | 143.8 s |
| 4 oak to palm | 120 | 120 | 65.0 s | 14.2 s | 1,520 MB | 747 MB | 44.0 s |
| 5 palm 40 to 1 | 90 | 39 | 0.4 s | 10.9 s | 1,367 MB | 747 MB | 2.2 s |
| 6 palm to Laurelin at 1 | 30 | 30 | 1.0 s | 2.2 s | 1,367 MB | 491 MB | 0.1 s |
| 7 Laurelin grown 1-80 | 210 | 80 | 169.7 s | 34.2 s | 6,310 MB | 1,771 MB | 11.8 s |
| 8 Laurelin to Telperion | 120 | 90 | 2,031.9 s | 38.9 s | 5,873 MB | 2,027 MB | 1,232.4 s |

Bounds pass (probe): segments 0-8 took 6.8, 434.7, 384.4, 213.9, 71.5, 0.4, 1.0, 200.1 and 1,976.1 s.

## For the host

- The Laurelin grown 1 to 80 peaks at 6,310 MB resident, against the queue's 8 GB cap (cut 8: 1,867 MB). Its build is 14 times cut 8's.
- Laurelin to Telperion builds in 1.65 times cut 8's time, and the walks at 40 in 1.3 to 1.6 times.
- The README's "62.9 %" no longer matches the map's 64.4 %.
- Not run: the oracle (`--oracle`). Unknown: whether the film's look is accepted; the trees were not judged here.
- The brief named `.flow/evidence/fn-351-the-release-candidate/`; on this branch the spec's directory is `fn-351-020-the-release-candidate`, and `film/raw/` is not ignored, so the frames are in `raw/film/` (FRICTION.md).
