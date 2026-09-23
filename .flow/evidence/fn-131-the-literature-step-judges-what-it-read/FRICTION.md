# fn-131 friction

## 2026-09-23 - floors have thin labelled sets

- Doing: calibrating R2's selection floor and R3's level floor from labelled
  cases, as docs/typesafe.md asks, with no live Jev call allowed in the task.
- Hindered: the only labelled live answers are the palm run's ledger on the
  fn-80 worktree and the fn-57 selection runs. The palm ledger stores no
  state, so a described answer's sentence is known only where select.json
  recorded it (6 of 36 identities). The selection picks do not separate:
  crown width's correct "6-10m" came at 0.34 and 0.36, and the wrong picks
  came at 0.35 ("35 m", a maximum) and 0.66 to 0.72 ("0.6 m" for leaflet
  width). The calibrated floor is 0.34 and catches none of the wrong ones. No
  live sufficiency answer has a label, so the sufficiency levels take the
  plain most probable level with no floor.
- Cost: about 25 minutes reading ledgers and reconstructing labels.
- Would have removed it: the ledger keeping each judgment's state (or its
  sentences) beside the answer, and a labelling pass on a live run's picks
  before a spec asks for a floor. A sufficiency labelled set of live answers
  would let that floor be set too.
