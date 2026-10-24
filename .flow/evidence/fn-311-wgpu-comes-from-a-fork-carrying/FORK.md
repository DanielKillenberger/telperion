# The fork branch (fn-311 task .1)

- Repository: https://github.com/DanielKillenberger/wgpu
- Branch: `30.0.1-async`
- Head: `165ebd16f930baefecca51c22e753a48f911a5fa` (task .2 pins this as the `rev`)
- Base: tag `v30.0.1`, `40f4a34ebaf56f9a046231f54125ad046239d3f3`

## Commits on top of v30.0.1

1. `b8b869ebf5cbb2596296c1370bd9ac0427a880f2`, author Islam Abdymazhit: the browser half of gfx-rs/wgpu#10438, applied with `git show 9d3215c1 -- <paths> | git apply --index` for `wgpu/src/api/device.rs`, `wgpu/src/backend/webgpu.rs`, `wgpu/src/backend/webgpu/webgpu_sys/` (the three `gen_GpuPipelineError*` files and `mod.rs`), `wgpu/src/dispatch.rs`, `wgpu/Cargo.toml`, `xtask/src/vendor_web_sys.rs` and `examples/standalone/custom_backend/src/custom.rs`. Every hunk applied cleanly.
2. `86206b38d9cb6e1ae00303858940e2372b30ae54`: `CoreDevice::create_render_pipeline_async` and `create_compute_pipeline_async` in `wgpu/src/backend/wgpu_core.rs` call the synchronous `create_*_pipeline` and return `Box::pin(core::future::ready(Ok(pipeline)))`. 16 lines; no other implementor needed changes.
3. `165ebd16f930baefecca51c22e753a48f911a5fa`: `README-30.0.1-async.md` at the repository root: what the branch carries, the PR and its head `9d3215c1`, that native errors still go to the error scope although the PR's doc comments say they come through the future, and that the branch is dropped once a wgpu release carries #10438.

`git diff v30.0.1 30.0.1-async -- wgpu-core` is empty (measured).

## Checks (rustc 1.93.1, `CARGO_TARGET_DIR` under the session scratchpad, each inside `telperion-heavy 5 --gpu 3 --threads 2`)

| Command | Exit | Warnings |
| --- | --- | --- |
| `cargo check -p wgpu` | 0 | 0 |
| `cargo check -p wgpu --target wasm32-unknown-unknown` | 0 | 0 |
| `cargo check -p wgpu-example-custom-backend` | 0 | 0 |
| `rustfmt --check wgpu/src/backend/wgpu_core.rs` | 0 | n/a |

The wasm check built wgpu's `rmeta` with the default `webgpu` feature, so the browser half was compiled.

## Review and push

- Codex impl-review (one correctness draw, inside `telperion-heavy`): SHIP. It compared the applied hunks with `9d3215c1` by patch id and found them equal.
- `git push origin 30.0.1-async`: a new branch, no force. `git ls-remote origin 30.0.1-async` returns `165ebd16f930baefecca51c22e753a48f911a5fa`. Nothing was pushed to gfx-rs.
