# The native entry

What an engine links to grow trees without a browser, the harness or a GPU (fn-351 P10, fn-365). It is the pipeline's own door (`docs/pipeline.md`), named here; there is no facade crate, no C ABI for native callers and no plugin.

## What an engine links

The crate `telperion-core`. Its dependencies are `telperion-space` (with `libm` and `rayon`) and `slotmap`, and with `json` also `serde` and `serde_json`; none is a browser, GPU or windowing crate (`.flow/evidence/fn-365-the-native-entry-is-named-and-a/READING.md`). Two features matter:

- `json`: `params::parse`, the parameter table read from its wire object. An engine that builds a `Family` in Rust does not need it.
- `geometry` (default): the wood, the leaves, the field and `growth::mesh`. Without it the crate returns the solved tree, the leaf plan and the structure, for an engine that draws its own geometry.

## The calls, in order

1. `params::parse(&serde_json::Value) -> Result<Family>`: the parameters, every row judged. A preset's id in their place is refused by name.
2. Either hold the tree between years:
   - `pipeline::growth::Params::of(&family)`, kept in an `Arc`;
   - `growth::begin(&params, seed, horizon)`, the tree before its first year;
   - `growth::grow_to(&mut state, age)`, any age, past the horizon too;
   - `growth::settle(&state, at) -> Tree` and `growth::mesh(&state, at, &family) -> TreeMesh`, the tree at any `at` within the last year grown; `growth::colouring(&state, at)`, how far its falling leaves have turned.
3. Or build once: `pipeline::build(&family, Request) -> Built`.

The seed and the age are arguments of `growth`: `begin`'s `seed` grows the tree, and `grow_to`'s `age` and `settle`'s `at` say when it is. `growth` does not read the family's `age` row. `growth::mesh` dresses the tree from the family's rows, and its leaves are placed by the family's `skeleton.seed`, not by `begin`'s. `pipeline::build` reads both rows and takes no seed or age of its own.

## What comes back

`growth::settle` returns the `Tree`: its nodes with position, parent, radii, kind and stem. `growth::mesh` returns the `TreeMesh`: the wood as chunked indexed meshes (`SurfaceMesh`), the wood as a curve, the leaf element with its instances, the flowers, and the bounds of all of it.

`pipeline::build` returns the solved tree and, by `Request`, these outputs:

- `wood`: the chunked wood mesh.
- `curve`: the wood as curves.
- `leaves`: the leaf element, its instances, the counts placed and kept, and their bounds.
- `structure`: six floats and three words a node.
- `field`: the occupancy grid of `n` cells a side.

## What is promised

- **Parameters, never a name.** The entry reads a parameter table and refuses a preset's id. Tests: `native_entry::a_native_entry_takes_parameters_and_a_seed_and_returns_the_trees_data_at_any_age` (`crates/telperion-core/tests/native_entry.rs`), which grows the oak from parameter text; `a_preset_id_is_refused_and_its_parameters_build` (`crates/telperion-wasm/src/generate/tests.rs`).
- **A year is a state.** The tree at a year reached on the way to a later age is the tree grown to that year alone, to the bit. Tests: `native_entry` (the oak at 20 on the way to 40), `a_tree_grown_a_year_at_a_time_repeats_and_ends_at_the_tree_asked_for` (`pipeline/space/growth/tests.rs`), and `a_year_is_a_state_not_a_horizon` and `every_year_is_the_horizons_prefix` (`telperion-space`, `grow/cycle/paused/tests.rs`).
- **The same tree on native and Wasm.** Every decision the grower takes has a fixed representation (`docs/pipeline.md`, "The one cycle and its grower"). Test: `native/wasm node buffers byte-identical` (`harness/parity.test.ts`), every shipped preset's structure through `pipeline::build`. No test compares a tree drawn through `growth` across the two targets.
- **A tree past the machine is refused by name.** In Wasm, a tree expected or grown past 4,000,000 kept phytomers is refused by a message that names the ceiling, and a wood larger than the memory left is refused with both numbers. Natively the generator holds no cap of its own. Tests: `a_tree_expected_over_the_ceiling_is_refused_before_it_grows` and `a_tree_over_the_wasm_ceiling_is_refused_by_name` (`pipeline/space/tests.rs`), `a_wood_over_what_is_left_is_refused_with_both_numbers` (`pipeline/surface/budget.rs`), `generation_limit_guard`.

## What is not built

- A plugin for any engine (owner, 1 Nov: "Any imports to game engines are out of scope for this goal").
- A native C ABI. The binding's ABI (`crates/telperion-wasm`) is the browser's, and no native build of it is produced or tested.
- Twigs for a region asked (fn-321 R11): fn-351's P10 does not carry it, and nothing builds wood on demand.
