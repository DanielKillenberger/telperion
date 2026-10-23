---
satisfies: [R4, R5]
---
# fn-272-the-release-film-through-species-space.2 The README embed and the owner's verdict

## Description
Find and check the inline form GitHub renders (committed file or uploaded asset), within the size budget, with a still as fallback and a sentence or two of text (R4); the owner's verdict (R5).

## Acceptance
R4 checked on GitHub's rendering rules, size within budget; R5 the owner's verdict.

## Done summary
R4: README.md carries the film under the one-line description: the owner's user-attachments video URL on its own line (GitHub renders it as a player, checked through GitHub's own POST /markdown), a sentence on what the viewer sees ("Every frame grown live by the engine", the walk through species space, growing and shrinking, the corner map at 62.9 % of the variance), and the poster still linked to the film at the end of that sentence as the fallback. A linked still on its own line renders as a second player on GitHub, so it sits inline. No media committed; the size budget is untouched. npm publishes README.md (bare URL as a link, the linked still as an image; npm's rendering unverified until publish); no crate publishes a README. R5: the owner's verdict on cut 6, "looked good" (spec Decisions, 36c77b4eb). Codex impl review SHIP. npm test not run: no check reads the root README.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b698a3069
- Tests: gh api -X POST markdown (README lines 1-9 rendered by GitHub: one <video>, the still an <a><img>)
- PRs: