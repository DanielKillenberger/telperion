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

## 2026-10-03, R1 round 1, stopping for design decisions

- **Doing:** the first probe round of the growth law on the beech, oak and spruce.
- **Hindered by:** the round's open questions (short shoots as wood or stations, the fine-wood comparison, the crown's ontogeny) are design judgments AGENTS.md reserves for the host, and the spec places the short-shoot one in R2, after R1 needs it. The task was dispatched as the whole spec (R1 to R7) to one worker.
- **Cost:** about 2 h of the 10 h timebox; five rounds remain.
- **What would remove it:** an R1 that names the short-shoot representation and the ontogeny model up front, or a probe task the host steers round by round, as fn-188's rounds were.

## 2026-10-03, R1 round 2, a runaway tree crashed the still

- **Doing:** rendering round 2's stills at seeds 1 and 7.
- **Hindered by:** the spruce at seed 7 grew to 597k nodes, and the headless renderer panicked: a compute dispatch of 100,753 groups exceeds wgpu's 65,535 limit. That still is missing, and the run lost a minute.
- **Cost:** ~2 min.
- **What would remove it:** the renderer splitting large dispatches, or the probe refusing to render a tree past a node count with an error naming it.
