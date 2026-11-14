# The browser's ceiling is measured again on today's records

Child of fn-351 (P1, the browser grows every shipped preset). Host-written, 14 Nov 2026. Builds on `fn-364-a-tip-extends-by-what-its-balance-leaves` at `fd303b473` (R8 in).

## Problem

`WASM_KEPT` = 4,000,000 kept phytomers (`crates/telperion-core/src/pipeline/space.rs`) is the count past which the pipeline refuses a tree in 32-bit Wasm, "so its CPU preparation fits the 4 GB Wasm memory". Its own comment says how it was got: measured in the harness at seed 42 in fn-198, the spruce at 73 keeping 4.27 M and building, at 75 keeping 4.48 M and trapping. It is a count standing for bytes, calibrated on records that have since shrunk: a retired root keeps 24 bytes (fn-357), a growing phytomer 72 bytes, not 136 (fn-358), a twig's ring has only the sides it can show (fn-362: drawn points down 59 to 72%), and the GPU grower is gone (fn-353).

With fn-363 the Two Trees at their preset ages keep 6.1 and 5.4 million at seed 42 and `every_shipped_preset_keeps_under_the_wasm_ceiling` fails; with fn-364 and R8 they keep more on seeds 1 and 7 (5.1 to 8.3 million). Whether they fit in 4 GB today is not known. The test is not loosened; the constant is measured again by its own procedure, or the Two Trees are a refusal the owner hears about.

## The design (host)

The ceiling is the machine's (AGENTS.md, "No artificial limits": a tree stops "on memory a platform really lacks ... 32-bit Wasm's 4 GB"). So the number follows a measurement of that memory, and the measurement is repeatable from the repository.

- **Measure in Wasm under node**, the package's own module and entry (as `scripts/test-dist.mjs` exercises the dist), no browser: for a tree, the Wasm memory's size in bytes after the build (`memory.buffer.byteLength`, which only grows, so it is the high-water mark), the kept phytomers, and whether it built or trapped. With the refusal lifted for the measurement only (a build switch or a test-only export that never ships; say which).
- **Find the trap** as fn-198 did, on the spruce at seed 42 by age, and on the beech and the oak at seed 42 by age, since bytes per kept phytomer differ by species (leaf instances, ring points): the largest age that builds and the first that traps for each, with kept phytomers and bytes at each. Then both of the Two Trees at their preset ages and seed 42, and on seeds 1 and 7.
- **The new bound** is the smallest trapping kept-count over the species measured, less the same share of slack fn-198 left (4.00 against 4.48 M: 0.89), rounded down to the hundred thousand. State bytes per kept phytomer by species beside it. If the species differ by more than a quarter in bytes per kept phytomer, return to the host before pinning: a count is then the wrong unit and the refusal should read bytes.
- A script (`scripts/wasm-ceiling.mjs`) reproduces the table; the constant's comment cites it and the date.

## Acceptance Criteria

- **R1.** The reading first: what `foreseen` and `fits` check and when, what holds memory at the high-water mark in a Wasm build (the grower's stores, the conversion, the curve, the leaf instances, the buffers handed out), and how fn-198's BROWSER.md measured (find it in `.flow/evidence` or git history; if it is gone, say so).
- **R2.** The script and the table in the evidence, as designed.
- **R3.** `WASM_KEPT` set by the rule above with its comment; `a_tree_over_the_wasm_ceiling_is_refused_by_name` updated to the new number by the same assertion; `every_shipped_preset_keeps_under_the_wasm_ceiling` run by name and its output recorded. If a shipped preset is still over the measured bound, the test stays red and the task returns with the numbers: which preset, by how much, what holds the bytes.
- **R4.** The package's browser-side refusal message and any document that states 4,000,000 follow the constant.
- **R5.** Narrow tests while building; the two sub-second guards before finishing.
- **R6.** (host, 14 Nov 2026) `scripts/test-dist.mjs`'s refusal case fails on the base: "telperion seed 7 at 64 sides: built a wood it should have refused". It was fn-291 R2f's proof that a wood past 32-bit Wasm's address space is refused by name before it is allocated, memory staying under 3.5 GiB. The tree it used no longer nears the limit (856,221 nodes today against 3,358,968 then), and since fn-362 `radialSegments` caps a ring's sides, so 64 sides give about 329 MiB of wood. The case keeps what it promised with an input that still crosses the limit: from the table, a request the pipeline must refuse on memory in Wasm (a tree whose wood or preparation needs more than is left), stated as parameters with no change to any preset, asserting the same refusal by name and the same memory bound. If the measurements show no request an honest caller can state crosses the limit before the kept-count refusal fires, return with that instead: the case would then test a refusal that cannot happen.

## Stops

- A species pair more than a quarter apart in bytes per kept phytomer (see the design).
- The measured trap lies under 4.48 M kept (the records did not get cheaper where it counts): return with the split of bytes.
- A preset still over.

## Boundaries

The constant, its tests, its script, its documents. No change to the grower, the pipeline or a preset; no age or seed of a preset is changed to fit.

## Rigor

Opus builds. Heavy runs (the Wasm release build, node runs that take up to 4 GB each) through `telperion-heavy` with `TELPERION_HEAVY_RESERVE=8` in the background, checked by the clock; the yield watcher may freeze them. Friction in this spec's FRICTION.md as it happens.
