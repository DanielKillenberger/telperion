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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
