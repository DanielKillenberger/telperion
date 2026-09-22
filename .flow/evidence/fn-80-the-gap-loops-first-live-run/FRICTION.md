# fn-80 friction

## 2026-09-22 — host: the conductor attached the driver's sentence as a spec

First live tick. The gap-loop dispatch's scope says "report the spec it minted in observed", the driver wrote a sentence there because no spec had been minted, and the conductor attached the whole sentence as a dependency and planned to design it. Cost: about 15 minutes, one code fix (attach only a spec the Flow tree holds), one test on the live record, and a replay of the two steps. What would have removed it: a result field that is typed, `minted_spec: Option<String>`, instead of a free sentence doing double duty. Recorded as a defect the live run found; the fix ships on this branch.

## 2026-09-22 — host: two pauses on a routine dependency before anything was tried

Second and third ticks. After fn-108 was attached, the conductor judged the spec routine (0.99) and then paused twice on the continuation check. The first pause priced the next attempt at one token because the only known usage was the driver's 0 (fixed: a zero usage is unknown). The second came from the continuation trio answering insufficient evidence on progress, risk and tractability, which is true by construction for a first attempt: there is no history to judge. fn-68's R11 settled this for the tuning loop on 2026-09-21 (a first bounded attempt proceeds without a judgment; only a repeat on moved evidence asks) and the conductor did not carry the rule. Cost: about 25 minutes, two resumes, one Jev trio spent on an unanswerable question. What would remove it: the same first-attempt rule in the conductor, applied below.

## 2026-09-22 — host: after the landing the conductor went after a halt the landing had cleared

Fourth tick. fn-108 landed, `gap resume` expired the gate stage's key, and the conductor's next action was the gap loop on the registry gate, the other halt the old gate run had filed, instead of rerunning the gate that would clear it. The plan read open halts before stale stages. Cost: about 10 minutes, one plan-order fix (once a dependency has landed and the stage fingerprint is stale, stages rerun before any halt is acted on). No Jev call was spent.

## 2026-09-22 — host: the gate left a passed halt open, and a landed round did not loop again

Fifth tick, after the gate reran on fn-108. Two things. The registry gate had passed (`gate.json` says `registry: true`) but its decision from the earlier run stayed open with its old inputs, because a rerun only replaces a decision it files again and never retires one it no longer files; the conductor saw an open halt. And the capability halt, narrowed to five names by the landing, was treated as "looped once, the owner's now", because the plan could not tell a landed round from a round that went to the owner. Cost: about 40 minutes, two fixes with tests: the gate stage retires the open decisions a rerun with changed inputs did not file again (`superseded`, by the stage), and a halt whose gap record shows a landed round loops again. What would have removed it: fn-63's gap loop had never been run twice on one halt; the second round is where both rules were missing.
