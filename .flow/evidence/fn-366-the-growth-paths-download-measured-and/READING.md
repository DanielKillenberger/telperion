# The reading before the levers (fn-366 R1)

Read 14 Nov 2026 by the task .1 worker on the base `240f66578`, before any lever was built. Module figures are the base's unstripped `telperion_wasm.wasm` (1,556,211 raw, 395,245 Brotli). Readers kept beside this file: `rodata.mjs` (string classes), `rodata-map.mjs` (symbols, from a wasm-ld `-Map`), `by-crate.mjs` (twiggy shallow bytes by class).

## What `.rodata` holds (214,632 bytes)

By symbol (`RUSTFLAGS='-C link-arg=-Map=…'`, base build in a scratch target): after fat LTO almost everything is anonymous. 176,667 bytes sit in 7,260 `.Lanon` constants (string literals, panic locations, small tables), 10,357 in the linker's merged strings. The named symbols: `zmij::STATIC_DATA` 17,088 (serde_json's float-to-text table), `serde_json` `POW10`, `HEX0`, `HEX1`, `ESCAPE` 3,752, `core` unicode and `flt2dec` tables about 4,000, `libm` `RSQRT_TAB` 256, `itoa` 200.

By string class (`rodata.mjs`: every string literal of the core, space and binding sources, matched in place, longest first; panic paths found in place):

| Class | Bytes | Share |
|---|--:|--:|
| binary: numeric tables, vtables, float constants (zmij's 17,088 among them) | 151,237 | 70.5% |
| labels, keys and short messages (catalogue labels, wire keys, refusals) | 30,890 | 14.4% |
| other printable runs of 6+ bytes, from no literal of ours (dependencies, derived names) | 13,710 | 6.4% |
| prose: catalogue docs and notes, refusals of 24+ characters | 9,342 | 4.4% |
| panic locations (source paths) | 6,008 | 2.8% |
| parameter paths (`/space/...`, `/canopy/...`) | 3,445 | 1.6% |
| format strings (`{}` placeholders) | 0 | 0% |

The preset tables are mostly binary rows and parameter paths; preset names and notes ("Phoenix dactylifera") are prose. Lever b, which drops every preset table the exports reach, took `.rodata` from 214,632 to 209,152 (5,480 bytes) and the module 6,196 raw: the tables are not what fills `.rodata`.

## Code by class (twiggy, shallow raw bytes)

| Class | Base | After a, b, c |
|---|--:|--:|
| `pipeline::foliage` | 132,890 | 132,890 |
| `pipeline::surface` | 69,855 | 69,855 |
| `catalogue` | 63,078 | 63,078 |
| `closed_form` | 49,494 | 49,494 |
| serde and serde_json | 47,472 | 47,472 |
| formatting (`core::fmt`, float and integer printing) | 46,652 | 42,818 |
| `telperion_space::geometry` | 37,234 | 37,204 |
| presets and `params` | 34,538 | 33,420 |
| rayon and crossbeam | 20,966 | 0 |
| function-names section | 236,914 | stripped by a |

## Callers on the Wasm build path

- **`preset(id)` and `catalogue()`** (exports, `crates/telperion-wasm/src/lib.rs`): `scripts/build-wasm.mjs` calls `catalogue()` once to write `src/browser/presets.generated.ts`; `tests/browser/bindings.mjs` calls both for its identity checks. `TreeEngine` (`src/browser/core.ts`, `Exports`) calls neither; nothing in `src` or `harness` does. Inside Rust, `params::preset` reaches `presets::by_identity`; the request path (`generate` → `params::parse`) does not call it.
- **`rayon`**: one place, `telperion-space/src/cores.rs` (`spread`: `current_num_threads`, `in_place_scope`), reached through `map` and `fill` from `grow.rs` (thread count), `geometry.rs`, `light.rs`, `upward.rs` and `presence.rs`. In Wasm `available()` is one, so every piece already ran on the calling thread. `telperion-core` names rayon only as a dev-dependency (`repeat_tests.rs`).
- **`serde_json`** on the request path: the binding parses the request into a `Value` (`build`), reads the family through `params::parse(&Value)` and `catalogue::Scalar::decode` (`serde_json::from_value` per row), writes the metadata with `json!` and `Value::to_string` (`generate`, `status`), and refusals with `format!`. Off the request path: `params::metadata` (exports only) and `capability::vocabulary`.
- **Formatting** stays for panic messages (`expect` on every target), `Error` display in `status`, and serde_json's number printing (zmij, itoa).

Lever d's reading: no narrow way. The request and metadata are the binding's wire; taking `Value` off it means a second parser for the family beside `params::parse` and a hand-written metadata writer, and formatting stays as long as panics carry messages (removing them needs a nightly `build-std` with `panic_immediate_abort`). Not built.

## What the field entry imports (base)

`dist/field.js` imports `./core.js`, `./module.js` and `./voxelize.js` and names `./telperion.wasm`. `core.js` imports `./module.js` and, for its side effects only, `./presets.js` (from `core.ts` re-exporting the presets, whose module computes its constants at load). `module.js` holds the renderer's wasm-bindgen glue, `wire.ts` and the Wasm loader (`wasm-source.ts`) and names `./telperion-render.wasm`, which is fetched only when a renderer is created. So a field page fetched `presets.js` and the renderer's glue although it runs neither (lever e).
