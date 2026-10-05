# fn-195 friction

## 2026-10-05, task 1, round 1 tuning

- **Doing:** measuring the oak's height, width and dbh between value changes with `telperion-space`'s measures example (`examples/beech.rs`), and rendering with `telperion-render`'s `space_oak`.
- **Hindered by:** the two binaries live in different crates, so rebuilding one leaves the other stale; I measured once on the old values without noticing (the numbers matched the previous run exactly, which gave it away). The same trap fn-193 recorded ("one example for both stills and measures").
- **Cost:** about 3 minutes and one wasted measuring run.
- **What would remove it:** the stills runner printing dbh and the expected grown count beside its height and width, so one binary serves both; or a `cargo run` wrapper that always rebuilds.

## 2026-10-05, task 1, round 1 tuning (second worker)

- **Doing:** letting the oak's stem carry on through the fork as a weak central limb, and rendering seeds 1, 2 and 7 at 80 years.
- **Hindered by:** the leaf count fell from 2.4M to 0.5M at seed 1 (and 0.3M at seed 2) while the fine wood grew, so the crown read sparse. The cause sits in the renderer's dressing: leaves go on shoots thinner than `shootRadius` times the stem's radius, and `Tree::stem_radius` measures that radius where the stem first parts into two stem runs, which moved high up the thin central limb once the stem carried on. A values change in the tree space silently changed how densely the preset clothes it, and differently per seed (seed 7 kept its leaves).
- **Cost:** about 15 minutes and two render rounds to find; the stills rows now carry `shootRadius` 0.06 (was 0.015).
- **What would remove it:** an absolute leaf-bearing radius for tree-space stills (the tree space knows each short shoot by its PA), or the stills runner printing leaves per fine metre so the drop shows in the log.

## 2026-10-05: splitting one working branch into three PRs

**What:** fn-197, fn-207 and the oak were built interleaved on one branch, then cherry-picked into three stacked branches from origin/master.

**What slowed it:**
- Master had meanwhile restructured the still runner, which E's flags conflicted with on every commit. They were ported once as `tuning.rs`.
- Master had added `Form::secondary` and the palm, so every `Form` literal and the palm's state conflicted or failed to build.
- Two dcg blocks on restoring a file from another commit.
- E's intermediate commits between step 3 and step 5 do not build on master (the palm lacks E's fields until step 5); each branch's head does.

**Cost:** about 45 minutes and three branch test runs.

**What would remove it:** one spec per branch from the start, rebased on master as each sibling merges.
