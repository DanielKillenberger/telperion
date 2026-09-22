# fn-80 friction

## 2026-09-22 — host: the conductor attached the driver's sentence as a spec

First live tick. The gap-loop dispatch's scope says "report the spec it minted in observed", the driver wrote a sentence there because no spec had been minted, and the conductor attached the whole sentence as a dependency and planned to design it. Cost: about 15 minutes, one code fix (attach only a spec the Flow tree holds), one test on the live record, and a replay of the two steps. What would have removed it: a result field that is typed, `minted_spec: Option<String>`, instead of a free sentence doing double duty. Recorded as a defect the live run found; the fix ships on this branch.

## 2026-09-22 — host: two pauses on a routine dependency before anything was tried

Second and third ticks. After fn-108 was attached, the conductor judged the spec routine (0.99) and then paused twice on the continuation check. The first pause priced the next attempt at one token because the only known usage was the driver's 0 (fixed: a zero usage is unknown). The second came from the continuation trio answering insufficient evidence on progress, risk and tractability, which is true by construction for a first attempt: there is no history to judge. fn-68's R11 settled this for the tuning loop on 2026-09-21 (a first bounded attempt proceeds without a judgment; only a repeat on moved evidence asks) and the conductor did not carry the rule. Cost: about 25 minutes, two resumes, one Jev trio spent on an unanswerable question. What would remove it: the same first-attempt rule in the conductor, applied below.

## 2026-09-22 — host: after the landing the conductor went after a halt the landing had cleared

Fourth tick. fn-108 landed, `gap resume` expired the gate stage's key, and the conductor's next action was the gap loop on the registry gate, the other halt the old gate run had filed, instead of rerunning the gate that would clear it. The plan read open halts before stale stages. Cost: about 10 minutes, one plan-order fix (once a dependency has landed and the stage fingerprint is stale, stages rerun before any halt is acted on). No Jev call was spent.
