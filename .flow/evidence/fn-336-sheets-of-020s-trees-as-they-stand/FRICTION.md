# fn-336 friction

## 2026-11-09 23:13 Zurich: the GPU queue waits while the owner games

Doing: the first still (oak, age 40, seed 1) through `telperion-heavy 6 --gpu 2`.
Slowed by: the wrapper's GPU admission holds every `--gpu` job while other processes hold half the card or more. World of Warcraft held 3.7 GB of the RTX 3080's 10 GB and the desktop apps about 2 GB more, 6.0 GB in all, so the job waited from 23:13 with 32 GB of RAM room. The wait is the wrapper doing its job (the owner games), not a defect.
Cost: about 6 minutes. The stills job was admitted at 23:19:33 and the CPU-only trunk probes (R2) ran in the meantime.
What would remove it: nothing in the repository; it is the desk's rule. A note in the handover so the host can schedule GPU children outside play.

## 2026-11-09 Zurich: no headless entry draws the website's voxel style

Doing: R1's grove sheet "on the website's voxel style".
Slowed by: the voxel style exists only as `src/field/voxelize.ts`, which "draws nothing", and the site that draws its cubes is not in this repository; no example in `crates/*/examples` renders cubes. Building one is a tooling change R6 returns to the host, and the dispatch said to return the other views instead.
Cost: the grove sheets are not made; about 5 minutes of search.
What would remove it: a headless voxel still, specced by the host. One possible shape: the field at the site's grid side, its cubes drawn by the native renderer or a small Node rasteriser.

## 2026-11-09 23:30 Zurich: the hero pose clipped two old oaks' bases

Doing: R3's screen of the old-oak sheet.
Slowed by: at seed 7 the space still example's hero pose cut the trunk base off at 300 and grazed the flare at 200. That example takes no camera, so each tree was grown once more through the headless still, with the age laid over the preset and `--camera` pulled back. The leaf counts confirm it is the same tree.
Cost: about 10 minutes, of which 4 were the two grows.
What would remove it: a `--camera` row on the space still examples, as headless has, or a hero pose that keeps the base in frame on a tree whose crown leans far off it. This is a tooling change for the host to weigh; it was not made here.
