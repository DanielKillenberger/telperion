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
