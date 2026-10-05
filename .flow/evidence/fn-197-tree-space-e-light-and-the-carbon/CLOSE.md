# fn-197 close, 2026-10-05 (host decision 28)

## Workspace gate

`cargo test --profile ci --workspace --no-fail-fast`, run after the review fixes: 1,060 passed, 1 failed, in 452 s.

- **The failure** is `telperion-core --test material_detail`, `every_shipped_young_wood_row_crosses_the_wire_the_page_sends_unchanged`. The oak's round 4 (`e1b6e1ea`, fn-195) gave `oregon-white-oak` a young-wood row and left the test's expected list behind. It is fixed in the oak's round 5 on this branch (host decision 29).
- **The beech's memory-ceiling test passed;** it did not flake.

## Codex review

`flowctl codex impl-review --base a3768d18`. The base is the oak's commit that E's work follows; this branch forks from its parent `9575c2bd`, and the review reads the branch's own diff.

| Round | Verdict | Findings |
|---|---|---|
| 1 | NEEDS_WORK | Four, all fixed in `bd44799e`: retained girth jumped at a branch's removal; allocation ignored balance-shed survival; the full lay dropped standing dead wood; retained radii left out the leaf term |
| 2 | NEEDS_WORK | Two regressions from those fixes, both fixed in `c101191e`: the full lay's shedding did not mark living apexes; the leaf mean's population switched at retained 0 |
| 3 | **SHIP** | All six fixed; R1, R2 and R3 met |

## Open

- **Decision 10:** sag's foliage term and light's `leaf_area` are two estimates of one thing. They are unified when E re-judges each species.
- **The palm's neutral hash under E** is not taken: its branch has diverged (R3.md).
- **The in-leaf walk of every setting** is a slow suite outside the gate.
- **Estimated, unsourced values:** `MEMORY` = 0.5; the oak's dead-branch persistence of 5 years.
- **Phase F:** seed 4's leaves a metre of fine wood (39 against 136) is a leaf-bearing marking item.
