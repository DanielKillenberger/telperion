# fn-277 R4: the film's walk frames and the stills

## The film's walk frames

Every walk frame of fn-272's cut 3 (`frames.tsv`, the eased shares it logged to four places) rebuilt on this branch as the film builds it (`blend::families`, `mesh::build`), seed 7: beech to spruce, spruce to birch, birch to oak and oak to palm at 40 (90, 90, 90 and 120 frames), palm to Laurelin at 1 (30). Every frame builds. Raw counts in `raw/film/` (ignored). Laurelin to Telperion at 80 was not rebuilt: both ends are the beech's engine species and the same canopy, so no count of organs moves on it.

| walk | frame pairs over 1.2 | where |
|---|---|---|
| beech to spruce | 0 | |
| spruce to birch | 0 (was 3: frames 323, 344, 366) | |
| birch to oak | 1 (was 4: 413, 435, 456, 463) | 0.906 to 0.916, 1.22: the steady slope CAUSE.md measured, 4 to 6 % at steps of 0.002 |
| oak to palm | 22 (was the jump to 4 leaves and its tail) | see below |
| palm to Laurelin at 1 | 19 | a steady fall from 396 leaves to the Laurelin's none, 1.2 to 1.4 a frame of 0.04 |

Oak to palm: the leaves now go by degree (stills below). The pairs over 1.2 fall in two kinds. From t 0.03 to 0.22 the crown's count falls steeply with the engine's walk of its wood (523,164 at 0 to 6,255 at 0.216): continuous at steps of 0.0005 (0.144 to 0.150: 4 to 6 % a step). From t 0.25 to 0.52, where the crown bears a few hundred to a few thousand leaves, the count moves 20 to 50 % between frames 0.0005 apart, up and down. That is the leaf-bearing wood itself: the same frames with every run cauline (`WALK_CAULINE=1`) step the same way (0.3285 to 0.329: 4,344 to 2,842), the crown alone does not (96 and 96; 148 to 156 over 0.3955 to 0.3975), and no whole row steps there. The engine's walk grows or drops whole branches of a sparse crown at those shares; no leaf form is involved.

## Stills (oak to palm at 40, seed 7)

`headless --family` (CPU build, native device), t = 0, 0.01, 0.05, 0.2, 0.5, 1; the sheet is `raw/stills/sheet.png` (ignored). Leaves: 523,164; 452,057; 186,525; 14,926; 474; 6,554. Viewed: the oak's crown thins and lifts by degree to 0.05, no frame bare; at 0.2 a single stem carries a thinning crown of small leaves; at 0.5 a tall bare stem carries a sparse crown of small fronds and scattered leaves; at 1 the palm. The sparse middle is the wood's and frond size's walk (fn-273's ground), not a switch.

## The per-bearer oracle on the film's walks (host ruling 5)

The oracle as built (`tests/walk_leaves.rs`, `forms`): each frame's leaves by form, each per its bearer, the wood's per metre of leaf-bearing wood, the short shoots' per tip, the crown's per stem apex; consecutive frames held to 1.2 where both bear the form. Run on fn-272 cut 3's walk frames, seed 7, with the device law of ruling 6 in place (`a_listed_walk_keeps_its_forms`, raw logs `raw/forms-seg*.log`).

| walk | frames | refusals |
|---|---|---|
| beech to spruce, 40 | 90 | 0 |
| spruce to birch, 40 | 90 | 0 |
| birch to oak, 40 | 90 | 0 |
| oak to palm, 40 | 120 | 19 |
| palm to Laurelin, 1 | 30 | 18 |

- **Oak to palm, the crown (10 of 19), t 0.028 to 0.156:** the crown per apex is 3 to 25 leaves: a frond or two of one or two leaflets on each of one or two apices. Each apex draws whole fronds and each frond whole leaflets, so one more frond or leaflet moves the count by a third to two thirds (5 at 0.0280, 3 at 0.0327, 5 at 0.0377; 13 at 0.1205, 18 at 0.1291).
- **Oak to palm, the wood (9 of 19), t 0.138 to 0.624:** leaves per metre of leaf-bearing wood rises with the leaflets (154 at 0.024, 3,358 at 0.612) and steps where the wood is a few short runs (1,116 at 0.4750, 812 at 0.4875; 3,358 at 0.6117, 18,429 at 0.6238, the last short run before the wood is gone at 0.636). A run shorter than an internode stands a whole internode's stations.
- **Palm to Laurelin at 1 (18):** the crown per apex falls steadily, 444 at t 0.175 to 2 at 0.987, by 1.2 to 1.4 a frame: the leaflets walk on the log scale from 113 to 1 (a factor of 0.81 over the film's 0.045 between frames) and the fronds from 42 to none. No frame steps against its neighbours; the bound was measured at steps of 0.001, where this walk moves by 1.005 a step.
