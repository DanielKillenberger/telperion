# fn-143 friction

## 2026-09-24 - the native stage comparison outlives the foreground limit again

Doing: R2's native `generation_stages` medians, base against candidate, for every catalogue and in-work preset at seeds 1 and 7, three alternated rounds. The first attempt ran eight fixtures in one foreground call and hit the 590-second guard after five of them; `stages-compare.py` prints its JSON only at the end, so those five fixtures' numbers were lost apart from the stderr lines. Cost: about 10 minutes and a rerun split into one call per preset in the background. fn-124 reported the same slowness. What would have removed it: a stage selector on `generation_stages` (skeleton and cull without the mesh), or `stages-compare.py` writing each fixture's summary as it finishes.
