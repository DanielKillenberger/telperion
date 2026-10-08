// fn-212 D6: grow engine trees in Node through the Wasm build.
//   node wasm-grow.mjs <telperion_wasm.wasm> [age]
import { readFile } from 'node:fs/promises';
const [file, ageArg] = process.argv.slice(2);
const age = Number(ageArg ?? 20);
const { instance } = await WebAssembly.instantiate(await readFile(file), { env: { now: () => performance.now() } });
const e = instance.exports;
const text = (p, n) => new TextDecoder().decode(new Uint8Array(e.memory.buffer, p, n));
if (e.catalogue()) throw Error('catalogue');
const catalogue = JSON.parse(text(e.metadata_ptr(), e.metadata_len()));
for (const id of ['european-beech', 'norway-spruce', 'oregon-white-oak', 'date-palm']) {
  const entry = catalogue.find(c => c.id === id);
  const family = { ...entry.family, age };
  const bytes = new TextEncoder().encode(JSON.stringify({ family, outputs: { surface: true, foliage: true } }));
  if (e.request_alloc(bytes.length)) throw Error('alloc');
  new Uint8Array(e.memory.buffer, e.request_ptr(), bytes.length).set(bytes);
  const t = performance.now();
  let status, meta;
  try {
    status = e.build();
    meta = text(e.metadata_ptr(), e.metadata_len());
  } catch (err) {
    console.log(JSON.stringify({ id, age, trapped: String(err) }));
    process.exitCode = 1;
    break; // a trapped instance is unusable
  }
  console.log(JSON.stringify({ id, age, status, ms: Math.round(performance.now() - t), wood: e.buffer_len(0), meta: meta.slice(0, 200) }));
  e.release();
}
