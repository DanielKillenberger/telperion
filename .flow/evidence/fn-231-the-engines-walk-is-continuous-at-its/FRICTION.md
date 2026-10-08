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
