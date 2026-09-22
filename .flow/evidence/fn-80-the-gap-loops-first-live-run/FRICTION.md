# fn-80 friction

## 2026-09-22 — host: the conductor attached the driver's sentence as a spec

First live tick. The gap-loop dispatch's scope says "report the spec it minted in observed", the driver wrote a sentence there because no spec had been minted, and the conductor attached the whole sentence as a dependency and planned to design it. Cost: about 15 minutes, one code fix (attach only a spec the Flow tree holds), one test on the live record, and a replay of the two steps. What would have removed it: a result field that is typed, `minted_spec: Option<String>`, instead of a free sentence doing double duty. Recorded as a defect the live run found; the fix ships on this branch.
