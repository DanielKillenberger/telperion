# fn-253 friction

## 2026-10-09, task 1 (start)

- **`flowctl` was not on the worker's PATH.** The anchor call failed with "command not found"; I found the plugin's newest `scripts/flowctl` by listing the plugin cache. Cost: 1 minute. What would remove it: the dispatch naming the `flowctl` path, as the worker template's `FLOWCTL` field intends.
- **The worktree had no `target/`, so the first build is cold.** Two checkouts must not share one, so every example and test waits on a full dependency build first, under a 4-job cap on a machine with 3 GB of free memory. Cost: measured below when the first build lands.
