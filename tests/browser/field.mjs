import { pathToFileURL } from 'node:url';

/* ------------------------------------------------------------------ *
 * THE SLIM ENTRY POINT, IN A BROWSER WORKER
 *
 * A module worker imports src/field/index.ts from the served origin,
 * lets it fetch the slim Wasm from beside itself (no bundler-resolved
 * URL, no source passed), grows one tree and answers one query. The
 * page never touches the module: a homepage runs this off its main
 * thread, so the smoke does too.
 * ------------------------------------------------------------------ */
const { chromium } = await import(process.env.PLAYWRIGHT_MODULE ? pathToFileURL(process.env.PLAYWRIGHT_MODULE).href : 'playwright');
const url = process.env.BROWSER_URL ?? 'http://127.0.0.1:5184';
const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_EXECUTABLE, headless: true, args: ['--no-sandbox'] });
try {
  const page = await browser.newPage();
  page.setDefaultTimeout(120000);
  await page.route(url + '/', r => r.fulfill({ contentType: 'text/html', body: '<!doctype html><html><body></body></html>' }));
  await page.goto(url + '/');
  const answer = await page.evaluate(async () => {
    const code = `
      import { growField, NO_LIMB } from '${location.origin}/src/field/index.ts';
      const tree = await growField('ordinary', 5, { limbOrder: 1 });
      const { min, max } = tree.bounds;
      const centre = [0, 1, 2].map(a => (min[a] + max[a]) / 2);
      const half = Math.max(...max.map((v, a) => v - min[a])) / 2;
      const one = tree.query(new Float64Array([...centre, half]));
      let refused = false;
      try { tree.query(new Float64Array([...centre, -1])); } catch { refused = true; }
      tree.release();
      postMessage({ bounds: tree.bounds, flags: [...one.flags], limbs: [...one.limbs], leaves: [...one.leaves], woodRadius: [...one.woodRadius], refused, noLimb: NO_LIMB });`;
    const worker = new Worker(URL.createObjectURL(new Blob([code], { type: 'text/javascript' })), { type: 'module' });
    return await new Promise((resolve, reject) => {
      worker.onmessage = e => resolve(e.data);
      worker.onerror = e => reject(Error(`worker: ${e.message}`));
    });
  });
  const check = (ok, message) => { if (!ok) throw Error(message); };
  check(answer.bounds.max[1] > answer.bounds.min[1], 'the field has bounds');
  check(answer.flags.length === 1 && answer.flags[0] === 3, `the whole-tree cell holds wood and foliage, got ${answer.flags}`);
  check(answer.woodRadius[0] > 0 && answer.leaves[0] > 0 && answer.limbs[0] !== answer.noLimb, 'the cell carries a radius, an estimate and a limb');
  check(answer.refused, 'a negative half extent is refused');
  console.log(`field worker: one query answered in the browser (wood radius ${answer.woodRadius[0].toFixed(3)} m, ${answer.leaves[0].toFixed(0)} leaves estimated)`);
} finally {
  await browser.close();
}
