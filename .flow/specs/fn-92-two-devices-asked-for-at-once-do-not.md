# Two devices asked for at once do not crash the process

## Goal

`cargo test --profile ci -p telperion-render --test bark_plates` died with SIGSEGV on the owner's desk (4 of 4) and passed with `--test-threads=1`. The core (2026-09-20) showed one test thread in `wgpu::Instance::new` -> `vkEnumerateInstanceExtensionProperties` calling a null pointer while another was inside `vkCreateInstance` loading the NVIDIA ICD. Every device-backed test builds a fresh instance through `Gpu::request(None)`, and libtest runs a binary's tests as threads of one process, so the workspace gate could not go green on an NVIDIA desk (CI was green because nextest isolates each test). Whether the loader or the driver is at fault is unknown; the repository's part is asking for instances concurrently, which it has no need to do. [user]

## Decisions

- Serialise instance creation inside `Gpu::request` on native targets: one process-wide `static Mutex<()>` in `crates/telperion-render/src/device.rs`, held around `wgpu::Instance::new` alone and released before any `.await` (host, 2026-09-20, narrowing the first draft's wider span). A poisoned lock is recovered with `into_inner`. `wasm32` untouched.
- The lock lives in `Gpu::request`, not `tests/common`, because four call sites bypass the helper and an embedder asking for two devices would meet the same crash.
- Sharing one `Gpu` per binary behind a `OnceLock` was set aside (couples tests through device state, misses in-crate callers); it is the fallback, to go to the host as a design question.
- R1 amended by the host (2026-09-20): the red proof is `bark_plates` itself; `concurrent_request.rs` stays as a smoke test that has never been seen red.

## Acceptance Criteria

- **R1:** `bark_plates` as the built `ci` binary with default threading crashes on base and not with the lock; counts and the core's two frames recorded — met: 5 of 12 on base `5a6b9aca`, 0 of 36 as shipped (`3b9eba06`). A bare concurrent `Gpu::request` did not reproduce (0 of 65 across three shapes); what `bark_plates` adds is unknown.
- **R2:** `cargo test --profile ci -p telperion-render` with default threading passes 5 of 5 — met.
- **R3:** `cargo test --profile ci --workspace` runs to the end with no target failed — met (exit 0, 375 s, 101 suites).
- **R4:** The lock exists only on non-`wasm32` targets and the wasm CI job stays green — met.
- **R5:** Render crate wall time recorded before and after — met.

## Result

Render crate wall time (`cargo test --profile ci -p telperion-render`, tests built): base with `--test-threads=1` 153 s (38 suites; no green default-threading baseline exists); shipped with default threading 95-102 s, 5 of 5 green (39 suites, `concurrent_request` adds about 10 s). One batch of five runs crashed and was discarded: it followed a base build sharing the same target directory, so it most likely ran the base binary (inferred, not observed).

## Open

None. Filing a loader/driver bug upstream would be a separate act if the owner wants it. The test that rewrites tracked `fn71/resolution/smooth_bark.json` on every run is a separate defect.

## Friction

- R1's prescribed repro (bare concurrent request) never went red; ~35 of 60 minutes spent before switching to `bark_plates` -> fed the "run a defect's repro before writing criteria around it" rule in AGENTS.md.
- Local `dcg` hook blocked `>` redirects and `git restore` -> local setup, reported only.
- A target directory shared with a base checkout ran the base binary (~12 min) -> AGENTS.md rule: two checkouts never share a `target/`.
