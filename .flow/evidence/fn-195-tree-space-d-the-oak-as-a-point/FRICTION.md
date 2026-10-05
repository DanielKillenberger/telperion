# fn-195 friction

## 2026-10-05, task 1, round 1 tuning

- **Doing:** measuring the oak's height, width and dbh between value changes with `telperion-space`'s measures example (`examples/beech.rs`), and rendering with `telperion-render`'s `space_oak`.
- **Hindered by:** the two binaries live in different crates, so rebuilding one leaves the other stale; I measured once on the old values without noticing (the numbers matched the previous run exactly, which gave it away). The same trap fn-193 recorded ("one example for both stills and measures").
- **Cost:** about 3 minutes and one wasted measuring run.
- **What would remove it:** the stills runner printing dbh and the expected grown count beside its height and width, so one binary serves both; or a `cargo run` wrapper that always rebuilds.
