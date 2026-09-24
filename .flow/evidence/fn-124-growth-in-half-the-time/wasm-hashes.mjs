// SHA-256 of every CPU output array the core module builds for the four
// fixtures (structure, wood, foliage), so two builds of the module compare
// byte for byte. Run from the checkout whose src/browser/telperion.wasm is
// measured; prints one JSON line per fixture.
import { createServer } from 'vite';
import { chromium } from 'playwright';
const server = await createServer({ server: { host: '127.0.0.1', port: 0 } });
await server.listen();
const url = `http://127.0.0.1:${server.httpServer.address().port}`;
const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_EXECUTABLE || '/usr/bin/chromium', args: ['--no-sandbox'] });
try {
  const page = await browser.newPage();
  await page.route(url + '/', r => r.fulfill({ contentType: 'text/html', body: '<p></p>' }));
  await page.goto(url);
  for (const preset of ['oregon-white-oak', 'norway-spruce']) for (const seed of [1, 7]) {
    const row = await page.evaluate(async ({ preset, seed }) => {
      const { TreeEngine, presetById } = await import('/src/browser/core.ts');
      const { familyJson, presetToParams } = await import('/harness/family.ts');
      const engine = await TreeEngine.create();
      const built = engine.build(JSON.parse(familyJson({ ...presetToParams(presetById(preset)), seed })), { surface: true, foliage: true, structure: true });
      const hex = async a => [...new Uint8Array(await crypto.subtle.digest('SHA-256', a))].map(b => b.toString(16).padStart(2, '0')).join('');
      const arrays = { structure: built.structure.values, topology: built.structure.topology, woodPositions: built.surface.positions, woodIndices: built.surface.indices, leafPositions: built.foliage.positions, leaves: built.foliage.leaves };
      const out = { preset, seed };
      for (const [k, v] of Object.entries(arrays)) out[k] = (await hex(v)).slice(0, 16);
      engine.release(); engine.dispose();
      return out;
    }, { preset, seed });
    console.log(JSON.stringify(row));
  }
} finally { await browser.close(); await server.close(); }
