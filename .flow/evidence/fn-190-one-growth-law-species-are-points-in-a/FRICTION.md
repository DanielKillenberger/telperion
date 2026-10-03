# fn-190 friction

## 2026-10-03, R1, the probe's first tree

- **Doing:** growing the first tree from the root with the law, no scaffold.
- **Hindered by:** with markers drawn evenly in the envelope, the seedling's crown (the envelope scaled to an eighth) held almost no markers within a bud's perception radius of the root, so nothing grew and the run returned a one-node tree without an error. Each marker now carries the crown scale at which it joins.
- **Cost:** ~10 min, ~6k tokens.
- **What would remove it:** a grown tree of one node is an error in the probe driver, not a row.

## 2026-10-03, R1, Astra's review of the bands

- **Doing:** fixing the R1 pass bands before tuning, with Astra's review.
- **Hindered by:** Astra rejected the bands' role, not only their values: it would take most bands out of the acceptance count and add a visual and a developmental check. That changes the criterion's shape, which a dispatched agent may not decide, so the bands stand as diagnostics with Astra's checks added as gates, and the choice goes to the host.
- **Cost:** ~5 min; an open decision for the host.
- **What would remove it:** a spec that names which bands vote and which are diagnostics before Astra reads them.
