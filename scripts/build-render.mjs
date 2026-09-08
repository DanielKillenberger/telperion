import { execFileSync } from 'node:child_process';
import { readFile } from 'node:fs/promises';

/* The renderer, as the page loads it: a wasm module and the glue that calls
 * into it. The glue is generated here and gitignored, so nothing in the repo
 * is authored untyped JavaScript - `--target web` emits its own declarations
 * and the browser adapter is typed off them.
 *
 * The CLI and the crate must be the same wasm-bindgen. A mismatch produces
 * glue that calls functions the module does not export, and the failure lands
 * at run time in the browser as a name error with no cause attached. So it is
 * checked here, before the module is generated, and the fix is printed as the
 * command to run. */
const root = new URL('../', import.meta.url);
const OUT = 'src/browser/render';

const lock = await readFile(new URL('Cargo.lock', root), 'utf8');
const wanted = /\[\[package\]\]\nname = "wasm-bindgen"\nversion = "([^"]+)"/.exec(lock)?.[1];
if (wanted === undefined) throw Error('Cargo.lock pins no wasm-bindgen version');
const install = `cargo install wasm-bindgen-cli --version ${wanted}`;

let found;
try {
  found = execFileSync('wasm-bindgen', ['--version'], { encoding: 'utf8' }).trim().split(/\s+/).at(-1);
} catch {
  console.error(`wasm-bindgen is not installed. The crate pins ${wanted}:\n  ${install}`);
  process.exit(1);
}
if (found !== wanted) {
  console.error(`wasm-bindgen ${found} is installed, the crate pins ${wanted}:\n  ${install}`);
  process.exit(1);
}

execFileSync('cargo', ['build', '--release', '--target', 'wasm32-unknown-unknown', '-p', 'telperion-render'],
  { cwd: root, stdio: 'inherit', timeout: 900_000 });
execFileSync('wasm-bindgen', ['--target', 'web', '--out-dir', OUT,
  'target/wasm32-unknown-unknown/release/telperion_render.wasm'],
  { cwd: root, stdio: 'inherit', timeout: 300_000 });
console.log(`Built the renderer module and its glue into ${OUT}/.`);
