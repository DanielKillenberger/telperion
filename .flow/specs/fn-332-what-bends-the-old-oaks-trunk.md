# What bends the old oak's trunk

## Conversation Evidence

> owner, on the full oak at age 200, seed 1: "yea that's bad"
> owner: "But then are we able to realistically grow 300 year old trees? That's really the goal here."

## Goal & Context

Child of goal fn-321, serving R7 and R10. With the phytomer budget gone (fn-331), the full cycle grows the oak to age 200 in about 105 s, but its still at 200 shows the trunk rising a few metres, then bending near horizontal and carrying the crown far to one side. The full cycle has only been judged to age 80, and the route of 3 Nov rests on its old trees as references and training data. This child finds the cause before anything is fixed. [paraphrase]

## Acceptance Criteria

- **R1 (the trunk year by year):** for the oak at seed 1, and at two more seeds, at ages 40, 60, 80, 100, 120, 160 and 200, a results file gives the trunk's path from the ground to the crown: its lean from vertical at heights along it, its highest point, how far the crown's centre sits from the base, and which axes make the path (the seed axis, its continuations, or relays that replaced it). [inferred]
- **R2 (the cause, by test):** each candidate cause is switched off or held in turn at age 200, with everything else unchanged, and the same measures read: at least bending under load (sag), growth toward light, and a leader replaced by a lateral (relay). The result names the cause or causes that account for the bend, with the numbers. A candidate the engine cannot isolate is written as unknown. [inferred]
- **R3 (against a real oak):** the measures are set beside the sourced form of a mature Oregon white oak already in the goal (height, trunk, crown), and the still at 200 is kept with the results. [inferred]
- **R4 (the next step):** the results end with the proposed fix, as a parameter or a rule, and whether it changes trees at the preset age; no fix ships in this child. [inferred]
- **R5 (verification):** measurement only, unless a probe needs code outside evidence; then the gate is green and the codex review ends SHIP. [inferred]

## Boundaries

- No threshold of the goal lowered. Nothing merges to master, nothing is tagged or published. [paraphrase]
- Heavy work runs through `telperion-heavy`. [paraphrase]
- Species values and rules are not changed here; this child measures. [inferred]

## Decision Context

The route of 3 Nov 2026 needs full cycles at old ages as references, and AGENTS.md's "No artificial limits" removed the budget that hid them. [paraphrase]

## Open Questions

None.

## Result

Closed 9 Nov 2026 at `1a84b2af0`, measurement plus one probe file and one render fix. The old oak's bend is mostly sag with no reaction wood: with sag off, the crown's mean at 200 on seed 1 sits 7.9 m off the base against 15.7 m as shipped. No single cause accounts for all of it, since one chain of relays carries the crown whatever bends it, and why that chain dominates is unknown here. Switching the light off does not straighten the trunk. Height depends on the bend: 24.9 m as shipped and 32.1 m with no sag, past Silvics' 15 to 27 m. Proposed next, not built: wood that rights itself and several scaffold limbs sharing the crown; the host's review of 9 Nov adds a limit on height. Gate green on 9 Nov (test run 1,025 s of 1,560, every suite run). Codex's re-review of the join fix was SHIP. Evidence: `.flow/evidence/fn-332-what-bends-the-old-oaks-trunk/RESULT.md`.

## Friction

- A gate beside a worker's builds overran its budget (32 minutes). Disposition: the host starts a gate only when no other build or suite runs; no spec.
- The no-shade run outgrew the desk (about an hour). Disposition: local setup; the owner's MacBook takes heavy probes over SSH.
- git could not write its index because `/home`'s btrfs metadata was full (two hours). Disposition: local setup, fixed by the owner's balance.
- The gate overran again under another project's load (35 minutes). Disposition: `telperion-heavy` queues on memory and not on CPU; local setup, reported to the owner. It passed on a quiet desk on 9 Nov.
- The gate on the MacBook: the render crate did not compile on macOS (fixed, `01869223c`), and 18 tests fail there, one of them a tree whose digest differs from the desk's. Disposition: the Mac runs probes and is no gate machine; why it grows a different tree to the bit is unknown and worth its own spec if the owner wants macOS held to the desk. Two evidence files of closed fn-294 differ only by case and keep a Mac checkout dirty; renaming one is the fix, proposed and not done.

