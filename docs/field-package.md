# The field package

A page that draws its own tree needs three things from Telperion: grow a
tree, prepare its leaf plan, and read its occupancy out to a grid. The
field package is that: `src/field/index.ts`, published as the package's
`./field` export beside the main entry. The tree grows on the CPU, through
the core's module (`telperion.wasm`), as the main entry's
`TreeEngine.build(family, { field: { n, limbOrder } })` grows it, and only
the grid comes back. The same entry runs in a browser page, a worker or
Node, with or without WebGPU.

## The entry point

```ts
import { growField, compileField } from "telperion/field";
import { presetById } from "telperion/presets";

const birch = presetById("silver-birch");
birch.skeleton.seed = 7;
const tree = await growField(birch, 36, { limbOrder: 3 });
tree.n;                      // 36 cells a side
tree.cell;                   // a cell's side in metres
tree.origin;                 // the grid's least corner [x, y, z] in metres, Y up
const { answer } = tree;     // one entry per cell i = x + n * (y + n * z)
answer.flags[i] & 1;         // wood reaches cell i
answer.flags[i] & 2;         // foliage reaches cell i
answer.woodRadius[i];        // the thickest wood radius reaching it, metres, 0 without wood
answer.leaves[i];            // the leaf stations estimated in it, before the crown cull
answer.limbs[i];             // the owning limb system, NO_LIMB where no foliage reaches
answer.flags[i] & 4;         // flowers in cell i
answer.flowers[i];           // the florets whose seat lies in it (fn-316)
```

The request is a tree's parameters (a `Family`) and the grid's side
(fn-317). A preset is a reusable parameter set: `telperion/presets` holds
the shipped ones as plain data (`presetById(id)`, `PRESETS`, `TELPERION`,
`LAURELIN`, ...), with no Wasm, so a page reads a preset's values (its seed,
its `material.bloom*` colours) without loading an engine, and grows a tuned
or its own parameter set the same way. The tree grows at its own seed,
`family.skeleton.seed`, an unsigned 32-bit integer. The parameters are read
and refused as the main entry reads them (`TreeEngine.build`): a preset's id
in place of the parameters is refused by name, and so is a nonfinite value;
parameters the core refuses are refused with the core's message.
`limbOrder` names limbs at that lateral order and defaults to the family's
own; a higher order parts the crown into more and smaller systems.

The grid is the one `telperion/field/voxelize`'s `grid` defines, over the
tree's own box: `n` cells a side, cubes of side `cell = longest box axis /
(n - 2)`, centred on the box in x and z with one empty cell below its lowest
point; cell `i + n * (j + n * k)` is centred at `origin + (i + 0.5, j + 0.5,
k + 0.5) * cell`. The pipeline's field and the voxelizer read this one
definition. `n` is a whole number from 3 to 256: a side outside that is
refused by name (`field grid side`) before any work.

The answers are read from the drawn tree to the grid (`pipeline/field.rs`).
A limb system is named by the cell where it holds the most leaves (fn-317).
A cell holds wood where its circumsphere meets a wood sweep, so a grid
overstates the wood by at most half a cell's diagonal; a finer grid shrinks
it.

The flowers (fn-316) are the florets the drawn tree hangs at its flowering
window's middle, each counted in the cell its seat (the floret's position)
lies in; over the grid they sum to the tree's florets, and a tree that
flowers nowhere answers zero everywhere. The grid's box holds every seat.

Left to itself the entry loads `telperion.wasm` from beside itself; a
caller that keeps the module elsewhere passes `source` (the bytes, a
`Response` or a compiled `WebAssembly.Module`). `compileField()` compiles
the module beside the entry once, however many calls ask, and the next call
retries a failed compile; `compileField(source)` compiles the source it is
given on every call, so a caller that grows in several workers compiles
once and passes the Module to each. One page or worker holds one core for
its fields: the first call to load it names the source.

Migrating from 0.1.4 (for a page on the published 0.1.4 package, such as
the site; fn-257 R1b, R8b): the `cut` option is gone and nothing replaces it; the
source is the core's module, and `telperion-field.wasm` no longer ships.
Beside it, the cube query is gone: `growField` takes the grid's side and
resolves to the grid with its answers, and `query(cells)`, `bounds` and
`release()` go with nothing in their place. A page that queried
`grid(tree.bounds, n).cells` asks for `n` instead; one that queried its own
cubes reads the cells of a grid fine enough for them. `growField` takes the
tree's parameters, not a species id and a seed
([docs/migration-0.2-parameters.md](migration-0.2-parameters.md)).

## The example voxelizer

`src/field/voxelize.ts` is an example beside the entry point, not part of it:
one reading of the field's answers that a consumer may import, copy or
replace. The entry point does not need it and it draws nothing.

```ts
import { voxelize, centre } from "telperion/field/voxelize";

const tree = await growField(birch, 64);  // 64 cells a side, 62 across the longest axis
const cubes = voxelize(tree, { woodCutoff: 0.02, thin: { rule: "gap", keep: 0.3 } });
for (const i of cubes.wood) centre(tree, i);      // a wood cube's centre in metres
for (const i of cubes.foliage) centre(tree, i);   // a foliage cube's centre
for (const i of cubes.flowers) centre(tree, i);   // a flower cube's centre
```

Each cell is in one list at most. Flowers draw where the field counts them
and no wood draws, in place of the foliage there: a cell with leaves keeps
its flowers where its foliage survives the thinning, and a cell with flowers
alone, which no limb owns, always keeps them. The wood alone (`thin` left
out) draws no flowers.

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

## Building and testing

`npm run build` emits `dist/field.js` beside `dist/telperion.wasm`. The
field's own tests are the core's (`cargo test --profile ci -p
telperion-core field`); the Node tests (`src/field/field.test.ts`,
`src/field/compile.test.ts`) hold the entry's answer against the main
entry's, its refusals and its compile, `src/field/voxelize.test.ts` holds
the voxelizer's grid to the core's, `scripts/test-dist.mjs` grows a field
from the built package in Node, and `tests/browser/field.mjs` grows one in
a browser worker.
