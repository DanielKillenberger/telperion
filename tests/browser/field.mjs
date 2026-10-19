import { pathToFileURL } from 'node:url';

/* ------------------------------------------------------------------ *
 * THE FIELD ENTRY POINT, IN A BROWSER WORKER
 *
 * A module worker imports src/field/index.ts from the served origin,
 * lets it fetch the renderer's Wasm from beside itself (no
 * bundler-resolved URL, no source passed), grows one tree on the
 * worker's WebGPU device and reads it out to one grid (fn-257). The page never
 * touches the module: a homepage runs this off its main thread, so the
 * smoke does too. It needs a browser with WebGPU in workers.
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
      const tree = await growField('ordinary', 5, 8, { limbOrder: 1 });
      let refused = '';
      try { await growField('ordinary', 5, 2); } catch (error) { refused = String(error); }
      const { flags, limbs, leaves, woodRadius } = tree.answer;
      postMessage({ n: tree.n, cell: tree.cell, flags: [...flags], limbs: [...limbs], leaves: [...leaves], woodRadius: [...woodRadius], refused, noLimb: NO_LIMB });`;
    const worker = new Worker(URL.createObjectURL(new Blob([code], { type: 'text/javascript' })), { type: 'module' });
    return await new Promise((resolve, reject) => {
      worker.onmessage = e => resolve(e.data);
      worker.onerror = e => reject(Error(`worker: ${e.message}`));
    });
  });
  const check = (ok, message) => { if (!ok) throw Error(message); };
  check(answer.n === 8 && answer.cell > 0 && answer.flags.length === 8 ** 3, 'the field is the requested grid');
  check(answer.flags.some(f => f === 3) && answer.woodRadius.some(r => r > 0) && answer.leaves.some(l => l > 0) && answer.limbs.some(l => l !== answer.noLimb), 'the grid carries wood, foliage, a radius, an estimate and a limb');
  check(answer.refused.includes('field grid side'), `a side of two is refused by name, got ${answer.refused}`);
  console.log(`field worker: one grid read out in the browser (${answer.flags.filter(f => f & 1).length} wood cells of ${answer.flags.length})`);
} finally {
  await browser.close();
}
