# fn-212 task 10: the device path in production, then in the browser (worker, 2026-10-08)

## Part 1: native wiring (host decision 2; R2's gate)

### What was built

- **The fine module is production code.** `generation.rs` no longer gates `mod fine` behind `cfg(test)`. Natively, the dress, sweep, stations, rings, handoff, settle kernels and `resident::build` are compiled into the library. The tests and probes stay `cfg(test)`. Two modules were split:
  - `units.rs` keeps the table packer; the unit-kernel tests moved to `units_tests.rs`.
  - `stubs.rs` takes the seam stubs out of the probe's fixture.
- **Routing.** `Generator::prepare_async` with resident delivery takes `prepare_fine` (`fine.rs`) wherever `executor::apart(family)` is `Ok`. `prepare_fine` is `resident::build` inside the device's error scopes, keeping only the prepared tree. Every other request takes `prepare_whole`, the body that ran before.
- **The gap, named** (`executor::apart`, `telperion_space::apart`). This is a capability selection made before growth, never a fallback from a failure:

  | Family | Gap |
  |---|---|
  | No engine species (birch, Laurelin, Telperion, ordinary) | "no engine species" (refused by `grow`, as before) |
  | Oregon white oak | "its buds read light, which fn-211 carries to fine roots (F2b)" |
  | A species with retained pipes or leaf girth | "its shed pipes and leaf girth read the tree as grown (fn-211)" |
  | Date palm | "its leaves stand on no regular twig stations". The device path refused it with "the fine path needs regular stations"; the whole path draws its fronds with the CPU foliage fallback, as it did before. |

  `grow_marked` reads the same `telperion_space::apart`, so the CPU executor's partition and the device path select by one rule.
- **The kernels.** Each generator makes the kernels once, on its first tree that comes apart (`OnceCell`). Their pool keeps one build's buffers for the next build (fn-209 task 8). The test lock `whole_tree` is now test-only.
- **Dependency.** `telperion-render` depends on `telperion-space` (it implements `Roots` on the engine's tables); before, it was a dev-dependency.
- **Wasm32 in part 1.** The browser still takes `prepare_whole`: `mod fine` is gated off wasm32 until part 2 makes its readbacks async.

### Tests

- `route_tests.rs` (in the gate):
  - One generator builds the spruce at 30, the beech at 30 and 3, the spruce at 1, the oak, the palm and the beech at 12, in that order. The large tree comes before the small ones, as a page's dials do.
  - Each tree takes the path `apart` names, its leaf count equals the whole path's, and it is submitted.
  - A CPU delivery still grows the tree whole.
- `drawn_tests.rs` (ignored, one side a process): a parameter change to a drawn tree, and the steady frame.

### R2: drawn natively within tolerance (`raw/task10/`)

**Parity probe on the real path** (`real_path_at_80`, `parity-80.txt`):

| | Spruce 80 s1 | Beech 80 s1 | Beech 80 s7 | Beech 40 s1 |
|---|--:|--:|--:|--:|
| Leaves (device, CPU) | 16,870,695 / 16,870,693 | 2,474,227 / 2,474,225 | 4,294,851 / equal | 65,633 / equal |
| Over the bound | 0 | 0 | 0 | 0 |
| Left out (device run unmatched, count apart, reference run unmatched) | 0 / 332 / 0 | **16 / 0 / 14** | 0 | 0 |
| Runs matched / parted / missing | all / 0 / 0 | all / 0 / 0 | | |
| Placement flags | 0 | 0 | 0 | 0 |

- **Spruce 80 s1** is task 7's figure after task 8: the half-micrometre rounding class (332 leaves, 2 more on the device).
- **Beech 80 s1 has 30 leaves on runs the other side lacks:** 16 on the device's runs, 14 on the reference's. That is 1.2e-5 of its leaves, and they make its 2 extra leaves. No earlier task measured the real path on this tree: task 7 took the beech at 30, and TASK6's station-count class names one key on beech 80 s1. **Not traced: unclassified.**

**Stills through `Generator::prepare`,** hero camera, 1280 × 900, against the whole path's (`stills/`, all four viewed, `stills-diff.txt`):

| Tree | Exact pixels apart | Under 6 % fuzz |
|---|--:|--:|
| Spruce 80 s1 | 0.15 % | 0.03 % |
| Beech 80 s1 | 0.27 % | 0.07 % |

Indistinguishable by eye on both trees.

### R5a natively: build latency and steady frame, before and after (`drawn-*.txt`, `report-*.json`)

- **Method.** One process a build, both sides on the same binary:
  - before is `prepare_whole` (the CPU executor's path);
  - after is `prepare`.
- **Build latency** runs from the parameter change to the first still drawn and read back. A small tree is built first, so the generator, its pipelines and the device's kernels already exist, as on a page.
- **Frame** is `timing::run`'s p50 at the hero camera: 120 frames, every verdict valid.
- **Conditions.** Spruce passes ran at a load of 12 to 36; the beech at 7.

| Tree | Side | Prepare ms | Drawn ms | Peak RSS MB | Vegetation p50 ms | Shadow p50 ms | Wood surfacing p50 ms | Frame p50 ms |
|---|---|--:|--:|--:|--:|--:|--:|--:|
| Spruce 80 s1 (4 runs a side) | before | 8,396 to 10,437 | 8,963 to 11,193 | 4,763 to 4,788 | 38.1 to 38.6 | 8.1 | 9.3 to 9.6 | 56.3 to 57.2 |
| | after | 1,453 to 2,349 | 1,592 to 2,570 | 766 to 800 | 48.5 to 48.9 | 10.7 | 9.2 to 10.2 | 69.2 to 70.7 |
| Beech 80 s1 | before | 1,731 | 2,003 | 1,145 | 9.39 | | 3.58 | 14.00 |
| | after | 373 | 550 | 414 | 9.29 | | 3.27 | 13.59 |

- **Build latency falls 5 to 6×** on the spruce at 80 and 4.6× on the beech. The process's peak memory falls 6× on the spruce, from 4.8 GB to 0.8 GB.
- **The spruce's steady frame rises 23 %:** vegetation +10.5 ms, shadow +2.6 ms. The instances, the levels (all at level 0), the casters and the selection (0.85 ms) are the same on both sides. What differs is the leaves' order in the buffer:
  - consecutive leaves stand 0.074 m apart on average on the device path, against 0.020 m before (p99 1.51 m against 0.29 m; `coherence-spruce80.txt`);
  - the beech shows the same spread (0.141 against 0.033 m) and no cost.

  The order is the likely cause, by correlation; it is not proven. The device's fine runs keep D5's order, widest first across every fine root, so each run's needles land far from the previous run's. **For the host** (below).

### Gate (part 1)

- **Render half,** under the GPU lock, first: 164 passed, 0 failed (`gate-render-1.txt`).
- **CPU half:** 988 passed, 0 failed (`gate-cpu-1.txt`).
- `RAYON_NUM_THREADS`, `RUST_TEST_THREADS` and `CARGO_BUILD_JOBS` were 8.

### For the host, after part 1

1. **The spruce's frame cost (+23 %) from leaf order.** Options:
   - (a) the device places its fine stations in their roots' order (bearer, then run) rather than D5's widest-first run order. Leaves would follow the tree, as on the CPU path. Placement and parity code would change; the curve would not.
   - (b) a device sort of the placed leaves by a spatial key after placement. It costs one pass of about 17M keys a build.
   - (c) accept it until fn-176's level of detail reorders leaves anyway.
2. **Beech 80 s1's 30 unclassified leaves.** Trace them now, or name them with the beech's other residual classes.
