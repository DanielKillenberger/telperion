import { pathToFileURL } from 'node:url';
import { readFile, writeFile, mkdir } from 'node:fs/promises';
import { createReadStream } from 'node:fs';
import { createServer, request } from 'node:http';
import { resolve, basename } from 'node:path';
const { chromium } = await import(process.env.PLAYWRIGHT_MODULE ? pathToFileURL(process.env.PLAYWRIGHT_MODULE).href : 'playwright');
const upstream = process.env.BROWSER_URL ?? 'http://127.0.0.1:5184';
const out = process.env.BROWSER_EVIDENCE ?? '/tmp/fn8-browser';
const reference = process.env.REFERENCE_DIR;
if (!reference) throw Error('Set REFERENCE_DIR to the exported final FN6 surface/foliage fixtures');
await mkdir(out, { recursive: true });
// A single test origin serves both application modules and streamed fixtures.
const server = createServer((req, res) => {
  if (req.url.startsWith('/reference/')) {
    const stream = createReadStream(resolve(reference, basename(req.url)));
    stream.on('error', () => { res.statusCode = 404; res.end(); });
    stream.pipe(res);
    return;
  }
  const proxy = request(new URL(req.url, upstream), { headers: { ...req.headers, host: new URL(upstream).host } }, response => {
    res.writeHead(response.statusCode, response.headers);
    response.pipe(res);
  });
  proxy.on('error', () => { res.statusCode = 502; res.end(); });
  req.pipe(proxy);
});
await new Promise(r => server.listen(0, '127.0.0.1', r));
const url = `http://127.0.0.1:${server.address().port}`;
const referenceUrl = url + '/reference';
const results = [];
try {
  for (const subject of (process.env.SUBJECTS ?? 'ordinary,telperion,laurelin,comparison').split(',')) {
    const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_EXECUTABLE, headless: process.env.BROWSER_HEADED !== '1',
      args: ['--no-sandbox', '--ozone-platform=x11', '--use-angle=gl', '--enable-gpu', '--ignore-gpu-blocklist', '--disable-software-rasterizer'] });
    try {
      const page = await browser.newPage({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
      page.setDefaultTimeout(120000);
      await page.route(url + '/', r => r.fulfill({ contentType: 'text/html', body: '<!doctype html><html><body style="margin:0;width:100vw;height:100vh;overflow:hidden"><canvas style="display:block;width:100%;height:100%"></canvas></body></html>' }));
      await page.goto(url + '/');
      const result = await page.evaluate(async ({ subject, referenceUrl }) => {
        const check = (ok, message) => { if (!ok) throw Error(message); };
        const { TreeEngine, initializeTreeCore, ORDINARY, TELPERION, LAURELIN, PRESETS } = await import('/src/browser/core.ts');
        const THREE = await import('/node_modules/.vite/deps/three.js');
        const { createStage } = await import('/harness/stage.ts');
        const { buildTree, buildComparison, presetToParams } = await import('/harness/skeleton-view.ts');
        const { DEFAULT_PARAMS } = await import('/harness/params.ts');
        const stage = createStage(document.querySelector('canvas')); window.rig = stage; stage.setPixelRatio(1);
        let stats;
        await initializeTreeCore();
        if (subject === 'comparison') {
          stage.setTree(clay => { const b = buildComparison(PRESETS, clay); stats = b.stats; return b.group; });
          check(stats.nodes === 279242 && stats.instances === 2148183 && stats.drawCalls === 4, 'Two Trees full comparison counts');
          check(stats.handoffs === 2724 && stats.twigs === 87043 && stats.complete, 'pooled diagnostics');
          stage.frame(148);
          return { subject, stats };
        }
        const fixture = await (await fetch(`${referenceUrl}/${subject}.json`)).json();
        check(fixture.revision === 'fdafb099b1495519de75a6b9a66d37f7d07e47bd', 'matched final FN6 reference');
        const family = structuredClone(subject === 'ordinary' ? ORDINARY : subject === 'telperion' ? TELPERION : LAURELIN);
        if (subject === 'ordinary') { family.skeleton.seed = 42; family.skeleton.attractors = 500; family.skeleton.step = 0.02; }
        const engine = await TreeEngine.create();
        const built = engine.build(family, { surface: true, foliage: true, structure: true });
        const numeric = { nodes: built.diagnostics.nodes, instances: built.diagnostics.instances };
        check(numeric.nodes === fixture.diagnostics.nodes, 'solved node count');
        async function compare(array, file, Type, tolerance) {
          const ref = new Type(await (await fetch(`${referenceUrl}/${subject}-${file}.bin`)).arrayBuffer());
          check(ref.length === array.length, `${file} length ${ref.length} != ${array.length}`);
          let maxError = 0, changed = 0;
          for (let i = 0; i < ref.length; i++) { const error = Math.abs(ref[i] - array[i]); check(Number.isFinite(array[i]), `${file} finite`); maxError = Math.max(maxError, error); if (error) changed++; }
          if (tolerance !== null) check(maxError <= tolerance, `${file} drift ${maxError} > ${tolerance}`);
          // Old and new float32 positions differ at a handful of ULP boundaries.
          // Normals are checked exactly on the new input above; report the old-input drift.
          if (file === 'surface-normals') {
            let maxAngleDegrees = 0;
            for (let i = 0; i < ref.length; i += 3) {
              if (ref[i] === array[i] && ref[i+1] === array[i+1] && ref[i+2] === array[i+2]) continue;
              const a = new THREE.Vector3(ref[i], ref[i+1], ref[i+2]);
              const b = new THREE.Vector3(array[i], array[i+1], array[i+2]);
              maxAngleDegrees = Math.max(maxAngleDegrees, a.angleTo(b) * 180 / Math.PI);
            }
            return { values: ref.length, maxError, changed, maxAngleDegrees };
          }
          return { values: ref.length, maxError, changed };
        }
        numeric.positions = await compare(built.surface.positions, 'surface-positions', Float32Array, 0.00002);
        const geometry = new THREE.BufferGeometry();
        geometry.setAttribute('position', new THREE.BufferAttribute(built.surface.positions, 3));
        geometry.setIndex(new THREE.BufferAttribute(built.surface.indices, 1));
        geometry.computeVertexNormals();
        const recomputed = geometry.getAttribute('normal').array;
        let normalRecomputeError = 0;
        for (let i = 0; i < recomputed.length; i++) normalRecomputeError = Math.max(normalRecomputeError, Math.abs(recomputed[i] - built.surface.normals[i]));
        check(normalRecomputeError === 0, 'native normals exactly match Three on new positions');
        numeric.normalRecomputeError = normalRecomputeError;
        numeric.normals = await compare(built.surface.normals, 'surface-normals', Float32Array, null);
        geometry.dispose();
        numeric.indices = await compare(built.surface.indices, 'surface-indices', Uint32Array, 0);
        numeric.foliage = await compare(built.foliage.matrices, 'foliage', Float32Array, 0.00002);
        engine.dispose();
        const params = subject === 'ordinary' ? { ...DEFAULT_PARAMS, seed: 42, density: (500 - 250) / (1600 - 250), step: 0.02 } : presetToParams(subject === 'telperion' ? TELPERION : LAURELIN);
        stage.setTree(clay => { const b = buildTree(params, clay, false); stats = b.stats; return b.tree; });
        stage.frame(params.height);
        return { subject, params, stats, numeric };
      }, { subject, referenceUrl });
      await page.mouse.move(800, 500); await page.mouse.wheel(0, 300);
      await page.evaluate(() => new Promise(resolve => { let n = 16; const tick = () => --n ? requestAnimationFrame(tick) : resolve(); requestAnimationFrame(tick); }));
      await page.screenshot({ path: out + '/' + subject + '.png' });
      result.frame = await page.evaluate(() => window.rig.stats());
      results.push(result);
      await writeFile(out + '/migration.json', JSON.stringify({ reference: 'fdafb099b1495519de75a6b9a66d37f7d07e47bd', viewport: [1600, 1000], dpr: 1, headed: process.env.BROWSER_HEADED === '1', results }, null, 2));
      console.log(JSON.stringify(result));
    } finally { await browser.close(); }
  }
} finally { server.close(); }
