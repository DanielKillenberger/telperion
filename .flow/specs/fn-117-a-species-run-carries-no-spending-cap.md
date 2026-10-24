# A species run carries no spending cap

## Goal

On 2026-09-23 the date palm's run stopped for a cap three times (conductor 2 M, 6 M, 10 M) and the tuning loop twice more on its own token, evaluation and round caps, each needing a decision restating new numbers. The owner wants the caps gone while the loop is built, runaway guards kept, and spend still recorded so a usual loop's cost can be learned.

Owner quotes (2026-09-23):
> "Can we stop with the budget pls? We're still building it. I think once we have run some loops to completion we can figure out what a usual loop would cost and start flagging ones that are completely out of line. But let's keep going for now, note when we're inefficient and try and fix that for later runs. Avoid runaway quotas but drop the cap."
> On a CLAUDE.md rule saying not to ask: "but can't we just remove the thing that was asking for it instead of writing a NOT rule" (PR #72, which wrote that rule, was closed).

## Decisions

- Host design (2026-09-23): every conductor and tuning cap becomes optional; absent means no cap, a set cap behaves as before. Shipped configs drop their caps; the conductor needs no `budget` block. Pilot authority and resume compare only caps a config sets.
- Spend recorded as before. Runaway guards kept, plus a new one: `runaway_rounds` (default 5) consecutive rounds with no kept adoption pause the run, naming rounds and spend.
- Capture budgets in AGENTS.md unchanged.

## Acceptance Criteria

- **R1:** Conductor config with no budget block runs, nothing pauses, spend reported — met.
- **R2:** Uncapped tuning has no "cannot fit" pause; pilot and resume carry no cap numbers — met.
- **R3:** A set cap still pauses — met.
- **R4:** `k` rounds keeping nothing pause as a runaway — met (red without the guard).
- **R5:** Gate green — met (658 passed).

## Result

Landed in `e0e157f3` (`tuning/caps.rs`, `tuning/runaway.rs`); carried into the fn-149 rewrite (#121), which kept the no-progress stop. Host-check decisions recorded: `next_tokens` asked only when a token cap is set (unknown otherwise); a fresh run's visual counter opens at 0; resuming a capped run with an uncapped config drops its caps.

## Open

None.
