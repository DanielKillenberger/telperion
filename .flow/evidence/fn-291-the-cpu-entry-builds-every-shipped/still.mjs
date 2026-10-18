// fn-291 task 4: a still of the wood one source hands over, drawn by a
// software rasterizer: orthographic from +z, y up, depth-tested, each
// triangle flat-shaded (Lambert) by the mean of its three vertex normals.
// Two panels: the whole tree, and the trunk's foot at a twelfth of its height.
//   node still.mjs dist <dist-dir> <preset> <seed> <out.png> <camera.json>
//   node still.mjs native <flatmesh-base binary> <preset> <seed> <out.png> <camera.json>
//   node still.mjs compose <before.png> <after.png> <out.png>
// The camera is read from <camera.json> when it exists, else taken from the
// source's bounds and written there, so a pair shares one camera.
import { spawn } from 'node:child_process';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { deflateSync, inflateSync, crc32 } from 'node:zlib';
import { distWood, streamWood, identity } from './wood-source.mjs';

const SIZE = 768;
const LIGHT = (([x, y, z]) => { const l = Math.hypot(x, y, z); return [x / l, y / l, z / l]; })([-0.45, 0.55, 0.7]);

function cameras({ min, max }) {
  const height = max[1] - min[1];
  return [
    { cx: (min[0] + max[0]) / 2, cy: (min[1] + max[1]) / 2, half: Math.max(max[0] - min[0], height) / 2 * 1.04 },
    { cx: 0, cy: min[1] + height / 24, half: height / 24 },
  ];
}

function canvas(cam) {
  return { cam, depth: new Float32Array(SIZE * SIZE).fill(-Infinity), shade: new Uint8Array(SIZE * SIZE).fill(255) };
}

/** One triangle into one canvas: pixel centres inside it, nearest wins. */
function draw(cv, P, N, a, b, c) {
  const { cx, cy, half } = cv.cam, s = SIZE / 2 / half, o = SIZE / 2;
  const ax = (P[3 * a] - cx) * s + o, ay = o - (P[3 * a + 1] - cy) * s;
  const bx = (P[3 * b] - cx) * s + o, by = o - (P[3 * b + 1] - cy) * s;
  const qx = (P[3 * c] - cx) * s + o, qy = o - (P[3 * c + 1] - cy) * s;
  const x0 = Math.max(0, Math.ceil(Math.min(ax, bx, qx) - 0.5)), x1 = Math.min(SIZE - 1, Math.floor(Math.max(ax, bx, qx) - 0.5));
  const y0 = Math.max(0, Math.ceil(Math.min(ay, by, qy) - 0.5)), y1 = Math.min(SIZE - 1, Math.floor(Math.max(ay, by, qy) - 0.5));
  if (x0 > x1 || y0 > y1) return;
  const area = (bx - ax) * (qy - ay) - (by - ay) * (qx - ax);
  if (area === 0) return;
  const az = P[3 * a + 2], bz = P[3 * b + 2], qz = P[3 * c + 2];
  let value = -1;
  for (let py = y0; py <= y1; py++) {
    const y = py + 0.5;
    for (let px = x0; px <= x1; px++) {
      const x = px + 0.5;
      const wa = ((bx - x) * (qy - y) - (by - y) * (qx - x)) / area;
      const wb = ((qx - x) * (ay - y) - (qy - y) * (ax - x)) / area;
      const wc = 1 - wa - wb;
      if (wa < 0 || wb < 0 || wc < 0) continue;
      const z = wa * az + wb * bz + wc * qz, k = py * SIZE + px;
      if (z <= cv.depth[k]) continue;
      if (value < 0) {
        const nx = N[3 * a] + N[3 * b] + N[3 * c], ny = N[3 * a + 1] + N[3 * b + 1] + N[3 * c + 1], nz = N[3 * a + 2] + N[3 * b + 2] + N[3 * c + 2];
        const l = Math.hypot(nx, ny, nz) || 1;
        value = Math.round(40 + 180 * Math.max(0, (nx * LIGHT[0] + ny * LIGHT[1] + nz * LIGHT[2]) / l));
      }
      cv.depth[k] = z;
      cv.shade[k] = value;
    }
  }
}

function render(wood, cameraPath) {
  const cams = existsSync(cameraPath) ? JSON.parse(readFileSync(cameraPath, 'utf8')) : cameras(wood.bounds);
  if (!existsSync(cameraPath)) writeFileSync(cameraPath, JSON.stringify(cams) + '\n');
  const views = cams.map(canvas);
  for (const piece of wood.pieces) {
    const { positions, normals, indices } = piece();
    for (let t = 0; t < indices.length; t += 3) {
      for (const cv of views) draw(cv, positions, normals, indices[t], indices[t + 1], indices[t + 2]);
    }
  }
  const image = new Uint8Array(2 * SIZE * SIZE);
  for (let y = 0; y < SIZE; y++) {
    for (const [v, cv] of views.entries()) image.set(cv.shade.subarray(y * SIZE, (y + 1) * SIZE), y * 2 * SIZE + v * SIZE);
  }
  return { width: 2 * SIZE, height: SIZE, pixels: image };
}

function png({ width, height, pixels }) {
  const chunk = (type, data) => {
    const out = Buffer.alloc(12 + data.length);
    out.writeUInt32BE(data.length, 0);
    out.write(type, 4, 'latin1');
    data.copy(out, 8);
    out.writeUInt32BE(crc32(out.subarray(4, 8 + data.length)), 8 + data.length);
    return out;
  };
  const head = Buffer.alloc(13);
  head.writeUInt32BE(width, 0); head.writeUInt32BE(height, 4); head[8] = 8; head[9] = 0;
  const rows = Buffer.alloc((width + 1) * height);
  for (let y = 0; y < height; y++) rows.set(pixels.subarray(y * width, (y + 1) * width), y * (width + 1) + 1);
  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  return Buffer.concat([signature, chunk('IHDR', head), chunk('IDAT', deflateSync(rows)), chunk('IEND', Buffer.alloc(0))]);
}

/** A PNG this tool wrote: 8-bit grey, filter 0 on every row. */
function unpng(bytes) {
  const width = bytes.readUInt32BE(16), height = bytes.readUInt32BE(20), idat = [];
  for (let at = 8; at < bytes.length;) {
    const n = bytes.readUInt32BE(at), type = bytes.toString('latin1', at + 4, at + 8);
    if (type === 'IDAT') idat.push(bytes.subarray(at + 8, at + 8 + n));
    at += 12 + n;
  }
  const rows = inflateSync(Buffer.concat(idat)), pixels = new Uint8Array(width * height);
  for (let y = 0; y < height; y++) {
    if (rows[y * (width + 1)] !== 0) throw Error('a row with a filter this tool never writes');
    pixels.set(rows.subarray(y * (width + 1) + 1, (y + 1) * (width + 1)), y * width);
  }
  return { width, height, pixels };
}

/** Before over after over their difference (eight times, inverted), and
 * the difference counted per panel. */
function compose(before, after) {
  const { width, height } = before;
  if (after.width !== width || after.height !== height) throw Error('the two stills differ in size');
  const pixels = new Uint8Array(width * height * 3);
  pixels.set(before.pixels, 0);
  pixels.set(after.pixels, width * height);
  const stats = [0, 1].map(() => ({ differ: 0, over8: 0, max: 0, sum: 0 }));
  for (let k = 0; k < width * height; k++) {
    const d = Math.abs(before.pixels[k] - after.pixels[k]), st = stats[(k % width) < SIZE ? 0 : 1];
    pixels[2 * width * height + k] = 255 - Math.min(255, 8 * d);
    st.sum += d; st.max = Math.max(st.max, d);
    if (d > 0) st.differ += 1;
    if (d > 8) st.over8 += 1;
  }
  const panel = SIZE * SIZE;
  const summary = ['whole', 'foot'].map((name, v) => {
    const { differ, over8, max, sum } = stats[v];
    return `${name}: ${differ} px differ (${(100 * differ / panel).toFixed(3)}%), ${over8} by more than 8 levels, max ${max}, mean ${(sum / panel).toFixed(4)}`;
  });
  return { image: { width, height: 3 * height, pixels }, summary };
}

/** The base native mesh, read from the driver's stdout as it writes it. */
async function native(binary, id, seed) {
  const child = spawn(binary, [id, seed], { stdio: ['ignore', 'pipe', 'inherit'] });
  const exited = new Promise(done => child.on('close', done));
  const wood = await streamWood(child.stdout);
  const code = await exited;
  if (code !== 0) throw Error(`${binary} ${id} ${seed} exited ${code}`);
  return wood;
}

const [mode, ...args] = process.argv.slice(2);
if (mode === 'compose') {
  const { image, summary } = compose(unpng(readFileSync(args[0])), unpng(readFileSync(args[1])));
  writeFileSync(args[2], png(image));
  console.log(`${args[2]}: ${summary.join('; ')}`);
} else {
  const wood = mode === 'dist' ? await distWood(args[0], args[1], Number(args[2])) : await native(...args.slice(0, 3));
  const [out, cameraPath] = args.slice(3);
  const started = performance.now();
  writeFileSync(out, png(render(wood, cameraPath)));
  const id = identity(wood);
  console.log(JSON.stringify({ out, form: wood.form, ...id, nodes: wood.nodes, memoryBytes: wood.memoryBytes, renderMs: Math.round(performance.now() - started) }));
}
