# fn-283 result: a frond draws its rachis

Branch `fn-283-a-frond-draws-its-rachis`. Task .4 records the walk, the timing and the stills. The owner's verdict on the stills is the host's to collect.

## R7: the oak-to-palm walk at 40, seed 7

The test is `crates/telperion-core/tests/walk_rachis.rs`, `every_frond_leaflet_on_the_walk_lies_on_drawn_wood`. It took 18 s in the ci profile (`raw/walk-4.log`).

How it measures:

- Each frame is built as the film builds it: `blend::families`, then `mesh::build`.
- A second build at `cauline` 0 grows the same wood (asserted equal curve) and keeps only the rosette's leaves. Each of its packed leaves is found in the frame, and that match sorts every leaf into frond or station-borne.
- Each base is measured against the drawn curve: each ring's sphere, and the tapered capsule between consecutive rings of one run.
- The leaf positions are the stored ones, decoded from the 16-bit leaf box, so a palm-sized box adds up to about 0.25 mm.

Frond leaflets are held under 1 mm. Station-borne leaves are printed and never held (host, 2026-10-19). Distances are in metres.

| t | frond leaves | max | median | over 0.5 m | station leaves | max | median | over 0.5 m |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 0 | 0 | - | - | 0 | 72,112 | 1.9e-4 | 0 | 0 |
| 0.025 | 2 | 0 | 0 | 0 | 49,394 | 0.175 | 9.6e-6 | 0 |
| 0.05 | 4 | 0 | 0 | 0 | 12,389 | 0.350 | 4.5e-5 | 0 |
| 0.067 | 7 | 0 | 0 | 0 | 6,074 | 0.468 | 0.019 | 0 |
| 0.1 | 11 | 0 | 0 | 0 | 1,445 | 0.665 | 0.116 | 37 |
| 0.15 | 17 | 0 | 0 | 0 | 352 | 1.03 | 0.314 | 74 |
| 0.2 | 31 | 0 | 0 | 0 | 87 | 1.01 | 0.240 | 16 |
| 0.25 | 49 | 0 | 0 | 0 | 168 | 1.15 | 0.336 | 57 |
| 0.3 | 79 | 0 | 0 | 0 | 56 | 1.26 | 0.302 | 16 |
| 0.35 | 117 | 0 | 0 | 0 | 5 | 1.51 | 0.880 | 4 |
| 0.4 | 155 | 0 | 0 | 0 | 0 | - | - | 0 |
| 0.45 | 226 | 1.5e-5 | 0 | 0 | 9 | 2.39 | 1.18 | 7 |
| 0.5 | 310 | 3.9e-5 | 0 | 0 | 0 | - | - | 0 |
| 0.5125 | 351 | 4.1e-5 | 0 | 0 | 0 | - | - | 0 |
| 0.55 | 441 | 6.3e-5 | 0 | 0 | 0 | - | - | 0 |
| 0.562 | 485 | 5.0e-5 | 0 | 0 | 0 | - | - | 0 |
| 0.6 | 615 | 3.4e-5 | 0 | 0 | 0 | - | - | 0 |
| 0.75 | 1,557 | 1.1e-4 | 0 | 0 | 0 | - | - | 0 |
| 0.9 | 3,729 | 1.8e-4 | 0 | 0 | 0 | - | - | 0 |
| 1 | 6,554 | 2.5e-4 | 0 | 0 | 0 | - | - | 0 |

What the table shows:

- **Frond leaves.** At t = 0.5125 the base had 203 of 351 leaves more than 0.5 m off the wood. On the branch every frond leaflet stands within 0.25 mm of the wood at every share.
- **Station-borne leaves.** They match the base probe (`raw/walk-probe2.tsv`: 1,445, 87 and 56 leaves at t = 0.1, 0.2 and 0.3). The grid also finds some the base probe did not sample: 352 at t = 0.15 (74 over 0.5 m) and 168 at t = 0.25 (57 over 0.5 m). At t = 0.35 there are 5 leaves and at t = 0.45 there are 9, with a median of 1.18 m and 7 over 0.5 m. The 0.45 frame contradicts the base note "from t = 0.4 none remain". This is the open question the owner judges in the re-rendered film.

## R5: the palm at 40, the device's warm build

The command was `BUILD_PRESETS=date-palm BUILD_AGE=40 cargo test --profile ci -p telperion-render build_time -- --ignored --nocapture`, run on the RTX 3080 at seeds 1, 7 and 42, twice each (`raw/timing-4.log`).

Width 0 used a temporary `BUILD_RACHIS_WIDTH` env read in `build_time_tests.rs`. Both widths ran on the same binary. The env read was outside this task's files, so it was removed uncommitted (FRICTION.md).

| run | seed 1 ms | seed 7 ms | seed 42 ms |
|---|--:|--:|--:|
| branch, rachisWidth 0.25 | 94, 92 | 92, 89 | 85, 85 |
| branch, rachisWidth 0 | 90, 93 | 85, 90 | 94, 82 |
| base (fn-259 RELEASE.md, same machine) | 79 | 78 | 79 |

- The branch is under 0.5 s warm: 85 to 94 ms.
- The axes' own cost is within run-to-run noise. Width 0 and 0.25 overlap, at 82 to 94 ms.
- The branch at width 0 runs about 10 ms above the base's recorded 78 to 79 ms. The frond table moved to the boots stage in .2, and the base was measured on another day (inferred, not isolated). No base worktree was built here (host).
- Every run keeps 6,554 instances and a 34 MB device peak.

## Stills

The headless example, CPU `mesh::build`, 900x1200 (`raw/stills-4.log`). All four were viewed.

- `raw/stills/palm-40.png`: the palm at 40, seed 1.
- `raw/stills/palm-12.png`: the young palm at 12, seed 1 (the age .1 tested).
- `raw/stills/walk/frame-0002.png`: oak to palm at 40, seed 7, t = 0.5 (`--walk 0.125 --hold 0 --sweep 0`, so the three frames are t = 0, 0.5 and 1).
- `raw/stills/walk/frame-0003.png`: the same walk at t = 1.
- The sheet the verdict rests on is `stills-4.jpg`: the four stills in that order, 180 KB.

What they show:

- **Palm at 40 and at t = 1.** Each frond reads as one organ along a drawn rachis.
- **Young palm.** Its rachises are visible from the crown out.
- **Walk at t = 0.5.** Each sparse frond's leaflets now sit on a long, thin, bark-coloured arch instead of floating, so the crown reads as bare spokes with leaflets on them. The bark material is the spec's boundary; a leaf-coloured axis material is the later R5 item.

## space_hash (branch, `raw/space-hash-4.log`)

```
beech 40 1 nodes 114700 clothed a422971d7a88a787
beech 40 7 nodes 205400 clothed 46f3d85adbb98fab
beech 80 1 nodes 1625652 clothed e98ceb2b7c6933a3
beech 80 7 nodes 2249902 clothed 1191cb9c31cce090
spruce 40 1 nodes 994902 clothed 3503f9a6f60f3def
spruce 40 7 nodes 966173 clothed cb4247e12acab864
spruce 80 1 nodes 3482637 clothed d42cb2c38231da1f
spruce 80 7 nodes 3419862 clothed cda5593e133cc6fa
oak 40 1 nodes 275902 clothed 91723b112075f6be
oak 40 7 nodes 384593 clothed 49aad271c5f265db
oak 80 1 nodes 1388472 clothed b68d30d683f01813
oak 80 7 nodes 2203052 clothed 4b9e3c2d3d4c9d65
palm 40 1 nodes 7605 clothed 674df43b278e4ae6
palm 40 7 nodes 7605 clothed 457708eb5d45444c
palm 80 1 nodes 8085 clothed 7a2dcd203715da10
palm 80 7 nodes 8085 clothed 3150617b4d331d3a
```

The CPU mesh hashes per preset are in HASHES.md. Only the palm moves.

## Decisions recorded across tasks .1 to .3

- **The clothing tail bears no leaves (.1, conductor, after review).** `plan::bearing_of` bore cauline leaves on any slender wood, frond axes and retained bases included. The clothing's nodes (`tree.clothing` on, both executors) are now unbearing. The palm with cauline 1 bears the same leaves at width 0 and 0.25 (HASHES.md).
- **The palm's value moved to .3 (conductor, after the red gate).** The shipped palm kept rachisWidth 0 until the device drew the axes. In .3 it took 0.25 m (FAO 156 ch. I), and its pins were re-pinned.
- **The frond's wood is held to an angular lineage bound (.3, conductor).** A frond-axis run's point in the sweep test is held to 1e-3 m + 6e-4 rad × its reach. Every other run keeps 1e-3 m, and a frond crown's field wood is held to 0.995. The cause is the spiral reference that `rosette::frame` squares off a near-vertical crown axis, which magnifies the executors' 1.2e-6 rad lineage gap. The leaflets and bases share it. A better-conditioned reference is a core frame change and its own spec.
- **The palm's wood grew from 49,960 to 316,760 triangles at 40 (.3).**

## Observed while measuring, not acted on

`executor::expand` was given the walk's grown, clothed tree at t = 0.4 with `cauline` 0. It drew a different curve from a full build: curve height 15.74 m against 14.69 m, and the trunk's first ring 4 mm lower. The test now grows both builds instead. The cause was not isolated. The likely cause is that a handed-in tree's envelope reaches up to its frond axes (inferred).
