## Conversation Evidence

> owner (2026-09-25): "We should have in agents.md and claude.md (or can we just remove claude.md at this point, claude reads agents.md?) that the design of specs and implementation will be evaluated with jev to align with design principles."
> host: AGENTS.md becomes the single instruction file, and CLAUDE.md shrinks to an `@AGENTS.md` import, so every agent (Claude, Codex, others) reads one set of rules. Moved out of fn-151 after Astra's review.
> owner: "ok capture the agents.md consolidation and work it first"

## Goal & Context
<!-- scope: business -->

The repository keeps two instruction files that have drifted apart. Codex agents read AGENTS.md and Claude Code reads CLAUDE.md, so the same repo gives different agents different rules. One instruction file removes the drift: AGENTS.md holds every rule, and CLAUDE.md only imports it. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-25 on master.** [checked]
- CLAUDE.md is 109 lines. AGENTS.md is 66 lines, an older partial copy.
- AGENTS.md lacks "Dispatch and escalation", "Token and evidence budget", "Code rules", "Mature trees are the product" and "Gates and checked claims".
- AGENTS.md carries an older friction rule, without the owner's 2026-09-23 amendment, and a shorter TypeSafe summary that points to CLAUDE.md.
- Both carry a flow-next snippet between its markers. CLAUDE.md also carries the model-routing block.

**The change.** [paraphrase]
- AGENTS.md takes CLAUDE.md's full current content: every owner rule, the flow-next snippet and the model-routing block. No rule is reworded.
- CLAUDE.md is removed. Claude Code reads AGENTS.md natively: verified 2026-09-25 in a directory holding only AGENTS.md, outside the repository, with every tool disabled. A fresh `claude -p` session stated the gate command and the mantra (owner: "we can remove claude.md now ... claude.md is not needed anymore").

**Unknown.** Whether flow-next's setup, run again later, writes its snippet back into CLAUDE.md; the implementer checks its behaviour and records it. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Every section and rule in CLAUDE.md on master appears in AGENTS.md word for word, and AGENTS.md holds nothing that contradicts it. Errors: a rule present only in the old AGENTS.md is carried over only if CLAUDE.md lacks it, and the PR names it. [paraphrase]
- **R2:** There is no CLAUDE.md. A fresh Claude Code session, run with tools disabled in a directory holding only AGENTS.md, states the rules. [paraphrase]
- **R3:** Nothing else in the repository refers to a CLAUDE.md section as the home of a rule; such references point to AGENTS.md. Errors: no error surface beyond stale references, which are updated. [paraphrase]

## Boundaries
<!-- scope: business -->

- No rule changes, additions or rewording. The principles guidance comes with fn-151.

## Strategy Alignment

- Serves "Our approach": lean, one source for each thing. [strategy:Our approach]
