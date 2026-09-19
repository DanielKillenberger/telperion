# fn-76 local evidence, 2026-09-18

Taken on the desk (32 cores, Rust 1.98.1, cargo-nextest 0.9.145) while
another session ran the workspace suite in a second worktree: the load
average sat between 13 and 35 for every run below, so the absolute seconds
are inflated and only same-run comparisons are fair. The runner numbers for
R5 are the conductor's, from the Actions runs after this branch is pushed.

## Where a species specimen's time goes

A throwaway example timed each stage of one fixed specimen at seed 1
(`cargo run --profile ci --example`, then deleted):

| Preset | nodes | generate | surface | place + cull | placed matrices |
|---|---|---|---|---|---|
| Oregon white oak | 125,087 | 0.16 s | 0.69 s | 0.46 s | 715,065 |
| Silver birch | 68,501 | 0.28 s | 0.37 s | 4.21 s | 226,057 |
| Norway spruce | 95,389 | 0.08 s | 0.42 s | 2.82 s | 7,353,754 |
| European beech | 187,968 | 0.31 s | 0.94 s | 3.95 s | 4,928,780 |

The skeleton is a twentieth of a specimen; the spec's cost model (the
generator, and 21 files rebuilding the species suite's seeds) is recorded as
wrong in FRICTION.md. The other files build the shipped tables at seed 7, or
at the presets' own seeds (42, birch 1, beech 2), through `mesh::build`.

## R1: one generation, a committed digest

`crates/telperion-core/tests/species/digests.json` holds 49 digests (12 fixed
seeds per species, the spruce's retained failure seed 4250668600 besides),
FNV-1a over the skeleton's bincode, the wood buffers and the placed matrices.
The digest costs 0.5 to 0.6 s per species seed on the desk (spruce: tree
9 ms, wood 158 ms, 7.35 million matrices 442 ms). The desk generated the
table; the runner's first run is the cross-machine observation R1 was
designed to make.

Species binary alone, nextest `-j 4`, back to back:

| | load | spruce | beech | oak | birch | wall |
|---|---|---|---|---|---|---|
| base d0f8516b | 13 to 26 | 382 s | 359 s | 208 s | 157 s | 382 s |
| this branch | 16 to 23 | 209 s | 173 s | 105 s | 94 s | 209 s |

In the whole-suite runs below (both `-j 6`, loads of 16 and 23 to 10) the
four are 156/133/75/69 s before and 148/118/70/65 s after: the halving the
spec expected does not appear because the second generation was never half
the seed's cost; `species_metrics::measure` and the single placement stay.

## R2: the fixed-specimen cache

`crates/telperion-core/tests/specimens/mod.rs`; entries under
`target/tmp/specimens/<sources digest>/tree-<seed>-<wire digest>.bin`, 166 MB
after a whole core run, one directory per sources digest with the others
evicted on a miss. Fifteen test files read shipped skeletons through it.

A first cut cached the whole `TreeMesh`; it is recorded here because it is
the reason the cache holds skeletons only. Twelve entries were 3.4 GB (a
spruce or beech mesh is 590 to 630 MB of instance matrices), a miss was half
again slower than a plain build (bincode walks ten million `f32` one by one),
and under `--partition count` the five tests that share a seed-7 mesh land on
different shards, so every shard would pay the miss and see no hit. The
skeleton entries cost milliseconds and the cache never makes a shard slower.

## R4: the count

`cargo test --profile ci -p <crate> -- --list`, names sorted and diffed:

| Crate | before | after |
|---|---|---|
| telperion-core | 343 | 348 |
| telperion-render | 129 | 129 |
| telperion-wasm | 2 | 2 |
| telperion-jev | 125 | 125 |

The diff is exactly five additions, none lost: the R1 message test in
`species.rs` and the four R2 error-case tests in `specimen_cache.rs`. No
crate has doc-tests, so nextest runs everything `cargo test` ran. 11 tests
stay ignored, as before.

## Local runs after the change

- core, nextest `-j 6`: 337 passed, 11 skipped, 148 s wall (before, same
  flags: 332 passed, 181.7 s wall); summed per-test time 813.6 s before,
  752.7 s after.
- render + wasm + jev, nextest `-j 6`: 256 passed, 135.9 s wall.
- workspace `cargo nextest list --partition count:k/4`, core load from the
  after run's per-test times: partition 1 sums 137 s (longest 65 s, the
  birch), 2 sums 254 s (the oak 70 s, the three stem tests at 35 s each,
  attachments 29 s), 3 sums 170 s (the spruce alone 148 s), 4 sums 152 s
  (the beech 118 s). The bound of a shard on the runner is the spruce test,
  one seed after another on one core; the sum of everything else divides by
  four cores.

## After the first runner run (RUNS.md): named shards, seeds on threads

The runner's per-test times (run 35385460955, `runner-durations.json` from
its log) predict the core shards. A species test's core-seconds stay what
they were; its chain becomes ceil(seeds / 4) rounds of the mean seed. Test
phase per shard is the larger of core-seconds / 4 cores and the longest
chain:

| Split | shard loads, core-s / 4 | longest chain | note |
|---|---|---|---|
| count 3 | 118, 84, 46 s | 56 s | the spruce and the beech share shard 1 |
| count 4 | 54, 89, 54, 51 s | 56 s | chosen; the stem trio, the oak, attachments and mesh share shard 2 |
| count 5 | 68, 96, 9, 42, 33 s | 56 s | shard 3 nearly empty |
| hash 4 | 55, 17, 89, 86 s | 56 s | worse than count at the same N |

So the core crate runs as `core 1/4` to `core 4/4`, render as one job (8 s
of tests), wasm and jev as one job (27 s); with a warm build cache the
predicted rust wall is the setup and build (about 70 s warm, 135 s cold on
this run) plus the 89 s shard.

`fixed_species` runs its seeds four at a time on scoped threads
(`SEEDS_IN_FLIGHT`), every seed and every assertion as before; each seed's
checks run under `catch_unwind` and the outcomes are read back in seed
order, so the failure reported is the first seed that failed however the
threads finished. Species binary on the desk, nextest `-j 1`, load 9:

| | spruce | beech | oak | birch |
|---|---|---|---|---|
| seeds one after another (branch, load 16 to 23) | 209 s | 173 s | 105 s | 94 s |
| four seeds in flight | 41 s | 28 s | 21 s | 16 s |

The test count is unchanged by the threads (604). Whole core suite after both follow-ups, nextest `-j 3`, load 6 to 9: 337 passed, 11 skipped, 158.4 s wall; the spruce test is 43.5 s and the tail is now the sweep walk at 34 s.
