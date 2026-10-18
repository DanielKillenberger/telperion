// fn-291 task 4: each preset's wood from the chunked dist against the flat
// base dist. Positions and the indices made absolute must hash equal; each
// decoded normal must lie within task 6's bound (1e-4 rad) of the base's
// float normal. Each side builds in its own child process; the base's
// normals pass between them through <scratch file>.
//   node surface-check.mjs <base-dist> <new-dist> <scratch file> <ids> <seeds>
// Exits nonzero on any mismatch or any case either side fails to build.
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { distWood, identity } from './wood-source.mjs';

const BOUND = 1e-4;

async function base(dist, id, seed, scratch) {
  const wood = await distWood(dist, id, seed);
  const { normals } = wood.pieces[0]();
  writeFileSync(scratch, new Uint8Array(normals.buffer, normals.byteOffset, normals.byteLength));
  console.log(JSON.stringify(identity(wood)));
}

async function chunked(dist, id, seed, scratch) {
  const wood = await distWood(dist, id, seed);
  const file = readFileSync(scratch), flat = new Float32Array(file.buffer, file.byteOffset, file.byteLength / 4);
  let at = 0, worst = 0;
  for (const piece of wood.pieces) {
    const n = piece().normals;
    for (let k = 0; k < n.length; k += 3, at += 3) {
      const [ax, ay, az] = [flat[at], flat[at + 1], flat[at + 2]], [bx, by, bz] = [n[k], n[k + 1], n[k + 2]];
      const cross = Math.hypot(ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx);
      worst = Math.max(worst, Math.atan2(cross, ax * bx + ay * by + az * bz));
    }
  }
  if (at !== flat.length) throw Error(`${at / 3} chunked normals for ${flat.length / 3} flat ones`);
  console.log(JSON.stringify({ ...identity(wood), worstRad: worst, memoryBytes: wood.memoryBytes }));
}

const args = process.argv.slice(2);
if (args[0] === '--base') await base(args[1], args[2], Number(args[3]), args[4]);
else if (args[0] === '--chunked') await chunked(args[1], args[2], Number(args[3]), args[4]);
else {
  const [baseDist, newDist, scratch, ids, seeds] = args, self = fileURLToPath(import.meta.url);
  const child = (...a) => JSON.parse(execFileSync(process.execPath, [self, ...a], { encoding: 'utf8', maxBuffer: 1 << 20 }).trim().split('\n').pop());
  let failed = false;
  for (const id of ids.split(',')) {
    for (const seed of seeds.split(',')) {
      const was = child('--base', baseDist, id, seed, scratch), now = child('--chunked', newDist, id, seed, scratch);
      const same = ['vertices', 'triangles', 'positions', 'indices'].every(k => was[k] === now[k]);
      const ok = same && now.worstRad <= BOUND;
      failed ||= !ok;
      console.log(`${id} seed ${seed}: ${now.vertices} vertices, ${now.triangles} triangles; positions ${now.positions} ${was.positions === now.positions ? '=' : '!='} base; indices ${now.indices} ${was.indices === now.indices ? '=' : '!='} base; worst normal ${now.worstRad.toExponential(3)} rad${ok ? '' : ' FAIL'}`);
    }
  }
  process.exitCode = failed ? 1 : 0;
}
