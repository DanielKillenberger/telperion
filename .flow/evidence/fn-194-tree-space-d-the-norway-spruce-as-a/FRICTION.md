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
