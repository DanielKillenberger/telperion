# fn-327 result: the time is spread, and the one lever tried was not worth keeping

No pass or operation dominates the cycle. The one optimisation tried saved 0 to 3%, so it was reverted, and the cycle that ships is the base's. A bump allocator saved 8% on the coarse oak and 19% on the fine oak. These measurements do not prove that byte-preserving work cannot reach 50 ms, since they do not bound what could be gained by rewriting the computation inside each pass. They do show there is no single fix, so this attempt stops here and the next one reduces the work itself.

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

Inside the three largest passes, in milliseconds. Each table is one timed run; the timers add their own cost, about 15 to 25% to Units, so read the shares and not the totals.

The pass called Balance is mostly retirement:

| Preset, fidelity | Open the working set | Totals | Laterals | Walk | Apply | Freeze girth | Judge closed subtrees | Apply verdicts |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Oak, 0 | 20 | 23 | 8 | 1 | 3 | 71 | 116 | 30 |
| Oak, 1 | 3 | 3 | 1 | 0 | 0 | 6 | 19 | 3 |
| Spruce, 0 | 106 | 142 | 42 | 5 | 18 | 247 | 488 | 186 |
| Spruce, 1 | 23 | 30 | 9 | 1 | 2 | 34 | 88 | 31 |
| Birch, 0 | 13 | 18 | 7 | 1 | 2 | 49 | 89 | 21 |
| Birch, 1 | 2 | 2 | 1 | 0 | 0 | 7 | 16 | 3 |

Units and Sketch:

| Preset, fidelity | Decide | Zone setup | Node draws and sort | Lay phytomers and sprout | Stop or move on | Draw wood | Frame new axes | Gather leaves |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Oak, 0 | 50 | 39 | 44 | 134 | 10 | 100 | 30 | 26 |
| Oak, 1 | 6 | 10 | 6 | 12 | 1 | 12 | 8 | 3 |
| Spruce, 0 | 271 | 161 | 154 | 423 | 53 | 572 | 169 | 137 |
| Spruce, 1 | 54 | 50 | 23 | 63 | 13 | 69 | 27 | 15 |
| Birch, 0 | 52 | 32 | 31 | 91 | 11 | 82 | 15 | 18 |
| Birch, 1 | 9 | 8 | 5 | 12 | 2 | 13 | 5 | 2 |

- **At the coarsest the work is per axis.** The oak grows 45 k axes and 91 k phytomers, about two phytomers an axis, in 77 k apex-cycles. Of its 975 k allocation calls, 772 k fall in the Balance pass, about 17 per axis.
- **Allocation is a part of it.** With a bump allocator, whose free does nothing, the oak runs 885 ms against 1,090 at the finest and 152 ms against 168 at the coarsest. This is one native run, it does not remove the allocations' memory traffic, and it stopped at the spruce when its region ran out. Wasm's allocator was not measured and may cost more.

## R2: the same trees

An ordinary test pins the digest of every preset's tree at age 16, seed 7, at fidelity 0 and 1, in `cycle_cost_tests`. `digests-base.txt` holds oak, spruce and birch at preset age, seeds 1, 1234 and 5678, at both fidelities. Lever 1 left all of them equal, and reverting it restores the base code.

## R3: the gain

Lever 1 stopped retirement cloning a closed subtree's phytomers. Base against candidate, five alternating batches, held-out seeds, one pinned thread, five repeats less the first (a grow over 0.5 s not repeated). A build shared the machine during part of this run, so a single batch's ratio is noisy; the medians are what to read.

| Preset | Fidelity | Base s | Candidate s | Median ratio | Worst batch |
| --- | --- | --- | --- | --- | --- |
| Oak | 0 | 1.489 | 1.495 | 0.999 | 1.035 |
| Oak | 1 | 0.245 | 0.247 | 0.997 | 1.107 |
| Spruce | 0 | 4.763 | 4.686 | 0.984 | 0.996 |
| Spruce | 1 | 0.729 | 0.718 | 0.983 | 0.991 |
| Birch | 0 | 0.964 | 0.938 | 0.974 | 0.981 |
| Birch | 1 | 0.216 | 0.215 | 0.994 | 0.997 |

It took about 70 net lines and a new trait for 0 to 3%, so it was reverted (`AGENTS.md`, "Minimalist af"). The cycle that ships is the base's, so no setting is slower than base. Against 50 ms it stands at 245, 729 and 216 ms on these batches.

## Leads, not taken

- A leaf subtree's verdict needs no local copy, which would remove most of the 17 allocations per axis.
- A coarse tree's cost follows its axes. The dial's `bud` row may only raise a lateral's probability, so no search has tried fewer laterals with more leaf on each. That changes the tree, so it belongs to the next child.

## Friction

- **`dcg` refuses `git checkout --` and `git show ref:path > path` on my own timer patches.** About 2 minutes in all; saving a probe as a patch and `git apply -R` is safer anyway. Disposition: none needed.
- **No sampling profiler.** `samply` needs `perf_event_paranoid` at 1 or less and the machine is at 2, so each question took a timer patch and a rebuild, about 5 minutes each, four times. Disposition: a local setup matter for the owner, reported and not specced: `sudo sysctl kernel.perf_event_paranoid=1` would remove it.
- **The first review caught an overclaim.** My first draft of this file said byte-preserving work could not reach 50 ms, which the measurements do not support. Cost: one review round. Disposition: the review caught it as it should; no change.
