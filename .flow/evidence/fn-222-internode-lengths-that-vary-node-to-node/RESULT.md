# fn-222 task 1: internode lengths that vary node to node (worker, 2026-10-07)

## What is built

- **`Form::internode_spread`** (per PA, linear scale, neutral 0, at most 1; `species.rs`, `validate.rs`, `scale.rs`). Each internode's length is `internode × scale × (1 + spread (2u − 1))`, where u is the node's own draw: `Key(phytomer.key).child(INTERNODE)`, `INTERNODE = u64::MAX − 9` (`lineage.rs`). The factor is uniform on [1 − s, 1 + s], with mean one. At 0 the function returns exactly 1 without drawing.
- **Lay only.** The factor enters `Layer::step`'s length (`geometry/lay.rs`) and nothing else. The phytomer's `scale` is untouched, and so are the walk's `run` and reach (the pull's fade, in scaled lengths) and D18's relay node, which is decided on presences. The topology, the scales and every decision are the same to the bit at any spread.
- **The bounds.** The closed form's reach bound (`kept/bound.rs`) and the landing zone's longest internode (`fine/landing.rs`) take each internode at its longest, `internode × (1 + spread)`. Host decision 3's routing therefore stays conservative. At 0 the bound is the same value.
- **GPU.** The PA table carries the spread in slot 22 (`laid.rs`, `geometry.wgsl`). `lay.wgsl` scales `len` by the same factor, drawn by `internode_draw` (`contract.wgsl`), which mirrors `fixed::internode_draw`.

## Requirements

| R | Evidence |
|---|---|
| R1 | `tests/internodes.rs`: beech and oak at 20 with spread 0.6 keep the same counts, axes, keys and scales to the bit. Their laid lengths over `internode × scale` have mean 0.997 for both species (9,386 and 36,723 internodes), sd 0.343 and 0.345 (uniform: 0.346), within [0.4, 1.6]. An explicit 0 gives the same tree (`assert_eq` on the structure), and 1.01 is refused. Walk: `states[k].form.internode_spread`, 0 to 1 for each of the three PAs, was added to `walk::settings`, and `every_setting_changes_the_tree_by_degree` passes within the bound of 30. |
| R2 | Probe selector 16 (`internode_draw`): 262,144 keys, GPU bits equal to CPU bits. `varied_internodes_settle_on_the_gpu_as_on_the_cpu`: beech 25 with spread 0.6, seeds 1 and 7, cuts 1,024 and 4,096, three orders and batches. Tips are at most 1.2e-6 and 1.6e-6 m (RMS 2.1e-7 and 3.4e-7) against the CPU executor, inside fn-125's screen (50 µm, 10 µm RMS). With the GPU's spread disabled, the test fails (checked, then restored). |
| R3 | `predictor::tests::an_internode_spread_moves_no_count_and_no_expected_length`: beech, spruce, oak and palm at 30 with spread 0.8. Every predictor count, every expected log length and every row count are the same to the bit. Every reach bound is at least as large. |
| R4 | `space_hash` (four species, ages 40 and 80, seeds 1 and 7) at the base `e8a9b05d` and after: all 16 clothed hashes are identical (`raw/hash-before.txt`, `raw/hash-after.txt`). Gate: `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` passes, apart from two `telperion-jev` replay tests that need render's `headless` example; they pass 3 of 3 once part two has built it (FRICTION.md). `cargo test --profile ci -p telperion-render --no-fail-fast` under the GPU lock exits 0. `npm test`: 13 files and 143 tests pass, `harness/parity.test.ts` among them. |

## Re-pinned

`crates/telperion-core/tests/catalogue_identity.rs`: the family, walk and override digests move only because the new field shows in `Debug`. With `, internode_spread: 0.0` left out of the text, each digest is still fn-198's (checked). The refusal digest did not move.

## Not modelled, for the host

- **The closed form's second moments.** `Along::square` and a row's `spread` and `along` (fn-211.4's stand-ins) still assume every internode is the same length. With a spread s, a unit of n internodes of length l has its squared length raised by n·(s²/3)·l² in expectation. The means are exact. Nothing in the spec asks for this.
- **The relay point within its span** is still placed by the scale-weighted share (`relay_point`), not by laid length. It moves by degree, and the node is unchanged.
- **The distribution** is uniform. It was chosen because it is bounded, which keeps the reach bound finite. A log-normal or a correlated draw would be a design choice for the host.
