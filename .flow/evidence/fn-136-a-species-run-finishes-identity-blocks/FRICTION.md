# fn-136 friction

## 2026-09-24 - worker, R2 (readiness leaves an improvement trait out)

- Doing: checking the spec's claim that `ready()` and the core-coverage gate refuse a failing core trait, then testing R2 on the palm's recorded state (fn-80 `conductor/tuning-1/run.json`, current visual).
- Hindered: the recorded state has three signals naming the date cluster, and code owns only one of them. The core-coverage gate's finding is code-derived and now leaves an unexpressed trait out. The other two are the reviewer's own: a `blocker` finding ("There are no hanging fruit clusters in either whole render.") and a defect prefixed `fruit-clusters-pendent:`. `ready()` refuses any blocker finding and any defect, and neither carries a trait id code can match. The spec did not foresee this, so on the recorded state the date cluster still blocks machine readiness through the reviewer.
- Cost: about 40 minutes of reading `state.rs`, `reference_first.rs`, `joint.rs`, the veto and the recorded run to establish it.
- What would remove it: a host decision on how the reviewer's own blockers leave readiness. Options: (a) findings and defects carry a `trait_id` the reviewer fills and code filters; (b) the comparison request names the unexpressed traits and the prompt tells the reviewer not to count them (changes the pinned prompt and calibration); (c) code matches a defect's `trait-id:` prefix (text parsing of model output).

## 2026-09-24 - worker, host decisions on R2 and bootstrap

- Doing: implementing the host's answer (typed `trait_id` on findings and defects, known gaps on the comparison request, a converged bootstrap reaching the packet).
- Hindered: the Claude twin of the reviewer adapter (`scripts/reference-first-claude.py`) exists only on the fn-80 branch, so this branch updates only the Codex adapter's schema. The twin must take the same v2 schema before the next live run. `Finding` and `Visual` literals in five test files needed the new fields added by hand.
- Cost: about 10 minutes.
- What would remove it: land the Claude adapter twin on master so that both adapters change in one diff.
