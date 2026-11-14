# fn-367 result, 14 Nov 2026 (worker draft for the host)

Base `78a8e6a94`. R1 is `R1-READERS.md`: no reader of `/fidelity` reads it for anything but the coarse rows, so the rows and the parameter were deleted together. Raw logs are in the ignored `raw/` beside this file; the base readings ran in a `git archive` export of `78a8e6a94` with its own target.

## R2, the deletion

- Gone: `telperion-space/src/species/coarse.rs` (`Coarsening`, `COARSEST`, `Species::coarsened`, their tests) and its export; `Space.coarse`, the preset parser's `["coarse", name]` arm, `coarse_row` and the writer's coarse rows; `Family`'s coarse and coarsest validation; the blend's coarse walk; `GrowInput`'s coarsening, its `fidelity` field and `fidelity_admitted` with its two callers; `pipeline/fidelity_tests.rs`; the `/fidelity` row, `FIDELITY` and its default; its panel group, the sweep's hold and the geometry benchmark's skip; the three presets' six rows and comments.
- Added: `/fidelity` to the catalogue's `RETIRED` list, so a wire or value file that names it is refused with why. This follows the list's own doc comment and the `/growth` precedent (a row retired with no replacement); the spec did not name it; the host kept it (14 Nov 2026): a family file that still names `/fidelity` is refused with the reason.
- `catalogue::tests` count 369 rows, ranks 0 to 368 (was 370): the deleted row was the last rank.
- `cargo build --profile ci --workspace`: exit 0, no warning from the changed code. The Wasm crate (`scripts/build-wasm.mjs`, release): exit 0; its three warnings (`angular.rs` `bytes`, `chunks.rs` `extent`, `live.rs` `unsafe`) are in untouched files and the base build shows the same count. `telperion-core --no-default-features` builds, exit 0.
- No orphaned helper: `measures::shed_share` and `limb_count`, read by the deleted test, are still read by `field/floor_tests.rs`.
- Lines: 69 added, 763 deleted, over `crates/`, `src/` and `docs/`.

## R3, digests

- `every_preset_grows_the_same_tree_again_and_on_any_cores` (ignored, run by name with `--ignored --exact`): passed, 184 s.
- The pin's fine end, now `every_preset_grows_the_tree_it_grew_at_sixteen`: **red on the base and red after, with the same eight digests.** At `78a8e6a94` the old both-ends test reads fine ends `[4262522910082062681, 13115723846314570422, 7598300905195022402, 12081055634519087481, 3196652887491806787, 1911908266268488713, 4032388747217622509, 16969238882047529609]` against the pinned `[13986120611573680668, ...]`; after the deletion the renamed test reads exactly those eight. So no shipped tree moved here; the pin was already stale at the base, most likely from fn-364's `f4ce1ded4`, which landed after its last re-pin `986867c3e` (not checked commit by commit: unknown). The pinned values are left as they were (host, 14 Nov 2026): the renamed pin is red on this branch for a cause inherited from the base, fn-364's rule, which moved the fine ends after this branch's base was cut. fn-364's own R4 has re-pinned them on its branch (`72169847a`, with R8 for the Two Trees after it), so the pin is fn-364's to move and this branch meets it at the merge. The eight values measured before and after this change are equal, which is R3's proof that this change moved no tree. The host checks the pin green after merging fn-364's tip.

## R4, generated files and identity

- `parameter_reference` run with `TELPERION_WRITE_REFERENCE=1`: rewrote `docs/parameters.md` (the `/fidelity` section) and `src/browser/parameters.generated.ts` (the field and the row). `scripts/build-wasm.mjs` with `TELPERION_WRITE_REFERENCE=1` rewrote `presets.generated.ts`: 8 `"fidelity": 0` lines and three `coarse` objects, 32 lines.
- `catalogue_identity` was green on the base (5 of 5); after the deletion its four digests moved, as the family text, the wire and the refusals lost the row; each re-pinned naming fn-367: family 14553619485299927510, walk 7879059221637557532, overrides 13198617521797237499, refusals 11839571377004440045. Green after, 5 of 5.
- The package's parameter tests (`harness/params`, `rows`, `panel`, `space`, `family`, `src/browser/presets`): 6 files, 74 tests passed.

## R5, the Wasm module

`target/wasm32-unknown-unknown/release/telperion_wasm.wasm`: 1,556,211 bytes at the base, 1,547,984 after, 8,227 fewer (0.53%).

## R6

Narrow tests only. `generation_limit_guard` 2 passed; `telperion-render --lib compile_size` 2 passed.

## Found, not fixed

- `cargo fmt --all --check` reports diffs in `pipeline/surface/ease_tests.rs` and `telperion-space/src/shed/tests.rs`, untouched by this task.
- `pipeline/surface/shoulder_tests.rs:26` warns of an unneeded `mut` in the test build, untouched.
