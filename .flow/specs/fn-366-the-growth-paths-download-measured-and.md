# The growth path's download, measured lever by lever and brought down

Child of fn-351 (P11; Phase 6). Host-written, 14 Nov 2026. Builds on `fn-364-a-tip-extends-by-what-its-balance-leaves`. Survey: `.flow/evidence/fn-351-020-the-release-candidate/PHASE6-SURVEY.md`, section 4.

## Problem

P11 (owner, 1 Nov): the website's growth path downloads "At most 250 KB" (Brotli; fn-351: 250,000 bytes). Today a field page fetches about 403,000 Brotli bytes, 388,544 of them `telperion.wasm` (survey 4.3). Nothing checks Brotli bytes. Only one lever is measured: removing the custom sections brings the module to 367,708.

Host's reading of "the growth path", recorded as such until the owner says otherwise: every file a field page fetches to grow a tree from parameters it already holds (`telperion.wasm`, `field.js`, `voxelize.js`, the shared chunks they import), without `presets.js` and without `telperion-render.wasm`.

## The design (host)

Measure, then cut in the order of bytes won per risk. One pipeline stays one pipeline: a smaller module is the same build with outputs not compiled in, never a second chain (fn-150's lesson).

1. **A measuring script first** (`scripts/growth-path.mjs`, typed as the other scripts are): builds nothing, reads `dist/`, follows the field entry's static imports, and prints raw and Brotli (quality 11) bytes per file and in total. It becomes a line of `scripts/artifact-budgets.mjs` that reports the total; it fails on a ceiling only once R4 sets one.
2. **Levers, each measured alone on the same commit** (Brotli bytes won, and the oak at 40's build time in Wasm before and after, three runs each with no watcher PAUSE inside), in `LEVERS.md`:
   a. custom sections stripped in `scripts/build-wasm.mjs` (names included), with no tool the machine lacks;
   b. the `preset(id)` and `catalogue()` exports out of the shipped module: `presets.generated.ts` written at build time by a native run of the same catalogue code, the check that the file equals what Rust renders kept;
   c. `rayon` out of the one-thread Wasm build behind a feature;
   d. formatting and `serde_json::Value` off the request path, if the reading shows a narrow way;
   e. the field page not fetching the renderer's glue (`module.js` split so `field.js` imports only what it runs);
   f. a size-minded profile for the Wasm crate only (`opt-level = "s"` and `"z"` tried against today's), kept only if the oak's Wasm time moves by under 3%;
   g. what a field request never runs (leaf placement, the surface sweep, chunks, bloom): measured only, as the bytes a `geometry`-off build of the binding would save, with what stands in the way ("the field needs the drawn tree"). Not built here.
   `wasm-opt` is not installed and nothing is installed for this spec; note it as unmeasured.
3. **Apply** every lever of a to e that wins bytes and breaks no test, and f only under its speed rule, each as its own commit.

Expected on paper from the survey's table: a 24,000; c about 4,000 to 6,000 (17 KB raw); b unknown (part of 214 KB of `.rodata` and 58 KB of `catalogue`); e 12,393. The floor after a to e is therefore expected well above 250,000. That is a finding, not a failure of this spec.

## Acceptance Criteria

- **R1.** The reading first, in the evidence: what `.rodata` holds by symbol or by string class (preset tables, catalogue prose, parameter labels, format strings); every caller of `preset`, `catalogue`, `rayon` and `serde_json` on the Wasm build path; what the field entry imports.
- **R2.** `scripts/growth-path.mjs` with a test beside the existing `artifact-budgets.test.mjs`, and its line in the budget script's output.
- **R3.** `LEVERS.md`: one row per lever, alone and cumulative, bytes and seconds, measured as above.
- **R4.** The levers applied; `npm run build`, `node scripts/artifact-budgets.mjs` and `scripts/test-dist.mjs` green; the raw-byte ceilings in `scripts/artifact-budgets.json` lowered to the new sizes plus the file's usual margin (they only go down here). If the total reaches 250,000, the Brotli ceiling is added at 250,000 with the owner's ruling as its reason. If it does not, return with the measured floor and lever g's number: holding or moving P11 is the owner's.
- **R5.** `presets.generated.ts` byte-equal before and after lever b; `every_shipped_preset_builds_through_the_main_entry` and the package's "refuses a preset's id" test pass.
- **R6.** Narrow tests while building; the two sub-second guards before finishing.

## Stops

- A lever that changes a tree (a digest moves): return with it.
- A lever that needs a second build chain or a new published module.
- Anything that needs a tool installed.

## Boundaries

The build scripts, the binding crate's exports and features, the package's chunking, the budget file's ceilings downward. No change to the grower, the pipeline or any preset.

## Rigor

Opus builds. Heavy runs (cargo, the Wasm release build, npm build) through `telperion-heavy` with `TELPERION_HEAVY_RESERVE=8` in the background, checked by the clock; the yield watcher may freeze them, and seconds count only with no PAUSE inside (`scratchpad/yield.log`). Friction in this spec's FRICTION.md as it happens.
