# fn-362 R4: the CPU mesh before and after

Base 038ce5548 (a scratch worktree with its own `target/`), head 5b9c9fbd5. One run each. The build stopped here once on the spec's fourth stop; the host's ruling below resumed it.

## Method

A scratch probe (an uncommitted example, `fn362_probe.rs`, kept in the session scratchpad) builds the tree with `pipeline::build(family, Request { schedule: Serial, ..Request::mesh() })` under `taskset -c 5` (one CPU, so the sweep's own workers do not start) through `telperion-heavy 9 --threads 1`. It reads:

- Wood vertices by drawn radius class, as `BEECH-VERTICES.md` classes them: each ring of each curve run in the curve's order takes its run's sides (base: the cap, 20; head: `CurveRun::sides`), its class read from the ring's radius (`CurvePoint::radius`, the swept radius); a run's two caps go to its last ring. The sum equals `SurfaceMesh::vertices()` on every tree here (no run was split).
- Peak memory: `VmHWM` straight after the build returned, one CPU. This is not the figure `BEECH-VERTICES.md` took (5,875 MiB on all cores); compare base with head only.
- Surface seconds: the pipeline's own `rings_ms + wood_ms`, serial schedule, one CPU.
- FNV digests of the grown tree (bincode), the curve (every point and run), the leaves (reference and words) and the wood (chunk bytes).

The yield watcher paused heavy jobs from 03:47:05 to 03:48:06; the beech at 40 ran inside it (03:47:35 to 03:47:41), so its seconds are not timed. No other run had a PAUSE inside it. The base beech at 80 was rerun once (FRICTION.md).

## Totals

| Tree | Wood vertices, base → head | Head / base | Peak MiB, base → head | Surface s, base → head |
|---|---|---|---|---|
| European beech 80, seed 1 | 101,808,366 → 38,229,268 | 0.376 | 4,433 → 2,498 | 8.15 → 3.98 |
| European beech 40, seed 1 | 9,414,878 → 3,177,671 | 0.338 | 447 → 259 | not timed |
| Oregon white oak 120, seed 1001 | 59,036,786 → 24,386,601 | 0.413 | 2,610 → 1,555 | 4.35 → 2.07 |
| Norway spruce 40, seed 1 | 26,373,798 → 7,499,428 | 0.284 | 1,363 → 800 | 2.23 → 0.76 |
| Telperion 82 (its age), seed 1 | 56,874,230 → 27,999,949 | 0.492 | 2,526 → 1,634 | 4.76 → 2.41 |
| Laurelin 80 (its age), seed 1 | 53,674,434 → 26,924,079 | 0.502 | 2,411 → 1,595 | 4.51 → 2.50 |

## Vertices by drawn radius class, base → head

| Tree | under 1 mm | 1 to 2 mm | 2 to 5 mm | 5 to 10 mm | 10 to 20 mm | 20 to 50 mm | 50 to 100 mm | over 100 mm |
|---|---|---|---|---|---|---|---|---|
| beech 80 | 18,311,932 → 5,129,235 | 73,529,562 → 26,568,454 | 6,756,406 → 3,815,725 | 2,436,622 → 1,954,698 | 458,138 → 445,450 | 260,452 → 260,452 | 44,404 → 44,404 | 10,850 → 10,850 |
| beech 40 | 4,293,354 → 1,186,714 | 4,525,872 → 1,601,182 | 419,990 → 240,980 | 137,614 → 111,441 | 25,498 → 24,804 | 8,204 → 8,204 | 3,144 → 3,144 | 1,202 → 1,202 |
| oak 120 | 25,958,342 → 8,002,632 | 21,071,746 → 8,206,813 | 7,597,030 → 4,402,875 | 2,855,794 → 2,254,985 | 1,008,050 → 973,472 | 348,124 → 348,124 | 167,480 → 167,480 | 30,220 → 30,220 |
| spruce 40 | 19,992,316 → 5,011,739 | 6,059,840 → 2,229,567 | 131,422 → 70,799 | 74,900 → 72,548 | 108,940 → 108,395 | 2,220 → 2,220 | 1,720 → 1,720 | 2,440 → 2,440 |
| Telperion 82 | 5,301,942 → 1,742,687 | 24,689,488 → 10,574,077 | 22,411,634 → 11,514,959 | 2,196,328 → 1,910,736 | 1,438,666 → 1,421,318 | 587,122 → 587,122 | 202,762 → 202,762 | 46,288 → 46,288 |
| Laurelin 80 | 4,260,472 → 1,410,592 | 22,510,752 → 9,814,005 | 22,481,324 → 11,581,539 | 2,138,124 → 1,850,702 | 1,479,642 → 1,463,121 | 549,584 → 549,584 | 208,444 → 208,444 | 46,092 → 46,092 |

Rings per class are unchanged on every tree (the beech at 80: 905,745 under 1 mm, 3,662,066 at 1 to 2 mm, as `BEECH-VERTICES.md`). Every class from 20 mm up is unchanged to the vertex: those rings sit on runs cut at the cap, as before.

Runs by sides on the head, beech at 80 (of 244,103): 3: 5,140; 4: 10,282; 5: 20,223; 6: 83,553; 7: 76,895; 8: 18,530; 9: 20,104; 10 to 19: 9,442; 20: 934.

## Against the paper: the host's arithmetic corrected

The design expected, on the beech at 80: under 1 mm 18.3 M → about 4 M; 1 to 2 mm 73.5 M → about 25 M; total 101.8 M → roughly 40 M; return if off by more than a quarter.

| Class | Paper | Measured | Off |
|---|---|---|---|
| under 1 mm | about 4 M | 5.13 M | +28% |
| 1 to 2 mm | about 25 M | 26.57 M | +6% |
| total | roughly 40 M | 38.23 M | −4% |

The class under 1 mm is off by more than a quarter. Its 905,745 rings average 5.66 vertices each (caps included), against the 3 to 5 sides the paper gave rings that thin: a ring is cut with its run's count, and many rings under 1 mm stand on runs whose widest ring is 1 to 2 mm (6 or 7 sides). The rule as built is the rule as written ("a run takes one count for all its rings: the count its widest drawn ring needs"); the paper's class estimate read each ring at its own radius. This is a reading of the numbers above, not a measured breakdown of which runs hold those rings.

**Host ruling (13 Nov 2026):** "the build is right and my paper figure for the class under 1 mm was loose; go on and finish." The cause accepted is the one above: a ring under 1 mm on a run whose widest ring is 1 to 2 mm takes that run's 6 or 7 sides, and the paper costed each ring alone. The total (38.2 M against roughly 40 M) and the class of 1 to 2 mm (26.6 M against 25 M) are within the bound. This is the host's arithmetic corrected, not a fault of the build.

## Decisions the host accepted, with their reasons (13 Nov 2026)

- **The cap is `max(radialSegments, 4·lobes)`**, the surface's existing count (20 on every shipped preset, the "cap 20" of the design), not the bare `radialSegments` row (12). Below the cap the lobe floor does not bind, as the design's numbers already took.
- **The socket is still sized from the cap's inscribed radius** (`samples.rs`). Sizing it from the bearer's own count would change the sample radius the curve stores, and so the GPU's wood. A child's sunk ring may therefore stand up to 0.2 mm outside its bearer's faces: `pr (1 − cos(π/n)) ≤ FINEST`, since the bearer's widest drawn ring is at least `pr`. That is under `FINEST`, which the drawing does not show.
- **The needle test's plane tolerance** (`suite::surface_attachments`) is the sum of the per-axis half steps along the facet's normal, `Σ h_k |n_k|`: the true bound of a quantised point's distance off a plane. The largest single half step was too tight for a slanted facet. A correction, not a loosening; the derivation is in the test's comment.
- **The spruce's leaf digest moves** because its needles sit on the wood's faces (`surfaceContact` 1); its tree and curve do not.

## Pins upstream of the surface stage (R6, as far as measured)

The digests of the grown tree, the curve and the leaves are equal on base and head for all six trees but one: the Norway spruce at 40 keeps its tree and curve digests (2caa72b377c06942, b4f35f4dbc40a18b) and its 3,484,945 leaves, and its leaves' digest moves (1cbbacca7ae4ff02 → e3df37eee36b55d8). The spruce is the one shipped preset whose leaves sit on the wood (`surfaceContact` 1; READING.md item 8 named the oak, which was wrong), so its needles are seated on the new polygons, downstream of the surface stage. The wood digest moves on all six. The committed species digests are listed in PINS.md.
