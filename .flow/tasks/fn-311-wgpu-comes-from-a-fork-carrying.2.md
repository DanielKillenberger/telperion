---
satisfies: [R2, R3, R4]
---
# fn-311-wgpu-comes-from-a-fork-carrying.2 The workspace takes every wgpu-family crate from the fork; the vendored copy goes; programs.rs on the PR's signature

## Description
Switch the workspace from the vendored copy to the fork at the rev task .1 recorded (`.flow/evidence/fn-311-wgpu-comes-from-a-fork-carrying/FORK.md`), move the one call site to the PR's signature, add the one-of-each check, and run the spec's R4 proof. One task because the patch switch, the call site and the lock do not build apart.

**Size:** M
**Files:** `Cargo.toml`, `Cargo.lock`, `vendor/**` (deleted), `docs/wgpu-fork.md` (new), `crates/telperion-render/src/programs.rs`, `crates/telperion-render/src/programs/tests.rs`, `scripts/wgpu-family.test.mjs` (new), `scripts/ci-key.mjs`
**Touches:** [Cargo.toml, Cargo.lock, vendor/**, docs/wgpu-fork.md, crates/telperion-render/src/programs.rs, crates/telperion-render/src/programs/tests.rs, scripts/wgpu-family.test.mjs, scripts/ci-key.mjs, .flow/evidence/fn-311-wgpu-comes-from-a-fork-carrying/**]

### Approach
- `Cargo.toml:9-17`: drop `exclude = ["vendor"]` and its comment; `[patch.crates-io]` lists wgpu, wgpu-core, wgpu-hal, wgpu-types, naga, wgpu-naga-bridge as `{ git = "https://github.com/DanielKillenberger/wgpu", rev = "<full sha>" }`, with a one-line comment pointing at `docs/wgpu-fork.md`. Then `cargo update -p wgpu` (or the minimal update that rewrites only the family entries) and read the lock diff: every family entry, including `naga-types` and `wgpu-core-deps-*`, must read `source = "git+https://github.com/DanielKillenberger/wgpu?rev=<sha>#<sha>"`. Add a patch entry for any family crate still from the registry. No other lock entry may change version; report any that does.
- `git rm -r vendor` (the dcg hook blocks `rm -rf`).
- `docs/wgpu-fork.md`: short, R2's content (fork, branch, rev, base v30.0.1 `40f4a34eb`, PR #10438 head `9d3215c1`, what the branch carries and that native errors stay on the error scope, drop when a wgpu release carries #10438, and how to re-pin: push to the branch, update the rev, rerun the check). Carry over `vendor/README.md`'s "Why" paragraph and its fn-243 R1 citation.
- `programs.rs:1-8` doc comment: point at `docs/wgpu-fork.md`. `programs.rs:85-95`: `gpu.device.create_compute_pipeline_async(desc)` now returns the future directly; drop the `ok_or_else`; map `wgpu::Error` to the old reason string: `Validation { description, .. }` -> `"validation: {description}"`, `Internal { description, .. }` -> `"internal: {description}"`, `OutOfMemory` -> its `Display`. Keep it a small private fn with a native unit test in `programs/tests.rs` (wgpu::Error is constructible natively); the `Pending` type and `settle` stay as they are. Native code (`programs.rs:62-80`) is untouched.
- `scripts/wgpu-family.test.mjs` (vitest picks up `scripts/**/*.test.mjs`): parse `Cargo.lock`; for each package named `wgpu`, `wgpu-*`, `naga`, `naga-*`, assert exactly one entry and that its source is the fork at the rev in root `Cargo.toml`'s patch table; assert the six R2 names are present. Add it to `scripts/ci-key.mjs` `node.inputs` beside `scripts/ci-key.test.mjs`. Prove it can fail: run it once against a lock with one family entry edited back to the registry (a temp copy, never the real lock) and note the failure text in the evidence.
- Evidence (`.flow/evidence/fn-311-wgpu-comes-from-a-fork-carrying/PROOF.md`): `cargo tree -p telperion-render -d` natively and with `--target wasm32-unknown-unknown` (no family crate listed), lines removed (`git diff --stat` against the base), Wasm size against `scripts/artifact-budgets.json`, gate and npm results.

### Proof order (one at a time; `ps -eo pid,args | grep target/ci/deps` first)
1. `~/.local/bin/telperion-heavy 5 --threads 2 -- cargo check --profile ci -p telperion-render` and the same with `--target wasm32-unknown-unknown`.
2. The two guards: `cargo test --profile ci -p telperion-core --test generation_limit_guard` and `cargo test --profile ci -p telperion-render --lib compile_size`, inside telperion-heavy.
3. `~/.local/bin/telperion-heavy 5 --gpu 3 --threads 2 -- npm run render:build`, then the size of `src/browser/telperion-render.wasm` against the `dist/telperion-render.wasm` row (ceiling 3,700,000) and against the base's size measured the same way (the 5% PR limit); `npm run build` + `node scripts/artifact-budgets.mjs` if it runs without network.
4. fn-243's tests: `programs::tests` (in the gate), `src/browser/render.test.ts` (in npm test), and `npm run test:render` once inside `telperion-heavy 5 --gpu 3 --threads 2`.
5. `npm ci`, then `npm test` on its own, inside telperion-heavy.
6. The gate last, once: `~/.local/bin/telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`, output saved under `.flow/evidence/fn-311-wgpu-comes-from-a-fork-carrying/raw/`. If it waits over 15 minutes for room, report that.

### Investigation targets
**Required:**
- `Cargo.toml:9-17` — exclude and patch table
- `crates/telperion-render/src/programs.rs:60-125` — call site, `Pending`, `settle`
- `crates/telperion-render/src/programs/tests.rs` — settlement test pattern
- `scripts/ci-key.mjs:47-60` — node inputs
- `scripts/artifact-budgets.json` — the render row
**Optional:**
- `vendor/README.md` — the text the docs section replaces
- `scripts/artifact-budgets.test.mjs` — a small `.test.mjs` to model on

### Key context
- Path deps inside a git dependency resolve inside that repo, so the fork's wgpu pulls the fork's wgpu-core, wgpu-hal, wgpu-types, naga, naga-types and deps crates; the dev-deps `naga = "30"` and `wgpu-naga-bridge = "30"` are what would drag in a second registry copy without the patch.
- The PR's future is `WasmNotSend + 'static`, which on wasm32 adds no `Send` bound; `Pending` boxes a non-`Send` future already.
- Friction goes into `.flow/evidence/fn-311-wgpu-comes-from-a-fork-carrying/FRICTION.md` as it happens.

## Acceptance
- [ ] `[patch.crates-io]` takes the six R2 crates from the fork at the full rev; `exclude = ["vendor"]` and `vendor/` are gone; `docs/wgpu-fork.md` replaces `vendor/README.md` (R2).
- [ ] `scripts/wgpu-family.test.mjs` passes, fails on a lock with a registry family entry, and is in `ci-key.mjs`'s node inputs (R2).
- [ ] `cargo tree -d` lists no wgpu-family crate natively or for wasm32 (R4).
- [ ] `programs.rs` calls the PR's `create_compute_pipeline_async`; a refused program's reason keeps the `<reason>: <message>` shape, covered by a unit test; the native path is unchanged (R3).
- [ ] Wasm render module built and its size stated against the budget and the base; fn-243's tests pass; `npm test` and the gate green (R4).

## Done summary
The workspace takes wgpu, wgpu-core, wgpu-hal, wgpu-types, naga and wgpu-naga-bridge from DanielKillenberger/wgpu at 165ebd16f930baefecca51c22e753a48f911a5fa. All ten family lock entries now come from the fork, and no other lock entry changed. `vendor/` and the workspace exclude are gone, and `docs/wgpu-fork.md` replaces `vendor/README.md`. `programs.rs` awaits the PR's `create_compute_pipeline_async` through a private `rejection` that keeps the `validation: …` / `internal: …` wording; `a_refused_compile_keeps_the_reason_and_message_shape` tests it. The native path is unchanged. `scripts/wgpu-family.test.mjs` checks for one of each family crate from the fork at the pinned rev. It covers the crates.io, other-rev and twice cases, fails on a temp lock with wgpu-hal set back to the registry, and is a node input in `ci-key.mjs`.

Proof (`.flow/evidence/fn-311-wgpu-comes-from-a-fork-carrying/PROOF.md`): `cargo tree -d` lists no family crate natively or for wasm32. The Wasm render module is 3,506,337 bytes against 3,504,023 on the base, +0.066% (ceiling 3,700,000). Against the spec base 0a62c7ec2: 206 files changed, 512 insertions, 42,641 deletions. The gate passed (1501 of 1501, 136 skipped), and `npm test` passed (181 of 181). Two failures predate this task. `npm run test:render` fails "Ordinary generated no wood" here and on an archive of the base. `artifact-budgets.mjs` fails on a missing `dist/wasm-source.js`; that one is inferred, not measured on the base. fn-243 R4's cold-profile Chrome run stays open for the owner.

Baseline: `npm run render:build` on the base was green. FRICTION.md has three new entries: vitest globalSetup, test:render needing a base build, and the gate outlasting the 600 s foreground limit.

Tier: not supplied by the conductor (model: claude-opus-5-5)

stage: impl-review - ran (codex, three draws correctness/contracts/integration, all SHIP, rid c9417caad85545c395310504b4f5e1e2)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: da4b8d9141e30d2639f53d5bee92f3e8ab5459ef, 7f22c3f2bcbac9dc2c5ae4da81c942b7b59b311b
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py, npm test, npx vitest run scripts/wgpu-family.test.mjs, cargo test --profile ci -p telperion-render --lib programs::tests
- PRs: