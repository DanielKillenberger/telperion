# The field package

A page that draws its own tree needs three things from Telperion: grow a
tree, prepare its leaf plan, and answer occupancy queries over it. The
field package is that: `src/field/index.ts`, published as the package's
`./field` export beside the main entry. Since 0.2.0 (fn-257) the tree grows
on the device (WebGPU), through the renderer's module, exactly as the
harness grows it, and its field stays there: a query sends its cells and
reads back only their answers. There is no CPU fallback.

## The entry point

```ts
import { growField, prepareField } from "telperion/field";

await prepareField();        // optional: the device and its programs, once per page or worker
const tree = await growField("silver-birch", 7, { limbOrder: 3 });
tree.bounds;                 // { min: [x, y, z], max: [x, y, z] } in metres, Y up
const answer = await tree.query(cells);  // cells: Float64Array of x, y, z, halfExtent per cell
answer.flags[i] & 1;         // wood reaches cell i
answer.flags[i] & 2;         // foliage reaches cell i
answer.woodRadius[i];        // the thickest wood radius reaching it, metres, 0 without wood
answer.leaves[i];            // the leaf stations estimated in it, before the crown cull
answer.limbs[i];             // the owning limb system, NO_LIMB where no foliage reaches
tree.release();              // frees the tree on the device; every later query is refused
```

The request is a species id and a seed. The species id selects one of the
value tables the core compiles in; an id the catalogue does not serve is
refused with the core's `preset identity` error. The seed is an unsigned
32-bit integer. `limbOrder` names limbs at that lateral order and defaults
to the family's own; a higher order parts the crown into more and smaller
systems. The answers are the CPU reference field's (`pipeline/field.rs`)
within the tolerances `resident/field_tests.rs` measures: every cell's wood
and foliage, the leaves within a few thousandths, the limbs as the same
partition. A frond's leaflets are drawn on the device's own draws, as the
harness draws them, so a palm's cells differ from the CPU's where its draws
do.

A query describes closed cubes: touching counts, a zero half extent is a
point. A batch whose length is not a multiple of four, or that holds a
non-finite centre or a negative half extent, is refused whole and answers
nothing.

Without WebGPU (Node, or a browser without it) every call rejects with
`WebGPU is not available`; what a page shows then is the page's decision.
Left to itself the entry loads `telperion-render.wasm` from beside itself; a
caller that keeps the module elsewhere passes `source` (the bytes, a
`Response` or a compiled `WebAssembly.Module`, which `compileField(source)`
makes once). One page or worker holds one device and one module instance;
each tree holds its field on that device until it is released.

Migrating from 0.1.4: `query` resolves a promise; the `cut` option is gone
and nothing replaces it; the source is the renderer's module, and
`telperion-field.wasm` no longer ships.

## The example voxelizer

`src/field/voxelize.ts` is an example beside the entry point, not part of it:
one reading of the field's answers that a consumer may import, copy or
replace. The entry point does not need it and it draws nothing.

```ts
import { grid, voxelize, centre } from "telperion/field/voxelize";

const g = grid(tree.bounds, 64);          // 64 cells a side, 62 across the longest axis
const answer = await tree.query(g.cells);
const cubes = voxelize(g, answer, { woodCutoff: 0.02, thin: { rule: "gap", keep: 0.3 } });
for (const i of cubes.wood) centre(g, i);      // a wood cube's centre in metres
for (const i of cubes.foliage) centre(g, i);   // a foliage cube's centre
```

Its dials are the ones `experiments/voxel-field` settled under fn-100:

- `woodCutoff`, in metres of radius: wood draws where the cell's thickest
  wood is at least this, through any foliage. About 2 cm for a broadleaf
  and 1 cm for the spruce reads as the limbs without the twigs.
- `thin`, a rule and a keep fraction, or left out for the wood alone:
  `coin` drops whole limb systems by a stable coin per limb; `density`
  keeps the cells with the most leaf stations; `mass` keeps the cells
  nearest drawn wood; `tuft` keeps each system's rounded mass about its
  own centre; `gap` erodes each system only where it meets a neighbour, so
  the crown's outer surface stays and a hanging curtain stays a curtain.
- the limb order, given to `growField`: how finely the crown parts into
  the systems the rules thin over.

How a page thins its foliage is the page's choice; these rules are the
ones tried so far, tested on a fixture grid in `src/field/voxelize.test.ts`.

## Building and measuring

`npm run build` emits `dist/field.js` beside `dist/telperion-render.wasm`.
The device field's tests are native (`cargo test --profile ci -p
telperion-render the_device_field -- --ignored`, against the CPU reference
on every shipped preset); the Node test (`src/field/field.test.ts`) holds the
input contract and the named refusal without WebGPU, and
`tests/browser/field.mjs` is a smoke for a browser with WebGPU in workers.
The measurements are in
`.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/`.
