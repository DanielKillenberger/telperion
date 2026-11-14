# fn-353-one-grower-the-gpu-grower-is-deleted.1 CPU growth bound for the browser; the harness and the field entry move to it (R1 to R3)

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Task 1 moved the page's growth and `telperion/field` onto the CPU. The grower now owns its species, so a page can keep a growth between calls (option A, `R1-ANSWERS.md` §5 and §6). R1 is held as the host restated it: a year drawn twice is the same tree, and at the horizon the tree equals the one asked for and the one grown straight through. The years before the horizon are fn-351's P4, still open (`R1-EQUALITY.md`).

Tier: product code changes. This task ran only its narrow tests; the full gate and the review come at task 3 (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; the host reviews the spec's diff after task 3)

### Commits

- `2f9e9df9d`: `Grower`, `Paused` and `Settling` hold an `Arc<Species>`. `Rules` drops the species, and `Shoot` borrows it from the grower. `GrowthState` owns an `Arc<Params>`.
- `ed8394bb5`: R1 and R2. The `WebRenderer` methods `beginGrowth`, `growTo` and `drawGrowth` run over `pipeline::growth`, synchronously on the page's thread. A draw is dressed with `growth::mesh`, put up through `Renderer::submit` and coloured with `growth::colouring`; `setTree` resets the colouring to 1.0. The harness builds with `setTree` and grows in the order draw, wait 150 ms, grow the next year. Admission, makeRoom and the GPU-programs note are gone from it.
- `10c5bafb4`: R3. `growField` returns `TreeEngine.build(family, { field })`'s `FieldGrid`. `prepareField`, `FieldError` and the device refusals are removed. `compileField` and the `source` option stay, and `TreeEngine.create` accepts a compiled Module. The migration note gains a section for the 0.2.0-next prereleases.
- Evidence: `BEFORE.md`, `R1-EQUALITY.md` (with its probe patch), `R1-OWNERSHIP.md`, `R1-OWNERSHIP-MEASURED.md`, `R1-ANSWERS.md`, `FRICTION.md`.

### Green (measured)

- `cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core --test-threads 4`, through telperion-heavy, at the ownership commit: 774 passed, 74 skipped. Every digest and repeat test passed unchanged, and no baseline was edited.
- `cargo nextest run --cargo-profile ci -p telperion-core --lib pipeline::space::growth::`: 4 passed, the new `a_tree_grown_a_year_at_a_time_repeats_and_ends_at_the_tree_asked_for` among them.
- `node scripts/build-wasm.mjs && node scripts/build-render.mjs`: both modules built. The binding compiles for `wasm32`.
- `cargo clippy --profile ci --target wasm32-unknown-unknown -p telperion-render -- -A clippy::absurd_extreme_comparisons`: no finding in `src/web*`. The allowed lint is a pre-existing error in `telperion-core/src/ranges.rs:127`, which fires on wasm32 only.
- `npx tsc --noEmit`: clean.
- `npx vitest run src/field src/browser harness`: 18 files, 144 tests passed.
- `npx tsc -p tsconfig.build.json && npx vite build && node scripts/test-dist.mjs`: 24 cases built. `field.js` grew the birch's grid in Node without WebGPU, equal to the main entry's, and refused a side of two by name.
- The still runner's oak at 40, seed 1001: 272,095 nodes and 47,279 leaves, before and after.

### Not verified by me: needs a browser with WebGPU

1. `drawGrowth` drawing a grown year on the canvas: the tree goes up, is framed, and its leaves colour through autumn in "play the year".
2. The harness's "grow in": years appear one at a time, each up for about 150 ms before the next grows, with the year label updating; a dial move during a grow-in abandons it between years.
3. The harness's "play the year": twelve draws through the last year, ending at the whole age, on the held growth without a new begin.
4. A build after "play the year" draws with whole-tree colouring, from the `setColouring(1.0)` reset in `setTree`.
5. The harness's build per dial move on the CPU (decision 8's cost), and the main thread being held while a tree grows.
6. `tests/browser/field.mjs` through `node scripts/test-wasm.mjs`. It needs Chromium and Playwright but no longer needs WebGPU, so it could join the CI binding suite. That is a follow-up, not done here.

### For task 2

- The still runner grows through `executor::grow` and `.expansion()`.
- `src/browser/render.ts` still carries `setTreeGpu`, `admission`, `programs` and `GpuSubmitted`, and `web/field.rs` and `web/generation.rs` remain (R5). Nothing in the harness or `telperion/field` reads them now.
- The core's 32-bit refusal message still says "build it through the device entry" (seen in `test-dist`'s Telperion case).
- `docs/field-package.md`, the README's field section and the migration note's `setTreeGpu`/`admission` line still describe the device (R7).
- The local branch `fn-353-r1-r3-wip` holds the parked copy of R1 to R3, now landed; it can be deleted.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5c15b3b1c7e9ac5247591c41e20c15d9b00322c4, a49060bcc487bbd1953150071035be2cc53c35be, ecfabacd495fecd49a899e603e16d5a90eb8d552, 2f9e9df9d6489c82d885548ca70faa45b172d150, 6f5b99998423dc27966f29b5d6687bc6fd7c7f94, ed8394bb5875441c59e7d541675537d8ab98fcb3, 10c5bafb40083d51d5772c7a84999a9a46375b05
- Tests: cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core --test-threads 4, cargo nextest run --cargo-profile ci -p telperion-core --lib pipeline::space::growth::, node scripts/build-wasm.mjs && node scripts/build-render.mjs, npx tsc --noEmit, npx vitest run src/field src/browser harness, node scripts/test-dist.mjs
- PRs: