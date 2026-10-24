# fn-311 task .2 proof (2026-10-24)

Fork rev pinned: `165ebd16f930baefecca51c22e753a48f911a5fa` (DanielKillenberger/wgpu, branch `30.0.1-async`). Raw logs are in `raw/` (gitignored).

## Lock and resolution (R2, R4)

- `cargo update --workspace` rewrote the ten family entries (wgpu, wgpu-core, wgpu-hal, wgpu-types, naga, naga-types, wgpu-naga-bridge, wgpu-core-deps-apple, -emscripten, -windows-linux-android) to `git+https://github.com/DanielKillenberger/wgpu?rev=165ebd16…#165ebd16…`. No other lock entry changed (`Cargo.lock`: 10 insertions, 18 deletions, all source and checksum lines). Cargo printed no "patch was not used" warning.
- `cargo tree -p telperion-render -d`, native: duplicates are hashbrown, miniz_oxide and syn only. With `--target wasm32-unknown-unknown`: libm and syn only. No wgpu-family crate is listed on either.
- `scripts/wgpu-family.test.mjs` passes on the real lock. Against a temp copy of the lock with `wgpu-hal`'s source set back to crates.io it fails, each test reporting `"wgpu-hal: from registry+https://github.com/rust-lang/crates.io-index"` (4 of 4 failed; `raw/vitest-neg.log`).

## Lines removed

`git diff --shortstat 0a62c7ec2 HEAD` (the spec base): 206 files changed, 512 insertions, 42,641 deletions. This task's commit alone: 199 files, 172 insertions, 42,632 deletions.

## Wasm render module (R4)

`npm run render:build`, `src/browser/telperion-render.wasm`:

| Build | Bytes |
| --- | --- |
| Base `8bb9f9e9b` (vendored copy) | 3,504,023 |
| This task (fork) | 3,506,337 |

+2,314 bytes, +0.066% against the base (limit 5%); ceiling 3,700,000. `node scripts/artifact-budgets.mjs` after `npm run build`: the render row passes its ceiling; the run exits 1 on `dist/wasm-source.js: not built by the recipe` (the build emitted `dist/module.js` and no `wasm-source.js`; no frontend or vite file changed in this task, so this is inferred to predate it, not measured on the base).

## Tests (R3, R4)

| Command (each inside telperion-heavy) | Result |
| --- | --- |
| `cargo check --profile ci -p telperion-render`, native and `--target wasm32-unknown-unknown` | exit 0, no new warnings |
| `cargo test --profile ci -p telperion-core --test generation_limit_guard` | 2 passed, 1 ignored |
| `cargo test --profile ci -p telperion-render --lib compile_size` | 2 passed, 1 ignored |
| `cargo test --profile ci -p telperion-render --lib programs::tests` | 5 passed, including the new `a_refused_compile_keeps_the_reason_and_message_shape` |
| `npm ci`, then `npm test` | 23 files, 181 tests passed (`src/browser/render.test.ts` among them) |
| `npm run test:render` | exit 1, "Ordinary generated no wood" at `tests/browser/render.mjs:337`; the base (an archive of `8bb9f9e9b`, its own target dir) fails identically, so the failure predates this task |
| `python3 scripts/gate.py` | exit 0: 1501 tests run, 1501 passed, 136 skipped, 1301 s |

Open for the owner: fn-243 R4's cold-profile Chrome run.
