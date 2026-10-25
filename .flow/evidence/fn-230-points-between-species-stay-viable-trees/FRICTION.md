# fn-230 friction

## 2026-10-08: the grown prediction makes a sized blend slow

- **Doing:** solving the lateral factor against the predicted grown phytomers as well as the kept ones (host's option A), then testing and building the age-80 walks.
- **Slowed:** `expected_counts` on a blend at 80 takes 0.3 to 7 s a call, 5 to 20 times `expected_kept`. Where the grown count binds, one `blend::families` call takes 50 to 60 s (oak to spruce at 0.75, spruce to ordinary). The first version of `space_walks.rs` ran for 309 s, and the slowest walk frame at 80 went from 15 s to 87 s.
- **Cost:** about 15 minutes of waiting on tests and frames, plus a second round of trimming the test to 100 s (the age-80 cases cut to t = 0.5 and one ordinary-tree walk).
- **Would have removed it:** a grown count as cheap as the kept one, for example a predictor that sums the totals without keeping a table per cycle for every substructure (inferred, not profiled). That is engine work outside this spec.
