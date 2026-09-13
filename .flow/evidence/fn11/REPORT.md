# FN11: growth over time, as the machine measured it

A tree is now a specimen with an age. The same family, seed and age give the
same tree on native and on wasm, byte for byte; a running page advances it by
any fraction of a year and it grows in place, branches keeping their identity,
wood thickening, leaves appearing on new shoots and staying on mature twigs;
scrubbing back within what has been simulated is a read, not a rebuild. This
report records what was built, what it costs, and what it does not yet do.
The numbers below were measured by the implementer on this machine and are
carried from the commit record of the branch; the host re-ran every gate.

Calibration against open-grown curves, routing production through growth,
the re-pin, the age strips and the mature stills moved to fn-30 on
2026-09-13, so this report carries no curve and no verdict slot. What this
spec judged is the machine.

## What was built

**The growth model.** Steps 1 to 3 of the spec's order: a `Specimen` beside
`Tree` that retains the scaffold, attractor and local frontiers; generational
identities through a dense slot map; a yearly slice with an integer remainder;
a Chapman-Richards unit budget that saturates so age past maturity is free;
thickening by the pipe solve; shoot state with birth year, bud fate and a
crown-depth vigour proxy; shedding by vigour under a threshold and tolerance
period, decided from a slice-start snapshot with a cap on sheds per slice;
every transcendental through pinned pure-Rust `libm`, which fixed a
native-to-wasm mismatch that predated this spec.

**The chronicle.** Rewritten on 2026-09-13 at the owner's request as one
design and reviewed to SHIP: the specimen is an append-only lifetime record
where every node, run, placement and radius keyframe carries a birth year and,
once gone, a death year; the simulation is the only writer; every reader, the
tree at an age, the change record between two ages, the snapshot, a backward
step, is a filter over stamps. Shedding stamps instead of compacting. Radii
are keyframes under the family's resize tolerance, canonical per simulated
year whatever the advance's length. Leaf cohorts are a function of shoot age,
not events, so a mature tree keeps its foliage and nothing is stamped after
saturation (the ratified foliage amendment, R12). A history cap bounds memory.

**Bindings and the page.** The wasm specimen handle in the shape of the
field handle; the native API mirrored; the snapshot as the chronicle in the
field's schema-1 portable shape, never the mesh; the harness age dial that
reads within the frontier and advances beyond it, the rebuild, and the play
control at a years-per-second rate.

## The slice length

Monthly slices were the design; the number chose the year. Measured
2026-09-13 before the chronicle, native, three-build medians: the mature oak
built in 5.906 s at monthly slices and 0.775 s at yearly ones against a
half-second target. The slice is a year; the clock keeps its integer
remainder at the yearly denominator; every determinism property holds.

## What it costs

Native, three-build medians, the core's own timers, no GPU, on the named
machine while other work shared it (each row names its commit).

| | oak | spruce | commit |
|---|---:|---:|---|
| envelope build, today's one-shot | 67.8 ms | 37.1 ms | 2fd2668 |
| mature build through growth, 173 yearly slices | 1,273.5 ms | 932.9 ms | 972bace, 2fd2668 |
| sparse yearly advance | 13.4 ms | 20.6 ms | 2fd2668 |
| of which the change record | 5.5 ms | 12.7 ms | 2fd2668 |
| sparse record plus packed read | 6.6 ms | 13.3 ms | 2fd2668 |
| a year with few changes, advance | 6.6 ms | 4.9 ms | 2fd2668 |
| read at an earlier age, mature tree | 262.3 ms | 2,314.7 ms | 2fd2668 |
| snapshot round trip, mature tree | 320.7 ms at 121.9 MB | | 972bace |

**Does a slice cost what it changed?** Yes, after the cost step. The sparse
oak record plus packed read replaced an 88.0 ms whole-output read-and-diff
with 6.6 ms; a year in which a handful of nodes change costs 6.6 ms on the
oak where the same year had cost 490.6 ms before events were indexed by year.
The spruce's sparse advance fell from 1,500.3 ms to 20.6 ms once foliage
stopped moving with the crown envelope: its transform now reads the wood
only, as the spec says, and the spurious moved placements fell from 2,012,758
to 777.

**What stays linear, and is stated as such.** A read at an earlier age is a
filter over the whole chronicle, about a quarter second on the mature oak and
over two seconds on the spruce, whose crown carries over a million placements.
The mature build through growth is about seventeen times the envelope build
on the oak; the spec's target was about half a second and the closed-form
alternative stays the owner's reserve. The snapshot is 122 MB on the mature
oak because it carries the chronicle, 1.12 million radius frames among it.

## Convergence, unresolved

The mature tree grown through time is not today's tree. Against the envelope
build at seed 7: oak nodes +29.3 percent and crossover +50.4 percent, spruce
nodes −15.3 percent and crossover +22.0 percent, bounds within 2.3 and 6.0
percent. Right size and shape, different amount of wood, because both presets
author a zero shedding threshold and no calibration has run. Production
therefore still routes through the envelope build and no pin has moved for
convergence; that is fn-30's R2.

## Deviations

- Monthly slices became yearly by the spec's own rule; recorded above.
- The foliage rule was amended with the owner's ratification on 2026-09-13
  from leaves-for-a-lifetime to living-shoot-and-cohort, after the earlier
  rule left 13 placements on the mature oak.
- The resize tolerance was raised from a nanometre to 0.1 mm, chosen by the
  number against byte-identical reads; the previous value wrote a radius frame
  per node per year.
- The implementer's own gate runs were blocked twice by a sandbox the host
  imposed; the host re-ran all five gates in the normal environment for every
  commit.

## Evidence

- Commits: `git log master..fn-11-growth-over-time`, 35 commits, each with its
  numbers and its commands.
- Gates at HEAD: `cargo fmt --all -- --check`; `cargo clippy --workspace
  --all-targets -- -D warnings`; `cargo test --release --workspace`;
  `npm run wasm:build && npm test`; `npm run typecheck`, all green, re-run by
  the host after every return.
- Parity: `harness/parity.test.ts`, every preset byte-identical native and
  wasm.
- The status document written after step 3, `STATUS.md` beside this file, is
  kept as the record of that handover.
