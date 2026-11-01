# fn-323 friction

- 2026-11-01, field measurement: I read order from `Tree`'s structural rule, which gave orders of at most 2 on a tree of 275,902 nodes, and caught it from the first output. It cost about 10 minutes. The tree carries no axis order, so an axis's order is read by joining on the phytomer's key. A node-level order on `Tree` would remove it, but no shipped code reads one, so none is added.
- 2026-11-01, field measurement: my nearest-cell bug cost one rerun, about 5 minutes of GPU-free time. It got into results because the first numbers looked plausible at the cuts that mattered, and only the extra probe's small subtrees exposed it. A test of the deposit before any measurement would have caught it, and it has one now.
