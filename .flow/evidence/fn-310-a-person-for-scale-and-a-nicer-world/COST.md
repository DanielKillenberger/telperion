# Cost: frame time and the render Wasm against the base (fn-310 task 3, R4)

Base: `201288747`, this spec's cut from the release branch, exported with `git archive` into a scratch directory and built there under its own `CARGO_TARGET_DIR`. It draws the old capsule and the flat sky. Branch: `6d73657e5`, variant C's row and the person, both at their defaults. Both are built in the `ci` profile on the RTX 3080 (driver 610.57.04, Vulkan). The base binary carries the base shaders: a comment that exists only in the branch's `sky.wgsl` is absent from its strings. Its `telperion-render` sources match `201288747` file by file. Only `telperion-render` differs between the two commits.

## Frame time

The probe is `examples/frames.rs` (docs/gpu-tests.md), at 1920x1080 with multisampling. It ran `AGE=<age> SEED=7 frames <preset> full whole mid close` through `telperion-heavy 5 --gpu 3 --threads 2`. Each command took one tree, and base and branch alternated. Three runs each, and the side that went first swapped every round. The trees were the oak at 40 and Telperion at 80, seed 7. The views it measured:

- `whole` is the hero pose, the concept's framing.
- `mid` is half the distance.
- `close` is the near view.

The script is `raw/frames/frames.sh` and its rows are `raw/frames/ab.jsonl`.

There are two clocks:

- `frame_ms` is wall time over 120 pipelined frames, so it includes the room pass with the sky, the floor and the person.
- `vegetation p50` is the GPU timestamp of the vegetation pass alone. It does not include the room.

Values are medians, with the range of the three runs in brackets. "Apart" means the two ranges do not overlap.

| Tree | Measure | Base | Branch | Change | Apart |
|---|---|---|---|---|---|
| oak 40 | whole frame_ms | 3.298 (3.288-3.299) | 3.333 (3.325-3.343) | +0.035 (+1.1%) | yes |
| oak 40 | mid frame_ms | 7.999 (7.979-8.002) | 8.053 (8.027-8.111) | +0.054 (+0.7%) | yes |
| oak 40 | close frame_ms | 10.347 (10.330-10.445) | 10.392 (10.380-10.466) | +0.045 (+0.4%) | no |
| oak 40 | whole vegetation p50 | 2.279 (2.262-2.280) | 2.268 (2.262-2.278) | -0.011 | no |
| oak 40 | warm_ms | 258.6 (257.5-260.1) | 259.9 (259.8-260.6) | +1.3 | no |
| oak 40 | **fresh_ms** | 6,083 (5,798-6,096) | 7,762 (7,683-7,771) | **+1,679 (+27.6%)** | yes |
| Telperion 80 | whole frame_ms | 7.591 (7.572-7.592) | 7.616 (7.614-7.683) | +0.025 (+0.3%) | yes |
| Telperion 80 | mid frame_ms | 18.464 (18.432-18.502) | 18.469 (18.452-18.488) | +0.005 | no |
| Telperion 80 | close frame_ms | 15.435 (15.428-15.480) | 15.443 (15.406-15.689) | +0.008 | no |
| Telperion 80 | whole vegetation p50 | 5.354 (5.348-5.360) | 5.340 (5.339-5.384) | -0.013 | no |
| Telperion 80 | warm_ms | 1,271.6 (1,262.4-1,272.9) | 1,264.2 (1,257.7-1,296.2) | -7.4 | no |
| Telperion 80 | **fresh_ms** | 6,892 (6,855-6,918) | 8,548 (8,499-8,708) | **+1,655 (+24.0%)** | yes |

Two differences are outside noise. Each is stated here and has not been tuned away.

- **Frame time, +0.025 to +0.054 ms a frame** where the ranges come apart: the whole view of both trees and the oak's mid view. That is 0.3 to 1.1 percent. The vegetation pass does not move, so the cost is the room pass, which is the glow, the ground noise and the haze. At 60 fps this is 0.3 percent of the 16.7 ms frame. The other views are within noise.
- **Start time, +1.66 to +1.68 s** to the first fenced frame (`fresh_ms`). The figure is about the same for both trees, and the wider trial round (not in the table: oak 6,914 against 7,749, Telperion 6,682 against 8,531) agrees with it. `fresh_ms` includes shader compilation. A rebuild of the tree (`warm_ms`) does not move, so the cost is paid once, when the renderer starts, and not per tree. The cause has not been looked into. One thing is consistent with it: the new WGSL (`sky_radiance`, `ground_field`, `value_noise`) sits in `common.wgsl`, the prelude every shipped module includes. That is an inference, not a measurement. Whether this cost stays, and where it goes if not, is a call for the host.

## The render Wasm

Both modules were built with `node scripts/build-render.mjs`, the package's recipe: release, `wasm32-unknown-unknown`, wasm-bindgen 0.2.128. Each came from its own checkout and target.

| Module | Bytes |
|---|---|
| Base `201288747` `dist/telperion-render.wasm` | 3,482,370 |
| Branch `6d73657e5` `dist/telperion-render.wasm` | 3,498,703 |
| Change | +16,333 (+0.47%) |
| Ceiling, `scripts/artifact-budgets.json` | 3,700,000 (201,297 bytes of headroom on the branch) |

The branch is inside the ceiling and inside the 5 percent growth share.

## The start-time cost under equal cache state (task 4)

The +1.7 s above is not the branch's: it is the state of the NVIDIA driver's shader disk cache. The renderer keeps no `wgpu` pipeline cache, so the driver's disk cache is the only one in play. Task 3's runs used the default cache, `~/.cache/nvidia/GLCache`, which stands at 1,073,737,677 bytes, at the driver's 1 GiB ceiling, and takes no new entries: two more default-cache runs of each binary after the measurements below wrote no file to it and did not get faster (base 5,773 and 5,902 ms, branch 7,682 and 7,914 ms). Entries the base's shaders left there before the cache filled still hit; the branch's new modules miss on every run. So the default cache gave the base a partial warm start and the branch a cold one.

Method: the same two binaries as task 3 (the base's carries no `sky_radiance`, the branch's does), `frames oregon-white-oak full whole` with `AGE=40 SEED=7` through `telperion-heavy 5 --gpu 3 --threads 2`, base and branch alternated with the first side swapped every round. Cold: `__GL_SHADER_DISK_CACHE=0`, the driver keeps no disk cache. Warm: a cache directory of each side's own (`__GL_SHADER_DISK_CACHE_PATH`), primed by one unrecorded run. The user's cache was neither moved nor cleared. Script and rows: `raw/frames/ab.sh`, `cold1.jsonl`, `cold2.jsonl`, `warm1.jsonl`.

| Cache state | Runs each | Base fresh_ms, median (range) | Branch fresh_ms, median (range) | Change | Apart |
|---|---|---|---|---|---|
| Cold, cache off | 7 | 8,109 (8,015-8,632) | 8,245 (8,071-8,858) | +136 (+1.7%) | no |
| Warm, own primed cache | 3 | 377 (374-380) | 374 (372-386) | -3 | no |
| Default user cache (task 3, full) | 3 | 6,083 (5,798-6,096) | 7,762 (7,683-7,771) | +1,679 | yes |

Under equal cache state the difference is within noise, cold and warm. The hero frame's wall `frame_ms` in the cold rows agrees with the table above (base 3.26-3.29 ms, branch 3.31-3.34 ms). So the remaining steps reduce to stating this, as the task's order says: the sky and ground functions stay in `common.wgsl`, no per-module timing was needed, and no code changed, so drawing is unchanged and there is no cause for a guard to catch.

The final start numbers: about 8.1 s cold and about 0.38 s warm for the oak at 40, the same on the base and the branch. The frame numbers are task 3's table above, unchanged.

Two things follow that are not this spec's to fix. Any `fresh_ms` taken on this machine with the default cache measures what the full cache happens to hold, not the code, so a release-bar start number (fn-237 R0a) needs a stated cache state. And a cold start is 8 s on the base too: that is the cost a first visit pays, and it predates this spec.
