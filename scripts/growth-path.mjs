import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { brotliCompressSync, constants } from 'node:zlib';

/* What a field page downloads to grow a tree from parameters it holds
 * (fn-366; P11, owner 1 Nov: "At most 250 KB", Brotli): the field entry,
 * every script it imports statically, and the core's module it grows on.
 * Neither `presets.js` nor `telperion-render.wasm` is on it (the host's
 * reading, fn-366). Reads a built dist and builds nothing.
 *
 *   node scripts/growth-path.mjs [dist]
 */
const root = new URL('../', import.meta.url);
export const ENTRY = 'field.js';
export const MODULE = 'telperion.wasm';

/**
 * @typedef {{ file: string, raw: number, brotli: number }} Row
 * @typedef {{ rows: Row[], raw: number, brotli: number }} Path
 */

/** Brotli at quality 11 over the largest window, as a server's static
 * precompression ships it.
 * @param {Uint8Array} bytes */
export const brotli = bytes =>
  brotliCompressSync(bytes, {
    params: {
      [constants.BROTLI_PARAM_QUALITY]: 11,
      [constants.BROTLI_PARAM_LGWIN]: 24,
      [constants.BROTLI_PARAM_SIZE_HINT]: bytes.length,
    },
  }).length;

const STATIC = /(?:^|[;}\s])(?:import|export)\s*(?:[\w*{}\s,$]*?\s*from\s*)?["'](\.\/[\w.-]+\.js)["']/g;

/** The relative scripts `code` imports or re-exports statically; a dynamic
 * `import()` is not followed.
 * @param {string} code */
export function imports(code) {
  return [...code.matchAll(STATIC)].map(m => m[1].slice(2));
}

/** The files on the path, the entry first, each once.
 * @param {(file: string) => Promise<string>} read */
export async function files(read) {
  const seen = [ENTRY];
  for (let i = 0; i < seen.length; i++) {
    for (const f of imports(await read(seen[i]))) if (!seen.includes(f)) seen.push(f);
  }
  const named = await Promise.all(seen.map(async f => (await read(f)).includes(`"./${MODULE}"`)));
  if (!named.some(Boolean)) throw Error(`no script on the field path names ./${MODULE}`);
  return [...seen, MODULE];
}

/** Raw and Brotli bytes of every file on the path in `dir`, and the totals.
 * @param {URL} dir
 * @returns {Promise<Path>} */
export async function measure(dir) {
  const rows = [];
  for (const file of await files(f => readFile(new URL(f, dir), 'utf8'))) {
    const bytes = await readFile(new URL(file, dir));
    rows.push({ file, raw: bytes.length, brotli: brotli(bytes) });
  }
  const sum = (/** @type {'raw' | 'brotli'} */ k) => rows.reduce((s, r) => s + r[k], 0);
  return { rows, raw: sum('raw'), brotli: sum('brotli') };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const dir = new URL(`${(process.argv[2] ?? 'dist').replace(/\/?$/, '/')}`, root);
  const path = await measure(dir);
  for (const { file, raw, brotli: b } of path.rows) console.log(`${file}: ${raw} raw, ${b} Brotli`);
  console.log(`growth path: ${path.brotli} Brotli bytes (${path.raw} raw) over ${path.rows.length} files`);
}
