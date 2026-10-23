# fn-310 result: a person for scale and a nicer world around the tree

Work in progress: task .7 (the person placed per shot and wholly in frame) is open. This file grows to the full result when the spec closes.

## Open

- **The film's write time rose by about 52 ms a frame, from the sky's dither (host, 2026-10-23: accepted, nothing changed).**
  - **The rise:** cut 7's draw time grew from 87 s (cut 6) to 141 s over 1,050 frames. That is about 55 s on a 38-minute offline render.
  - **Where it is:** in the film's own PNG encoding, not in the shipped renderer. On the film path, render and read-back are unchanged (base 1.06-1.09 s, HEAD 0.99-1.05 s for 75 renders), and the sun's map is drawn 24 times on both.
  - **The cause:** encoding a frame went from 20.8 to 74.1 ms. The sky's dither, about one 8-bit step of per-pixel noise, accounts for 48.0 ms of the rise. The ground mottle and haze account for about 5 ms, and the person costs nothing measurable.
  - **Why the dither stays:** it stops the sky banding, a look the owner approved.
  - **The details:** `FILM-DRAW.md`.
- **File size.** The dither makes each PNG frame about 2.2 times larger (161 KB to 356 KB). Cut 7's mp4 is 5 % larger than cut 6's (32,908,133 B against 31,324,321 B). If cut 8's README encode comes out over GitHub's 10 MB limit because of the dither, the host is told then.
- **`film --frames <list>`** (task .6) renders chosen frames through the film path and builds only the segments they need. It is kept as the friction fix that FILM.md's earlier entries asked for. Its frames are byte-identical to a full run's.
- **A step in the tree, not the person (host's finding to take up; not fixed in fn-310).** In the film's spruce walk the crown width jumps from 8.81 m at frame 241 to 11.19 m at frame 242, a 2.4 m step in one frame. Task .8 measured it from `frames.tsv` and the strip `film-d7-strip-240.jpg`. fn-308's by-degree rule should have caught a step like this in the tree itself.
