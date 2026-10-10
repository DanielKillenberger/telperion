# fn-253 friction

## 2026-10-09, task 1 (start)

- **`flowctl` was not on the worker's PATH.** The anchor call failed with "command not found"; I found the plugin's newest `scripts/flowctl` by listing the plugin cache. Cost: 1 minute. What would remove it: the dispatch naming the `flowctl` path, as the worker template's `FLOWCTL` field intends.
- **The worktree had no `target/`, so the first build is cold.** Two checkouts must not share one, so every example and test waits on a full dependency build first, under a 4-job cap on a machine with 3 GB of free memory. Cost: measured below when the first build lands.
- **Cost of the cold build:** 27 s for the core example and 82 s for the render tests, smaller than feared.

## 2026-10-10, task 1 (R2 values)

- **The organ-mask coverage cannot pick a flowering share.** With the florets cut tenfold (979,974 to 99,916 on Laurelin), whole-view coverage rose from 0.320 to 0.333. The measure that fn-248 tuned its floret values on does not follow the floret count, so the share was set by the inflorescence count and by viewing stills. Cost: one GPU round of four builds, about 1 minute, plus the image reads. What would remove it: a coverage measure that responds to the florets, for example the mask with the leaves and wood removed, or a fix to whatever saturates this one (unknown; not investigated).
- **The approved look sits in another worktree's ignored `raw/`.** fn-248's floret sheets, which the owner approved, exist only in `.worktrees/twigs`. A fresh worktree has no "before" image to compare against without rebuilding the base. Cost: 2 minutes to find them. What would remove it: keep the approved sheet (one JPEG) in the committed evidence.
