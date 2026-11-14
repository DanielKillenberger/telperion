# fn-351 L2 (part) and L3: the candidate's measured release checks

Measured on 14 Nov 2026, 09:04 to 16:00 Zurich, by an Opus executor for the host. The candidate is `fn-351-candidate` at `a7ae69477`, checked out in its own worktree (`.worktrees/relcheck`, branch `fn-351-release-checks`) with its own `target/` and `node_modules`. The desk is the owner's RTX 3080 (driver 615.78.08, Vulkan) with 62 GiB RAM. Other sessions were running throughout: a codex review, the film probe and cut, and the host's `gate.py --full`. The load average is given beside each timing. Raw output is in the ignored `raw/relcheck/`.

This file records measurements against the bars as written. Whether a bar still applies to the candidate is the host's call, and so is any verdict. One such case is R0a's harness budget, which was set for the GPU grower that P5 deleted.

## The table

| Check | Command | Bar, with its source | Measured | Result |
|---|---|---|---|---|
| Format | `cargo fmt --all --check` | L2: "`cargo fmt --all --check` clean" | 9 hunks, all in one file: `crates/telperion-render/examples/bark_distance_frames.rs` (lines 27, 35, 57, 94, 134, 148, 188, 198, 257) | **fail** |
| Artifact sizes | `npm run build`, then `node scripts/artifact-budgets.mjs` | L2: "shipped artifacts inside `scripts/artifact-budgets.json`" | all 7 inside their ceilings; exit 0 (sizes below) | pass |
| npm test | `telperion-heavy 8 --threads 4 -- npm test` | L2: "`npm test` green" | catalogue check passed; 27 files, 221 of 221 tests; 262.7 s at load 7.6 | pass |
| Harness, warm build | `__GL_SHADER_DISK_CACHE=0 SEED=1 node scripts/gpu-row.mjs <rows> -- target/ci/examples/frames <preset> full whole`, `warm_ms` | fn-237 R0a: "every shipped tree ready in under 0.5 s warm in the harness on the owner's RTX 3080; Telperion and Laurelin named exceptions at about 1.25 s warm" | palm 24 ms, ordinary 213 ms, birch 529 to 596 ms, beech 627 to 634 ms, oak 730 to 742 ms, spruce 6,328 to 6,447 ms, Laurelin 10,625 to 11,056 ms, Telperion 11,723 to 12,347 ms | **fail** for 6 of 8 (birch, beech, oak, spruce, Laurelin, Telperion); pass for palm and ordinary |
| Harness, cold start, shader cache off | same rows, `fresh_ms` | fn-237 R0b: a start-time figure "states its shader-cache state, and the release checks measure it cold with `__GL_SHADER_DISK_CACHE=0`". No number is set. | 2.5 s (palm) to 14.5 s (Telperion); table below | measured; no bar to pass or fail |
| Harness against master | none | fn-198 task 9: "every catalogue species's build time in the harness against master" | master (`origin/master` `74410f7ad`) has no `frames` probe. fn-259 measured against R0a's absolute budget instead. | **unknown**: no command against master exists |
| Per-tree field cost against 0.1.4, through the package entry | candidate: fn-352's `scoreboard/wasm.mjs` with the `family.fidelity` line removed, run as `taskset -c 10 node wasm-nofid.mjs dist 0`. 0.1.4: `scoreboard/wasm014.mjs` run on the published 0.1.4 package | fn-237 R0a: "the site's per-tree field ... at or under 0.1.4's per-tree time"; fn-237: "its per-tree bar, 0.1.4's, is measured through the package entry"; fn-351 P1 sets an aim of half 0.1.4's time and says it is "never a stop" | 2.7× to 325× 0.1.4's time on every species 0.1.4 has (table below) | **fail** on every preset and seed |
| npm version | `package.json`, `CHANGELOG.md`, `Cargo.toml`, `git show v0.1.4:package.json` | fn-198 task 9: "the npm version and changelog per the repo's release convention" | `package.json` 0.2.0; CHANGELOG's head is "0.2.0 (release candidate, not yet released)"; v0.1.4's `package.json` is 0.1.4; every crate is 0.1.0, as at v0.1.4 | consistent; the version was not changed |
| Package contents | `npm pack --dry-run --json` | none written; `files: ["dist"]` | `telperion@0.2.0`, 25 files, 1,229,736 bytes packed, 3,793,519 unpacked; every `exports` target present | pass on existence; no bar |
| Docs | none | fn-198 task 9: "docs/pipeline.md and docs/tree-space.md current" | not judged here; L4 owns it | not run |
| Tint on every shipped module | `TINT=<Dawn's bin/tint> cargo test --profile ci -p telperion-render every_shader_passes_tint -- --ignored` | `docs/gpu-tests.md`: a release check, "run before a release PR" (fn-237 R14) | no `tint` binary on this desk; fetching one would install a tool | not run |
| Cold pipeline probe | `every_pipeline_within_2_5_s_cold` | `docs/gpu-tests.md` (fn-237 R9) | the test no longer exists on the candidate; `docs/gpu-tests.md` still names it | not run (no command) |
| The site's field against 0.1.4 | none | fn-198 task 9 | fn-237 moved it out of this release (killenberger.com's work) | not run (out of scope) |
| The harness in a browser (WebGPU) | `npm run dev` | | agents do not drive WebGPU | not run: needs a browser |
| The field suite on WebGPU | `npm run rust:test:wasm` | `docs/gpu-tests.md`: "only in a browser that has WebGPU" | | not run: needs a browser |

## Details

### Format

`cargo fmt --all --check` exits 1. All nine hunks are in `crates/telperion-render/examples/bark_distance_frames.rs`. FRICTION.md (14 Nov 07:00) calls this file a scratch example from the bark-distance work, and it is committed on the candidate. Nothing was applied. The full diff is in `raw/relcheck/fmt.log`.

### Artifact sizes (`npm run build` at `a7ae69477`)

| Artifact | Bytes | Ceiling |
|---|--:|--:|
| `dist/telperion.wasm` | 1,224,400 | 1,338,000 |
| `dist/telperion-render.wasm` | 2,178,805 | 3,700,000 |
| `dist/telperion.js` | 4,021 | 200,000 |
| `dist/presets.js` | 189,375 | 200,000 |
| `dist/field.js` | 1,269 | 16,000 |
| `dist/module.js` | 59,707 | 63,000 |
| `dist/voxelize.js` | 4,573 | 16,000 |

- The per-PR growth share was not checked: "not a pull request run ... the ceilings alone were enforced".
- The script also prints the growth path: **351,568 Brotli bytes** (field.js, core.js, voxelize.js, telperion.wasm). That is above P11's 250,000. P11 is not an L2 check; it is recorded here only.
- The sizes were the same after `npm test`'s `pretest` rebuilt the modules.
- `npm ci` skipped esbuild's postinstall script (npm's install-scripts warning). The build succeeded anyway.

### The `frames` rows (seed 1, cache off, view `whole`)

`frames` is built with `cargo build --profile ci -p telperion-render --example frames`. `fresh_ms` runs from `Renderer::new` to a fenced first frame, shader compilation included. `warm_ms` is a second build in the same process. Since fn-353 that is the CPU grower, `mesh::build` and the submission.

The script is `scratchpad/relcheck/frames-rows.sh`, one process a row, inside `telperion-heavy 6 --gpu 2 --threads 4`. Every row is valid except Laurelin's third. A foreign process came onto the card during it: GNO's model worker, 1,151 MiB. That row's `warm_ms` of 367,465 ms is left out below.

| Preset | Age | `fresh_ms` cold (3 runs) | `warm_ms` | Frame, whole (ms) | Peak RSS MB | Load (1 min) |
|---|--:|---|---|---|--:|---|
| date-palm | 40 | 2,550 / 2,537 / 2,509 | 24 / 24 / 24 | 0.72 | 420 | 2.8 / 2.7 / 2.1 |
| european-beech | 40 | 3,240 / 3,205 / 3,192 | 625 / 634 / 627 | 2.3 to 3.1 | 696 | 2.1 / 2.0 / 1.9 |
| oregon-white-oak | 40 | 3,293 / 3,284 / 3,360 | 730 / 742 / 741 | 1.5 to 2.6 | 665 to 712 | 1.8 |
| ordinary | 40 | 3,200 / 3,231 / 3,181 | 216 / 213 / 213 | 0.60 | 470 | 1.7 to 1.8 |
| silver-birch | 40 | 3,459 / 3,198 / 3,273 | 596 / 529 / 581 | 1.7 to 2.6 | 607 to 625 | 1.9 to 2.0 |
| norway-spruce | 40 | 9,126 / 8,987 / 8,952 | 6,447 / 6,331 / 6,328 | 7.4 | 1,583 to 1,612 | 1.7 to 1.9 |
| laurelin | 80 | 13,455 / 13,503 / (13,278 invalid) | 10,625 / 11,056 | 6.5 | 3,150 to 3,257 | 3.0 / 3.1 |
| telperion | 82 | 14,371 / 14,504 / 14,491 | 11,723 / 11,823 / 12,347 | 5.8 to 6.0 | 3,403 to 3,440 | 2.2 / 6.8 / 3.7 |

- The cold start without the build is roughly `fresh_ms` − `warm_ms`. On the first run of each preset it is 2.5 to 3.0 s: palm 2.53, oak 2.56, beech 2.62, Telperion 2.65, spruce 2.68, Laurelin 2.83, birch 2.86 and ordinary 2.98 s. This is a difference read from the table, not measured separately.
- Only seed 1 was run. fn-259 also took seeds 7 and 42; those were not run.
- The load column is the 1-minute load average at the start of each row (`frames-cold.log`).

### Per-tree time in Wasm through the package entry, against 0.1.4

Both scripts run in Node on one pinned core (`taskset -c 10`, `telperion-heavy … --threads 1`). Each tree gets one cold build, then five warm builds, and the table gives the median of the warm five.

- **The candidate:** `engine.build(family, { field: { n: 36 } })` through `dist/telperion.js` at `a7ae69477`. The script is fn-352's `scoreboard/wasm.mjs` with one line removed, `family.fidelity = fidelity;`, because fn-367 retired `/fidelity` and it is refused by name. The copy is `scratchpad/relcheck/wasm-nofid.mjs`. It ran 09:20 to 09:34, at load 6.7 to 8.1.
- **0.1.4:** `scoreboard/wasm014.mjs`, unchanged, on the published package (`npm pack telperion@0.1.4`, unpacked on 11 Nov in the session scratchpad, `package.json` version 0.1.4). It uses `build(family, { field: true })` at 0.1.4's own field and preset ages (0.1.4 reports `age` 100, and 70 for the birch). It ran 09:34 to 09:35, at load 8.1 to 8.2.

| Preset | Seed | Candidate ms | 0.1.4 ms | Ratio |
|---|--:|--:|--:|--:|
| ordinary | 1 / 1234 / 5678 | 357 / 436 / 355 | 66 / 55 / 61 | 5.4 / 7.9 / 5.8 |
| oregon-white-oak | 1 / 1234 / 5678 | 1,444 / 2,615 / 1,801 | 147 / 154 / 202 | 9.8 / 16.9 / 8.9 |
| norway-spruce | 1 / 1234 / 5678 | 6,162 / 6,710 / 5,955 | 134 / 131 / 139 | 46 / 51 / 43 |
| silver-birch | 1 / 1234 / 5678 | 970 / 1,208 / 683 | 174 / 239 / 249 | 5.6 / 5.0 / 2.7 |
| date-palm | 1 / 1234 / 5678 | 19.2 / 17.2 / 17.5 | 5.1 / 5.2 / 5.0 | 3.8 / 3.3 / 3.5 |
| telperion | 1 / 1234 / 5678 | 23,137 / 18,208 / 16,457 | 470 / 525 / 154 | 49 / 35 / 107 |
| laurelin | 1 / 1234 / 5678 | 21,366 / 17,686 / 15,235 | 128 / 54 / 56 | 167 / 325 / 272 |
| european-beech | 1 / 1234 / 5678 | 947 / 691 / 942 | (none in 0.1.4) | |

The scoreboard's base run of 11 Nov (`fn-352/.../base-2026-11-11/wasm*.jsonl`) agrees in kind. At seed 1, 0.1.4 measured 58 to 447 ms then and 66 to 470 ms here; the candidate's oak was 1,134 ms on 11 Nov and 1,444 ms here.

### Version and package

- `package.json` is 0.2.0. `CHANGELOG.md` begins "## 0.2.0 (release candidate, not yet released)". Every crate in the workspace is 0.1.0, as at `v0.1.4`.
- The local `master` ref is stale (`ebd2e37b1`); `origin/master` is `74410f7ad`.
- `CHANGELOG.md` is not in the packed tarball, because `files` is `["dist"]`. Whether it should ship there is not judged here.
- `npm pack --dry-run` lists LICENSE, README.md, package.json and 22 files under `dist/`, the type declarations included.

## Not run, for the owner or the host

- **The harness in a browser on WebGPU:** `npm run dev`, then build each preset in Chrome. It needs a browser.
- **The field suite on WebGPU:** `npm run rust:test:wasm`. It needs a browser with WebGPU, and no CI job runs it.
- **Tint:** `gh release download -R google/dawn -p '*ubuntu-latest-Release.tar.gz'`, unpack it, then `TINT=<dir>/bin/tint cargo test --profile ci -p telperion-render every_shader_passes_tint -- --ignored`. It needs a fetched binary but no device.
- **The harness against master:** there is no command. `origin/master` has no `frames` probe.
- **`every_pipeline_within_2_5_s_cold`:** it was removed from the code, but `docs/gpu-tests.md` still lists it.
- **The full gate:** run by the host and not run here.
