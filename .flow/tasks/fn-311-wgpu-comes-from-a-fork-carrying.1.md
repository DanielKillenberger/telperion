---
satisfies: [R1]
---
# fn-311-wgpu-comes-from-a-fork-carrying.1 The fork branch 30.0.1-async: #10438's browser half, a minimal native side, a README, pushed

## Description
Build branch `30.0.1-async` of `DanielKillenberger/wgpu` per R1 in the scratch clone, check it compiles natively and for `wasm32-unknown-unknown`, and push it. The workspace task (.2) pins the pushed commit, so this task ends with the rev recorded.

**Size:** M
**Files:** fork only (scratch clone); `.flow/evidence/fn-311-wgpu-comes-from-a-fork-carrying/FORK.md` in telperion
**Touches:** [.flow/evidence/fn-311-wgpu-comes-from-a-fork-carrying/**]

### Approach
- Scratch clone: `/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/wgpu-fork`. Remotes `origin` (the fork), `upstream` (gfx-rs), local ref `pr-10438` = `9d3215c1`, local branch `30.0.1-async` at v30.0.1 (`40f4a34eb`). Reuse it; if missing, re-clone the fork and fetch `upstream` tag v30.0.1 and `pull/10438/head`.
- Commit 1, the browser half verbatim: `git checkout pr-10438 -- <paths>` is blocked by the dcg hook, so take the PR commit's own diff for exactly these paths and apply it: `git show 9d3215c1 -- wgpu/src/api/device.rs wgpu/src/backend/webgpu.rs wgpu/src/backend/webgpu/webgpu_sys/ wgpu/src/dispatch.rs wgpu/Cargo.toml xtask/src/vendor_web_sys.rs examples/standalone/custom_backend/src/custom.rs | git apply --index`. These all apply cleanly to v30.0.1 (trial cherry-pick, 2026-10-24). Not taken: `wgpu-core/**`, `wgpu/src/backend/wgpu_core.rs`, `tests/tests/wgpu-gpu/pipeline.rs` (tests the PR's native error semantics), `CHANGELOG.md`. Commit message names the PR, head and author (`Islam Abdymazhit`, keep `--author` from the PR commit).
- Commit 2, the native side: in `wgpu/src/backend/wgpu_core.rs` (`impl dispatch::DeviceInterface for CoreDevice`, near `create_render_pipeline` ~1337 and `create_compute_pipeline` ~1525 on v30.0.1) add `create_render_pipeline_async` and `create_compute_pipeline_async`: call `self.create_*_pipeline(desc)` and return `Box::pin(core::future::ready(Ok(pipeline)))`, the form the PR uses in the custom-backend example. Nothing else changes; errors keep the sync error-scope path. Any other in-tree `DeviceInterface` implementor that fails to compile gets the same two-line form.
- Commit 3, `README-30.0.1-async.md` at the repo root (do not overwrite wgpu's README.md): R1.3's content, plus that the PR's doc comment on native errors (delivered through the future) does not hold on this branch, where native errors still go to the error scope.
- Check, one cargo at a time, each inside `~/.local/bin/telperion-heavy 5 --threads 2 -- …` with `CARGO_TARGET_DIR` under the scratchpad (never telperion's target): `cargo check -p wgpu` and `cargo check -p wgpu --target wasm32-unknown-unknown`, and `cargo check -p wgpu-example-custom-backend` (or whatever the custom backend example's package is named in its Cargo.toml). Check `ps -eo pid,args | grep target/ci/deps` first.
- Push: `git push origin 30.0.1-async` (no force, only this branch, only `origin` = the fork). Verify with `git ls-remote origin 30.0.1-async`.
- Write `FORK.md` in the evidence dir: branch, head rev (full sha), the three commits, the checks run and their result. Commit it in telperion.

### Investigation targets
**Required:**
- `git show 9d3215c1 --stat` in the scratch clone: the PR's file list
- `wgpu/src/backend/wgpu_core.rs` at v30.0.1, the `DeviceInterface for CoreDevice` impl
- `examples/standalone/custom_backend/src/custom.rs` after commit 1: the ready-future pattern

### Key context
- The PR's trait methods have no default implementation, so every implementor must gain them or the crate fails to build; that is the compile check's job.
- No comment or PR upstream (Boundaries). Never push to `upstream`.

## Acceptance
- [ ] Branch `30.0.1-async` on `DanielKillenberger/wgpu` is v30.0.1 plus three commits: the browser half applied verbatim from `9d3215c1`, the native ready-future methods, the README (R1).
- [ ] `git diff v30.0.1 30.0.1-async -- wgpu-core` is empty.
- [ ] `cargo check -p wgpu` passes natively and for `wasm32-unknown-unknown`, and the custom-backend example builds.
- [ ] `git ls-remote origin 30.0.1-async` shows the pushed head; no force push, nothing pushed upstream.
- [ ] `FORK.md` in the spec's evidence dir records the full rev and the checks.

## Done summary
Branch `30.0.1-async` of DanielKillenberger/wgpu is v30.0.1 plus three commits (the browser half of gfx-rs/wgpu#10438 applied verbatim from 9d3215c1 under its author, the native ready-future `create_*_pipeline_async` on `CoreDevice`, and `README-30.0.1-async.md`), checked natively, for wasm32 and for the custom-backend example, and pushed at `165ebd16f930baefecca51c22e753a48f911a5fa`; FORK.md in the evidence dir records it for task .2's pin.

wgpu-core is untouched (`git diff v30.0.1 30.0.1-async -- wgpu-core` empty). The trait and `Device` doc comments from the PR say failures arrive through the future; on this branch that holds only on WebGPU, and the README says so.

Baseline: none (fork-only task; the spec's Quick commands target task .2's workspace switch). Telperion gate classify: TIER_B docs-only.

Review note: flowctl's triage-skip was not run, because the telperion range holds only evidence notes and would have passed as SHIP without the fork being read; the reviewer was pointed at the fork range through `--focus` (FRICTION.md, second entry).

Tier: not supplied by the conductor (model: claude-opus-5-5)

stage: impl-review - ran (codex, one correctness draw, SHIP, rid 945faa8770f1451880f6e134d24f4e23)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b66dc09e1fee4ef5bbb34d938fc88767f5a4dccf, 54044cfa0664f1be00e60ea9e5006e51df3e7fbd
- Tests: fork 30.0.1-async: cargo check -p wgpu (rc 0), fork 30.0.1-async: cargo check -p wgpu --target wasm32-unknown-unknown (rc 0), fork 30.0.1-async: cargo check -p wgpu-example-custom-backend (rc 0), GATE_SKIPPED:telperion:docs-only - cumulative telperion diff classified tier-B (evidence notes only)
- PRs: