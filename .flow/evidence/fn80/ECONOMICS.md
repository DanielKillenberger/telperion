# fn-80 economics: what a species costs and what would make it cheap

The goal this serves (owner, 2026-09-18): most tree species in the world,
onboarded by swarms of cheap, fast agents running the `add-species` loop. The
unit that matters is therefore cost per species at swarm scale, not the cost of
one careful run. A loop that is correct but expensive does not reach the goal.

This file is the judgment the numbers do not carry. The loop already records
gaps by route, rounds to acceptance, reversals, tokens, wall clock, Jev calls,
Firecrawl credits and captures (`gap metrics`, fn-63 R5). What it cannot record
is which step dominated, what a cheaper model would have done instead, and what
could be cached, batched or skipped across species. That goes here, as it
happens, one dated entry per observation, never reconstructed at the end.

Distinct from `FRICTION.md`, which records what slowed or hindered an agent.
This records what a step cost and what would make the next hundred species
cheaper. A step can be fast and still be the wrong place to spend.

## What every entry answers

- **Step.** Which stage or command, and what it was doing.
- **Cost.** The measured number: credits, Jev calls, tokens, wall clock,
  captures, driver dispatches. A number, not an adjective.
- **Dominates?** Whether this is a large share of the run's total, and of what.
- **Cheaper next time.** The concrete change: a cheaper model for this step, a
  cache across species, a batch across species, a step skipped when a condition
  holds, or an owner stop answered once for many species rather than per
  species.
- **Swarm reading.** What this costs when multiplied by a thousand species, and
  whether that number is acceptable.

## Anchors from before this run

| Run | Cost | Note |
|---|---|---|
| European ash pipeline, 2026-09-18 | 107 Firecrawl credits, 6 driver dispatches for a 2-dispatch path | the four defects behind the extra four became fn-75 |
| fn-34, three species, 2026-09-14 | 23 rounds, 15 capability dependencies | the reason for one species per spec |
| fn-13 task 5 | a full weekly quota, 22 full-forest captures | the reason for small before large |

## Entries

