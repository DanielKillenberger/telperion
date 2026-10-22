# fn-304 result

Each red run applied one temporary patch to the generator with `git apply`, ran the named tests under nextest inside `telperion-heavy 8 --threads 4`, and reversed the patch; none was committed. The patches and the runner are kept in ignored `raw/` (`raw/patches/specs.py` writes each patch; `raw/proofs.sh` applies, runs and reverses; `raw/proofs/summary.tsv` lists every run).

## Task .1: the core lib (rulings 7, 8, 9)

| Test | Change | Own time before → after (nextest) |
|---|---|--:|
| `repeat_tests::the_largest_tree_and_the_palm_grow_the_same_tree_again_and_on_any_cores` | the gate keeps Telperion and the date palm; the print precondition stays | 182.2 s (all presets, rayon 2) → 79–92 s (rayon 4) |
| `repeat_tests::every_preset_grows_the_same_tree_again_and_on_any_cores` | the full catalogue, now `#[ignore = "fn-304: ..."]` | 180–201 s (rayon 4), outside the gate |
| `suite::packed_leaf::every_station_stands_inside_its_box_at_a_quarter_millimetre_step` | the two `packed_leaf` tests merged: one mesh a preset, both assertions | 49.8 + 49.4 s → 51–56 s |
| `family::tests::every_row_past_its_bound_is_refused_on_every_family` | the eight families at once (`thread::scope`, `resume_unwind`) | 70.1 s → 11.8–13.2 s |

Test count in the core lib: the two `packed_leaf` tests become one (merge, ruling 8); the repeat test becomes a gate test plus an ignored sweep (move to ignored, ruling 7). Run: 459 → 458; ignored: 37 → 38; total 496 unchanged (gate: 458 passed, 38 ignored in the core lib).

### Red and green

| Patch (temporary) | What it breaks | Gate subset | Full sweep |
|---|---|---|---|
| `tail-piece` (`telperion-space/src/cores.rs`, `fill`) | with more than one pool thread the last piece of a parallel fill is never written, so the tree follows the core count | red: `telperion: on one core` | red: `ordinary: on one core` |
| `sum-order` (`telperion-space/src/light.rs`) | the light's sum over directions reversed on more than one thread | green | green |
| `offset` (`telperion-space/src/cores.rs`, `map`) | a parallel piece loses its offset | red (panic in `girth.rs:263`) | red (the same panic) |
| `box-small` (`foliage/reference.rs`, `Reference::grown`) | the leaf box half its size | merged test red: `ordinary: leaf 0 stands on the wall ...` | — |
| `box-coarse` (the same) | the leaf box twenty times too coarse | merged test red: `ordinary: half a step is 0.00192 m ...` | — |
| `bloom-rails` (`family.rs`, both bloom checks) | the bloom rows lose their rails | row-bound test red on every family | — |

Green on the engine: all four tests pass (`raw/proofs/t1-green.log`).

`sum-order` is the one nondeterminism that neither the subset nor the full sweep sees: the shipped presets' light sums over too few directions for their order to move a bit. It is recorded, not acted on; `tail-piece` is the ruling's proof (deterministic, the defect "on any cores" names, caught by the "on one core" assertion on both).

Gate at the end of task .1 (`telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, 3b880374b..b1feb485a): 1,466 passed, 2 failed, 116 ignored, 2,374 s wall including 215 s of compile, peak 7,637 MiB anon. The two failures are fn-302's (`film_walk_tests::the_film_walks_build_on_the_device`, `the_rounds_frames_have_the_cpus_counts`), red on F4 before this spec.

## Task .2: the core sweeps (rulings 1, 2, 5)

| Test | Change | Own time (nextest, rayon 4) |
|---|---|--:|
| `walk_leaves` `oak_to_palm_keeps_its_leaves_by_degree`, `spruce_to_birch_...`, `birch_to_oak_...` | each walks seed 1 | 20 s, 81 s, 29 s (280 + 93 + 65 s at three seeds, rayon 2) |
| `walk_leaves` `every_walk_changes_its_leaves_by_degree_at_every_seed` | the three walks at seeds 1, 7 and 42, ignored | 359 s |
| `ground_reach` `nothing_drawn_passes_below_the_ground` | fn-301 R1's 45 touching rows and every preset at its own age, seed 1 (50 rows: three own-age rows are already touching) | 54–58 s, peak 4.68 GB (was 159 s, 7.77 GB) |
| `ground_reach` `nothing_drawn_passes_below_the_ground_on_every_row` | all 168 rows, ignored | 136 s |
| `walk_sizes` `the_teaser_frame_fits` | the teaser frame alone | 21 s (was 79.5 s with the exceptions) |
| `walk_sizes` `the_named_exceptions_grow_no_further_out` | fn-273's eight exceptions, ignored | 46 s |

Test counts: `walk_leaves` 3 run + 1 ignored → 3 + 2; `ground_reach` 1 → 1 + 1; `walk_sizes` 1 → 1 + 1 (the test is split: the teaser stays, the exceptions move to ignored, renamed `the_teaser_frame_fits` and `the_named_exceptions_grow_no_further_out`).

### ground_reach's 7.8 GB is one tree, not accumulation

Measured with a temporary probe that prints `VmRSS` and `VmHWM` after each row's tree is dropped (`raw/patches/probe-rss.patch`, `raw/proofs/ground-probe2.log`, `ground-probe3.log`):

| Row | Alone, fresh process: peak | In the sweep: peak so far, RSS after the drop |
|---|--:|--:|
| Telperion at 82, seed 1 | 4,623 MiB | 4,658 MiB, 828 MiB |
| Telperion at 82, seed 7 | 7,677 MiB | 7,666 MiB, 1,253 MiB |
| Laurelin at 80, seed 1 | 4,326 MiB | under the Telperion peak |
| Norway spruce at 40, seed 1 | 2,697 MiB | 2,689 MiB |
| Telperion at 82, seed 42 | not run alone | 7,692 MiB, 1,195 MiB |
| the other rows run alone (birch, oak, beech and ordinary tree at 40, palm at 40, Telperion at 20) | 20 to 713 MiB | |

Telperion at 82, seed 7 peaks at 7.68 GB on its own, and the sweep's high-water mark rises only at the Telperion rows (seed 42 to 7.69 GB): the sweep's peak is one tree, not trees held together. The test already drops each tree before the next is built; the RSS left after a drop (up to 1.3 GB) is memory the allocator keeps, not a tree held. The gate's rows include Telperion at its own age only at seed 1, so the gate test peaks at 4.68 GB; seed 7 stays in the ignored sweep.

### Red and green

| Patch (temporary) | What it breaks | Gate subset | Full sweep |
|---|---|---|---|
| `cluster-round` (`telperion-space/src/lineage.rs`, `cluster`) | a short shoot's leaves rounded, not drawn (fn-277's defect) | red: `spruce_to_birch` and `birch_to_oak` at seed 1 | red |
| `leaflets-round` (`foliage/organs.rs`, `placement`) | a placement's leaflets rounded | red: `oak_to_palm` at seed 1 (`leaflets per station ... against ...`) | red |
| `fronds-round` (`foliage/organs.rs`, `apex`) | the palm's fronds rounded | green | green |
| `no-rest` (`foliage/rest.rs`, `leaf`) | a leaf that would cross the ground is never rested (fn-301) | red: the touching rows named | red |
| `budget-eighth-cycle` (`telperion-space/src/grow.rs`) | the cycle's phytomer budget an eighth of the asked | red: `the teaser's frame: the tree outgrew its budget` | red (the exceptions) |

`fronds-round` is caught by neither: the frond count on the oak-to-palm walk stays below the oracle's 30 organs, where a form is dormant. Recorded, not acted on.

Gate at the end of task .2 (same invocation, ffce4e524): 1,466 passed, 2 failed (fn-302's two device tests), 119 ignored, 1,995 s wall including about 217 s of compile, peak 6,867 MiB anon.
