# The browser bindings suite: the red checks at the start (fn-259 task 2, R5)

Measured 2026-10-19 on `9ebcb2098`, headless Chromium with no WebGPU flag, vite on port 0 and the rebuilt Wasm. A scratch copy of `tests/browser/bindings.mjs` collected every failed check instead of stopping at the first. The species loop and the empty and partial builds were wrapped so a throw did not stop the run. The copy was not committed.

Twelve checks were red. The spruce and birch threw before their own checks ran, so the rest of their checks are listed only after the fixture fix (below).

Most causes come back to one fact. Since fn-198, stage 2 is the tree-space engine. `GrowInput::of` (`crates/telperion-core/src/pipeline/input.rs:56`) reads only `space`, `age`, `skeleton.seed`, `supernatural` and `canopy`. The scaffold's skeleton rows (`skeleton.envelope`, `skeleton.attractors`, `skeleton.step`, `skeleton.growth`, `skeleton.habit`) are read by no build. `skeleton.twigs` is still read, by the foliage plan through `Inputs::of`; the fixture's twig internode override goes only because sizing by age makes it unnecessary. `Family::validate`, which judges them, is not on the build path either. fn-198 recorded the same thing on the Rust side: `the_build_and_validate_refuse_the_same_values_by_the_same_name` is `#[ignore]`d as "the scaffold's grower on an engine preset's rows, which no build grows now" (`crates/telperion-core/src/family/tests.rs`). fn-288 retires the scaffold after the release.

| # | Check | Cause (measured) | Outcome |
|---|---|---|---|
| 1 | `ordinary has local branches and a leaf plan` | `nodes > crossover` fails: nodes 152,178 = crossover 152,178. The engine sets `crossover = nodes.len()` (`pipeline/space.rs:587`), so it draws no local branches past a crossover. The leaf plan is there (13,780 planned, 0 placed). | **Fixed**, by dropping the crossover half: the engine draws no crossover split. The check keeps a grown tree, a leaf plan and no placed leaf. |
| 2 | `bad height` | NaN and Infinity are refused (by the binding's finite check, before the core). `envelope.height = -1` builds the same 152,178-node tree, because the envelope is a scaffold row no build reads. | **Fixed**, by keeping the binding's NaN and Infinity refusals. The -1 case is dropped because nothing reads the height it judged. |
| 3 | `caps explicitly partial` | `growth.maxNodes = 20` builds the whole tree, complete. No code in the engine (`pipeline/space*`, `telperion-space`) sets `node_capped`, `level_capped` or `attraction_capped`; only the scaffold does. | **Deleted.** A partial build exists only on the scaffold, which no build reaches. |
| 4 | `valid empty outputs` | `growth.maxNodes = 0` builds the whole tree (52 wood chunks, 41,340 leaf words). The engine refuses `age = 0` by name (`engine = age: a tree grows at least one cycle`), and age 1 already grows wood. No family yields an empty tree through the binding. | **Fixed**, on today's contract: a tree of no cycle (`age = 0`) is refused by name. |
| 5 | `complete identity catalogue` | `PRESETS.length` is 8, not 6: the European beech and the date palm ship (`IN_WORK` is empty, `crates/telperion-core/src/presets.rs:45`). | **Fixed:** expects 8 unique ids. |
| 6 | `the beech in work is not listed` | `presetById('european-beech')` resolves: the beech is in `CATALOGUE`. | **Deleted.** No species is in work, so nothing is left to hide. |
| 7 | `the beech in work is not built by name` | Same cause. | **Deleted**, same reason. |
| 8 | `oregon-white-oak refuses crookedness` | `skeleton.habit.crookedness = 90` builds. The habit row is the scaffold's, judged only by `validate_skeleton`, which the build does not call. | **Deleted** from the refusal list, for all three species. The habit row is read by no build. |
| 9 | `norway-spruce` (throw) | `foliage instance budget` at `maxInstances = 40000`. The fixture's shrink (`envelope.height` 4, `attractors` 40, `maxNodes` 12000) sets dormant rows, so the spruce grows at its full default age of 40. The budget refusal itself is correct (`plan.rs:213`). | **Fixed (fixture):** the fixture shrinks by `age` (8 years: oak 743, spruce 7,285, birch 1,714 leaves, measured). The dormant rows and the twig internode override, which age sizing makes unnecessary, are removed. |
| 10 | `silver-birch` (throw) | Same cause. | **Fixed (fixture)**, same change. |
| 11 | `an empty tree has no field` | `maxNodes = 0` no longer empties the tree (row 4). `field of an empty tree` (`pipeline/field.rs:102`) cannot be reached through the binding, because the engine refuses the zero-cycle tree first. | **Deleted.** Its subject cannot be reached; row 4's age refusal covers the empty request. |
| 12 | `native empty recovery` | The raw `{"skeleton":{"growth":{"maxNodes":0}}}` request builds a whole tree, so the build succeeds and leaves chunks. | **Fixed:** the native request sends the whole family at `age: 0`. It is refused (status 1), leaves no chunk, and the error names the cycle (impl-review: a family of only `age` was refused as `no engine species`). |

## Checks found only after the fixture fix

None. With the fixture at age 8, the oak, spruce and birch ran every check in the loop, and the collector reported no failed check and no throw.

## The run

`npm run test:bindings` (`scripts/test-bindings.mjs`): it builds the core Wasm, starts vite on port 0 and runs `tests/browser/bindings.mjs` alone in headless Chromium, with no WebGPU flag and no `CHROMIUM_EXECUTABLE` (Playwright's own Chromium). The suite reaches its final `console.log` with all ten summary keys and exits 0 (measured 2026-10-19). The `partialDiagnostics` key is gone from the summary along with check 3.
