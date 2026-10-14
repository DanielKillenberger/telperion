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
