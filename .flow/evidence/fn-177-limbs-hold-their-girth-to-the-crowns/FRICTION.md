# fn-177 friction

## 2026-09-27, worker, task .1 (R1 measurement, then a design return)

- Doing: measuring radius along the plane candidate's thickest limbs, split into the tip-count and length-taper parts, before building any row.
- Slowed by: (1) Nothing in the repository reads radius along a limb, so a measurement example had to be written first (`examples/limb_girth.rs`). (2) Radius feeds growth, so rebuilding a family with `lengthTaper` 0 changes the topology at some settings (3 of 8 seeds at `lateralShare` 0.01). The exact split is only available where the wood is unchanged, and the tool reports null elsewhere. (3) The command guard refused a loop that redirected to a shell-expanded path, and the loop had to be rewritten as a script.
- Cost: about 25 minutes, and no product code.
- Would have removed it: a radius-profile read in the species metrics (radius at shares of a limb's path), and a spec whose architecture claim ("per limb system") had been checked against the candidate. On this candidate the thick wood is codominant stems, which fn-61's limb bound does not cover. That check is exactly what R1 does, so running R1 before the spec was marked ready would have settled the scope first.

## 2026-09-27, worker, task .1 (two new dial rows re-record the beech tape)

- Doing: adding `girthHold` and `girthFall` to `/radii` with their dials, as the host decided.
- Slowed by: every new dial reaches the tune round's Jev request bodies, so the beech replay fails until the round is extended live (`bash -ic`, `species --extend`, 8 Jev calls). `tape_trim` on the extended copy also rewrote nine firecrawl pages that the run had not changed (their markdown grew), so only the new Jev answers could be copied in by hand. Finding the superseded answers took an `inotifywait` watch over a 40-second replay. The replay test also needs `target/ci/examples/headless`, which only the workspace gate builds, so a focused `cargo test -p telperion-jev` fails until the examples are built by hand.
- Cost: about 20 minutes and 8 Jev calls, for a change that moves no beech render.
- Would have removed it: the fn-170 remedy again (a tape key that does not hash the whole dial table), a `tape_trim` that is idempotent on a trimmed tape, and a replay that names the tape files it did not open.

## 2026-09-27, worker, task .1 (the axis reach was defined twice)

- Doing: building the hold to the host's rule (t = d / (d + L), where L is the longest path to a tip).
- Slowed by: when L runs through a node's laterals, a trunk whose leader ends in a whorl keeps its full girth up to its tip, so the stub ends blunt at 0.47 m. Taking L along the axis's own continuation fixed it. A first build that restarted the hold at a fork's primary as well stopped the girth at every fork. Both showed up only in the candidate measurement, not in the synthetic limb.
- Cost: about 15 minutes, two candidate measurements, and no images.
- Would have removed it: a synthetic test tree with a lateral whorl at a leader's tip and a codominant fork. The test now has the whorl case; the fork case is covered by the candidate measurement.
