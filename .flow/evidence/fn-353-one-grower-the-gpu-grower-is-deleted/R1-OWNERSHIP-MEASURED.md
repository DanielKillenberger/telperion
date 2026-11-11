# Option A built, and measured (11 Nov 2026)

The change follows answer 5 of `R1-ANSWERS.md`. `Grower`, `Paused` and `Settling` hold an `Arc<Species>`, and `grow`, `grow_cycle` and `grow_census` clone their borrowed species into one once per tree. `Rules` drops its species, and `Shoot` borrows it from the grower (`Shoot::new((rules, species), ...)`; `node_wood` takes the pair). In `telperion-core`, `growth::Params` holds `Arc<Species>`, and `GrowthState` owns an `Arc<Params>` and no lifetime (`begin(&Arc<Params>, ...)`).

Ownership was the only thing that changed: no pass order, no test expectation and no baseline. The copy-out sites (`let species = self.species;`) all compiled as plain borrows of the field (`&*self.species`), so none needed an `Arc` clone. Public signatures outside `telperion-space` changed only at the callers listed: `growth::begin`, and `Paused::begin`'s callers in `examples/premises.rs`, `examples/footprint.rs`, `examples/unfold.rs`, `suite/establish.rs` and the device's `present_tests.rs`.

## Behaviour (measured)

`telperion-heavy 18 --threads 8 -- cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core --no-fail-fast --test-threads 4`: **774 tests run, 774 passed, 74 skipped**. That includes every digest and repeat test in both crates, unchanged. The still runner's oak at 40, seed 1001, is still 272,095 nodes and 47,279 leaves, as in `BEFORE.md`.

## Cost: the oak at 40, seed 1001, one thread (measured)

Command: `telperion-heavy 12 --gpu 2 --threads 1|8 -- taskset -c 3 target/ci/examples/space_oak <dir> 40 --seeds 1001 --shots none`, three runs each. The number is the runner's "grown in", which covers `executor::grow` and `expansion()`.

| | Run 1 | Run 2 | Run 3 | Median |
|---|--:|--:|--:|--:|
| Before (`8a3a20b74`'s code) | 1,347.8 ms | 1,402.3 ms | 1,235.3 ms | **1,347.8 ms** |
| After (option A) | 1,367.4 ms | 1,260.6 ms | 1,292.5 ms | **1,292.5 ms** |

The median moved **-4.1 %**, faster, which is past the host's 2 % line, so task 1 stopped here as answer 5 says to. The spread within one side is 13 % before and 8 % after, so three runs cannot resolve a 2 % move in either direction. No clone cost shows: one `Species` clone a tree is inside the noise. Deciding whether that holds the claim is the host's call.
