// Runs the Wasm binding's build for families x output combinations, one fresh
// instance per build, and prints per build: a hash of every output buffer and
// of the metadata without its timings, the linear memory after the build (its
// peak: Wasm memory never shrinks), and the binding's own stage timings.
// usage: node binding.mjs <wasm> <families.jsonl> <id|all> [repeats] [combos]
import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
const [wasmPath, familiesPath, only = 'all', repeatArg = '1', comboArg = 'all'] = process.argv.slice(2);
const repeats = Number(repeatArg);
const module = await WebAssembly.compile(await readFile(wasmPath));
// The family wire goes on as text: its large integers do not survive a JS number.
const families = (await readFile(familiesPath, 'utf8')).trim().split('\n')
  .map(l => ({ ...JSON.parse(l), wire: l.slice('{"family":'.length, l.lastIndexOf(',"id":')).replaceAll('18446744073709551615', '4294967295') }))
  .filter(f => only === 'all' || f.id === only);
const keys = ['surface', 'foliage', 'structure', 'field'];
const combos = [];
for (let m = 1; m < 16; m++) combos.push(Object.fromEntries(keys.map((k, i) => [k, Boolean(m & (1 << i))])));
const chosen = comboArg === 'all' ? combos : comboArg.split(',').map(c => combos[Number(c) - 1]);
const slots = [0, 1, 2, 3, 4, 5, 6, 7, 14, 15];
for (const f of families) for (const outputs of chosen) for (let r = 0; r < repeats; r++) {
  const { exports: e } = await WebAssembly.instantiate(module, { env: { now: () => performance.now() } });
  const request = new TextEncoder().encode(`{"family":${f.wire},"outputs":${JSON.stringify(outputs)}}`);
  if (e.request_alloc(request.length)) throw Error('alloc');
  new Uint8Array(e.memory.buffer, e.request_ptr(), request.length).set(request);
  const code = e.build();
  const meta = JSON.parse(new TextDecoder().decode(new Uint8Array(e.memory.buffer, e.metadata_ptr(), e.metadata_len())));
  if (code) { console.error(f.id, JSON.stringify(meta)); process.exit(1); }
  const timings = meta.timings; const marks = meta.marks; delete meta.marks; delete meta.timings; delete meta.revision;
  const hash = createHash('sha256');
  hash.update(JSON.stringify(meta));
  for (const s of slots) {
    const len = e.buffer_len(s), ptr = e.buffer_ptr(s);
    const width = [5, 6].includes(s) ? (s === 6 ? 8 : 4) : 4;
    hash.update(`|${s}:${len}|`);
    if (len) hash.update(new Uint8Array(e.memory.buffer, ptr, len * width));
  }
  console.log(JSON.stringify({ id: f.id, seed: f.seed, outputs: Object.keys(outputs).filter(k => outputs[k]).join('+'),
    code, hash: hash.digest('hex').slice(0, 16), memory: e.memory.buffer.byteLength, timings, marks, repeat: r }));
}
