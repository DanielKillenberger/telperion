# fn-194 friction

## 2026-10-04, task 1, rendering the spruce's sheet

- **Doing:** rendering the spruce at 20, 40 and 80 years while giving the branchlets the comb's hang.
- **Hindered by:** the engine refuses wood below the ground (`BelowGround`), which is right, but the error names only the kept axis's index. Finding which PA and which branch dipped (a seedling whorl, a medial branchlet, a branchlet on a low branch) took guessing and four rebuild-and-rerender loops, and a failure at one age stops the whole run, so the five-seed sheet is all or nothing.
- **Cost:** about 15 minutes and five rebuilds.
- **What would remove it:** the error naming the axis's PA, its birth cycle and its base height; and the stills example writing the other ages and seeds when one fails, reporting the failure at the end.
- **Since done:** the refusal now names the PA, birth cycle and base height (fn-200).

## 2026-10-04, task 1, round 3, density by values

- **Doing:** rendering an 80-year spruce whose branchlets live 70 years.
- **Hindered by:** the headless renderer panicked in wgpu instead of refusing: "dispatch group size dimension ([74572, 1, 1]) must be less or equal to 65535". Too many foliage instances overran one dimension of a compute dispatch. The tree grew fine, but the render crashed after 20 s of growing and dressing, with no message saying which limit or how many instances.
- **Cost:** about 5 minutes, plus a rows change (needle spacing 2.5 mm to 3.5 mm) to stay under the limit.
- **What would remove it:** the renderer splitting large dispatches over a second dimension, or refusing by name with the instance count and its limit before submitting.

## 2026-10-04, task 1, round 8 renders

- **Doing:** rendering round 8, one 80-year tree per process, back to back.
- **Hindered by:** "wgpu error: Out of Memory" on trees that had rendered before or rendered again moments later with the same binary and values (seeds 1 and 7 at 3.2 mm needles; seed 2 at 3.0 mm). The limit sits near 19M to 20M needles plus about 55 km of wood. Whether a tree fits seems to depend on what the GPU still holds from the previous process. Other desktop apps hold about 1.4 GB of the 10 GB card.
- **Cost:** about 15 minutes: two full re-renders and a retry.
- **What would remove it:** the renderer refusing by name with its memory estimate before it allocates (as fn-201 did for the dispatch limit), and a short wait or a GPU-idle check between processes in the stills runner.

## 2026-10-05, task 1, round 9 probes

- **Doing:** probing the bough curve, one 80-year tree per process.
- **Hindered by:** GPU out of memory again, now on most attempts. Other desktop apps hold 1.4 to 2.6 GB of the 10 GB card, varying from minute to minute, and round 8's trees (17M to 18M needles, about 50 km of wood) sit at the edge. Probe 9b's first variant failed three times with 20 s pauses, and probe 9d failed twice.
- **Cost:** about 25 minutes of failed renders.
- **What would remove it:** a renderer memory estimate that refuses by name before allocating; rendering the stills' needles in batches; or the owner closing GPU-heavy apps during render runs (local setup).

## 2026-10-05, task 1, round 11

- **Doing:** moving foliage mass into hanging branchlets under the needle budget.
- **Hindered by:** the GPU's limit is set by wood triangles (about 210M per tree at 80 years), not needles. The still runner's mesh print showed it. More spur shoots add wood, so they ran out of memory twice even with fewer needles.
- **Cost:** about 10 minutes and two failed renders.
- **What would remove it:** fewer cross-section segments on fine wood in the dressing (a level of detail for twigs under a millimetre), which would free the memory for foliage.
