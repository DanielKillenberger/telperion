# fn-231 friction

## 2026-10-08: the spec's guess was wrong, and the row diff needed a single-row probe

- **Doing:** R1. I was checking the guess that a newly lived age causes the jump.
- **Slowed:** I put the newly lived ages back to unlived, and the counts barely moved: beech to oak at 1e-6 kept 1,652,207 against 1,652,214. A diff of the rows did not name the cause either, because every row moves by O(t). Finding it took a throwaway single-row injection probe (`walk_probe.rs.txt`), three rebuilds of the measurement example, and about 25 minutes.
- **Would have removed it:** checking the guess against the engine before writing it into the spec. The engine counts a fractional element whole by design (`closed_form/events.rs`, "counted whole, sized by the mean share"), and that design was already enough to predict a count jump with continuous size.

## 2026-10-08: the age-80 measurement is slow

- **Doing:** R1 at 80, six pairs, eight t values each, plus two checks per end.
- **Slowed:** one grown and length prediction off an end at 80 takes about 5 s for beech to oak, and longer on the ordinary tree's walks. That is the same cost fn-230 reported for `expected_counts`.
- **Cost:** the run takes several minutes in the background.

## 2026-10-08: the host's whole-element rule meets fn-192's by-degree tests

- **Doing:** building the host's rule for fractional bud places on both executors and the closed form.
- **Slowed:** the rule is about 20 files deep in each executor, because the GPU's asks and table layout mirror the CPU word for word. Once built, it fails fn-192's by-degree walks, which the decision expected to bound it. The lifespan part reads two ways (unit or tick), and one of them overturns host decision 7.
- **Cost:** about 70 minutes, a 260 s space suite run and a 90 s GPU parity run. The work is parked on `fn-231-places-wip` rather than left on a broken branch.
- **Would have removed it:** running the decision against the walk tests before ruling. A 10-line probe that sets `zone.buds = 1.3` in `tests/walks.rs`'s species would have shown the jump.

## 2026-10-08: the second answer's lifespan reading did not match the engine

- **Doing:** building answer 2, the lifespan's last tick as a making draw.
- **Slowed:** the tick-by-tick measurement (one probe, two minutes) shows the count jumps at the first tick and is flat from 1 tick to half a cycle. The element is the partial unit, not a tick, so the instructed rule changes nothing, and it was the second round trip on the same question.
- **Cost:** a third escalation. Places and shedding are built and committed, which is about 40 minutes of the round.
- **Would have removed it:** the tick probe run before the ruling. It is `tick_probe.rs.txt`, and it takes under a minute.

## 2026-10-08: the fourth round, the tick grid against a draw's lead

- **Doing:** building the per-zone cut on both executors, the closed form and the kept tables.
- **Slowed:** three things the rule did not name had to be found by running.
  - The woken bud's growing axis is a continuation after slept stages, so origin alone could not tell it from a successor. It needed a flag on both executors.
  - The kept count jumped on the death cycle. That needed `Step::ends_at`.
  - fn-192's vanishing-size bound is tied to the tick (about 9.5e-7), which a log-odds lead over one tick of fraction overshoots.
- **Cost:** about 2 hours, four space-suite runs, two GPU parity runs, and a fourth escalation.
- **Would have removed it:** a probe of the rule against `branches_are_made_and_unmade_at_vanishing_size` on a lifespan walk before ruling. The crossing probe (`cross_probe.rs.txt`) takes 5 s.

## 2026-10-08: the fifth round, two more lifespan-made rests found by R3 at 80

- **Doing:** writing R3's convergence test at 40 and 80 after the merge.
- **Slowed:** R3 passed at 40 but not at 80. Two lifespan-made partial units were still crowded:
  - An age's last unit shortened by a draw-made room (a failed relay's rest). This was 6.7e-4 of the oak's kept count at one tick, and it is now cut from the room.
  - An ended age's relay rest. This was 1.4 % of the beech at 80, and it is now cut like a continuation's.
  Each needed a 12 s single-row probe at 80 and a tick probe with relays, abortion, failure, shedding and sleep switched off in turn to name it. fn-230's sizing tests also assumed every blend is oversized, which the continuous walk ends.
- **Cost:** about 50 minutes, two space-suite runs and two GPU parity runs. R3 itself runs 64 s, threaded over its 12 walks.
- **Would have removed it:** the tick probe parameterised by age and switches, kept as an example rather than retyped each round (`tick_probe.rs.txt`).

## 2026-10-08: the GPU lock held by another session's 45-minute sleep

- **Doing:** rendering fn-230's walk frames at 40 and 80 for R3's pop check, and then the render half of the gate.
- **Slowed:** the shared `gpu.lock` was held by `flock ... sh -c 'echo held; sleep 2700'`, with another session's `fixpass.sh` queued behind it. My first frame waited 26 minutes on its 30-minute `flock -w` before I stopped the run. I did not touch the holder.
- **Cost:** about 30 minutes waiting. In the meantime I timed the solve on the CPU (R5).
- **Would have removed it:** a lock that names its holder and purpose, for example a line written into the lock file, so a waiter can tell a deliberate hold from a stuck one. Or one queue that reports its position.

## 2026-10-09: the reboot took the scratchpad; the sixth answer's premise did not hold

- **Doing:** resuming after the reboot, building the presence share, re-solving the ordinary tree.
- **Slowed:** the base worktree and its target under the scratchpad (`base231`, `base231-target`) and the solve timers went with `/tmp`. I recreated what I needed: the timers are kept in evidence (`walk_solve.rs.txt`, `predict_bench.rs.txt`). The ordinary tree's answer assumed an arithmetic mean that the code does not use (`blend::size` is geometric), so the re-solve changed nothing and the answer's stop condition was hit at once.
- **Cost:** about 10 minutes, plus one more escalation round.
- **Would have removed it:** checking `blend::size`'s mean before ruling, or keeping base worktrees outside `/tmp`.
