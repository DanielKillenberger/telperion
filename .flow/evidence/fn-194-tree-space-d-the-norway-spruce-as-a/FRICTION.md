# fn-194 friction

## 2026-10-04, task 1, rendering the spruce's sheet

- **Doing:** rendering the spruce at 20, 40 and 80 years while giving the branchlets the comb's hang.
- **Hindered by:** the engine refuses wood below the ground (`BelowGround`), which is right, but the error names only the kept axis's index. Finding which PA and which branch dipped (a seedling whorl, a medial branchlet, a branchlet on a low branch) took guessing and four rebuild-and-rerender loops, and a failure at one age stops the whole run, so the five-seed sheet is all or nothing.
- **Cost:** about 15 minutes and five rebuilds.
- **What would remove it:** the error naming the axis's PA, its birth cycle and its base height; and the stills example writing the other ages and seeds when one fails, reporting the failure at the end.
