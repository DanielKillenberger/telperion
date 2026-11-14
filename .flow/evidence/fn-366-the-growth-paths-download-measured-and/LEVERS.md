# The growth path's levers (fn-366 R3)

Measured 14 Nov 2026 by the task .1 worker. Bytes are Brotli at quality 11 (window 24) from `scripts/growth-path.mjs` and `sizes.cjs`'s identical setting; the `brotli -q 11` CLI gives the same 395,245 on the base module. "Path" is the growth path's total: the field entry, every script it imports statically, and `telperion.wasm`.

Base: `240f66578` (the branch's start), `npm run build`. The field page then fetched six files: `field.js` 587, `core.js` 1,493, `module.js` 12,393, `voxelize.js` 1,703, **`presets.js` 8,562** (imported by `core.js` for its side effects, so on the path although the host's reading leaves it off) and `telperion.wasm` 395,245: **419,983**. The module was 1,556,211 raw; the survey's 388,544 was fn-353's tree.

Time: the oak at 40 (`oregon-white-oak`, 316,270 nodes), one field request at n = 36, through `dist/telperion.js` under Node, each run its own process (`oak-time.mjs`). Every variant below gave the same tree digest, `271ba36519838eee` (structure, topology and field grid), so no lever moved a tree.

## Alone, each on the base

| Lever | Module raw | Module Brotli | Path won | Oak ms (median of 3) | Applied |
|---|--:|--:|--:|--:|---|
| base | 1,556,211 | 395,245 | | 1,722 | |
| a. custom sections stripped (`build-wasm.mjs`, no tool) | 1,318,997 | 371,563 | **23,682** | 2,206 | yes, `5321d1b85` |
| b. `preset`/`catalogue` exports out, catalogue written natively | 1,550,015 | 394,059 | 1,186 | 2,036 | no: dropped by the host (below) |
| c. rayon out behind `threads` | 1,496,135 | 382,905 | **12,340** | 1,894 | yes, `055489df0` |
| d. formatting and `serde_json::Value` off the request path | | | not built | | no: no narrow way (READING.md) |
| e. field page without the renderer's glue and the presets | (JS only) | | **20,759** (`module.js` 12,393 and `presets.js` 8,562 off, `core.js` +192, `field.js` +4) | unchanged module | yes, `ef7acb165` |
| f. `opt-level = "s"` | 1,728,608 | 377,719 | 17,526 | 2,195 | no: slower (below) |
| f. `opt-level = "z"` | 1,877,393 | 376,650 | 18,595 | 2,712 | no: 57% slower |
| g. `geometry` off (measured only) | 1,296,617 | 319,360 | 75,885 | not run | no: not built here |

The three-run medians of a, b, c and f(s) sit inside the desk's noise: they ran 03:15:58 to 03:17:10 with no watcher PAUSE, but beside other agents' heavy jobs, and lever a, which cannot change what runs, reads 28% slower than the base. They show no tree moved; they do not resolve a few percent. Only f was judged on time, and on an interleaved run (below).

`wasm-opt` is not installed and was not measured. `twiggy` was already on the desk and was used for the reading.

## Cumulative

| State | Module raw | Module Brotli | Path | Oak ms |
|---|--:|--:|--:|--:|
| base | 1,556,211 | 395,245 | 419,983 | 1,722 |
| + a | 1,318,997 | 371,563 | 396,301 | |
| + a, c | 1,273,894 | 360,891 | 385,629 | |
| **+ a, c, e (applied, `dist` at `ef7acb165`)** | **1,273,894** | **360,891** | **364,870** | 1,833 (interleaved median of 7) |
| + a, c, e, f(s) | 1,149,432 | 312,878 | 316,857 | 2,193 (interleaved, +19.6%) |
| + a, c, e, g (stub, cannot serve the field) | 1,034,011 | 287,144 | 291,123 | not run |
| + a, c, e, f(s), g | | | unmeasured | |
| + b | | | unmeasured cumulatively; alone 1,186, and 1,777 on top of c unstripped | |

## The floor, for the owner's page

- **364,870 Brotli bytes applied** (levers a, c and e), 114,870 over P11's 250,000.
- **291,123 the lowest measured**, still 41,123 over. It needs a field that does not read the drawn tree (lever g; Phase 6's grove read from the tree's cells) and it is without the size profile, which slows the oak about 20%.
- `wasm-opt` is unmeasured: it is not installed, and nothing was installed for this spec.

f(s) and its speed rule: stripped, the size profile wins 48,013 Brotli bytes on top of a and c (raw falls too, 1,273,894 to 1,149,432; unstripped it grows, because the name section grows with it). The oak was timed in seven interleaved pairs, 03:38:02 to 03:38:58, no PAUSE: a, c at 1,637 to 2,018 ms (median 1,833), with f(s) at 2,069 to 2,513 (median 2,193). Every pair was slower with f(s), by 13 to 30%. The rule is under 3%, so f is not kept.

g, measured only: a scratch copy of the base with the binding on `telperion-core` without `geometry`, and the four lines that read the field's grid and the flowers removed so it compiles (the field is geometry-gated: "the field needs the drawn tree"). It saves 75,885 alone; it is an upper bound for a field-serving module, since this stub serves no field at all. What stands in the way is the field's dependence on the drawn wood and leaf plan, as the spec says.

## Decisions and what is open

- **Lever b, measured and not applied (host, 14 Nov 2026: 1,186 bytes do not pay for a second way to write the presets).** Its finding: a native run does not render `presets.generated.ts` byte-equal. `canopy.maxInstances` is a `usize` defaulting to `usize::MAX`, so a 64-bit run writes 18446744073709552000 where the Wasm run writes 4294967295, in eight presets.
- **Ceilings.** `telperion.wasm` 1,600,000 to 1,338,000 and `module.js` 70,000 to 63,000: each the measured size and the file's 5% growth share. The two budget tests that replay fn-170's 1,410,330- and 1,500,000-byte modules read the rows they were written against, frozen at `240f66578` in `scripts/artifact-budgets.fn170-frozen.json` beside the test; no assertion changed (host's ruling).
- **`scripts/test-dist.mjs`'s refused case is red on the base.** It builds telperion at seed 7 with `surface.radialSegments` 64 and asserts the build is refused by name ("the wood needs N bytes and M are left in 32-bit Wasm memory") with memory under 3.5 GiB: fn-291 R2f's proof that a wood 32-bit Wasm cannot hold is refused before it is allocated. In fn-291 (16 Oct) that tree had 3,358,968 nodes and its wood alone needed about 3.5 to 3.8 GiB at 20 sides a ring, so 64 sides crossed the 4 GiB address space. Today (`dist` at `e0d8306d6`) it has 856,221 nodes. At 64 sides it builds 12,382,299 vertices and 73,482,510 indices, about 329 MiB of wood (three f32 and two i16 a vertex, u16 indices), and the module's memory peaks at 911 MiB. At 12 sides it is 293 MiB, also peaking at 911 MiB. Two things moved: the grower makes a tree a quarter the size, and since fn-362 `radialSegments` is the most sides a run may have, not every ring's count (`surface/build.rs`, `sides`: the fewest that keep the polygon within `FINEST` of the circle). So 64 raises the wood by 12%, not 3.2 times. The case no longer reaches the limit it was written to cross, and is unchanged here (`raw`: the worker's `refused-probe.mjs`).
- `tests/browser/bindings.mjs` was not run (agents do not drive a browser); with lever b dropped it is unchanged.
