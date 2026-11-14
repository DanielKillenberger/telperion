# Changelog

## 0.2.0 (release candidate, not yet released)

Against 0.1.4 (tagged 25 Sep 2026). The spec in parentheses owns each line and lives in `.flow/specs/`; the release's goal and its dated log are `fn-351`. Measurements give their date where the tree has changed since.

### What changed in the trees

- **One growth engine for every tree.** 0.1.4 drew a crown inside an authored envelope by space colonization. 0.2.0 grows every tree from its seed by one engine, in which a species is a set of `/space` rows: buds carry a physiological age, and light and a carbon balance decide what lives (fn-191 to fn-198). A family with no `/space` rows is refused by name (`no engine species`).
- **One grower, on the CPU.** The GPU grower and the device's expansion of its trees are deleted. Every entry grows on the CPU, natively and in Wasm, and the GPU only draws (fn-353). The render module fell from 841,705 to 534,938 Brotli bytes with it.
- **A year is a state.** The grower reads no horizon. The tree at year y on the way to a later age is, to the bit, the tree asked for at y, and a growth can go on past the age it was begun for (fn-355).
- **Girth from sapwood and leaf.** A section is heartwood plus sapwood. The sapwood is the leaf above it over the species' sapwood area per leaf area (`form.sapwood`), and one ring in `form.sapwood_rings` turns to heartwood each year. Wood is never lost. Three rows (`primary`, `sapwood`, `sapwood_rings`) replace six, and sag reads green wood's measured stiffness (fn-354). On 12 Nov 2026 the oak's median trunk at 40 over twelve seeds was 0.33 m, inside White's open-grown 0.28 to 0.40 m, where the build before it drew 0.78 m; see the known limits for older oaks.
- **One leaf account.** The leaf a phytomer bears comes from one function that the light and the wood both read: its leaf area while its age is under the species' `leafYears` and its wood is open (fn-354 R5).
- **A lateral's kind follows its light.** A shaded bud may become a short shoot by the age's `shadeShort` table (fn-354 R7). The beech's crown is fuller for it.
- **The carbon balance is charged by height.** A living tip earns its size times the light at its tip and is charged its size times the square of its height above the root collar over the species' potential height. Dead wood is charged nothing (fn-349, fn-363). Before fn-363 the charge read the tip's path length. A tree whose supernatural group lengthens its internodes has its potential height lengthened by the same ratio (fn-364 R8).
- **A subtree fades once.** A line of relays is drawn at its living apex's size however many axes carry it (fn-359).
- **The drawn girth eases at every drop.** Along a run the drawn girth eases from what is drawn to the grown radius over twice the radius it fell from, at forks, growth units and relays alike, and never falls faster than 1 in 1 (fn-360).
- **Ring sides by width.** The CPU wood mesh cuts each run with the fewest sides that stay within 0.2 mm of the circle, capped at the preset's `radialSegments`. Every preset's CPU mesh has under half the vertices it had before this change, and the grown tree is unchanged (fn-362).
- **The grower holds less.** A retired subtree keeps a 24-byte stand and a growing phytomer 72 bytes. On the oak at 120 the grower's footprint fell from 7.1 to 3.8 GiB, with every tree equal to the bit (fn-357, fn-358).
- **No cap of the generator's own.** No phytomer, node or shoot budget stops a tree; a tree stops only where the machine's memory does (fn-330, fn-351 P7).
- **A European beech preset**, beside the seven 0.1.4 shipped (fn-193).
- **Flowers** on a tree whose ages flower, Telperion and Laurelin among the presets (fn-216, fn-294, fn-316).

### Removed

- The GPU grower and its device expansion, with the renderer's `setTreeGpu`, `admission`, `preparePrograms` and `programs`, and the field's `prepareField` and `FieldError` (fn-353).
- 0.1.4's growth path: `TreeEngine.buildSpecimen`, `importSpecimen`, the `Specimen*` types, and the renderer's `buildSpecimen` and `seekSpecimen` (fn-181). `docs/growth-path.md` keeps why it did not hold.
- The field's cube query (`tree.field.query`), its `bounds` and `snapshot()` (fn-257).
- One wood buffer per field: `surface.positions`, `normals` and `indices` (fn-291).
- A preset's id at any entry. Every entry takes parameters only (fn-317).
- The `/fidelity` parameter and the per-species `/space/coarse` rows (fn-367). `/fidelity` is refused by name.
- Retired rows, each refused by name with what replaced it: `/growth`, `/skeleton/habit/stems` and its `stem*` siblings, and the `/canopy/shortShoot*` rows.
- The `/space` form rows `pipe` (renamed `primary`), `exponent`, `ripening`, `secondary`, `retained` and `leafGirth` (fn-354).

### Added

- **The native entry**: `telperion-core`'s `pipeline::growth` and `pipeline::build`, named and documented in `docs/native-entry.md`, with a headless test that grows the oak from parameter text (fn-365). No plugin and no C ABI.
- **One seed for a growth.** A growth draws its wood and its leaves by the seed it was begun with, never the family's `skeleton.seed` row (fn-365 R6).
- **The growth path's size**: `scripts/growth-path.mjs` prints the Brotli bytes a field page downloads to grow a tree, and the budget check prints it (fn-366).
- `telperion/presets`: the shipped presets as plain data, with no Wasm (fn-317).
- The renderer's `beginGrowth(family)`, a growth held between years and drawn at any age in its last year (fn-270, fn-353).
- `decodeNormals` for the chunked wood's normals (fn-291).
- `docs/pipeline.md` and `docs/tree-space.md`, the pipeline's boundary and the engine's programme.

### Breaking changes, and what a caller does

- **A family saved for 0.1.4 does not build.** It has no `/space` rows and is refused (`no engine species`), and its `/growth` and `shortShoot*` rows are refused by name. Start from a 0.2.0 preset with `presetById(id)` and change its rows.
- **Pass parameters, not an id**, to `TreeEngine.build`, `growField`, `setTree` and `beginGrowth`: [docs/migration-0.2-parameters.md](docs/migration-0.2-parameters.md).
- **Read the wood in chunks**: [docs/migration-0.2-wood-chunks.md](docs/migration-0.2-wood-chunks.md).
- **Ask for the field by its side**: `{ field: true }` is `{ field: { n } }`, and the cube query has no replacement (README, "Migrating from 0.1.4").
- **Flowers** arrive as `tree.organ` beside the leaves: [docs/migration-0.2-flowers.md](docs/migration-0.2-flowers.md).
- **Drop `/fidelity` and `/space/coarse`**: [docs/migration-0.2-fidelity.md](docs/migration-0.2-fidelity.md).
- **Grow through `beginGrowth`** where 0.1.4 called `buildSpecimen`; it returns at once and grows on the page's thread (docs/migration-0.2-parameters.md, "One grower").
- **Expect different trees at the same seed.** Seeds name a specimen within one revision; 0.2.0's trees are not 0.1.4's (STRATEGY.md, "Attributability").

### Known limits

- **Speed against 0.1.4 is not met.** On 11 Nov 2026, before the changes above, the oak at 40 took 2,073 ms in Wasm and the spruce 7,548 ms, against 0.1.4's 124 and 122 ms for its own presets at their own ages (fn-352). The candidate's Wasm times are unknown. Natively on one thread the oak at 40 takes 1.44 s and 0.68 s on four threads, against an aim of 0.1 s; one speed round with the tree held equal took 3.6 to 5.3% off the CPU time (fn-368). A thousand oaks at 40 took 496 s across 32 threads on 12 Nov, against an aim of 10 s.
- **The download is over its budget.** A field page fetches 364,870 Brotli bytes to grow a tree, against the owner's 250,000. The lowest measured, 291,123, needs a field that does not read the drawn tree (fn-366).
- **Old oaks are larger than their sources.** On seeds 1001, 1007 and 1012 the oak's trunk at 1.3 m is 1.15, 1.65 and 1.34 m at 120 (White: 0.83 to 1.09 m) and 2.20, 3.13 and 2.90 m at 300 (1.56 to 1.92 m). Its tallest tip reaches 36.0 to 39.4 m from 200 on, against the species' recorded 34.8 m. It keeps a living crown to 300 and grows within 6.3 GiB (fn-363). A rule that held the old oak inside its size (fn-364) was built and withdrawn on 14 Nov 2026 because its limbs stalled and coiled.
- **The beech's leaf is not settled.** Its drawn leaf is 2.9 times what its grower counts (the drawn blade is 2.16 times the measured leaf), and its grower counts about half what Bartelink 1997 gives for its trunk (fn-370 R1). Sourced shoot and leaf tables for the real species are not built (fn-351 Phase 4).
- **The Two Trees' size on the candidate is unknown**; it was being measured again on 14 Nov 2026.
- **Not built**: counted foliage, levels of detail for a whole distant tree, the grove drawn from a tree's cells, the renderer's growth off the page's thread, and a forest rig (fn-351 Phases 5 and 6).
- **The browser's memory.** A tree in the browser is limited by 32-bit Wasm's 4 GiB and is refused by name past it. How that is decided is being changed (fn-369, not merged); whether every shipped preset builds at every seed is unknown until it lands.
- **A jump close.** When the camera jumps close, the first frame is drawn coarser and may leave wood out; the next is whole (fn-258).
- **Not checked by an agent**: a grown year reaching the canvas, grow-in and play-the-year on live WebGPU (fn-353).
- **The owner's verdict** on each species beside its photographs is pending (fn-351 P14).
