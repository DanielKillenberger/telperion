# fn-351: where the film's Laurelin seconds go (Fable, 14 Nov 2026)

Branch `fn-351-two-trees-time` from `fn-351-candidate` at 7685ee85b, worktree `.worktrees/slow2`, its own `target/`. Raw output is in ignored `raw/time/`.

## The answer

The film's segment 7 is no single build. It is one growth drawn at each of its 80 years, and each year settles and meshes the whole tree as it stands. At cut 8 (31e45b614) that segment ran on the device grower: `film.rs::grow` called `Generator::begin_growth`, `grow_to` and `settle`, and the tree stayed resident on the device. fn-353 deleted that grower on 11 Nov (710a8e2ab), and since then the film grows through `pipeline::growth` on the CPU and builds a CPU mesh each year. The 11.8 s and the 169.7 s measure two different growers.

No pass is superlinear. Over the 80 years the film draws 122.9 million node-years, and each costs 1.38 µs: 0.41 in `grow_to`, 0.43 in the settle, 0.54 in clothing and meshing. The per-node cost rises from 0.35 µs at year 40 to 0.47 at year 80 in the settle, and no further.

One part of it was work nobody read. `growth::begin` keeps the tree drawn a year before (`Paused::presenting`), so every `grow_to` of one year first settles the whole tree at the year before, for a draw within the year. The film draws whole years only and never reads it. That is change 1 below.

## Decomposition (candidate, before any change)

Probe: `crates/telperion-core/examples/film_grow.rs` (not committed; source kept in `raw/time/`), the film's loop without a GPU: `begin`, then per year `grow_to` and `mesh`. Laurelin, seed 7 (the film's `release.json`), years 1 to 80, four threads, 12:25 to 12:29, load 26 falling to 4. Its total is 169.6 s (`grow_to` 50.2 s, `mesh` 119.4 s) against the film's 169.7 s, and its resident peak 5,773 MiB against the film's 6,310 MB with a renderer.

| part | builds | seconds | share |
|---|--:|--:|--:|
| the growth itself, 80 cycles (one build's passes, `early three laurelin 80 7 cycle`) | 1 | about 16 | 9 % |
| the year before settled in `grow_to`, never read | 79 | about 34 | 20 % |
| the settle and its conversion to the pipeline's tree, in `mesh` | 80 | 52.6 | 31 % |
| clothing, rings, wood and leaves, in `mesh` | 80 | 66.8 | 39 % |

The settle row is measured alone (`film_grow … split`, a second settle a year). The first two rows split the measured 50.2 s by the single build's growth passes, 16.1 s of its 18.66 s; they are derived, not timed apart.

Per year, seconds (the `split` run):

| year | nodes drawn | `grow_to` | settle | rest of `mesh` | resident peak so far |
|--:|--:|--:|--:|--:|--:|
| 20 | 36,256 | 0.010 | 0.011 | 0.034 | 51 MiB |
| 40 | 707,735 | 0.236 | 0.248 | 0.342 | 888 MiB |
| 60 | 3,004,300 | 1.186 | 1.180 | 1.580 | 3,567 MiB |
| 80 | 4,673,612 | 2.108 | 2.191 | 2.588 | 5,773 MiB |

The film's own log agrees (`.worktrees/film/…/raw/film/cut/frames.tsv`, one build time per year): year 40 0.88 s, year 60 3.72 s, year 80 7.03 s, summing to 169.7 s. Cut 8's log (`.worktrees/f4/…/fn-272…/raw/cut8/frames.tsv`) has year 80 at 0.37 s on the device, while one direct CPU build of the same tree in its segment 8 took 13.7 s.

One build at 80 (seed 7, four threads, 12:47, load 9): 18.66 s, 2,543 MiB, 25,009,596 phytomers grown and 6,552,595 kept, so 0.75 µs a phytomer grown. Its passes, seconds: Balance 3.93, Units 3.40, Sketch 3.33, Wood 1.99, ShadeField 1.89, Allot 1.12, Assign 0.68, SagPlace 0.58, Scale 0.42, Place 0.27, Homes 0.21, Merge 0.20, Shed 0.18, Levers 0.16. No pass dominates, as fn-368's reading found on the oak and the beech.

## Segment 8, Laurelin to Telperion

90 direct builds (`mesh::build`), one a frame, read from the two films' `frames.tsv`. Cut 8: 13.3 to 15.9 s each, median 13.6 s, 864,843 to 864,909 leaves, 1,232 s. Candidate: 940,750 leaves at the Laurelin end rising to 998,679 at the Telperion end, 2,032 s. 49 builds took 16.1 to 20.7 s and 41 took 22 to 52 s. The slow ones come in runs of neighbouring frames: two at 44 and 47 s, a run of eight near 29 s at shares 0.55 to 0.66 with 18 s builds on either side, and the last 23 at a steady 24 to 25 s. At 18.5 s each the segment would be 1,665 s, 1.35 times cut 8's for a tree with about 1.5 times the phytomers. The other 370 s was not explained here (see Unknowns).

## Change 1: a growth drawn at whole years keeps no year before

`pipeline::growth::begin_whole(params, seed, horizon)`: `begin` without `Paused::presenting`. A year grown settles nothing, and a draw within the year is refused by name, as `Paused::settle_at` already refuses it for a grower that keeps no year before. The film's `bounds::begin` (both its bounds pass and its drawing pass) calls it. The grower is untouched.

Functions touched:

- `crates/telperion-core/src/pipeline/space/growth.rs`: `begin` (now a call), new `begin_whole`, new private `begun` (the old body, with the choice).
- `crates/telperion-core/src/pipeline/space/growth/tests.rs`: new `a_tree_begun_for_whole_years_is_the_same_tree_at_each`.
- `crates/telperion-render/examples/film/bounds.rs`: `begin`.

This adds a public entry to `pipeline::growth`, which is the host's to keep or drop: the commit stands alone and reverts cleanly.

### Proof and before/after: not run

The change is written and not compiled. Its build, its narrow tests and the timed runs were queued from 13:06; the queue admitted the batch at 13:37 at load 67 and the desk's watcher froze its scope during the build, with the load at 112 from another project's suites (FRICTION.md, 13:45). Nothing below this line is measured.

- Expected, from the decomposition: the film's segment 7 loses the 79 settles of the year before, about 34 of 169.7 s, and the resident memory of one settled tree. Unknown until run.
- A single build (`mesh::build`, the walks, the gate's digest pins) does not pass through `begin` and cannot change: the oak at 40, Laurelin at 80 and Telperion at 82 are the same before and after by construction, and were not timed three times as the brief asked.
- To run it: `TELPERION_HEAVY_RESERVE=8 telperion-heavy 4 --threads 4 -- bash .flow/evidence/fn-351-020-the-release-candidate/raw/time/batch-a.sh` (build, `pipeline::space::growth::tests`, the space crate's `paused` tests, `cargo check` of the film, `catalogue_identity`), then `telperion-heavy 8 … batch-b.sh` (the film's loop with and without the year before, twice; three single builds each of Laurelin 80, Telperion 82 and the oak at 40; the timer probe). Both write `raw/time/STATUS`.

## Not changed, for the host

- **The settle copies the grower first.** `Paused::settle` clones every held axis, draws list and successor list (`Grower::settled_copy`), and the settle then widens each cloned phytomer into the tree's (`Axis::settled`) and drops the clone. At year 80 the grower holds 1,473,979 axes and 7,501,674 phytomers of history against 4.67 million nodes drawn. A settle that read the grower by reference would save the copy in time and in peak memory; it changes `Grower::settle_shed` and `mark`, which the data-layout work is changing now, so it is not built here. Its seconds were not timed apart (the timer probe was queued and not admitted; see Unknowns).
- **The browser pays the year before too.** `WebRenderer` grows through `growth::begin`, so each `growTo` of one year settles the year before. It needs it for a draw within the year; a page that draws whole years only could use `begin_whole`.
- **Clothing and meshing, 39 %, is the tree's whole mesh built anew each year.** That is the rule (a draw is a snapshot of its own), not an accident.

## Unknowns

- Why 20 of segment 8's 90 builds took 22 to 52 s. The film ran 09:59 to 10:59 beside other heavy jobs, and the desk's watcher froze and thawed scopes all morning (`systemctl --user status` shows "Unit now frozen" on a queue scope); a frozen build would read as a slow one. Not reproduced.
- The split of the settle between the copy, its passes, the lineage order and the conversion. The timer patch is `raw/time/probe.patch`, built to `target/prof`.
- Cut 8's per-phytomer cost on the CPU: the 3.5 million phytomers at seed 42 is the brief's figure, not measured here.
