---
satisfies: [R2]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.2 Renderer crate foundation and headless still

## Description
Create `crates/telperion-render` on wgpu with typed device creation, the clay scene with its scale figure, the camera with the hero pose, the wood pipeline for the plaited surface, and the native offscreen render to PNG behind a headless example. This is the early proof point (spec §Early proof point); foliage, views, timing and the browser come after it.

**Size:** M
**Files:** `Cargo.toml` (workspace member), `crates/telperion-render/Cargo.toml`, `crates/telperion-render/src/lib.rs`, `src/device.rs`, `src/scene.rs`, `src/camera.rs`, `src/wood.rs`, `src/headless.rs`, `src/shaders/scene.wgsl`, `src/shaders/wood.wgsl`, `crates/telperion-render/examples/headless.rs`, `crates/telperion-render/tests/headless.rs`
**Touches:** [Cargo.toml, Cargo.lock, crates/telperion-render/**]

### Approach
- Pin `wgpu = "30"` (verify with `cargo add`), `bytemuck` for slice casts, native-only `pollster` and `png` (or `image` with only the png feature) under `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`; `crate-type = ["cdylib", "rlib"]`.
- `device.rs`: `Instance::new(InstanceDescriptor)` → `request_adapter` (compatible surface when one exists) → reject `AdapterInfo.device_type == Cpu` as fallback-only, naming the adapter → `request_device` with `required_limits` = defaults raised to the adapter's `max_buffer_size` and `required_features` = `TIMESTAMP_QUERY` only when the adapter offers it. `RenderError` enum with a `Display` naming the condition (WebGPU absent, no hardware adapter with the fallback's name, device refused with the limit or feature, device lost with reason); implement `std::error::Error`.
- `scene.rs`: one uniform block (view-projection, sky, ground, clay colour), the hemisphere term from the prototype idea `clay * mix(ground, sky, 0.5 + 0.5 * n.y)` with the harness's values at `harness/stage.ts:27-33` and the hemisphere pair `0xffffff` / `0x6a6966` at `harness/stage.ts:520-560`, a ground disc, the background as clear colour, depth32, and the 1.8 m scale figure (two spheres of radius 0.28 and a 1.24 m body, `harness/stage.ts:38-42`) standing at the roots in the figure colour.
- `camera.rs`: perspective from position, target, vertical fov; `hero_pose(bounds)` reproduces the harness framing rule (`FRAME_DIRECTION` (0.62, 0.28, 1), `FRAME_MARGIN` 1.3 at `harness/stage.ts:47-53`, solve at `harness/stage.ts:340-388`) so the still and the browser share one pose.
- `wood.rs`: position + normal vertex buffers straight from `SurfaceMesh` via `bytemuck::cast_slice` and `queue.write_buffer`, u32 indices, one indexed draw; buffers allocated with headroom and a used range recorded (the append seam, spec §Architecture); frame stats (draw calls, triangles).
- `headless.rs` (native only): colour target `Rgba8UnormSrgb` + depth, `copy_texture_to_buffer` with rows padded to `COPY_BYTES_PER_ROW_ALIGNMENT`, `map_async` + `device.poll(PollType::Wait)`, strip padding, encode PNG.
- `examples/headless.rs`: hand-parsed args like `crates/telperion-core/examples/measure.rs` (no clap): `--preset <id> --seed <n> --out <png> [--size WxH]`; resolves `Preset::from_id`, overrides `skeleton.seed`, calls `mesh::build`, renders at `hero_pose`, prints the reason to stderr and exits 1 on any error. Only this file may name presets.
- `tests/headless.rs`: renders the ordinary preset at 256×256, asserts some pixels differ from the background, and returns early with a printed skip when `request_adapter` yields none.

### Investigation targets
**Required** (read before coding):
- `harness/stage.ts:27-60,340-388,520-560` — clay values, figure, framing rule, hemisphere light
- `crates/telperion-core/examples/measure.rs` — arg parsing and reporting precedent
- `crates/telperion-core/src/surface.rs:59-70` — the buffers to upload
- `crates/telperion-core/src/mesh.rs` — the contract from task 1

**Optional** (reference as needed):
- wgpu `render_to_texture` and `hello_triangle` examples in the wgpu repository (features examples), Learn Wgpu windowless showcase for padded readback
- `rust-toolchain.toml` — channel 1.98.1, wasm32 target already installed

### Key context
- wgpu 25+ renamed `ImageCopyBuffer` to `TexelCopyBufferInfo` and `request_device` takes one `DeviceDescriptor`; `Device::poll` takes `PollType`. Older snippets will not compile.
- Never call `Surface::configure` while a `SurfaceTexture` is alive (headless has no surface; this matters for task 5, keep the frame loop shaped for it).
- Oak at full detail is about 5 million wood triangles; the wood upload is about 40 MB and fits default limits, but request the adapter's `max_buffer_size` now because task 3's spruce foliage needs it.
## Acceptance
- [ ] `cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 1 --out /tmp/oak.png` writes a PNG of the wood, ground and scale figure at the hero pose and exits 0
- [ ] Unknown preset id, unwritable output path, no adapter and device refusal each exit non-zero with the reason on stderr
- [ ] `RenderError` names the failing condition for WebGPU absent, fallback-only adapter (with its name), device refused (with the limit or feature) and device lost
- [ ] A unit test on `hero_pose` shows the eight bounds corners project inside the frame with the margin
- [ ] Wood buffers record a used range and are allocated with headroom; a unit test shows a second smaller submit reuses them
- [ ] `crates/telperion-render/src` contains no `presets` reference (`grep -r presets crates/telperion-render/src` is empty)
- [ ] `cargo test --release -p telperion-render` passes here and skips with a printed reason when no adapter exists; clippy clean; every file under 400 lines
## Done summary
`crates/telperion-render` now draws the hero tree: a wgpu 30 device whose every failure names its own condition, the clay room rebuilt in Rust, the harness's framing rule solved from bounds alone, the core's wood arrays uploaded as they lie, and a native offscreen render out to PNG behind a headless example. The oak still renders at 1024x1024 in about a second on this machine and the spec's early proof point holds: wgpu on this toolchain and the plaited surface are the right base for the foliage, timing and browser tasks.

Scope notes for the reviewer:

- `hero_pose` takes the aspect and the ground reach, not bounds alone as the API sketch had it. Both are load-bearing: the harness solves the horizontal fit against the aspect, and the far plane is sized off the ground disc from wherever the camera stands. The camera unit tests are margin-sensitive by construction - dropping `FRAME_MARGIN` to 1.0 fails both of them, which is how the assertions were checked before they were trusted.
- The acceptance asked for a unit test showing the corners inside the frame "with the margin". Asserting the widest corner at `1/FRAME_MARGIN` is false for perspective: the solve fits the subject's centre plane, and a near corner subtends more. The margin's real purpose, in the harness's own words, is that the subject stays clear of every edge while it is orbited - so the test turns the pose through a full circle at four aspects and asserts no corner ever leaves. That is what the number buys, and it is strictly stronger than the static claim.
- Buffer reuse is decided in one pure `Region` type, so the "second smaller submit reuses them" criterion is a unit test with no device, and the same decision is exercised against real hardware in `tests/headless.rs`.
- The device asks for the adapter's `max_buffer_size` now, ahead of task 3's 507 MB of spruce instance matrices, and pre-checks the limits with `check_limits_with_fail_fn` so a refusal names the limit instead of arriving as a validation panic.
- No oversize-buffer rejection and no foliage draw: R3's fit check and the instanced foliage pipeline are task 3's, and building them here would have been scope. The frame loop is already shaped for a surface (nothing configures while a texture is held), for task 5.
- `wgpu::Device::poll` is checked for device loss on the readback path, and the device-lost callback stores the reason, so `RenderError::DeviceLost` carries something real rather than a placeholder.
- Line counts: the largest file is `src/scene.rs` at 337. Nothing under `src` mentions a named family; only `examples/headless.rs` resolves an id.

stage: impl-review - skipped(config: REVIEW_MODE=none; parallel wave - conductor reviews after integration)

Conductor note: the oak still at 1024x1024 frames the tree at roughly a third of the frame height; task 4 judges the hero pose framing against the harness rule when it records the evidence stills.

stage: plan-sync - skipped(config: planSync.enabled != true)
stage: wave-join - ran (commit 9ddf531 already on target; no integration needed)
## Evidence
- Commits: 9ddf53170b6d5787a5464b6877e78725261b70da
- Tests: cargo test --release --workspace (green, suite_rc=0; green receipt 9ddf5317-unittest), cargo test --release -p telperion-render (17 tests: 15 unit, 2 device-backed), WGPU_BACKEND=noop cargo test --release -p telperion-render --test headless (both device tests skip with a printed reason), cargo clippy --workspace --all-targets --release (telperion-render clean; one inherited assign_op_pattern warning in crates/telperion-core/examples/geometry_benchmark/metrics.rs, untouched by this task), cargo fmt --all -- --check (clean), cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 1 --out /tmp/oak.png (exit 0, 183 KB PNG, 5,190,920 wood triangles in 2 draw calls on an RTX 3080), example error paths: unknown id, unwritable path, missing --out, malformed --size, malformed --seed, no adapter (WGPU_BACKEND=noop) all exit 1 with the reason on stderr, grep -r presets crates/telperion-render/src (empty)
- PRs: