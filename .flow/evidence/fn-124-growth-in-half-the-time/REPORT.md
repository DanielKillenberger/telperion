# Growth in half the time: profile, three exact fixes, gate missed

R2's gate is missed. Three byte-identical fixes cut the browser skeleton by 11 to 26%. Half the base needs 35 to 38 ms on the oak and 26 to 28 ms on the spruce. The candidate is at 62 to 65 ms and 40 to 41 ms. No candidate the profile ranks closes that on one Wasm thread without changing output. The task stops with `NEEDS_HUMAN`; the ranked remainder is at the end.

## What shipped (R3: all byte-identical)

1. **Hypot with a hardware square root.** `libm` is pinned with `default-features = false`, so its `arch` feature is off and every `libm::hypot` finishes in libm's software square root, on native and in Wasm. `math/transcendental/hypot.rs` is libm's hypot step for step, with `f64::sqrt` (`f64.sqrt` in Wasm, `sqrtsd` natively). IEEE 754 rounds square root correctly, so both give the same bits. A test checks the port against the pinned `libm::hypot` bit for bit on 250,000 random and edge inputs.
2. **No bearing for an outline without lobes.** `Envelope::radius_toward` computed a hypot for the bearing and then `lobed` returned `radius_at(y)` unchanged at `irregularity == 0`, which both presets use. It now returns early; the value is the same by construction.
3. **Planned-run buffers sized once.** `Planner::run` reserves `count + 1` for its point and distance vectors instead of growing them by push.

Identity evidence: `growth_profile` hashes every node's full debug record, and all four fixtures match base across every sample (`native-compare.py`). `generation_stages` canonical hashes match in all 64 paired processes (`native-stages.json`). The core Wasm module's structure, wood and leaf arrays match base by SHA-256 (`wasm-hashes.mjs`). No visual comparison is needed.

## R2: browser skeleton and completed frame

Three rounds, alternated as base, candidate and SIMD candidate in rotating order. Each round is one page per fixture with one cold request and five warm ones. The table gives medians of the 15 pooled warm samples in ms (`browser-rounds.json`, Chromium 152 with WebGPU, not headless). The machine was shared with other agents' jobs.

| Fixture | Base skeleton | Candidate skeleton | Change | Half of base | Base frame | Candidate frame | Change |
|---|---:|---:|---:|---:|---:|---:|---:|
| Oak 1 | 69.7 | 61.8 | -11.3% | 34.9 | 159.4 | 151.6 | -4.9% |
| Oak 7 | 76.5 | 65.3 | -14.6% | 38.3 | 179.2 | 170.2 | -5.0% |
| Spruce 1 | 55.2 | 41.1 | -25.5% | 27.6 | 180.0 | 165.6 | -8.0% |
| Spruce 7 | 51.6 | 40.3 | -21.9% | 25.8 | 167.2 | 159.7 | -4.5% |

## R5: Wasm SIMD, dropped

Both modules built with `-C target-feature=+simd128` produce byte-identical CPU output. On the full `setTreeGpu` path in the same rounds they measure 61.3, 65.8, 41.0 and 40.1 ms skeleton, and 152.6, 169.8, 166.6 and 157.7 ms completed frame. Against the non-SIMD candidate that is -0.5 to +0.5 ms skeleton and -2.0 to +1.0 ms frame, within round-to-round spread. The hot code is scalar libm and branchy tree walking, which does not autovectorize. The flag is dropped; no `.cargo/config.toml` is added.

## R4: native stages and peak memory

`generation_stages` ran 10 alternated rounds per fixture, first build dropped, with peak RSS from `wait4` (`native-stages.json`). The table gives the candidate's change against base.

| Fixture | growth | rings | surface | placement | cull | total | Peak RSS base / candidate KiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| Oak 1 | -9.1% | +1.1% | +0.1% | +0.3% | -3.7% | -2.6% | 260,296 / 259,508 |
| Oak 7 | -4.9% | +1.5% | +1.6% | +0.3% | -3.8% | -0.8% | 296,316 / 293,892 |
| Spruce 1 | -9.5% | +1.6% | +1.4% | +0.8% | -14.3% | -1.6% | 302,592 / 301,318 |
| Spruce 7 | -9.9% | +3.1% | +0.7% | +0.8% | -14.9% | -1.7% | 287,756 / 286,742 |

Spruce 7 rings read +3.06%, just past R4's 3%. The rings code and every function it calls are unchanged. A 6-round rerun under `GENERATION_SERIAL` reads +1.55% rings, +2.09% placement and -6.87% surface (`native-stages-serial.json`). An earlier 4-round run read spruce 7 rings at +1.68% and oak 1 rings at -3.23%. The +3.06% is run-to-run variation, recorded rather than retimed away. The cull's -4 to -15% comes from hypot, which the cull's containment test calls.

## R1: where the skeleton's time goes (base 123c4261)

Native uses `growth_sampler.rs`, a scratch-only SIGPROF sampler with frame-pointer stacks and DWARF inline frames. It records about 1 kHz over 150 builds and 6,000 to 12,000 skeleton samples per fixture. Browser uses V8's sampling profiler at 50 µs over five `setTreeGpu` calls. The browser module is built from base with `#[inline(never)]` on `Planner::{run, heading, width}`, `rejected`, `Curtain::{admits, sagged, length, separation, clear}`, `in_band`, `radius_at`, `radius_toward`, `noise::seeded`, `limit_turn`, `GrowthBias::apply`, `internodes`, `child_radius` and the `Transcendental` wrappers, so stages keep their names; this makes it about 9% slower than the shipped base module. Stage attribution is 97.1 to 98.7% native and 97.9 to 98.8% browser (`profile-summary.json`). The rest are samples whose stage frame was lost at a function prologue. Function-level attribution in the browser is 100%. Natively, 6 to 16% of self time is in libc's allocator and memcpy outside the executable, and still falls in its stage.

| Stage, ms | Oak 1 native | Oak 1 browser | Spruce 7 native | Spruce 7 browser |
|---|---:|---:|---:|---:|
| Local advance, loop body (births, laterals, queue) | 16.7 | 22.4 | 14.4 | 14.1 |
| Planner run: envelope admission | 13.1 | 15.8 | 1.4 | 2.0 |
| Admission outside the planner | 7.5 | 10.6 | 1.7 | 3.6 |
| Planner run: heading and turn limit | 5.1 | 6.4 | 0.9 | 1.2 |
| Planner run: rest (stations, storage, resampling) | 4.0 | 8.7 | 1.9 | 2.9 |
| Scaffold advance | 3.7 | 5.1 | 10.8 | 15.1 |
| Local seeding | 1.1 | 0.8 | 7.1 | 7.3 |
| Radius solve | 2.9 | 2.8 | 2.8 | 3.5 |
| Identity and remap | 2.6 | 2.7 | 1.6 | 2.2 |
| Twig heading outside the planner | 1.2 | 2.8 | 1.2 | 2.0 |
| Validation | 0.6 | 1.4 | 0.3 | 0.8 |

The oak's local advance outside the planner, fn-91's unnamed 28 ms, is the loop body plus admission of births. Within the loop body the largest pieces are:

- `tree.nodes.push`, about 6% of the oak's samples, which grows and copies the node vector.
- `child_radius`'s `pow`, 4.5%.
- Shoot queue writes and allocation.

The functions under all stages, per browser call on base oak 1 (74 ms):

- `libm::pow`: 26 ms, 22 of it in `Envelope::radius_at`, two calls per containment test.
- Envelope containment: 28 ms inclusive.
- libm's software square root: 6 ms on the oak and 9 ms on the spruce.
- `limit_turn`: 5 ms, 2.7 of it in `acos`.

On the spruce, `pow` is cheap because its shoulder is 1. There, hypot's software square root and the scaffold's eight-point edge containment lead. After the fixes, candidate oak 1 still spends 22 of its 65 ms in `pow` inside `radius_toward`.

## Ranked remainder (why the gate cannot be met here)

1. **Oak: prepared envelope bounds.** fn-91 rejected this candidate. It is now the single largest oak item. fn-91 measured browser oak skeleton 73.4 to 61.7 ms and 77.4 to 60.6 ms with it, about 12 to 17 ms. On top of this candidate that projects the oak to about 45 to 53 ms, still above 35 to 38 ms. The spruce gains nothing, because its shoulder-1 tables were never prepared. Reviving it is the owner's call under the "rejected stays rejected" rule.
2. **Spruce: no item above 4 ms remains** in the candidate profile except the scaffold (7.6 ms, of which 3.1 is `Envelope::contains` over eight lerped points per edge), seeding (7.4 ms: the floor walk to the root per hanging station, `branch_length`'s two `pow`s, SipHash) and the loop body (8.2 ms). Reaching 26 to 28 ms needs about 13 ms more from these, in pieces of 1 to 3 ms.
3. **Parallel planning** (resolved before ready) helps native only; Wasm has no threads.
4. **Byte-changing options** need the owner's word before anyone builds them: a coarser clipping bisection, reserved node capacity sized from the envelope, or a cheaper envelope formula than two `pow`s per test.

## Reproduction

The base checkout is `git worktree add --detach <dir> 123c4261`, with its own `target/`, and `node_modules` linked from the main checkout.

- **Native identity and time:** `native-compare.py <base growth_profile> <candidate growth_profile>`.
- **Stages and RSS:** `stages-compare.py <base generation_stages> <candidate generation_stages> 10`, with `FIXTURES` and `GENERATION_SERIAL` as needed.
- **Browser timing:** after `npm run wasm:build && npm run render:build`, run `GENERATION_GPU=1 GENERATION_COMPLETED=1 node scripts/benchmarks/mature-generation.mjs` per arm, then `browser-rounds-summary.py`.
- **Browser profile:** `browser-profile.mjs`, then `analyze-v8.py` (`callers-v8.py` for callers).
- **Native profile:** copy `growth_sampler.rs` into `crates/telperion-core/examples/` of a scratch checkout. Build it with `RUSTFLAGS="-C force-frame-pointers=yes" CARGO_PROFILE_RELEASE_DEBUG=line-tables-only` into a separate target, then run `analyze-native.py`.
- **SIMD identity:** `wasm-hashes.mjs` per module build.

Raw profiles, rounds and logs are in the ignored `raw/`.
