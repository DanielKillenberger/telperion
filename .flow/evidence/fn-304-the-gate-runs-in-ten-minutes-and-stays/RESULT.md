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
