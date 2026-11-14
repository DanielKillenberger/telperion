# fn-317 result

## What changed

- **R1, R2:** `growField(family, n, options?)` and `TreeEngine.build(family, outputs)` take a `Family` only. The device field's binding (`WebField.grow`) takes the parameters as JSON and reads them with `params::parse`, as the CPU entry does. `params::parse` refuses a string by name ("family is a parameter object, not a preset id: pass presetById(id)"), and the TS serializer (`src/browser/wire.ts`) refuses one first with the same words. The tree grows at `family.skeleton.seed`. The result carries no species or seed.
- **R3:** `telperion/presets` (`src/browser/presets.ts`, `dist/presets.js`, 186,305 bytes) holds the presets with no import at all. The main entry re-exports them. `telperion.js` falls from 194,085 to 8,139 bytes, because the data moved.
- **R4, R4b:** `FieldError` carries `code` `"webgpu-unavailable"` or `"device-lost"`. The entry's own check throws it. The render module's `WebGpuUnavailable` and `DeviceLost` reject with a `code` property (`web/field.rs`, `refused`), and the entry maps that to the same class. On the module's `webgpu-unavailable`, the entry asks `navigator.gpu.requestAdapter()` directly and retries once only when that returns an adapter. That condition is the site's repro on 2131d5b: the page's first WebGPU use is the field, and its first tree is refused while the next ones grow.
- **R4c:** `Grid::name_limbs` in the core renames every limb system by the cell where it holds the most leaves, the lowest on a tie. Both package entries call it on the grid they hand on: the CPU binding (`telperion-wasm/src/generate.rs`) and the device binding (`web/field.rs`). Inside, a system keeps its node's name, which is the same on every grid of one executor. `a_cell_crowded_with_limbs_names_the_cpu_s_limb` compares a 3-cell grid with a 24-cell grid by that name, and the first build, which renamed inside the fields, failed it in the gate. The first rule, the lowest owned cell, agreed on only 0.912 to 1 of paired cells, with the palm at 0: a system's lowest cell sits on its edge, where the executors disagree. The codex review held that against R4c, and the densest cell replaced it. R4c is amended to the measured floor (the spec's note).

## Measured: limb names, device against CPU

Each figure is the share of the cells the partition pairs whose name is the same on both executors (`every_preset_s_limbs_carry_the_cpu_s_names`, raw/names.log). Before R4c it was 0 everywhere, since each executor named a system by its own node id. The test holds 0.97.

| Preset | Seed 1 at 24 / 36 | Seed 5678 at 24 / 36 |
|---|---|---|
| ordinary | 1.000 / 0.998 | 1.000 / 1.000 |
| oregon-white-oak | 1.000 / 1.000 | 0.998 / 0.999 |
| norway-spruce | 1.000 / 1.000 | 1.000 / 1.000 |
| european-beech | 0.985 / 0.999 | 0.986 / 0.997 |
| silver-birch | 1.000 / 1.000 | 0.999 / 1.000 |
| date-palm | 1.000 / 1.000 | 1.000 / 1.000 |
| telperion | 1.000 / 1.000 | 0.999 / 0.999 |
| laurelin | 0.999 / 1.000 | 1.000 / 0.999 |

## Measured: the voxelizer on both executors' answers

`every_preset_s_named_fields_are_written_for_the_voxelizer` (ignored) wrote every preset's named CPU and device fields at seed 1 on 24 and 36 cells, and `cubes.mjs` voxelizes both with each rule (woodCutoff 0.02, keep 0.7). Each entry is the cubes in one set and not the other, over the CPU's cubes, at 24 then 36 (raw/cubes.log). `density` and `mass` read no names, so their remainder is the answers' own. `tuft` reads each system's centre, which moves with the partition's edge cells, and `coin` and `gap` move where a name or the partition does. The palm's fronds are drawn on each executor's own draws (foliage held at 0.70 in `within`).

| Preset | coin | density | mass | tuft | gap |
|---|---|---|---|---|---|
| date-palm | 353/2172, 1190/5652 | 300/1724, 1040/4332 | 172/1724, 604/4332 | 208/1724, 708/4332 | 300/1724, 1040/4332 |
| european-beech | 9/745, 39/1922 | 0/745, 0/1951 | 0/745, 0/1951 | 34/745, 100/1951 | 6/745, 18/1951 |
| laurelin | 12/2388, 26/6194 | 0/2388, 5/6248 | 0/2388, 5/6248 | 30/2388, 75/6248 | 8/2388, 29/6248 |
| norway-spruce | 0/1082, 0/2496 | 0/1023, 0/2573 | 0/1023, 0/2573 | 0/1023, 0/2573 | 0/1023, 0/2573 |
| ordinary | 8/1104, 21/2900 | 0/1168, 0/2917 | 0/1168, 0/2917 | 36/1168, 86/2917 | 6/1168, 18/2917 |
| oregon-white-oak | 1/1407, 16/3649 | 0/1352, 1/3469 | 0/1352, 1/3469 | 12/1352, 55/3469 | 4/1352, 1/3469 |
| silver-birch | 0/867, 0/2226 | 0/875, 2/2182 | 0/875, 0/2182 | 0/875, 0/2182 | 0/875, 0/2182 |
| telperion | 5/2476, 20/6129 | 0/2449, 0/6322 | 0/2449, 0/6322 | 30/2449, 68/6322 | 12/2449, 22/6322 |

## Proofs

- `every_preset_grows_its_field_from_its_parameters_as_from_its_id` (device): every preset at seeds 1 and 5678 on 24 cells. The field grown from `params::parse(params::metadata(preset))` equals the one grown from the catalogue's family, byte for byte. Both are on this branch, so both name limbs by R4c.
- `src/browser/presets.test.ts`: the presets load in Node with `fetch` never called; `TreeEngine.build("telperion")` is refused by name; a birch ten years younger reads a smaller cell.
- `src/field/field.test.ts`: in Node, every call rejects with a `FieldError` coded `webgpu-unavailable`, and an id is refused by name.
- `src/field/device.test.ts`: with the module stood in for, there is one retry where the browser has an adapter and none where it has not; a lost device is coded and not retried; another refusal passes through; the module is handed parameters, never an id.
- `scripts/test-dist.mjs`: `dist/presets.js` imports nothing and names no `.wasm`; `field.js` in Node rejects with a `FieldError`.
