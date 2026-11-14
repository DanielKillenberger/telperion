import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join } from 'node:path';

/* What `.rodata` holds in a built telperion.wasm, by string class (fn-366 R1).
 *
 *   node rodata.mjs <unstripped telperion_wasm.wasm> <repo root>
 *
 * Reads the data segments and their names from the module's name section,
 * then marks the bytes of `.rodata` that every string literal of the three
 * crates' non-test sources covers, classed by the literal's form, and the
 * panic locations and dependency strings found in place. What no class
 * covers is split into printable runs and binary (numeric tables, vtables,
 * float constants). Bytes are counted once, in the first class that marks
 * them. */
const [wasmPath, repo] = process.argv.slice(2);
const bytes = readFileSync(wasmPath);
let at = 8;
const leb = () => {
  let v = 0, shift = 0, b;
  do { b = bytes[at++]; v += (b & 0x7f) * 2 ** shift; shift += 7; } while (b & 0x80);
  return v;
};
const segments = [], names = new Map();
while (at < bytes.length) {
  const id = bytes[at++], size = leb(), end = at + size;
  if (id === 11) {
    for (let n = leb(), i = 0; i < n; i++) {
      const flag = leb();
      if (flag !== 0) throw Error(`data segment flag ${flag}`);
      if (bytes[at++] !== 0x41) throw Error('offset not i32.const');
      leb(); at++; // offset value (signed, small here) and `end`
      const len = leb();
      segments.push(bytes.subarray(at, at + len));
      at += len;
    }
  } else if (id === 0) {
    const nlen = leb(), name = bytes.subarray(at, at + nlen).toString(); at += nlen;
    if (name === 'name') {
      while (at < end) {
        const sub = bytes[at++], ssize = leb(), send = at + ssize;
        if (sub === 9) for (let n = leb(), i = 0; i < n; i++) {
          const idx = leb(), l = leb();
          names.set(idx, bytes.subarray(at, at + l).toString()); at += l;
        }
        at = send;
      }
    }
  }
  at = end;
}
const index = [...names].find(([, n]) => n === '.rodata')?.[0];
if (index === undefined) throw Error('no .rodata segment name; pass the unstripped module');
const ro = segments[index];
console.log(`segments: ${[...names].map(([i, n]) => `${n} ${segments[i].length}`).join(', ')}`);

const sources = [];
const walk = dir => {
  for (const f of readdirSync(dir)) {
    const p = join(dir, f);
    if (statSync(p).isDirectory()) { if (!/tests?$|suite$/.test(f)) walk(p); }
    else if (p.endsWith('.rs') && !/tests?\.rs$/.test(f)) sources.push(p);
  }
};
for (const c of ['telperion-core/src', 'telperion-space/src', 'telperion-wasm/src']) walk(join(repo, 'crates', c));
const literals = new Set();
for (const p of sources) {
  for (const m of readFileSync(p, 'utf8').matchAll(/"((?:[^"\\\n]|\\.){4,})"/g)) {
    literals.add(m[1].replace(/\\n/g, '\n').replace(/\\"/g, '"').replace(/\\\\/g, '\\'));
  }
}
const covered = new Uint8Array(ro.length);
const totals = new Map();
const mark = (cls, needle) => {
  const n = Buffer.from(needle);
  let won = 0;
  for (let i = ro.indexOf(n); i !== -1; i = ro.indexOf(n, i + 1)) {
    for (let k = i; k < i + n.length; k++) if (!covered[k]) { covered[k] = 1; won++; }
  }
  totals.set(cls, (totals.get(cls) ?? 0) + won);
};
const classOf = s =>
  /^\/[a-z][\w]*(\/[\w]+)+$/.test(s) ? 'parameter paths' :
  /\{[\w:.?]*\}/.test(s) ? 'format strings' :
  /\s/.test(s) && s.length >= 24 ? 'prose (docs, notes, refusals)' :
  'labels, keys, short messages';
// Longest first, so a literal inside a longer one is not counted apart.
for (const s of [...literals].sort((a, b) => b.length - a.length)) mark(classOf(s), s);
// Panic locations and dependency strings sit in place, not as our literals.
const text = ro.toString('latin1');
for (const m of text.matchAll(/(?:\/rustc\/[0-9a-f]+\/|\/home\/[^\0]*?\/\.cargo\/registry\/src\/[^/]+\/|crates\/telperion-[a-z]+\/)[\w/.-]+?\.rs/g)) mark('panic locations (source paths)', m[0]);
let printable = 0, run = 0;
for (let i = 0; i <= ro.length; i++) {
  const b = ro[i];
  if (i < ro.length && !covered[i] && b >= 0x20 && b < 0x7f) { run++; continue; }
  if (run >= 6) printable += run;
  run = 0;
}
totals.set('other printable runs (6+ bytes)', printable);
const markedTotal = [...totals.values()].reduce((a, b) => a + b, 0);
totals.set('binary (numeric tables, vtables, constants)', ro.length - markedTotal);
console.log(`.rodata ${ro.length} bytes`);
for (const [cls, n] of [...totals].sort((a, b) => b[1] - a[1])) console.log(`  ${cls}: ${n} (${(100 * n / ro.length).toFixed(1)}%)`);
