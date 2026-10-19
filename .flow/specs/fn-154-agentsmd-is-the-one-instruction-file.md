## Goal

Codex read AGENTS.md and Claude Code read CLAUDE.md, and the two had drifted (AGENTS.md was an older partial copy missing five sections). The owner asked for one instruction file so every agent reads one set of rules; moved out of fn-151 after Astra's review.

## Decisions

- owner (2026-09-25): "We should have in agents.md and claude.md (or can we just remove claude.md at this point, claude reads agents.md?) that the design of specs and implementation will be evaluated with jev to align with design principles."
- owner: "we can remove claude.md now ... claude.md is not needed anymore"
- owner: "ok capture the agents.md consolidation and work it first"
- AGENTS.md takes CLAUDE.md's content word for word; no rule is changed, added or reworded (principles guidance belongs to fn-151).

## Acceptance Criteria

- **R1:** Every CLAUDE.md rule appears in AGENTS.md word for word, nothing contradicts it — met (diff identical).
- **R2:** No CLAUDE.md of its own; a fresh tools-off Claude session states the rules from AGENTS.md — met (CLAUDE.md is an `@AGENTS.md` import plus one pointer line; `claude -p` quoted the gate command and the mantra).
- **R3:** References naming CLAUDE.md as a rule's home point to AGENTS.md — met (six updated).

## Result

AGENTS.md holds the full instructions; CLAUDE.md only imports it; `.flow/meta.json` no longer tracks a CLAUDE.md flow-next block. Commit 9c8d1336.

## Open

Whether a later flow-next setup writes its snippet back into CLAUDE.md was left to be checked when it next runs.
