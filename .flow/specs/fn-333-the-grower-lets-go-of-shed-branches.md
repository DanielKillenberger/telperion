# The grower lets go of shed branches' records

## Conversation Evidence

> owner: "how can a tree be that memory expensive..? there must be a way to make that compress 100x at least?"
> owner: "what why would we store the history..?"

## Goal & Context

Child of goal fn-321, serving R7 and the route of 3 Nov. The full oak at age 200 peaks at 9.2 GiB while it keeps about 1.6 million live phytomers; it holds about 10 million axis records, most of them branches shed long ago. Retirement already frees a shed subtree's phytomers, but its axis records and the working arrays beside them stay, because axes are addressed by index. Nothing is known to read them once shed (to be checked). [paraphrase]

## Acceptance Criteria

- **R1 (where the memory is):** a breakdown of the oak at 200's peak by what holds it (axis records, their phytomers and units, the working arrays, captures, the census), measured, in evidence before any change. [inferred]
- **R2 (records let go):** the grower frees what it holds for a retired subtree, its axis records included, so memory follows the live tree. Every tree it grows is unchanged to the bit: the pinned digests at both ends of the dial hold for every preset, and the oak at 80 and 200 digests the same before and after. [inferred]
- **R3 (the gain):** peak memory and seconds for the oak at 80, 120, 200 and 300, seed 1, before and after, one thread. [inferred]
- **R4 (what still reads them):** any census count, capture or test that read shed records is named, and either reads what it needs another way or is stated as changed. [inferred]
- **R5 (verification):** the gate is green and the codex review of the child's diff ends SHIP. [inferred]

## Boundaries

- No threshold of the goal lowered. Nothing merges to master, nothing is tagged or published. [paraphrase]
- No change to any grown tree; this is storage only. [inferred]

## Decision Context

AGENTS.md, "Generator evolution": prefer byte-identical output for performance work. [paraphrase]

## Open Questions

None.

## Result

Closed 3 Nov 2026 at `0994e59b6`. The oak at 200, seed 1, peaks at 2.22 GB, down from 9.48; at 300 it peaks at 2.22 GB in 196 s. Every grown tree is unchanged to the bit (the oak's digests at 80, 120 and 200, the pinned digests, and a test that compacts every year). Gate green (test run 1,265 s of 1,560). Codex: three reviewers NEEDS_WORK on the footprint's axis tally, fixed; re-review SHIP. The 300 baseline was declined: the old grower needs about 18 GB there, the wait the owner ruled out. Evidence: `.flow/evidence/fn-333-the-grower-lets-go-of-shed-branches/RESULT.md`, `BREAKDOWN.md`.

## Friction

- The worker's isolated worktree was created on `master`, not the goal branch (a minute). Disposition: the host's dispatch brief now tells a worktree worker to reset to the goal head first; no spec.
- Staging the measurement-only half of mixed files by hand (10 minutes). Disposition: none needed; committing the measurement before writing grower code removes it.
- The queue's room was unknown before choosing a claim (5 minutes); the queue counts memory and not CPU, so timings under shared load varied 3x. Disposition: `telperion-heavy` is local setup, so reported to the owner, not specced.
- The 300 baseline needed 18 GB. Disposition: moot; the grower no longer needs it.

