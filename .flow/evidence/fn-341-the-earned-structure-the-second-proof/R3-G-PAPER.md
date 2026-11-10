# fn-341 R3, step 1: G's paper check, and the stop it calls

Drafted by the worker on 10 Nov 2026, following `R2-ANSWERS.md` "How it is run", step 1.

**The beech's G trunk falls outside half to twice its range, so the run stops here and returns.** The oak's is inside at every age. The rule's own words: "If it falls outside half to twice a range at any age, stop and return with the arithmetic." No G, P or U probe has been run on a tree.

## How

- **The yearly leaf above 1.3 m.** `structure_trace` is in `probes/earned.patch`. It grows the preset once through `telperion_space::Paused` and settles it every year. Each year it reads the standing leaf carried by the trunk internode that spans 1.3 m. The leaf counted is fn-339's: each node's leaf while its years last, and clusters at a grown tip, by scale.
  - Runs: oak seed 1001 to 300, beech seed 1001 to 80, each once inside `telperion-heavy 8 --threads 4`. Logs are in the ignored `raw/trace-oak-1001.log` and `raw/trace-beech-1001.log`.
  - The oak's run took about 20 minutes, because it settles 300 times.
- **The rings.** `probes/paper_g.py` sums ring(t) = A_L(t) / (R N) over the years and gives the trunk at 40, 80, 120 and 300.

## Traits used

| trait | value | source | checked |
|---|---|---|---|
| R, *Q. garryana* | 0.23 m² cm⁻² (10 m, 40 yr); 0.37 (25 m, 250 yr) | Phillips et al. 2003, Funct. Ecol. 17, Table 1; the same values in Phillips et al. 2003, Tree Physiol. 23: "0.37, SE = 0.01 m2 cm–2 and 0.23, SE = 0.02 m2 cm–2, for the 25-m and 10-m groves" | full text |
| N, oak | Germany: 16.00 (< 100 yr), 20.40 (100–200 yr), 25.90 (> 200 yr); Europe 12.8 to 32.4 | Haneca, Čufar and Beeckman 2009, J. Archaeol. Sci., Table 1 "Sapwood statistics for different regions in Europe" (Hollstein's rows); "The average number of sapwood rings tends to decrease from west to east, i.e., from the British Isles (23.70–32.40) towards the Baltic States (12.76–18.20)" | full text of a review; the primary studies were not read. *Q. robur* and *Q. petraea*: no ring count found for *Q. garryana*, whose sapwood depth is 4.3 cm at 10 m and 4.9 cm at 25 m (Phillips et al. 2003, Tree Physiol.) |
| R, *F. sylvatica* | 0.331 m² cm⁻² | Bartelink 1997: "a leaf area-sapwood area ratio of 0.331 m2 cm-2" | full text |
| R, *F. sylvatica* | 0.54 (the same trees, 3–23 m) | McDowell et al. 2002, Oecologia 132, Table 1 | full text |
| N, *F. sylvatica* | 73 and 70 conducting rings at 108 and 104 years (dye) | Gebauer, Horna and Leuschner 2008, Tree Physiol. 28, Table 1: "Annual rings in sapwood 73", "70" | full text |
| N, *F. sylvatica* | every ring: "sapwood area was considered to be equal to basal area" | Bartelink 1997 | full text |

The oak's R is read at the tree's current height, linear between Phillips' two measured groves and held beyond them. That is the worker's reading of two measured points, and it is shown beside each constant.

## The oak (seed 1001), G trunk in cm

| age | height m | leaf above m² | cycle trunk | G: N 16/20.4/25.9 by age, R by height | N 16 | N 26 | R 0.23 | R 0.37 | published range | G over the range |
|---|---|---|---|---|---|---|---|---|---|---|
| 40 | 10.7 | 102 | 54 | **19.9** | 19.9 | 15.6 | 19.9 | 15.7 | White 28–40; Phillips' 40-yr *Q. garryana* 23 | 0.56–0.71 of White's lower end; 0.68–0.87 of Phillips' |
| 80 | 17.4 | 441 | 111 | **58.9** | 58.9 | 46.2 | 63.7 | 50.2 | 56–79 | inside (0.83 to 1.14 of the lower end) |
| 120 | 24.9 | 610 | 136 | **91.4** | 94.9 | 74.5 | 105.8 | 83.4 | 83–109 | inside (0.90 to 1.27 of the lower end) |
| 300 | 40.0 | 465 | 120 | **153.8** | 176.3 | 138.3 | 189.1 | 149.1 | 156–192 | inside (0.89 to 1.21 of the lower end) |

Inside half to twice at every age on every reading.
- **Against today's cycle:** about a third of the trunk at 40 and half at 80.
- **Over the years:** it never thins. The leaf above falls from 610 m² at 120 to 465 at 300 (the shed limbs), but the trunk goes on growing.
- **The host's rough figure was 24 cm at 40 and about 60 at 80.** The sums give 16 to 20 cm and 46 to 64 cm.

## The beech (seed 1001), G trunk in cm

| age | height m | leaf above m² | cycle trunk | N 70, R 0.331 | N 70, R 0.54 | N = every ring, R 0.331 | N = every ring, R 0.54 | sapwood = basal area, A_L/R (Bartelink's statement read directly), R 0.331 | published range (White) | best reading over the range |
|---|---|---|---|---|---|---|---|---|---|---|
| 40 | 9.5 | 20 | 24 | 3.2 | 2.5 | 4.8 | 3.7 | 8.8 | 32–48 | **0.08 to 0.27 of the lower end** |
| 80 | 17.3 | 483 | 107 | 21.2 | 16.6 | 21.9 | 17.1 | 43.1 | 63–96 | 0.26 to 0.68 |

Outside half to twice at 40 on every reading, and at 80 on every ring-summed reading.

**Why (measured, and fn-340's).** The beech carries 20 m² of leaf above its trunk at 40, against 102 m² on the oak of the same height. fn-340 measured the beech's leaf area index at 0.214 of the nearer end of its published range (Leuschner 2006, 5.6 to 9.5). By G's form the trunk follows the leaf, so a beech with a fifth of a real beech's leaf gets a trunk far too thin. The cycle's beech trunk (0.24 m at 40 on this trace) stands near White's range only because the pipes count wood, not leaf.

## What the stop leaves open, for the host

- **The beech.** G on the beech needs either its leaf moved toward the measured range first, which the spec puts out of scope ("The foliage is not changed here"), or the beech left out of G's sheet. The worker does not choose.
- **The oak** is inside the range on every reading. On the host's word the run can go on to G alone on the oak at 80.
- **N.** The oak's value is European oak's, from a review's table, since no *Q. garryana* count was found. The beech's two sources disagree: 70 to 73 conducting rings by dye, against no heartwood at all.
- **The young oak.** It sits below White's range at 40, at 0.56 to 0.71 of the lower end. That is the form's own consequence: a young stem has fewer than N rings, so its sapwood is less than A_L/R.

## Built and not yet run

`probes/earned.patch` holds G and P as the answers specify. Nothing in it has been run on a tree.
- **G** (`girth/rings.rs`): settles a copy every year, lays each section's ring by phytomer key, and reads the rings in `thicken` in place of the pipes.
- **P** (`sag::unheld`): eq. 39's held share, which scales the moment the sag turns, with H as the chord to the far tip.

One choice in it is the worker's and needs the host's word: the rings are laid over the primary wood. That primary wood is each phytomer's `established` pipe, the one existing row for a shoot's first-year radius. Without it, a current-year shoot is one ring of its own leaf, about 0.1 mm in radius.

**U's physiology is stated** (checked, full text):
- Ryan 1990: "Stem maintenance respiration was linearly related to live-cell volume … Sapwood contained greater than 80% of the total live-cell volume in stems".
- Ryan et al. 1994: "Estimated maintenance respiration was linearly related to sapwood volume".
- Unchecked: no full text read says outright that heartwood does not respire. Sprugel 1990 was seen only as a snippet.

U is not built.
