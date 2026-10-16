# fn-154-agentsmd-is-the-one-instruction-file.1 AGENTS.md is the one instruction file

## Description
Move CLAUDE.md's full content into AGENTS.md and reduce CLAUDE.md to an import.

## Acceptance
- AGENTS.md holds every CLAUDE.md rule word for word; stale references updated.

## Done summary
AGENTS.md holds CLAUDE.md's content word for word; CLAUDE.md is an @AGENTS.md import plus one pointer line. Six references now point to AGENTS.md; .flow/meta.json no longer tracks a CLAUDE.md block. A fresh `claude -p` quoted the gate command and the mantra through the import.

## Evidence
- Commits: 9c8d133684ac2522e2b3ee70beb03425f8450d9e
- Tests: claude -p in the worktree; diff origin/master:CLAUDE.md AGENTS.md identical
- PRs:
