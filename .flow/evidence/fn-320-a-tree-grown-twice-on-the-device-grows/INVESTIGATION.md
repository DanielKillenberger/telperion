# Investigation, 28 Oct 2026 (host-dispatched investigator, RTX 3080, native)

The device growth itself diverges; the field read-out and the renderer's device build follow it. One mechanism explains every class seen so far.

- **Where it starts.** Hashes after every stage in `resident/field/make.rs` and `cycle/tree.rs`: whenever the grid differed, the drawn records differed, and the earliest stage that differs is the grown state after `grow_hinted`. Array sizes, moves, compactions, retries, per-cycle census and node counts all matched; only float values differ.
- **Two states, not noise.** Each tree has one fixed alternate (ordinary 1 grown `887e…`/`3be9…`, ordinary 5678 `05b6…`/`b80d…`, oak 1 `8ca2…`/`412a…`), bunched in time: about 15 of 40 grows in one run, 0 of 240 in two others. A read that sees the old or the new value by GPU timing.
- **First divergence.** Ordinary seed 1, cycles 1-26 identical in 40/40; at cycle 27, 1 of 40 differs in axis 4422 and its child 10242 only: unit 23102's shade (low word of an f64), phytomers 33987 onward sized `0.99999994` against `1.0`. The size comes from `hier_cycle.wgsl:326` (`sizes_main`, `S_SIZE`) over the per-type sums of `partials_main`/`combine_main` (`hier_cycle.wgsl:245-300`), read by `draw_axis` (`lay_draw.wgsl`).
- **The renderer's device build.** Repeated `prepare_resident`: 3 of 75 digests differed, each on a grow with the alternate grown hash.
- **The 2.44e-4 leaves cell** is one fixed-point quantum (`UNIT = 4096`, `splat.wgsl:26`): a different drawn tree, not an order-dependent sum. One real order-dependence exists in splat's best-limb tie-break (`splat.wgsl:106-129`, `348-359`); never observed.
- **Not carried state.** Every grow makes its buffers anew (`io::buffer`); the kernels hold none.

Unknown: the exact dispatch and line; whether F4's birch "first grow differs" is the same mechanism. Instrumentation and logs: `/tmp/fn319/fn320-instrument.diff`, `/tmp/fn319/fn320-*.log`.
