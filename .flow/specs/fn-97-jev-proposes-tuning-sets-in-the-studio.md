# Jev proposes tuning sets in the studio

## Conversation Evidence

> user: "Jev will propose tuning sets and the llm will set the parameters. You can then iterate this way and if you're unable to get something done create flow-next spec that would close the gap."
> user (selected): "Bring your own key"
> user (selected): "One shared table"
> user (selected): "Don't wait; Jev lights up later"
> user (selected): "fn-68; studio extends later"
> user: "ok go ahead do it" (accepted splitting the page rework and the Jev switch-on out of fn-94)

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 50% [user], 40% [paraphrase], 10% [inferred] -->

In the agent studio (fn-94) a person describes a tree and their agent sets the dials. The owner's idea has one more voice in that loop: Jev proposes tuning sets and the agent sets the parameters. Jev's part needs what the tuning loop (fn-68) is still building, an authored dial table with named steps and a validated adjustment question, so it was split out of fn-94 to let the studio finish first.

This spec switches the proposals on once both exist, and ends the period with two dial tables: the page's and fn-68's become one shared source that the page and Jev both read. Proposals need the person's own TypeSafe key; without one the studio works as fn-94 left it.

## Architecture & Data Models
<!-- scope: technical -->

- **Propose is one more studio tool.** Over the person's description, the agent's written observations and the recent rounds, Jev selects a named action per dial from fn-68's authored action set; code turns actions into numbers inside each dial's range. The agent applies the set, or part of it, through the studio's existing apply. [user]
- **Jev runs in the local studio process** through the shared caller with a ledger entry per call, never in the page, generation or rendering. [paraphrase]
- **One shared dial table.** The page's table (label, group, explainer, quick) and fn-68's (meaning, range, named steps) merge into one source; the page, the agent's tools and Jev read it. [user]
- **A round records the proposal:** the set Jev proposed, what the agent applied of it, and the ledger reference. [inferred]

## API Contracts
<!-- scope: technical -->

- **Propose:** description, observations and recent rounds in; a tuning set out, each entry a dial, a named action and the value code computed for it; or no-match; or insufficient evidence; or unavailable with the reason (no key, call failed). [paraphrase]
- **Shared table row:** the page's columns plus fn-68's meaning and named steps each way. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Jev selects; it never supplies a number,** and its confidence never scales a step (fn-68's rule). [paraphrase]
- **Jev reads text, never images.** The agent looks at the still and writes the observations Jev routes. [inferred]
- **The key is the person's own.** The studio never reads, echoes or stores it; without one the page says proposals are off. [user]
- **fn-68's question is validated on photographs and owner verdicts,** not on free descriptions. A proposal here is advice the agent may set aside, and the person's correction outranks it. [inferred]
- **Workspace tests keep the key unset** and use the mock transport. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Jev proposes a tuning set from the description, the agent's observations and recent rounds by selecting named actions per dial; code computes every value. Errors: no-match and insufficient evidence are valid answers and produce no change; a failed or missing Jev call leaves the studio usable without proposals; Jev never supplies a number. [user]
- **R2:** The page, the agent's tools and Jev read one shared dial table, and a check fails if a second copy exists. Errors: a dial present in one former table and missing from the merged one fails the check. [user]
- **R3:** Without a TypeSafe key the studio works as before and the page says proposals are off. Errors: the key never appears in a log, a session file or the page. [user]
- **R4:** Each round that used a proposal records the proposed set, what was applied and the ledger reference. Errors: a proposal that was ignored is recorded as ignored, never dropped. [inferred]
- **R5:** In one owner session on a described tree, proposals are on, and the owner records whether they helped reach the tree sooner than the agent alone. Errors: a verdict of no help is a valid outcome and is recorded, never tuned away. [inferred]

## Boundaries
<!-- scope: business -->

- No change to fn-68's loop, score or validation; this spec consumes its table and question. [paraphrase]
- No key held by the project. [user]
- No new dials. Whether the table covers enough dials for free-form trees is measured here, and widening it is its own spec. [inferred]

## Decision Context

Split out of fn-94 at the owner's word on 2026-09-20. The owner chose not to wait for fn-68, yet these criteria cannot be met until its table and question land; left inside fn-94 they would hold that spec open after its own proof was reached. The owner also chose that fn-68 authors its table undisturbed and the merge happens afterwards, which is here. [paraphrase]

## Strategy Alignment

- Serves "The catalogue": the owner's eye is spent on the final round and cheap judgment spends the rest. [strategy:The catalogue]

## Parked unknowns

- Whether fn-68's dial table, authored for the beech, covers enough dials for free-form trees. Reading the landed table against the generator's declared capabilities settles it.
- Whether a question validated on photographs and verdicts routes free descriptions well. R5's session answers it.
