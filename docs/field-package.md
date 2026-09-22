# The slim growth-and-field package

A page that draws its own tree needs three things from Telperion: grow a
tree, prepare its leaf plan, and answer occupancy queries over it. The slim
package is that and nothing else: the direct build's skeleton, the leaf plan
and the field, compiled without the wood surface, the leaf placement, the
materials, the specimen API and the JSON request path. It is
`crates/telperion-field` (the Wasm) and `src/field/index.ts` (the typed entry
point), published as the package's `./field` export beside the main entry.

## The entry point

```ts
import { growField } from "telperion/field";

const tree = await growField("silver-birch", 7, { limbOrder: 3 });
tree.bounds;                 // { min: [x, y, z], max: [x, y, z] } in metres, Y up
const answer = tree.query(cells);  // cells: Float64Array of x, y, z, halfExtent per cell
answer.flags[i] & 1;         // wood reaches cell i
answer.flags[i] & 2;         // foliage reaches cell i
answer.woodRadius[i];        // the thickest wood radius reaching it, metres, 0 without wood
answer.leaves[i];            // the leaf stations estimated in it, before the crown cull
answer.limbs[i];             // the owning limb system, NO_LIMB where no foliage reaches
tree.release();              // drops the tree; every later query is refused
```

The request is a species id and a seed. The species id selects one of the
value tables the core compiles in (`oregon-white-oak`, `silver-birch`,
`norway-spruce`, `ordinary`, `telperion`, `laurelin`); an id the catalogue
does not serve is refused with the core's `preset identity` error. The seed
is an unsigned 32-bit integer. `limbOrder` names limbs at that lateral order
and defaults to the family's own; a higher order parts the crown into more
and smaller systems. The same species, seed and order answer the same bytes
on every call and every build.

A query describes closed cubes: touching counts, a zero half extent is a
point. A batch whose length is not a multiple of four, or that holds a
non-finite centre or a negative half extent, is refused whole and answers
nothing.

The same file runs in a browser worker and in Node. In the browser it
fetches `telperion-field.wasm` from beside itself; a Node caller, or a page
that keeps the module elsewhere, passes `source` (the bytes, a `Response` or
a compiled `WebAssembly.Module`). `compileField(source)` compiles once for a
caller that grows many trees. Each `growField` call owns its own Wasm
instance, so releasing one tree touches no other.

## The example voxelizer

`src/field/voxelize.ts` is an example beside the entry point, not part of it:
one reading of the field's answers that a consumer may import, copy or
replace. The entry point does not need it and it draws nothing.

```ts
import { grid, voxelize, centre } from "telperion/field/voxelize";

const g = grid(tree.bounds, 64);          // 64 cells a side, 62 across the longest axis
const answer = tree.query(g.cells);
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

`npm run wasm:build` builds the full and the slim Wasm; `npm run build`
emits `dist/telperion.js` and `dist/field.js`. The slim crate's own tests
run with `cargo test -p telperion-field`; the Node and browser-worker smokes
are `src/field/field.test.ts` and `tests/browser/field.mjs`. The sizes and
timings of the first build are in
`.flow/evidence/fn-101-a-slim-growth-and-field-package-for-the/RESULTS.md`.
