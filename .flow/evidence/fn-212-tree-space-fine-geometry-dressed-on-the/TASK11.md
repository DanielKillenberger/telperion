# fn-212 task 11: Astra's nine implementation-review findings (worker, 2026-10-08)

The review is `ASTRA-REVIEW-IMPL.md`, beside this file. Each finding got a test first, run on its own against the code before the fix, and then the fix. Raw output is in `raw/task11/`.

## Per finding

| # | Finding | Reproduced | Fix commit | Test |
|--:|---|---|---|---|
| 1 | Deferred destroys drained by an unrelated submit | Red: "Buffer with 'let go' label has been destroyed" at the work's submit | `34e1722a` | `generation::release_tests::a_buffer_let_go_waits_for_its_own_encoder_not_another_submit` |
| 2 | `Oversize` unwrapped into a panic | Red: panic at `chains.rs:282` ("chain axes", 550,502,400 bytes against 134,217,728) | `528c06f1` | `route_tests::a_buffer_over_the_devices_limit_is_refused_by_name` |
| 3 | Tied widest nodes publish mixed coordinates | **Not reproduced** (below); fixed as the host directed | `302d81a2` | `dress_tests::one_widest_node_supplies_all_its_coordinates` |
| 4 | Fine foliage bypasses `max_instances` | Red: the device path built a tree the CPU reference refused | `f8143b1c` | `route_tests::the_foliage_budget_holds_the_whole_trees_leaves` |
| 5 | Octahedral `sign(0)` reverses frames | Red: frame (0,0,-1) packed as (0,0) | `7169eea1` | `handoff::tests::the_device_packs_a_frame_as_the_cpu_does` |
| 6 | Divergence narrowed to f32 unreduced | Red: turns 1.94 apart (sine/cosine) at 1e9 degrees | `4abd0ac2` | `stations_tests::a_large_divergence_turns_the_stations_as_the_pipeline_does` |
| 7 | Zero leaf size still places leaves | Red: 27,794 device leaves against the CPU's 0 | `88687a81` | `route_tests::a_zero_leaf_size_places_no_leaves` |
| 8 | Lobed surfaces and `surface_contact=0` refused | Red for lobed wood with contact 0 ("ring positions"); round wood with contact 0 already built. Red for seated lobed wood (not qualified) | `ad01db84`, `f488fdde` | `route_tests::the_device_path_draws_what_the_cpu_path_draws`, `lobed_tests::lobed_wood_is_laid_on_the_device_as_on_the_cpu` |
| 9 | Test-only `run_words` live in production | Red: the stations held 808,608 bytes more than their segments (beech at 30) | `6f4e46a4` | `stations_tests::the_stations_keep_only_their_segments` |

## What changed

1. **Destroys tied to encoders.**
   - `Gpu::encoder` opens an `Encoder` in the device's ledger. `Gpu::submit` takes its `Recorded` commands and closes it after the queue submit.
   - A buffer let go while encoders are open is destroyed once each of them has closed, by submit or by drop. A buffer let go with none open is destroyed at once.
   - A raw `wgpu::CommandEncoder` can no longer reach `Gpu::submit`: it does not compile.
   - Every encoder in the crate, its tests and its examples is made through `Gpu::encoder`.
2. **Every fine buffer is refused by name.**
   - Every `io::buffer` on the fine path returns its error: the grower's regions, tables, trim and pack, the steps, the pool, the laid waves, the fades, the dress and folds, the sweep, stations, rings and handoff.
   - The device executor keeps its failure as a `RenderError` (`Device::kept`), so `resident::build` reports `Oversize` itself, not the engine's `Executor` words.
   - "A chain no region holds" was an `assert!`. It is now a `ResourceLimit`.
   - `Gpu::request_within` lets the test hold a device to WebGPU's default limits.
   - What remains in production `fine/` is invariants, not device calls: the tables' `expect`, the table builder thread's `join`, and a node's parent in `stubs.rs`.
3. **One widest node.** `widest` takes the least id among the tied nodes (`atomicMin`). A new `publish` pass has that one node store all three coordinates.
4. **The foliage budget.** The CPU block's stations and the device's are checked together against `canopy.max_instances` before rings and placement. The refusal uses the CPU reference's words.
5. **signum.** `handoff.wgsl` folds the lower hemisphere with a sign-bit `signum`, as `f64::signum` does. The decode (`curve_walk.wgsl`) only sees values decoded from integers, which carry no negative zero, so it was left alone.
6. **Divergence.** `station_rows` reduces the divergence to a turn in f64 before narrowing, as `data::config` already does for the CPU block.
7. **Zero leaf size.** `Rows::leaves` is false for a size of 0, and the stations then place none.
8. **Ring positions only for seated leaves.**
   - When `x.seats()` is false, neither block lays ring positions, as `preparation.rs` already does. Both placements bind an empty buffer instead.
   - The beech (contact 0) no longer lays positions that no pass reads.
9. **`run_words`.** `Stationed` no longer carries them. `Stationer::stations` lets them go, and the probe reads them through `stations_and_runs`.

## Finding 3: the attempt

- **Setup.** A synthetic drawn wave of 65,536 nodes, then 4,194,304, all tied at reach 2: (r,y,0), (0,y,r), (-r,y,0) and (0,y,-r) in turn, with y varying. 8 and 16 runs, through `Dressed::extent` on the RTX 3080 (Vulkan).
- **Result.** Every run returned reach exactly 2, never √8.
- **Why.** On this driver a tied invocation's three stores land together.
- **The race is still real.** WGSL orders no stores between invocations.
- **What the test holds now.** After the fix, the reach must be exactly r, and the least id (node 0, height 0) must supply the coordinates.

## Finding 8, seated leaves on lobed wood: the host's option (a)

**The host's decision** (2026-10-08, in CONTRACT.md): lobed ring vertices on the device. A named gap was rejected, because lobe depth crossing 0 would switch the tree between ways of building.

**What changed.**
- **Positions pass.** `positions.wgsl` lays each ring's outline as `radius × (1 + depth × cos(lobes × (angle + phase)))`, with the phase taken from the ring's twist word. This is `angular.rs`'s `profile`. The lobes and the depth ride in the pass's config (`positions.rs`, `ring_vertices`).
- **Admission.** `CompactSurface::qualified` no longer excludes lobed wood.
- **`rings.wgsl`** needed no change: it already writes the twist phase into each ring.
- **The whole path** now lays seated lobed wood through the compact positions too, rather than through the prepared CPU wood.

**Measured** on the spruce, two lobes a tenth deep, twist 0.5 (`raw/task11/lobed-parity.txt`).

Vertex parity: the device's ring vertices against the CPU's own build of the lobed wood (`Expansion::wood`), vertex by vertex. The screen is the compact path's own, at most 50 µm and 10 µm root mean square.

| Age | Vertices | Most apart | RMS |
|--:|--:|--:|--:|
| 12 (gate test, twist 0 and 0.5) | 449,806 | 0.24 µm | 0.04 µm |
| 40 | 17,020,190 | 0.96 µm | 0.20 µm |
| 80 | 67,890,476 | 1.93 µm | 0.48 µm |

Leaf parity, the real path against the CPU path (`real_path_at_80` with `PROBE=norway-spruce:<age>:1:lobed`):

| Tree | Leaves (device, CPU) | Compared | Left out | Over | Flags |
|---|--:|--:|--:|--:|--:|
| Spruce 40 s1, lobed | 4,598,685 / 4,598,685 | 4,598,685 | 0 / 0 / 0 | 0 | 0 |
| Spruce 80 s1, lobed | 16,870,702 / 16,870,700 | 16,870,370 | 0 / 332 / 0 | 0 | 0 |

The 332 leaves at 80 are the half-micrometre class already named for the round spruce.

**Close-up stills.** `seam_stills` was run with `PROBE=norway-spruce:80:1:lobed`, at 1024 × 768, at the seams of two device-drawn fine roots (`raw/task11/lobed-stills/`).
- `seam0-whole` and `seam0-bare` were viewed, device and CPU.
- The needles sit on the lobed wood on both sides, and the two sides are indistinguishable by eye.

| Still | Exact pixels apart | Under 6 % fuzz |
|---|--:|--:|
| seam0-whole | 0.74 % | 0.12 % |
| seam1-whole | 1.03 % | 0.19 % |
| seam0-bare | 0.05 % | 0.02 % |
| seam1-bare | 0.06 % | 0.02 % |

**Found on the way.** `generation/position_probe.rs` and `position_tests.rs` have compiled into nothing since F3 (#156, `cc0da82a`). Their module was declared in `wood_tests.rs`, which F3 deleted. They hold the compact path's old vertex screen. The new `lobed_tests.rs` checks lobed wood against the CPU's build without them. For the host: delete them, or bring them back into the suite.

## Parity probes at 80 (`raw/task11/parity-80.txt`)

| Tree | Leaves (device, CPU) | Compared | Left out | Over | Flags |
|---|--:|--:|--:|--:|--:|
| Beech 80 s1 | 2,474,266 / 2,474,266 | 2,474,266 | 0 / 0 / 0 | 0 | 0 |
| Beech 80 s7 | 4,294,872 / 4,294,872 | 4,294,872 | 0 / 0 / 0 | 0 | 0 |
| Spruce 80 s1 | 16,870,702 / 16,870,700 | 16,870,370 | 0 / 332 / 0 | 0 | 0 |

- All three are unchanged from task 10 part 4 (`raw/task10/parity-80-slender.txt`).
- The beech is unchanged although it no longer lays ring positions: its leaves seat on nothing.
- The spruce's curve-demand residual (4.9251e-5) lies inside the spread of earlier runs (4.9251e-5 to 4.9340e-5).

## Gate (after the lobed vertices)

- **Render half,** under the GPU lock, first: 176 passed, 0 failed (`gate-render-2.txt`).
- **CPU half:** 988 passed, 0 failed (`gate-cpu-2.txt`).
- **`npm test`:** green. The catalogue check passes, and vitest passed 143 of 143 (`npm-test-2.txt`).
- **Sizes and threads.** `dist/telperion-render.wasm` is 2,754,766 bytes (2,728,728 after task 10), under the 2,800,000 ceiling. `RAYON_NUM_THREADS`, `RUST_TEST_THREADS` and `CARGO_BUILD_JOBS` were 8.
