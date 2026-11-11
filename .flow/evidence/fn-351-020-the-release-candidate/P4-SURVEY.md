# P4 survey: where the grower reads its horizon, 11 Nov 2026

By a Fable agent, read-only, at the integration branch's head; condensed by the host. [code] read in source, [inferred] reasoned from it, [U] not determined. Paths are under `crates/telperion-space/src/` unless given in full. The measurement it starts from is fn-353's `R1-EQUALITY.md`.

## The finding

The requested age reaches growth through the presence windows and the sleeping-bud draw. The settle is already a function of the state and the year. The light layout skipped in the last year is deferred work the settle never reads.

## Growth-time reads of the horizon [code]

| # | Where | What | From | Changes |
|---|---|---|---|---|
| G1 | `grow.rs:201`, `presence.rs:48-66` | `Windows::new(species, request.age)`: every share is normalised by the expected tree at the requested age | fn-192 decision 3 (`24cfca867`, 4 Oct 2026) | every presence window |
| G2 | `presence.rs:71, 81, 92, 149` | the row index `(age + 1) − cycle`, the cycles left | same | same |
| G3 | `presence.rs:113-117` | a sleeping bud's expected wood over its waking years to the horizon | `c22e045fe` (5 Oct 2026) | node windows and the sleeper's presence |
| G4 | `grow/wake.rs:149`, `dormant.rs:77-79` | a sleeper that would wake at or after the horizon is never recorded | same | which axes stay open, so when a subtree closes and retires |
| G5 | `grow.rs:324` | `asleep` sized `age + 2` | | storage |
| G6 | `grow/cycle/paused.rs:203` | no layout or light in the horizon year | fn-270, fn-198, fn-210 | nothing a settle reads; it stops growth going on |
| G8 | `paused.rs:174`, `paused.rs:55` | growth past the horizon refused | fn-270 | |
| G9 | `telperion-core/src/pipeline/space/growth.rs:85`, `space.rs:59` | the expected-kept ceiling judged at the horizon | | a growth refused for its horizon though the year asked for fits |

Presences feed back into growth (leaf area reads `scale`, `grow/draw.rs:228`; the balance and allotment weigh buds by presence; a move under presence one also grows its relay; retirement evaluates closed subtrees from presences), so the horizon moves topology as well as size.

## Presence, and why it has a horizon

Each draw records how far past its bound it fell, in log-odds; its presence is `min(lead / window, 1)`, the window `clamp(6 × share × stake, 0.05 × stake, 2)` (`presence.rs:19-23`). `share` is the expected wood the draw decides from its cycle to the horizon over the whole tree's expected wood at the horizon. fn-192's record (`RESULT.md:98-154`): a fixed window popped or lost 35 to 75.6% of length; a window scaled by realised wood coupled draws; "the cycles left" was taken, and the requested age was the only end available. fn-270 lists "open-ended growth without a horizon" as open.

It serves a walk over a setting at a fixed seed and a fixed age (a branch enters at vanishing size, no step moves a measure by more than 30 times the step). No test walks age; the pipeline refuses a fractional age (`telperion-core/src/pipeline/space.rs:159`). Holders: `tests/walks.rs`, `tests/relays.rs`, `tests/death.rs`, `tests/closed_form.rs`, `tests/lineage.rs`.

## The settle, fractional ages, pause

- Every settle pass (mark, assign, girth, shed fades, thicken, place, sag, order) recomputes from a copy and reads nothing after the year grown [code]. All of it is "the tree at year y".
- Age 12.5 is a blend of the settled year 12 and year 13 of one growth by axis identity (`paused.rs:235-255`, `unfold.rs`). It needs two consecutive year states of one growth, not the horizon [inferred]. A new growth per year would break it.
- A growth for horizon H cannot go on to H' > H: stored presences used H's windows, later sleepers were never recorded, `asleep` has H + 2 buckets, `dormant.rs:77` underflows past the age, and year H's light was never laid.
- Cost of the grown-on path per drawn year beyond the cycle: a settle of the year before inside `grow_to` (`paused.rs:187-196`, the caller's last draw is not reused), a settle of the year, each a clone of every axis, draw and apex. A single growth settles once with no copy.

## Options seen by the surveyor

| Option | What | Cost and risk |
|---|---|---|
| A | Windows from a forward look fixed at draw time, a function of growth stage and cycle alone | every pinned digest changes; walk bounds re-measured; more young wood partly present at any asked age |
| B | Two presences: an age-free one for growth, a drawn one recomputed at each settle from stored leads | memory for leads; old wood's drawn size moves every year; retirement must keep leads |
| C | A one-cycle look | fn-192 rejected this shape: narrow windows pop |
| D | The horizon as its own trait, a reference age separate from the age asked for | smallest build; the tree still depends on a chosen age (owner, 13 Oct 2026: "is there a way to not have to fix maximum age?") |

All but D also need: the sleepers' horizon drop replaced by an age-free bound, `asleep` growing as needed, year y's layout run or deferred to the start of y + 1, and the ceiling judged per year grown.

## Only a build can answer

Under A: the length lost at the asked age, and whether the walk bound holds. Whether the expected-length table at a longer age reproduces the shorter one to the bit. How many more sleepers a horizon-free grower holds. The cost of the last year's layout. The CPU cost of a draw per year.
