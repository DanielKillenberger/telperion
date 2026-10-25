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
- `telperion/presets` exports `presetById`, `PRESETS`, the named presets (`ORDINARY`, `TELPERION`, `LAURELIN`, ...) and the `Family` and `TreePreset` types; the main entry re-exports them unchanged.
- `growField`'s result carries no `species` or `seed`; the caller holds the parameters it passed.
- The field's refusals for no WebGPU and a lost device are a `FieldError` with `code` `"webgpu-unavailable"` or `"device-lost"`; the messages are as before.
- Both fields name a limb system by the lowest cell it owns, so a limb's id in `answer.limbs` differs from 0.1.4's. Ids stay opaque; `NO_LIMB` is unchanged.
