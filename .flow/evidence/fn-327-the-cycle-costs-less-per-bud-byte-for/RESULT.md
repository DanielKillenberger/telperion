# fn-327 result: no byte-preserving lever reaches 50 ms

The cycle's cost has no single hot spot, and removing every allocation's cost saves at most 8% at the coarsest and 19% at the finest, measured on the oak. So byte-preserving work cannot close the gap from 165-259 ms to 50 ms by itself. The child ships the digests that pin the trees and the cost tests. No optimisation ships, because the one tried gained only 1 to 3%.

## R1: where the time goes

One thread, native, seed 1, the preset's age, second run. Milliseconds by pass. The timers and allocator are in `probes/`.

| Preset, fidelity | Total | Balance | Units | Sketch | Shade | Allot | Allocation calls |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Oak, 0 | 1,101 | 288 | 266 | 154 | 69 | 28 | 3.4 M |
| Oak, 1 | 165 | 41 | 33 | 25 | 15 | 8 | 1.0 M |
| Spruce, 0 | 5,204 | 1,324 | 971 | 909 | 217 | 143 | 18.2 M |
| Spruce, 1 | 789 | 226 | 179 | 116 | 36 | 24 | 3.5 M |
| Birch, 0 | 802 | 175 | 204 | 113 | 65 | 15 | 2.2 M |
| Birch, 1 | 165 | 35 | 33 | 21 | 24 | 5 | 0.8 M |

- **The pass called Balance is mostly retirement.** On the oak at the finest, of 272 ms timed inside it: opening the working set 20, the balance's totals 23, its laterals 8, its walk 1, applying it 3, freezing girth 71, judging closed subtrees 116, applying the verdicts 30.
- **At the coarsest the cost is per axis.** The oak grows 45 k axes and 91 k phytomers, about two phytomers an axis, in 77 k apex-cycles. Of its 975 k allocation calls, 772 k fall in that pass, about 17 per axis.
- **Allocation is not the lever.** With a bump allocator, whose free does nothing, the oak runs 885 ms against 1,090 at the finest and 152 ms against 168 at the coarsest. That is an upper bound for any allocation work, on one native run, and the run stopped on the spruce when the bump region ran out. Wasm's allocator was not measured and may cost more.
- **No other pass dominates.** Halving every pass in the table would still leave the oak near 80 ms before Wasm.

## R2: the same trees

An ordinary test pins the digest of every preset's tree at age 16, seed 7, at fidelity 0 and 1, in `cycle_cost_tests`. `digests-base.txt` holds oak, spruce and birch at preset age, seeds 1, 1234 and 5678, at both fidelities. Lever 1 left all of them equal. Reverting it restores the base code.

## R3: the gain

Lever 1 stopped retirement cloning a closed subtree's phytomers. Base against candidate, five alternating batches, held-out seeds, one pinned thread:

| Preset | Fidelity | Base s | Candidate s | Median ratio | Worst batch |
| --- | --- | --- | --- | --- | --- |
| Oak | 0 | 1.473 | 1.428 | 0.968 | 0.977 |
| Oak | 1 | 0.241 | 0.239 | 0.994 | 0.998 |
| Spruce | 0 | 4.730 | 4.675 | 0.988 | 0.991 |
| Spruce | 1 | 0.735 | 0.715 | 0.979 | 1.061 |
| Birch | 0 | 0.963 | 0.943 | 0.978 | 1.014 |
| Birch | 1 | 0.217 | 0.214 | 0.986 | 0.999 |

It took about 70 net lines and a new trait for 1 to 3%, so it was reverted (`AGENTS.md`, "Minimalist af"). The shipped cycle is therefore the base's, and against 50 ms it stands at 241, 735 and 217 ms on these batches.

## Leads, not taken

- A leaf subtree's verdict needs no local copy, which removes most of the 17 allocations per axis. Its ceiling is the allocation bound above.
- A coarse tree's cost follows its axes. The dial's `bud` row may only raise a lateral's probability, so no search has tried fewer laterals with more leaf on each. That changes the tree, so it belongs to the next child.

## Friction

- **`dcg` refuses `git checkout --` on a timer patch.** 1 minute; saving the probe as a patch and `git apply -R` is safer anyway. Disposition: none needed.
- **No sampling profiler.** `samply` needs `perf_event_paranoid` at 1 or less and the machine is at 2, so each question took a timer patch and a rebuild, about 5 minutes each, four times. Disposition: a local setup matter for the owner, reported and not specced: `sudo sysctl kernel.perf_event_paranoid=1` would remove it.
