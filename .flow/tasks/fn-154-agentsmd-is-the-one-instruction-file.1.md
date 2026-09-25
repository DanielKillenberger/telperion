# fn-154-agentsmd-is-the-one-instruction-file.1 AGENTS.md is the one instruction file

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
AGENTS.md now holds CLAUDE.md's full content word for word (the old AGENTS.md was an older partial copy missing five sections). CLAUDE.md is an @AGENTS.md import plus one pointer line. Six references that named CLAUDE.md as a rule's home now point to AGENTS.md, and .flow/meta.json no longer tracks a CLAUDE.md flow-next block. A fresh `claude -p` session quoted the gate command and the mantra through the import.
## Evidence
- Commits: 9c8d133684ac2522e2b3ee70beb03425f8450d9e
- Tests: claude -p in the worktree: quoted the gate command and the mantra through the @AGENTS.md import, diff origin/master:CLAUDE.md AGENTS.md: identical
- PRs: