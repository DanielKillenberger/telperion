# fn-231 friction

## 2026-10-08: the spec's guess was wrong, and the row diff needed a single-row probe

- **Doing:** R1. I was checking the guess that a newly lived age causes the jump.
- **Slowed:** I put the newly lived ages back to unlived, and the counts barely moved: beech to oak at 1e-6 kept 1,652,207 against 1,652,214. A diff of the rows did not name the cause either, because every row moves by O(t). Finding it took a throwaway single-row injection probe (`walk_probe.rs.txt`), three rebuilds of the measurement example, and about 25 minutes.
- **Would have removed it:** checking the guess against the engine before writing it into the spec. The engine counts a fractional element whole by design (`closed_form/events.rs`, "counted whole, sized by the mean share"), and that design was already enough to predict a count jump with continuous size.

## 2026-10-08: the age-80 measurement is slow

- **Doing:** R1 at 80, six pairs, eight t values each, plus two checks per end.
- **Slowed:** one grown and length prediction off an end at 80 takes about 5 s for beech to oak, and longer on the ordinary tree's walks. That is the same cost fn-230 reported for `expected_counts`.
- **Cost:** the run takes several minutes in the background.
