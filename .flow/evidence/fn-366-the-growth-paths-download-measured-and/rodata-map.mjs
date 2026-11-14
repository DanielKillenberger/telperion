import { readFileSync } from 'node:fs';

/* `.rodata` by symbol, from a wasm-ld link map of the binding (fn-366 R1).
 *
 *   RUSTFLAGS='-C link-arg=-Map=<file>' cargo build --release \
 *     --target wasm32-unknown-unknown -p telperion-wasm
 *   node rodata-map.mjs <file> [top = 25]
 *
 * Each input section of `.rodata` is counted under the symbol it holds:
 * a named constant by its crate and path (hash and generics dropped), an
 * anonymous constant (`.Lanon`: string literals, panic locations, small
 * tables) and the linker's merged strings apart. */
const [file, top = '25'] = process.argv.slice(2);
const lines = readFileSync(file, 'utf8').split('\n');
const sums = new Map();
let on = false, pending = 0;
const add = (k, n) => sums.set(k, (sums.get(k) ?? 0) + n);
const name = sym => {
  if (sym.startsWith('.Lanon')) return 'anonymous constants (.Lanon: literals, panic locations, small tables)';
  const parts = [...sym.matchAll(/(\d+)([A-Za-z_$.][\w$.]*)/g)].map(m => m[2].slice(0, Number(m[1])));
  const path = parts.filter(p => !/^h[0-9a-f]{16}$/.test(p)).join('::');
  return path || sym;
};
for (const line of lines) {
  const m = /^\s+([0-9a-f]+|-)\s+([0-9a-f]+)\s+([0-9a-f]+)\s(\s*)(.*)$/.exec(line);
  if (!m) continue;
  const [, , , size, indent, rest] = m;
  if (indent.length === 0) { on = rest === '.rodata'; continue; }
  if (!on) continue;
  if (rest.includes(':(')) {
    if (pending) add(rest.includes('<internal>') ? 'merged strings (<internal>)' : 'unnamed', pending);
    pending = parseInt(size, 16);
    if (rest.includes('<internal>')) { add('merged strings (<internal>)', pending); pending = 0; }
  } else if (pending) {
    add(name(rest.trim()), pending);
    pending = 0;
  }
}
const total = [...sums.values()].reduce((a, b) => a + b, 0);
console.log(`.rodata input sections: ${total} bytes`);
const byCrate = new Map();
for (const [k, v] of sums) {
  const crate = /^(telperion_\w+|core|alloc|std|serde_json|serde_core|libm|rayon_core|crossbeam_\w+|dlmalloc)::/.exec(k)?.[1] ?? k;
  byCrate.set(crate, (byCrate.get(crate) ?? 0) + v);
}
console.log('by crate or class:');
for (const [k, v] of [...byCrate].sort((a, b) => b[1] - a[1]).slice(0, 12)) console.log(`  ${v}\t${k}`);
console.log(`largest ${top} symbols:`);
for (const [k, v] of [...sums].sort((a, b) => b[1] - a[1]).slice(0, Number(top))) console.log(`  ${v}\t${k}`);
