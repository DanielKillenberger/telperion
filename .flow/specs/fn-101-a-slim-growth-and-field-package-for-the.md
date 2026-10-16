# A slim growth-and-field package for the homepage

## Goal

killenberger.com wants a black-and-white voxel tree, a different species and seed per visit, without shipping the renderer. The page needs to grow a tree, prepare its plan and answer occupancy queries; it draws the cubes itself. The generator Wasm was 1,242,383 bytes raw / 344,254 brotli (master `3fdfb440`, 2026-09-21), carrying surface, placement, materials, specimen API and JSON, none of which the page uses. This spec ships the core with only the stages the page reads, behind a small typed entry point.

Owner quotes:
> "I have an idea on how to get my homepage killenberger.com shipped more quickly with a tree. We can just use the generated field and make a voxel tree no? one that fits the black white aesthetic."
> "But for that we'd need the generated field to be lightweight enough to be included in killenberger.com We shouldn't drag in the full rendering thing."
> "HOw do you propose to go about this making an importable package that's efficient and fast"
> "no we don't want voxels output in telperion-core."
> On clump dropping staying in the consumer: "i agree with that last bit."
> "this pipeline restructure's performance must be faster or at worst equal to current performance."

## Decisions

- Each output stage sits behind its own Cargo feature; the default is today's full build. The slim build is a separate crate, `crates/telperion-field` (core without geometry or JSON), so its artifact keeps its name, its features cannot unify with the full crate's, and `cargo tree` answers R2.
- Request is species id + seed (+ optional limb order); a family the plan cannot describe is rejected with the core's input error (no placement fallback linked).
- Typed entry point `telperion/field` (`src/field/index.ts`), running in a browser worker and Node. The Wasm is named through a constant so its URL resolves at run time.
- Example voxelizer `src/field/voxelize.ts` (owner, 2026-09-22): wood cutoff in metres, foliage thinning rule with keep fraction, limb order; an example beside the primary query, never the only entry point.
- No voxel type in the core; homepage rendering and clump thinning live in killenberger.com (owner). Publishing to a registry is the owner's step.
- R3 amended by the host (2026-09-22): the first draft's 250 ms ceiling came from the prototype's structure-export path, was never measured, and is withdrawn in favour of parity with the full binding.

## Acceptance Criteria

- **R1:** Slim Wasm brotli-11 smaller than 344,254 bytes, target half — met: 101,417 bytes; JS entry 1,067 bytes brotli.
- **R2:** Isolated feature build with no surface, placement, material, specimen, JSON or serialization code; slim import bundles without the full module — met.
- **R3:** Species+seed to a queried 64-cell grid in Node at parity (no tree >10% slower warm) with the full binding — met: warm medians 349, 388, 406, 649, 289, 273 ms; binding overhead 1-17 ms.
- **R4:** Full build unchanged; existing binding tests pass unedited — met.
- **R5:** Smoke tests in Node and a browser worker — met.
- **R6:** `experiments/voxel-field` on the entry point reproduces the accepted order-3 sheet and the 2 cm wood-only sheet byte for byte — met.
- **R7:** Example voxelizer typed, tested per rule and dial, documented in `docs/field-package.md` — met.

## Result

Shipped in `04407452`, `217c502e`; gate green (891 passed). `presets::by_identity` and the catalogue tables moved out from under the `json` feature so both bindings resolve a species through one path. Full numbers: `.flow/evidence/fn-101-a-slim-growth-and-field-package-for-the/RESULTS.md`.

## Open

- Strip the 35 KB function-names section from the release Wasm.
- A flags-only field query for consumers that read no counts or limbs (core follow-up).

## Friction

- The accepted sheets sat in another worktree's ignored `raw/` (~3 min) -> a results note naming a sheet should name its checkout.
- First R3 run on a loaded desk (load 15.8) -> measure the full binding in the same minute; a load-aware script would remove it.
- R3's 250 ms ceiling was written unchecked against fn-100's own measured figures -> led to the AGENTS.md "checked claims" rule; ceiling amended to parity.

## Evidence

- `.flow/evidence/fn-101-a-slim-growth-and-field-package-for-the/RESULTS.md` (referenced by `docs/field-package.md`)
