import { growField } from "telperion/field";

const tree = await growField("silver-birch", 7);
const { min, max } = tree.bounds;
const n = 8;
const cells = new Float64Array(n * n * n * 4);
const step = max.map((v, k) => (v - min[k]) / n);
let c = 0;
for (let i = 0; i < n; i++)
  for (let j = 0; j < n; j++)
    for (let k = 0; k < n; k++) {
      cells[c++] = min[0] + (i + 0.5) * step[0];
      cells[c++] = min[1] + (j + 0.5) * step[1];
      cells[c++] = min[2] + (k + 0.5) * step[2];
      cells[c++] = Math.max(...step) / 2;
    }
const a = tree.query(cells);
const count = (f) => a.flags.reduce((s, x) => s + ((x & f) ? 1 : 0), 0);
const leaves = a.leaves.reduce((s, x) => s + x, 0);
const limbs = new Set(a.limbs.filter((x, i) => a.flags[i] & 2)).size;
console.log(JSON.stringify({
  node: process.version,
  bounds: tree.bounds,
  cells: n ** 3,
  wood: count(1),
  foliage: count(2),
  maxWoodRadius: Math.max(...a.woodRadius),
  leaves,
  limbSystems: limbs,
}));
tree.release();
