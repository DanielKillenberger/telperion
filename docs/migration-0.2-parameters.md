# Every entry takes a tree's parameters

In 0.1.4 `growField(species, seed, n)` and `TreeEngine.build(family | id, outputs)` took a preset's id and resolved it inside the Wasm. 0.2.0 takes the parameters only (fn-317, owner: "We shouldn't hand a species name to the generator"): a preset is a reusable parameter set, and a tuned or new one grows the same way.

```ts
import { presetById } from "telperion/presets";

const birch = presetById("silver-birch");   // plain data, no Wasm
birch.skeleton.seed = 7;                    // the seed is a parameter
await growField(birch, 36);                 // was growField("silver-birch", 7, 36)
engine.build(birch, { field: { n: 36 } });  // was engine.build("silver-birch", ...)
```

- An id string is refused by name at run time ("not a preset id: pass presetById(id)") and by the type.
- The renderer takes the same parameters: `createRenderer(canvas)`'s `setTree` and `beginGrowth` take a `Family` where they took its JSON text, and serialize it as `TreeEngine.build` and `growField` do (`renderer.setTree(birch)`, was `renderer.setTree(JSON.stringify(birch))`). A nonfinite parameter is refused by its name.
- The field's types have one definition: `Grid`, `FieldAnswer`, `FieldGrid` and `Bounds` from `telperion/field` (and `Bounds` from the main entry) are the same types `TreeEngine.build` returns, its `diagnostics.field` a `Grid` and its `field` a `FieldGrid`.
- `telperion/presets` exports `presetById`, `PRESETS`, the named presets (`ORDINARY`, `TELPERION`, `LAURELIN`, ...) and the `Family` and `TreePreset` types; the main entry re-exports them unchanged.
- `growField`'s result carries no `species` or `seed`; the caller holds the parameters it passed.
- The field names a limb system by the cell where it holds the most leaves, so a limb's id in `answer.limbs` differs from 0.1.4's. Ids stay opaque; `NO_LIMB` is unchanged.

## One grower: the field and growth run on the CPU (fn-353)

Against the 0.2.0-next prereleases, which grew `telperion/field`'s trees and the renderer's growth on the device:

- `growField` grows on the CPU through `telperion.wasm`, the main entry's core, and returns the same `FieldGrid` as `TreeEngine.build(family, { field: { n, limbOrder } })`. It works with and without WebGPU, in a page, a worker or Node.
- `prepareField` and `FieldError` (with its codes `"webgpu-unavailable"` and `"device-lost"`) are gone. There is no device to prepare, and nothing refuses for one. A family the core refuses rejects with the core's message (`Tree core: ...`), as `TreeEngine.build` does.
- `growField`'s `source` and `compileField` name `telperion.wasm`, not `telperion-render.wasm`. A compiled `Module` is still accepted, and `TreeEngine.create` takes one too.
- The renderer's growth runs on the CPU, on the page's thread: `beginGrowth(family)` returns its `Growth` at once rather than a promise, and `growTo` and `draw` return their results directly. `Growth.admission()` is gone, and so are a draw's `madeRoom` and its nulls for a spent or busy tree.
- The renderer's `setTreeGpu`, `admission`, `preparePrograms` and `programs` are gone. `createRenderer` resolves once the wood's programs are compiled, so a created renderer draws the tree `setTree` builds; a program the browser refuses, or a device lost while they compile, rejects `createRenderer` by name.
