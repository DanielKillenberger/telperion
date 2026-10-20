# The flowers in the package output

0.2.0 grows flowers on a tree whose ages flower (Telperion and Laurelin among the shipped presets), and the CPU entry (`TreeEngine.build` with `{ foliage: true }`) returns them beside the leaves (fn-294). Nothing a 0.1.4 reader used changed: `tree.foliage` is still the leaves alone, and its counts (`instances`, `leavesPlaced`, `biologicalUnits`) still count leaves.

## The new field

```ts
tree.organ?: { positions: Float32Array; indices: Uint32Array; leaves: Uint32Array; reference: LeafReference }
tree.diagnostics.organReference: LeafReference | null
```

- `positions`, `indices`: the flower mesh, three f32 a vertex and `u32` triangles, drawn once a floret as the leaf mesh is drawn once a leaf.
- `leaves`: three `u32` words a floret, the leaves' layout.
- `reference`: the box the florets' positions are quantised against. It is their own, not `foliage.reference`; the same object as `diagnostics.organReference`.

`tree.organ` is absent, never empty, where nothing flowers or no foliage was asked for, and `diagnostics.organReference` is then `null`. Asking for foliage on a flowering tree already paid for its flowers; there is no separate request flag.

## Decoding

A floret decodes as a leaf does, against the flowers' reference:

```ts
import { leafWords, leafTransform } from "telperion";
for (let i = 0; i < organ.leaves.length / 3; i++) {
  const m = leafTransform(leafWords(organ.leaves, i), organ.reference); // sixteen column-major floats
}
```

The petals' colours are the family's `/material/bloom*` rows, not part of the output.
