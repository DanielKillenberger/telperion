# Gap loop: options, escalation and resume

## Goal

The owner's end state is one instruction, "add species A", after which an agent prepares the docs and artifacts, runs fn-58's pipeline, produces the species properly QA'd, and improves the generator where a missing capability blocks it. This spec is what happens at fn-58's halt on a gap (a capability no value table reaches): the agent or a stronger model writes typed candidate fixes, Jev answers narrow questions over them, a threshold table routes to proceed, a stronger model, or the owner, the chosen fix is minted as its own reviewed spec, and the run resumes where it halted. Per-run autonomy, quality and efficiency numbers are recorded, and owner reversals tune the thresholds.

## Decisions

- user (turn 21): "what i want in the end basically is: i open the repo with an agent i tell it to add species A and it will prepare all the docs and artifacts and go through the pipeline to produce species A in the generator properly QA'd and improved the generator to fix missing gaps that allow the species to continue."
- user (turn 21): "Jev should evaluate the options on how to fix the gaps and should evaluate also if the decision needs a better reasoning model or human to make the decision."
- user (turn 22): "should be as autonomous as possible but also as efficient and high quality as possible. Min max problem."
- user (turn 24): "in my view we're building the machine to serve the conductor. And the conductor only really is a run-book or smth? so some skill file that comes with the repo so i'd probably merge it."
- user (turn 26): "how do we create the manifest? don't we need a conductor for that?"
- user (turn 27): "ok sure"
- Owner, 2026-09-18: a gap routes to the owner when its fix would move a pin, change another species' output, spend a capture budget, lower a data-quality bar, or change a spec Boundary; everything else the conducting frontier model decides and records, and reversals tune the thresholds.
- Kept separate from fn-58 on finish lines (Jev and the standing reviewer agreed). Jev never writes an option, fix or number; no generator change inside a species run; no visual verdict by any route; no preference for the cheapest model.
- Escalation target on 2026-09-18: the routing block's reviewer tier (claude-fable-5-1).

## Acceptance Criteria

- **R1:** two to four typed options per gap halt; empty sets escalate to the stronger model then the owner — met (`pipeline/gap/option.rs`).
- **R2:** versioned option question set with labelled cases from fn-34's gaps; held-out bars 0.9 change kind and prior verdict, 0.8 best match — met (live: change kind 24/24 and 13/13 held out, prior verdict 23/24 and 12/13, best match 9/10 and 4/5).
- **R3:** route read from one threshold table over recorded signals, re-routable without a new call; missing signal routes to the owner — met (`pipeline/gap/{table,route}.rs`, `data/gap-routes.json`).
- **R4:** fix minted as its own spec; run resumes from the halted stage via expired keys; second needs-work files for the owner — met (`resume.rs`; a landing is tool version `fix:<spec>`).
- **R5:** per-run metrics record — met (`pipeline/gap/metrics.rs`).
- **R6:** repo-shipped "add species A" skill with two value rounds per verdict — met (`.claude/skills/add-species/SKILL.md`, `pipeline/gap/rounds.rs`).
- **R7:** one real species end to end with a real gap — not met; machinery exercised against a fixture transport in `tests/gap.rs`. The first live run is a dependent spec with its own species and capture budget.

## Result

The interrupted first worker's six modules were kept with two corrections: `resume::fix` no longer falls back to Jev's best match on an owner route; the question set's first live run missed its bars (change kind 13/16 held out, prior verdict 14/24) because `value_table` and `appearance` both claimed colour rows and `against` let one endorsement count against every other option. Criteria sharpened and mislabelled cases corrected to `none`; no bar moved. `jev cases` gained `--only labelled|pipeline|gap`; raw ledgers are gitignored at `.flow/evidence/*/ledger/`. 168 jev tests green.

## Open

R7, the first live species run through the loop (fn-80).

## Friction

- Interrupted worker left ~1,650 lines that did not compile (~25 min of reading) -> commit a compiling checkpoint before each next module.
- R7 needs a real species run the machinery's own task cannot close -> separate machinery from its first live exercise in acceptance.
- Workspace suite outruns an agent's 600 s foreground timeout (~25 min) -> per-crate Quick gates in specs; workspace run is CI's (later fn-185, fn-271).
