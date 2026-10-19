# Migrating to the chunked wood

The CPU entry (`TreeEngine.build` with `{ surface: true }`) returns the wood in chunks (fn-291). A chunk holds at most 65,536 vertices, so its indices are 16-bit and local to it, and no buffer of the wood needs one long run of the core's memory. That is what lets every shipped preset build in 32-bit Wasm.

## The old shape

```ts
tree.surface: { positions: Float32Array; normals: Float32Array; indices: Uint32Array; bounds: Bounds | null }
```

One buffer a field over the whole wood, three floats a normal, indices absolute.

## The new shape

```ts
tree.surface: { chunks: { positions: Float32Array; normals: Int16Array; indices: Uint16Array }[]; bounds: Bounds | null }
```

- `positions`: three f32 a vertex, as before.
- `normals`: two snorm16 a vertex, the unit normal folded onto the octahedron. Decoded, it is within 1e-4 radians of the normal the core computed.
- `indices`: `u16`, local to the chunk; each chunk is drawn on its own.
- `tree.diagnostics.surfaceChunks` counts the chunks.

A reader of the old fields gets a type error: `surface.positions`, `surface.normals` and `surface.indices` are gone from `TreeOutput`. Bark coords never reached JavaScript and still do not.

## Decoding

Each chunk uploads as it is, `normals` as a `snorm16x2` vertex attribute decoded in the shader. Where three floats are wanted on the CPU, this is the decode `src/browser/wood.ts` (`decodeNormals`) writes:

```ts
const n = new Float32Array(oct.length / 2 * 3), sign = (v: number) => (v < 0 ? -1 : 1);
for (let i = 0, j = 0; i < oct.length; i += 2, j += 3) {
  let x = oct[i] / 32767, y = oct[i + 1] / 32767; const z = 1 - Math.abs(x) - Math.abs(y);
  if (z < 0) [x, y] = [(1 - Math.abs(y)) * sign(x), (1 - Math.abs(x)) * sign(y)];
  const l = Math.hypot(x, y, z); n[j] = x / l; n[j + 1] = y / l; n[j + 2] = z / l;
}
```

One flat buffer, where a consumer still needs it, is the chunks laid end to end with each chunk's indices offset by the vertices before it, as `Uint32Array`. The core no longer builds that buffer; a consumer that does builds it in its own memory.
