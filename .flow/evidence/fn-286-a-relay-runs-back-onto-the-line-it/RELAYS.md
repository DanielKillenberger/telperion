# fn-286: a relay runs back onto the line it takes over

Base: F4 at 28f495793 with fn-236's sourced sprig segment (ce22a336d). Probes: `relay_turns.rs` (every relay's drawn turn, seeds 1-10, `turns.sh`), fn-236's `seed7/oak_lines.rs` (the oak's limb lines, seeds 1-50, `seed7/lines.sh`), `relay_views.rs` (stills, `views.sh`). Raw lines in `raw/` (ignored); sheets in `sheets/`, viewed.

## The mechanism

`takeover`, a per-PA share: a relay's phytomers are pulled onto its parent's running direction at the parent's tip, by `takeover` times the relay's blend at its base, fading to none over the relay's own reach, as its straightening fades (`geometry::taken`, `Layer::step`). The pulls towards the vertical share what it leaves. 0 is today's tree to the bit: the oak's fifty lines at 0 equal the run before the row existed byte for byte, and `tests/takeover.rs` holds a tree without relays unchanged whatever the row says. The device holds it in four pencil columns (`P_TAKE`, `P_LINE`) and the PA form's twentieth word.

## Values, by measure (relay_turns, seeds 1-10)

A relay's *joint* is its turn off the parent's heading at its node to its first drawn phytomer; *chord*, to the chord 0.5 m on.

| preset | takeover | relays | joint ≥ 40° | chord ≥ 40° | joint p90 (median of seeds) |
|---|---|---|---|---|---|
| oak (7, 9; limb 6 kept at 0) | 0 → 0.75 | 11,956 | 3,173 → 2 | 167 → 0 | 57.2 → 20.0 |
| beech (2, 4, 6, 7) | 0 → 0.5 | 1,598 | 121 → 0 | 29 → 0 | 34.0 → 20.2 |
| Telperion (beech's) | 0 → 0.5 | 19,725 | 1,630 → 8 | 405 → 1 | 37.0 → 26.7 |
| Laurelin (beech's) | 0 → 0.5 | 18,213 | 1,515 → 7 | 381 → 1 | 37.1 → 26.5 |
| birch (5, 6, 7) | 0 → 0.5 | 2,649 | 4 → 0 | 50 → 7 | 34.3 → 17.6 |
| ordinary (0-4, 6, 7, 9, 10) | 0 → 0.75 | 997 | 242 → 1 | 79 → 0 | 52.7 → 19.9 |

- The beech's relays erect by its straightening (1.0 on the trunk and leader, 0.6 on its modules); at takeover 1 the two fight and joints of 40° or more rise again (beech 48, Telperion 5,883), so the beech family takes 0.5.
- The oak's limbs (PA 6) keep their relays' kinks, the tortuous course `oak.rs` asks for.

## The oak at 40, seeds 1-50 (oak_lines; drawn joins)

| per tree | F4 (median / max, seed 7) | sprig 0.031 | + takeover |
|---|---|---|---|
| young-stem lines reaching past crown95 | 13 / 20, 17 | 1 / 4, 4 | 1 / 4, 4 |
| farthest tip / crown95 | 1.40 / 1.63, 1.60 | 1.05 / 1.33, 1.13 | 1.04 / 1.32, 1.13 |
| relay turns ≥ 40° on those lines | 1 / 4, 3 | 1 / 4, 3 | 0 / 0, 0 |
| relay turns ≥ 40° on every line | 3 / 10, 5 | 3 / 11, 5 | 0 / 0, 0 |
| kinks > 45° (0.5 m chords), every line | 0 / 2, 2 | 0 / 2, 2 | 0 / 1, 0 |
| crown95 (m) | 3.35 | 2.96 | 2.96 |

Five kinks over 45° are left, one each at seeds 4, 11, 24, 34 and 42. Three are limbs' (PA 6) relays in close succession (51-62° over 0.5 m chords), the course `oak.rs` keeps. Two are on young-stem lines: two relays 0.13 m apart adding to 46° (seed 42), and a relay of 14° beside the line's continuation into its shoot age at 33° (seed 24). None is a single hook.

## Cost (CPU executor, the probe's build, ms and nodes, seeds 1 / 7 / 42)

| preset | today ms | new ms | today nodes | new nodes |
|---|---|---|---|---|
| oak | 718 / 1,057 / 859 | 751 / 1,064 / 811 | 280,024 / 371,210 / 284,328 | 275,902 / 384,593 / 285,930 |
| beech | 172 / 252 / 379 | 173 / 230 / 341 | 114,649 / 205,341 / 249,111 | 114,700 / 205,400 / 251,816 |
| birch | 499 / 940 / 602 | 504 / 958 / 632 | 242,346 / 355,757 / 301,033 | 242,453 / 355,649 / 300,787 |
| Telperion | 6,343 / 11,023 / 11,595 | 6,374 / 10,951 / 11,499 | 1,985,984 / 3,355,228 / 3,363,723 | 1,985,624 / 3,358,968 / 3,359,470 |
| Laurelin | 5,815 / 9,763 / 10,614 | 5,790 / 9,844 / 10,762 | 1,859,233 / 3,122,312 / 3,139,949 | 1,859,573 / 3,127,439 / 3,143,210 |
| ordinary | 263 / 375 / 435 | 244 / 322 / 357 | 86,353 / 130,350 / 152,170 | 86,361 / 130,355 / 152,178 |

These are the CPU build in one process, not the harness's warm device time; within run-to-run noise the row costs nothing.

## Stills (sheets/, today left, new right, seeds 1, 7, 42), viewed

- **Oak:** the long low wands and the left hook at seed 7 are gone. The crown is narrower than today's (crown95 3.35 → 2.96 m): the wands made much of today's width. The owner's call whether the narrower oak reads better.
- **Telperion, Laurelin:** limbs run straighter and reach further; the crowns read a little wider and more open (Laurelin seed 1 shows a gap in the crown's middle). Not clearly better or worse: the owner's call.
- **Beech, birch, ordinary:** no difference visible at the whole-tree view.
