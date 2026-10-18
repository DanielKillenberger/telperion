// fn-291 task 4: the wood one source hands over, as pieces of positions,
// float normals and indices, so the still and the surface check read the
// chunked entry, the flat base entry and the base native mesh the same way.
import { pathToFileURL } from 'node:url';
import { resolve } from 'node:path';
import { createHash } from 'node:crypto';

/** One preset and seed through a dist's CPU entry, surface and foliage, as
 * the probe builds it. A chunked dist hands its chunks with their normals
 * decoded; a flat (base) dist hands one piece. */
export async function distWood(dir, id, seed) {
  const entry = await import(pathToFileURL(resolve(dir, 'telperion.js')).href);
  const family = structuredClone(entry.PRESETS.find(p => p.id === id));
  family.skeleton.seed = seed;
  const engine = await entry.TreeEngine.create();
  const tree = engine.build(family, { surface: true, foliage: true });
  const memoryBytes = engine.memoryBytes;
  engine.dispose();
  const s = tree.surface;
  const pieces = s.chunks
    ? s.chunks.map(c => () => ({ positions: c.positions, indices: c.indices, get normals() { return entry.decodeNormals(c.normals); } }))
    : [() => s];
  const form = s.chunks ? `dist ${s.chunks.length} chunks` : 'dist flat';
  return { form, bounds: s.bounds, nodes: tree.diagnostics.nodes, memoryBytes, pieces };
}

/** The base native flat mesh `flatmesh-base.rs` writes: vertex and index
 * counts as u32, positions and normals as f32, indices as u32. */
export async function streamWood(input) {
  const head = new Uint32Array(2);
  let parts, at = 0, part = -1;
  const target = () => (part < 0 ? new Uint8Array(head.buffer) : parts[part]);
  for await (const buf of input) {
    let from = 0;
    while (from < buf.length) {
      if (part >= 0 && part >= parts.length) throw Error('more bytes than the header states');
      const t = target(), n = Math.min(t.length - at, buf.length - from);
      t.set(buf.subarray(from, from + n), at);
      at += n; from += n;
      if (at === t.length) {
        if (part < 0) {
          const [v, i] = head;
          parts = [new Float32Array(3 * v), new Float32Array(3 * v), new Uint32Array(i)].map(a => new Uint8Array(a.buffer));
        }
        part += 1; at = 0;
      }
    }
  }
  if (!parts || part !== parts.length) throw Error('the native mesh stream ended early');
  const [positions, normals] = parts.slice(0, 2).map(p => new Float32Array(p.buffer));
  const indices = new Uint32Array(parts[2].buffer);
  const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
  for (let k = 0; k < positions.length; k++) {
    const axis = k % 3;
    if (positions[k] < min[axis]) min[axis] = positions[k];
    if (positions[k] > max[axis]) max[axis] = positions[k];
  }
  return { form: 'native flat', bounds: { min, max }, pieces: [() => ({ positions, normals, indices })] };
}

/** The wood's identity across forms: vertex and triangle counts, and a hash
 * of the positions and of the indices made absolute, in draw order. */
export function identity(wood) {
  const pos = createHash('sha256'), idx = createHash('sha256');
  let vertices = 0, triangles = 0;
  for (const piece of wood.pieces) {
    const { positions, indices } = piece();
    pos.update(new Uint8Array(positions.buffer, positions.byteOffset, positions.byteLength));
    const absolute = vertices === 0 && indices instanceof Uint32Array ? indices : Uint32Array.from(indices, i => i + vertices);
    idx.update(new Uint8Array(absolute.buffer, absolute.byteOffset, absolute.byteLength));
    vertices += positions.length / 3;
    triangles += indices.length / 3;
  }
  return { vertices, triangles, positions: pos.digest('hex').slice(0, 16), indices: idx.digest('hex').slice(0, 16) };
}
