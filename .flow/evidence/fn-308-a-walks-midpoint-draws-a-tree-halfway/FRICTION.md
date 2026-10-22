# fn-308 friction

## 2026-10-22, R1 curves: the five other walks killed at the 8 GB cap

What: `the_lateral_wood_against_t` over the five other film walks at three seeds, 11 frames each, in one admission. The probe spawned one thread per walk and seed (15 trees at 40 years at once), and `telperion-heavy 8` killed it after about 70 s with no output kept.
Cost: about 3 minutes and one admission.
What would have removed it: the probe bounding its own concurrency (a worker count, as `units_bound`'s `UNITS_WORKERS` does). Rerun walk by walk.

## 2026-10-22, candidate runs: moving and reverting outputs through the dcg hook

What: renaming one candidate's outputs (a glob loop of `mv`) and chaining a patch revert with a guard run that held the heavy command in a variable. dcg blocked both (dynamic `mv` paths; a dynamically assembled launcher).
Cost: about 3 minutes, two retries with literal paths and a script file.
What would have removed it: a candidate runner in the repository (a label and a patch in; apply, measure, revert) so no hand step renames or reverts anything.
