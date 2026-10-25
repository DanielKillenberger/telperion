# fn-317 result

## What changed

- **R1, R2:** `growField(family, n, options?)` and `TreeEngine.build(family, outputs)` take a `Family` only. The device field's binding (`WebField.grow`) takes the parameters as JSON and reads them with `params::parse`, as the CPU entry does. `params::parse` refuses a string by name ("family is a parameter object, not a preset id: pass presetById(id)"), and the TS serializer (`src/browser/wire.ts`) refuses one first with the same words. The tree grows at `family.skeleton.seed`. The result carries no species or seed.
- **R3:** `telperion/presets` (`src/browser/presets.ts`, `dist/presets.js`, 186,305 bytes) holds the presets with no import at all. The main entry re-exports them. `telperion.js` falls from 194,085 to 8,139 bytes, because the data moved.
- **R4, R4b:** `FieldError` carries `code` `"webgpu-unavailable"` or `"device-lost"`. The entry's own check throws it. The render module's `WebGpuUnavailable` and `DeviceLost` reject with a `code` property (`web/field.rs`, `refused`), and the entry maps that to the same class. On the module's `webgpu-unavailable`, the entry asks `navigator.gpu.requestAdapter()` directly and retries once only when that returns an adapter. That condition is the site's repro on 2131d5b: the page's first WebGPU use is the field, and its first tree is refused while the next ones grow.
- **R4c:** `Grid::name_limbs` in the core renames every limb system by the lowest cell it owns. Both package entries call it on the grid they hand on: the CPU binding (`telperion-wasm/src/generate.rs`) and the device binding (`web/field.rs`). Inside, a system keeps its node's name, which is the same on every grid of one executor. `a_cell_crowded_with_limbs_names_the_cpu_s_limb` compares a 3-cell grid with a 24-cell grid by that name, and the first build, which renamed inside the fields, failed it in the gate.

## Measured: limb names, device against CPU

Each figure is the share of the cells the partition pairs whose name is the same on both executors (`every_preset_s_limbs_carry_the_cpu_s_names`, raw/names.log). Before R4c it was 0 everywhere, since each executor named a system by its own node id.

| Preset | Seed 1 at 24 / 36 | Seed 5678 at 24 / 36 |
|---|---|---|
| ordinary | 0.969 / 0.951 | 0.992 / 0.983 |
| oregon-white-oak | 0.989 / 0.968 | 0.986 / 0.980 |
| norway-spruce | 1.000 / 1.000 | 1.000 / 1.000 |
| european-beech | 0.964 / 0.956 | 0.912 / 0.970 |
| silver-birch | 1.000 / 1.000 | 0.999 / 1.000 |
| date-palm | 1.000 / 0.000 | 0.000 / 0.000 |
| telperion | 0.973 / 0.979 | 0.973 / 0.965 |
| laurelin | 0.968 / 0.978 | 0.936 / 0.971 |

The remainder: a system whose lowest cell the other executor gives to a neighbouring system takes another name as a whole. The palm's frond leaflets fall on each executor's own draws, so its single system's lowest cell differs. The test holds 0.9 for every preset except a frond crown. `voxelize` is a pure function of the answers, so it thins identically wherever the answers agree. It was not run on device answers in Node, which has no WebGPU.

## Proofs

- `every_preset_grows_its_field_from_its_parameters_as_from_its_id` (device): every preset at seeds 1 and 5678 on 24 cells. The field grown from `params::parse(params::metadata(preset))` equals the one grown from the catalogue's family, byte for byte. Both are on this branch, so both name limbs by R4c.
- `src/browser/presets.test.ts`: the presets load in Node with `fetch` never called; `TreeEngine.build("telperion")` is refused by name; a birch ten years younger reads a smaller cell.
- `src/field/field.test.ts`: in Node, every call rejects with a `FieldError` coded `webgpu-unavailable`, and an id is refused by name.
- `src/field/device.test.ts`: with the module stood in for, there is one retry where the browser has an adapter and none where it has not; a lost device is coded and not retried; another refusal passes through; the module is handed parameters, never an id.
- `scripts/test-dist.mjs`: `dist/presets.js` imports nothing and names no `.wasm`; `field.js` in Node rejects with a `FieldError`.
