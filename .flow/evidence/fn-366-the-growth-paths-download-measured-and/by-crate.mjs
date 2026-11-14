import { execFileSync } from 'node:child_process';

/* Raw code bytes of an unstripped telperion.wasm by crate and by the classes
 * fn-366 asks about, from `twiggy top` (installed on the desk; nothing
 * installed for this). Shallow bytes, so a function counts once.
 *
 *   node by-crate.mjs <unstripped module.wasm> */
const rows = execFileSync('twiggy', ['top', '-n', '100000', '-f', 'csv', process.argv[2]], { encoding: 'utf8', maxBuffer: 1 << 28 })
  .trim().split('\n').slice(1).map(line => {
    const m = /^"?(.*?)"?,(\d+),([\d.]+)/.exec(line) ?? /^(.*),(\d+),([\d.]+)/.exec(line);
    return m ? { name: m[1], bytes: Number(m[2]) } : null;
  }).filter(Boolean);
const classes = [
  ['serde_json and serde (request parse, metadata JSON)', /serde_json|serde_core|serde\[/],
  ['formatting (core::fmt, float and integer printing)', /core\[[0-9a-f]+\]::fmt|::fmt::|<.* as core\[[0-9a-f]+\]::fmt::|float_to_decimal|flt2dec|ryu|itoa/],
  ['rayon and crossbeam', /rayon|crossbeam/],
  ['catalogue', /telperion_core\[[0-9a-f]+\]::catalogue/],
  ['presets', /telperion_core\[[0-9a-f]+\]::presets|::params::/],
  ['pipeline::foliage', /pipeline::foliage/],
  ['pipeline::surface', /pipeline::surface/],
  ['telperion_space::geometry', /telperion_space\[[0-9a-f]+\]::geometry/],
  ['closed_form', /closed_form/],
];
const sums = new Map(classes.map(([k]) => [k, 0]));
for (const { name, bytes } of rows) {
  const hit = classes.find(([, re]) => re.test(name));
  if (hit) sums.set(hit[0], sums.get(hit[0]) + bytes);
}
for (const [k, v] of sums) console.log(`${k}: ${v}`);
