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
- **Step 2 (host, 14 Nov)**, replacing the old R3 and R4. The count goes; the machine answers. Step 1 (CEILING.md) found both traps to be `nodes` in the conversion doubling 384 to 768 MiB with 1.8 to 2.2 GiB of the 4 GiB not live.
  - **A. The conversion sizes its stores once.** `unfolded`'s per-node vectors (`nodes`, `frames`, `lengths`, `bearing`, `keys`, the 16-byte one, any other filled a node at a time) are reserved exactly at the bound the code has in hand (kept phytomers + 1) before the first push, with `try_reserve_exact`; a failed reserve is the refusal by name, an `Err` saying which store, the bytes it needed and that the machine did not give them, on every target (no `cfg` branch). `fitted`'s shrink stays only where the bound overshoots what was pushed. The tree is equal to the bit: the preset digest pin and `every_preset_grows_the_same_tree_again_and_on_any_cores` by name. The same for any other store after growth whose size is known before it fills (foliage, flowers, the curve, the handout read; say which are and which are not; reserve the ones that are).
  - **B. A failure the code cannot return is still named.** The shipped binding wraps the allocator, keeping only what a refusal needs: on a failed request the size asked, the live bytes and the stage tag (the tag setter ships; one static store per stage boundary). The package's loader catches the trap around `build` and the growth calls, reads that record through exports that touch no engine state, throws the package's refusal error by name with the amount asked and the stage, and replaces the instance (`TreeEngine`, `initializeTreeCore`, the field entry's cached promise each hand out a fresh instance after a trap); a trap with no allocation record is rethrown as it is. Package tests: a request that cannot fit is refused by name, and the next ordinary request on the same engine builds.
  - **C.** `WASM_KEPT`, `KEPT`, `fits` and the ceiling use in `foreseen` are deleted with their three readers' calls and `a_tree_over_the_wasm_ceiling_is_refused_by_name`; `foreseen` goes if nothing else reads it. The wood's fn-291 refusal (`memory::left()` counts freed bytes as available) is replaced by the same fallible reserve on the wood's stores if their sizes are exact there, else left with the reason. Comments and documents that state 4,000,000 follow.
  - **D.** `every_shipped_preset_keeps_under_the_wasm_ceiling` becomes a package-side check in `scripts/test-dist.mjs` (or beside it): every shipped preset at its own seed and age builds in Wasm under node in a fresh instance, its peak printed. The refused-wood case becomes: the spruce at seed 42 at the smallest age refused after A plus two years is refused by name, memory at or under 4 GiB, and the next request builds. The 3.5 GiB bar on the presets stays; each preset's peak is reported against it after A; if the Two Trees are still over, the bar stays red and the task returns with the numbers and what holds the bytes at their peak.
  - **E.** Measured and reported: step 1's table again after A (peak, failed request, largest age that builds for the spruce and the beech), the Two Trees on seeds 42, 1 and 7, and one reuse case: Telperion at 82 on seed 42 after the oak at 40 in the same instance, and twice in a row in one instance.
  - **Step 2's Stops:** a digest that moves; a store whose exact size needs a rule changed; the loader unable to replace an instance without a change to the package's public surface.
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
